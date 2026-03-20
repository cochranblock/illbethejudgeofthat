use std::io::{self, BufRead, Write};
use std::path::Path;
use crate::analyze::{Finding, CustodyParent};
use crate::thread::Thread;
use crate::contradict::Contradiction;
use crate::gaps::TimelineGap;

/// Interactive query mode — load JSON exports and filter/search findings
pub fn run_query(output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Load data
    let findings: Vec<Finding> = load_json(output_dir, "findings.json")?;
    let threads: Vec<Thread> = load_json(output_dir, "threads.json")?;
    let contradictions: Vec<Contradiction> = load_json(output_dir, "contradictions.json")?;
    let gaps: Vec<TimelineGap> = load_json(output_dir, "gaps.json")?;

    println!("Query mode. {} findings, {} threads, {} contradictions, {} gaps loaded.",
        findings.len(), threads.len(), contradictions.len(), gaps.len());
    println!("Commands: filter, exhibit, thread, contradictions, gaps, stats, help, quit");
    println!();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("query> ");
        stdout.flush()?;

        let mut line = String::new();
        if stdin.lock().read_line(&mut line)? == 0 {
            break;
        }

        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0] {
            "quit" | "exit" | "q" => break,
            "help" | "h" => print_help(),
            "stats" => print_stats(&findings, &threads, &contradictions, &gaps),
            "contradictions" => print_contradictions(&contradictions),
            "gaps" => print_gaps(&gaps),
            "exhibit" => {
                if let Some(num_str) = parts.get(1) {
                    if let Ok(num) = num_str.parse::<usize>() {
                        print_exhibit(&findings, num);
                    } else {
                        println!("Usage: exhibit <number>");
                    }
                } else {
                    println!("Usage: exhibit <number>");
                }
            }
            "thread" => {
                if let Some(id) = parts.get(1) {
                    print_thread(&threads, &findings, id);
                } else {
                    println!("Usage: thread <thread_id or index>");
                }
            }
            "filter" => {
                if parts.len() < 3 {
                    println!("Usage: filter <category|date|sender|custody|keyword> <value> [value2]");
                    continue;
                }
                match parts[1] {
                    "category" | "cat" => filter_category(&findings, parts[2]),
                    "date" => {
                        let end = parts.get(3).unwrap_or(&"9999-99-99");
                        filter_date(&findings, parts[2], end);
                    }
                    "sender" => filter_sender(&findings, parts[2]),
                    "custody" => filter_custody(&findings, parts[2]),
                    "keyword" | "kw" => {
                        let kw = parts[2..].join(" ");
                        filter_keyword(&findings, &kw);
                    }
                    "child" => filter_child(&findings, parts[2]),
                    _ => println!("Unknown filter: {}. Try: category, date, sender, custody, keyword, child", parts[1]),
                }
            }
            _ => println!("Unknown command: {}. Type 'help' for commands.", parts[0]),
        }
    }

    Ok(())
}

fn load_json<T: serde::de::DeserializeOwned>(dir: &Path, filename: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let path = dir.join(filename);
    if !path.exists() {
        println!("  {} not found, skipping", filename);
        return Ok(Vec::new());
    }
    let data = std::fs::read_to_string(&path)?;
    let items: Vec<T> = serde_json::from_str(&data)?;
    Ok(items)
}

fn print_help() {
    println!("Commands:");
    println!("  filter category <name>       — filter by finding category");
    println!("  filter date <start> [end]     — filter by date range (YYYY-MM-DD)");
    println!("  filter sender <name>          — filter by sender");
    println!("  filter custody <plaintiff|defendant> — filter by custody week");
    println!("  filter keyword <text>         — full-text search");
    println!("  filter child <name>           — filter by child name");
    println!("  exhibit <number>              — show specific exhibit");
    println!("  thread <id>                   — show thread by ID or index");
    println!("  contradictions                — list all contradictions");
    println!("  gaps                          — list all gaps");
    println!("  stats                         — show summary statistics");
    println!("  quit                          — exit query mode");
}

fn print_stats(findings: &[Finding], threads: &[Thread], contradictions: &[Contradiction], gaps: &[TimelineGap]) {
    println!("{}", crate::analyze::summarize_findings(findings));
    println!("{}", crate::thread::summarize_threads(threads));
    println!("{}", crate::contradict::summarize_contradictions(contradictions));
    println!("{}", crate::gaps::summarize_gaps(gaps));
}

fn print_exhibit(findings: &[Finding], num: usize) {
    if let Some(f) = findings.iter().find(|f| f.exhibit_number == Some(num)) {
        println!("--- EXHIBIT {} ---", num);
        println!("Date:     {}", f.parsed_date.as_deref().unwrap_or(&f.date));
        println!("Category: {}", f.category);
        println!("From:     {}", f.from);
        println!("To:       {}", f.to);
        println!("Subject:  {}", f.subject);
        if let Some(child) = &f.child_name {
            println!("Child:    {}", child);
        }
        println!("Custody:  {:?}", f.custody_week);
        println!("Summary:  {}", f.summary);
        if let Some(att) = &f.source_attachment {
            println!("Attach:   {}", att);
        }
        println!();
        if !f.highlighted_text.is_empty() {
            println!("Highlighted:");
            for h in &f.highlighted_text {
                println!("  > {}", h);
            }
            println!();
        }
        println!("{}", f.detail);
        println!("---");
    } else {
        println!("Exhibit {} not found", num);
    }
}

fn print_thread(threads: &[Thread], findings: &[Finding], id: &str) {
    // Try by index first
    let thread = if let Ok(idx) = id.parse::<usize>() {
        threads.get(idx)
    } else {
        threads.iter().find(|t| t.thread_id.contains(id))
    };

    if let Some(t) = thread {
        println!("--- Thread: {} ---", t.subject);
        println!("ID: {}", t.thread_id);
        println!("Messages: {}", t.message_count);
        println!("Participants: {}", t.participants.join(", "));
        println!("Date range: {} to {}", t.start_date.as_deref().unwrap_or("?"), t.end_date.as_deref().unwrap_or("?"));
        println!();

        for &email_idx in &t.emails {
            let related: Vec<&Finding> = findings.iter()
                .filter(|f| f.source_email_index == Some(email_idx))
                .collect();
            if !related.is_empty() {
                for f in related {
                    println!("  [Exhibit {}] {} | {} | {}", f.exhibit_number.unwrap_or(0), f.category, f.from, f.summary);
                }
            }
        }
        println!("---");
    } else {
        println!("Thread not found: {}", id);
    }
}

fn print_contradictions(contradictions: &[Contradiction]) {
    for c in contradictions {
        println!("  Exhibits {} vs {} | {} | {}", c.exhibit_a, c.exhibit_b, c.contradiction_type, c.explanation);
    }
    println!("{} total", contradictions.len());
}

fn print_gaps(gaps: &[TimelineGap]) {
    for g in gaps {
        println!("  {} | {} to {} ({} days) | {} | {}", g.gap_type, g.start_date, g.end_date, g.duration_days, g.expected_source, g.significance);
    }
    println!("{} total", gaps.len());
}

fn filter_category(findings: &[Finding], cat: &str) {
    let cat_lower = cat.to_lowercase();
    let matches: Vec<&Finding> = findings.iter()
        .filter(|f| f.category.to_string().to_lowercase().contains(&cat_lower))
        .collect();
    print_finding_list(&matches);
}

fn filter_date(findings: &[Finding], start: &str, end: &str) {
    let matches: Vec<&Finding> = findings.iter()
        .filter(|f| {
            if let Some(d) = &f.parsed_date {
                d.as_str() >= start && d.as_str() <= end
            } else {
                false
            }
        })
        .collect();
    print_finding_list(&matches);
}

fn filter_sender(findings: &[Finding], sender: &str) {
    let sender_lower = sender.to_lowercase();
    let matches: Vec<&Finding> = findings.iter()
        .filter(|f| f.from.to_lowercase().contains(&sender_lower))
        .collect();
    print_finding_list(&matches);
}

fn filter_custody(findings: &[Finding], who: &str) {
    let target = match who.to_lowercase().as_str() {
        "plaintiff" | "p" => CustodyParent::Plaintiff,
        "defendant" | "d" => CustodyParent::Defendant,
        _ => {
            println!("Use: plaintiff or defendant");
            return;
        }
    };
    let matches: Vec<&Finding> = findings.iter()
        .filter(|f| f.custody_week.as_ref() == Some(&target))
        .collect();
    print_finding_list(&matches);
}

fn filter_keyword(findings: &[Finding], keyword: &str) {
    let kw_lower = keyword.to_lowercase();
    let matches: Vec<&Finding> = findings.iter()
        .filter(|f| {
            f.detail.to_lowercase().contains(&kw_lower)
                || f.summary.to_lowercase().contains(&kw_lower)
                || f.subject.to_lowercase().contains(&kw_lower)
                || f.highlighted_text.iter().any(|h| h.to_lowercase().contains(&kw_lower))
        })
        .collect();
    print_finding_list(&matches);
}

fn filter_child(findings: &[Finding], name: &str) {
    let name_lower = name.to_lowercase();
    let matches: Vec<&Finding> = findings.iter()
        .filter(|f| f.child_name.as_ref().map(|c| c.to_lowercase().contains(&name_lower)).unwrap_or(false))
        .collect();
    print_finding_list(&matches);
}

fn print_finding_list(findings: &[&Finding]) {
    for f in findings {
        println!("  [{:>3}] {} | {:24} | {} | {}",
            f.exhibit_number.unwrap_or(0),
            f.parsed_date.as_deref().unwrap_or("?"),
            f.category.to_string(),
            truncate(&f.from, 25),
            truncate(&f.summary, 50),
        );
    }
    println!("{} results", findings.len());
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max { s.to_string() } else { format!("{}...", &s[..max]) }
}
