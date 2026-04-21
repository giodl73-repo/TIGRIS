---
name: NEST — Concept
slug: nest-concept
game: 0026-nest
stage: concept
version: 1.0.0
rubric_version: v2.22
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: feld
anchor_axis: C6
hopper_source: HOP-005
dormancy_watch_target: C2
counter_pressure_note: >
  C2 Minimum-Score Shape is in dormancy-watch (v2.22). NEST is targeted earn game #2 of 3.
  NEST must include a designed minimum-score floor — a mechanism that prevents players from
  scoring below a threshold regardless of tile placement quality. Designed mechanism:
  each of NEST's 3 scoring phases (Age 30 / Age 50 / Age 70) awards a base minimum
  of 2 VP to every player before applying the phase rubric. A player who places no useful
  tiles still scores 6 VP total (2 per phase). This is the C2 designed floor.
---

# NEST — Concept

## Premise

NEST is a 2–4 player polyomino tile-placement game about building a domestic life across three life chapters. Over 30 in-game years (three rounds of ten years each), players place "life-event" tiles on their personal 6×6 life-boards: houses, children, career milestones, moves, relationships. Once placed, tiles are permanent — the game is a Brass-level irreversibility system at domestic scale.

At the end of each decade (Age 30, Age 50, Age 70), a different scoring rubric fires:
- **Age 30 (Stability)**: scores compact adjacency clusters — "did you build a stable foundation?"
- **Age 50 (Family Density)**: scores adjacent child-tiles and relationship-tiles — "who did you build it with?"
- **Age 70 (Legacy)**: scores diagonal reach and isolated late-life tiles — "what did you leave behind?"

No rubric can be fully maximized without partially sacrificing the others. A player who front-loads stability tiles for Age 30 may have a rigid life-board that performs poorly on the diagonal-reach Legacy rubric.

**C2 dormancy-watch design note:** Each scoring phase awards a base **2 VP floor** to every player before the phase rubric is applied. A player with a completely suboptimal board scores at minimum 6 VP (2 per phase). The floor is designed: the base VP represents the baseline dignity of "having lived" — it cannot be taken away by poor tile placement. This is NEST's explicit C2 counter-pressure: a designed minimum-score shape.

## Players and length

- **Players:** 2–4
- **Length:** 60–90 min
- **Weight:** 3.0

## Anchor mechanic

**Phase-Incommensurable Scoring.** Three scoring rubrics (Stability / Family Density / Legacy) fire sequentially at decades' ends, each rewarding different tile-adjacency patterns. A Stability-optimal board (dense, compact, central) is incompatible with a Legacy-optimal board (spread, diagonal, isolated). The game rewards foresight: placing tiles in the early decades that will score well across multiple future rubrics, while accepting that full optimization of any one rubric sacrifices the others.

## Artifact-as-story

**The 6×6 personal life-board** — each player's is unique by the end of the game. The board is a spatial autobiography: where tiles are placed reveals what choices were made and when. Tiles from the early decades are usually more central; late-decade tiles often inhabit the edges (moves, late-career changes, legacy projects). The board is the game's physical storytelling surface.

## Inspiration / lineage

- **Feld / Castles of Burgundy (2011)** — personal tile-placement board with phase scoring. NEST borrows the personal-grid + phase-turn structure.
- **Lacerda / Brass: Birmingham (adjacent via Wallace)** — permanent irreversible decisions; early choices create the conditions for late scoring. NEST borrows the forward-commitment design ethos.
- **Kiesling / Azul (2017)** — polyomino shapes on personal boards; spatial adjacency scoring. NEST borrows the tile-placement + adjacency mechanic.

## Target review in the TIGRIS pipeline

**Earn-candidates:**
- **C6 Point-Salad Incommensurability (anchor; Feld)** — 3 incommensurable rubrics. Canonical C6.
- **C2 Minimum-Score Shape (dormancy-watch targeted)** — 2 VP floor per phase = 6 VP minimum total. Designed floor. Target earn.
- **D4 Late-Game Lock-in (Rosenberg)** — tile placements are permanent; early-game decisions lock in late-game scoring options. Canonical D4.
- **A1 Elegance (Knizia)** — minimal rule-count (place tile, score at decade-end) with deep strategic depth.
- **B1 System Gearing (Lacerda)** — tile placement gears into adjacency scoring gears into phase rubric gears into VP.
- **A2 Decision Density (Feld)** — each tile placement is a multi-rubric decision: where to place this tile that works for all 3 future rubric fires.

**Dormancy-watch critical:** C2 MUST earn in NEST or Furrow (#27) to avoid de-adoption review.

**Retire-explicit candidates:** A3 (multiplayer-solitaire risk per hopper tension hypothesis; same pattern as Wingspan), B5 (no novelty claim), B2 (no escalating catastrophe).

## Non-goals

- No player interaction beyond shared tile-supply competition. Parallel life-boards; no blocking.
- No randomness beyond tile-draw order (identical tile sets per player; only draw order varies).
- No legacy elements — NEST is strictly anti-one-shot (all tiles returned to box each game).

## Open questions (to resolve during DESIGN)

1. **C2 floor calibration**: 2 VP per phase = 6 VP minimum. Is this right? Too high may trivialize catastrophic play; too low may fail to earn C2. The floor must be "designed" (intentional) not emergent.
2. **Tile-set composition**: how many tile-types? Current estimate: 8 tile-types (House, Partner, Child, Career, Move, Hobby, Health, Legacy). Each type fits different scoring rubrics differently.
3. **A3 interaction risk**: hopper predicts C6↔A3 collision (Wingspan pattern). If parallel life-boards produce multiplayer-solitaire, A3 refutes and C6 wins the collision. Design mitigation: include a small inter-player mechanism (e.g., shared Family-tree tiles that can only be placed if a partner player has placed a Partner tile).
