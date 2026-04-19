# TIGRIS Tracker

Per-artifact pipeline state. Most recent rows on top.

## Rubric / bet version history

| Version | Date | Bet | Summary | Proposed by |
|---|---|---|---|---|
| v2.2 | 2026-04-19 | parliament | **First axis adoptions**: C2 Minimum-Score Shape, B4 Information-Transparency-Cost. Both earned in Parliament + T&E. | T&E anchor review |
| v2.1 | 2026-04-19 | parliament | FORCED-ENGAGEMENT micro-phase adopted (I-parliament-01). New states: hold-explicit, retire-explicit, silent-retire. Ignored no longer terminal. | Parliament anchor (48% ignore rate in game #1) |
| v2.0 | 2026-04-19 | parliament | Adversarial-stakes architecture. 24-axis Pool; personas draft 3 each; stakes earned/refuted/contested/ignored/collided. | v1.0 panel review (5-voice chorus cluster) |
| v1.0 | 2026-04-19 | chorus (superseded) | 8 flat axes, per-persona weight vectors, weighted-aggregate scoring. | original spec |

## Games

| Slug | Status | Stage | Rubric ver | Last update | Earned / Refuted / Collisions | Notes |
|---|---|---|---|---|---|---|
| 0001-parliament | **parliament-complete** | HANDOFF | v2.0.1 | 2026-04-19 | 3 / 2 / 4 | Anchor game #1 done. Success criteria: 3/4 pass, 1 fails (≥5-earned → ignore rate 48%). 3 axes queued-for-adoption, 2 queued-for-retirement. Bet validated: Architectural Novelty earned 3 marks. |
| 0002-tigris-and-euphrates | **parliament-complete** | HANDOFF | v2.2 | 2026-04-19 | 5 / 1 / 2 | Anchor game #2 done. All 4 success criteria pass. 0% silent-retire (v2.1 fix validated). **Two adoptions**: C2 Min-Score, B4 Transparency-Cost. 3 queued-for-adoption, 3 queued-for-retirement. |
| 0003-agricola | *queued* | — | — | — | — | Anchor #3. Rosenberg-on-Rosenberg; should trigger B1/B2/D2 adoptions and surface first retirements. |

## Meta-reviews (reviews of TIGRIS itself, not a game)

| Artifact | Status | Stage | Rubric ver | Last update | Earned / Refuted / Collisions | Notes |
|---|---|---|---|---|---|---|
| docs/specs/2026-04-19-tigris-design.md | parliament-complete (v1.0) | PANEL → INNOVATE | v1.0 | 2026-04-19 | 7 marginal / 0 earned v1.0-shape / n/a | Triggered v2.0 bet change. 9 amendments logged. |
| docs/specs/2026-04-19-tigris-v2.0-design.md | queued for review | — | v2.0 | 2026-04-19 | — | Should be reviewed under v2.0 Parliament procedure *after* Parliament ships, since Parliament must prove the procedure works first. |

## Active amendments (v2.x era)

Parliament (game #1) innovations:
- **I-parliament-01** — Ignored-stake protocol. **ADOPTED v2.1** (2026-04-19). Validated by T&E (0% silent-retire).
- **I-parliament-02** — Scoring Multiplier Dependency (new axis). Candidate (1 instance). Awaiting Scythe/Wingspan.
- **I-parliament-03** — Anchor-adjacency GATE check. **PROPOSED-AMENDMENT** (2 instances now clustered). Scheduled for next rate-limit cycle.
- **I-parliament-04** — Architectural Novelty bet validation. Adopted log entry.

T&E (game #2) innovations:
- **I-te-01** — B2↔C5 adjacency chart update. Candidate; low-risk, could adopt as v2.2.1 patch.
- **I-te-02** — Identical drafts across games. Observational candidate.
- **I-te-03** — FORCED-ENGAGEMENT fix validation. Adopted log entry.

Rate-limit status: 1 adopted per 2-game cycle. Cycle 1 consumed by I-parliament-01. Cycle 2 likely consumed by I-parliament-03 after game #3.

## Session log

| Date | Session | Focus | Outcome |
|---|---|---|---|
| 2026-04-19 | S01 | Scaffolding + v1.0 spec | v1.0 spec written, personas+lenses+skill authored, v1.0 panel review generated (7 marginal, 5-voice orthogonality cluster) |
| 2026-04-19 | S02 | v2.0 restructure | Parliament bet adopted; rubric rebuilt (24-axis Pool); personas swapped to preferred_axes; Parliament scaffolded as anchor #1 |
| 2026-04-19 | S03 | Parliament anchor run | Full pipeline: STAKES → GATE (pass) → ARGUMENT (4 collisions, 3 earned, 2 refuted, 10 ignored) → AMENDMENT (v2.0.1, 3 queued-for-adoption, 2 queued-for-retirement) → HANDOFF. Bet validated. |
| 2026-04-19 | S04 | v2.1 patch | I-parliament-01 adopted. FORCED-ENGAGEMENT micro-phase. Push to GitHub. |
| 2026-04-19 | S05 | T&E anchor #2 run | Full pipeline. All 4 success criteria pass. 5 earned / 1 refuted / 3 retire-explicit / 2 collisions / 0 silent-retire. **First adoptions: C2, B4.** v2.1 fix validated. Rubric → v2.2. |
