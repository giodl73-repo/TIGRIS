---
name: WELLSPRING — Concept
slug: wellspring-concept
game: 0032-wellspring
stage: concept
version: 1.0.0
rubric_version: v2.23.3
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: vaccarino
anchor_axis: A4
hopper_source: HOP-007
counter_pressure_note: C4 Engine-Garden Dependency is 7 games since last earn (CANTON #24). WELLSPRING includes explicit engine-garden design elements to reset the C4 monitoring clock before dormancy-watch triggers at 8 games.
---

# WELLSPRING — Concept

## Premise

WELLSPRING is a 2–4 player simultaneous-action deck-builder where variance is the resource. Each player's deck is a "well" of cards organized by pools (stacks of face-down cards grouped by type). On each round, all players simultaneously reveal the top card of their chosen pool, then resolve actions. Unlike Dominion's known-hand model, WELLSPRING's deck is face-down until the moment of resolution — you know your well's composition (the face-up discard history) but not the exact order. The game rewards calibrating how much variance you're willing to carry in your well versus how deterministic you want your engine to be.

The engine-garden layer: purchased cards don't shuffle into a random deck — they are placed face-down in one of 4 personal pool-stacks. You manage not just WHAT's in your deck, but WHERE it is. A well with 3 "Harvest" cards in Pool A is different from a well with 1 in each pool — Pool A becomes reliable; spread pools become flexible.

## Players and length

- **Players:** 2–4
- **Length:** 45–60 min
- **Weight:** 2.5

## Anchor mechanic

**Variance Calibration via Pool Management.** Players manage 4 face-down pool-stacks. Each round: simultaneously reveal the top card of a chosen pool. Cards have actions that fire on reveal. Purchasing adds cards to a chosen pool (not random shuffle). Key decision: concentrate cards in one pool (reliable, high-density outcomes) vs. distribute across pools (flexible, but any pool could fire anything). The well composition is always fully visible (face-up discard pile adjacent to each pool shows its contents); only the order is hidden. This produces managed variance: you know the probability, not the outcome.

## Artifact-as-story

**4 Pool-stacks per player** (labeled by element: Fire, Water, Earth, Air — cosmetic). The face-up discard adjacent to each pool is the information display: players can read what their opponent's well composition is and calibrate their purchases accordingly. The stacks are the artifact — physical tower of cards representing the engine you've built.

## Inspiration / lineage

- **Vaccarino / Dominion (2008)** — deck-building core. WELLSPRING extends Dominion's deck-management into pool-management: instead of a shuffled deck, you have 4 targeted pools.
- **Leacock / Pandemic (2008)** — managed variance: epidemic-spaced shuffle ensures epidemics arrive evenly. WELLSPRING uses the same principle per-pool (you know the composition; the order is managed).
- **Feld / Castles of Burgundy (2011)** — simultaneous play; personal board with distinct tile types. WELLSPRING borrows simultaneous resolution to eliminate downtime.

WELLSPRING's architectural claim: **pool-management as a deck-building extension** — Dominion builds a deck; WELLSPRING builds 4 pools with targeted density. The variance is managed, not random.

## Target review

**Earn-candidates:**
- **A4 Variance Calibration (anchor; Vaccarino)** — pool management is the explicit variance-calibration mechanism. Earn canonical.
- **A2 Decision Density (Feld)** — per-round: which pool to reveal + which card action to pursue; also which pool to place purchases into.
- **C4 Engine-Garden Dependency (Rosenberg)** — the pool-stack architecture IS the garden; concentrated pools become self-sustaining engines. **CRITICAL: must earn to reset C4 monitoring clock.**
- **B3 Conversion Chain (Vaccarino secondary)** — card actions chain: reveal → action → purchase → place in pool → future reveal. 3-step pipeline.
- **A3 Interaction (Chvátil)** — simultaneous reveals create reactive interaction (seeing opponents' reveals adjusts your choice). Indirect but present.
- **B5 Architectural Novelty (Chvátil)** — pool-management deck-building is novel: Dominion's shuffle is replaced by pool-targeting. Test stake.

**Collision candidate:**
- **A4↔B3** — are variance calibration (A4: managing pool composition) and conversion chain depth (B3: reveal→action→purchase→pool) orthogonal registers? A4 fires on the cross-pool strategic management; B3 fires on the single-round pipeline. OP candidate.

## Non-goals

- No hidden hand (all well composition visible via discard piles).
- No shuffle — pool management replaces Dominion's shuffle entirely.
- WELLSPRING does not claim to replace Dominion; it extends one mechanism.

## Open questions

1. **Pool size**: 4 pools of up to 10 cards each = 40-card well maximum. Right size?
2. **Simultaneous reveal timing**: all players reveal simultaneously, then resolve in turn order? Or fully simultaneous including resolution?
3. **C4 design**: the engine-garden requires that concentrated pools create scoring-multiplier effects (e.g., if Pool A has 5+ Harvest cards, each Harvest scores +1 bonus). This is the garden mechanic.
