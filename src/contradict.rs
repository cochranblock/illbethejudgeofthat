use std::collections::HashMap;
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use crate::analyze::{T6, T7};
use crate::thread::T5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct T10 {
    pub exhibit_a: usize,
    pub exhibit_b: usize,
    pub contradiction_type: T11,
    pub explanation: String,
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum T11 {
    SchoolVsParentClaim,
    FoodRefusalPattern,
    AttendanceConflict,
    CustodyWeekConflict,
    BehavioralConflict,
}

impl std::fmt::Display for T11 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchoolVsParentClaim => write!(f, "School vs Parent Claim"),
            Self::FoodRefusalPattern => write!(f, "Food Refusal Pattern"),
            Self::AttendanceConflict => write!(f, "Attendance Conflict"),
            Self::CustodyWeekConflict => write!(f, "Custody Week Conflict"),
            Self::BehavioralConflict => write!(f, "Behavioral Conflict"),
        }
    }
}

/// Detect contradictions across findings and threads
pub fn f7(
    findings: &[T6],
    threads: &[T5],
) -> Vec<T10> {
    let mut contradictions = Vec::new();

    // Build index: date → findings
    let mut by_date: HashMap<String, Vec<&T6>> = HashMap::new();
    for f in findings {
        if let Some(d) = &f.parsed_date {
            by_date.entry(d.clone()).or_default().push(f);
        }
    }

    // Build index: thread → findings
    let mut by_thread: HashMap<String, Vec<&T6>> = HashMap::new();
    for thread in threads {
        for &email_idx in &thread.emails {
            for f in findings {
                if f.source_email_index == Some(email_idx) {
                    by_thread.entry(thread.thread_id.clone()).or_default().push(f);
                }
            }
        }
    }

    // Rule 1: Same date, school says food refusal, parent claims well-fed
    for (date, day_findings) in &by_date {
        let school_food: Vec<&&T6> = day_findings.iter()
            .filter(|f| f.category == T7::FoodRecord)
            .filter(|f| is_school_sender(&f.from))
            .collect();

        let parent_interference: Vec<&&T6> = day_findings.iter()
            .filter(|f| f.category == T7::CustodyInterference)
            .collect();

        for sf in &school_food {
            for pi in &parent_interference {
                contradictions.push(T10 {
                    exhibit_a: sf.exhibit_number.unwrap_or(0),
                    exhibit_b: pi.exhibit_number.unwrap_or(0),
                    contradiction_type: T11::FoodRefusalPattern,
                    explanation: format!(
                        "School reports food refusal on {} while parent claims food was provided",
                        date
                    ),
                    date: Some(date.clone()),
                });
            }
        }
    }

    // Rule 2: Same week — school absence vs no parent communication about it
    let mut by_week: HashMap<String, Vec<&T6>> = HashMap::new();
    for f in findings {
        if f.parsed_date.is_some() {
            if let Some(date) = crate::analyze::f4(&f.date) {
                let week_key = format!("{}-W{:02}", date.year(), date.iso_week().week());
                by_week.entry(week_key).or_default().push(f);
            }
        }
    }

    for (week, week_findings) in &by_week {
        let absences: Vec<&&T6> = week_findings.iter()
            .filter(|f| f.category == T7::SchoolAbsence)
            .collect();

        let deescalations: Vec<&&T6> = week_findings.iter()
            .filter(|f| f.category == T7::DeEscalation)
            .collect();

        // Absence during a week where plaintiff is showing good faith = conflict worth noting
        if !absences.is_empty() && !deescalations.is_empty() {
            for abs in &absences {
                for de in &deescalations {
                    if abs.custody_week != de.custody_week {
                        contradictions.push(T10 {
                            exhibit_a: abs.exhibit_number.unwrap_or(0),
                            exhibit_b: de.exhibit_number.unwrap_or(0),
                            contradiction_type: T11::AttendanceConflict,
                            explanation: format!(
                                "School absence recorded in {} while de-escalation attempt in different custody week",
                                week
                            ),
                            date: abs.parsed_date.clone(),
                        });
                    }
                }
            }
        }
    }

    // Rule 3: Within-thread contradictions — different senders, same category, opposite stance
    for (thread_id, thread_findings) in &by_thread {
        // If thread has behavioral incident from school + de-escalation from parent = conflict narrative
        let school_behavioral: Vec<&&T6> = thread_findings.iter()
            .filter(|f| f.category == T7::BehavioralIncident)
            .collect();

        let parent_deesc: Vec<&&T6> = thread_findings.iter()
            .filter(|f| f.category == T7::DeEscalation)
            .collect();

        for sb in &school_behavioral {
            for pd in &parent_deesc {
                contradictions.push(T10 {
                    exhibit_a: sb.exhibit_number.unwrap_or(0),
                    exhibit_b: pd.exhibit_number.unwrap_or(0),
                    contradiction_type: T11::BehavioralConflict,
                    explanation: format!(
                        "Thread {}: school reports behavioral incident while parent shows de-escalation",
                        &thread_id[..thread_id.len().min(12)]
                    ),
                    date: sb.parsed_date.clone(),
                });
            }
        }
    }

    // Sort by date
    contradictions.sort_by(|a, b| {
        let da = a.date.as_deref().unwrap_or("9999");
        let db = b.date.as_deref().unwrap_or("9999");
        da.cmp(db)
    });

    contradictions
}

fn is_school_sender(from: &str) -> bool {
    let lower = from.to_lowercase();
    lower.contains("chimes.org")
        || lower.contains("aacps.org")
        || lower.contains("bcps.org")
}

pub fn f8(contradictions: &[T10]) -> String {
    let mut out = String::new();
    out.push_str(&format!("Contradictions found: {}\n", contradictions.len()));

    let mut by_type: HashMap<String, usize> = HashMap::new();
    for c in contradictions {
        *by_type.entry(c.contradiction_type.to_string()).or_default() += 1;
    }

    for (t, count) in &by_type {
        out.push_str(&format!("  {:30} {}\n", t, count));
    }

    out
}
