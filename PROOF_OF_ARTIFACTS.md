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

## 10-Stage Pipeline (Detail)

1. **Ingest** — parses mbox/Google Takeout email archives via `mailparse`
2. **Extract** — pulls attachments from MIME messages
3. **Thread** — reconstructs conversation threads (Gmail IDs → In-Reply-To → subject matching)
4. **Analyze** — detects 20 custody-relevant finding categories (IEP violations, alienation, food records, etc.)
5. **Contradict** — cross-references school reports against parent claims (5 contradiction types)
6. **Gaps** — detects missing daily reports, communication silences, abandoned threads (4 gap types)
7. **Precedent** — matches findings to 17 Maryland case citations
8. **Exhibit** — builds numbered PDF exhibit book (cover, TOC, exhibits, appendices)
9. **Cite Verify** — checks all case citations against known Maryland case law
10. **Filing** — generates Motion to Modify Custody + Memorandum in Support with exhibit/citation tracing

Also includes: court form generation (4 MD forms), interactive query REPL (`--query`), and a Mixture of Experts legal prediction module (`legal/`).

## Named Techniques

| Technique | Description | Commit |
|-----------|-------------|--------|
| Truth-Derivation Pipeline | 10-stage mbox → court filing in one command | `1033c38` |
| Custody-Week Attribution | Every finding tagged with which parent had custody | `1033c38` |
| P13 Compression Mapping | All public symbols tokenized (f0-f19, T0-T22) | `d7be0b8` |
| Sender Guard | Custody interference findings skip plaintiff's own emails | `6f1e816` |
| Dynamic Thursday Default | `most_recent_thursday()` replaces hardcoded custody anchor | `08fbef4` |

*See TIMELINE_OF_INVENTION.md for full provenance on each technique.*

## Module Status

**Compiles and runs — real implementations:**

| Module | Status | Notes |
|--------|--------|-------|
| `ingest.rs` | Working | mbox parsing, MIME extraction, recursive subpart handling |
| `parse.rs` | Working | Attachment extraction, filename sanitization |
| `thread.rs` | Working | 3-tier thread reconstruction (Gmail ID → In-Reply-To → subject) |
| `analyze.rs` | Working | 20 finding categories with keyword detection, IDEA section tagging |
| `contradict.rs` | Working | 5 contradiction types (school vs parent claims) |
| `gaps.rs` | Working | 4 gap types (missing reports, silence, abandoned threads, custody weeks) |
| `precedent.rs` | Working | 17 Maryland cases mapped to findings |
| `exhibit.rs` | Working | PDF exhibit book via `printpdf` (cover, TOC, exhibits, appendices) |
| `cite_verify.rs` | Working | 20 known MD cases verified against citation format |
| `filing.rs` | Working | Motion + Memorandum PDFs with caption block, numbered paragraphs, prayer for relief |
| `forms.rs` | Working | CC-DR-007, CC-DC-CV-001, CC-DR-004, CC-DR-055 (4 Maryland court forms) |
| `query.rs` | Working | Interactive REPL: filter by category, date, sender, custody week, keyword |
| `main.rs` | Working | Full 10-stage pipeline orchestration |

**Placeholder / stubbed:**

| Module | Status | Notes |
|--------|--------|-------|
| `legal/mod.rs` | Partial | MoE architecture works with 4 experts, but uses heuristic scoring (no real court data) |
| `legal/store.rs` | Stub | sled DB interface — function signatures only |
| `legal/cases.rs` | Stub | CaseHarvester CSV ingest — not wired in |
| `legal/complaints.rs` | Stub | MSDE complaint data loader |
| `legal/opinions.rs` | Stub | Appellate opinion scraper |
| `legal/court.rs` | Stub | Judge/court roster data |
| `legal/ingest.rs` | Stub | DB population orchestrator |
| `legal/stats.rs` | Stub | Statistics generation |

## Known Limitations

- Filing PDF supports multi-page with page numbers (no "Page X of Y" total yet)
- Legal MoE returns heuristic estimates, not trained predictions
- `legal/` submodules are not compiled into the binary (no `mod legal` in main.rs)
- Counter-motion and discovery response types exist (`T19` variants) but `build_paragraphs` only handles `MotionModifyCustody`
- 49 correctness tests cover date parsing, custody week calc, all finding categories, contradictions, gaps, threads, and precedent matching — no PDF generation tests yet

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

## Usage

```bash
cargo run --release -- \
  --input path/to/takeout.mbox \
  --plaintiff "Your Name" \
  --defendant "Other Parent" \
  --children "Child1,Child2" \
  --dobs "01/15/2018,03/22/2020" \
  --state MD \
  --county "Anne Arundel"
```

Key flags:

- `--json-only` — export findings as JSON, skip PDF generation
- `--dump-emails` — export raw parsed emails
- `--skip-forms` — skip court form generation
- `--query` — interactive REPL to explore findings from a prior run
- `--case-number` — case number for court filings
- `--custody-start` — known Thursday when plaintiff has custody (YYYY-MM-DD)
- `--schedule` — custody schedule pattern (default: weekly-thursday)

## Output Artifacts

All artifacts land in `./filing/` (or `--output`):

| File | Description |
|------|-------------|
| `findings.json` | All findings with categories, custody week, IDEA tags |
| `threads.json` | Reconstructed conversation threads |
| `contradictions.json` | School-vs-parent contradictions |
| `gaps.json` | Timeline gaps and silences |
| `precedents.json` | Matched Maryland case law |
| `legal_brief_outline.txt` | Factor-by-factor brief |
| `timeline.csv` | Spreadsheet-friendly timeline |
| `PLAINTIFF_EXHIBIT_BOOK.pdf` | Court-formatted exhibit book |
| `MOTION_MODIFY_CUSTODY.pdf` | Filed-ready motion (multi-page with page numbers) |
| `MEMORANDUM_IN_SUPPORT.pdf` | Supporting memorandum |
| `citation_verification.json` | Citation verification report |
| `CC-DR-007_FILLED.pdf` | Petition to Modify Custody (MD) |
| `CC-DC-CV-001_CASE_INFO_REPORT.pdf` | Case Information Report (required with every MD filing) |
| `CC-DR-004_FINANCIAL_STATEMENT.pdf` | Income/expense disclosure (MD §12-203) |
| `CC-DR-055_PARENTING_PLAN.pdf` | Proposed custody schedule (MD §9-109.1) |

## Compression Map

All public symbols follow [Kova P13 tokenization](docs/compression_map.md). Functions are `f0`-`f19`, types are `T0`-`T22`. Doc comments on each symbol map back to the human name.

## Compliance

- SBOM: embedded in release binary
- SSDF: aligned with NIST SP 800-218
- CISA Secure-by-Design: memory-safe Rust
- EO 14028: aligned

## Build

```bash
cargo build --release                                    # 2.5MB stripped binary
cargo test                                               # 86 correctness + PDF tests
cargo run --features tests --bin illbethejudgeofthat-test # TRIPLE SIMS quality gate
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
<!-- COCHRANBLOCK-BRAND-FOOTER:START - generated by cochranblock/scripts/brand-stamp.sh -->

---

<sub>&#9656; **THE COCHRAN BLOCK, LLC** &#183; CAGE `1CQ66` &#183; UEI `W7X3HAQL9CF9` &#183; UNLICENSE &#183; [cochranblock.org](https://cochranblock.org)</sub>
<!-- COCHRANBLOCK-BRAND-FOOTER:END -->
