use std::path::{Path, PathBuf};
use std::io::BufWriter;
use thiserror::Error;
use printpdf::*;
use crate::analyze::Finding;
use crate::contradict::Contradiction;
use crate::gaps::TimelineGap;

#[derive(Error, Debug)]
pub enum ExhibitError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("pdf error: {0}")]
    Pdf(String),
}

const PAGE_H: f32 = 279.4; // Letter height in mm
const MARGIN: f32 = 25.4;  // 1 inch in mm
const LINE_HEIGHT: f32 = 5.0; // ~14pt in mm
const FONT_SIZE_BODY: f32 = 10.0;
const FONT_SIZE_HEADER: f32 = 14.0;
const FONT_SIZE_TITLE: f32 = 18.0;
const MAX_CHARS_PER_LINE: usize = 80;

/// Build a court-ready exhibit book PDF from findings
#[allow(clippy::too_many_arguments)] // court filing fields are irreducible
pub fn build_exhibit_book(
    findings: &[Finding],
    contradictions: &[Contradiction],
    gaps: &[TimelineGap],
    output_dir: &Path,
    plaintiff: &str,
    defendant: &str,
    case_number: &Option<String>,
    county: &str,
    state: &str,
) -> Result<PathBuf, ExhibitError> {
    let output_path = output_dir.join("PLAINTIFF_EXHIBIT_BOOK.pdf");
    std::fs::create_dir_all(output_dir)?;

    let (doc, page1, layer1) = PdfDocument::new(
        "Plaintiff's Exhibit Book",
        Mm(215.9), // Letter width
        Mm(279.4), // Letter height
        "Cover",
    );

    let font = doc.add_builtin_font(BuiltinFont::TimesRoman)
        .map_err(|e| ExhibitError::Pdf(format!("{}", e)))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::TimesBold)
        .map_err(|e| ExhibitError::Pdf(format!("{}", e)))?;

    // === COVER PAGE ===
    let layer = doc.get_page(page1).get_layer(layer1);
    render_cover(&layer, &font, &font_bold, plaintiff, defendant, case_number, county, state);

    // === TABLE OF CONTENTS ===
    let (toc_page, toc_layer) = doc.add_page(Mm(215.9), Mm(279.4), "TOC");
    let layer = doc.get_page(toc_page).get_layer(toc_layer);
    render_toc(&layer, &font, &font_bold, findings);

    // === EXHIBIT PAGES ===
    for finding in findings {
        let (pg, ly) = doc.add_page(Mm(215.9), Mm(279.4), format!("Exhibit {}", finding.exhibit_number.unwrap_or(0)));
        let layer = doc.get_page(pg).get_layer(ly);
        render_finding_page(&layer, &font, &font_bold, finding);
    }

    // === CONTRADICTION SUMMARY ===
    if !contradictions.is_empty() {
        let (pg, ly) = doc.add_page(Mm(215.9), Mm(279.4), "Contradictions");
        let layer = doc.get_page(pg).get_layer(ly);
        render_contradiction_section(&layer, &font, &font_bold, contradictions);
    }

    // === GAP ANALYSIS ===
    if !gaps.is_empty() {
        let (pg, ly) = doc.add_page(Mm(215.9), Mm(279.4), "Timeline Gaps");
        let layer = doc.get_page(pg).get_layer(ly);
        render_gap_section(&layer, &font, &font_bold, gaps);
    }

    // Save
    let file = std::fs::File::create(&output_path)?;
    doc.save(&mut BufWriter::new(file))
        .map_err(|e| ExhibitError::Pdf(format!("{}", e)))?;

    Ok(output_path)
}

#[allow(clippy::too_many_arguments)]
fn render_cover(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    plaintiff: &str,
    defendant: &str,
    case_number: &Option<String>,
    county: &str,
    state: &str,
) {
    let mut y = PAGE_H - MARGIN - 14.0;
    let x = MARGIN;

    write_text(layer, font_bold, FONT_SIZE_TITLE, x, y, &format!("STATE OF {}", state.to_uppercase()));
    y -= 8.0;
    write_text(layer, font, FONT_SIZE_HEADER, x, y, &format!("Circuit Court for {} County", county));
    y -= 14.0;

    if let Some(cn) = case_number {
        write_text(layer, font, FONT_SIZE_BODY, x, y, &format!("Case No. {}", cn));
        y -= 8.0;
    }

    write_text(layer, font_bold, FONT_SIZE_HEADER, x, y, &plaintiff.to_uppercase());
    y -= 6.0;
    write_text(layer, font, FONT_SIZE_BODY, x + 14.0, y, "Plaintiff");
    y -= 8.0;
    write_text(layer, font, FONT_SIZE_HEADER, x, y, "v.");
    y -= 8.0;
    write_text(layer, font_bold, FONT_SIZE_HEADER, x, y, &defendant.to_uppercase());
    y -= 6.0;
    write_text(layer, font, FONT_SIZE_BODY, x + 14.0, y, "Defendant");
    y -= 20.0;

    write_text(layer, font_bold, FONT_SIZE_TITLE, x, y, "PLAINTIFF'S EXHIBIT BOOK");
}

fn render_toc(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    findings: &[Finding],
) {
    let mut y = PAGE_H - MARGIN;
    let x = MARGIN;

    write_text(layer, font_bold, FONT_SIZE_HEADER, x, y, "TABLE OF CONTENTS");
    y -= 8.0;

    write_text(layer, font_bold, FONT_SIZE_BODY, x, y, "Exhibit  Date        Category                    Summary");
    y -= LINE_HEIGHT;

    for finding in findings {
        if y < MARGIN {
            break; // Would need multi-page TOC for large sets
        }

        let exhibit_num = finding.exhibit_number.unwrap_or(0);
        let date = finding.parsed_date.as_deref().unwrap_or("?");
        let line = format!("{:>5}    {:10}  {:27} {}",
            exhibit_num,
            date,
            finding.category,
            truncate(&finding.summary, 35),
        );
        write_text(layer, font, 8.0, x, y, &line);
        y -= LINE_HEIGHT * 0.85;
    }
}

fn render_finding_page(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    finding: &Finding,
) {
    let mut y = PAGE_H - MARGIN;
    let x = MARGIN;

    // Exhibit header
    let exhibit_num = finding.exhibit_number.unwrap_or(0);
    write_text(layer, font_bold, FONT_SIZE_HEADER, x, y, &format!("EXHIBIT {}", exhibit_num));
    y -= 7.0;

    // Metadata
    let date = finding.parsed_date.as_deref().unwrap_or(&finding.date);
    let custody = match &finding.custody_week {
        Some(crate::analyze::CustodyParent::Plaintiff) => "Plaintiff's Week",
        Some(crate::analyze::CustodyParent::Defendant) => "Defendant's Week",
        _ => "Unknown",
    };

    write_text(layer, font, FONT_SIZE_BODY, x, y, &format!("Date: {}    Category: {}    Custody: {}", date, finding.category, custody));
    y -= LINE_HEIGHT;
    write_text(layer, font, FONT_SIZE_BODY, x, y, &format!("From: {}", finding.from));
    y -= LINE_HEIGHT;
    write_text(layer, font, FONT_SIZE_BODY, x, y, &format!("To: {}", finding.to));
    y -= LINE_HEIGHT;
    write_text(layer, font, FONT_SIZE_BODY, x, y, &format!("Subject: {}", truncate(&finding.subject, 70)));
    y -= LINE_HEIGHT;

    if let Some(child) = &finding.child_name {
        write_text(layer, font, FONT_SIZE_BODY, x, y, &format!("Child: {}", child));
        y -= LINE_HEIGHT;
    }

    y -= 3.0;

    // Summary
    write_text(layer, font_bold, FONT_SIZE_BODY, x, y, &finding.summary);
    y -= LINE_HEIGHT * 1.5;

    // Highlighted text
    if !finding.highlighted_text.is_empty() {
        write_text(layer, font_bold, 9.0, x, y, "Key passages:");
        y -= LINE_HEIGHT;

        for highlight in finding.highlighted_text.iter().take(5) {
            let lines = word_wrap(highlight, MAX_CHARS_PER_LINE);
            for line in &lines {
                if y < MARGIN { break; }
                write_text(layer, font, 9.0, x + 4.0, y, &format!("> {}", line));
                y -= LINE_HEIGHT * 0.9;
            }
            y -= 1.5;
        }
    }

    y -= 3.0;

    // Body excerpt
    if y > MARGIN + 35.0 {
        write_text(layer, font_bold, 9.0, x, y, "Email body:");
        y -= LINE_HEIGHT;

        let body_lines = word_wrap(&finding.detail, MAX_CHARS_PER_LINE);
        for line in &body_lines {
            if y < MARGIN { break; }
            write_text(layer, font, 8.0, x, y, line);
            y -= LINE_HEIGHT * 0.85;
        }
    }

    // Attachment reference
    if let Some(att) = &finding.source_attachment {
        write_text(layer, font, 8.0, x, MARGIN, &format!("Attachment: {}", att));
    }
}

fn render_contradiction_section(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    contradictions: &[Contradiction],
) {
    let mut y = PAGE_H - MARGIN;
    let x = MARGIN;

    write_text(layer, font_bold, FONT_SIZE_HEADER, x, y, "CONTRADICTION ANALYSIS");
    y -= 8.0;

    for c in contradictions {
        if y < MARGIN + 14.0 { break; }

        write_text(layer, font_bold, FONT_SIZE_BODY, x, y,
            &format!("Exhibits {} vs {} — {}", c.exhibit_a, c.exhibit_b, c.contradiction_type));
        y -= LINE_HEIGHT;

        let lines = word_wrap(&c.explanation, MAX_CHARS_PER_LINE);
        for line in &lines {
            if y < MARGIN { break; }
            write_text(layer, font, 9.0, x + 4.0, y, line);
            y -= LINE_HEIGHT * 0.9;
        }
        y -= 3.0;
    }
}

fn render_gap_section(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    gaps: &[TimelineGap],
) {
    let mut y = PAGE_H - MARGIN;
    let x = MARGIN;

    write_text(layer, font_bold, FONT_SIZE_HEADER, x, y, "TIMELINE GAP ANALYSIS");
    y -= 8.0;

    for g in gaps.iter().take(50) {
        if y < MARGIN + 10.0 { break; }

        write_text(layer, font_bold, FONT_SIZE_BODY, x, y,
            &format!("{} — {} to {} ({} days)", g.gap_type, g.start_date, g.end_date, g.duration_days));
        y -= LINE_HEIGHT;
        write_text(layer, font, 9.0, x + 4.0, y,
            &format!("Expected: {}  Significance: {}", g.expected_source, g.significance));
        y -= LINE_HEIGHT + 1.5;
    }
}

fn write_text(layer: &PdfLayerReference, font: &IndirectFontRef, size: f32, x: f32, y: f32, text: &str) {
    // Sanitize: printpdf can't handle certain control chars
    let clean: String = text.chars()
        .map(|c| if c.is_control() && c != '\n' && c != '\t' { ' ' } else { c })
        .collect();

    layer.use_text(&clean, size, Mm(x), Mm(y), font);
}

fn word_wrap(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.lines() {
        if paragraph.is_empty() {
            lines.push(String::new());
            continue;
        }
        let words: Vec<&str> = paragraph.split_whitespace().collect();
        let mut current_line = String::new();

        for word in words {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= max_chars {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }
    lines
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max { s.to_string() } else { format!("{}...", &s[..max]) }
}
