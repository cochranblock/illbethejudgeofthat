use std::path::Path;
use thiserror::Error;
use serde::{Deserialize, Serialize};
use crate::ingest::Email;
use crate::parse::ExtractedAttachment;

#[derive(Error, Debug)]
pub enum AnalyzeError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub category: FindingCategory,
    pub date: String,
    pub summary: String,
    pub detail: String,
    pub source_email_index: Option<usize>,
    pub source_attachment: Option<String>,
    pub highlighted_text: Vec<String>,
    pub exhibit_number: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

/// Analyze emails and attachments for custody-relevant findings
pub fn analyze(
    emails: &[Email],
    attachments: &[ExtractedAttachment],
    plaintiff: &str,
    defendant: &str,
    children: &str,
    schedule: &str,
) -> Result<Vec<Finding>, AnalyzeError> {
    let mut findings = Vec::new();

    // TODO: AI-powered analysis pipeline
    // 1. Identify custody weeks from schedule
    // 2. Cross-reference food reports with custody timing
    // 3. Detect contradictions in school reporting
    // 4. Flag alienating statements
    // 5. Identify admissions against interest
    // 6. Build timeline of events
    // 7. Map findings to HB 1191 factors (or state equivalent)

    // For now, return keyword-based findings
    for email in emails {
        // Food interference detection
        if email.body.to_lowercase().contains("provided by mom")
            || email.body.to_lowercase().contains("mom provided")
        {
            findings.push(Finding {
                category: FindingCategory::CustodyInterference,
                date: email.date.clone(),
                summary: "Food provided by defendant during plaintiff's custody time".into(),
                detail: email.body.clone(),
                source_email_index: Some(email.index),
                source_attachment: None,
                highlighted_text: vec!["provided by mom".into()],
                exhibit_number: None,
            });
        }

        // Alienation detection
        if email.body.to_lowercase().contains("don't feel safe")
            || email.body.to_lowercase().contains("don't like")
                && email.from.to_lowercase().contains(&defendant.to_lowercase())
        {
            findings.push(Finding {
                category: FindingCategory::Alienation,
                date: email.date.clone(),
                summary: "Potential alienating statement from defendant".into(),
                detail: email.body.clone(),
                source_email_index: Some(email.index),
                source_attachment: None,
                highlighted_text: vec![],
                exhibit_number: None,
            });
        }

        // Health concerns
        if email.body.to_lowercase().contains("weight")
            && email.body.to_lowercase().contains("lbs")
        {
            findings.push(Finding {
                category: FindingCategory::HealthConcern,
                date: email.date.clone(),
                summary: "Weight-related health concern documented".into(),
                detail: email.body.clone(),
                source_email_index: Some(email.index),
                source_attachment: None,
                highlighted_text: vec![],
                exhibit_number: None,
            });
        }

        // Refused all food
        if email.body.to_lowercase().contains("refused all food") {
            findings.push(Finding {
                category: FindingCategory::FoodRecord,
                date: email.date.clone(),
                summary: "Child refused all food at school".into(),
                detail: email.body.clone(),
                source_email_index: Some(email.index),
                source_attachment: None,
                highlighted_text: vec!["refused all food".into()],
                exhibit_number: None,
            });
        }
    }

    // Sort chronologically
    findings.sort_by(|a, b| a.date.cmp(&b.date));

    // Assign exhibit numbers
    for (i, finding) in findings.iter_mut().enumerate() {
        finding.exhibit_number = Some(i + 1);
    }

    Ok(findings)
}
