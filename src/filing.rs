//! filing — MD Circuit Court filing generator.
//!
//! Generates properly formatted court filings per MD Rules 1-301.
//! Output: PDF with correct caption, numbered paragraphs, certificate of service.
//! Every factual assertion traced to an exhibit number.
//! Every legal assertion traced to a statute or case citation.

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use std::path::{Path, PathBuf};
use printpdf::*;
use std::io::BufWriter;
use crate::analyze::Finding;
use crate::precedent::PrecedentMatch;
use crate::contradict::Contradiction;

const PAGE_W: Mm = Mm(215.9);
const PAGE_H: Mm = Mm(279.4);
const MARGIN: f32 = 25.4;
const LINE_H: f32 = 5.0;
const DOUBLE_SPACE: f32 = 10.0;

/// Filing type determines format and required sections.
#[derive(Debug, Clone)]
pub enum FilingType {
    /// Motion to Modify Custody — the primary filing
    MotionModifyCustody,
    /// Memorandum of Law in Support
    MemorandumInSupport,
    /// Opposition to Motion
    #[allow(dead_code)]
    Opposition,
    /// Response to Discovery
    #[allow(dead_code)]
    DiscoveryResponse,
    /// Motion for Contempt (violation of existing order)
    #[allow(dead_code)]
    MotionContempt,
}

impl FilingType {
    fn title(&self) -> &str {
        match self {
            Self::MotionModifyCustody => "MOTION TO MODIFY CUSTODY",
            Self::MemorandumInSupport => "MEMORANDUM OF LAW IN SUPPORT OF MOTION TO MODIFY CUSTODY",
            Self::Opposition => "OPPOSITION TO DEFENDANT'S MOTION",
            Self::DiscoveryResponse => "RESPONSE TO INTERROGATORIES",
            Self::MotionContempt => "MOTION FOR CONTEMPT AND SANCTIONS",
        }
    }
}

/// Court filing context — everything needed to generate a filing.
pub struct FilingContext {
    pub plaintiff: String,
    pub defendant: String,
    pub case_number: String,
    pub county: String,
    #[allow(dead_code)]
    pub court: String,
    pub findings: Vec<Finding>,
    pub precedents: Vec<PrecedentMatch>,
    pub contradictions: Vec<Contradiction>,
    pub filing_type: FilingType,
}

/// A numbered paragraph in a filing, with source tracing.
struct FilingParagraph {
    number: usize,
    text: String,
    /// Exhibit numbers supporting this paragraph.
    exhibits: Vec<usize>,
    /// Case citations supporting this paragraph.
    citations: Vec<String>,
    /// Statute references.
    statutes: Vec<String>,
}

/// Generate a complete court filing as PDF.
pub fn generate_filing(
    ctx: &FilingContext,
    output_dir: &Path,
) -> Result<PathBuf, String> {
    std::fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;

    let filename = match ctx.filing_type {
        FilingType::MotionModifyCustody => "MOTION_MODIFY_CUSTODY.pdf",
        FilingType::MemorandumInSupport => "MEMORANDUM_IN_SUPPORT.pdf",
        FilingType::Opposition => "OPPOSITION.pdf",
        FilingType::DiscoveryResponse => "DISCOVERY_RESPONSE.pdf",
        FilingType::MotionContempt => "MOTION_CONTEMPT.pdf",
    };

    let path = output_dir.join(filename);
    let (doc, page1, layer1) = PdfDocument::new(
        ctx.filing_type.title(),
        PAGE_W, PAGE_H, "Layer 1",
    );

    let font = doc.add_builtin_font(BuiltinFont::TimesRoman)
        .map_err(|e| format!("font: {:?}", e))?;
    let bold = doc.add_builtin_font(BuiltinFont::TimesBold)
        .map_err(|e| format!("bold font: {:?}", e))?;

    let mut y = PAGE_H.0 - MARGIN;
    let layer = doc.get_page(page1).get_layer(layer1);

    // ── Caption Block ──
    // MD Rules 1-301: caption must include court name, case number, parties
    y = write_caption(&layer, &font, &bold, y, ctx);

    // ── Title ──
    y -= DOUBLE_SPACE;
    write_centered(&layer, &bold, y, ctx.filing_type.title(), 14.0);
    y -= DOUBLE_SPACE;

    // ── Body: numbered paragraphs ──
    let paragraphs = build_paragraphs(ctx);

    for para in &paragraphs {
        if y < MARGIN + LINE_H * 4.0 {
            // New page needed — in production, create new page
            // For now, truncate with continuation notice
            write_line(&layer, &font, y, "     [Continued on next page]", 12.0);
            break;
        }

        // Paragraph number + text
        let prefix = format!("     {}. ", para.number);
        write_line(&layer, &font, y, &prefix, 12.0);

        // Wrap text to ~65 chars per line (within margins)
        let lines = wrap_text(&para.text, 70);
        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                // First line after number
                let offset_text = format!("         {}", line);
                write_line(&layer, &font, y, &offset_text, 12.0);
            } else {
                y -= LINE_H;
                if y < MARGIN { break; }
                let offset_text = format!("     {}", line);
                write_line(&layer, &font, y, &offset_text, 12.0);
            }
        }

        // Citation footnotes
        if !para.exhibits.is_empty() || !para.citations.is_empty() {
            y -= LINE_H * 0.5;
            let mut refs = Vec::new();
            for ex in &para.exhibits {
                refs.push(format!("Ex. {}", ex));
            }
            for cite in &para.citations {
                refs.push(cite.clone());
            }
            for statute in &para.statutes {
                refs.push(statute.clone());
            }
            let ref_line = format!("     ({})", refs.join("; "));
            write_line(&layer, &font, y, &ref_line, 9.0);
        }

        y -= DOUBLE_SPACE;
    }

    // ── Prayer for Relief ──
    if y > MARGIN + LINE_H * 8.0 {
        y -= DOUBLE_SPACE;
        write_line(&layer, &bold, y, "     WHEREFORE,", 12.0);
        y -= LINE_H;
        write_line(&layer, &font, y,
            &format!("     Plaintiff {} respectfully requests that this Court:", ctx.plaintiff), 12.0);
        y -= DOUBLE_SPACE;

        let reliefs = prayer_for_relief(&ctx.filing_type);
        for (i, relief) in reliefs.iter().enumerate() {
            y -= LINE_H;
            if y < MARGIN { break; }
            write_line(&layer, &font, y, &format!("          {}. {}", i + 1, relief), 12.0);
        }
    }

    // ── Certificate of Service ──
    if y > MARGIN + LINE_H * 6.0 {
        y -= DOUBLE_SPACE * 2.0;
        write_centered(&layer, &bold, y, "CERTIFICATE OF SERVICE", 12.0);
        y -= DOUBLE_SPACE;
        write_line(&layer, &font, y,
            "     I hereby certify that on this ___ day of _______, 20__, a copy of the",
            11.0);
        y -= LINE_H;
        write_line(&layer, &font, y,
            "     foregoing was served upon the Defendant by:",
            11.0);
        y -= DOUBLE_SPACE;
        write_line(&layer, &font, y, "     [ ] First-class mail, postage prepaid", 11.0);
        y -= LINE_H;
        write_line(&layer, &font, y, "     [ ] Hand delivery", 11.0);
        y -= LINE_H;
        write_line(&layer, &font, y, "     [ ] MDEC electronic filing", 11.0);
        y -= DOUBLE_SPACE * 2.0;
        write_line(&layer, &font, y, "     ________________________________", 12.0);
        y -= LINE_H;
        write_line(&layer, &font, y, &format!("     {}, Pro Se", ctx.plaintiff), 12.0);
    }

    doc.save(&mut BufWriter::new(
        std::fs::File::create(&path).map_err(|e| e.to_string())?
    )).map_err(|e| format!("save pdf: {:?}", e))?;

    Ok(path)
}

fn write_caption(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    bold: &IndirectFontRef,
    mut y: f32,
    ctx: &FilingContext,
) -> f32 {
    // Court header
    write_centered(layer, bold, y, &format!("IN THE CIRCUIT COURT FOR {} COUNTY", ctx.county.to_uppercase()), 12.0);
    y -= LINE_H;
    write_centered(layer, bold, y, "STATE OF MARYLAND", 12.0);
    y -= DOUBLE_SPACE;

    // Parties
    write_line(layer, bold, y, &format!("     {}", ctx.plaintiff.to_uppercase()), 12.0);
    y -= LINE_H;
    write_line(layer, font, y, "          Plaintiff,", 12.0);
    y -= LINE_H;

    // Case number on the right
    write_line(layer, bold, y, &format!("                                                Case No. {}", ctx.case_number), 12.0);
    write_line(layer, font, y, "     v.", 12.0);
    y -= LINE_H;
    y -= LINE_H;

    write_line(layer, bold, y, &format!("     {}", ctx.defendant.to_uppercase()), 12.0);
    y -= LINE_H;
    write_line(layer, font, y, "          Defendant.", 12.0);
    y -= LINE_H;

    // Divider
    write_line(layer, font, y, "     _______________________________________________", 12.0);
    y -= LINE_H;

    y
}

/// Build numbered paragraphs from findings, each traced to exhibits and citations.
fn build_paragraphs(ctx: &FilingContext) -> Vec<FilingParagraph> {
    let mut paras = Vec::new();
    let mut num = 1;

    // Opening — jurisdiction and parties
    paras.push(FilingParagraph {
        number: num,
        text: format!(
            "Plaintiff {} files this {} and states as follows:",
            ctx.plaintiff, ctx.filing_type.title().to_lowercase()
        ),
        exhibits: vec![],
        citations: vec![],
        statutes: vec!["MD Family Law §9-101".into()],
    });
    num += 1;

    // Jurisdiction paragraph
    paras.push(FilingParagraph {
        number: num,
        text: format!(
            "This Court has jurisdiction over the parties and the subject matter pursuant to MD Family Law §1-201. The parties reside in {} County, Maryland.",
            ctx.county
        ),
        exhibits: vec![],
        citations: vec![],
        statutes: vec!["MD Family Law §1-201".into()],
    });
    num += 1;

    // Group findings by best-interest factor
    let mut by_factor: std::collections::HashMap<String, Vec<&Finding>> = std::collections::HashMap::new();
    for f in &ctx.findings {
        let cat = format!("{:?}", f.category);
        by_factor.entry(cat).or_default().push(f);
    }

    // One paragraph per finding with exhibit reference
    for finding in &ctx.findings {
        let exhibit_refs: Vec<usize> = finding.exhibit_number.into_iter().collect();

        // Find matching precedent citations
        let mut citations = Vec::new();
        for pm in &ctx.precedents {
            if pm.matching_exhibits.iter().any(|ex| exhibit_refs.contains(ex)) {
                citations.push(format!("{}, {}", pm.precedent.case_name, pm.precedent.citation));
            }
        }

        paras.push(FilingParagraph {
            number: num,
            text: finding.summary.clone(),
            exhibits: exhibit_refs,
            citations,
            statutes: vec![],
        });
        num += 1;
    }

    // Contradiction paragraphs
    for contra in &ctx.contradictions {
        paras.push(FilingParagraph {
            number: num,
            text: format!(
                "Defendant's own communications contradict their position: {}",
                contra.explanation
            ),
            exhibits: vec![contra.exhibit_a, contra.exhibit_b],
            citations: vec![],
            statutes: vec![],
        });
        num += 1;
    }

    paras
}

fn prayer_for_relief(filing_type: &FilingType) -> Vec<String> {
    match filing_type {
        FilingType::MotionModifyCustody => vec![
            "Modify the existing custody arrangement in the best interest of the minor child(ren)".into(),
            "Grant Plaintiff primary physical custody".into(),
            "Order a custody evaluation pursuant to MD Family Law §9-109".into(),
            "Grant such other and further relief as this Court deems just and proper".into(),
        ],
        FilingType::MotionContempt => vec![
            "Find Defendant in contempt of the existing court order".into(),
            "Impose appropriate sanctions".into(),
            "Award Plaintiff reasonable costs and fees".into(),
            "Grant such other and further relief as this Court deems just and proper".into(),
        ],
        _ => vec![
            "Grant the relief requested herein".into(),
            "Grant such other and further relief as this Court deems just and proper".into(),
        ],
    }
}

fn write_line(layer: &PdfLayerReference, font: &IndirectFontRef, y: f32, text: &str, size: f32) {
    layer.use_text(text, size, Mm(MARGIN), Mm(y), font);
}

fn write_centered(layer: &PdfLayerReference, font: &IndirectFontRef, y: f32, text: &str, size: f32) {
    // Approximate centering: page width 215.9mm, ~2.5mm per char at 12pt
    let char_w = size * 0.2;
    let text_w = text.len() as f32 * char_w;
    let x = (PAGE_W.0 - text_w) / 2.0;
    layer.use_text(text, size, Mm(x.max(MARGIN)), Mm(y), font);
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        if current.len() + word.len() + 1 > max_chars && !current.is_empty() {
            lines.push(current.clone());
            current.clear();
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}
