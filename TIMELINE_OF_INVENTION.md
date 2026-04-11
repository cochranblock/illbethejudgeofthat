<!-- Unlicense — cochranblock.org -->

# Timeline of Invention — illbethejudgeofthat

*Dated, commit-level record of what was built, when, and why. Proves human-piloted AI development — not generated spaghetti.*

> Every entry below maps to real commits. Run `git log --oneline` to verify.

## How to Read This Document

Each entry follows this format:

- **Date**: When the work shipped (not when it was started)
- **What**: Concrete deliverable — binary, feature, fix, architecture change
- **Why**: Business or technical reason driving the decision
- **Commit**: Short hash(es) for traceability
- **AI Role**: What the AI did vs. what the human directed
- **Proof**: Link to artifact, screenshot, test output, or live URL

This document exists because AI-assisted code has a trust problem. Anyone can generate 10,000 lines of spaghetti. This timeline proves that a human pilot directed every decision, verified every output, and shipped working software.

---

## Human Revelations — Invented Techniques

*Novel ideas that came from human insight, not AI suggestion. These are original contributions to the field.*

### Truth-Derivation Pipeline (2026-03-13)

**Invention:** 10-stage pipeline that converts raw Google Takeout mbox into court-ready exhibit books, briefs, and forms in one command.

**The Problem:** Pro se litigants drown in email evidence. No tool exists to go from raw export to courtroom in one step.

**The Insight:** Custody cases follow predictable evidentiary patterns — school reports, contradictions, timeline gaps, statutory factors. Automate the pattern recognition, keep the human as the legal strategist.

**The Technique:** Chain ingest → extract → thread → analyze → contradict → gaps → precedent → brief → exhibit → forms. Each stage produces structured JSON that feeds the next. 20 finding categories with custody-week assignment, 5 contradiction types, 4 gap types, 11 precedent matches.

**Result:** 1000+ emails processed into a complete court filing package in under 60 seconds.

**Named:** Truth-Derivation Pipeline
**Commit:** `1033c38`
**Origin:** A father needed his custody evidence organized in one evening. Built from firsthand courtroom experience, not legal AI marketing.

### Custody-Week Attribution (2026-03-13)

**Invention:** Every finding tagged with which parent had custody that week — plaintiff or defendant.

**The Problem:** Courts care about *who had the child* when an incident happened. Raw email analysis misses this.

**The Insight:** Custody schedules are deterministic. Given a known anchor Thursday and alternating weeks, every date maps to a parent. Attribute findings automatically.

**The Technique:** `CustodySchedule` computes weeks_diff from anchor date. Even weeks = plaintiff, odd = defendant. Every finding carries `custody_week: Plaintiff | Defendant`.

**Result:** Judges see at a glance which parent's time produced which problems. No manual annotation needed.

**Named:** Custody-Week Attribution
**Commit:** `1033c38`
**Origin:** Real custody case — a father realized the school complaints clustered on the other parent's weeks.

---

## Entries

*Reverse chronological. Most recent first.*

### 2026-04-10 — Standardize TOI/POA to Canonical Templates

**What:** Reformatted TIMELINE_OF_INVENTION.md and PROOF_OF_ARTIFACTS.md to match canonical CochranBlock templates. Added "How to Read" section, Human Revelations, Project Metrics table, Named Techniques, Test Coverage, Compliance, and Verification sections. Added 2026-04-10 entry for this work.
**Why:** All CochranBlock repos ship with consistent provenance docs. Standardization proves the ecosystem is coordinated, not ad hoc.
**Commit:** `pending`
**AI Role:** AI reformatted docs to match templates. Human directed template conformance and selected which techniques to highlight as Human Revelations.
**Proof:** Diff shows structural alignment with `~/.claude/templates/` canonical templates.

### 2026-04-07 — P23 Triple Lens: Readjust Fire + 27 New Tests + Multi-Page PDF

**What:** 27 new tests covering pipeline stages, regex coverage, and end-to-end mbox run. Multi-page PDF support with 5 PDF smoke tests. `most_recent_thursday()` dynamic default replaces hardcoded 2025-01-02 custody start. Valid `examples/sample.mbox` with 22 emails. P23 triple lens readjustment.
**Why:** Backlog items #1 and #3 — the test suite needed real pipeline coverage and the PDF generator needed multi-page support.
**Commit:** `08fbef4`, `486a205`, `7e62a8e`
**AI Role:** AI wrote tests, PDF multi-page logic, and sample mbox. Human directed test priorities and backlog triage.
**Proof:** `cargo test` — 86 tests pass. `examples/sample.mbox` parses to 22 emails.

### 2026-04-03 — 49 Correctness Tests + Bug Fix + P23 Paranoia Lens

**What:** Replaced 7 toy tests (string contains, arithmetic — none called crate functions) with 49 real correctness tests. Every major module tested with known inputs and expected outputs: date parsing (6 tests), custody week calculation (7 tests, every day of week verified), all 14 finding categories with sender guards, contradiction detection, gap analysis (daily report gaps, weekend exclusion, communication silence threshold, abandoned threads), thread reconstruction (3-tier matching), precedent matching (alienation→Domingues, food→health, IEP→fitness). Added src/lib.rs to expose modules for integration test access.
**Why:** The old tests were self-licking ice cream — none exercised real code paths.
**Commit:** `6f1e816`, `d40449a`
**AI Role:** AI wrote all tests, found custody interference bug, conducted P23 paranoia audit. Human directed test priorities and red team scope.
**Proof:** `cargo test` — 49 tests pass. Bug fix in analyze.rs sender guard.

### 2026-04-01 — v0.4.1: Honest Docs + Warning Fixes

**What:** README rewritten with scaffold status — "What Works" vs "Placeholder" tables, known limitations documented. Fixed 2 test warnings (unused PathBuf import, unused start_day variable). Tracked litigation support docs (email_to_opposing_counsel.md, exhibit_work_activity.md). All docs audited: line count corrected (4,482 → 6,125), finding categories (19 → 20), precedent count clarified (11 matched, 20 in citation DB).
**Why:** Honesty-first approach — docs must match what the code actually does.
**Commit:** `47be791`, `1e6964c`
**AI Role:** AI audited all docs against actual code and fixed every discrepancy. Human directed honesty-first approach.
**Proof:** README "What Works" vs "Placeholder" tables verified against source.

### 2026-03-30 — v0.4.0: P13 Tokenization + Dead Dep Removal + Binary Diet

**What:** Kova P13 compression mapping applied — all 20 public functions (f0-f19) and 23 types (T0-T22) tokenized with doc-comment mappings. `docs/compression_map.md` added. Removed dead deps (image 0.25, base64 0.22 — never imported). Moved tokio to optional test-only. Main binary is now fully synchronous. .gitignore expanded. Release binary: 2.5MB stripped.
**Why:** Token economy — compressed identifiers reduce AI context consumption.
**Commit:** `d7be0b8`
**AI Role:** AI executed P13 rename and dep audit. Human directed tokenization scheme and ecosystem alignment.
**Proof:** `docs/compression_map.md` maps every public symbol. Binary size verified at 2.5MB.

### 2026-03-27 — Zero Clippy Warnings + AI Slop Eradication

**What:** Fixed 8 clippy warnings: regex compiled in loop (analyze.rs), needless borrow (exhibit.rs), useless format! (filing.rs, forms.rs), redundant trim() (query.rs), too_many_arguments justified with allow (exhibit.rs, forms.rs). Eradicated P12 slop word "comprehensive" from precedent.rs.
**Why:** Zero-warning builds and clean language are non-negotiable.
**Commit:** `99ea077`
**AI Role:** AI identified and fixed all warnings. Human directed P12 compliance.
**Proof:** `cargo clippy` — zero warnings.

### 2026-03-27 — CC-DR-004 Financial Statement + CC-DR-055 Parenting Plan

**What:** Two new MD court forms. CC-DR-004: income/expense disclosure per MD Family Law §12-203 (gross income, deductions, monthly expenses, children, verification under penalty of perjury). CC-DR-055: two-page parenting plan per MD Family Law §9-109.1 (legal/physical custody, weekly schedule, 14-holiday rotation table, summer/vacation, decision-making, communication, right of first refusal, relocation, health/safety provisions, FERPA school access).
**Why:** These forms are required in Maryland custody filings.
**Commit:** `a24f62d`
**AI Role:** AI generated form structure. Human specified MD statutory requirements and legal content.
**Proof:** Generated forms match Maryland court clerk requirements.

### 2026-03-27 — CC-DC-CV-001 Case Information Report

**What:** Added CC-DC-CV-001 per MD Rule 20-201 — required with every circuit court domestic filing. Auto-fills court/county, parties (pro se flagged), children with computed ages, case type (custody modification), relief sought, related cases, ADR status, estimated trial time.
**Why:** Clerk rejects filing package without this form.
**Commit:** `2c16fd6`
**AI Role:** AI generated form. Human specified that clerk rejects filing package without this form.
**Proof:** Form fields match MD Rule 20-201 requirements.

### 2026-03-27 — Exopack: TRIPLE SIMS Quality Gate + Stripped Release Binary

**What:** Exopack optional dep behind `tests` feature. `illbethejudgeofthat-test` binary runs cargo test + --help smoke test through TRIPLE SIMS (3x). Release profile: opt-level z, LTO, codegen-units 1, strip, panic=abort.
**Why:** Every CochranBlock project ships with a quality gate — no exceptions.
**Commit:** `f3e894b`
**AI Role:** AI implemented exopack integration following oakilydokily/kova pattern. Human directed quality gate design.
**Proof:** `cargo run -p illbethejudgeofthat --bin illbethejudgeofthat-test --features tests` passes 3x.

### 2026-03-27 — Docs: Update All Docs to Reflect 10-Stage Pipeline

**What:** README rewritten with full CLI usage, all flags, output table, all 10 pipeline stages. PROOF_OF_ARTIFACTS: fixed line count and CLI flags. TIMELINE_OF_INVENTION: fixed chronological ordering. main.rs: fixed stage numbering from inconsistent [1/7]...[10/10] to [1/10]...[10/10]. Version bumped to 0.3.2.
**Why:** Docs must match code. Stage numbering was wrong.
**Commit:** `05dd1d1`
**AI Role:** AI audited all docs against actual code and fixed every discrepancy. Human directed scope.
**Proof:** CLI `--help` output matches README.

### 2026-03-21 — Proof of Artifacts + Timeline + Zero-Cloud Banner

**What:** Added Proof of Artifacts, Timeline of Invention, and CochranBlock zero-cloud banner to README.
**Why:** Every repo ships with provenance docs — concrete evidence the work is real and human-directed.
**Commit:** `4a93553`
**AI Role:** AI drafted templates. Human verified all claims and artifact descriptions.
**Proof:** Documents exist in repo root.

### 2026-03-20 — MoE Legal Prediction + Filing Generator + Citation Verifier

**What:** Added Mixture of Experts legal prediction (4 experts: Judge disposition patterns, Statute factor scoring, MSDE complaint analysis, Appellate survivability). Citation verifier (20 known MD cases). Filing generator with exhibit/citation tracing (Motion to Modify Custody + Memorandum in Support).
**Why:** Case prediction informs legal strategy — know your strengths and weaknesses before filing.
**Commit:** `5948f3f`
**AI Role:** AI built MoE architecture and filing templates. Human designed expert weighting, gating logic, and Maryland-specific court formatting rules.
**Proof:** Filing output in `filing/` directory with cross-referenced exhibits and citations.

### 2026-03-15 — IDEA Section Tagging for IEP Findings

**What:** IEP findings now tagged with IDEA (Individuals with Disabilities Education Act) section references.
**Why:** Courts care about statutory violations, not just "the school messed up." Section tagging strengthens legal arguments.
**Commit:** `bf97795`
**AI Role:** AI implemented tagging logic. Human mapped finding patterns to IDEA sections.
**Proof:** Finding output includes IDEA section references.

### 2026-03-14 — v0.3.1: Precedent Matching + Legal Brief

**What:** 11 Maryland case citations mapped to findings (20 in citation verification DB). Automatic legal brief generation with factor-by-factor argument structure.
**Why:** Pro se litigants need case law backing — judges dismiss unsupported arguments.
**Commit:** `10a7cf2`
**AI Role:** AI wrote brief generation code. Human provided case law and verified citation accuracy.
**Proof:** Generated brief includes Maryland case citations with exhibit cross-references.

### 2026-03-13 — v0.3.0: Full Truth-Derivation Pipeline

**What:** Complete 10-stage pipeline: ingest → extract → thread → analyze → contradict → gaps → precedent → brief → exhibit → forms. Processes 1000+ emails into court-ready filing.
**Why:** A father needed his custody evidence organized in one evening. This pipeline did it.
**Commit:** `1033c38`
**AI Role:** AI generated pipeline code. Human directed every legal requirement, finding category, and output format.
**Proof:** Pipeline runs end-to-end on real mbox input. Output in `filing/` directory.

### 2026-03-10 — Unit Tests + Sample Data

**What:** Real unit tests for parser, keyword detection, scheduling, exhibit numbering. Realistic sample data with fictitious names.
**Why:** Tests must exercise real code paths with real data patterns.
**Commits:** `3a209a7`, `d17fcf7`
**AI Role:** AI generated tests. Human verified test assertions match legal requirements.
**Proof:** `cargo test` passes.

### 2026-02-18 — Initial Scaffold

**What:** Full pipeline architecture scaffolded: ingest → parse → analyze → exhibit → forms.
**Why:** Pro se litigant needed court-ready evidence from Google Takeout. No existing tool does this.
**Commit:** `d2620c6`
**AI Role:** AI generated scaffold. Human designed the entire legal workflow based on real custody case experience.
**Proof:** Compiles with zero errors. Architecture proven by subsequent pipeline completion.

---

*Built by a father, for his custody case. Every finding maps to a real email. Every precedent is a real Maryland court decision.*

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
