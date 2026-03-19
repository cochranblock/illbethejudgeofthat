use clap::Parser;
use std::path::PathBuf;

mod ingest;
mod parse;
mod analyze;
mod exhibit;
mod forms;

#[derive(Parser)]
#[command(name = "illbethejudgeofthat")]
#[command(about = "Pro se custody case builder. Takeout → Courtroom.")]
struct Cli {
    /// Path to Google Takeout mbox file or extracted directory
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

    /// Case number
    #[arg(long)]
    case_number: Option<String>,

    /// State jurisdiction
    #[arg(long, default_value = "MD")]
    state: String,

    /// County
    #[arg(long, default_value = "Anne Arundel")]
    county: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("illbethejudgeofthat v0.1.0");
    println!("========================");
    println!();
    println!("Plaintiff:  {}", cli.plaintiff);
    println!("Defendant:  {}", cli.defendant);
    println!("Children:   {}", cli.children);
    println!("Input:      {}", cli.input.display());
    println!("Output:     {}", cli.output.display());
    println!("State:      {}", cli.state);
    println!("County:     {}", cli.county);
    println!();

    // Stage 1: Ingest
    println!("[1/5] Ingesting email archive...");
    let emails = ingest::ingest_mbox(&cli.input)?;
    println!("      {} emails parsed", emails.len());

    // Stage 2: Parse attachments
    println!("[2/5] Extracting attachments...");
    let attachments = parse::extract_attachments(&emails, &cli.output)?;
    println!("      {} attachments extracted", attachments.len());

    // Stage 3: Analyze
    println!("[3/5] Analyzing for inconsistencies...");
    let findings = analyze::analyze(
        &emails,
        &attachments,
        &cli.plaintiff,
        &cli.defendant,
        &cli.children,
        &cli.schedule,
    )?;
    println!("      {} findings identified", findings.len());

    // Stage 4: Build exhibit book
    println!("[4/5] Building exhibit book...");
    let exhibit_path = exhibit::build_exhibit_book(
        &findings,
        &cli.output,
        &cli.plaintiff,
        &cli.defendant,
        &cli.case_number,
        &cli.county,
        &cli.state,
    )?;
    println!("      {}", exhibit_path.display());

    // Stage 5: Generate forms
    println!("[5/5] Generating court forms...");
    let forms = forms::generate_forms(
        &cli.output,
        &cli.plaintiff,
        &cli.defendant,
        &cli.children,
        &cli.dobs,
        &cli.case_number,
        &cli.county,
        &cli.state,
    )?;
    for form in &forms {
        println!("      {}", form.display());
    }

    println!();
    println!("Filing package ready at: {}", cli.output.display());
    println!("Sign, upload to Tyler, serve the other party.");
    println!();
    println!("At your service.");

    Ok(())
}
