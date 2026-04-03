<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized work stack. Most important at top. Max 20 items.

Cross-project deps: **exopack** (TRIPLE SIMS, integrated), **kova** (inference/RAG for legal MoE, not yet integrated), **ironhive** (swarm sync, available).

---

1. [test] End-to-end pipeline test — feed a real mbox through all 10 stages, assert finding counts and output files exist
2. [feature] Counter-motion template — T19::MotionContempt build_paragraphs branch (filter findings to order violations, show willfulness)
3. [feature] Discovery response template — T19::DiscoveryResponse with interrogatory-answer format and exhibit references
4. [feature] --filing-type CLI arg — let user choose which document to generate (default: motion-modify)
5. [build] Wire legal/ module — add sled/reqwest/bincode/zstd as optional deps behind `legal` feature flag, add `mod legal` to lib.rs
6. [build] Fix legal/mod.rs crate::config::sled_path() — either add config module or inline path as ~/.illbethejudgeofthat/legal.sled
7. [feature] --predict flag — run legal MoE (f370) after pipeline, output prediction report with expert scores and weaknesses
8. [feature] --legal-ingest subcommand — populate sled DB from CaseHarvester CSV + MSDE complaints + appellate opinions
9. [research] Kova RAG integration — replace hardcoded 11 precedent cases with dynamic retrieval via kova f166 (fastembed semantic search over MD case law)
10. [test] Analyze category edge cases — negation detection ("did NOT refuse food"), context-dependent alienation, rhetorical questions in gap detection
11. [docs] Architecture doc — data flow diagram, pipeline stage dependencies, finding→exhibit→citation trace
12. [build] Distribute TRIPLE SIMS across swarm — ironhive push to n1/gd, run test binary remotely, pull results (20c/31G available)
13. [fix] Sender normalization — "Mom <mom@gmail.com>" vs "mom@gmail.com" treated as different senders in gap detection
14. [feature] NanoSign model integrity — if legal MoE ships trained models, sign nanobyte files per kova NanoSign spec (NSIG + BLAKE3)
15. [test] Leap year age calculation — children born Feb 29 return "—" on non-leap years in forms.rs compute_age()
16. [docs] Examples update — convert examples/sample_emails.txt to proper mbox format so users can actually run the pipeline
17. [research] P23 optimist + pessimist lenses — paranoia lens done; complete the triple lens on pyramid architecture for full synthesis
