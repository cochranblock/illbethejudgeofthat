use std::path::{Path, PathBuf};
use thiserror::Error;
use crate::ingest::T0;

#[derive(Error, Debug)]
pub enum T4 {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // fields used by analyze.rs attachment linking
pub struct T3 {
    pub email_index: usize,
    pub filename: String,
    pub path: PathBuf,
    pub content_type: String,
}

/// Extract attachments from parsed emails to output directory
pub fn f1(
    emails: &[T0],
    output_dir: &Path,
) -> Result<Vec<T3>, T4> {
    let attach_dir = output_dir.join("attachments");
    std::fs::create_dir_all(&attach_dir)?;

    let mut extracted = Vec::new();

    for email in emails {
        for attachment in &email.attachments {
            let safe_name = format!("{}_{}", email.index,
                attachment.filename.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_"));
            let path = attach_dir.join(&safe_name);
            std::fs::write(&path, &attachment.data)?;
            extracted.push(T3 {
                email_index: email.index,
                filename: attachment.filename.clone(),
                path,
                content_type: attachment.content_type.clone(),
            });
        }
    }

    Ok(extracted)
}
