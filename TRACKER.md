# TIGRIS Tracker

Per-artifact pipeline state. Most recent rows on top.

## Rubric / bet version history

| Version | Date | Bet | Summary | Proposed by |
|---|---|---|---|---|
| v2.1 | 2026-04-19 | parliament | FORCED-ENGAGEMENT micro-phase adopted (I-parliament-01). New states: hold-explicit, retire-explicit, silent-retire. Ignored no longer terminal. | Parliament anchor (48% ignore rate in game #1) |
| v2.0 | 2026-04-19 | parliament | Adversarial-stakes architecture. 24-axis Pool; personas draft 3 each; stakes earned/refuted/contested/ignored/collided. | v1.0 panel review (5-voice chorus cluster) |
| v1.0 | 2026-04-19 | chorus (superseded) | 8 flat axes, per-persona weight vectors, weighted-aggregate scoring. | original spec |

## Games

| Slug | Status | Stage | Rubric ver | Last update | Earned / Refuted / Collisions | Notes |
|---|---|---|---|---|---|---|
| 0001-parliament | **parliament-complete** | HANDOFF | v2.0.1 | 2026-04-19 | 3 / 2 / 4 | Anchor game #1 done. Success criteria: 3/4 pass, 1 fails (≥5-earned → ignore rate 48%). 3 axes queued-for-adoption, 2 queued-for-retirement. Bet validated: Architectural Novelty earned 3 marks. |
| 0002-tigris-and-euphrates | *queued* | — | — | — | — | Anchor #2. Should follow v2.1 protocol patch for ignored-stakes (I-parliament-01). |

## Meta-reviews (reviews of TIGRIS itself, not a game)

| Artifact | Status | Stage | Rubric ver | Last update | Earned / Refuted / Collisions | Notes |
|---|---|---|---|---|---|---|
| docs/specs/2026-04-19-tigris-design.md | parliament-complete (v1.0) | PANEL → INNOVATE | v1.0 | 2026-04-19 | 7 marginal / 0 earned v1.0-shape / n/a | Triggered v2.0 bet change. 9 amendments logged. |
| docs/specs/2026-04-19-tigris-v2.0-design.md | queued for review | — | v2.0 | 2026-04-19 | — | Should be reviewed under v2.0 Parliament procedure *after* Parliament ships, since Parliament must prove the procedure works first. |

## Active amendments (v2.0 era)

Four innovations logged from Parliament (game #1):
- **I-parliament-01** — Ignored-stake protocol too permissive (48% ignore rate). Candidate. Recommended for adoption in v2.1 before game #2.
- **I-parliament-02** — Scoring Multiplier Dependency (new axis candidate). Candidate; awaiting game #2 evidence.
- **I-parliament-03** — Anchor axis without collision partner fragile. Candidate; awaiting second instance.
- **I-parliament-04** — Architectural Novelty validation (bet log). Adopted as validation entry (not a protocol change). The v2.0 bet produced evidence for itself.

## Session log

| Date | Session | Focus | Outcome |
|---|---|---|---|
| 2026-04-19 | S01 | Scaffolding + v1.0 spec | v1.0 spec written, personas+lenses+skill authored, v1.0 panel review generated (7 marginal, 5-voice orthogonality cluster) |
| 2026-04-19 | S02 | v2.0 restructure | Parliament bet adopted; rubric rebuilt (24-axis Pool); personas swapped to preferred_axes; Parliament scaffolded as anchor #1 |
| 2026-04-19 | S03 | Parliament anchor run | Full pipeline: STAKES → GATE (pass) → ARGUMENT (4 collisions, 3 earned, 2 refuted, 10 ignored) → AMENDMENT (v2.0.1, 3 queued-for-adoption, 2 queued-for-retirement) → HANDOFF. Bet validated. |
