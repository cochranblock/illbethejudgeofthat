use std::path::{Path, PathBuf};
use std::io::BufWriter;
use thiserror::Error;
use printpdf::*;

#[derive(Error, Debug)]
pub enum FormsError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("pdf error: {0}")]
    Pdf(String),
    #[error("state {0} not supported")]
    UnsupportedState(String),
}

const PAGE_W: Mm = Mm(215.9);
const PAGE_H: Mm = Mm(279.4);
const MARGIN: f32 = 25.4; // 1 inch in mm
const LINE_H: f32 = 5.0;  // ~14pt in mm

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
    skip: bool,
) -> Result<Vec<PathBuf>, FormsError> {
    if skip {
        return Ok(Vec::new());
    }

    std::fs::create_dir_all(output_dir)?;

    match state {
        "MD" => generate_md_forms(output_dir, plaintiff, defendant, children, dobs, case_number, county),
        other => Err(FormsError::UnsupportedState(other.to_string())),
    }
}

fn generate_md_forms(
    output_dir: &Path,
    plaintiff: &str,
    defendant: &str,
    children: &str,
    dobs: &str,
    case_number: &Option<String>,
    county: &str,
) -> Result<Vec<PathBuf>, FormsError> {
    let mut forms = Vec::new();

    // CC-DR-007: Petition to Modify Custody
    let form_path = output_dir.join("CC-DR-007_FILLED.pdf");
    generate_cc_dr_007(&form_path, plaintiff, defendant, children, dobs, case_number, county)?;
    forms.push(form_path);

    Ok(forms)
}

fn generate_cc_dr_007(
    path: &Path,
    plaintiff: &str,
    defendant: &str,
    children: &str,
    dobs: &str,
    case_number: &Option<String>,
    county: &str,
) -> Result<(), FormsError> {
    let (doc, page1, layer1) = PdfDocument::new(
        "CC-DR-007 Petition to Modify Custody",
        PAGE_W, PAGE_H, "Page 1",
    );

    let font = doc.add_builtin_font(BuiltinFont::TimesRoman)
        .map_err(|e| FormsError::Pdf(format!("{}", e)))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::TimesBold)
        .map_err(|e| FormsError::Pdf(format!("{}", e)))?;

    let layer = doc.get_page(page1).get_layer(layer1);
    let mut y = 279.4 - MARGIN;
    let x = MARGIN;

    // Court header
    write_text(&layer, &font_bold, 12.0, x, y, "IN THE CIRCUIT COURT FOR");
    y -= 6.0;
    write_text(&layer, &font_bold, 12.0, x, y, &format!("{} COUNTY, MARYLAND", county.to_uppercase()));
    y -= 10.0;

    // Case number
    if let Some(cn) = case_number {
        write_text(&layer, &font, 10.0, 140.0, y + 4.0, &format!("Case No. {}", cn));
    }

    // Parties
    write_text(&layer, &font_bold, 11.0, x, y, &plaintiff.to_uppercase());
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 7.0, y, "Plaintiff");
    y -= 7.0;
    write_text(&layer, &font, 11.0, x + 7.0, y, "v.");
    y -= 7.0;
    write_text(&layer, &font_bold, 11.0, x, y, &defendant.to_uppercase());
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 7.0, y, "Defendant");
    y -= 10.0;

    // Title
    write_text(&layer, &font_bold, 14.0, x, y, "PETITION TO MODIFY CUSTODY/VISITATION");
    y -= 10.0;

    // Body
    let children_list: Vec<&str> = children.split(',').map(|s| s.trim()).collect();
    let dobs_list: Vec<&str> = dobs.split(',').map(|s| s.trim()).collect();

    write_text(&layer, &font, 10.0, x, y, &format!(
        "The Plaintiff, {}, respectfully petitions this Court to modify the existing",
        plaintiff
    ));
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x, y, "custody and/or visitation order and states as follows:");
    y -= LINE_H * 1.5;

    // Paragraph 1: Children
    write_text(&layer, &font_bold, 10.0, x, y, "1. Minor Children:");
    y -= LINE_H;

    for (i, name) in children_list.iter().enumerate() {
        let dob = dobs_list.get(i).unwrap_or(&"Unknown");
        write_text(&layer, &font, 10.0, x + 7.0, y, &format!("{}. {} (DOB: {})", i + 1, name, dob));
        y -= LINE_H;
    }
    y -= LINE_H;

    // Paragraph 2
    write_text(&layer, &font_bold, 10.0, x, y, "2. Material Change in Circumstances:");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 7.0, y, "Since the entry of the last custody order, there has been a material");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 7.0, y, "change in circumstances affecting the best interests of the minor child(ren),");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 7.0, y, "including but not limited to the matters documented in the attached Exhibit Book.");
    y -= LINE_H * 1.5;

    // Paragraph 3
    write_text(&layer, &font_bold, 10.0, x, y, "3. Best Interest Factors:");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 7.0, y, "The proposed modification serves the best interests of the minor child(ren)");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 7.0, y, "as set forth in Maryland Family Law Section 9-101 et seq.");
    y -= LINE_H * 1.5;

    // Prayer for relief
    write_text(&layer, &font_bold, 10.0, x, y, "WHEREFORE, the Plaintiff prays that this Court:");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 7.0, y, "a) Modify the existing custody/visitation order;");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 7.0, y, "b) Grant such other relief as the Court deems just and proper.");
    y -= LINE_H * 2.0;

    // Signature block
    write_text(&layer, &font, 10.0, x, y, "Respectfully submitted,");
    y -= LINE_H * 2.0;
    write_text(&layer, &font, 10.0, x, y, "________________________________");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x, y, &format!("{}, Pro Se", plaintiff));

    // Save
    let file = std::fs::File::create(path)?;
    doc.save(&mut BufWriter::new(file))
        .map_err(|e| FormsError::Pdf(format!("{}", e)))?;

    Ok(())
}

fn write_text(layer: &PdfLayerReference, font: &IndirectFontRef, size: f32, x: f32, y: f32, text: &str) {
    let clean: String = text.chars()
        .map(|c| if c.is_control() && c != '\n' && c != '\t' { ' ' } else { c })
        .collect();
    layer.use_text(&clean, size, Mm(x), Mm(y), font);
}
