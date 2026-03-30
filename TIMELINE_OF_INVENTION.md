<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

### 2026-03-30 — v0.4.0: P13 Tokenization + 4 Court Forms + Exopack + Binary Diet

**What:** Kova P13 compression mapping applied — all 20 public functions (f0-f19) and 23 types (T0-T22) tokenized with doc-comment mappings. Added CC-DC-CV-001 (Case Information Report), CC-DR-004 (Financial Statement), CC-DR-055 (Parenting Plan) court forms. Exopack TRIPLE SIMS quality gate. Removed dead deps (image, base64), moved tokio to test-only. Release binary: 2.5MB stripped.
**Why:** Kova ecosystem alignment. Court filing package was incomplete without mandatory Case Information Report. Binary size matters for zero-cloud deployment.
**Commits:** `05dd1d1` through current
**AI Role:** AI executed P13 rename, form generation, and dep audit. Human directed tokenization scheme, form requirements, and legal content.

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

**What:** 17 Maryland case citations mapped to findings. Automatic legal brief generation with factor-by-factor argument structure.
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

---

*Built by a father, for his custody case. Every finding maps to a real email. Every precedent is a real Maryland court decision.*

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
