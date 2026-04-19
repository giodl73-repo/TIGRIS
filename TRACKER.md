# TIGRIS Tracker

Per-artifact pipeline state. Most recent rows on top.

## Rubric / bet version history

| Version | Date | Bet | Summary | Proposed by |
|---|---|---|---|---|
| v2.6 | 2026-04-19 | parliament | **First retirement** (C5 Anti-Catch-up). **First new-axis adoption** (B6 Scoring Multiplier Dependency from I-parliament-02 3-cluster). Pool: 25 axes, 24 active. | Puerto Rico anchor review |
| v2.5 | 2026-04-19 | parliament | **4 adoptions** (A7, D1, D2, C4). Pool at 11/24 (46%). First A4/A5/B3 drafts. C4 retirement reversal permanent. | Scythe anchor review |
| v2.4 | 2026-04-19 | parliament | **Vaccarino persona added** (8th designer). Resolves I-facets-03 + I-dominion-04. Rate-limit cycle 3 consumed. | Pre-Scythe |
| v2.3.1 | 2026-04-19 | parliament | **Adjacency chart patch.** I-te-01, I-facets-01, I-facets-02 adopted. Canonical chart at personas/adjacency-chart.md. 17 adjacencies. | Low-risk patch |
| v2.3 | 2026-04-19 | parliament | **5 adoptions + I-parliament-03 protocol amendment.** B5, A6, A2, B1, D4 adopted. Anchor-adjacency GATE check added. | Dominion anchor review |
| v2.2.1 | 2026-04-19 | parliament | FACETS ledger update. 4 new queued-for-adoption axes (A6, A2, D4, C6); C4 tightened to 1.5 refute weight. 6 new innovations. | FACETS anchor review |
| v2.2 | 2026-04-19 | parliament | **First axis adoptions**: C2 Minimum-Score Shape, B4 Information-Transparency-Cost. Both earned in Parliament + T&E. | T&E anchor review |
| v2.1 | 2026-04-19 | parliament | FORCED-ENGAGEMENT micro-phase adopted (I-parliament-01). New states: hold-explicit, retire-explicit, silent-retire. Ignored no longer terminal. | Parliament anchor (48% ignore rate in game #1) |
| v2.0 | 2026-04-19 | parliament | Adversarial-stakes architecture. 24-axis Pool; personas draft 3 each; stakes earned/refuted/contested/ignored/collided. | v1.0 panel review (5-voice chorus cluster) |
| v1.0 | 2026-04-19 | chorus (superseded) | 8 flat axes, per-persona weight vectors, weighted-aggregate scoring. | original spec |

## Games

| Slug | Status | Stage | Rubric ver | Last update | Earned / Refuted / Collisions | Notes |
|---|---|---|---|---|---|---|
| 0001-parliament | **parliament-complete** | HANDOFF | v2.0.1 | 2026-04-19 | 3 / 2 / 4 | Anchor game #1 done. Success criteria: 3/4 pass, 1 fails (≥5-earned → ignore rate 48%). 3 axes queued-for-adoption, 2 queued-for-retirement. Bet validated: Architectural Novelty earned 3 marks. |
| 0002-tigris-and-euphrates | **parliament-complete** | HANDOFF | v2.2 | 2026-04-19 | 5 / 1 / 2 | Anchor game #2 done. All 4 success criteria pass. 0% silent-retire (v2.1 fix validated). **Two adoptions**: C2 Min-Score, B4 Transparency-Cost. 3 queued-for-adoption, 3 queued-for-retirement. |
| 0003-facets | **parliament-complete** | HANDOFF | v2.2.1 | 2026-04-19 | 5 / 1 / 3 | Game #3 (original). Feld-anchor jewel game with 6 incommensurable scoring functions. Criteria 1-3 pass; 4 fails transiently (0 adoptions this game). 0% silent-retire. 4 new axes queued-for-adoption (A6, A2, D4, C6). Skills /tigris-concept, /tigris-design, /tigris-amendment first-used. |
| 0004-dominion | **parliament-complete** | HANDOFF | **v2.3** | 2026-04-19 | **8 / 0 / 3** | Game #4 (review). Chvátil-anchor. **Strongest review to date.** All 4 success criteria PASS. **5 adoptions**: B5 Architectural Novelty, A6 Teachability, A2 Decision Density, B1 System Gearing, D4 Late-Game Lock-in. **Retirement reversal**: C4 Engine-Garden. **Protocol amendment adopted**: I-parliament-03 (anchor-adjacency GATE). First cross-player anchor collision. |
| 0005-scythe | *queued* | — | — | — | — | Anchor #5. Recommend adding Vaccarino persona before start. Tests I-parliament-02 (Scoring Multiplier); Stegmaier-on-Stegmaier. Likely adopts A7, D1, C4. |

## Meta-reviews (reviews of TIGRIS itself, not a game)

| Artifact | Status | Stage | Rubric ver | Last update | Earned / Refuted / Collisions | Notes |
|---|---|---|---|---|---|---|
| docs/specs/2026-04-19-tigris-design.md | parliament-complete (v1.0) | PANEL → INNOVATE | v1.0 | 2026-04-19 | 7 marginal / 0 earned v1.0-shape / n/a | Triggered v2.0 bet change. 9 amendments logged. |
| docs/specs/2026-04-19-tigris-v2.0-design.md | queued for review | — | v2.0 | 2026-04-19 | — | Should be reviewed under v2.0 Parliament procedure *after* Parliament ships, since Parliament must prove the procedure works first. |

## Active amendments (v2.x era)

Parliament (game #1) innovations:
- **I-parliament-01** — Ignored-stake protocol. **ADOPTED v2.1** (2026-04-19). Validated by T&E + FACETS (0% silent-retire across 2 games).
- **I-parliament-02** — Scoring Multiplier Dependency (new axis). Candidate (1 instance). Awaiting Scythe/Wingspan.
- **I-parliament-03** — Anchor-adjacency GATE check. **READY-FOR-ADOPTION** (3 instances clustered after FACETS). Next rate-limit cycle.
- **I-parliament-04** — Architectural Novelty bet validation. Adopted log entry.

T&E (game #2) innovations:
- **I-te-01** — B2↔C5 adjacency chart update. Candidate.
- **I-te-02** — Identical drafts across games. **PROPOSED-AMENDMENT** (3 instances). User decision needed.
- **I-te-03** — FORCED-ENGAGEMENT fix validation. Adopted log entry (strengthened by FACETS).

FACETS (game #3) innovations:
- **I-facets-01** — C6↔A6 new adjacency. Candidate.
- **I-facets-02** — A2↔A6 new adjacency. Candidate.
- **I-facets-03** — Systematically undrafted axes (A4, A5, B3). Candidate — pool curation decision.
- **I-facets-04** — Self-refutation bet validation. Adopted log entry.

Rate-limit status: 1 adopted per 2-game cycle. Cycle 1 consumed by I-parliament-01 (v2.1). **Cycle 2 consumed by I-parliament-03 (v2.3 anchor-adjacency GATE)**. Cycle 3 starts game #5; likely I-facets-03 adoption (Vaccarino persona or retirement of never-drafted axes).

Dominion (game #4) innovations:
- **I-dominion-01** — First cross-player anchor collision. Bet validation. Adopted log entry.
- **I-dominion-02** — Retirement reversal via earning (C4 Engine-Garden). Protocol validation. Adopted log entry.
- **I-dominion-03** — Multi-axis adoption event pattern (5 adoptions in one session). Observational candidate.
- **I-dominion-04** — Vaccarino persona candidate. Justified by lens-gap during Dominion review. Leading candidate for cycle 3 adoption (combined with I-facets-03).

## Session log

| Date | Session | Focus | Outcome |
|---|---|---|---|
| 2026-04-19 | S01 | Scaffolding + v1.0 spec | v1.0 spec written, personas+lenses+skill authored, v1.0 panel review generated (7 marginal, 5-voice orthogonality cluster) |
| 2026-04-19 | S02 | v2.0 restructure | Parliament bet adopted; rubric rebuilt (24-axis Pool); personas swapped to preferred_axes; Parliament scaffolded as anchor #1 |
| 2026-04-19 | S03 | Parliament anchor run | Full pipeline: STAKES → GATE (pass) → ARGUMENT (4 collisions, 3 earned, 2 refuted, 10 ignored) → AMENDMENT (v2.0.1, 3 queued-for-adoption, 2 queued-for-retirement) → HANDOFF. Bet validated. |
| 2026-04-19 | S04 | v2.1 patch | I-parliament-01 adopted. FORCED-ENGAGEMENT micro-phase. Push to GitHub. |
| 2026-04-19 | S05 | T&E anchor #2 run | Full pipeline. All 4 success criteria pass. 5 earned / 1 refuted / 3 retire-explicit / 2 collisions / 0 silent-retire. **First adoptions: C2, B4.** v2.1 fix validated. Rubric → v2.2. |
| 2026-04-19 | S06 | Build skills | /tigris-concept, /tigris-design, /tigris-amendment authored. |
| 2026-04-19 | S07 | FACETS game #3 run | Original Feld-anchor jewel game. Full pipeline via new skills. 5 earned / 1 refuted / 4 retire-explicit / 3 collisions / 0 silent-retire. Rubric → v2.2.1. 4 new queued-for-adoption. Second self-refutation. |
| 2026-04-19 | S08 | Dominion anchor #4 | Chvátil-anchor review of Vaccarino's genre-founding game. 8 earned / 0 refuted / 4 retire-explicit / 3 collisions / 0 silent-retire. **5 adoptions** + I-parliament-03 adopted → **v2.3**. Retirement reversal on C4. First cross-player anchor collision. Strongest review to date. |
| 2026-04-19 | S09 | v2.3.1 + v2.4 patches | Adjacency chart canonical file (personas/adjacency-chart.md, 17 pairs). Vaccarino persona added (8th designer). |
| 2026-04-19 | S10 | Scythe anchor #5 | Stegmaier-on-Stegmaier. First under v2.4 (8 personas). 24 drafts cover 24-axis Pool (full coverage first time). 9 earned / 0 refuted / 3 retire-explicit / 2 collisions / 0 silent-retire. **4 adoptions** (A7, D1, D2, C4) → **v2.5**. First A4/A5/B3 drafts ever. |
| 2026-04-19 | S11 | Puerto Rico anchor #6 | Knizia-on-dormant-C1. 10 earned / 1 refuted / 4 retire-explicit / 1 collision / 0 silent-retire. **First retirement event (C5)** + **first new-axis adoption (B6 SMD)** → **v2.6**. 4 first-earnings queued (C1, C3, B3, C7). Knizia first fallback-draft (A5). |
