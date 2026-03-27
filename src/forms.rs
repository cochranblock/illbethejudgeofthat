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

    // CC-DR-004: Financial Statement (required in custody/support proceedings)
    let fin_path = output_dir.join("CC-DR-004_FINANCIAL_STATEMENT.pdf");
    generate_cc_dr_004(&fin_path, plaintiff, children, dobs, case_number, county)?;
    forms.push(fin_path);

    // CC-DR-055: Parenting Plan (courts expect a concrete custody proposal)
    let pp_path = output_dir.join("CC-DR-055_PARENTING_PLAN.pdf");
    generate_cc_dr_055(&pp_path, plaintiff, defendant, children, dobs, case_number, county)?;
    forms.push(pp_path);

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

/// CC-DR-004: Financial Statement
/// MD Family Law §12-203 requires disclosure of income and expenses in
/// custody/support proceedings. Court uses this to assess child support
/// and each parent's capacity to provide. Blank fields for the filer to
/// complete — the tool cannot know salary/expenses, but pre-fills the
/// structure so nothing is missed.
fn generate_cc_dr_004(
    path: &Path,
    plaintiff: &str,
    children: &str,
    dobs: &str,
    case_number: &Option<String>,
    county: &str,
) -> Result<(), FormsError> {
    let (doc, page1, layer1) = PdfDocument::new(
        "CC-DR-004 Financial Statement",
        PAGE_W, PAGE_H, "Page 1",
    );

    let font = doc.add_builtin_font(BuiltinFont::TimesRoman)
        .map_err(|e| FormsError::Pdf(format!("{}", e)))?;
    let bold = doc.add_builtin_font(BuiltinFont::TimesBold)
        .map_err(|e| FormsError::Pdf(format!("{}", e)))?;

    let layer = doc.get_page(page1).get_layer(layer1);
    let mut y = PAGE_H.0 - MARGIN;
    let x = MARGIN;
    let val_col = 145.0; // right-aligned value column

    // Header
    write_text(&layer, &bold, 11.0, x, y, "CC-DR-004 (Rev. 06/2022)");
    write_text(&layer, &font, 9.0, 120.0, y, "FINANCIAL STATEMENT");
    y -= 6.0;
    write_text(&layer, &font, 8.0, x, y, "Maryland Family Law Section 12-203. Required in all custody and child support proceedings.");
    y -= 8.0;

    // Court/case header
    write_text(&layer, &font, 10.0, x, y, &format!("Circuit Court for {} County", county));
    let cn = case_number.as_deref().unwrap_or("____________");
    write_text(&layer, &font, 10.0, val_col, y, &format!("Case No. {}", cn));
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x, y, &format!("Party: {} (Plaintiff, Pro Se)", plaintiff));
    y -= 8.0;

    // Section A: Monthly gross income
    write_text(&layer, &bold, 10.0, x, y, "A. MONTHLY GROSS INCOME");
    y -= LINE_H;
    let income_lines = [
        "1. Salary / wages (before deductions)",
        "2. Overtime / bonuses / commissions",
        "3. Self-employment income (net)",
        "4. Social Security / disability",
        "5. Unemployment / workers comp",
        "6. Pension / retirement income",
        "7. Interest / dividends / rental income",
        "8. Child support received (other children)",
        "9. Other income (describe): ____________",
    ];
    for line in &income_lines {
        write_text(&layer, &font, 9.0, x + 5.0, y, line);
        write_text(&layer, &font, 9.0, val_col, y, "$ __________");
        y -= LINE_H;
    }
    write_text(&layer, &bold, 9.0, x + 5.0, y, "10. TOTAL MONTHLY GROSS INCOME");
    write_text(&layer, &bold, 9.0, val_col, y, "$ __________");
    y -= 8.0;

    // Section B: Deductions
    write_text(&layer, &bold, 10.0, x, y, "B. MONTHLY DEDUCTIONS");
    y -= LINE_H;
    let deduction_lines = [
        "1. Federal income tax",
        "2. State income tax",
        "3. FICA (Social Security + Medicare)",
        "4. Health insurance (self only)",
        "5. Health insurance (children)",
        "6. Mandatory retirement contributions",
        "7. Union dues",
    ];
    for line in &deduction_lines {
        write_text(&layer, &font, 9.0, x + 5.0, y, line);
        write_text(&layer, &font, 9.0, val_col, y, "$ __________");
        y -= LINE_H;
    }
    write_text(&layer, &bold, 9.0, x + 5.0, y, "8. TOTAL DEDUCTIONS");
    write_text(&layer, &bold, 9.0, val_col, y, "$ __________");
    y -= LINE_H;
    write_text(&layer, &bold, 9.0, x + 5.0, y, "NET MONTHLY INCOME (A.10 minus B.8)");
    write_text(&layer, &bold, 9.0, val_col, y, "$ __________");
    y -= 8.0;

    // Section C: Monthly expenses
    write_text(&layer, &bold, 10.0, x, y, "C. MONTHLY EXPENSES");
    y -= LINE_H;
    let expense_lines = [
        "1. Rent / mortgage",
        "2. Utilities (electric, gas, water, sewer)",
        "3. Phone / internet",
        "4. Food / groceries",
        "5. Clothing",
        "6. Transportation (car payment, gas, insurance)",
        "7. Medical / dental (unreimbursed)",
        "8. Childcare / aftercare",
        "9. Children's activities / extracurricular",
        "10. School expenses (supplies, fees, lunch)",
        "11. Personal care / household supplies",
        "12. Debt payments (credit cards, loans)",
        "13. Other: ____________",
    ];
    for line in &expense_lines {
        write_text(&layer, &font, 9.0, x + 5.0, y, line);
        write_text(&layer, &font, 9.0, val_col, y, "$ __________");
        y -= LINE_H;
    }
    write_text(&layer, &bold, 9.0, x + 5.0, y, "14. TOTAL MONTHLY EXPENSES");
    write_text(&layer, &bold, 9.0, val_col, y, "$ __________");
    y -= 8.0;

    // Section D: Children
    let children_list: Vec<&str> = children.split(',').map(|s| s.trim()).collect();
    let dobs_list: Vec<&str> = dobs.split(',').map(|s| s.trim()).collect();

    write_text(&layer, &bold, 10.0, x, y, "D. MINOR CHILDREN OF THIS RELATIONSHIP");
    y -= LINE_H;
    for (i, name) in children_list.iter().enumerate() {
        let dob_str = dobs_list.get(i).unwrap_or(&"Unknown");
        let age = compute_age(dob_str);
        write_text(&layer, &font, 9.0, x + 5.0, y, &format!("{}. {} — DOB: {} — Age: {}", i + 1, name, dob_str, age));
        y -= LINE_H;
    }
    y -= 3.0;

    // Verification
    write_text(&layer, &bold, 10.0, x, y, "VERIFICATION");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x, y, "I solemnly affirm under the penalties of perjury that the contents of this");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x, y, "Financial Statement are true and correct to the best of my knowledge.");
    y -= LINE_H * 2.0;

    let today = chrono::Local::now().format("%m/%d/%Y").to_string();
    write_text(&layer, &font, 10.0, x, y, "________________________________");
    write_text(&layer, &font, 10.0, val_col, y, &format!("Date: {}", today));
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x, y, &format!("{}, Pro Se", plaintiff));

    let file = std::fs::File::create(path)?;
    doc.save(&mut BufWriter::new(file))
        .map_err(|e| FormsError::Pdf(format!("{}", e)))?;

    Ok(())
}

/// CC-DR-055: Parenting Plan
/// MD Family Law §9-109.1 — courts may require a parenting plan in any
/// custody proceeding. Filing one proactively shows the court you have a
/// concrete proposal, not just grievances. Covers legal/physical custody,
/// regular schedule, holidays, vacations, decision-making, and communication.
fn generate_cc_dr_055(
    path: &Path,
    plaintiff: &str,
    defendant: &str,
    children: &str,
    dobs: &str,
    case_number: &Option<String>,
    county: &str,
) -> Result<(), FormsError> {
    let (doc, page1, layer1) = PdfDocument::new(
        "CC-DR-055 Parenting Plan",
        PAGE_W, PAGE_H, "Page 1",
    );

    let font = doc.add_builtin_font(BuiltinFont::TimesRoman)
        .map_err(|e| FormsError::Pdf(format!("{}", e)))?;
    let bold = doc.add_builtin_font(BuiltinFont::TimesBold)
        .map_err(|e| FormsError::Pdf(format!("{}", e)))?;

    let layer = doc.get_page(page1).get_layer(layer1);
    let mut y = PAGE_H.0 - MARGIN;
    let x = MARGIN;

    // Header
    write_text(&layer, &bold, 11.0, x, y, "CC-DR-055 (Rev. 01/2023)");
    write_text(&layer, &font, 9.0, 120.0, y, "PARENTING PLAN");
    y -= 6.0;
    write_text(&layer, &font, 8.0, x, y, "Maryland Family Law Section 9-109.1. Filed with Motion to Modify Custody.");
    y -= 8.0;

    // Court/case header
    write_text(&layer, &font, 10.0, x, y, &format!("Circuit Court for {} County", county));
    let cn = case_number.as_deref().unwrap_or("____________");
    write_text(&layer, &font, 10.0, 140.0, y, &format!("Case No. {}", cn));
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x, y, &format!("Parent A (Plaintiff): {}", plaintiff));
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x, y, &format!("Parent B (Defendant): {}", defendant));
    y -= 8.0;

    // Section 1: Children
    let children_list: Vec<&str> = children.split(',').map(|s| s.trim()).collect();
    let dobs_list: Vec<&str> = dobs.split(',').map(|s| s.trim()).collect();

    write_text(&layer, &bold, 10.0, x, y, "1. CHILDREN COVERED BY THIS PLAN");
    y -= LINE_H;
    for (i, name) in children_list.iter().enumerate() {
        let dob_str = dobs_list.get(i).unwrap_or(&"Unknown");
        let age = compute_age(dob_str);
        write_text(&layer, &font, 9.0, x + 5.0, y, &format!("{}. {} — DOB: {} — Age: {}", i + 1, name, dob_str, age));
        y -= LINE_H;
    }
    y -= 3.0;

    // Section 2: Legal custody
    write_text(&layer, &bold, 10.0, x, y, "2. LEGAL CUSTODY (decision-making authority)");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "[ ] Joint legal custody (both parents share major decisions)");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "[ ] Sole legal custody to Parent A (Plaintiff)");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "[ ] Sole legal custody to Parent B (Defendant)");
    y -= 6.0;

    // Section 3: Physical custody
    write_text(&layer, &bold, 10.0, x, y, "3. PHYSICAL CUSTODY (where children reside)");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "[ ] Primary physical custody to Parent A; visitation to Parent B");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "[ ] Primary physical custody to Parent B; visitation to Parent A");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "[ ] Shared physical custody (schedule below)");
    y -= 6.0;

    // Section 4: Regular schedule
    write_text(&layer, &bold, 10.0, x, y, "4. REGULAR PARENTING SCHEDULE");
    y -= LINE_H;
    write_text(&layer, &bold, 9.0, x + 5.0, y, "Weekdays during school year:");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 10.0, y, "Parent A: ____________ through ____________");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 10.0, y, "Parent B: ____________ through ____________");
    y -= LINE_H;
    write_text(&layer, &bold, 9.0, x + 5.0, y, "Weekends:");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 10.0, y, "[ ] Alternating weekends (Fri ___PM to Sun ___PM)");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 10.0, y, "[ ] Other: ____________________________________________");
    y -= LINE_H;
    write_text(&layer, &bold, 9.0, x + 5.0, y, "Exchange location: ____________________________________________");
    y -= LINE_H;
    write_text(&layer, &bold, 9.0, x + 5.0, y, "Exchange time: ____________  Transportation provided by: ____________");
    y -= 6.0;

    // Section 5: Holiday schedule
    write_text(&layer, &bold, 10.0, x, y, "5. HOLIDAY AND SPECIAL DAY SCHEDULE");
    y -= LINE_H;
    let holidays = [
        ("New Year's Day",        "Parent __", "Parent __"),
        ("MLK Day / Presidents'", "Parent __", "Parent __"),
        ("Spring Break",          "Parent __", "Parent __"),
        ("Easter / Passover",     "Parent __", "Parent __"),
        ("Memorial Day weekend",  "Parent __", "Parent __"),
        ("July 4th",              "Parent __", "Parent __"),
        ("Labor Day weekend",     "Parent __", "Parent __"),
        ("Halloween",             "Parent __", "Parent __"),
        ("Thanksgiving",          "Parent __", "Parent __"),
        ("Winter Break (1st half)","Parent __","Parent __"),
        ("Winter Break (2nd half)","Parent __","Parent __"),
        ("Child's birthday",      "Parent __", "Parent __"),
        ("Mother's Day",          "Mother",    "Mother"),
        ("Father's Day",          "Father",    "Father"),
    ];
    // Compact: two columns
    write_text(&layer, &bold, 8.0, x + 5.0, y, "Holiday");
    write_text(&layer, &bold, 8.0, x + 65.0, y, "Even Years");
    write_text(&layer, &bold, 8.0, x + 100.0, y, "Odd Years");
    y -= LINE_H;
    for (holiday, even, odd) in &holidays {
        write_text(&layer, &font, 8.0, x + 5.0, y, holiday);
        write_text(&layer, &font, 8.0, x + 65.0, y, even);
        write_text(&layer, &font, 8.0, x + 100.0, y, odd);
        y -= 4.0;
    }
    y -= 3.0;

    // Section 6: Summer / vacation
    write_text(&layer, &bold, 10.0, x, y, "6. SUMMER AND VACATION");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "Each parent may have ___ weeks of uninterrupted vacation time.");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "Notice required: ___ days before travel.  Out-of-state travel: [ ] Consent  [ ] Notice only");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "Itinerary and contact info: [ ] Required  [ ] Not required");
    y -= 6.0;

    // Section 7: Decision-making
    write_text(&layer, &bold, 10.0, x, y, "7. DECISION-MAKING");
    y -= LINE_H;
    let decisions = [
        "Education (school choice, IEP, tutoring)",
        "Healthcare (medical, dental, mental health, medication)",
        "Religious upbringing",
        "Extracurricular activities",
    ];
    for d in &decisions {
        write_text(&layer, &font, 9.0, x + 5.0, y, &format!("{}:  [ ] Joint  [ ] Parent A  [ ] Parent B", d));
        y -= LINE_H;
    }
    y -= 3.0;

    // Section 8: Communication
    write_text(&layer, &bold, 10.0, x, y, "8. COMMUNICATION");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "Child may contact non-custodial parent: [ ] Daily  [ ] Reasonable  [ ] Scheduled times");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "Parent-to-parent communication: [ ] Email  [ ] Text  [ ] Co-parenting app  [ ] Other");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "Response time expected: [ ] 24 hours  [ ] 48 hours  [ ] Reasonable");
    y -= 6.0;

    // Section 9: Dispute resolution
    write_text(&layer, &bold, 10.0, x, y, "9. DISPUTE RESOLUTION");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "If parents disagree on a matter covered by this plan:");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x + 5.0, y, "[ ] Mediation first   [ ] Parenting coordinator   [ ] Return to court");
    y -= 8.0;

    // Signature
    let today = chrono::Local::now().format("%m/%d/%Y").to_string();
    write_text(&layer, &bold, 9.0, x, y, "SUBMITTED BY:");
    y -= LINE_H;
    write_text(&layer, &font, 10.0, x, y, "________________________________");
    write_text(&layer, &font, 10.0, 120.0, y, "________________________________");
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x, y, &format!("{}, Pro Se", plaintiff));
    write_text(&layer, &font, 9.0, 120.0, y, &format!("{}", defendant));
    y -= LINE_H;
    write_text(&layer, &font, 9.0, x, y, &format!("Date: {}", today));
    write_text(&layer, &font, 9.0, 120.0, y, "Date: ____________");

    // Page 2: additional provisions
    let (page2, layer2) = doc.add_page(PAGE_W, PAGE_H, "Page 2");
    let layer_p2 = doc.get_page(page2).get_layer(layer2);
    y = PAGE_H.0 - MARGIN;

    write_text(&layer_p2, &bold, 11.0, x, y, "CC-DR-055 — PARENTING PLAN (continued)");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x, y, &format!("Case No. {}", cn));
    y -= 10.0;

    // Section 10: Right of first refusal
    write_text(&layer_p2, &bold, 10.0, x, y, "10. RIGHT OF FIRST REFUSAL");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "If the custodial parent will be away for more than ___ hours, the other");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "parent must be offered the opportunity to care for the child(ren) first.");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "[ ] Applies   [ ] Does not apply");
    y -= 8.0;

    // Section 11: Relocation
    write_text(&layer_p2, &bold, 10.0, x, y, "11. RELOCATION");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "Either parent must provide ___ days written notice before relocating");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "more than ___ miles from the current residence.");
    y -= 8.0;

    // Section 12: New partners / third parties
    write_text(&layer_p2, &bold, 10.0, x, y, "12. INTRODUCTION OF NEW PARTNERS");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "[ ] No restrictions");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "[ ] New romantic partners not introduced to children until ___ months");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "[ ] No overnight guests of romantic nature when children are present");
    y -= 8.0;

    // Section 13: Health and safety
    write_text(&layer_p2, &bold, 10.0, x, y, "13. HEALTH AND SAFETY PROVISIONS");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "Each parent shall:");
    y -= LINE_H;
    let health_items = [
        "a) Maintain current emergency contact info with the other parent",
        "b) Notify the other parent within 24 hours of any medical emergency",
        "c) Ensure prescribed medications are administered as directed",
        "d) Ensure child(ren) have current inhaler / EpiPen / medical devices at both homes",
        "e) Maintain age-appropriate car seats and safety equipment",
        "f) Not consume illegal substances or be intoxicated while caring for child(ren)",
        "g) Not disparage the other parent in the child(ren)'s presence",
    ];
    for item in &health_items {
        write_text(&layer_p2, &font, 9.0, x + 10.0, y, item);
        y -= LINE_H;
    }
    y -= 3.0;

    // Section 14: School and education
    write_text(&layer_p2, &bold, 10.0, x, y, "14. SCHOOL AND EDUCATION");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "Both parents shall have access to school records per FERPA (20 U.S.C. 1232g).");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "Both parents shall attend parent-teacher conferences and IEP meetings.");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "Homework folders shall be maintained at both homes.");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "Neither parent shall cause unexcused absences during their custodial time.");
    y -= 8.0;

    // Section 15: Additional provisions
    write_text(&layer_p2, &bold, 10.0, x, y, "15. ADDITIONAL PROVISIONS");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "___________________________________________________________________________");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "___________________________________________________________________________");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "___________________________________________________________________________");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x + 5.0, y, "___________________________________________________________________________");
    y -= 10.0;

    // Final note
    write_text(&layer_p2, &bold, 9.0, x, y, "NOTE TO COURT:");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x, y, "This parenting plan is submitted as a proposed order. Plaintiff requests the");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x, y, "Court adopt this plan or modify it as the Court deems in the best interest");
    y -= LINE_H;
    write_text(&layer_p2, &font, 9.0, x, y, "of the minor child(ren) per MD Family Law Section 9-101 et seq.");

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
