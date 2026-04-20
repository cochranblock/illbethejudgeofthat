# Assumed Breach Threat Model

> **Operating assumption: every component below is already compromised. Design for damage containment and loud detection, not for prevention.**

This document is the canonical threat model for every project in the `cochranblock/*` portfolio. Each project adapts the Threat Surface section for its own context but shares the same first principles, mitigations, and verification protocol.

---

## First Principles

1. **Every record that matters has an external witness.** Hashes published to public git (or equivalent neutral timestamp authority) so tampering requires simultaneously corrupting your system AND the public chain.
2. **No single point of compromise.** Signing keys in hardware (YubiKey / TPM / Secure Enclave). Never in software. Never in env vars. Never in config files.
3. **Default air-gap.** No network dependency for correctness. Network is for backup + publishing hashes, both signed, both verifiable post-hoc.
4. **Append-only everything.** No delete path in any storage layer. Corrections are reversing entries referencing the original. Standard accounting discipline, enforced in code.
5. **Cryptographic audit chain.** Every day's state derives from the previous day's hash. Tampering with any day invalidates every subsequent day.
6. **Disclosure of methodology is a security feature.** If an auditor can independently verify the algorithm, they can independently verify the outputs. No "trust us" layers.
7. **Separation of duties enforced in software.** Entry, approval, and audit live in different trust zones. Compromise of one does not compromise the others.
8. **Redundancy across trust zones.** Local + different-cloud + different-format + offline. Attacker must compromise all to hide damage.
9. **Test breach scenarios regularly.** Triple Sims applied to tamper detection. If the chain does not detect a simulated tamper, the chain is broken.

---

## Threat Surface — illbethejudgeofthat

`illbethejudgeofthat` ingests Google Takeout mbox archives and produces court-filed PDFs (motions, memoranda, exhibit books, Maryland court forms) plus intermediate JSON. Every output lands in an adversarial proceeding where opposing counsel actively probes for inconsistency.

### Records of consequence emitted

| Artifact | Legal weight |
|----------|-------------|
| `PLAINTIFF_EXHIBIT_BOOK.pdf` | Numbered exhibit book filed with court |
| `MOTION_MODIFY_CUSTODY.pdf` | Filed-ready motion |
| `MEMORANDUM_IN_SUPPORT.pdf` | Supporting memorandum with citations |
| `CC-DR-007_FILLED.pdf` | Petition to Modify Custody (MD) |
| `CC-DC-CV-001_CASE_INFO_REPORT.pdf` | Case Information Report (required with every MD filing) |
| `CC-DR-004_FINANCIAL_STATEMENT.pdf` | Income/expense disclosure (MD §12-203) |
| `CC-DR-055_PARENTING_PLAN.pdf` | Proposed custody schedule (MD §9-109.1) |
| `findings.json`, `threads.json`, `contradictions.json`, `gaps.json`, `precedents.json` | Discoverable intermediates — opposing counsel can subpoena |
| `citation_verification.json` | Rule 11 / MD Rule 19-303.3 candor record |
| `timeline.csv` | Spreadsheet-friendly timeline — discoverable |

### Project-specific threats

| Threat | Vector | Impact |
|--------|--------|--------|
| **mbox input tampering** | Opposing party had access to shared Gmail account or device. Archive pruned before Takeout export removes inculpatory messages. | Missing evidence. Detection requires independent proof emails existed (server-side Gmail log, recipient copies, ISP records). |
| **Attachment / MIME misparse** | Malformed message causes `parse.rs` to extract wrong attachment or misattribute sender. | Exhibit book displays wrong content under wrong label. One bad attachment in 10k emails invalidates the exhibit. |
| **Thread reconstruction error** | `thread.rs` stitches by Gmail-ID → In-Reply-To → subject. A wrong stitch presents two unrelated conversations as one. | Misrepresents the record to the court. Judge sees a narrative that never happened. |
| **Finding false positive / negative** | Keyword detection in `analyze.rs` triggers on unrelated context (e.g. "alienation" in a school peer-dynamics memo) or misses coded language. | False positives surviving into the filing are credibility hits. False negatives are abandoned evidence. |
| **Contradiction misattribution** | `contradict.rs` pairs school reports against parent claims. Bad pairing fabricates a contradiction. | Judge sees a contradiction that does not exist. Source and target must be reverifiable from the mbox. |
| **Citation fabrication / misstatement** | `precedent.rs` / `cite_verify.rs` map findings to 17 MD cases. Wrong citation or a case that does not stand for the proposition claimed. | Rule 11 sanctions. MD Rule 19-303.3 candor-to-the-tribunal violation. Credibility destroyed even for pro se litigant. |
| **Court form mis-fill** | Wrong county, case number, children's DOBs on CC-DR-007 or CC-DR-004. | Filing rejected, or admitted with prejudicial errors that opposing counsel exploits. |
| **Post-filing PDF drift** | Exhibit book on disk edited after being filed with court. Disk copy later produced in discovery. | Divergence from court's filed copy = fraud on the court. Hash every filed artifact at filing time. |
| **Opposing-party physical access** | Family-law context: ex-spouse has had keys, shared devices, shared iCloud, known passwords. | Full-disk encryption assumes attacker without credentials; that assumption routinely fails here. Treat device as shared until hardware key separates trust. |
| **Custody week miscalculation** | `--custody-start` anchor drives every finding's "during my week / during their week" tag. Off-by-one on the anchor Thursday flips the tag for every finding. | Every custody-week-tagged finding in the filing is wrong. Opposing counsel cross-references against the court order and impeaches the entire exhibit. |
| **Metadata forgery** | Gmail message IDs, `Received:` headers, `Date:` headers authenticate email evidence under MD Rule 5-901(b)(4). | Tampered metadata → inadmissible hearsay. Entire email chain excluded. |
| **Judicial prediction data poisoning** | `legal/` submodules (currently stubbed) would ingest CaseHarvester CSV, MSDE complaints, appellate opinions, judge rosters. | Poisoned input biases strategy before filing. Gate with signed corpora before wiring in. |
| **Supply chain (deps)** | `mailparse`, `printpdf`, `regex`, `chrono` bugs directly corrupt exhibit content. | Pin, audit, SBOM, reproducible build. A mailparse bug that silently drops a MIME part loses evidence. |

### N/A for current scope

- **Network MITM** — tool is offline CLI. No network on core path.
- **Hardware-key signatures on outputs** — not integrated. User physically signs filings at court. Chain of custody is the filed court docket, not a hardware key. Future: sign each PDF hash at generation time.
- **Public-chain repo** — no `illbethejudgeofthat-chain` yet. Court docket is the external witness; each filed PDF is time-stamped in the court's system. Future: signed hash of each filed PDF to a companion repo at filing time for pre-filing immutability.
- **Separate audit log tree** — no sled storage today (`legal/store.rs` is stubbed). When `legal/` is wired, append-only audit trees apply.
- **DCAA compliance** — civilian family court, not DoD/FAR.

---

## Mitigations

| Assume | Mitigation | Verification |
|--------|-----------|--------------|
| Binary compromised | Hardware-key signatures for every output of consequence | Anyone can verify the public key matches expected fingerprint |
| Storage compromised | Append-only sled trees. Delete is not a function, not a policy. | Hash chain breaks on any rewrite. External witness detects. |
| Network MITM | Air-gap capable. Network used only for signed backups + hash publishing. | NTP + GitHub timestamp + hardware counter cross-checked. |
| Signing key stolen | Daily hash committed to public git. Stolen key cannot retroactively change committed days. | Any day older than the public commit is immutable in evidence. |
| Audit log tampered | Separate sled tree, write-only from main app. Auditor tool reads both + cross-checks. | Compromise of main app leaves audit log intact. |
| Backup tampered | 3 different targets with 3 different credentials (local USB + off-site cloud + paper). | Attacker needs all three to hide damage. |
| Insider / self-tampering | No admin role. No delete. Reversing entries only. | Legal record immune to author second-thoughts. |
| Clock manipulation | Multiple time sources: local clock, NTP, git commit timestamp, hardware-key counter. | Divergence flags exception requiring supervisor approval. |
| Supply chain (deps) | `cargo audit` in CI. Pinned SBOM. Reproducible builds where possible. | Anyone can reproduce the binary from source + lockfile. |
| Physical device seizure | Full-disk encryption. Hardware key physically separate from device. | Stolen laptop without key is useless for forgery. |

---

## Public-Chain Deployment

This project publishes tamper-evident hashes to a public companion repo: `cochranblock/<project>-chain` (where `<project>` is the project name).

- **Daily cycle:** at 23:59 local, compute BLAKE3 of all records-of-consequence from the day. Sign with hardware key. Commit to chain repo. Push.
- **GitHub timestamp** on the commit = neutral third-party witness. Anyone can cold-verify records were not rewritten after commit time.
- **Verification:** `<project> verify` reads the chain and re-derives hashes. Any divergence = tampering detected.

This pattern is a private Certificate Transparency log for project state. Same primitive Google uses for TLS certs, applied to whatever the project tracks.

---

## Triple Sims for Tamper Detection

Standard Triple Sims gate (run 3x identically) extended with a tamper-scenario sim:

1. Normal run → produce canonical output
2. Simulated tampering (flip one bit in storage) → `verify` must flag it
3. Simulated clock rewind → `verify` must flag it

If any sim fails to detect, the chain is broken. Fix before merge.

---

## Scope of this Document

- Covers: any artifact this project emits that has legal, financial, or audit consequence.
- Does NOT cover: source code itself (public under Unlicense, not sensitive), build outputs (reproducible), marketing content (public by design).
- If your project emits no records of consequence, the relevant sections are zero-length and the public-chain deployment is skipped. Document that explicitly.

---

## Relation to Other Docs

- **TIMELINE_OF_INVENTION.md** — establishes priority dates for contributions. Feeds into the chain's initial state.
- **PROOF_OF_ARTIFACTS.md** — cryptographic signatures on release artifacts. Adjacent pattern, same first principles.
- **DCAA_COMPLIANCE.md** (where applicable) — how this threat model satisfies FAR/DFARS audit requirements.

---

## Status

- [x] Threat Surface section adapted for this project
- [ ] Hardware-key signing integrated or N/A documented
- [ ] Public-chain repo created and connected or N/A documented
- [ ] Triple Sims tamper-detection test present or N/A documented
- [ ] External verification procedure documented

---

*Unlicensed. Public domain. Fork, strip attribution, adapt, ship.*

*Canonical source: cochranblock.org/threat-model — last revision 2026-04-14*
