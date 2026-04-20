---
name: The Castles of Burgundy — Design (Rules summary, imported)
slug: castles-of-burgundy-design
game: 0012-castles-of-burgundy
stage: design
version: 1.0.0
rubric_version: v2.10
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
canonical_source: The Castles of Burgundy rulebook, alea / Ravensburger 2011 (Alea big-box #16)
bgg_id: 84876
---

# The Castles of Burgundy — Rules Summary (Review Import)

Condensed summary for panel review. Canonical rules in alea/Ravensburger 2011 rulebook. This summary is critical commentary, not reproduction.

## 1. Overview

2–4 players are 15th-century Burgundian princes developing their estates. Over 5 rounds × 5 turns = 25 turns, each player rolls 2 dice and converts them into actions: acquire tiles from the central board, place tiles on their estate, ship goods for VP, or take workers (which modify dice). Filled regions (groups of same-terrain tiles) trigger placement bonuses; these bonuses scale with the current round. Goods sales happen once per phase. Highest VP wins.

**Object:** accumulate most VP across 6 scoring tracks (placement VP, filled-region bonuses, shipped goods, livestock count, tech/knowledge tile VP, endgame completion bonuses).

## 2. Components

- 1 central board (phase-divided depot + numbered tile holding areas, 1–6)
- 4 base player estate boards (hex-tile grids in 6 terrain regions: castle / mine / ship / pasture / building / knowledge) — expansion adds ~20 more variant boards
- ~160 tiles across 6 types: buildings, livestock (pig/sheep/cow/chicken), ships (with goods), knowledge/tech tiles, mines (silver), castles
- Goods barrels (4 colors), silver coins, worker tokens, 2 dice per player
- VP chips + score track
- Turn/round phase markers

## 3. Setup

Each player receives an estate board. Hex-tile grid has 6 region-types partitioned into groups of 2–7 hexes. Starting tiles filled (castle hex).

Central board populated: depot holds tiles (numbered 1–6 slots matching dice values). Goods barrels on the goods track. Silver/livestock markets set.

Turn order marker; first player chosen randomly.

## 4. Turn structure — 5 rounds × 5 turns per player

### 4.1 Phase sequence per round

1. Replenish central depot tiles (slots 1–6 refill from stacks).
2. Goods market refill (new goods barrels in play).
3. Each player takes 5 turns in player order.
4. End-of-round: round bonus tiles score at round-dependent scaling.

### 4.2 Per-turn actions

Each turn: **roll 2 white dice**. Spend each die on ONE of:

- **Acquire a tile** (take tile from central depot slot matching die value; place in private depot)
- **Place a tile** from private depot onto estate (must match die value = target hex number)
- **Ship goods** (dice must match goods' value; returns VP + silver + optional livestock tile)
- **Take 2 workers** (any die value; workers modify future dice by ±1 each)

And, once per phase (not per turn):
- **Sell 1 good** from surplus for 3 doubloons (converts to silver)

Dice can be modified by spending workers (±1 per worker). Silver can be spent to purchase any tile from slot (flexibility).

## 5. Key subsystems

### 5.1 Estate board (hex grid)

Each estate is partitioned into **6 terrain regions** by color:
- **Castle (dark blue)** — castles placed via dice; instant turn-repeat effect in placement
- **Mine (brown)** — mines; produce silver each round-end
- **Ship (light blue)** — ships; trigger shipping actions + turn order
- **Pasture (green)** — animals stacked; scale non-linearly (1/2/4/7/11 VP for 1/2/3/4/5 of same type)
- **Building (yellow)** — buildings; each type has unique ongoing effect
- **Knowledge (beige)** — tech/tile that grants persistent bonuses (discount on placement, extra VP, etc.)

### 5.2 Filled-region bonus (round-scaled)

When a player places the last tile into a same-colored region, they earn a **filled-region bonus**:
- Bonus VP = 4–10 VP (region-size dependent)
- **Scaled by round**: in round 1, bonus doubled. In round 5, bonus is just printed value.
- Forces placement sequencing decisions: fill regions early for maximum bonus.

### 5.3 Goods / shipping

Goods barrels (4 colors) matched to ship values. Ship placed on an estate → collects goods of its color. Ship turn: dice must match good value → ships those goods; ships VP + silver + bonus livestock tile.

### 5.4 Knowledge tiles (tech)

~30 different knowledge tiles, each with unique ongoing effect:
- Dice modifiers (treat any die as N)
- VP multipliers (bonus VP per ship / mine / castle placed)
- Trade discounts
- Endgame bonus conditions

Each estate has 1 knowledge region (typically 3 hexes). Knowledge tiles are varied — draft choice matters.

### 5.5 Workers (dice mitigation)

Workers modify dice ±1 per worker spent. Acquired 2 at a time via a dice action. Primary tool to **convert unwanted rolls into wanted actions**. This is the calibrated-variance mechanism.

### 5.6 Silver (flexibility)

Silver purchases any tile from any slot regardless of die value. Scarce resource; main source = mine tiles producing end-of-round.

## 6. Scoring / end condition

### End condition

After 5 rounds × 5 turns per player (= 25 turns). All players complete the round; then score.

### Scoring components (6 distinct tracks)

1. **Placement VP** — each building placed = printed VP
2. **Filled-region bonus** — scaled by round (doubled in round 1, full in 5)
3. **Shipped goods VP** — 3 per 4-barrel ship
4. **Livestock VP** — non-linear by count (1/2/4/7/11 for 1–5 of same type)
5. **Knowledge/tech tile VP** — some knowledge tiles print endgame VP conditions
6. **Endgame completion bonus** — +8 VP for completely filled estate; +4 per fully-filled region; color/region set bonuses

Final VP = sum of 6 tracks.

### Canonical point-salad property

Each track has a different VP curve:
- Placement: linear (1 tile = fixed VP)
- Region bonus: round-scaled (early-round placements earn 2×)
- Livestock: **non-linear** (each additional animal of same type worth more than the last)
- Shipping: fixed 3 VP per ship
- Knowledge: conditional (VP only if endgame condition met)
- Completion: endgame step function

**You cannot directly compare 1 castle VP vs 1 ship VP vs 1 livestock VP.** The tracks are incommensurable. This is the C6 definitional property.

## 7. Catastrophes (B2 relevant — base game has none)

Base Castles of Burgundy has **no plague phase, no events deck, no catastrophe mechanic**. Each round is the same 5-turn cycle. Time pressure comes only from round count and scaling bonus — not from reversal events.

Some expansions / variants introduce plague tiles; base game does not. B2 diagnostic-low for this review.

## 8. Interaction (A3 relevant)

Players share a single central depot of tiles. Contention is indirect:
- Tile you want in slot 3 — if opponent takes it, you can't place that tile this turn.
- Goods of a color — first-come-first-served each round.
- Turn-order ship advantage (play order manipulated by ship tiles).

No direct aggression, no combat, no blocking, no stealing, no trading. Each player develops their own estate; opponents' choices affect tile availability but not board state. **This is the origin of the "multiplayer solitaire" critique** that recurs in BGG community commentary.

## 9. Key mechanics summary

| Mechanic | Role | Relevant axes |
|---|---|---|
| 6 distinct VP tracks (placement/region/ship/livestock/knowledge/endgame) | Scoring structure | **C6 Point-Salad Incommensurability (CANONICAL)** |
| Dice roll each turn + workers to modify | Action menu | A2 Decision Density, A4 Variance Calibration |
| Variable player estate boards | Replayability | A7 Emergence/Replayability |
| Filled-region bonus × round multiplier | Engine timing | C4 Engine-Garden, B6 Scoring Multiplier |
| Knowledge tiles (30+ unique) | Path diversity | A7 Emergence, B5 Architectural Novelty |
| Goods market + ship chain | Conversion | B3 Conversion Chain Depth |
| No plague / no events | Catastrophe | B2 (LOW — retire-explicit expected) |
| Indirect tile contention only | Interaction | A3 (LOW — multiplayer-solitaire critique), D2 (LOW) |
| All info public | Transparency | B4 (LOW) |
| 25 turns for ~35 tile-sized estate | Tension | C1 Tension Budget |
| 2p/3p/4p all play | Count | D3 Count-Robustness |

## 10. Weight & audience

- Weight ~3.1/5 (mid-weight euro)
- Teach: 20–30 min
- Play: ≈90 min at 4p; ≈60 min at 3p; ≈45 min at 2p
- Audience: Euro-tradition players, dice-driven point-salad fans, strategy-puzzlers

## 11. Citations

- *The Castles of Burgundy* rulebook, alea/Ravensburger 2011 (Alea big-box #16).
- BGG entry for Castles of Burgundy (game ID 84876): https://boardgamegeek.com/boardgame/84876
- BGG community strategy threads and the recurrent "multiplayer solitaire" critique.
- Stefan Feld designer interview material (BGG, Spielbox).
- 20th Anniversary edition (2019) rules unchanged from 2011; review uses 2011 base rules.

Summary posture: critical commentary under fair use. No rule text reproduced verbatim.

## 12. What this game is NOT

- Not catastrophe-driven (no plague phase in base).
- Not directly interactive (no combat / blocking / trading).
- Not hidden-info (all tiles visible in central depot).
- Not narrative-theme-first (15th-century Burgundy theme is iconographic only; tiles don't tell a story).
- Not single-track VP (6 distinct VP sources — the defining C6 property).
