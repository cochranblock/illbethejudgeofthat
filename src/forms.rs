use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FormsError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Other(String),
}

/// Generate jurisdiction-specific court forms
pub fn generate_forms(
    output_dir: &Path,
    plaintiff: &str,
    defendant: &str,
    children: &str,
    dobs: &str,
    case_number: &Option<String>,
    county: &str,
    state: &str,
) -> Result<Vec<PathBuf>, FormsError> {
    std::fs::create_dir_all(output_dir)?;

    let mut forms = Vec::new();

    match state {
        "MD" => {
            // Maryland forms
            // CC-DR-007: Petition to Modify Custody
            // CC-DR-031: Financial Statement
            // Download templates, fill fields programmatically
            let form_007 = output_dir.join("CC-DR-007_FILLED.pdf");
            let form_031 = output_dir.join("CC-DR-031_FILLED.pdf");

            // TODO: Download official PDFs from mdcourts.gov
            // Fill using pypdf or equivalent Rust PDF library
            // Handle checkboxes, dropdowns, text fields

            forms.push(form_007);
            forms.push(form_031);
        }
        _ => {
            // TODO: Support other state jurisdictions
            return Err(FormsError::Other(format!(
                "State {} not yet supported. MD is available.",
                state
            )));
        }
    }

    Ok(forms)
}
