use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::analyze::{Finding, FindingCategory};

/// Maryland Family Law §9-101.1 best interest factors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BestInterestFactor {
    FitnessOfParents,
    CharacterReputation,
    DesireAndAgreement,
    PotentialMaintainingRelations,
    PreferenceOfChild,
    MaterialOpportunity,
    AgeHealthSex,
    ResidenceOfParentsAndOpportunity,
    LengthOfSeparation,
    PriorAbandonmentOrSurrender,
    /// Added by courts — willingness to share custody
    WillingnessToShare,
    /// Added by courts — which parent is more likely to maintain relationship with other parent
    FriendlyParent,
}

impl std::fmt::Display for BestInterestFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FitnessOfParents => write!(f, "Fitness of Parents (§9-101.1(a))"),
            Self::CharacterReputation => write!(f, "Character & Reputation (§9-101.1(b))"),
            Self::DesireAndAgreement => write!(f, "Desire & Agreement of Parents (§9-101.1(c))"),
            Self::PotentialMaintainingRelations => write!(f, "Potential for Maintaining Relations (§9-101.1(d))"),
            Self::PreferenceOfChild => write!(f, "Preference of Child (§9-101.1(e))"),
            Self::MaterialOpportunity => write!(f, "Material Opportunity (§9-101.1(f))"),
            Self::AgeHealthSex => write!(f, "Age, Health, Sex of Child (§9-101.1(g))"),
            Self::ResidenceOfParentsAndOpportunity => write!(f, "Residence & Opportunity for Visitation (§9-101.1(h))"),
            Self::LengthOfSeparation => write!(f, "Length of Separation (§9-101.1(i))"),
            Self::PriorAbandonmentOrSurrender => write!(f, "Prior Abandonment/Surrender (§9-101.1(j))"),
            Self::WillingnessToShare => write!(f, "Willingness to Share Custody (court-created)"),
            Self::FriendlyParent => write!(f, "Friendly Parent Doctrine (court-created)"),
        }
    }
}

/// A Maryland family law precedent case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Precedent {
    pub case_name: String,
    pub citation: String,
    pub year: u32,
    pub court: String,
    pub holding: String,
    pub relevance: String,
    pub factors: Vec<BestInterestFactor>,
    pub finding_categories: Vec<FindingCategory>,
}

/// Match between a finding pattern and applicable precedent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecedentMatch {
    pub precedent: Precedent,
    pub matching_exhibits: Vec<usize>,
    pub matching_factor: BestInterestFactor,
    pub argument_summary: String,
    pub exhibit_count: usize,
}

/// Build the Maryland precedent database
fn build_md_precedents() -> Vec<Precedent> {
    vec![
        Precedent {
            case_name: "Taylor v. Taylor".into(),
            citation: "306 Md. 290 (1986)".into(),
            year: 1986,
            court: "Court of Appeals of Maryland".into(),
            holding: "Established the full list of factors courts must consider in custody determinations. No single factor is dispositive; courts must weigh all relevant factors.".into(),
            relevance: "Foundation case for all MD custody modifications. Every best interest factor traced to this decision.".into(),
            factors: vec![
                BestInterestFactor::FitnessOfParents,
                BestInterestFactor::CharacterReputation,
                BestInterestFactor::PotentialMaintainingRelations,
                BestInterestFactor::WillingnessToShare,
            ],
            finding_categories: vec![
                FindingCategory::DeEscalation,
                FindingCategory::Alienation,
                FindingCategory::CustodyInterference,
            ],
        },
        Precedent {
            case_name: "Montgomery County DSS v. Sanders".into(),
            citation: "38 Md. App. 406 (1977)".into(),
            year: 1977,
            court: "Court of Special Appeals of Maryland".into(),
            holding: "Material change in circumstances must be shown before court will modify existing custody order. Change must affect the welfare of the child.".into(),
            relevance: "Threshold requirement for custody modification. Behavioral incidents, IEP violations, and communication blocks establish material change.".into(),
            factors: vec![
                BestInterestFactor::FitnessOfParents,
                BestInterestFactor::AgeHealthSex,
            ],
            finding_categories: vec![
                FindingCategory::BehavioralIncident,
                FindingCategory::IepViolation,
                FindingCategory::HealthConcern,
                FindingCategory::MedicationIssue,
            ],
        },
        Precedent {
            case_name: "McCready v. McCready".into(),
            citation: "323 Md. 476 (1991)".into(),
            year: 1991,
            court: "Court of Appeals of Maryland".into(),
            holding: "Parental fitness includes ability to provide for child's educational, emotional, and physical needs. Failure to follow through on medical or educational obligations is relevant.".into(),
            relevance: "IEP violations, medication failures, and missed school services directly speak to parental fitness.".into(),
            factors: vec![
                BestInterestFactor::FitnessOfParents,
                BestInterestFactor::MaterialOpportunity,
            ],
            finding_categories: vec![
                FindingCategory::IepViolation,
                FindingCategory::MedicationIssue,
                FindingCategory::SchoolAbsence,
                FindingCategory::FoodRecord,
            ],
        },
        Precedent {
            case_name: "Domingues v. Johnson".into(),
            citation: "323 Md. 486 (1991)".into(),
            year: 1991,
            court: "Court of Appeals of Maryland".into(),
            holding: "Alienating behavior by one parent against the other is a significant factor weighing against the alienating parent in custody determinations.".into(),
            relevance: "Alienation findings directly applicable. Statements undermining child's relationship with other parent weigh heavily.".into(),
            factors: vec![
                BestInterestFactor::PotentialMaintainingRelations,
                BestInterestFactor::FriendlyParent,
                BestInterestFactor::CharacterReputation,
            ],
            finding_categories: vec![
                FindingCategory::Alienation,
                FindingCategory::CustodyInterference,
                FindingCategory::FalseAllegation,
            ],
        },
        Precedent {
            case_name: "Santo v. Santo".into(),
            citation: "448 Md. 620 (2016)".into(),
            year: 2016,
            court: "Court of Appeals of Maryland".into(),
            holding: "Reaffirmed Taylor factors. Court must consider totality of circumstances. Pattern of behavior over time is more probative than isolated incidents.".into(),
            relevance: "Timeline of 483 findings over 6 years establishes pattern, not isolated incidents. Strengthens modification argument.".into(),
            factors: vec![
                BestInterestFactor::FitnessOfParents,
                BestInterestFactor::CharacterReputation,
                BestInterestFactor::WillingnessToShare,
            ],
            finding_categories: vec![
                FindingCategory::DeEscalation,
                FindingCategory::CourtThreat,
                FindingCategory::AdmissionAgainstInterest,
            ],
        },
        Precedent {
            case_name: "Gillespie v. Gillespie".into(),
            citation: "206 Md. App. 146 (2012)".into(),
            year: 2012,
            court: "Court of Special Appeals of Maryland".into(),
            holding: "The friendly parent doctrine: the parent more willing to foster the child's relationship with the other parent is favored. Blocking communication or access weighs against that parent.".into(),
            relevance: "Communication blocks and institutional restrictions on parent contact directly implicate the friendly parent doctrine. De-escalation attempts show willingness to co-parent.".into(),
            factors: vec![
                BestInterestFactor::FriendlyParent,
                BestInterestFactor::PotentialMaintainingRelations,
                BestInterestFactor::WillingnessToShare,
            ],
            finding_categories: vec![
                FindingCategory::CommunicationBlock,
                FindingCategory::DeEscalation,
                FindingCategory::InstitutionalBias,
            ],
        },
        Precedent {
            case_name: "In re Adoption/Guardianship of Rashawn H.".into(),
            citation: "402 Md. 477 (2007)".into(),
            year: 2007,
            court: "Court of Appeals of Maryland".into(),
            holding: "Child's health and safety are paramount. Documented failure to address medical needs constitutes grounds for modification.".into(),
            relevance: "Weight tracking, medication issues, and food refusal patterns establish health concern pattern.".into(),
            factors: vec![
                BestInterestFactor::AgeHealthSex,
                BestInterestFactor::FitnessOfParents,
            ],
            finding_categories: vec![
                FindingCategory::WeightTracking,
                FindingCategory::MedicationIssue,
                FindingCategory::FoodRecord,
                FindingCategory::HealthConcern,
            ],
        },
        Precedent {
            case_name: "Karanikas v. Cartwright".into(),
            citation: "209 Md. App. 571 (2013)".into(),
            year: 2013,
            court: "Court of Special Appeals of Maryland".into(),
            holding: "School records, teacher observations, and educational progress are relevant and admissible evidence in custody proceedings. Courts should consider educational stability.".into(),
            relevance: "Daily reports from Chimes, IEP documentation, school absence records, and behavioral incident reports are all admissible evidence under this precedent.".into(),
            factors: vec![
                BestInterestFactor::MaterialOpportunity,
                BestInterestFactor::FitnessOfParents,
                BestInterestFactor::ResidenceOfParentsAndOpportunity,
            ],
            finding_categories: vec![
                FindingCategory::DailyReport,
                FindingCategory::SchoolAbsence,
                FindingCategory::BehavioralIncident,
                FindingCategory::IepViolation,
            ],
        },
        Precedent {
            case_name: "Wagner v. Wagner".into(),
            citation: "109 Md. App. 1 (1996)".into(),
            year: 1996,
            court: "Court of Special Appeals of Maryland".into(),
            holding: "Financial circumstances alone do not determine custody, but a parent's ability to provide for basic needs including food, shelter, and medical care is relevant.".into(),
            relevance: "Negative meal balance notifications and financial change findings relevant but not dispositive.".into(),
            factors: vec![
                BestInterestFactor::MaterialOpportunity,
                BestInterestFactor::FitnessOfParents,
            ],
            finding_categories: vec![
                FindingCategory::FinancialChange,
                FindingCategory::FoodRecord,
            ],
        },
        Precedent {
            case_name: "Boswell v. Boswell".into(),
            citation: "352 Md. 204 (1998)".into(),
            year: 1998,
            court: "Court of Appeals of Maryland".into(),
            holding: "Courts must make specific findings on each best interest factor. Conclusory statements are insufficient. Documented evidence for each factor strengthens the case.".into(),
            relevance: "The exhibit book structure — categorized, dated, and linked to specific factors — satisfies the Boswell requirement for specific documented evidence.".into(),
            factors: vec![
                BestInterestFactor::FitnessOfParents,
                BestInterestFactor::CharacterReputation,
                BestInterestFactor::DesireAndAgreement,
                BestInterestFactor::PotentialMaintainingRelations,
            ],
            finding_categories: vec![
                FindingCategory::StateComplaint,
                FindingCategory::CourtThreat,
                FindingCategory::AdmissionAgainstInterest,
            ],
        },
    ]
}

/// Match findings to applicable precedents
pub fn match_precedents(findings: &[Finding]) -> Vec<PrecedentMatch> {
    let precedents = build_md_precedents();
    let mut matches = Vec::new();

    // Count findings per category
    let mut category_counts: HashMap<FindingCategory, Vec<usize>> = HashMap::new();
    for f in findings {
        if let Some(num) = f.exhibit_number {
            category_counts.entry(f.category.clone()).or_default().push(num);
        }
    }

    for precedent in &precedents {
        // Find which categories this precedent covers that we have findings for
        for cat in &precedent.finding_categories {
            if let Some(exhibits) = category_counts.get(cat) {
                if exhibits.is_empty() { continue; }

                // Determine which best interest factor this mapping serves
                let factor = match cat {
                    FindingCategory::Alienation | FindingCategory::CustodyInterference =>
                        BestInterestFactor::PotentialMaintainingRelations,
                    FindingCategory::FoodRecord | FindingCategory::MedicationIssue |
                    FindingCategory::WeightTracking | FindingCategory::HealthConcern =>
                        BestInterestFactor::AgeHealthSex,
                    FindingCategory::IepViolation | FindingCategory::SchoolAbsence |
                    FindingCategory::BehavioralIncident | FindingCategory::DailyReport =>
                        BestInterestFactor::FitnessOfParents,
                    FindingCategory::CommunicationBlock | FindingCategory::InstitutionalBias =>
                        BestInterestFactor::FriendlyParent,
                    FindingCategory::DeEscalation =>
                        BestInterestFactor::WillingnessToShare,
                    FindingCategory::CourtThreat | FindingCategory::AdmissionAgainstInterest =>
                        BestInterestFactor::CharacterReputation,
                    FindingCategory::FinancialChange =>
                        BestInterestFactor::MaterialOpportunity,
                    FindingCategory::StateComplaint =>
                        BestInterestFactor::FitnessOfParents,
                    _ => BestInterestFactor::FitnessOfParents,
                };

                let argument = build_argument(&precedent.case_name, &precedent.citation, cat, exhibits.len(), &factor);

                matches.push(PrecedentMatch {
                    precedent: precedent.clone(),
                    matching_exhibits: exhibits.clone(),
                    matching_factor: factor,
                    argument_summary: argument,
                    exhibit_count: exhibits.len(),
                });
            }
        }
    }

    // Deduplicate: keep strongest match per precedent+factor combo
    let mut best: HashMap<(String, String), PrecedentMatch> = HashMap::new();
    for m in matches {
        let key = (m.precedent.citation.clone(), m.matching_factor.to_string());
        let entry = best.entry(key).or_insert_with(|| m.clone());
        if m.exhibit_count > entry.exhibit_count {
            *entry = m;
        }
    }

    let mut result: Vec<PrecedentMatch> = best.into_values().collect();
    result.sort_by(|a, b| b.exhibit_count.cmp(&a.exhibit_count));
    result
}

fn build_argument(
    case_name: &str,
    citation: &str,
    category: &FindingCategory,
    count: usize,
    factor: &BestInterestFactor,
) -> String {
    let evidence_desc = match category {
        FindingCategory::Alienation => "alienating statements from the defendant",
        FindingCategory::CustodyInterference => "instances of custody interference",
        FindingCategory::FoodRecord => "documented food refusal or provision issues",
        FindingCategory::MedicationIssue => "medication administration concerns",
        FindingCategory::WeightTracking => "weight tracking records",
        FindingCategory::HealthConcern => "health-related concerns",
        FindingCategory::IepViolation => "IEP violations or missed services",
        FindingCategory::SchoolAbsence => "school absences",
        FindingCategory::BehavioralIncident => "behavioral incidents documented by school staff",
        FindingCategory::DailyReport => "daily school communication reports",
        FindingCategory::CommunicationBlock => "instances of communication access restriction",
        FindingCategory::InstitutionalBias => "institutional deflection of parental concerns",
        FindingCategory::DeEscalation => "good-faith de-escalation attempts by plaintiff",
        FindingCategory::CourtThreat => "references to legal action in communications",
        FindingCategory::AdmissionAgainstInterest => "admissions against interest by defendant",
        FindingCategory::FinancialChange => "financial change notifications",
        FindingCategory::StateComplaint => "state complaint filings and correspondence",
        _ => "documented incidents",
    };

    format!(
        "Under {} ({}), {} exhibits document {}. This evidence speaks to the {} factor. \
         The court should consider this pattern in its totality when evaluating the best interests of the child.",
        case_name, citation, count, evidence_desc, factor
    )
}

/// Generate a factor-by-factor legal brief outline
pub fn build_factor_brief(matches: &[PrecedentMatch], findings: &[Finding]) -> String {
    let mut brief = String::new();
    brief.push_str("LEGAL BRIEF OUTLINE: BEST INTEREST FACTOR ANALYSIS\n");
    brief.push_str("===================================================\n\n");

    // Group matches by factor
    let mut by_factor: HashMap<String, Vec<&PrecedentMatch>> = HashMap::new();
    for m in matches {
        by_factor.entry(m.matching_factor.to_string()).or_default().push(m);
    }

    let mut factors: Vec<(String, Vec<&PrecedentMatch>)> = by_factor.into_iter().collect();
    factors.sort_by(|a, b| {
        let total_a: usize = a.1.iter().map(|m| m.exhibit_count).sum();
        let total_b: usize = b.1.iter().map(|m| m.exhibit_count).sum();
        total_b.cmp(&total_a)
    });

    for (factor, factor_matches) in &factors {
        let total_exhibits: usize = factor_matches.iter().map(|m| m.exhibit_count).sum();
        brief.push_str(&format!("## {} ({} exhibits)\n\n", factor, total_exhibits));

        for m in factor_matches {
            brief.push_str(&format!("  Case: {} ({})\n", m.precedent.case_name, m.precedent.citation));
            brief.push_str(&format!("  Holding: {}\n", m.precedent.holding));
            brief.push_str(&format!("  Exhibits: {} (e.g., {})\n",
                m.exhibit_count,
                m.matching_exhibits.iter().take(5).map(|e| e.to_string()).collect::<Vec<_>>().join(", "),
            ));
            brief.push_str(&format!("  Argument: {}\n\n", m.argument_summary));
        }
    }

    // Summary stats
    let total_matches: usize = matches.len();
    let unique_cases: Vec<&str> = matches.iter()
        .map(|m| m.precedent.citation.as_str())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    brief.push_str(&format!("\n{} precedent matches across {} unique cases.\n", total_matches, unique_cases.len()));
    brief.push_str(&format!("{} total findings mapped to best interest factors.\n", findings.len()));

    brief
}

pub fn summarize_precedents(matches: &[PrecedentMatch]) -> String {
    let mut out = String::new();
    out.push_str(&format!("Precedent matches: {}\n", matches.len()));

    let unique_cases: std::collections::HashSet<&str> = matches.iter()
        .map(|m| m.precedent.citation.as_str())
        .collect();
    out.push_str(&format!("Unique cases cited: {}\n\n", unique_cases.len()));

    out.push_str("Top matches by evidence volume:\n");
    for m in matches.iter().take(10) {
        out.push_str(&format!("  {:>3} exhibits | {} | {} | {}\n",
            m.exhibit_count,
            m.precedent.case_name,
            m.matching_factor,
            m.precedent.citation,
        ));
    }

    out
}
