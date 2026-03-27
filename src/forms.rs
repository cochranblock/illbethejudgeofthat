use std::path::{Path, PathBuf};
use std::io::BufWriter;
use thiserror::Error;
use printpdf::*;
use chrono::Datelike;

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

    // CC-DC-CV-001: Case Information Report (required with every circuit court filing)
    let cir_path = output_dir.join("CC-DC-CV-001_CASE_INFO_REPORT.pdf");
    generate_cc_dc_cv_001(&cir_path, plaintiff, defendant, children, dobs, case_number, county)?;
    forms.push(cir_path);

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

/// CC-DC-CV-001: Civil-Domestic Case Information Report
/// Required by MD Rule 20-201 with every circuit court domestic filing.
/// Without this form, the clerk will reject the filing package.
fn generate_cc_dc_cv_001(
    path: &Path,
    plaintiff: &str,
    defendant: &str,
    children: &str,
    dobs: &str,
    case_number: &Option<String>,
    county: &str,
) -> Result<(), FormsError> {
    let (doc, page1, layer1) = PdfDocument::new(
        "CC-DC-CV-001 Case Information Report",
        PAGE_W, PAGE_H, "Page 1",
    );

    let font = doc.add_builtin_font(BuiltinFont::TimesRoman)
        .map_err(|e| FormsError::Pdf(format!("{}", e)))?;
    let bold = doc.add_builtin_font(BuiltinFont::TimesBold)
        .map_err(|e| FormsError::Pdf(format!("{}", e)))?;

    let layer = doc.get_page(page1).get_layer(layer1);
    let mut y = PAGE_H.0 - MARGIN;
    let x = MARGIN;
    let col2 = 120.0; // second column for checkboxes/values

    // Header
    write_text(&layer, &bold, 11.0, x, y, "CC-DC-CV-001 (Rev. 07/2023)");
    write_text(&layer, &font, 9.0, col2, y, "CIVIL-DOMESTIC CASE INFORMATION REPORT");
    y -= 8.0;
    write_text(&layer, &font, 8.0, x, y, "Maryland Rule 20-201. File with Clerk with initial complaint or petition.");
    y -= 10.0;

    // Section 1: Court and case
    write_text(&layer, &bold, 10.0, x, y, "1. COURT AND CASE INFORMATION");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, &format!("Court: Circuit Court for {} County", county));
    y -= LINE_H;
    let cn = case_number.as_deref().unwrap_or("____________");
    write_text(&layer, &font, 10.0, x + 5.0, y, &format!("Case Number: {}", cn));
    y -= LINE_H;
    let today = chrono::Local::now().format("%m/%d/%Y").to_string();
    write_text(&layer, &font, 10.0, x + 5.0, y, &format!("Date Filed: {}", today));
    y -= 8.0;

    // Section 2: Case type
    write_text(&layer, &bold, 10.0, x, y, "2. CASE TYPE (check one)");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "[ ] Divorce     [ ] Annulment     [ ] Separate Maintenance");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "[X] Custody     [ ] Visitation     [ ] Child Support");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "[ ] Paternity   [ ] Protective Order   [ ] Other: ____________");
    y -= LINE_H;
    write_text(&layer, &bold, 9.0, x + 5.0, y, "Subcategory: [X] Modification of existing order");
    y -= 8.0;

    // Section 3: Parties
    write_text(&layer, &bold, 10.0, x, y, "3. PARTY INFORMATION");
    y -= LINE_H;

    // Plaintiff
    write_text(&layer, &bold, 9.0, x + 5.0, y, "PLAINTIFF / PETITIONER:");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 10.0, y, &format!("Name: {}", plaintiff));
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 10.0, y, "Attorney: [X] Self-represented (Pro Se)   [ ] Represented by: ____________");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 10.0, y, "Address: ____________________________________________");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 10.0, y, "Phone: ____________________  Email: ____________________");
    y -= 8.0;

    // Defendant
    write_text(&layer, &bold, 9.0, x + 5.0, y, "DEFENDANT / RESPONDENT:");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 10.0, y, &format!("Name: {}", defendant));
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 10.0, y, "Attorney: [ ] Self-represented (Pro Se)   [ ] Represented by: ____________");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 10.0, y, "Address: ____________________________________________");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 10.0, y, "Phone: ____________________  Email: ____________________");
    y -= 8.0;

    // Section 4: Minor children
    let children_list: Vec<&str> = children.split(',').map(|s| s.trim()).collect();
    let dobs_list: Vec<&str> = dobs.split(',').map(|s| s.trim()).collect();

    write_text(&layer, &bold, 10.0, x, y, "4. MINOR CHILDREN");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, &format!("Number of minor children: {}", children_list.len()));
    y -= LINE_H;

    // Child table header
    write_text(&layer, &bold, 9.0, x + 5.0, y, "Name");
    write_text(&layer, &bold, 9.0, x + 80.0, y, "DOB");
    write_text(&layer, &bold, 9.0, x + 115.0, y, "Age");
    write_text(&layer, &bold, 9.0, x + 135.0, y, "Resides With");
    y -= LINE_H;

    for (i, name) in children_list.iter().enumerate() {
        let dob_str = dobs_list.get(i).unwrap_or(&"Unknown");
        let age = compute_age(dob_str);
        write_text(&layer, &font, 9.0, x + 5.0, y, name);
        write_text(&layer, &font, 9.0, x + 80.0, y, dob_str);
        write_text(&layer, &font, 9.0, x + 115.0, y, &age);
        write_text(&layer, &font, 9.0, x + 135.0, y, "Both parents (shared)");
        y -= LINE_H;
    }
    y -= 3.0;

    // Section 5: Related cases
    write_text(&layer, &bold, 10.0, x, y, "5. RELATED CASES");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "Are there any related cases pending or previously filed?");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "[ ] No     [X] Yes — Original custody order (same case number)");
    y -= 8.0;

    // Section 6: Relief sought
    write_text(&layer, &bold, 10.0, x, y, "6. RELIEF SOUGHT");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "[X] Modification of custody");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "[X] Modification of visitation / parenting time");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "[ ] Modification of child support");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "[ ] Contempt / enforcement");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "[ ] Other: ____________");
    y -= 8.0;

    // Section 7: ADR
    write_text(&layer, &bold, 10.0, x, y, "7. ALTERNATIVE DISPUTE RESOLUTION");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "Have the parties attempted mediation or other ADR?");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "[ ] Yes   [ ] No   [ ] Not applicable (safety concerns)");
    y -= 8.0;

    // Section 8: Scheduling
    write_text(&layer, &bold, 10.0, x, y, "8. ESTIMATED TRIAL TIME");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "[ ] 1 day or less   [X] 2-3 days   [ ] More than 3 days");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x + 5.0, y, "Number of witnesses: ______   Expert witnesses: [ ] Yes  [ ] No");
    y -= 10.0;

    // Signature
    write_text(&layer, &font, 10.0, x, y, "________________________________");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x, y, &format!("{}, Pro Se", plaintiff));
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x, y, &format!("Date: {}", today));

    let file = std::fs::File::create(path)?;
    doc.save(&mut BufWriter::new(file))
        .map_err(|e| FormsError::Pdf(format!("{}", e)))?;

    Ok(())
}

fn compute_age(dob_str: &str) -> String {
    // Parse MM/DD/YYYY
    let parts: Vec<&str> = dob_str.split('/').collect();
    if parts.len() != 3 { return "—".into(); }
    let month: u32 = parts[0].parse().unwrap_or(0);
    let day: u32 = parts[1].parse().unwrap_or(0);
    let year: i32 = parts[2].parse().unwrap_or(0);
    if month == 0 || day == 0 || year == 0 { return "—".into(); }

    let today = chrono::Local::now().date_naive();
    match chrono::NaiveDate::from_ymd_opt(year, month, day) {
        Some(dob) => {
            let mut age = today.year() - dob.year();
            if today.ordinal() < dob.ordinal() { age -= 1; }
            format!("{}", age)
        }
        None => "—".into(),
    }
}

fn write_text(layer: &PdfLayerReference, font: &IndirectFontRef, size: f32, x: f32, y: f32, text: &str) {
    let clean: String = text.chars()
        .map(|c| if c.is_control() && c != '\n' && c != '\t' { ' ' } else { c })
        .collect();
    layer.use_text(&clean, size, Mm(x), Mm(y), font);
}
