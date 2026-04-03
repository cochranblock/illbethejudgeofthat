> **It's not the Mech — it's the pilot.**
>
> This repo is part of [CochranBlock](https://cochranblock.org) — 8 Unlicense Rust repositories that power an entire company on a **single <10MB binary**, a laptop, and a **$10/month** Cloudflare tunnel. No AWS. No Kubernetes. No six-figure DevOps team. Zero cloud.
>
> **[cochranblock.org](https://cochranblock.org)** is a live demo of this architecture. You're welcome to read every line of source code — it's all public domain.
>
> Every repo ships with **[Proof of Artifacts](PROOF_OF_ARTIFACTS.md)** (wire diagrams, screenshots, and build output proving the work is real) and a **[Timeline of Invention](TIMELINE_OF_INVENTION.md)** (dated commit-level record of what was built, when, and why — proving human-piloted AI development, not generated spaghetti).
>
> **Looking to cut your server bill by 90%?** → [Zero-Cloud Tech Intake Form](https://cochranblock.org/deploy)

---

# illbethejudgeofthat

Pro se custody case builder. Google Takeout → Court-ready exhibit book + filled forms.

**Status: scaffold (v0.4.1).** The core pipeline compiles, runs, and produces output. The legal prediction module is stubbed. See [What Works / What Doesn't](#what-works) below.

## What it does

10-stage pipeline from email archive to court-ready filing:

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

## What Works

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

**Known limitations:**
- Filing PDF truncates to one page (no page-break logic yet)
- Legal MoE returns heuristic estimates, not trained predictions
- `legal/` submodules are not compiled into the binary (no `mod legal` in main.rs)
- Counter-motion and discovery response types exist (`T19` variants) but `build_paragraphs` only handles `MotionModifyCustody`
- 49 correctness tests cover date parsing, custody week calc, all finding categories, contradictions, gaps, threads, and precedent matching — no PDF generation tests yet

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

## Output

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
| `MOTION_MODIFY_CUSTODY.pdf` | Filed-ready motion (single page — multi-page TBD) |
| `MEMORANDUM_IN_SUPPORT.pdf` | Supporting memorandum |
| `citation_verification.json` | Citation verification report |
| `CC-DR-007_FILLED.pdf` | Petition to Modify Custody (MD) |
| `CC-DC-CV-001_CASE_INFO_REPORT.pdf` | Case Information Report (required with every MD filing) |
| `CC-DR-004_FINANCIAL_STATEMENT.pdf` | Income/expense disclosure (MD §12-203) |
| `CC-DR-055_PARENTING_PLAN.pdf` | Proposed custody schedule (MD §9-109.1) |

## Compression Map

All public symbols follow [Kova P13 tokenization](docs/compression_map.md). Functions are `f0`-`f19`, types are `T0`-`T22`. Doc comments on each symbol map back to the human name.

## Build

```bash
cargo build --release                                    # 2.5MB stripped binary
cargo test                                               # 49 correctness tests
cargo run --features tests --bin illbethejudgeofthat-test # TRIPLE SIMS quality gate
```

## Built by a father who needed it.

Unlicense
