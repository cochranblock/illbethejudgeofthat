<!-- Unlicense — cochranblock.org -->

# Proof of Artifacts — illbethejudgeofthat

*Hard evidence that this project is real, working, and built by humans with AI assistance — not AI hallucination.*

## Project Metrics

| Metric | Value |
|--------|-------|
| Source files (.rs) | 24 |
| Lines of code | 6,307 (+ 1,637 lines of tests) |
| Tests | 86 |
| Commits | 28 |
| Binary size (release) | 2.5 MB |
| Dependencies (direct) | 8 |
| Edition | 2021 |
| MSRV | 1.85 |
| License | Unlicense |

## Repository

- **GitHub:** https://github.com/cochranblock/illbethejudgeofthat
- **Live deployment:** CLI tool — runs locally on user's machine, no server required

## Architecture

Google Takeout mbox → court-ready exhibit books, legal briefs, and filled forms in one command. 10-stage pipeline: ingest (mailparse) → extract attachments → reconstruct threads → analyze (20 finding categories with custody-week attribution) → contradiction detection (5 rules) → timeline gap analysis (4 types) → precedent matching (MD §9-101.1) → legal brief generation → exhibit book PDF → court forms + filing. Mixture of Experts legal prediction (4 experts: Judge, Statute, Complaint, Appellate) with gating network scores case strength.

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

## Named Techniques

| Technique | Description | Commit |
|-----------|-------------|--------|
| Truth-Derivation Pipeline | 10-stage mbox → court filing in one command | `1033c38` |
| Custody-Week Attribution | Every finding tagged with which parent had custody | `1033c38` |
| P13 Compression Mapping | All public symbols tokenized (f0-f19, T0-T22) | `d7be0b8` |
| Sender Guard | Custody interference findings skip plaintiff's own emails | `6f1e816` |
| Dynamic Thursday Default | `most_recent_thursday()` replaces hardcoded custody anchor | `08fbef4` |

*See TIMELINE_OF_INVENTION.md for full provenance on each technique.*

## Test Coverage

| Category | Count |
|----------|-------|
| Date parsing | 6 |
| Custody week calculation | 7 |
| Finding categories (14 categories with sender guards) | 14 |
| Contradiction detection | 3 |
| Gap analysis (daily reports, weekends, silence, threads) | 5 |
| Thread reconstruction (3-tier matching) | 3 |
| Precedent matching | 3 |
| Pipeline stages | 5 |
| Regex coverage | 7 |
| End-to-end mbox run | 7 |
| PDF smoke tests | 5 |
| most_recent_thursday() correctness | 5 |
| Sample mbox validation | 6 |
| **Total** | **86** |

TRIPLE SIMS quality gate: `illbethejudgeofthat-test` runs full suite 3x via exopack.

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

## Compliance

- SBOM: embedded in release binary
- SSDF: aligned with NIST SP 800-218
- CISA Secure-by-Design: memory-safe Rust
- EO 14028: aligned

## Build

```
cargo build --release
cargo test
```

## Verification

A third party can verify every claim in this document:

1. **Clone and build:** `git clone` + `cargo build --release` — zero errors, zero warnings.
2. **Run tests:** `cargo test` — 86 tests pass covering every pipeline stage.
3. **Quality gate:** `cargo run --bin illbethejudgeofthat-test --features tests` — TRIPLE SIMS 3x pass.
4. **Commit history:** `git log --oneline` — 28 commits match every entry in TIMELINE_OF_INVENTION.md.
5. **Binary smoke test:** `cargo run -- --help` prints all CLI flags matching README documentation.
6. **End-to-end:** Feed `examples/sample.mbox` through the pipeline — 22 emails parse, findings generate, PDF outputs to `filing/`.
7. **Compression map:** `docs/compression_map.md` maps every tokenized symbol to its original name.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
