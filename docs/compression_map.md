<!-- Unlicense — cochranblock.org -->

# Compression Map — illbethejudgeofthat

> P13 tokenization. Every public symbol has a compressed identifier.

## Functions (f + number)

| Token | Module | Human Name | Signature |
|-------|--------|------------|-----------|
| f0 | ingest | ingest_mbox | (path) -> Vec\<Email\> |
| f1 | parse | extract_attachments | (emails, output) -> Vec\<ExtractedAttachment\> |
| f2 | thread | reconstruct_threads | (emails) -> Vec\<Thread\> |
| f3 | thread | summarize_threads | (threads) -> String |
| f4 | analyze | parse_email_date | (date_str) -> Option\<NaiveDate\> |
| f5 | analyze | analyze | (emails, attachments, plaintiff, defendant, children, custody_start) -> Vec\<Finding\> |
| f6 | analyze | summarize_findings | (findings) -> String |
| f7 | contradict | detect_contradictions | (findings, threads) -> Vec\<Contradiction\> |
| f8 | contradict | summarize_contradictions | (contradictions) -> String |
| f9 | gaps | detect_gaps | (emails, findings, threads) -> Vec\<TimelineGap\> |
| f10 | gaps | summarize_gaps | (gaps) -> String |
| f11 | precedent | match_precedents | (findings) -> Vec\<PrecedentMatch\> |
| f12 | precedent | build_factor_brief | (matches, findings) -> String |
| f13 | precedent | summarize_precedents | (matches) -> String |
| f14 | exhibit | build_exhibit_book | (findings, contradictions, gaps, output, plaintiff, defendant, case_number, county, state) -> PathBuf |
| f15 | forms | generate_forms | (output, plaintiff, defendant, children, dobs, case_number, county, state, skip) -> Vec\<PathBuf\> |
| f16 | filing | generate_filing | (ctx, output) -> PathBuf |
| f17 | cite_verify | verify_all | (precedents) -> Vec\<CitationCheck\> |
| f18 | cite_verify | print_report | (checks) |
| f19 | query | run_query | (output_dir) |

## Types (t + number)

| Token | Module | Human Name |
|-------|--------|------------|
| T0 | ingest | Email |
| T1 | ingest | Attachment |
| T2 | ingest | IngestError |
| T3 | parse | ExtractedAttachment |
| T4 | parse | ParseError |
| T5 | thread | Thread |
| T6 | analyze | Finding |
| T7 | analyze | FindingCategory |
| T8 | analyze | CustodyParent |
| T9 | analyze | AnalyzeError |
| T10 | contradict | Contradiction |
| T11 | contradict | ContradictionType |
| T12 | gaps | TimelineGap |
| T13 | gaps | GapType |
| T14 | precedent | BestInterestFactor |
| T15 | precedent | Precedent |
| T16 | precedent | PrecedentMatch |
| T17 | exhibit | ExhibitError |
| T18 | forms | FormsError |
| T19 | filing | FilingType |
| T20 | filing | FilingContext |
| T21 | cite_verify | CitationCheck |
| T22 | cite_verify | CitationStatus |

## Fields (s + number)

### t0 (Email)
| Token | Human Name |
|-------|------------|
| s0 | index |
| s1 | from |
| s2 | to |
| s3 | cc |
| s4 | subject |
| s5 | date |
| s6 | body |
| s7 | labels |
| s8 | message_id |
| s9 | in_reply_to |
| s10 | gmail_thread_id |

### t1 (Attachment)
| Token | Human Name |
|-------|------------|
| s11 | filename |
| s12 | content_type |
| s13 | data |
| s14 | email_index |

### t5 (Thread)
| Token | Human Name |
|-------|------------|
| s15 | thread_id |
| s16 | emails |
| s17 | participants |
| s18 | subject |
| s19 | start_date |
| s20 | end_date |
| s21 | message_count |

### t6 (Finding)
| Token | Human Name |
|-------|------------|
| s22 | category |
| s23 | date |
| s24 | parsed_date |
| s25 | summary |
| s26 | detail |
| s27 | source_email_index |
| s28 | source_attachment |
| s29 | highlighted_text |
| s30 | exhibit_number |
| s31 | from |
| s32 | to |
| s33 | subject |
| s34 | custody_week |
| s35 | child_name |

### t10 (Contradiction)
| Token | Human Name |
|-------|------------|
| s36 | exhibit_a |
| s37 | exhibit_b |
| s38 | contradiction_type |
| s39 | explanation |
| s40 | date |

### t12 (TimelineGap)
| Token | Human Name |
|-------|------------|
| s41 | gap_type |
| s42 | start_date |
| s43 | end_date |
| s44 | duration_days |
| s45 | custody_parent |
| s46 | expected_source |
| s47 | significance |

### t15 (Precedent)
| Token | Human Name |
|-------|------------|
| s48 | case_name |
| s49 | citation |
| s50 | year |
| s51 | court |
| s52 | holding |
| s53 | relevance |
| s54 | factors |
| s55 | finding_categories |

### t16 (PrecedentMatch)
| Token | Human Name |
|-------|------------|
| s56 | precedent |
| s57 | matching_exhibits |
| s58 | matching_factor |
| s59 | argument_summary |
| s60 | exhibit_count |

### t20 (FilingContext)
| Token | Human Name |
|-------|------------|
| s61 | plaintiff |
| s62 | defendant |
| s63 | case_number |
| s64 | county |
| s65 | court |
| s66 | findings |
| s67 | precedents |
| s68 | contradictions |
| s69 | filing_type |

### t21 (CitationCheck)
| Token | Human Name |
|-------|------------|
| s70 | citation |
| s71 | case_name |
| s72 | valid_format |
| s73 | found_in_db |
| s74 | court |
| s75 | year |
| s76 | status |

## Error Variants (E + number)

### t2 (IngestError)
| Token | Human Name |
|-------|------------|
| E0 | Io |
| E1 | Parse |

### t4 (ParseError)
| Token | Human Name |
|-------|------------|
| E2 | Io |
| E3 | Decode |

### t9 (AnalyzeError)
| Token | Human Name |
|-------|------------|
| E4 | Io |

### t17 (ExhibitError)
| Token | Human Name |
|-------|------------|
| E5 | Io |
| E6 | Pdf |

### t18 (FormsError)
| Token | Human Name |
|-------|------------|
| E7 | Io |
| E8 | Pdf |
| E9 | UnsupportedState |

### t22 (CitationStatus)
| Token | Human Name |
|-------|------------|
| E10 | Verified |
| E11 | FormatOk |
| E12 | BadFormat |
| E13 | NotFound |

## Enum Variants

### t7 (FindingCategory) — 20 variants
| Token | Human Name |
|-------|------------|
| C0 | CustodyInterference |
| C1 | FoodRecord |
| C2 | HealthConcern |
| C3 | Alienation |
| C4 | SchoolAbsence |
| C5 | InstitutionalBias |
| C6 | FinancialChange |
| C7 | FalseAllegation |
| C8 | ReportingDiscrepancy |
| C9 | DeEscalation |
| C10 | CourtThreat |
| C11 | DailyReport |
| C12 | IepViolation |
| C13 | BehavioralIncident |
| C14 | CommunicationBlock |
| C15 | StateComplaint |
| C16 | MedicationIssue |
| C17 | WeightTracking |
| C18 | TransportationIssue |
| C19 | AdmissionAgainstInterest |

### t8 (CustodyParent)
| Token | Human Name |
|-------|------------|
| P0 | Plaintiff |
| P1 | Defendant |
| P2 | Unknown |

### t11 (ContradictionType) — 5 variants
| Token | Human Name |
|-------|------------|
| X0 | SchoolVsParent |
| X1 | FoodRefusal |
| X2 | AttendanceConflict |
| X3 | CustodyWeekConflict |
| X4 | BehavioralConflict |

### t13 (GapType) — 4 variants
| Token | Human Name |
|-------|------------|
| G0 | MissingDailyReport |
| G1 | CommunicationSilence |
| G2 | CustodyWeekGap |
| G3 | ThreadAbandoned |

### t14 (BestInterestFactor) — 12 variants
| Token | Human Name |
|-------|------------|
| B0 | WishesOfParents |
| B1 | WishesOfChild |
| B2 | InteractionWithParent |
| B3 | InteractionWithSiblings |
| B4 | AdjustmentToHome |
| B5 | AdjustmentToSchool |
| B6 | AdjustmentToCommunity |
| B7 | MentalPhysicalHealth |
| B8 | GovernmentRecommendation |
| B9 | AbuseOrNeglect |
| B10 | FitnessOfParent |
| B11 | Proximity |

### t19 (FilingType) — 5 variants
| Token | Human Name |
|-------|------------|
| F0 | MotionModifyCustody |
| F1 | MemorandumInSupport |
| F2 | Opposition |
| F3 | DiscoveryResponse |
| F4 | MotionContempt |
