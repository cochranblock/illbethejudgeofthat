use thiserror::Error;
use serde::{Deserialize, Serialize};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use regex::Regex;
use crate::ingest::Email;
use crate::parse::ExtractedAttachment;

#[derive(Error, Debug)]
pub enum AnalyzeError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub category: FindingCategory,
    pub date: String,
    pub parsed_date: Option<String>,
    pub summary: String,
    pub detail: String,
    pub source_email_index: Option<usize>,
    pub source_attachment: Option<String>,
    pub highlighted_text: Vec<String>,
    pub exhibit_number: Option<usize>,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub custody_week: Option<CustodyParent>,
    pub child_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CustodyParent {
    Plaintiff,
    Defendant,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FindingCategory {
    CustodyInterference,
    FoodRecord,
    HealthConcern,
    Alienation,
    SchoolAbsence,
    InstitutionalBias,
    FinancialChange,
    FalseAllegation,
    ReportingDiscrepancy,
    DeEscalation,
    CourtThreat,
    DailyReport,
    IepViolation,
    BehavioralIncident,
    CommunicationBlock,
    StateComplaint,
    MedicationIssue,
    WeightTracking,
    TransportationIssue,
    AdmissionAgainstInterest,
}

impl std::fmt::Display for FindingCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CustodyInterference => write!(f, "Custody Interference"),
            Self::FoodRecord => write!(f, "Food Record"),
            Self::HealthConcern => write!(f, "Health Concern"),
            Self::Alienation => write!(f, "Alienation"),
            Self::SchoolAbsence => write!(f, "School Absence"),
            Self::InstitutionalBias => write!(f, "Institutional Bias"),
            Self::FinancialChange => write!(f, "Financial Change"),
            Self::FalseAllegation => write!(f, "False Allegation"),
            Self::ReportingDiscrepancy => write!(f, "Reporting Discrepancy"),
            Self::DeEscalation => write!(f, "De-Escalation"),
            Self::CourtThreat => write!(f, "Court Threat"),
            Self::DailyReport => write!(f, "Daily Report"),
            Self::IepViolation => write!(f, "IEP Violation"),
            Self::BehavioralIncident => write!(f, "Behavioral Incident"),
            Self::CommunicationBlock => write!(f, "Communication Block"),
            Self::StateComplaint => write!(f, "State Complaint"),
            Self::MedicationIssue => write!(f, "Medication Issue"),
            Self::WeightTracking => write!(f, "Weight Tracking"),
            Self::TransportationIssue => write!(f, "Transportation Issue"),
            Self::AdmissionAgainstInterest => write!(f, "Admission Against Interest"),
        }
    }
}

/// Custody schedule: alternating weeks starting from a known date
struct CustodySchedule {
    plaintiff_start: NaiveDate,
}

impl CustodySchedule {
    fn new(start: NaiveDate) -> Self {
        Self { plaintiff_start: start }
    }

    fn who_has_custody(&self, date: NaiveDate) -> CustodyParent {
        let days_since_thursday = (date.weekday().num_days_from_monday() + 4) % 7;
        let this_thursday = date - chrono::Duration::days(days_since_thursday as i64);
        let weeks_diff = (this_thursday - self.plaintiff_start).num_weeks();
        if weeks_diff % 2 == 0 {
            CustodyParent::Plaintiff
        } else {
            CustodyParent::Defendant
        }
    }
}

/// Parse email Date header into NaiveDate (public for thread.rs)
pub fn parse_email_date(date_str: &str) -> Option<NaiveDate> {
    let formats = [
        "%a, %d %b %Y %H:%M:%S %z",
        "%a, %d %b %Y %H:%M:%S",
        "%d %b %Y %H:%M:%S %z",
        "%d %b %Y %H:%M:%S",
        "%a, %d %b %Y",
    ];

    let cleaned = date_str.trim();
    for fmt in &formats {
        if let Ok(dt) = chrono::DateTime::parse_from_str(cleaned, fmt) {
            return Some(dt.date_naive());
        }
        if let Ok(dt) = NaiveDateTime::parse_from_str(cleaned, fmt) {
            return Some(dt.date());
        }
    }

    let re = Regex::new(r"(\d{1,2})\s+(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)\s+(\d{4})").ok()?;
    if let Some(caps) = re.captures(cleaned) {
        let day: u32 = caps[1].parse().ok()?;
        let month = match &caps[2] {
            "Jan" => 1, "Feb" => 2, "Mar" => 3, "Apr" => 4,
            "May" => 5, "Jun" => 6, "Jul" => 7, "Aug" => 8,
            "Sep" => 9, "Oct" => 10, "Nov" => 11, "Dec" => 12,
            _ => return None,
        };
        let year: i32 = caps[3].parse().ok()?;
        return NaiveDate::from_ymd_opt(year, month, day);
    }

    None
}

/// Check if email sender matches a person (case-insensitive partial match)
fn sender_is(email: &Email, name_or_addr: &str) -> bool {
    let from_lower = email.from.to_lowercase();
    let check = name_or_addr.to_lowercase();
    from_lower.contains(&check)
}

/// Word-boundary aware match to reduce false positives
fn body_has_word(body_lower: &str, word: &str) -> bool {
    if let Ok(re) = Regex::new(&format!(r"(?i)\b{}\b", regex::escape(word))) {
        re.is_match(body_lower)
    } else {
        body_lower.contains(word)
    }
}

/// Find attachment paths for an email index
fn find_attachment_for_email(email_index: usize, attachments: &[ExtractedAttachment]) -> Option<String> {
    attachments.iter()
        .find(|a| a.email_index == email_index)
        .map(|a| a.path.display().to_string())
}

/// Detect which child an email concerns
fn detect_child(body_lower: &str, subject_lower: &str, children: &[String]) -> Option<String> {
    for name in children {
        if body_lower.contains(name.as_str()) || subject_lower.contains(name.as_str()) {
            return Some(name.clone());
        }
    }
    None
}

/// Analyze emails for custody-relevant findings
pub fn analyze(
    emails: &[Email],
    attachments: &[ExtractedAttachment],
    plaintiff: &str,
    defendant: &str,
    children: &str,
    custody_start: NaiveDate,
) -> Result<Vec<Finding>, AnalyzeError> {
    let mut findings = Vec::new();
    let custody = CustodySchedule::new(custody_start);
    let children_names: Vec<String> = children.split(',').map(|s| s.trim().to_lowercase()).collect();

    for email in emails {
        let body_lower = email.body.to_lowercase();
        let subject_lower = email.subject.to_lowercase();
        let date = parse_email_date(&email.date);
        let custody_week = date.map(|d| custody.who_has_custody(d)).unwrap_or(CustodyParent::Unknown);
        let parsed_date_str = date.map(|d| d.format("%Y-%m-%d").to_string());
        let child = detect_child(&body_lower, &subject_lower, &children_names);
        let attachment_path = find_attachment_for_email(email.index, attachments);

        let base = |cat: FindingCategory, summary: String, highlights: Vec<String>| Finding {
            category: cat,
            date: email.date.clone(),
            parsed_date: parsed_date_str.clone(),
            summary,
            detail: truncate_body(&email.body, 2000),
            source_email_index: Some(email.index),
            source_attachment: attachment_path.clone(),
            highlighted_text: highlights,
            exhibit_number: None,
            from: email.from.clone(),
            to: email.to.clone(),
            subject: email.subject.clone(),
            custody_week: Some(custody_week.clone()),
            child_name: child.clone(),
        };

        // === FOOD RECORDS ===
        if body_lower.contains("refused all food")
            || body_lower.contains("refused food")
            || body_lower.contains("did not eat")
            || body_lower.contains("wouldn't eat")
            || body_lower.contains("would not eat")
        {
            findings.push(base(
                FindingCategory::FoodRecord,
                "Child food refusal documented".into(),
                extract_matching_lines(&email.body, &[
                    "refused all food", "refused food", "did not eat",
                    "wouldn't eat", "would not eat",
                ]),
            ));
        }

        // Food provided by specific parent
        if body_lower.contains("provided by mom")
            || body_lower.contains("mom provided")
            || body_lower.contains("mom sent")
            || body_lower.contains("brought from home")
        {
            findings.push(base(
                FindingCategory::CustodyInterference,
                format!("Food attribution — provided by defendant during {} custody",
                    match &custody_week {
                        CustodyParent::Plaintiff => "plaintiff's",
                        CustodyParent::Defendant => "defendant's",
                        CustodyParent::Unknown => "unknown",
                    }),
                extract_matching_lines(&email.body, &[
                    "provided by mom", "mom provided", "mom sent", "brought from home",
                ]),
            ));
        }

        // Negative meal balance
        if subject_lower.contains("negative meal balance")
            || body_lower.contains("negative meal balance")
        {
            findings.push(base(
                FindingCategory::FinancialChange,
                "Negative school meal balance notification".into(),
                extract_matching_lines(&email.body, &["meal balance", "negative"]),
            ));
        }

        // === WEIGHT / HEALTH ===
        let weight_re = Regex::new(r"(?i)(\d{2,3})\s*(lbs?|pounds?)").unwrap();
        if weight_re.is_match(&email.body) {
            let highlights: Vec<String> = weight_re
                .find_iter(&email.body)
                .map(|m| m.as_str().to_string())
                .collect();
            findings.push(base(
                FindingCategory::WeightTracking,
                "Weight measurement documented".into(),
                highlights,
            ));
        }

        // === DAILY COMMUNICATION REPORTS (Chimes) ===
        if (subject_lower.contains("daily communication")
            || subject_lower.contains("daily parent communication")
            || subject_lower.contains("daily chimes"))
            && sender_is(email, "chimes.org")
        {
            findings.push(base(
                FindingCategory::DailyReport,
                format!("Chimes daily report: {}", email.subject),
                vec![],
            ));
        }

        // === IEP ===
        if subject_lower.contains("iep")
            || subject_lower.contains("annual review")
            || body_has_word(&body_lower, "individualized education")
        {
            let mut cat = FindingCategory::IepViolation;
            let mut summary = "IEP-related communication".to_string();

            if body_lower.contains("missed services")
                || body_lower.contains("not provided")
                || body_lower.contains("did not receive")
            {
                summary = "IEP services missed or not provided".into();
            } else if subject_lower.contains("draft") {
                summary = "IEP draft document exchange".into();
            } else if subject_lower.contains("invite") || subject_lower.contains("meeting") {
                summary = "IEP meeting scheduled".into();
                cat = FindingCategory::DailyReport;
            }

            findings.push(base(cat, summary, vec![]));
        }

        // === STATE COMPLAINT ===
        if subject_lower.contains("state complaint")
            || body_has_word(&body_lower, "state complaint")
            || subject_lower.contains("26-154")
        {
            findings.push(base(
                FindingCategory::StateComplaint,
                "State complaint correspondence (26-154)".into(),
                extract_matching_lines(&email.body, &["state complaint", "26-154", "complaint"]),
            ));
        }

        // === ALIENATION INDICATORS ===
        if sender_is(email, defendant) {
            let alienation_phrases = [
                "don't feel safe", "doesn't feel safe",
                "don't want to go", "doesn't want to go",
                "afraid of", "scared of",
                "don't like dad", "doesn't like",
                "not comfortable",
            ];
            for phrase in &alienation_phrases {
                if body_lower.contains(phrase) {
                    findings.push(base(
                        FindingCategory::Alienation,
                        format!("Potential alienation: \"{}\" from defendant", phrase),
                        extract_matching_lines(&email.body, &[phrase]),
                    ));
                    break;
                }
            }
        }

        // === COURT THREATS (word-boundary aware) ===
        if body_has_word(&body_lower, "going to court")
            || body_has_word(&body_lower, "take you to court")
            || body_has_word(&body_lower, "my lawyer")
            || body_has_word(&body_lower, "my attorney")
            || body_has_word(&body_lower, "divorce proceedings")
        {
            findings.push(base(
                FindingCategory::CourtThreat,
                "Court/legal action referenced".into(),
                extract_matching_lines(&email.body, &[
                    "going to court", "take you to court", "my lawyer",
                    "my attorney", "divorce proceedings",
                ]),
            ));
        }

        // === BEHAVIORAL INCIDENTS ===
        if (body_has_word(&body_lower, "aggression")
            || body_has_word(&body_lower, "meltdown")
            || body_has_word(&body_lower, "tantrum")
            || body_has_word(&body_lower, "hitting")
            || body_has_word(&body_lower, "biting")
            || body_has_word(&body_lower, "elopement")
            || subject_lower.contains("behavior expectations"))
            && (sender_is(email, "chimes.org")
                || sender_is(email, "aacps.org")
                || sender_is(email, "bcps.org"))
        {
            findings.push(base(
                FindingCategory::BehavioralIncident,
                "Behavioral incident reported by school".into(),
                extract_matching_lines(&email.body, &[
                    "aggression", "meltdown", "tantrum",
                    "hitting", "biting", "elopement",
                ]),
            ));
        }

        // === COMMUNICATION BLOCKING ===
        if subject_lower.contains("letter regarding")
            && (subject_lower.contains("access") || subject_lower.contains("communication channels"))
        {
            findings.push(base(
                FindingCategory::CommunicationBlock,
                "Communication access restriction".into(),
                vec![],
            ));
        }

        if body_lower.contains("no longer")
            && (body_has_word(&body_lower, "contact") || body_has_word(&body_lower, "communicate"))
            && sender_is(email, "chimes.org")
        {
            findings.push(base(
                FindingCategory::CommunicationBlock,
                "School restricting parent communication".into(),
                extract_matching_lines(&email.body, &["no longer", "contact", "communicate"]),
            ));
        }

        // === MEDICATION (fixed operator precedence) ===
        if subject_lower.contains("prn medication")
            || (body_has_word(&body_lower, "medication")
                && (body_lower.contains("administered") || body_lower.contains("refused medication")))
        {
            findings.push(base(
                FindingCategory::MedicationIssue,
                "Medication administration documented".into(),
                extract_matching_lines(&email.body, &["medication", "prn", "administered", "refused"]),
            ));
        }

        // === SCHOOL ABSENCE ===
        if subject_lower.contains("child absence")
            || body_lower.contains("absent from school")
            || body_lower.contains("was not in school")
            || subject_lower.contains("absent")
        {
            findings.push(base(
                FindingCategory::SchoolAbsence,
                "School absence recorded".into(),
                extract_matching_lines(&email.body, &["absent", "absence", "not in school"]),
            ));
        }

        // === TRANSPORTATION (fixed operator precedence) ===
        if subject_lower.contains("bus belt")
            || (body_has_word(&body_lower, "bus")
                && (body_has_word(&body_lower, "harness")
                    || body_has_word(&body_lower, "belt")
                    || body_has_word(&body_lower, "transportation")))
        {
            findings.push(base(
                FindingCategory::TransportationIssue,
                "Transportation/bus concern".into(),
                extract_matching_lines(&email.body, &["bus", "belt", "harness", "transportation"]),
            ));
        }

        // === DE-ESCALATION ATTEMPTS (plaintiff showing good faith) ===
        if sender_is(email, plaintiff) {
            let deesc_phrases = [
                "i understand", "let's work together", "for the kids",
                "co-parent", "best interest", "i appreciate",
                "thank you for", "willing to",
            ];
            for phrase in &deesc_phrases {
                if body_lower.contains(phrase) {
                    findings.push(base(
                        FindingCategory::DeEscalation,
                        format!("Plaintiff de-escalation: \"{}\"", phrase),
                        extract_matching_lines(&email.body, &[phrase]),
                    ));
                    break;
                }
            }
        }

        // === ADMISSIONS AGAINST INTEREST ===
        if sender_is(email, defendant) {
            let admission_phrases = [
                "my fault", "i forgot", "i didn't",
                "i should have", "i was wrong",
                "i apologize", "sorry about",
            ];
            for phrase in &admission_phrases {
                if body_lower.contains(phrase) {
                    findings.push(base(
                        FindingCategory::AdmissionAgainstInterest,
                        format!("Defendant admission: \"{}\"", phrase),
                        extract_matching_lines(&email.body, &[phrase]),
                    ));
                    break;
                }
            }
        }

        // === INSTITUTIONAL BIAS ===
        if (sender_is(email, "chimes.org") || sender_is(email, "aacps.org"))
            && (body_lower.contains("we cannot") || body_lower.contains("we do not have the authority"))
            && body_has_word(&body_lower, "recommend")
        {
            findings.push(base(
                FindingCategory::InstitutionalBias,
                "Institution deflecting parental concern".into(),
                extract_matching_lines(&email.body, &["cannot", "authority", "recommend"]),
            ));
        }
    }

    // Sort by parsed date
    findings.sort_by(|a, b| {
        let da = a.parsed_date.as_deref().unwrap_or("9999-99-99");
        let db = b.parsed_date.as_deref().unwrap_or("9999-99-99");
        da.cmp(db)
    });

    // Assign exhibit numbers
    for (i, finding) in findings.iter_mut().enumerate() {
        finding.exhibit_number = Some(i + 1);
    }

    Ok(findings)
}

/// Extract lines from body that contain any of the given phrases
fn extract_matching_lines(body: &str, phrases: &[&str]) -> Vec<String> {
    let mut matches = Vec::new();
    for line in body.lines() {
        let lower = line.to_lowercase();
        if phrases.iter().any(|p| lower.contains(p)) {
            let trimmed = line.trim();
            if !trimmed.is_empty() && trimmed.len() > 5 {
                matches.push(trimmed.to_string());
            }
        }
    }
    matches
}

/// Truncate body text for storage (char-boundary safe)
fn truncate_body(body: &str, max_len: usize) -> String {
    if body.len() <= max_len {
        body.to_string()
    } else {
        let mut end = max_len;
        while end > 0 && !body.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}...[truncated]", &body[..end])
    }
}

/// Generate summary statistics
pub fn summarize_findings(findings: &[Finding]) -> String {
    use std::collections::HashMap;

    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut plaintiff_custody_findings = 0;
    let mut defendant_custody_findings = 0;

    for f in findings {
        *counts.entry(f.category.to_string()).or_default() += 1;
        match &f.custody_week {
            Some(CustodyParent::Plaintiff) => plaintiff_custody_findings += 1,
            Some(CustodyParent::Defendant) => defendant_custody_findings += 1,
            _ => {}
        }
    }

    let mut summary = String::new();
    summary.push_str(&format!("Total findings: {}\n\n", findings.len()));
    summary.push_str("By category:\n");

    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    for (cat, count) in &sorted {
        summary.push_str(&format!("  {:30} {}\n", cat, count));
    }

    summary.push_str(&format!("\nFindings during plaintiff custody weeks: {}\n", plaintiff_custody_findings));
    summary.push_str(&format!("Findings during defendant custody weeks: {}\n", defendant_custody_findings));

    if let Some(first) = findings.first() {
        summary.push_str(&format!("\nDate range: {} to {}\n",
            first.parsed_date.as_deref().unwrap_or(&first.date),
            findings.last().and_then(|f| f.parsed_date.as_deref()).unwrap_or("?"),
        ));
    }

    summary
}
