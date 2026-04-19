---
name: Dominion Summary — Panel
slug: dominion-panel-SUMMARY
game: 0004-dominion
stage: panel
version: 1.0.0
rubric_version: v2.3
bet_version: parliament
artifact_under_review: games/0004-dominion/design.md
personas_seated: [chvatil (anchor), stegmaier, kramer-kiesling, rosenberg, knizia, lacerda, feld]
total_stakes: 21
earned: 8
refuted_full: 0
retire_explicit: 4
hold_explicit: 5
weak_defended: 3
contested: 1
silent_retire: 0
collisions: 3
axes_adopted: [B5, A6, A2, B1, D4]
axes_retired: []
retirement_reversal: [C4]
protocol_amendment_adopted: [I-parliament-03 → v2.3 anchor-adjacency GATE]
created: 2026-04-19
updated: 2026-04-19
---

# Dominion — Panel Summary

## Headline

**Dominion is the strongest anchor review to date.** 8 earned stakes (highest ever), all 4 success criteria PASS, 5 axis adoptions in a single session, 1 protocol amendment adopted, 1 retirement reversal (C4 Engine-Garden). The rubric now has 7 adopted axes (29% of Pool). The adversarial bet scales.

## Earned stakes (8 — record)

| # | Persona | Axis | Band | Marks | Adoption event? |
|---|---|---|---|---|---|
| 1 | Chvátil | B5 Architectural Novelty (anchor) | B | 2d + 1cl | **YES** → 3rd adoption |
| 2 | Stegmaier | A6 Teachability | A | LAST-CALL | **YES** → 4th adoption |
| 3 | Feld | A2 Decision Density | A | LAST-CALL | **YES** → 5th adoption |
| 4 | Lacerda | B1 System Gearing | B | LAST-CALL | **YES** → 6th adoption |
| 5 | Rosenberg | D4 Late-Game Lock-in | D | 2d + 1cw | **YES** → 7th adoption |
| 6 | Rosenberg | C4 Engine-Garden Dependency | C | 2d + 1cw | Retirement reversal; queued-for-adoption |
| 7 | Lacerda | A7 Emergence-Replayability | A | 2d + 1cw | First earning; queued-for-adoption |
| 8 | K-K | D1 Family-to-Expert Scaling | D | 2d | First earning; queued-for-adoption |

## First cross-player anchor collision (milestone)

Turn 10 collision: **B5 Architectural Novelty (Chvátil anchor) vs A7 Emergence-Replayability (Lacerda)**. 4 games in, the first time an anchor axis and its adjacency partner were drafted by DIFFERENT personas. Outcome: A7 wins 3-2. Sharp, informative collision.

This validates the I-parliament-03 amendment (adopted this session, v2.3): anchor-adjacency partners produce rich collisions when present. Without them, anchors only earn via direct defense.

## Retire-explicit (4 — clean v2.1 retreats)

- Chvátil B2 Catastrophe Pressure (Dominion has no catastrophes)
- K-K D2 Spatial-Interaction Presence (no board)
- Knizia C2 Min-Score Shape (Dominion uses raw VP; diagnostic-low on adopted axis)
- Feld C6 Point-Salad Incommensurability (Dominion is single-metric VP)

## Collisions resolved (3)

1. **B5 ↔ A7 (anchor)** — 3-2 for A7
2. **B1 ↔ C4 (gearing vs engine-garden)** — 3-2 for C4
3. **A1 ↔ C7 (elegance vs menu-clarity)** — 3-2 for C7

All collisions had real dissent (3-2 votes). No unanimous outcomes = genuine disagreement preserved.

## Adoption events — 5 in one session

**Rubric Pool now has 7 adopted axes out of 24 (29%):**

| Axis | Cycle | Source games |
|---|---|---|
| C2 Minimum-Score Shape | v2.2 | P + T&E |
| B4 Info-Transparency-Cost | v2.2 | P + T&E |
| B5 Architectural Novelty | **v2.3** | P + Dominion |
| A6 Teachability | **v2.3** | FACETS + Dominion |
| A2 Decision Density | **v2.3** | FACETS + Dominion |
| B1 System Gearing | **v2.3** | T&E + Dominion |
| D4 Late-Game Lock-in | **v2.3** | FACETS + Dominion |

Three games' adoptions per axis is the canonical pattern (need 2+ earnings + 2+ games). The ledger works.

## Retirement reversal (C4 Engine-Garden)

**C4 trajectory:**
- Parliament: Refuted (1 full refute)
- T&E: Contested (1 defend + 1 collision loss)
- FACETS: Retire-explicit (0.5 refute)
- Dominion: **Earned (2 defends + 1 collision win = 3 marks)**

The retirement-queue status is reversed. Rosenberg's axis, which looked headed for retirement, is rehabilitated by the right context. Dominion is THE engine-building game; the axis shines when it applies.

**I-dominion-02 log entry**: the ledger handles rehabilitation cleanly via normal thresholds. No special logic needed.

## Protocol amendment — I-parliament-03 adopted (v2.3)

**Added to GATE:**

> 4. **Anchor adjacency partner present.** The anchor axis has at least one semantically-adjacent axis drafted by a DIFFERENT persona than the anchor.

Three cluster instances (Parliament, T&E, FACETS) of `anchor_axis_without_collision_partner` + positive counter-example in Dominion (where the check passes naturally).

**Forward-only application.** Games 1-3 remain at their versions; games 5+ enforce the check.

## v2.1 FORCED-ENGAGEMENT validation (3 games sustained)

| Game | Silent-retire rate |
|---|---:|
| Parliament (v2.0) | 48% |
| T&E (v2.1) | 0% |
| FACETS (v2.1) | 0% |
| Dominion (v2.1) | **0%** |

**0% silent-retire across 3 consecutive games under v2.1.** Fix is absolutely stable.

## Success criteria

| # | Criterion | Dominion |
|---|---|---|
| 1 | ≥ 5 earned stakes | **PASS (8)** |
| 2 | ≥ 2 earned on Band B or C | **PASS (3 — B5, B1, C4)** |
| 3 | ≥ 1 clean collision | **PASS (3)** |
| 4 | ≥ 1 axis promoted OR retired | **PASS (5 adoptions + 1 reversal)** |

**All 4 PASS by wide margins.** Dominion is the strongest review so far.

## New innovations

- **I-dominion-01**: First cross-player anchor collision (bet validation).
- **I-dominion-02**: Retirement reversal via earning (protocol validation).
- **I-dominion-03**: Multi-axis adoption event (ledger dynamics pattern).
- **I-dominion-04**: Vaccarino persona candidate (roster gap).

## Cluster updates

- **I-parliament-03**: ADOPTED as v2.3. Cycle 2 rate-limit consumed.
- **I-te-02** (identical drafts): closed-observational (Dominion broke the pattern; not a bug).
- **I-facets-03** (never-drafted axes): proposed-amendment (4 instances). Scheduled for cycle 3.
- **I-dominion-04** (Vaccarino persona): candidate; aligns with I-facets-03 resolution.

## Punchlist for game #5

1. **Decide on I-facets-03 resolution**: add Vaccarino persona with A4/A5/B3 preferences, OR retire those axes, OR accept them as "aspirational" pool members.
2. **Apply v2.3 GATE check** to game #5 before TIER-A. Verify anchor-adjacency partner drafted cross-player.
3. **Track A7, D1, C4 toward second earning** — all three are queued-for-adoption with 1 earning. Game #5 could trigger 2-3 more adoptions.
4. **Consider first retirement event** — C3 (1 refute), C5 (1+2 contested) are closest to retirement but not moving much. A game where they're actively refuted would test the retirement protocol for the first time.

## What Dominion revealed

- **The rubric can recognize genre-founding architecture.** B5 Architectural Novelty earned strongly. Dominion is the canonical case; the axis vindicated.
- **Retirement isn't permanent.** C4 came back from 1.5 refute weight via one strong earning. The forward-only versioning + rehabilitation protocol handles this cleanly.
- **Anchor-adjacency matters.** Dominion's anchor was the first with a cross-player adjacency partner. The resulting collision (B5 ↔ A7) was the richest of the session.
- **Multiple adoptions emerge from rubric-diverse games.** Dominion hits 8 axes strongly; adoption events cluster. Pattern: pick diverse games to build the Pool faster.
- **The bet is 4-games stable.** No sign of degeneracy; rubric evolves by play, not drift.

## Next step

HANDOFF. Cycle 2 complete (FACETS + Dominion). Cycle 3 begins with game #5. Rate limit available for one protocol amendment per cycle (likely I-facets-03 if Vaccarino persona adopted).
