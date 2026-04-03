<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

### 2026-04-03 — 49 Correctness Tests + Bug Fix + P23 Paranoia Lens

**What:** Replaced 7 toy tests (string contains, arithmetic — none called crate functions) with 49 real correctness tests. Every major module tested with known inputs and expected outputs: date parsing (6 tests), custody week calculation (7 tests, every day of week verified), all 14 finding categories with sender guards, contradiction detection, gap analysis (daily report gaps, weekend exclusion, communication silence threshold, abandoned threads), thread reconstruction (3-tier matching), precedent matching (alienation→Domingues, food→health, IEP→fitness). Added src/lib.rs to expose modules for integration test access.

**Bug found and fixed:** Custody interference (food attribution) triggered on plaintiff's own emails. Added sender guard — now skips when sender is plaintiff (analyze.rs).

**P23 Paranoia Lens:** Red team audit of Kova pyramid architecture. Identified 10 attack vectors and failure modes: unsigned nanobyte model files (no integrity verification), training data poisoning via unfiltered crates.io corpus, unauthenticated localhost HTTP API, 60s discovery blind spot on node failure, priority queue starvation with no timeout, confidence miscalibration at 0.7 escalation threshold, bridge log injection for future model corruption, TOCTOU on mmap'd weights, split brain in mesh (no consensus), and the irreversibility of Phase 4 API key deletion.

**Commits:** `47be791` (docs), `1e6964c` (doc corrections), `6f1e816` (tests + bug fix)
**AI Role:** AI wrote all tests, found custody interference bug, conducted P23 paranoia audit. Human directed test priorities and red team scope.

### 2026-03-30 — v0.4.0: P13 Tokenization + Dead Dep Removal + Binary Diet

**What:** Kova P13 compression mapping applied — all 20 public functions (f0-f19) and 23 types (T0-T22) tokenized with doc-comment mappings. `docs/compression_map.md` added. Removed dead deps (image 0.25, base64 0.22 — never imported). Moved tokio to optional test-only. Main binary is now fully synchronous. .gitignore expanded. README, POA, TOI updated. Release binary: 2.5MB stripped.
**Commit:** `d7be0b8`
**AI Role:** AI executed P13 rename and dep audit. Human directed tokenization scheme and ecosystem alignment.

### 2026-03-27 — Zero Clippy Warnings + AI Slop Eradication

**What:** Fixed 8 clippy warnings: regex compiled in loop (analyze.rs), needless borrow (exhibit.rs), useless format! (filing.rs, forms.rs), redundant trim() (query.rs), too_many_arguments justified with allow (exhibit.rs, forms.rs). Eradicated P12 slop word "comprehensive" from precedent.rs.
**Commit:** `99ea077`
**AI Role:** AI identified and fixed all warnings. Human directed P12 compliance.

### 2026-03-27 — CC-DR-004 Financial Statement + CC-DR-055 Parenting Plan

**What:** Two new MD court forms. CC-DR-004: income/expense disclosure per MD Family Law §12-203 (gross income, deductions, monthly expenses, children, verification under penalty of perjury). CC-DR-055: two-page parenting plan per MD Family Law §9-109.1 (legal/physical custody, weekly schedule, 14-holiday rotation table, summer/vacation, decision-making, communication, right of first refusal, relocation, health/safety provisions, FERPA school access).
**Commit:** `a24f62d`
**AI Role:** AI generated form structure. Human specified MD statutory requirements and legal content.

### 2026-03-27 — CC-DC-CV-001 Case Information Report

**What:** Added CC-DC-CV-001 per MD Rule 20-201 — required with every circuit court domestic filing. Auto-fills court/county, parties (pro se flagged), children with computed ages, case type (custody modification), relief sought, related cases, ADR status, estimated trial time.
**Commit:** `2c16fd6`
**AI Role:** AI generated form. Human specified that clerk rejects filing package without this form.

### 2026-03-27 — Exopack: TRIPLE SIMS Quality Gate + Stripped Release Binary

**What:** Exopack optional dep behind `tests` feature. `illbethejudgeofthat-test` binary runs cargo test + --help smoke test through TRIPLE SIMS (3x). Release profile: opt-level z, LTO, codegen-units 1, strip, panic=abort.
**Commit:** `f3e894b`
**AI Role:** AI implemented exopack integration following oakilydokily/kova pattern. Human directed quality gate design.

### 2026-03-27 — Docs: Update All Docs to Reflect 10-Stage Pipeline

**What:** README rewritten with full CLI usage, all flags, output table, all 10 pipeline stages. PROOF_OF_ARTIFACTS: fixed line count and CLI flags. TIMELINE_OF_INVENTION: fixed chronological ordering. main.rs: fixed stage numbering from inconsistent [1/7]...[10/10] to [1/10]...[10/10]. Version bumped to 0.3.2.
**Commit:** `05dd1d1`
**AI Role:** AI audited all docs against actual code and fixed every discrepancy. Human directed scope.

### 2026-03-21 — Proof of Artifacts + Timeline + Zero-Cloud Banner

**What:** Added Proof of Artifacts, Timeline of Invention, and CochranBlock zero-cloud banner to README.
**Why:** Every repo ships with provenance docs — concrete evidence the work is real and human-directed.
**Commit:** `4a93553`
**AI Role:** AI drafted templates. Human verified all claims and artifact descriptions.

### 2026-03-20 — MoE Legal Prediction + Filing Generator + Citation Verifier

**What:** Added Mixture of Experts legal prediction (4 experts: Judge disposition patterns, Statute factor scoring, MSDE complaint analysis, Appellate survivability). Citation verifier (20 known MD cases). Filing generator with exhibit/citation tracing (Motion to Modify Custody + Memorandum in Support).
**Why:** Case prediction informs legal strategy — know your strengths and weaknesses before filing.
**Commit:** `5948f3f`
**AI Role:** AI built MoE architecture and filing templates. Human designed expert weighting, gating logic, and Maryland-specific court formatting rules.

### 2026-03-15 — IDEA Section Tagging for IEP Findings

**What:** IEP findings now tagged with IDEA (Individuals with Disabilities Education Act) section references.
**Why:** Courts care about statutory violations, not just "the school messed up." Section tagging strengthens legal arguments.
**Commit:** `bf97795`
**AI Role:** AI implemented tagging logic. Human mapped finding patterns to IDEA sections.

### 2026-03-14 — v0.3.1: Precedent Matching + Legal Brief

**What:** 11 Maryland case citations mapped to findings (20 in citation verification DB). Automatic legal brief generation with factor-by-factor argument structure.
**Commit:** `10a7cf2`
**AI Role:** AI wrote brief generation code. Human provided case law and verified citation accuracy.

### 2026-03-13 — v0.3.0: Full Truth-Derivation Pipeline

**What:** Complete 10-stage pipeline: ingest → extract → thread → analyze → contradict → gaps → precedent → brief → exhibit → forms. Processes 1000+ emails into court-ready filing.
**Why:** A father needed his custody evidence organized in one evening. This pipeline did it.
**Commit:** `1033c38`
**AI Role:** AI generated pipeline code. Human directed every legal requirement, finding category, and output format.

### 2026-03-10 — Unit Tests + Sample Data

**What:** Real unit tests for parser, keyword detection, scheduling, exhibit numbering. Realistic sample data with fictitious names.
**Commits:** `3a209a7`, `d17fcf7`
**AI Role:** AI generated tests. Human verified test assertions match legal requirements.

### 2026-02-18 — Initial Scaffold

**What:** Full pipeline architecture scaffolded: ingest → parse → analyze → exhibit → forms.
**Why:** Pro se litigant needed court-ready evidence from Google Takeout. No existing tool does this.
**Commit:** `d2620c6`
**AI Role:** AI generated scaffold. Human designed the entire legal workflow based on real custody case experience.

### 2026-04-01 — v0.4.1: Honest Docs + Warning Fixes

**What:** README rewritten with scaffold status — "What Works" vs "Placeholder" tables, known limitations documented. Fixed 2 test warnings (unused PathBuf import, unused start_day variable). Tracked litigation support docs (email_to_opposing_counsel.md, exhibit_work_activity.md). All docs audited: line count corrected (4,482 → 6,125), finding categories (19 → 20), precedent count clarified (11 matched, 20 in citation DB).
**Commit:** `47be791` (README + warnings + tracking), plus doc correction commit
**AI Role:** AI audited all docs against actual code and fixed every discrepancy. Human directed honesty-first approach.

---

*Built by a father, for his custody case. Every finding maps to a real email. Every precedent is a real Maryland court decision.*

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
