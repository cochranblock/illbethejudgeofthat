// Unlicense — cochranblock.org
// Tests that verify correctness of custody case analysis logic.
// These test real functions with known inputs and expected outputs.

use chrono::NaiveDate;
use illbethejudgeofthat::analyze::{f4, f5, f6, T6, T7, T8};
use illbethejudgeofthat::contradict::{f7, f8, T11};
use illbethejudgeofthat::gaps::{f9, f10, GapType};
use illbethejudgeofthat::thread::{f2, f3};
use illbethejudgeofthat::precedent::{f11, f12, f13};
use illbethejudgeofthat::ingest::f0;
use illbethejudgeofthat::cite_verify::{f17, T22};
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

// ============================================================
// PDF GENERATION — smoke tests + multi-page verification
// ============================================================

use illbethejudgeofthat::filing::{f16, T19, T20};
use illbethejudgeofthat::exhibit::f14;
use illbethejudgeofthat::forms::f15;

fn test_output_dir() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("illbethejudgeofthat_test");
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn make_ctx(findings: Vec<T6>, filing_type: T19) -> T20 {
    let precedents = f11(&findings);
    let threads = f2(&[]);
    let contradictions = f7(&findings, &threads);
    T20 {
        plaintiff: "Michael Cochran".into(),
        defendant: "Jane Doe".into(),
        case_number: "C-99-CV-25-000123".into(),
        county: "Anne Arundel".into(),
        court: "Circuit Court".into(),
        findings,
        precedents,
        contradictions,
        filing_type,
    }
}

#[test]
fn pdf_motion_produces_valid_file() {
    let dir = test_output_dir();
    let emails = vec![
        email(0, "mom@test.com", "dad@test.com",
            "Visit", "Wed, 08 Jan 2025 18:00:00",
            "The kids don't feel safe going to your house."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let ctx = make_ctx(findings, T19::MotionModifyCustody);
    let path = f16(&ctx, &dir).unwrap();

    assert!(path.exists(), "motion PDF should exist");
    let bytes = std::fs::read(&path).unwrap();
    assert!(bytes.len() > 100, "PDF should not be empty");
    assert_eq!(&bytes[0..5], b"%PDF-", "file should start with PDF header");
}

#[test]
fn pdf_memorandum_produces_valid_file() {
    let dir = test_output_dir();
    let ctx = make_ctx(vec![], T19::MemorandumInSupport);
    let path = f16(&ctx, &dir).unwrap();

    assert!(path.exists());
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(&bytes[0..5], b"%PDF-");
}

#[test]
fn pdf_multi_page_motion_does_not_truncate() {
    let dir = test_output_dir();
    // Generate enough findings to force multiple pages (50+ paragraphs)
    let mut emails = Vec::new();
    for i in 0..50 {
        emails.push(email(i, "mom@test.com", "dad@test.com",
            &format!("Issue {}", i),
            &format!("Mon, {:02} Jan 2025 09:00:00", (i % 28) + 1),
            &format!("I forgot to handle issue {}. I should have done it. Sorry about that.", i)));
    }
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    assert!(findings.len() >= 40, "should produce many findings for multi-page test");

    let ctx = make_ctx(findings, T19::MotionModifyCustody);
    let path = f16(&ctx, &dir).unwrap();

    let bytes = std::fs::read(&path).unwrap();
    let content = String::from_utf8_lossy(&bytes);
    assert!(!content.contains("[Continued on next page]"),
        "multi-page PDF should NOT contain truncation notice");
    assert!(bytes.len() > 5000,
        "50+ paragraph filing should produce substantial PDF (got {} bytes)", bytes.len());
}

#[test]
fn pdf_court_forms_produce_valid_files() {
    let dir = test_output_dir();
    let paths = f15(
        &dir, "Michael Cochran", "Jane Doe",
        "Hazel", "06/11/2018",
        &Some("C-99-CV-25-000123".into()),
        "Anne Arundel", "MD", false,
    ).unwrap();

    assert_eq!(paths.len(), 4, "should produce 4 MD court forms");
    for path in &paths {
        assert!(path.exists(), "form {} should exist", path.display());
        let bytes = std::fs::read(path).unwrap();
        assert_eq!(&bytes[0..5], b"%PDF-", "{} should be valid PDF", path.display());
    }
}

#[test]
fn pdf_exhibit_book_produces_valid_file() {
    let dir = test_output_dir();
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "Lunch", "Mon, 06 Jan 2025 12:00:00",
            "Hazel refused all food at lunch today."),
        email(1, "mom@test.com", "dad@test.com",
            "Visit", "Tue, 07 Jan 2025 18:00:00",
            "The kids don't feel safe with you."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let threads = f2(&emails);
    let contradictions = f7(&findings, &threads);
    let gaps = f9(&emails, &findings, &threads);

    let path = f14(
        &findings, &contradictions, &gaps, &dir,
        "Michael Cochran", "Jane Doe",
        &Some("C-99-CV-25-000123".into()),
        "Anne Arundel", "MD",
    ).unwrap();

    assert!(path.exists(), "exhibit book should exist");
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(&bytes[0..5], b"%PDF-");
    assert!(bytes.len() > 1000, "exhibit book with 2 findings should be non-trivial");
}

// ============================================================
// STAGE 1 — INGEST (f0): mbox parsing from a real temp file
// ============================================================

fn make_mbox(messages: &[(&str, &str, &str, &str, &str)]) -> String {
    // messages: (from_addr, to_addr, subject, date, body)
    let mut mbox = String::new();
    for (i, (from_addr, to_addr, subject, date, body)) in messages.iter().enumerate() {
        mbox.push_str(&format!("From {from_addr} Mon Jan 01 00:00:00 2025\n"));
        mbox.push_str(&format!("From: {from_addr}\n"));
        mbox.push_str(&format!("To: {to_addr}\n"));
        mbox.push_str(&format!("Subject: {subject}\n"));
        mbox.push_str(&format!("Date: {date}\n"));
        mbox.push_str(&format!("Message-ID: <msg-{i}@test.example.com>\n"));
        mbox.push_str("Content-Type: text/plain; charset=utf-8\n");
        mbox.push_str("\n");
        mbox.push_str(body);
        mbox.push_str("\n\n");
    }
    mbox
}

#[test]
fn ingest_f0_parses_minimal_mbox() {
    let mbox = make_mbox(&[
        ("teacher@chimes.org", "dad@test.com",
         "Daily Communication", "Mon, 06 Jan 2025 09:00:00",
         "Hazel had a great day today."),
        ("mom@test.com", "dad@test.com",
         "Visit", "Tue, 07 Jan 2025 18:00:00",
         "The kids don't feel safe going to your house."),
    ]);

    let tmp = std::env::temp_dir().join("test_ingest.mbox");
    std::fs::write(&tmp, &mbox).unwrap();

    let emails = f0(&tmp).unwrap();
    assert_eq!(emails.len(), 2, "should parse 2 emails from mbox");
    assert!(emails[0].from.contains("chimes.org"), "first email from chimes");
    assert!(emails[1].from.contains("mom@test.com"), "second email from mom");
    assert!(emails[0].body.contains("great day"), "body text preserved");
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn ingest_f0_handles_empty_mbox() {
    let tmp = std::env::temp_dir().join("test_ingest_empty.mbox");
    std::fs::write(&tmp, "").unwrap();
    let emails = f0(&tmp).unwrap();
    assert!(emails.is_empty(), "empty mbox produces no emails");
    std::fs::remove_file(&tmp).ok();
}

#[test]
fn ingest_f0_assigns_sequential_indices() {
    let mbox = make_mbox(&[
        ("a@test.com", "b@test.com", "First", "Mon, 06 Jan 2025 09:00:00", "body one"),
        ("b@test.com", "a@test.com", "Second", "Tue, 07 Jan 2025 09:00:00", "body two"),
        ("c@test.com", "a@test.com", "Third", "Wed, 08 Jan 2025 09:00:00", "body three"),
    ]);
    let tmp = std::env::temp_dir().join("test_ingest_idx.mbox");
    std::fs::write(&tmp, &mbox).unwrap();
    let emails = f0(&tmp).unwrap();
    assert_eq!(emails.len(), 3);
    assert_eq!(emails[0].index, 0);
    assert_eq!(emails[1].index, 1);
    assert_eq!(emails[2].index, 2);
    std::fs::remove_file(&tmp).ok();
}

// ============================================================
// STAGE 2 — PARSE (f1): attachment extraction
// ============================================================

#[test]
fn parse_f1_extracts_no_attachments_from_plain_emails() {
    use illbethejudgeofthat::parse::f1;
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "Daily", "Mon, 06 Jan 2025", "No attachments here."),
    ];
    let dir = test_output_dir().join("parse_test");
    std::fs::create_dir_all(&dir).unwrap();
    let attachments = f1(&emails, &dir).unwrap();
    assert!(attachments.is_empty(), "plain email has no attachments");
}

// ============================================================
// STAGE 3 — THREAD (f3): summary output
// ============================================================

#[test]
fn thread_f3_produces_non_empty_summary() {
    let emails = vec![
        email(0, "mom@test.com", "dad@test.com",
            "Visit schedule", "Mon, 06 Jan 2025 09:00:00",
            "Can we discuss the schedule?"),
        email(1, "dad@test.com", "mom@test.com",
            "Re: Visit schedule", "Tue, 07 Jan 2025 09:00:00",
            "Sure, happy to discuss."),
    ];
    let threads = f2(&emails);
    let summary = f3(&threads);
    assert!(!summary.is_empty(), "thread summary should not be empty");
    assert!(summary.contains("thread") || summary.contains("Thread"),
        "summary should mention threads");
}

#[test]
fn thread_f3_handles_empty_thread_list() {
    let summary = f3(&[]);
    assert!(!summary.is_empty(), "empty-thread summary should still return string");
}

// ============================================================
// STAGE 4 — ANALYZE (f6): summary statistics
// ============================================================

#[test]
fn analyze_f6_counts_categories_correctly() {
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "Lunch", "Mon, 06 Jan 2025 12:00:00",
            "Hazel refused all food at lunch today."),
        email(1, "teacher@chimes.org", "dad@test.com",
            "Lunch", "Tue, 07 Jan 2025 12:00:00",
            "Hazel refused all food again."),
        email(2, "mom@test.com", "dad@test.com",
            "Visit", "Wed, 08 Jan 2025 18:00:00",
            "The kids don't feel safe going to your house."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let summary = f6(&findings);
    assert!(summary.contains("Food Record") || summary.contains("food"),
        "summary should mention food category");
    assert!(summary.contains("Total"), "summary should have total count");
}

#[test]
fn analyze_f6_empty_findings() {
    let summary = f6(&[]);
    assert!(summary.contains("0"), "empty findings should show 0");
}

// ============================================================
// STAGE 5 — CONTRADICT (f8): summary output
// ============================================================

#[test]
fn contradict_f8_produces_summary_string() {
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "Lunch", "Mon, 06 Jan 2025 12:00:00",
            "Hazel refused all food at lunch."),
        email(1, "mom@test.com", "dad@test.com",
            "Re: Lunch", "Mon, 06 Jan 2025 14:00:00",
            "I provided lunch. She had plenty to eat."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let threads = f2(&emails);
    let contradictions = f7(&findings, &threads);
    let summary = f8(&contradictions);
    assert!(!summary.is_empty(), "summary should not be empty");
    assert!(summary.contains("Contradiction") || summary.contains("contradiction") || summary.contains("found"),
        "summary should describe contradictions");
}

// ============================================================
// STAGE 6 — GAPS (f10): summary output
// ============================================================

#[test]
fn gaps_f10_produces_summary_string() {
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "Daily Communication", "Mon, 06 Jan 2025 09:00:00",
            "Daily Chimes report: Hazel had a good day"),
        email(1, "teacher@chimes.org", "dad@test.com",
            "Daily Communication", "Mon, 27 Jan 2025 09:00:00",
            "Daily Chimes report: Hazel had a good day"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let threads = f2(&emails);
    let gaps = f9(&emails, &findings, &threads);
    let summary = f10(&gaps);
    assert!(!summary.is_empty(), "gap summary should not be empty");
}

#[test]
fn gaps_f10_reports_no_gaps_for_empty_input() {
    let summary = f10(&[]);
    assert!(summary.contains("0"), "no gaps = 0 in summary");
}

// ============================================================
// STAGE 7 — PRECEDENT (f12, f13): brief and summary output
// ============================================================

#[test]
fn precedent_f13_produces_summary() {
    let emails = vec![
        email(0, "mom@test.com", "dad@test.com",
            "Visit", "Mon, 06 Jan 2025 09:00:00",
            "The kids don't feel safe going to your house."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let matches = f11(&findings);
    let summary = f13(&matches);
    assert!(!summary.is_empty(), "precedent summary should not be empty");
}

#[test]
fn precedent_f12_brief_mentions_factors() {
    let emails = vec![
        email(0, "teacher@chimes.org", "dad@test.com",
            "IEP Review", "Mon, 06 Jan 2025 10:00:00",
            "The IEP placement was changed without prior written notice."),
        email(1, "mom@test.com", "dad@test.com",
            "Visit", "Wed, 08 Jan 2025 18:00:00",
            "The kids don't feel safe."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let matches = f11(&findings);
    let brief = f12(&matches, &findings);
    assert!(!brief.is_empty(), "brief should not be empty");
    // Brief should reference MD law factors
    assert!(brief.contains("§") || brief.contains("factor") || brief.contains("Factor"),
        "brief should reference legal factors");
}

// ============================================================
// STAGE 9 — CITE VERIFY (f17): citation format checking
// ============================================================

#[test]
fn cite_verify_f17_verifies_known_citations() {
    let emails = vec![
        email(0, "mom@test.com", "dad@test.com",
            "Safety concern", "Mon, 06 Jan 2025 09:00:00",
            "The kids don't feel safe going to your house."),
        email(1, "teacher@chimes.org", "dad@test.com",
            "IEP meeting", "Tue, 07 Jan 2025 10:00:00",
            "IEP placement changed without prior written notice."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let matches = f11(&findings);
    // only run f17 if there are precedent matches
    if matches.is_empty() { return; }
    let checks = f17(&matches);
    assert!(!checks.is_empty(), "should produce citation checks");
    // Every check should have a non-empty citation string
    for c in &checks {
        assert!(!c.citation.is_empty(), "citation should not be empty");
    }
}

#[test]
fn cite_verify_known_citations_verified() {
    // A precedent for alienation should match a known DB citation
    let emails = vec![
        email(0, "mom@test.com", "dad@test.com",
            "Kids upset", "Mon, 13 Jan 2025 09:00:00",
            "The kids don't feel safe going to your house."),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
    let matches = f11(&findings);
    if matches.is_empty() { return; }
    let checks = f17(&matches);
    let any_verified = checks.iter().any(|c| matches!(c.status, T22::Verified | T22::FormatOk));
    assert!(any_verified, "at least one precedent citation should pass format check");
}

// ============================================================
// FINDING EXTRACTION REGEX — keyword trigger coverage
// ============================================================

#[test]
fn regex_detects_weight_various_formats() {
    let cases = [
        ("Hazel weighed 52 lbs today.", T7::WeightTracking),
        ("She weighs 48 pounds at checkup.", T7::WeightTracking),
        ("Weight recorded: 61 lb.", T7::WeightTracking),
    ];
    for (body, expected_cat) in &cases {
        let emails = vec![
            email(0, "nurse@chimes.org", "dad@test.com",
                "Health", "Mon, 06 Jan 2025 11:00:00", body),
        ];
        let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
        assert!(
            findings.iter().any(|f| f.category == *expected_cat),
            "should detect {:?} in: {}", expected_cat, body
        );
    }
}

#[test]
fn regex_detects_food_refusal_variants() {
    let phrases = [
        "Hazel refused all food at lunch.",
        "She refused food this morning.",
        "Child did not eat breakfast.",
        "She wouldn't eat anything provided.",
        "She would not eat the meal.",
    ];
    for phrase in &phrases {
        let emails = vec![
            email(0, "teacher@chimes.org", "dad@test.com",
                "Lunch Report", "Mon, 06 Jan 2025 12:00:00", phrase),
        ];
        let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
        assert!(
            findings.iter().any(|f| f.category == T7::FoodRecord),
            "should detect FoodRecord in: {}", phrase
        );
    }
}

#[test]
fn regex_detects_alienation_phrases() {
    let phrases = [
        "The kids don't feel safe going to your house.",
        "She doesn't feel safe with dad.",
        "She doesn't want to go.",
        "He is afraid of dad.",
        "She is scared of him.",
        "She is not comfortable there.",
    ];
    for phrase in &phrases {
        let emails = vec![
            email(0, "mom@test.com", "dad@test.com",
                "Concern", "Mon, 06 Jan 2025 09:00:00", phrase),
        ];
        let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
        assert!(
            findings.iter().any(|f| f.category == T7::Alienation),
            "should detect Alienation in: {}", phrase
        );
    }
}

#[test]
fn regex_detects_court_threat_phrases() {
    let phrases = [
        "I am going to court over this.",
        "I will take you to court.",
        "My lawyer will handle this.",
        "My attorney has been notified.",
    ];
    for phrase in &phrases {
        let emails = vec![
            email(0, "mom@test.com", "dad@test.com",
                "Warning", "Mon, 06 Jan 2025 09:00:00", phrase),
        ];
        let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
        assert!(
            findings.iter().any(|f| f.category == T7::CourtThreat),
            "should detect CourtThreat in: {}", phrase
        );
    }
}

#[test]
fn regex_detects_admission_against_interest() {
    let phrases = [
        "It was my fault, I forgot the medication.",
        "I didn't follow the schedule.",
        "I should have called. I was wrong.",
        "I apologize for missing pickup.",
        "Sorry about the missed appointment.",
    ];
    for phrase in &phrases {
        let emails = vec![
            email(0, "mom@test.com", "dad@test.com",
                "Apology", "Mon, 06 Jan 2025 09:00:00", phrase),
        ];
        let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
        assert!(
            findings.iter().any(|f| f.category == T7::AdmissionAgainstInterest),
            "should detect AdmissionAgainstInterest in: {}", phrase
        );
    }
}

#[test]
fn regex_detects_iep_provisions() {
    let cases = [
        ("The IEP placement was changed without prior written notice.", "IDEA §300.503"),
        ("Child had elopement incident. BIP needs updating.", "IDEA §300.324"),
        ("FBA was not completed as required.", "IDEA §300.530"),
    ];
    for (body, expected_tag) in &cases {
        let emails = vec![
            email(0, "teacher@chimes.org", "dad@test.com",
                "IEP Update", "Mon, 06 Jan 2025 10:00:00", body),
        ];
        let findings = f5(&emails, &[], "dad", "mom", "hazel", custody_start()).unwrap();
        let iep_findings: Vec<_> = findings.iter()
            .filter(|f| f.category == T7::IepViolation)
            .collect();
        assert!(!iep_findings.is_empty(), "should detect IepViolation in: {}", body);
        let has_tag = iep_findings.iter()
            .any(|f| f.summary.contains(expected_tag));
        assert!(has_tag, "summary should contain '{}' for: {}", expected_tag, body);
    }
}

// ============================================================
// END-TO-END PIPELINE — all 10 stages from mbox to output files
// ============================================================

#[test]
fn end_to_end_all_10_stages() {
    use illbethejudgeofthat::parse::f1;
    use illbethejudgeofthat::contradict::f7;
    use illbethejudgeofthat::gaps::f9;
    use illbethejudgeofthat::exhibit::f14;
    use illbethejudgeofthat::cite_verify::{f17, f18};
    use illbethejudgeofthat::filing::{f16, T19, T20};
    use illbethejudgeofthat::forms::f15;

    // Build a realistic mbox with varied email types across multiple weeks
    let mbox = make_mbox(&[
        // Week 1 (plaintiff) — daily reports + food refusal
        ("teacher@chimes.org", "dad@test.com",
         "Daily Communication", "Mon, 06 Jan 2025 09:00:00",
         "Daily Chimes report: Hazel had a good day. She weighed 52 lbs."),
        ("teacher@chimes.org", "dad@test.com",
         "Daily Communication", "Tue, 07 Jan 2025 09:00:00",
         "Daily Chimes report: Hazel refused all food at lunch today."),
        ("teacher@chimes.org", "dad@test.com",
         "Daily Communication", "Wed, 08 Jan 2025 09:00:00",
         "Daily Chimes report: Hazel had a good day."),
        // IEP concern
        ("teacher@chimes.org", "dad@test.com",
         "IEP Review Meeting", "Thu, 09 Jan 2025 10:00:00",
         "The IEP placement was changed without prior written notice. Elopement concerns also noted. FBA needed."),
        // Alienation from defendant
        ("mom@test.com", "dad@test.com",
         "Kids upset", "Fri, 10 Jan 2025 18:00:00",
         "The kids don't feel safe going to your house. They don't want to go."),
        // Week 2 (defendant) — communication silence (no emails Mon-Wed)
        ("teacher@chimes.org", "dad@test.com",
         "Daily Communication", "Thu, 16 Jan 2025 09:00:00",
         "Daily Chimes report: Hazel was absent from school today."),
        // Admission against interest
        ("mom@test.com", "dad@test.com",
         "Apology", "Fri, 17 Jan 2025 09:00:00",
         "I forgot to give Hazel her medication. I should have remembered. Sorry about that."),
        // Court threat
        ("mom@test.com", "dad@test.com",
         "Warning", "Mon, 20 Jan 2025 09:00:00",
         "I am going to court over the custody arrangement. My lawyer agrees."),
        // De-escalation from plaintiff
        ("dad@test.com", "mom@test.com",
         "Let's resolve this", "Tue, 21 Jan 2025 10:00:00",
         "I understand your concerns. Let's work together for the kids' best interest."),
        // State complaint
        ("admin@msde.md.gov", "dad@test.com",
         "State Complaint 26-154", "Wed, 22 Jan 2025 14:00:00",
         "This is regarding the state complaint filed against Chimes School."),
    ]);

    let mbox_path = std::env::temp_dir().join("e2e_test.mbox");
    std::fs::write(&mbox_path, &mbox).unwrap();

    let out_dir = std::env::temp_dir().join("e2e_test_output");
    std::fs::create_dir_all(&out_dir).unwrap();

    let cs = NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();

    // Stage 1: Ingest
    let emails = f0(&mbox_path).unwrap();
    assert!(!emails.is_empty(), "stage 1: should parse emails from mbox");

    // Stage 2: Attachments
    let attachments = f1(&emails, &out_dir).unwrap();
    // no attachments in this synthetic mbox, just verify no panic
    let _ = attachments;

    // Stage 3: Thread reconstruction
    let threads = f2(&emails);
    assert!(!threads.is_empty(), "stage 3: should produce at least one thread");
    let thread_summary = f3(&threads);
    assert!(!thread_summary.is_empty(), "stage 3: thread summary non-empty");

    // Stage 4: Analyze
    let findings = f5(&emails, &[], "dad", "mom", "hazel", cs).unwrap();
    assert!(findings.len() >= 5, "stage 4: should detect at least 5 findings, got {}", findings.len());

    // Verify key categories are detected
    let cats: Vec<_> = findings.iter().map(|f| &f.category).collect();
    assert!(cats.iter().any(|c| **c == T7::FoodRecord), "should find food refusal");
    assert!(cats.iter().any(|c| **c == T7::Alienation), "should find alienation");
    assert!(cats.iter().any(|c| **c == T7::IepViolation), "should find IEP violation");
    assert!(cats.iter().any(|c| **c == T7::AdmissionAgainstInterest), "should find admission");
    assert!(cats.iter().any(|c| **c == T7::CourtThreat), "should find court threat");

    // Exhibit numbers assigned
    assert!(findings.iter().all(|f| f.exhibit_number.is_some()), "stage 4: all findings have exhibit numbers");

    // Stage 5: Contradictions
    let contradictions = f7(&findings, &threads);
    let _contra_summary = f8(&contradictions);

    // Stage 6: Gaps
    let gaps = f9(&emails, &findings, &threads);
    let gap_summary = f10(&gaps);
    assert!(!gap_summary.is_empty(), "stage 6: gap summary non-empty");

    // Stage 7: Precedents
    let matches = f11(&findings);
    assert!(!matches.is_empty(), "stage 7: should match at least one precedent");
    let brief = f12(&matches, &findings);
    assert!(!brief.is_empty(), "stage 7: brief non-empty");
    let prec_summary = f13(&matches);
    assert!(!prec_summary.is_empty(), "stage 7: precedent summary non-empty");

    // Persist JSON outputs (verifies serialize doesn't panic)
    std::fs::write(out_dir.join("findings.json"), serde_json::to_string_pretty(&findings).unwrap()).unwrap();
    std::fs::write(out_dir.join("contradictions.json"), serde_json::to_string_pretty(&contradictions).unwrap()).unwrap();
    std::fs::write(out_dir.join("gaps.json"), serde_json::to_string_pretty(&gaps).unwrap()).unwrap();
    std::fs::write(out_dir.join("precedents.json"), serde_json::to_string_pretty(&matches).unwrap()).unwrap();
    assert!(out_dir.join("findings.json").exists(), "findings.json written");

    // Stage 8: Exhibit book (PDF)
    let exhibit_path = f14(
        &findings, &contradictions, &gaps, &out_dir,
        "Michael Cochran", "Jane Doe",
        &Some("C-99-CV-25-000123".into()),
        "Anne Arundel", "MD",
    ).unwrap();
    assert!(exhibit_path.exists(), "stage 8: exhibit book PDF exists");
    let exhibit_bytes = std::fs::read(&exhibit_path).unwrap();
    assert_eq!(&exhibit_bytes[0..5], b"%PDF-", "stage 8: exhibit book is valid PDF");

    // Stage 9: Citation verification
    let cite_checks = f17(&matches);
    assert!(!cite_checks.is_empty(), "stage 9: citation checks produced");
    f18(&cite_checks); // just verify it doesn't panic

    // Stage 10: Court filings (PDFs)
    let ctx = T20 {
        plaintiff: "Michael Cochran".into(),
        defendant: "Jane Doe".into(),
        case_number: "C-99-CV-25-000123".into(),
        county: "Anne Arundel".into(),
        court: "Circuit Court for Anne Arundel County".into(),
        findings: findings.clone(),
        precedents: matches.clone(),
        contradictions: contradictions.clone(),
        filing_type: T19::MotionModifyCustody,
    };
    let motion_path = f16(&ctx, &out_dir).unwrap();
    assert!(motion_path.exists(), "stage 10: motion PDF exists");
    let motion_bytes = std::fs::read(&motion_path).unwrap();
    assert_eq!(&motion_bytes[0..5], b"%PDF-", "stage 10: motion is valid PDF");

    let forms = f15(
        &out_dir, "Michael Cochran", "Jane Doe",
        "Hazel", "06/11/2018",
        &Some("C-99-CV-25-000123".into()),
        "Anne Arundel", "MD", false,
    ).unwrap();
    assert!(!forms.is_empty(), "stage 10: court forms generated");
    for form in &forms {
        assert!(form.exists(), "stage 10: form {} exists", form.display());
    }

    // Cleanup
    std::fs::remove_file(&mbox_path).ok();
}

// ============================================================
// FIX A — most_recent_thursday() correctness (backlog #1a)
// ============================================================

use illbethejudgeofthat::analyze::most_recent_thursday;
use chrono::Datelike;

#[test]
fn most_recent_thursday_is_a_thursday() {
    let thu = most_recent_thursday();
    assert_eq!(thu.weekday(), chrono::Weekday::Thu,
        "most_recent_thursday() must return a Thursday, got {:?} ({})", thu.weekday(), thu);
}

#[test]
fn most_recent_thursday_is_not_in_the_future() {
    let thu = most_recent_thursday();
    let today = chrono::Local::now().date_naive();
    assert!(thu <= today, "most_recent_thursday() must be <= today, got {}", thu);
}

#[test]
fn most_recent_thursday_is_within_last_7_days() {
    let thu = most_recent_thursday();
    let today = chrono::Local::now().date_naive();
    let days_back = (today - thu).num_days();
    assert!(days_back < 7, "most_recent_thursday() should be within 6 days of today, got {} days back", days_back);
}

#[test]
fn most_recent_thursday_consistent_with_custody_schedule() {
    // The custody schedule anchors to plaintiff_start (must be a Thursday).
    // most_recent_thursday() feeds into that anchor. Verify the schedule
    // assigns the anchor date itself as plaintiff's week (weeks_diff = 0, even).
    let thu = most_recent_thursday();
    let emails = vec![
        email(0, "school@chimes.org", "dad@test.com",
            "Daily Communication", &format!("Thu, {:02} {} {} 09:00:00",
                thu.day(),
                ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"]
                    [(thu.month() - 1) as usize],
                thu.year()),
            "Daily Chimes report: good day"),
    ];
    let findings = f5(&emails, &[], "dad", "mom", "hazel", thu).unwrap();
    // weeks_diff = 0 → even → plaintiff. Anchor date must be plaintiff's week.
    assert!(!findings.is_empty(), "should detect daily report");
    assert_eq!(findings[0].custody_week, Some(T8::Plaintiff),
        "anchor Thursday should be plaintiff's week (weeks_diff=0)");
}

#[test]
fn hardcoded_2025_date_is_not_the_default() {
    // Regression: ensure the old hardcoded 2025-01-02 is gone.
    // most_recent_thursday() in 2026 should NOT return 2025-01-02.
    let thu = most_recent_thursday();
    let old_default = NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();
    let today = chrono::Local::now().date_naive();
    if today > NaiveDate::from_ymd_opt(2025, 1, 9).unwrap() {
        // After Jan 9 2025, the dynamic default must differ from the old hardcode
        assert_ne!(thu, old_default,
            "most_recent_thursday() must not return the old hardcoded 2025-01-02 default");
    }
}

// ============================================================
// FIX B — sample.mbox is valid mbox (backlog #1b)
// ============================================================

#[test]
fn sample_mbox_parses_to_22_emails() {
    let mbox_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/sample.mbox");
    assert!(mbox_path.exists(), "examples/sample.mbox must exist");
    let emails = f0(&mbox_path).unwrap();
    assert_eq!(emails.len(), 22,
        "sample.mbox should parse to exactly 22 emails, got {}", emails.len());
}

#[test]
fn sample_mbox_has_expected_senders() {
    let mbox_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/sample.mbox");
    let emails = f0(&mbox_path).unwrap();
    let senders: Vec<&str> = emails.iter().map(|e| e.from.as_str()).collect();

    assert!(senders.iter().any(|s| s.contains("privera@silverleafelem")),
        "should have school nurse email");
    assert!(senders.iter().any(|s| s.contains("dsantos@example.com")),
        "should have defendant (Derek) emails");
    assert!(senders.iter().any(|s| s.contains("msantos@example.com")),
        "should have plaintiff (Maria) emails");
    assert!(senders.iter().any(|s| s.contains("kwebb@maplewoodpeds")),
        "should have pediatrician email");
}

#[test]
fn sample_mbox_bodies_are_non_empty() {
    let mbox_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/sample.mbox");
    let emails = f0(&mbox_path).unwrap();
    let empty_bodies: Vec<usize> = emails.iter()
        .filter(|e| e.body.trim().is_empty())
        .map(|e| e.index)
        .collect();
    assert!(empty_bodies.is_empty(),
        "all emails should have non-empty bodies; empty at indices: {:?}", empty_bodies);
}

#[test]
fn sample_mbox_produces_findings_end_to_end() {
    // Verify the sample actually drives findings through the pipeline.
    // Custody start: Jan 16 2025 (Thu) — a known date in the sample's range.
    let mbox_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/sample.mbox");
    let emails = f0(&mbox_path).unwrap();
    assert!(!emails.is_empty());

    let cs = NaiveDate::from_ymd_opt(2025, 1, 16).unwrap();
    let findings = f5(&emails, &[], "maria", "derek", "tommy", cs).unwrap();

    assert!(!findings.is_empty(),
        "sample.mbox should produce findings through the pipeline");

    // Health concern: inhaler incident has explicit health keywords
    let has_health = findings.iter().any(|f|
        matches!(f.category, T7::MedicationIssue | T7::HealthConcern));
    assert!(has_health, "sample should produce MedicationIssue or HealthConcern from inhaler email");

    // School absence: "marked absent on Friday, April 4"
    let has_absence = findings.iter().any(|f| f.category == T7::SchoolAbsence);
    assert!(has_absence, "sample should produce SchoolAbsence from absence notification email");

    // Court threat: "I'll go back to court and let a judge sort it out"
    let has_court = findings.iter().any(|f| f.category == T7::CourtThreat);
    assert!(has_court, "sample should produce CourtThreat from Derek's court threat email");
}

#[test]
fn sample_mbox_old_format_txt_does_not_parse_as_mbox() {
    // Confirm the old === EMAIL N === format fails — this is the bug we fixed.
    let txt_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/sample_emails.txt");
    if !txt_path.exists() { return; } // file may be removed later
    let emails = f0(&txt_path).unwrap();
    assert!(emails.is_empty() || emails.iter().all(|e| e.body.trim().is_empty()),
        "old .txt format should not produce usable emails via mbox parser");
}
