<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

### 2026-03-20 — MoE Legal Prediction + Filing Generator

**What:** Added Mixture of Experts legal prediction (4 experts: Judge disposition patterns, Statute factor scoring, MSDE complaint analysis, Appellate survivability). Citation verifier. Filing generator with exhibit/citation tracing.
**Why:** Case prediction informs legal strategy — know your strengths and weaknesses before filing.
**Commit:** `5948f3f`
**AI Role:** AI built MoE architecture and filing templates. Human designed expert weighting, gating logic, and Maryland-specific court formatting rules.

### 2026-03-15 — IDEA Section Tagging for IEP Findings

**What:** IEP findings now tagged with IDEA (Individuals with Disabilities Education Act) section references.
**Why:** Courts care about statutory violations, not just "the school messed up." Section tagging strengthens legal arguments.
**Commit:** `bf97795`
**AI Role:** AI implemented tagging logic. Human mapped finding patterns to IDEA sections.

### 2026-03-13 — v0.3.0: Full Truth-Derivation Pipeline

**What:** Complete 10-stage pipeline: ingest → extract → thread → analyze → contradict → gaps → precedent → brief → exhibit → forms. Processes 1000+ emails into court-ready filing.
**Why:** A father needed his custody evidence organized in one evening. This pipeline did it.
**Commit:** `1033c38`
**AI Role:** AI generated pipeline code. Human directed every legal requirement, finding category, and output format.

### 2026-03-12 — v0.3.1: Precedent Matching + Legal Brief

**What:** 17 Maryland case citations mapped to findings. Automatic legal brief generation with factor-by-factor argument structure.
**Commit:** `10a7cf2`
**AI Role:** AI wrote brief generation code. Human provided case law and verified citation accuracy.

### 2026-03-10 — Unit Tests + Sample Data

**What:** Real unit tests for parser, keyword detection, scheduling, exhibit numbering. Realistic sample data with fictitious names.
**Commits:** `3a209a7`, `d17fcf7`
**AI Role:** AI generated tests. Human verified test assertions match legal requirements.

### 2026-02-18 — Initial Scaffold

**What:** Full pipeline architecture scaffolded: ingest → parse → analyze → exhibit → forms.
**Why:** Pro se litigant needed court-ready evidence from Google Takeout. No existing tool does this.
**Commit:** `d2620c6`
**AI Role:** AI generated scaffold. Human designed the entire legal workflow based on real custody case experience.

---

*Built by a father, for his custody case. Every finding maps to a real email. Every precedent is a real Maryland court decision.*

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
