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

## What it does

1. **Ingest** — parses mbox/Google Takeout email archives
2. **Extract** — pulls attachments from MIME messages
3. **Thread** — reconstructs conversation threads (Gmail IDs → In-Reply-To → subject matching)
4. **Analyze** — detects 19 custody-relevant finding categories (IEP violations, alienation, food records, etc.)
5. **Contradict** — cross-references school reports against parent claims
6. **Gaps** — detects missing daily reports, communication silences, abandoned threads
7. **Precedent** — matches findings to 17 Maryland case citations
8. **Exhibit** — builds numbered PDF exhibit book (cover, TOC, exhibits, appendices)
9. **Cite Verify** — checks all case citations against known Maryland case law
10. **Filing** — generates Motion to Modify Custody + Memorandum in Support with exhibit/citation tracing

Also includes: court form generation (MD CC-DR-007), interactive query REPL (`--query`), and a Mixture of Experts legal prediction engine (`legal/`).

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
| `MOTION_MODIFY_CUSTODY.pdf` | Filed-ready motion |
| `MEMORANDUM_IN_SUPPORT.pdf` | Supporting memorandum |
| `citation_verification.json` | Citation verification report |
| `CC-DR-007_FILLED.pdf` | Filled court form (MD) |

## Built by a father who needed it.

Unlicense
