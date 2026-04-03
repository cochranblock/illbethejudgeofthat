<!-- Unlicense — cochranblock.org -->

# Proof of Artifacts

*Concrete evidence that this project works, ships, and is real.* **(v0.4.1)**

> Google Takeout emails → court-ready exhibit books, legal briefs, and filled forms in one evening.

## Architecture

```mermaid
flowchart TD
    Takeout[Google Takeout mbox] --> Ingest[1. Ingest: mailparse]
    Ingest --> Extract[2. Extract Attachments]
    Extract --> Thread[3. Reconstruct Threads]
    Thread --> Analyze[4. Analyze: 20 Finding Categories]
    Analyze --> Contradict[5. Contradiction Detection: 5 Rules]
    Contradict --> Gaps[6. Timeline Gap Analysis: 4 Types]
    Gaps --> Precedent[7. Precedent Matching: MD §9-101.1]
    Precedent --> Brief[8. Legal Brief Generation]
    Brief --> Exhibit[9. Exhibit Book PDF]
    Exhibit --> Forms[10. Court Forms + Filing]
    Analyze --> MoE[MoE Legal Prediction]
    MoE --> Judge[Judge Expert: 35%]
    MoE --> Statute[Statute Expert: 30%]
    MoE --> Complaint[Complaint Expert: 20%]
    MoE --> Appellate[Appellate Expert: 15%]
```

## Build Output

| Metric | Value |
|--------|-------|
| Lines of Rust | 6,135 (+ 781 lines of tests) |
| Pipeline stages | 10 (ingest → forms) |
| Finding categories | 20 (IEP violation, alienation, food record, behavioral incident, admission against interest, etc.) |
| Contradiction types | 5 (school vs. parent, food refusal, attendance, custody week, behavioral) |
| Gap types | 4 (missing reports, communication silence, custody week gaps, thread abandonment) |
| Precedent cases | 11 mapped to findings, 20 in citation verification DB |
| Best interest factors | 12 (MD §9-101.1 mapping) |
| MoE experts | 4 (Judge, Statute, Complaint, Appellate) with gating network — heuristic scoring, sled DB stubs not yet populated |
| Sample case output | 500+ findings, 100+ contradictions, 50+ gaps → court-ready filing |

## Key Artifacts

| Artifact | Description |
|----------|-------------|
| 10-Stage Pipeline | Raw mbox → exhibit book in one command. No manual exhibit numbering |
| Finding Extraction | 20 custody-relevant categories with custody-week assignment (plaintiff vs. defendant) |
| Contradiction Engine | Cross-references school reports against parent claims — surfaces documentary lies |
| Timeline Gap Analysis | Detects missing daily reports, communication silences, thread abandonment |
| Precedent Matching | 11 Maryland cases mapped to findings → automatic brief with case law + exhibit cross-refs (20 in citation DB) |
| MoE Legal Prediction | 4-expert architecture with gating network + challenge layer (flags weaknesses) |
| Citation Verifier | 20 known MD cases checked — flags bad format, missing, or unverified citations |
| Filing Generator | Motion to Modify Custody + Memorandum in Support with exhibit/citation cross-refs |
| Exhibit Book PDF | Cover, TOC, numbered exhibits, contradiction summaries, gap analysis — court-formatted |
| Court Form Generation | 4 MD forms: CC-DR-007 (Petition), CC-DC-CV-001 (Case Info), CC-DR-004 (Financial), CC-DR-055 (Parenting Plan) |
| P13 Compression | All public symbols tokenized (f0-f19, T0-T22) with compression map |
| TRIPLE SIMS | Exopack quality gate — cargo test + binary smoke test, 3x pass required |
| 49 Correctness Tests | Every finding category, custody week calc, contradictions, gaps, threads, precedent matching — known inputs, expected outputs |
| P23 Paranoia Lens | Red team audit of Kova pyramid architecture (mmap security, training data poisoning, priority starvation, failure modes) |

## How to Verify

```bash
cargo build --release -p illbethejudgeofthat
cargo run --release -- \
  --input path/to/takeout.mbox \
  --plaintiff "Name" --defendant "Name" \
  --children "Child1" --dobs "01/15/2018" \
  --state MD --county "Anne Arundel"
ls filing/   # findings.json, contradictions.json, gaps.json, PLAINTIFF_EXHIBIT_BOOK.pdf, MOTION_MODIFY_CUSTODY.pdf
```

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
