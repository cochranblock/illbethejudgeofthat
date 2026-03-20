use std::collections::{HashMap, HashSet};
use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use crate::analyze::{Finding, FindingCategory, CustodyParent, parse_email_date};
use crate::thread::Thread;
use crate::ingest::Email;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineGap {
    pub gap_type: GapType,
    pub start_date: String,
    pub end_date: String,
    pub duration_days: i64,
    pub custody_parent: Option<CustodyParent>,
    pub expected_source: String,
    pub significance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GapType {
    DailyReportMissing,
    CommunicationSilence,
    CustodyWeekNoSchool,
    ThreadAbandoned,
}

impl std::fmt::Display for GapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DailyReportMissing => write!(f, "Daily Report Missing"),
            Self::CommunicationSilence => write!(f, "Communication Silence"),
            Self::CustodyWeekNoSchool => write!(f, "Custody Week No School Comms"),
            Self::ThreadAbandoned => write!(f, "Thread Abandoned"),
        }
    }
}

/// Detect timeline gaps in the communication record
pub fn detect_gaps(
    emails: &[Email],
    findings: &[Finding],
    threads: &[Thread],
) -> Vec<TimelineGap> {
    let mut gaps = Vec::new();

    // Collect all dates with daily reports
    let daily_report_dates: HashSet<NaiveDate> = findings.iter()
        .filter(|f| f.category == FindingCategory::DailyReport)
        .filter_map(|f| f.parsed_date.as_deref().and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()))
        .collect();

    // Find date range of daily reports
    if let (Some(&min_date), Some(&max_date)) = (daily_report_dates.iter().min(), daily_report_dates.iter().max()) {
        // Walk weekdays looking for missing daily reports
        let mut date = min_date;
        let mut gap_start: Option<NaiveDate> = None;

        while date <= max_date {
            let is_weekday = !matches!(date.weekday(), Weekday::Sat | Weekday::Sun);

            if is_weekday && !daily_report_dates.contains(&date) {
                if gap_start.is_none() {
                    gap_start = Some(date);
                }
            } else if let Some(start) = gap_start {
                let prev = date - chrono::Duration::days(1);
                let duration = (prev - start).num_days() + 1;
                if duration >= 2 {
                    gaps.push(TimelineGap {
                        gap_type: GapType::DailyReportMissing,
                        start_date: start.format("%Y-%m-%d").to_string(),
                        end_date: prev.format("%Y-%m-%d").to_string(),
                        duration_days: duration,
                        custody_parent: None,
                        expected_source: "Chimes daily report".into(),
                        significance: format!("{} weekdays without daily report", duration),
                    });
                }
                gap_start = None;
            }

            date += chrono::Duration::days(1);
        }

        // Close any trailing gap
        if let Some(start) = gap_start {
            let duration = (max_date - start).num_days() + 1;
            if duration >= 2 {
                gaps.push(TimelineGap {
                    gap_type: GapType::DailyReportMissing,
                    start_date: start.format("%Y-%m-%d").to_string(),
                    end_date: max_date.format("%Y-%m-%d").to_string(),
                    duration_days: duration,
                    custody_parent: None,
                    expected_source: "Chimes daily report".into(),
                    significance: format!("{} weekdays without daily report", duration),
                });
            }
        }
    }

    // Communication silence: per-sender, 5+ day gaps
    let mut sender_dates: HashMap<String, Vec<NaiveDate>> = HashMap::new();
    for email in emails {
        let sender = email.from.to_lowercase();
        if let Some(d) = parse_email_date(&email.date) {
            sender_dates.entry(sender).or_default().push(d);
        }
    }

    for (sender, dates) in &mut sender_dates {
        if dates.len() < 3 { continue; }
        dates.sort();
        dates.dedup();

        for window in dates.windows(2) {
            let gap = (window[1] - window[0]).num_days();
            if gap >= 5 {
                let short_sender = if sender.len() > 30 {
                    format!("{}...", &sender[..30])
                } else {
                    sender.clone()
                };
                gaps.push(TimelineGap {
                    gap_type: GapType::CommunicationSilence,
                    start_date: window[0].format("%Y-%m-%d").to_string(),
                    end_date: window[1].format("%Y-%m-%d").to_string(),
                    duration_days: gap,
                    custody_parent: None,
                    expected_source: short_sender,
                    significance: format!("{} days of silence", gap),
                });
            }
        }
    }

    // Thread abandoned: last message is a question with no reply within 48h
    for thread in threads {
        if thread.message_count < 2 { continue; }
        let last_idx = *thread.emails.last().unwrap();
        let email = &emails[last_idx];
        let body_lower = email.body.to_lowercase();

        // Heuristic: ends with a question
        let has_question = body_lower.contains('?')
            || body_lower.contains("please let me know")
            || body_lower.contains("can you")
            || body_lower.contains("could you");

        if has_question {
            if let (Some(start), Some(end)) = (&thread.start_date, &thread.end_date) {
                if start != end {
                    gaps.push(TimelineGap {
                        gap_type: GapType::ThreadAbandoned,
                        start_date: end.clone(),
                        end_date: end.clone(),
                        duration_days: 0,
                        custody_parent: None,
                        expected_source: format!("Reply to: {}", thread.subject),
                        significance: "Thread ends with unanswered question/request".into(),
                    });
                }
            }
        }
    }

    // Sort by start date
    gaps.sort_by(|a, b| a.start_date.cmp(&b.start_date));

    gaps
}

pub fn summarize_gaps(gaps: &[TimelineGap]) -> String {
    let mut out = String::new();
    out.push_str(&format!("Timeline gaps found: {}\n", gaps.len()));

    let mut by_type: HashMap<String, usize> = HashMap::new();
    for g in gaps {
        *by_type.entry(g.gap_type.to_string()).or_default() += 1;
    }

    for (t, count) in &by_type {
        out.push_str(&format!("  {:30} {}\n", t, count));
    }

    // Longest gaps
    let mut sorted: Vec<&TimelineGap> = gaps.iter()
        .filter(|g| g.gap_type != GapType::ThreadAbandoned)
        .collect();
    sorted.sort_by(|a, b| b.duration_days.cmp(&a.duration_days));

    if !sorted.is_empty() {
        out.push_str("\nLongest gaps:\n");
        for g in sorted.iter().take(10) {
            out.push_str(&format!("  {} days | {} to {} | {} | {}\n",
                g.duration_days, g.start_date, g.end_date, g.gap_type, g.expected_source,
            ));
        }
    }

    out
}
