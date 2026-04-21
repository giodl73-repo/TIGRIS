---
name: NEST — Design (Rules)
slug: nest-design
game: 0026-nest
stage: design
version: 1.0.0
rubric_version: v2.22
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: feld
anchor_axis: C6
dormancy_watch_target: C2
---

# NEST — Rules

## 1. Overview & Object of the Game

NEST is a 2–4 player tile-placement game about constructing a life across three chapters: the early years (to Age 30), the middle years (to Age 50), and the later years (to Age 70). Each player places "life-event" tiles on their personal 6×6 life-board decade by decade, building an irreversible spatial autobiography.

At the end of each chapter (Age 30 / Age 50 / Age 70), a **Decade Rubric** scores all players' boards simultaneously: Stability, Family Density, and Legacy, respectively. Each rubric favors different tile-placement patterns. Players win by constructing a board that performs well across all three rubrics without being dominated by opponents' scores.

**VP totaling:** Sum scores from all three Decade Rubrics. Each rubric begins with a **2 VP base award** (the minimum-score floor) before the rubric modifier is applied. Highest total wins.

## 2. Components

- **4 personal Life-Boards** (6×6 grids; compass-direction labeled for adjacency clarity)
- **Life-Event tile set** (identical set per player; 48 tiles, 8 types × 6 each):
  - House (brown) — stability base; scores on Stability Rubric
  - Partner (pink) — relationship tile; scores on Family Density Rubric
  - Child (blue) — family tile; scores on Family Density Rubric
  - Career (gray) — achievement tile; scores on Stability Rubric
  - Move (orange) — transition tile; resets local adjacency when placed
  - Hobby (green) — scores on Legacy Rubric
  - Health (white) — defensive tile; protects against Rubric penalties
  - Legacy (purple) — end-game tile; scores only on Legacy Rubric, double if isolated
- **1 Decade Track** (shared; marks current decade and which rubric fires next)
- **3 Rubric cards** (one per Decade; public; visible throughout the game)
- **1 VP score board** (0–60 range)
- **4 VP markers**

## 3. Setup

1. Each player takes a Life-Board and a complete set of 48 tiles (shuffled, face-down, into a personal draw-pile).
2. Place all 3 Rubric cards face-up: Stability / Family Density / Legacy.
3. Set the Decade Track to Age 20 (game start; players are building from young adulthood).
4. Determine first player randomly.

## 4. Turn Structure

Each decade (Ages 20–30, 30–50, 50–70) consists of turns until the **Decade Trigger** fires. The Decade Trigger fires when every player has placed 10 tiles in the current decade.

On your turn:
1. **Draw**: draw 3 tiles from your personal draw-pile; choose 1 to place, return 2 to the bottom of your pile.
2. **Place**: place the chosen tile on any empty space on your Life-Board. Tiles may touch any edge; no rotation restrictions. Once placed, tiles are **permanent** — no removal, no repositioning.
3. **Pass** (optional after placing): if you choose not to draw, pass your turn. (This is rare but allows forcing the decade-end before opponents are ready.)

## 5. Actions

| Action | When | Effect |
|---|---|---|
| **Draw and place** | Your turn | Draw 3 tiles, keep 1, place it on Life-Board |
| **Pass** | Your turn (after ≥1 placement this decade) | Skip turn; contributes to decade-end if all others also pass |

The Move tile has a special rule: when placed adjacent to any existing tile, the existing tile's adjacency-scoring relationships are reset (the Move "breaks" one adjacency). This allows late-correction at cost.

## 6. Decade Scoring (Rubric fire)

When all players have placed 10 tiles in a decade, score simultaneously. Each rubric begins with a **2 VP base award**:

### Age 30 — Stability Rubric

**Base: 2 VP.**

Score: +1 VP per House tile adjacent to at least one Career tile. +1 VP per fully-enclosed region (a contiguous group of tiles surrounded on all 4 sides by other tiles or board edges). −1 VP per Move tile placed this decade (each Move represents instability). **Minimum from this rubric: 2 VP (base award only; penalties cannot reduce below 0 from the base).**

### Age 50 — Family Density Rubric

**Base: 2 VP.**

Score: +2 VP per Child tile adjacent to a Partner tile. +1 VP per Partner tile adjacent to at least one other non-Move tile. +1 VP per contiguous group of 3+ Family tiles (Child or Partner or Health). **Minimum from this rubric: 2 VP.**

### Age 70 — Legacy Rubric

**Base: 2 VP.**

Score: +3 VP per Legacy tile that has zero orthogonal neighbors of any type (isolated). +1 VP per Hobby tile in the board's outer ring (outermost row or column). +2 VP per diagonal pair of Legacy tiles (diagonally adjacent). −2 VP per empty space on the board at game end (unused life capacity). **Minimum from this rubric: 2 VP (base; Age 70 penalties are capped so they cannot take a player below 2 VP from this rubric).**

**Total minimum score:** 6 VP (2 VP per rubric × 3 rubrics). A player who places nothing useful still earns 6 VP for having lived.

## 7. Edge Cases & Clarifications

- **Move tile placement**: breaking an adjacency applies immediately when the Move tile is placed. Adjacent tiles lose the adjacency relationship for scoring purposes — but only for the adjacency to the Move tile's location; tiles' other adjacencies are unaffected.
- **Penalties cannot reduce below 0 within a rubric**: if a player has so many Move tiles in Age 30 that the penalty would push their rubric score negative, the floor is 0 from the rubric modifier (plus the 2 VP base award = minimum 2 VP from that rubric).
- **Isolated Legacy tile**: a Legacy tile is "isolated" if it has no orthogonal neighbors AT THE TIME OF SCORING. A Legacy tile that had neighbors removed by a Move tile counts as isolated.
- **Board completion**: players are not required to fill all 36 board spaces. Unfilled spaces incur the Age 70 −2 VP penalty per empty space, incentivizing active tile placement.

## 8. Designer Notes — C2 Dormancy-Watch Documentation

*Required for v2.22 targeted-earn documentation.*

**C2 Minimum-Score Shape** is in dormancy-watch (v2.22). NEST is targeted earn game #2 of 3. The designed floor:

- **Mechanism**: 2 VP base award per Decade Rubric, applied before any scoring modifier. This award is unconditional — every player receives it regardless of tile placement.
- **Total minimum**: 6 VP (2 × 3 rubrics).
- **Design intent**: the floor represents the "minimum dignity" of having engaged with the game — even catastrophic placement earns 6 VP because you participated in each decade of life. This is not an incidental floor; it is an explicit design decision documented in this file.
- **C2 earning condition**: C2 earns if the panel confirms that the designed minimum-score floor is non-trivial (a player genuinely cannot score below 6 VP no matter how badly they play) and creates a meaningful shape for the scoring distribution (not just a formality).

## 9. Variants

**Competitive variant**: add a shared "Milestone board" (6 milestone tiles placed at the game center); players compete for adjacency to Milestone tiles. Adds direct competition to what is otherwise a parallel-solitaire game. Recommended for experienced players.

**Solo variant**: one player vs. a "Life Target" card that sets minimum scores per rubric. If the player meets all three targets, they win.

## 10. Collision Adjacency Chart

| Axes | Adjacency type | Reasoning |
|---|---|---|
| **C6 ↔ A3** | co-firing (predicted) | Incommensurable rubrics on parallel life-boards risk multiplayer-solitaire (same pattern as Wingspan). If A3 fails to earn (insufficient inter-player interaction), C6↔A3 CR collision likely: C6 wins. |
| **D4 ↔ C6** | co-firing (existing) | Lock-in (D4) amplifies incommensurability (C6): you cannot fix a bad tile placement that locked you into the wrong rubric. |
| **C2 ↔ D4** | co-firing (candidate) | The minimum score floor (C2) mitigates the worst lock-in outcomes (D4). If D4 earns because placements lock in, C2 ensures the floor is present. OP candidate: D4 = within-game lock-in; C2 = across-scoring-phases minimum. |

## 11. Open Questions at Design-Stage

1. **C2 floor value**: 2 VP per phase = 6 VP total. Is 6 VP a meaningful floor given likely total scores of 25–45 VP? If the floor is too low relative to the scoring range, C2's minimum shape is cosmetic. Test in panel.
2. **Move tile balance**: Move tiles break adjacency at cost (they're in the tile set, so placing them is mandatory-ish). Is 1 Move tile enough to create meaningful instability, or should each player have 2?
3. **A3 interaction**: the base design is parallel-solitaire. The Competitive Variant's Milestone board adds interaction, but the base game may need one more inter-player mechanism to pass the A3 argument. Consider: "Shared Relationship cards" — each player may once per decade look at an opponent's board for 30 seconds before placing their tile. No change to tiles; adds information-sharing.
4. **Board size**: 6×6 = 36 spaces; 30 tiles per game (3 decades × 10 tiles). 6 spaces unfilled on average. Is this the right density?
