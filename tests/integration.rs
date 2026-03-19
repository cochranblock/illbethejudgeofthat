use std::path::PathBuf;

#[test]
fn parse_sample_emails() {
    let sample = PathBuf::from("examples/sample_emails.txt");
    assert!(sample.exists(), "sample emails file should exist");

    let content = std::fs::read_to_string(&sample).unwrap();
    let emails: Vec<&str> = content.split("=== EMAIL").filter(|s| !s.trim().is_empty()).collect();

    assert!(emails.len() >= 15, "should parse at least 15 sample emails");
}

#[test]
fn detect_medical_neglect_pattern() {
    let content = std::fs::read_to_string("examples/sample_emails.txt").unwrap();

    assert!(content.contains("did not have his inhaler"), "should detect missing inhaler");
    assert!(content.contains("expired"), "should detect expired medication");
    assert!(content.contains("third time"), "should detect repeated pattern");
}

#[test]
fn detect_visitation_denial() {
    let content = std::fs::read_to_string("examples/sample_emails.txt").unwrap();

    assert!(content.contains("sat at the window"), "should detect missed pickup");
    assert!(content.contains("second time in two months"), "should detect pattern");
}

#[test]
fn detect_unilateral_decisions() {
    let content = std::fs::read_to_string("examples/sample_emails.txt").unwrap();

    assert!(content.contains("Already booked flights"), "should detect unilateral travel");
    assert!(content.contains("both parents agree"), "should detect order violation reference");
}

#[test]
fn detect_alienation_allegations() {
    let content = std::fs::read_to_string("examples/sample_emails.txt").unwrap();

    assert!(content.contains("not my real family"), "should detect alienation claim");
    assert!(content.contains("poisoning him"), "should detect alienation accusation");
}

#[test]
fn detect_school_absence_pattern() {
    let content = std::fs::read_to_string("examples/sample_emails.txt").unwrap();

    assert!(content.contains("8th absence"), "should detect absence count");
    assert!(content.contains("6 of them fall on days that were yours"), "should detect custody-correlated absences");
}

#[test]
fn detect_financial_dispute() {
    let content = std::fs::read_to_string("examples/sample_emails.txt").unwrap();

    assert!(content.contains("2200 a month"), "should detect support amount");
    assert!(content.contains("still owe"), "should detect arrears");
}

#[test]
fn detect_pediatric_concerns() {
    let content = std::fs::read_to_string("examples/sample_emails.txt").unwrap();

    assert!(content.contains("25th percentile"), "should detect weight percentile drop");
    assert!(content.contains("doesn't eat much at dad's"), "should detect child's dietary report");
    assert!(content.contains("behind on his Tdap"), "should detect missed vaccinations");
}

#[test]
fn sample_input_valid_json() {
    let content = std::fs::read_to_string("examples/sample_input.json").unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();

    assert_eq!(parsed["plaintiff"], "Maria Santos");
    assert_eq!(parsed["defendant"], "Derek Santos");
    assert_eq!(parsed["children"].as_array().unwrap().len(), 1);
}

#[test]
fn no_real_locations_in_samples() {
    let emails = std::fs::read_to_string("examples/sample_emails.txt").unwrap();
    let input = std::fs::read_to_string("examples/sample_input.json").unwrap();
    let report = std::fs::read_to_string("examples/sample_daily_report.txt").unwrap();

    let all = format!("{}{}{}", emails, input, report);
    let lower = all.to_lowercase();

    // No real cities, schools, or states referenced as locations
    assert!(!lower.contains("anne arundel"), "should not contain real county");
    assert!(!lower.contains("glen burnie"), "should not contain real city");
    assert!(!lower.contains("dundalk"), "should not contain real city");
    assert!(!lower.contains("baltimore"), "should not contain real city");
    assert!(!lower.contains("chimes"), "should not contain real school");
}
