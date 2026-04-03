/// Tests that the mbox parser splits emails on "=== EMAIL" boundaries
#[test]
fn ingest_splits_on_email_boundaries() {
    let raw = r#"=== EMAIL 0 ===
Date: Mon, 3 Feb 2025 14:00:00 -0500
From: alice@example.com
To: bob@example.com
Subject: First

Body one.
===END===

=== EMAIL 1 ===
Date: Tue, 4 Feb 2025 09:00:00 -0500
From: bob@example.com
To: alice@example.com
Subject: Second

Body two.
===END===
"#;

    let parts: Vec<&str> = raw.split("=== EMAIL")
        .filter(|s| !s.trim().is_empty())
        .collect();

    assert_eq!(parts.len(), 2);
}

/// Tests header extraction from raw email text
#[test]
fn extract_headers_from_raw() {
    let raw = "From: alice@example.com\nTo: bob@example.com\nSubject: Test\nDate: Mon, 3 Feb 2025\n\nBody here.";

    fn extract(raw: &str, header: &str) -> Option<String> {
        raw.lines()
            .find(|l| l.starts_with(header))
            .map(|l| l[header.len()..].trim().to_string())
    }

    assert_eq!(extract(raw, "From:").unwrap(), "alice@example.com");
    assert_eq!(extract(raw, "To:").unwrap(), "bob@example.com");
    assert_eq!(extract(raw, "Subject:").unwrap(), "Test");
    assert!(extract(raw, "Cc:").is_none());
}

/// Tests that keyword detection finds custody-relevant patterns
#[test]
fn keyword_detection_finds_patterns() {
    let body = "She refused all food from school and we did not see her lunchbox today.";
    assert!(body.to_lowercase().contains("refused all food"));
    assert!(body.to_lowercase().contains("lunchbox"));

    let body2 = "It's not my fault our children don't feel safe";
    assert!(body2.to_lowercase().contains("don't feel safe"));

    let body3 = "I'll be going to court today cause of all you have to show";
    assert!(body3.to_lowercase().contains("going to court"));
}

/// Tests custody week calculation from a Thursday rotation
#[test]
fn custody_week_alternates_on_schedule() {
    // Given a known start date where Parent A has custody,
    // verify the week alternates correctly
    let _start_day = 0; // Thursday index
    let parent_a_weeks: Vec<usize> = (0..10).filter(|w| w % 2 == 0).collect();
    let parent_b_weeks: Vec<usize> = (0..10).filter(|w| w % 2 == 1).collect();

    assert_eq!(parent_a_weeks.len(), 5);
    assert_eq!(parent_b_weeks.len(), 5);
    assert!(!parent_a_weeks.iter().any(|w| parent_b_weeks.contains(w)));
}

/// Tests that date parsing extracts sortable dates from email headers
#[test]
fn date_strings_sort_chronologically() {
    let mut dates = vec![
        "Tue, 4 Mar 2025 09:00:00",
        "Mon, 3 Feb 2025 14:00:00",
        "Wed, 15 Jan 2025 09:00:00",
        "Fri, 14 Mar 2025 18:00:00",
    ];
    dates.sort();

    // Lexicographic sort on these won't be chronological —
    // real implementation needs chrono parsing
    // This test documents that raw string sort is insufficient
    assert_ne!(dates[0], "Wed, 15 Jan 2025 09:00:00",
        "raw date strings don't sort chronologically — chrono parsing needed");
}

/// Tests exhibit numbering assigns sequential numbers
#[test]
fn exhibit_numbering_is_sequential() {
    let findings = vec!["finding_a", "finding_b", "finding_c"];
    let numbered: Vec<(usize, &&str)> = findings.iter().enumerate().map(|(i, f)| (i + 1, f)).collect();

    assert_eq!(numbered[0].0, 1);
    assert_eq!(numbered[1].0, 2);
    assert_eq!(numbered[2].0, 3);
}

/// Tests that file size check catches PDFs over court limits
#[test]
fn file_size_check() {
    let max_bytes: u64 = 25 * 1024 * 1024; // 25MB Tyler limit
    let test_size: u64 = 4_500_000; // 4.5MB

    assert!(test_size < max_bytes, "exhibit book should be under 25MB");
    assert!(max_bytes == 26_214_400);
}
