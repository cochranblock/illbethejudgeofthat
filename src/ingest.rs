use std::path::Path;
use thiserror::Error;
use serde::{Deserialize, Serialize};
use mailparse::{parse_mail, MailHeaderMap};

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
    pub cc: String,
    pub subject: String,
    pub date: String,
    pub body: String,
    pub labels: Vec<String>,
    pub message_id: String,
    pub in_reply_to: String,
    pub thread_id: String,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

/// Ingest an mbox file using mailparse for proper MIME decoding
pub fn ingest_mbox(path: &Path) -> Result<Vec<Email>, IngestError> {
    let content = std::fs::read(path)
        .map_err(|e| IngestError::Parse(format!("failed to read mbox: {}", e)))?;

    let raw_str = String::from_utf8_lossy(&content);
    let raw_messages = split_mbox(&raw_str);

    let mut emails = Vec::with_capacity(raw_messages.len());

    for (index, raw) in raw_messages.into_iter().enumerate() {
        match parse_single_email(index, raw.as_bytes()) {
            Ok(email) => emails.push(email),
            Err(e) => {
                eprintln!("  warn: email {} parse failed: {}", index, e);
            }
        }
    }

    Ok(emails)
}

/// Split mbox content on "From " envelope lines
fn split_mbox(content: &str) -> Vec<String> {
    let mut messages = Vec::new();
    let mut current = String::new();

    for line in content.lines() {
        if line.starts_with("From ") && !current.is_empty() {
            messages.push(std::mem::take(&mut current));
        }
        current.push_str(line);
        current.push('\n');
    }

    if !current.is_empty() {
        messages.push(current);
    }

    messages
}

/// Parse a single raw email using mailparse
fn parse_single_email(index: usize, raw: &[u8]) -> Result<Email, IngestError> {
    let parsed = parse_mail(raw)
        .map_err(|e| IngestError::Parse(format!("mailparse: {}", e)))?;

    let headers = &parsed.headers;

    let from = headers.get_first_value("From").unwrap_or_default();
    let to = headers.get_first_value("To").unwrap_or_default();
    let cc = headers.get_first_value("Cc").unwrap_or_default();
    let subject = headers.get_first_value("Subject").unwrap_or_default();
    let date = headers.get_first_value("Date").unwrap_or_default();
    let message_id = headers.get_first_value("Message-ID").unwrap_or_default();
    let in_reply_to = headers.get_first_value("In-Reply-To").unwrap_or_default();
    let thread_id = headers.get_first_value("X-GM-THRID").unwrap_or_default();

    // Gmail labels
    let labels_raw = headers.get_first_value("X-Gmail-Labels").unwrap_or_default();
    let labels: Vec<String> = labels_raw
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Extract body text (prefer plain text, fall back to html stripped)
    let body = extract_body_text(&parsed);

    // Extract attachments
    let attachments = extract_attachments_from_mime(&parsed);

    Ok(Email {
        index,
        from,
        to,
        cc,
        subject,
        date,
        body,
        labels,
        message_id,
        in_reply_to,
        thread_id,
        attachments,
    })
}

/// Recursively extract plain text body from MIME parts
fn extract_body_text(mail: &mailparse::ParsedMail) -> String {
    // If this part is an attachment, skip it for body text
    let disp = mail.get_content_disposition();
    if disp.disposition == mailparse::DispositionType::Attachment {
        return String::new();
    }

    let mime = mail.ctype.mimetype.to_lowercase();

    if mime == "text/plain" {
        return mail.get_body().unwrap_or_default();
    }

    // Recurse into subparts — prefer text/plain
    if !mail.subparts.is_empty() {
        // First pass: look for text/plain
        for sub in &mail.subparts {
            let sub_mime = sub.ctype.mimetype.to_lowercase();
            if sub_mime == "text/plain" {
                let body = extract_body_text(sub);
                if !body.is_empty() {
                    return body;
                }
            }
        }
        // Second pass: recurse into multipart containers
        for sub in &mail.subparts {
            let sub_mime = sub.ctype.mimetype.to_lowercase();
            if sub_mime.starts_with("multipart/") {
                let body = extract_body_text(sub);
                if !body.is_empty() {
                    return body;
                }
            }
        }
        // Third pass: try text/html as fallback, strip tags
        for sub in &mail.subparts {
            let sub_mime = sub.ctype.mimetype.to_lowercase();
            if sub_mime == "text/html" {
                if let Ok(html) = sub.get_body() {
                    return strip_html_tags(&html);
                }
            }
        }
    }

    // If single-part html, strip it
    if mime == "text/html" {
        if let Ok(html) = mail.get_body() {
            return strip_html_tags(&html);
        }
    }

    String::new()
}

/// Basic HTML tag stripping
fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    let mut last_was_space = false;

    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                if !last_was_space {
                    result.push(' ');
                    last_was_space = true;
                }
            }
            _ if !in_tag => {
                if ch.is_whitespace() {
                    if !last_was_space {
                        result.push(' ');
                        last_was_space = true;
                    }
                } else {
                    result.push(ch);
                    last_was_space = false;
                }
            }
            _ => {}
        }
    }

    // Decode common HTML entities
    result
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
        .replace("\u{00a0}", " ")
}

/// Extract MIME attachments recursively
fn extract_attachments_from_mime(mail: &mailparse::ParsedMail) -> Vec<Attachment> {
    let mut attachments = Vec::new();

    let disp = mail.get_content_disposition();
    if disp.disposition == mailparse::DispositionType::Attachment {
        let filename = disp
            .params
            .get("filename")
            .cloned()
            .unwrap_or_else(|| "unnamed".to_string());
        let content_type = mail.ctype.mimetype.clone();
        let data = mail.get_body_raw().unwrap_or_default();

        attachments.push(Attachment {
            filename,
            content_type,
            data,
        });
    }

    for sub in &mail.subparts {
        attachments.extend(extract_attachments_from_mime(sub));
    }

    attachments
}
