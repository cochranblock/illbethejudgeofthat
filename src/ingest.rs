use std::path::{Path, PathBuf};
use thiserror::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Error, Debug)]
pub enum IngestError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error: {0}")]
    Parse(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub index: usize,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub date: String,
    pub body: String,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

/// Ingest an mbox file and return parsed emails
pub fn ingest_mbox(path: &Path) -> Result<Vec<Email>, IngestError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| IngestError::Parse(format!("failed to read mbox: {}", e)))?;

    let mut emails = Vec::new();
    let mut current = String::new();
    let mut index = 0;

    for line in content.lines() {
        if line.starts_with("From ") && !current.is_empty() {
            if let Some(email) = parse_email(index, &current) {
                emails.push(email);
                index += 1;
            }
            current.clear();
        }
        current.push_str(line);
        current.push('\n');
    }

    if !current.is_empty() {
        if let Some(email) = parse_email(index, &current) {
            emails.push(email);
        }
    }

    Ok(emails)
}

fn parse_email(index: usize, raw: &str) -> Option<Email> {
    // TODO: full MIME parsing with mailparse crate
    let from = extract_header(raw, "From:")?;
    let to = extract_header(raw, "To:").unwrap_or_default();
    let subject = extract_header(raw, "Subject:").unwrap_or_default();
    let date = extract_header(raw, "Date:").unwrap_or_default();

    let body_start = raw.find("\n\n").unwrap_or(0);
    let body = raw[body_start..].trim().to_string();

    Some(Email {
        index,
        from,
        to,
        subject,
        date,
        body,
        attachments: Vec::new(), // TODO: MIME attachment extraction
    })
}

fn extract_header(raw: &str, header: &str) -> Option<String> {
    for line in raw.lines() {
        if line.starts_with(header) {
            return Some(line[header.len()..].trim().to_string());
        }
    }
    None
}
