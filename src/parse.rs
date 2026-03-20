use std::path::{Path, PathBuf};
use thiserror::Error;
use crate::ingest::Email;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // fields used by analyze.rs attachment linking
pub struct ExtractedAttachment {
    pub email_index: usize,
    pub filename: String,
    pub path: PathBuf,
    pub content_type: String,
}

/// Extract attachments from parsed emails to output directory
pub fn extract_attachments(
    emails: &[Email],
    output_dir: &Path,
) -> Result<Vec<ExtractedAttachment>, ParseError> {
    let attach_dir = output_dir.join("attachments");
    std::fs::create_dir_all(&attach_dir)?;

    let mut extracted = Vec::new();

    for email in emails {
        for attachment in &email.attachments {
            let safe_name = format!("{}_{}", email.index,
                attachment.filename.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_"));
            let path = attach_dir.join(&safe_name);
            std::fs::write(&path, &attachment.data)?;
            extracted.push(ExtractedAttachment {
                email_index: email.index,
                filename: attachment.filename.clone(),
                path,
                content_type: attachment.content_type.clone(),
            });
        }
    }

    Ok(extracted)
}
