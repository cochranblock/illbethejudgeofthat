use std::path::PathBuf;

#[test]
fn sample_emails_parse() {
    let sample = PathBuf::from("examples/sample_emails.txt");
    assert!(sample.exists());

    let content = std::fs::read_to_string(&sample).unwrap();
    let emails: Vec<&str> = content.split("=== EMAIL").filter(|s| !s.trim().is_empty()).collect();

    assert!(emails.len() >= 10);
}

#[test]
fn sample_contains_school_communications() {
    let content = std::fs::read_to_string("examples/sample_emails.txt").unwrap();
    assert!(content.contains("@silverleafelem.example.org"));
}

#[test]
fn sample_contains_medical_records() {
    let content = std::fs::read_to_string("examples/sample_emails.txt").unwrap();
    assert!(content.contains("@maplewoodpeds.example.org"));
}

#[test]
fn sample_contains_interparental_conflict() {
    let content = std::fs::read_to_string("examples/sample_emails.txt").unwrap();
    let has_both = content.contains("msantos@example.com") && content.contains("dsantos@example.com");
    assert!(has_both);
}

#[test]
fn sample_input_valid_json() {
    let content = std::fs::read_to_string("examples/sample_input.json").unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert!(parsed["plaintiff"].is_string());
    assert!(parsed["defendant"].is_string());
    assert!(parsed["children"].is_array());
}

#[test]
fn sample_daily_report_exists() {
    let report = PathBuf::from("examples/sample_daily_report.txt");
    assert!(report.exists());
    let content = std::fs::read_to_string(&report).unwrap();
    assert!(content.contains("Daily Report"));
}

#[test]
fn no_real_pii_in_samples() {
    let emails = std::fs::read_to_string("examples/sample_emails.txt").unwrap();
    let input = std::fs::read_to_string("examples/sample_input.json").unwrap();
    let report = std::fs::read_to_string("examples/sample_daily_report.txt").unwrap();
    let all = format!("{}{}{}", emails, input, report).to_lowercase();

    // All domains should be example.com or example.org
    assert!(!all.contains("@gmail.com"));
    assert!(!all.contains("@aacps.org"));
    assert!(!all.contains("@chimes.org"));
    assert!(!all.contains("@maryland.gov"));
}
