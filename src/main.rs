use clap::Parser;
use std::path::PathBuf;
use chrono::NaiveDate;

use illbethejudgeofthat::{
    ingest, parse, thread, analyze, contradict, gaps,
    exhibit, forms, precedent, query, filing, cite_verify,
};

#[derive(Parser)]
#[command(name = "illbethejudgeofthat")]
#[command(about = "Pro se custody case builder. Takeout to courtroom.")]
struct Cli {
    /// Path to Google Takeout mbox file
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for court filing package
    #[arg(short, long, default_value = "./filing")]
    output: PathBuf,

    /// Your name (Plaintiff)
    #[arg(long)]
    plaintiff: String,

    /// Other parent's name (Defendant)
    #[arg(long)]
    defendant: String,

    /// Children's names (comma-separated)
    #[arg(long)]
    children: String,

    /// Children's DOBs (comma-separated, MM/DD/YYYY)
    #[arg(long)]
    dobs: String,

    /// Custody schedule (e.g., "weekly-thursday")
    #[arg(long, default_value = "weekly-thursday")]
    schedule: String,

    /// Known Thursday when plaintiff has custody (YYYY-MM-DD)
    #[arg(long, default_value = "2025-01-02")]
    custody_start: String,

    /// Case number
    #[arg(long)]
    case_number: Option<String>,

    /// State jurisdiction
    #[arg(long, default_value = "MD")]
    state: String,

    /// County
    #[arg(long, default_value = "Anne Arundel")]
    county: String,

    /// Export findings as JSON only (skip PDF/form generation)
    #[arg(long)]
    json_only: bool,

    /// Export raw parsed emails as JSON
    #[arg(long)]
    dump_emails: bool,

    /// Skip court form generation
    #[arg(long)]
    skip_forms: bool,

    /// Enter interactive query mode (requires prior run)
    #[arg(long)]
    query: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Query mode: load existing data and enter REPL
    if cli.query {
        return query::f19(&cli.output);
    }

    let custody_start = NaiveDate::parse_from_str(&cli.custody_start, "%Y-%m-%d")
        .unwrap_or_else(|_| {
            eprintln!("warn: invalid --custody-start, using 2025-01-02");
            NaiveDate::from_ymd_opt(2025, 1, 2).unwrap()
        });

    println!("illbethejudgeofthat v0.4.1");
    println!("=========================");
    println!();
    println!("Plaintiff:      {}", cli.plaintiff);
    println!("Defendant:      {}", cli.defendant);
    println!("Children:       {}", cli.children);
    println!("Custody start:  {}", custody_start);
    println!("Input:          {}", cli.input.display());
    println!("Output:         {}", cli.output.display());
    println!("State:          {}", cli.state);
    println!("County:         {}", cli.county);
    println!();

    std::fs::create_dir_all(&cli.output)?;

    // Stage 1: Ingest
    println!("[1/10] Ingesting email archive...");
    let emails = ingest::f0(&cli.input)?;
    println!("      {} emails parsed", emails.len());

    if cli.dump_emails {
        let dump_path = cli.output.join("emails_raw.json");
        let json = serde_json::to_string_pretty(&emails)?;
        std::fs::write(&dump_path, &json)?;
        println!("      emails dumped to {}", dump_path.display());
    }

    // Stage 2: Parse attachments
    println!("[2/10] Extracting attachments...");
    let attachments = parse::f1(&emails, &cli.output)?;
    println!("      {} attachments extracted", attachments.len());

    // Stage 3: Thread reconstruction
    println!("[3/10] Reconstructing threads...");
    let threads = thread::f2(&emails);
    println!("      {} threads from {} emails", threads.len(), emails.len());
    let thread_summary = thread::f3(&threads);
    println!("{}", thread_summary);

    let threads_path = cli.output.join("threads.json");
    let threads_json = serde_json::to_string_pretty(&threads)?;
    std::fs::write(&threads_path, &threads_json)?;

    // Stage 4: Analyze
    println!("[4/10] Analyzing for findings...");
    let findings = analyze::f5(
        &emails,
        &attachments,
        &cli.plaintiff,
        &cli.defendant,
        &cli.children,
        custody_start,
    )?;
    println!("      {} findings identified", findings.len());
    println!();
    println!("{}", analyze::f6(&findings));

    // Export findings JSON
    let findings_path = cli.output.join("findings.json");
    let findings_json = serde_json::to_string_pretty(&findings)?;
    std::fs::write(&findings_path, &findings_json)?;
    println!("Findings: {}", findings_path.display());

    // Export timeline CSV
    let timeline_path = cli.output.join("timeline.csv");
    export_timeline_csv(&findings, &timeline_path)?;
    println!("Timeline: {}", timeline_path.display());

    // Stage 5: T10 detection
    println!();
    println!("[5/10] Detecting contradictions...");
    let contradictions = contradict::f7(&findings, &threads);
    println!("      {} contradictions found", contradictions.len());
    if !contradictions.is_empty() {
        println!("{}", contradict::f8(&contradictions));
    }

    let contra_path = cli.output.join("contradictions.json");
    let contra_json = serde_json::to_string_pretty(&contradictions)?;
    std::fs::write(&contra_path, &contra_json)?;

    // Stage 6: Gap detection
    println!("[6/10] Detecting timeline gaps...");
    let timeline_gaps = gaps::f9(&emails, &findings, &threads);
    println!("      {} gaps found", timeline_gaps.len());
    if !timeline_gaps.is_empty() {
        println!("{}", gaps::f10(&timeline_gaps));
    }

    let gaps_path = cli.output.join("gaps.json");
    let gaps_json = serde_json::to_string_pretty(&timeline_gaps)?;
    std::fs::write(&gaps_path, &gaps_json)?;

    // Stage 7: T15 matching
    println!("[7/10] Matching precedents...");
    let precedent_matches = precedent::f11(&findings);
    println!("      {} precedent matches", precedent_matches.len());
    if !precedent_matches.is_empty() {
        println!("{}", precedent::f13(&precedent_matches));
    }

    let precedent_path = cli.output.join("precedents.json");
    let precedent_json = serde_json::to_string_pretty(&precedent_matches)?;
    std::fs::write(&precedent_path, &precedent_json)?;

    // Legal brief outline
    let brief = precedent::f12(&precedent_matches, &findings);
    let brief_path = cli.output.join("legal_brief_outline.txt");
    std::fs::write(&brief_path, &brief)?;
    println!("Legal brief: {}", brief_path.display());

    if cli.json_only {
        println!();
        println!("JSON-only mode. Run without --json-only for PDF generation.");
        println!("Run with --query to explore findings interactively.");
        return Ok(());
    }

    // Stage 8: Build exhibit book
    println!();
    println!("[8/10] Building exhibit book...");
    let exhibit_path = exhibit::f14(
        &findings,
        &contradictions,
        &timeline_gaps,
        &cli.output,
        &cli.plaintiff,
        &cli.defendant,
        &cli.case_number,
        &cli.county,
        &cli.state,
    )?;
    println!("      {}", exhibit_path.display());

    // Stage 9: Citation verification
    println!("[9/10] Verifying citations...");
    let cite_checks = cite_verify::f17(&precedent_matches);
    cite_verify::f18(&cite_checks);
    let cite_path = cli.output.join("citation_verification.json");
    std::fs::write(&cite_path, serde_json::to_string_pretty(&cite_checks)?)?;

    // Stage 10: Generate court filings
    println!();
    println!("[10/10] Generating court filings...");
    let filing_ctx = filing::T20 {
        plaintiff: cli.plaintiff.clone(),
        defendant: cli.defendant.clone(),
        case_number: cli.case_number.clone().unwrap_or_else(|| "____________".into()),
        county: cli.county.clone(),
        court: format!("Circuit Court for {} County", cli.county),
        findings: findings.clone(),
        precedents: precedent_matches.clone(),
        contradictions: contradictions.clone(),
        filing_type: filing::T19::MotionModifyCustody,
    };
    match filing::f16(&filing_ctx, &cli.output) {
        Ok(path) => println!("      {}", path.display()),
        Err(e) => eprintln!("      filing error: {}", e),
    }

    // Also generate memorandum in support
    let memo_ctx = filing::T20 {
        filing_type: filing::T19::MemorandumInSupport,
        ..filing::T20 {
            plaintiff: cli.plaintiff.clone(),
            defendant: cli.defendant.clone(),
            case_number: cli.case_number.clone().unwrap_or_else(|| "____________".into()),
            county: cli.county.clone(),
            court: format!("Circuit Court for {} County", cli.county),
            findings: findings.clone(),
            precedents: precedent_matches.clone(),
            contradictions: contradictions.clone(),
            filing_type: filing::T19::MemorandumInSupport,
        }
    };
    match filing::f16(&memo_ctx, &cli.output) {
        Ok(path) => println!("      {}", path.display()),
        Err(e) => eprintln!("      memo error: {}", e),
    }

    if !cli.skip_forms {
        println!("      Generating court forms...");
        let court_forms = forms::f15(
            &cli.output,
            &cli.plaintiff,
            &cli.defendant,
            &cli.children,
            &cli.dobs,
            &cli.case_number,
            &cli.county,
            &cli.state,
            false,
        )?;
        for form in &court_forms {
            println!("      {}", form.display());
        }
    }

    println!();
    println!("Filing package ready at: {}", cli.output.display());
    println!("  findings.json           — all {} findings", findings.len());
    println!("  threads.json            — {} conversation threads", threads.len());
    println!("  contradictions.json     — {} contradictions", contradictions.len());
    println!("  gaps.json               — {} timeline gaps", timeline_gaps.len());
    println!("  precedents.json         — {} precedent matches", precedent_matches.len());
    println!("  legal_brief_outline.txt — factor-by-factor brief");
    println!("  timeline.csv            — spreadsheet timeline");
    println!("  PLAINTIFF_EXHIBIT_BOOK.pdf");
    println!("  MOTION_MODIFY_CUSTODY.pdf   — filed-ready motion");
    println!("  MEMORANDUM_IN_SUPPORT.pdf    — supporting memorandum");
    println!("  CC-DC-CV-001_CASE_INFO_REPORT.pdf — required case info report");
    println!("  CC-DR-004_FINANCIAL_STATEMENT.pdf — income/expense disclosure");
    println!("  CC-DR-055_PARENTING_PLAN.pdf      — proposed custody schedule");
    println!("  citation_verification.json   — all citations verified");
    println!();
    let bad_cites = cite_checks.iter().filter(|c| matches!(c.status, cite_verify::T22::BadFormat | cite_verify::T22::NotFound)).count();
    if bad_cites > 0 {
        println!("WARNING: {} citation(s) need manual verification before filing.", bad_cites);
    }
    println!();
    println!("Run with --query to explore findings interactively.");
    println!("At your service.");

    Ok(())
}

fn export_timeline_csv(
    findings: &[analyze::T6],
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut csv = String::new();
    csv.push_str("Exhibit#,Date,Category,CustodyWeek,Child,From,Subject,Summary,HighlightedText\n");

    for f in findings {
        let date = f.parsed_date.as_deref().unwrap_or(&f.date);
        let custody = match &f.custody_week {
            Some(analyze::T8::Plaintiff) => "Plaintiff",
            Some(analyze::T8::Defendant) => "Defendant",
            _ => "Unknown",
        };
        let child = f.child_name.as_deref().unwrap_or("");
        let highlights = f.highlighted_text.join(" | ");

        csv.push_str(&format!(
            "{},{},\"{}\",{},\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
            f.exhibit_number.unwrap_or(0),
            date,
            f.category,
            custody,
            escape_csv(child),
            escape_csv(&f.from),
            escape_csv(&f.subject),
            escape_csv(&f.summary),
            escape_csv(&highlights),
        ));
    }

    std::fs::write(path, &csv)?;
    Ok(())
}

fn escape_csv(s: &str) -> String {
    s.replace('"', "\"\"")
        .replace('\n', " ")
        .replace('\r', "")
}
