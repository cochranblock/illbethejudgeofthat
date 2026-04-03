// Unlicense — cochranblock.org
// Tests that verify correctness of custody case analysis logic.
// These test real functions with known inputs and expected outputs.

use chrono::NaiveDate;
use illbethejudgeofthat::analyze::{f4, f5, T6, T7, T8};
use illbethejudgeofthat::contradict::{f7, T11};
use illbethejudgeofthat::gaps::{f9, GapType};
use illbethejudgeofthat::thread::f2;
use illbethejudgeofthat::precedent::f11;
use illbethejudgeofthat::ingest::T0;

/// Build a test email with sensible defaults.
fn email(index: usize, from: &str, to: &str, subject: &str, date: &str, body: &str) -> T0 {
    T0 {
        index,
        from: from.into(),
        to: to.into(),
        cc: String::new(),
        subject: subject.into(),
        date: date.into(),
        body: body.into(),
        labels: vec![],
        message_id: format!("msg-{index}@test"),
        in_reply_to: String::new(),
        thread_id: String::new(),
        attachments: vec![],
    }
}

// Known custody start: Thursday Jan 2 2025 = plaintiff's week.
fn custody_start() -> NaiveDate {
    NaiveDate::from_ymd_opt(2025, 1, 2).unwrap()
}

// ============================================================
// DATE PARSING (f4) — every format the pipeline must handle
// ============================================================

#[test]
fn f4_parses_rfc2822_with_timezone() {
    let d = f4("Wed, 01 Jan 2025 12:00:00 -0500");
    assert_eq!(d, Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()));
}

#[test]
fn f4_parses_rfc2822_without_timezone() {
    let d = f4("Wed, 01 Jan 2025 12:00:00");
    assert_eq!(d, Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()));
}

#[test]
fn f4_parses_day_month_year_no_weekday() {
    let d = f4("15 Mar 2025 09:30:00");
    assert_eq!(d, Some(NaiveDate::from_ymd_opt(2025, 3, 15).unwrap()));
}

#[test]
fn f4_parses_short_format() {
    let d = f4("Fri, 14 Mar 2025");
    assert_eq!(d, Some(NaiveDate::from_ymd_opt(2025, 3, 14).unwrap()));
}

#[test]
fn f4_parses_regex_fallback() {
    // Non-standard format that none of the chrono patterns match
    let d = f4("3 Jan 2025 at noon");
    assert_eq!(d, Some(NaiveDate::from_ymd_opt(2025, 1, 3).unwrap()));
}

#[test]
fn f4_returns_none_for_garbage() {
    assert_eq!(f4("not a date"), None);
    assert_eq!(f4(""), None);
    assert_eq!(f4("2025-01-01"), None); // ISO 8601 not supported
}

// ============================================================
// CUSTODY WEEK CALCULATION — verified for every day of the week
// ============================================================

#[test]
fn custody_week_thursday_is_plaintiff() {
    // Jan 2 2025 is a Thursday, plaintiff's start date → plaintiff's week
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Thu, 02 Jan 2025 09:00:00",
            "Daily Chimes report: child had a good day"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    assert!(!findings.is_empty(), "should detect daily report");
    let f = &findings[0];
    assert_eq!(f.custody_week, Some(T8::Plaintiff),
        "Thursday Jan 2 (start date) must be plaintiff's week");
}

#[test]
fn custody_week_friday_same_week_as_thursday() {
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Fri, 03 Jan 2025 09:00:00",
            "Daily Chimes report: another good day"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    assert_eq!(findings[0].custody_week, Some(T8::Plaintiff),
        "Friday Jan 3 is same week as Thursday Jan 2 → plaintiff");
}

#[test]
fn custody_week_wednesday_before_thursday_is_previous_week() {
    // Wed Jan 8 → its Thursday is Jan 2 (Thu Jan 2 + 6 days = Wed Jan 8? No.)
    // Actually: Wed Jan 8 → days_since_thursday = (2+4)%7 = 6 → this_thursday = Jan 8 - 6 = Jan 2
    // weeks_diff = 0 → even → Plaintiff
    // But wait — Jan 9 is next Thursday. Wed Jan 8 belongs to Jan 2 Thursday's week.
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Wed, 08 Jan 2025 09:00:00",
            "Daily Chimes report"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    assert_eq!(findings[0].custody_week, Some(T8::Plaintiff),
        "Wed Jan 8 maps to Thu Jan 2 week → plaintiff");
}

#[test]
fn custody_week_alternates_correctly() {
    // Jan 2 (Thu) = plaintiff. Jan 9 (Thu) = defendant.
    // Jan 9 is 1 week after start → odd → defendant.
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Thu, 09 Jan 2025 09:00:00",
            "Daily Chimes report"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    assert_eq!(findings[0].custody_week, Some(T8::Defendant),
        "Thu Jan 9 is 1 week after start → defendant's week");
}

#[test]
fn custody_week_monday_maps_to_prior_thursday() {
    // Mon Jan 6 → days_since_thursday = (0+4)%7 = 4 → this_thursday = Jan 6 - 4 = Jan 2
    // weeks_diff = 0 → plaintiff
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Mon, 06 Jan 2025 09:00:00",
            "Daily Chimes report"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    assert_eq!(findings[0].custody_week, Some(T8::Plaintiff),
        "Mon Jan 6 maps to Thu Jan 2 → plaintiff");
}

#[test]
fn custody_week_sunday_maps_to_prior_thursday() {
    // Sun Jan 5 → (6+4)%7 = 3 → Jan 5 - 3 = Jan 2 → plaintiff
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Sun, 05 Jan 2025 09:00:00",
            "Daily Chimes report"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    assert_eq!(findings[0].custody_week, Some(T8::Plaintiff),
        "Sun Jan 5 maps to Thu Jan 2 → plaintiff");
}

#[test]
fn custody_week_second_week_all_days_defendant() {
    // Thu Jan 9 through Wed Jan 15 should all be defendant's week
    let dates = [
        "Thu, 09 Jan 2025 09:00:00",
        "Fri, 10 Jan 2025 09:00:00",
        "Sat, 11 Jan 2025 09:00:00",
        "Sun, 12 Jan 2025 09:00:00",
        "Mon, 13 Jan 2025 09:00:00",
        "Tue, 14 Jan 2025 09:00:00",
        "Wed, 15 Jan 2025 09:00:00",
    ];
    for (i, date) in dates.iter().enumerate() {
        let emails = vec![
            email(0, "school@chimes.org", "dad@test.com",
                "Daily Communication", date,
                "Daily Chimes report"),
        ];
        let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
        assert_eq!(findings[0].custody_week, Some(T8::Defendant),
            "Day {} ({}) should be defendant's week", i, date);
    }
}

// ============================================================
// FINDING DETECTION — each category with known trigger input
// ============================================================

#[test]
fn detects_food_refusal() {
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "Lunch Report", "Mon, 06 Jan 2025 12:00:00",
            "Hazel refused all food at lunch today. She would not eat the provided meal."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let food: Vec<&T6> = findings.iter().filter(|f| f.category == T7::FoodRecord).collect();
    assert!(!food.is_empty(), "should detect food refusal from 'refused all food'");
}

#[test]
fn detects_custody_interference() {
    let emails = vec![
        email(0, "mom@test.com", "dad@test.com",
            "Lunch", "Tue, 07 Jan 2025 08:00:00",
            "I already provided food. Mom sent lunch with her today."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let ci: Vec<&T6> = findings.iter().filter(|f| f.category == T7::CustodyInterference).collect();
    assert!(!ci.is_empty(), "'mom sent' from defendant should trigger custody interference");
}

#[test]
fn does_not_detect_custody_interference_from_plaintiff() {
    // Same phrase but from plaintiff — should NOT trigger
    let emails = vec![
        email(0, "dad@test.com", "mom@test.com",
            "Lunch", "Tue, 07 Jan 2025 08:00:00",
            "Mom sent lunch with her today."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let ci: Vec<&T6> = findings.iter().filter(|f| f.category == T7::CustodyInterference).collect();
    assert!(ci.is_empty(), "plaintiff saying 'mom sent' should NOT be custody interference");
}

#[test]
fn detects_alienation_from_defendant_only() {
    let emails = vec![
        email(0, "mom@test.com", "dad@test.com",
            "Visit", "Wed, 08 Jan 2025 18:00:00",
            "The kids don't feel safe going to your house anymore."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let al: Vec<&T6> = findings.iter().filter(|f| f.category == T7::Alienation).collect();
    assert!(!al.is_empty(), "'don't feel safe' from defendant should trigger alienation");
}

#[test]
fn no_alienation_from_plaintiff() {
    let emails = vec![
        email(0, "dad@test.com", "mom@test.com",
            "Visit", "Wed, 08 Jan 2025 18:00:00",
            "The kids don't feel safe there."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let al: Vec<&T6> = findings.iter().filter(|f| f.category == T7::Alienation).collect();
    assert!(al.is_empty(), "plaintiff saying 'don't feel safe' should NOT be alienation");
}

#[test]
fn detects_court_threat() {
    let emails = vec![
        email(0, "mom@test.com", "dad@test.com",
            "Custody", "Thu, 02 Jan 2025 15:00:00",
            "I'm going to court about this. My lawyer will be in touch."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let ct: Vec<&T6> = findings.iter().filter(|f| f.category == T7::CourtThreat).collect();
    assert!(!ct.is_empty(), "'going to court' + 'my lawyer' should trigger court threat");
}

#[test]
fn detects_iep_violation_with_idea_tags() {
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "IEP Annual Review", "Mon, 06 Jan 2025 10:00:00",
            "The placement decision was made without prior written notice. \
             The student's BIP needs updating. Elopement occurred twice this week."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let iep: Vec<&T6> = findings.iter().filter(|f| f.category == T7::IepViolation).collect();
    assert!(!iep.is_empty(), "IEP email with IDEA keywords should trigger IepViolation");
    let summary = &iep[0].summary;
    assert!(summary.contains("300.116") || summary.contains("Placement"),
        "should tag IDEA §300.116 for 'placement': got {}", summary);
    assert!(summary.contains("300.503") || summary.contains("Prior Written Notice"),
        "should tag IDEA §300.503 for 'prior written notice': got {}", summary);
    assert!(summary.contains("300.324") || summary.contains("BIP"),
        "should tag IDEA §300.324 for 'elopement' or 'bip': got {}", summary);
}

#[test]
fn detects_behavioral_incident_from_school_only() {
    // School sender → should detect
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "Behavior Report", "Mon, 06 Jan 2025 14:00:00",
            "Hazel had a meltdown during recess and was hitting other students."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let bi: Vec<&T6> = findings.iter().filter(|f| f.category == T7::BehavioralIncident).collect();
    assert!(!bi.is_empty(), "school sender with 'meltdown'+'hitting' should trigger behavioral incident");

    // Non-school sender → should NOT detect
    let emails2 = vec![
        email(0, "mom@test.com", "dad@test.com",
            "Behavior", "Mon, 06 Jan 2025 14:00:00",
            "She had a meltdown at home and was hitting."),
    ];
    let findings2 = f5(&emails2, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let bi2: Vec<&T6> = findings2.iter().filter(|f| f.category == T7::BehavioralIncident).collect();
    assert!(bi2.is_empty(), "non-school sender should NOT trigger behavioral incident");
}

#[test]
fn detects_deescalation_from_plaintiff_only() {
    let emails = vec![
        email(0, "dad@test.com", "mom@test.com",
            "Schedule", "Thu, 02 Jan 2025 10:00:00",
            "I understand your concerns. Let's work together for the kids' best interest."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let de: Vec<&T6> = findings.iter().filter(|f| f.category == T7::DeEscalation).collect();
    assert!(!de.is_empty(), "plaintiff saying 'i understand' should trigger de-escalation");
}

#[test]
fn detects_admission_against_interest_from_defendant() {
    let emails = vec![
        email(0, "mom@test.com", "dad@test.com",
            "Oops", "Fri, 03 Jan 2025 10:00:00",
            "I forgot to give her the medication. I should have checked the bag."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let ad: Vec<&T6> = findings.iter().filter(|f| f.category == T7::AdmissionAgainstInterest).collect();
    assert!(!ad.is_empty(), "defendant 'i forgot' + 'i should have' should trigger admission");
}

#[test]
fn detects_school_absence() {
    let emails = vec![
        email(0, "office@aacps.org", "dad@test.com",
            "Child Absence Notification", "Mon, 06 Jan 2025 08:00:00",
            "Your child was absent from school today."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let sa: Vec<&T6> = findings.iter().filter(|f| f.category == T7::SchoolAbsence).collect();
    assert!(!sa.is_empty(), "'child absence' in subject + 'absent from school' in body should trigger");
}

#[test]
fn detects_medication_issue() {
    let emails = vec![
        email(0, "nurse@chimes.org", "dad@test.com",
            "PRN Medication Notice", "Tue, 07 Jan 2025 11:00:00",
            "Medication was administered at 10:30 AM per doctor's orders."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let med: Vec<&T6> = findings.iter().filter(|f| f.category == T7::MedicationIssue).collect();
    assert!(!med.is_empty(), "'PRN medication' in subject should trigger medication issue");
}

#[test]
fn detects_weight_tracking() {
    let emails = vec![
        email(0, "nurse@chimes.org", "dad@test.com",
            "Health Check", "Wed, 08 Jan 2025 09:00:00",
            "Hazel weighed 52 lbs at today's checkup."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let wt: Vec<&T6> = findings.iter().filter(|f| f.category == T7::WeightTracking).collect();
    assert!(!wt.is_empty(), "'52 lbs' should trigger weight tracking");
}

#[test]
fn detects_communication_block() {
    let emails = vec![
        email(0, "admin@chimes.org", "dad@test.com",
            "Letter Regarding Communication Channels",
            "Mon, 06 Jan 2025 09:00:00",
            "We are writing regarding access to parent communication portals."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let cb: Vec<&T6> = findings.iter().filter(|f| f.category == T7::CommunicationBlock).collect();
    assert!(!cb.is_empty(), "'letter regarding' + 'communication channels' should trigger");
}

#[test]
fn detects_state_complaint() {
    let emails = vec![
        email(0, "msde@maryland.gov", "dad@test.com",
            "State Complaint 26-154", "Fri, 10 Jan 2025 10:00:00",
            "Your state complaint has been received and assigned."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let sc: Vec<&T6> = findings.iter().filter(|f| f.category == T7::StateComplaint).collect();
    assert!(!sc.is_empty(), "'state complaint' + '26-154' should trigger state complaint");
}

// ============================================================
// EXHIBIT NUMBERING — sequential, 1-based
// ============================================================

#[test]
fn exhibit_numbers_are_sequential_and_one_based() {
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Mon, 06 Jan 2025 09:00:00",
            "Daily Chimes report: Monday"),
        email(1, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Tue, 07 Jan 2025 09:00:00",
            "Daily Chimes report: Tuesday"),
        email(2, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Wed, 08 Jan 2025 09:00:00",
            "Daily Chimes report: Wednesday"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    for (i, f) in findings.iter().enumerate() {
        assert_eq!(f.exhibit_number, Some(i + 1),
            "exhibit {} should be numbered {}", i, i + 1);
    }
}

// ============================================================
// FINDINGS SORTED BY DATE
// ============================================================

#[test]
fn findings_sorted_chronologically() {
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Wed, 08 Jan 2025 09:00:00",
            "Daily Chimes report: Wednesday"),
        email(1, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Mon, 06 Jan 2025 09:00:00",
            "Daily Chimes report: Monday"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    assert!(findings.len() >= 2);
    assert!(findings[0].parsed_date <= findings[1].parsed_date,
        "findings must be sorted by date: {:?} should be <= {:?}",
        findings[0].parsed_date, findings[1].parsed_date);
}

// ============================================================
// CHILD NAME DETECTION
// ============================================================

#[test]
fn child_name_detected_in_body() {
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Mon, 06 Jan 2025 09:00:00",
            "Daily Chimes report: Hazel had a great day at school."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    assert!(!findings.is_empty());
    assert_eq!(findings[0].child_name, Some("hazel".into()),
        "child name 'hazel' should be detected in body");
}

// ============================================================
// CONTRADICTION DETECTION (f7) — known pairs
// ============================================================

#[test]
fn contradiction_food_refusal_pattern() {
    // Same date: school reports food refusal + parent claims food provided
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "Lunch", "Mon, 06 Jan 2025 12:00:00",
            "Hazel refused all food at lunch today."),
        email(1, "mom@test.com", "dad@test.com",
            "Lunch", "Mon, 06 Jan 2025 08:00:00",
            "I already sent food. Mom provided lunch for her."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let threads = f2(&emails);
    let contradictions = f7(&findings, &threads);

    let food_pattern: Vec<_> = contradictions.iter()
        .filter(|c| c.contradiction_type == T11::FoodRefusalPattern)
        .collect();
    assert!(!food_pattern.is_empty(),
        "same-date food refusal (school) + custody interference (parent) = contradiction");
}

#[test]
fn no_contradiction_without_matching_pair() {
    // Only food refusal, no parent claim → no contradiction
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "Lunch", "Mon, 06 Jan 2025 12:00:00",
            "Hazel refused all food at lunch today."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let threads = f2(&emails);
    let contradictions = f7(&findings, &threads);
    let food: Vec<_> = contradictions.iter()
        .filter(|c| c.contradiction_type == T11::FoodRefusalPattern)
        .collect();
    assert!(food.is_empty(), "no contradiction without matching pair");
}

// ============================================================
// THREAD RECONSTRUCTION (f2) — 3-tier matching
// ============================================================

#[test]
fn threads_by_message_id_chain() {
    let mut e1 = email(0, "dad@test.com", "mom@test.com",
        "Schedule", "Mon, 06 Jan 2025 10:00:00", "Can we swap weekends?");
    e1.message_id = "msg-001@test".into();

    let mut e2 = email(1, "mom@test.com", "dad@test.com",
        "Re: Schedule", "Mon, 06 Jan 2025 11:00:00", "No.");
    e2.message_id = "msg-002@test".into();
    e2.in_reply_to = "msg-001@test".into();

    let threads = f2(&[e1, e2]);
    // Should be grouped into one thread via In-Reply-To chain
    let multi: Vec<_> = threads.iter().filter(|t| t.message_count >= 2).collect();
    assert!(!multi.is_empty(), "reply chain should merge into single thread");
    assert_eq!(multi[0].message_count, 2);
}

#[test]
fn threads_by_gmail_thread_id() {
    let mut e1 = email(0, "dad@test.com", "mom@test.com",
        "Pickup", "Mon, 06 Jan 2025 10:00:00", "I'll pick up at 3.");
    e1.thread_id = "GMTHRID-123".into();
    e1.message_id = "a@test".into();

    let mut e2 = email(1, "mom@test.com", "dad@test.com",
        "Re: Pickup", "Mon, 06 Jan 2025 11:00:00", "Fine.");
    e2.thread_id = "GMTHRID-123".into();
    e2.message_id = "b@test".into();

    let threads = f2(&[e1, e2]);
    let multi: Vec<_> = threads.iter().filter(|t| t.message_count >= 2).collect();
    assert!(!multi.is_empty(), "same X-GM-THRID should merge into single thread");
}

#[test]
fn threads_by_subject_within_7_days() {
    // Same normalized subject, no message-id chain, within 7 days
    let e1 = email(0, "dad@test.com", "teacher@chimes.org",
        "Hazel's IEP", "Mon, 06 Jan 2025 10:00:00", "Question about placement.");
    let e2 = email(1, "teacher@chimes.org", "dad@test.com",
        "Re: Hazel's IEP", "Wed, 08 Jan 2025 14:00:00", "We'll discuss at the meeting.");

    let threads = f2(&[e1, e2]);
    let multi: Vec<_> = threads.iter().filter(|t| t.message_count >= 2).collect();
    assert!(!multi.is_empty(),
        "same subject within 7 days should merge (Tier 3 matching)");
}

#[test]
fn threads_not_merged_beyond_7_days() {
    let e1 = email(0, "dad@test.com", "teacher@chimes.org",
        "Hazel's IEP", "Mon, 06 Jan 2025 10:00:00", "Question.");
    let e2 = email(1, "teacher@chimes.org", "dad@test.com",
        "Re: Hazel's IEP", "Tue, 21 Jan 2025 14:00:00", "Reply.");

    let threads = f2(&[e1, e2]);
    let multi: Vec<_> = threads.iter().filter(|t| t.message_count >= 2).collect();
    assert!(multi.is_empty(),
        "same subject but >7 days apart should NOT merge");
}

// ============================================================
// GAP DETECTION (f9) — daily reports and communication silence
// ============================================================

#[test]
fn gap_detects_missing_daily_reports() {
    // Reports on Mon, Tue, skip Wed+Thu, report on Fri → gap of 2 weekdays
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Mon, 06 Jan 2025 09:00:00",
            "Daily Chimes report"),
        email(1, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Tue, 07 Jan 2025 09:00:00",
            "Daily Chimes report"),
        // Wed 8 and Thu 9 missing
        email(2, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Fri, 10 Jan 2025 09:00:00",
            "Daily Chimes report"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let threads = f2(&emails);
    let gaps = f9(&emails, &findings, &threads);

    let missing: Vec<_> = gaps.iter()
        .filter(|g| g.gap_type == GapType::DailyReportMissing)
        .collect();
    assert!(!missing.is_empty(), "should detect Wed+Thu missing daily reports");
    assert_eq!(missing[0].duration_days, 2, "gap should be 2 weekdays");
}

#[test]
fn gap_skips_weekends() {
    // Report on Friday, next report on Monday → no gap (weekend excluded)
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Fri, 03 Jan 2025 09:00:00",
            "Daily Chimes report"),
        email(1, "school@chimes.org", "dad@test.com",
            "Daily Communication", "Mon, 06 Jan 2025 09:00:00",
            "Daily Chimes report"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let threads = f2(&emails);
    let gaps = f9(&emails, &findings, &threads);

    let missing: Vec<_> = gaps.iter()
        .filter(|g| g.gap_type == GapType::DailyReportMissing)
        .collect();
    assert!(missing.is_empty(), "Fri→Mon has no weekday gap (weekend excluded)");
}

#[test]
fn gap_detects_communication_silence() {
    // Sender has 3+ emails with a 5+ day gap between two
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "Update 1", "Mon, 06 Jan 2025 09:00:00", "Status update."),
        email(1, "teacher@chimes.org", "dad@test.com",
            "Update 2", "Tue, 07 Jan 2025 09:00:00", "Another update."),
        // 7-day gap
        email(2, "teacher@chimes.org", "dad@test.com",
            "Update 3", "Tue, 14 Jan 2025 09:00:00", "Back after silence."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let threads = f2(&emails);
    let gaps = f9(&emails, &findings, &threads);

    let silence: Vec<_> = gaps.iter()
        .filter(|g| g.gap_type == GapType::CommunicationSilence)
        .collect();
    assert!(!silence.is_empty(), "7-day gap from same sender (3+ emails) should trigger silence");
    assert!(silence[0].duration_days >= 5, "silence gap should be >= 5 days");
}

#[test]
fn gap_no_silence_under_5_days() {
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "A", "Mon, 06 Jan 2025 09:00:00", "a"),
        email(1, "teacher@chimes.org", "dad@test.com",
            "B", "Wed, 08 Jan 2025 09:00:00", "b"),
        email(2, "teacher@chimes.org", "dad@test.com",
            "C", "Fri, 10 Jan 2025 09:00:00", "c"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let threads = f2(&emails);
    let gaps = f9(&emails, &findings, &threads);

    let silence: Vec<_> = gaps.iter()
        .filter(|g| g.gap_type == GapType::CommunicationSilence)
        .collect();
    assert!(silence.is_empty(), "2-day gaps should NOT trigger silence (threshold is 5)");
}

#[test]
fn gap_detects_abandoned_thread() {
    let mut e1 = email(0, "dad@test.com", "teacher@chimes.org",
        "IEP Question", "Mon, 06 Jan 2025 10:00:00",
        "When is the IEP meeting scheduled?");
    e1.message_id = "q1@test".into();

    let mut e2 = email(1, "teacher@chimes.org", "dad@test.com",
        "Re: IEP Question", "Tue, 07 Jan 2025 09:00:00",
        "Can you attend on Thursday? Please let me know");
    e2.message_id = "q2@test".into();
    e2.in_reply_to = "q1@test".into();

    let emails = vec![e1, e2];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let threads = f2(&emails);
    let gaps = f9(&emails, &findings, &threads);

    let abandoned: Vec<_> = gaps.iter()
        .filter(|g| g.gap_type == GapType::ThreadAbandoned)
        .collect();
    assert!(!abandoned.is_empty(),
        "thread ending with 'please let me know' should be flagged as abandoned");
}

// ============================================================
// PRECEDENT MATCHING (f11) — findings map to correct MD cases
// ============================================================

#[test]
fn precedent_matches_alienation_to_domingues() {
    let emails = vec![
        email(0, "mom@test.com", "dad@test.com",
            "Visit", "Wed, 08 Jan 2025 18:00:00",
            "The kids don't feel safe going to your house."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let matches = f11(&findings);
    let domingues: Vec<_> = matches.iter()
        .filter(|m| m.precedent.case_name.contains("Domingues"))
        .collect();
    assert!(!domingues.is_empty(),
        "alienation finding should match Domingues v. Johnson");
}

#[test]
fn precedent_matches_food_to_health_factor() {
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "Lunch", "Mon, 06 Jan 2025 12:00:00",
            "Hazel refused all food at lunch."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let matches = f11(&findings);
    // Food findings should map to a health/safety case
    assert!(!matches.is_empty(), "food refusal should match at least one precedent");
}

#[test]
fn precedent_matches_iep_to_fitness() {
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "IEP Review", "Mon, 06 Jan 2025 10:00:00",
            "The IEP placement was changed without prior written notice."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let matches = f11(&findings);
    assert!(!matches.is_empty(), "IEP violation should match fitness precedents");
}

// ============================================================
// MULTI-FINDING EMAIL — one email can trigger multiple categories
// ============================================================

#[test]
fn single_email_multiple_findings() {
    // Email that triggers food refusal + weight tracking + medication
    let emails = vec![
        email(0, "nurse@chimes.org", "dad@test.com",
            "PRN Medication and Health", "Mon, 06 Jan 2025 11:00:00",
            "Hazel refused all food at lunch. She weighed 48 lbs at today's \
             checkup. Medication was administered at 10:30 AM."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let categories: Vec<T7> = findings.iter().map(|f| f.category.clone()).collect();
    assert!(categories.contains(&T7::FoodRecord), "should detect food refusal");
    assert!(categories.contains(&T7::WeightTracking), "should detect weight (48 lbs)");
    assert!(categories.contains(&T7::MedicationIssue), "should detect PRN medication");
}

// ============================================================
// EDGE CASES — boundary conditions that matter in court
// ============================================================

#[test]
fn unparseable_date_gets_unknown_custody_week() {
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", "sometime in January",
            "Daily Chimes report"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    assert!(!findings.is_empty());
    assert_eq!(findings[0].custody_week, Some(T8::Unknown),
        "unparseable date should result in Unknown custody week");
}

#[test]
fn empty_email_list_produces_no_findings() {
    let findings = f5(&[], &[], "dad", "mom", "hazel", custody_start()).unwrap();
    assert!(findings.is_empty());
}

#[test]
fn empty_body_produces_no_findings() {
    let emails = vec![
        email(0, "someone@test.com", "dad@test.com",
            "Generic", "Mon, 06 Jan 2025 09:00:00", ""),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    assert!(findings.is_empty(), "empty body should produce no findings");
}
