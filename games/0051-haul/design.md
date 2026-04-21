---
name: HAUL — Design (Rules)
slug: haul-design
game: 0051-haul
stage: design
version: 1.0.0
rubric_version: v2.23.22
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: rosenberg
anchor_axis: B3
---

# HAUL — Rules

## 1. Overview & Object of the Game

HAUL is a 2–4 player harvest-and-transport Euro played over four seasons. Players are hauliers: they plant and harvest crops, process them at shared mills, store and upgrade goods in personal warehouses, and sell at seasonal markets before they close.

The game's core mechanism is a four-stage conversion pipeline: **Farm → Mill → Warehouse → Market**. Raw Grain is worth 1 VP at market; processed Flour fetches 2 VP; warehouse-upgraded Bread commands 4 VP. Running the full pipeline in a single season earns a bonus. The competition for the shared Mill creates a bottleneck — only one player can process per round — and market stalls close when full, so timing matters as much as chain depth.

The **warehouse is the engine**. Players invest in upgrade tiles (Preservation, Spice, Cooling) that create adjacency bonuses: Flour stored next to a Spice tile converts to Bread automatically; Preservation tiles prevent spoilage. A fully-upgraded warehouse turns the conversion chain semi-automatic. A bare warehouse is a liability: goods spoil if unsold by season's end.

**Winning condition:** highest VP after four seasons. Ties broken by fewest Spoilage tokens; further ties by turn-order proximity to the first player.

---

## 2. Components

- **1 Countryside board** (shared hex grid; 20 Farmland hexes rated 1–3 fertility; 4 Mill sites; 4 Market zones with 4 stalls each; 8 Warehouse locations; 6 River crossing hexes)
- **4 Personal Warehouse boards** (4×2 grid = 8 spaces; upgradeable via Upgrade tiles)
- **Goods tokens:**
  - Grain ×40 (yellow; raw crop)
  - Flour ×24 (white; processed)
  - Bread ×16 (brown; premium processed)
  - Spice ×12 (green; warehouse ingredient)
- **Warehouse Upgrade tiles** (×6 per type × 3 types = 18 total):
  - Preservation (blue) — stored goods in this Warehouse space don't spoil
  - Spice-Rack (orange) — Flour stored adjacent to Spice converts to Bread
  - Cooling (gray) — extends spoilage immunity one additional season
- **Cart tokens** — 4 per player (used to mark active transport routes)
- **Spoilage tokens** ×30 (black)
- **VP tokens** — 1/5/10/20 denominations
- **Season board** (4 season tracks; 4-round markers per track)
- **First-player marker**

---

## 3. Setup

1. Place the Countryside board. Seed Farmland hexes with Grain tokens equal to their fertility rating (1, 2, or 3).
2. Each player takes a Warehouse board and 4 Carts.
3. Each player starts with 2 Grain tokens (in their Warehouse).
4. Shuffle Upgrade tiles by type; place face-up stacks adjacent to the board.
5. Season board: set Season 1, Round 1.
6. Determine first player randomly; hand them the First-player marker.

---

## 4. Turn Structure

Each of 4 **Seasons** contains 4 **Action Rounds**. The Season board advances after each round.

**Season flow:**
1. **Round start**: Refill Farmland hexes that were fully harvested (place Grain tokens equal to fertility rating).
2. **Player turns**: starting with first player, each player takes **one action**. Continue clockwise until all players have taken their action.
3. **Round end**: check Spoilage (see §8). First-player marker passes left.
4. **Season end** (after Round 4): close all Market stalls; apply end-of-season penalties; unsold Warehouse goods take 1 Spoilage token each (unless Preserved).

---

## 5. Actions

**On your turn, take exactly one action:**

| Action | Cost | Effect |
|---|---|---|
| **Farm** | 1 Cart (placed on Farmland hex) | Take all Grain tokens from the target hex. Remove your Cart at round end. A hex with a Cart already cannot be farmed by another player this round. |
| **Mill** | 1 Cart (placed on Mill site) + 2 Grain from Warehouse | Convert 2 Grain → 1 Flour (place in Warehouse). **Mill is shared: only one player per Mill site per round.** First Cart placed wins access; others must wait. |
| **Warehouse** | Free | Transfer goods from your Cart(s) to your Warehouse (up to Warehouse capacity). Apply any adjacency bonuses triggered (see §8). |
| **Sell** | 1 Cart (placed on Market stall) | Move 1 good from your Warehouse to an open Market stall; collect VP immediately. Stall holds 4 goods max; a full stall is closed permanently. |
| **Upgrade** | Cost (see below) | Pay goods/VP to place one Upgrade tile on your Warehouse board. Costs: Preservation = 2 Grain; Spice-Rack = 2 Flour; Cooling = 1 Bread + 1 VP. |
| **Pass** | Free | End your turn. You may not act again this round. |

**Cart recovery:** all Carts return to your hand at the end of each Season (not each Round).

---

## 6. Scoring & End Condition

**Game end:** after Season 4, Round 4 is complete.

**Scoring:**

| Good sold at Market | VP |
|---|---|
| Grain | 1 VP |
| Flour | 2 VP |
| Bread | 4 VP |

**Bonuses (scored at game end):**
- **Full Chain Bonus**: each Season in which you sold at least 1 Bread (completing all 4 pipeline stages in that season): +3 VP.
- **Warehouse Garden Bonus**: for each Warehouse board with all 3 Upgrade tile types present: +5 VP.
- **First Seller Bonus**: first player to sell a Bread in each Season: +1 VP (tracked on Season board).

**Penalties:**
- **Spoilage**: −1 VP per Spoilage token you hold at game end.

**Remaining goods in Warehouse at game end score 0 VP** (they were not sold before the final Market closed).

---

## 7. Edge Cases & Clarifications

- **Mill capacity**: a Mill site has 1 Cart slot. A second player arriving at the same Mill in the same round is blocked — they must use a different Mill or pass.
- **Warehouse overflow**: Warehouse capacity = 8 spaces (4×2 grid). If your Warehouse is full and you Farm or Mill goods, you must discard before warehousing. Discarded goods score 0 VP.
- **Spice-Rack adjacency**: a Flour token stored in a Warehouse space orthogonally adjacent to a Spice-Rack tile converts to Bread at the end of the Warehouse action (not immediately — the conversion fires when you finish your Warehouse action). Spice-Rack does not convert Grain; only Flour → Bread.
- **Preservation coverage**: a Preservation tile covers all goods in the same Warehouse space it occupies (1 tile per space; 1 space = 1 good max). Each Preservation tile is single-space.
- **Market stall closure**: a stall closes when it contains 4 goods OR at Season end (whichever comes first). Closed stalls cannot receive new goods. If all stalls in a Market zone close, that zone is exhausted for the game (not just the season).
- **Ties in Mill access**: if two players place a Cart on the same Mill site on the same round (both declare Mill action simultaneously), the player earlier in turn order wins access; the other player's action is wasted (they may not redirect to a different action).

---

## 8. Special Subsystem — Spoilage

At the **end of each Round** (after all players have acted), check each player's Warehouse:

- For each good that is **not** covered by a Preservation tile: place 1 Spoilage token on that good.
- A good with **2 Spoilage tokens** spoils: discard the good; keep both Spoilage tokens (they become end-game penalties).
- **Cooling tile effect**: goods adjacent to a Cooling tile do not receive Spoilage tokens during the season the Cooling tile was placed AND the following season. (Cooling provides 2-season immunity starting from placement.)
- **Spoilage is the catastrophe mechanism**: a Warehouse full of unsold goods decays. Players must sell or Preserve proactively.

---

## 9. Variants

**2-player variant**: remove 2 Farmland hexes (the two lowest-fertility hexes). Reduces board density; increases per-hex competition for the remaining Farmland.

**Expert variant**: add "Market Demand cards" — each season, 1 card is revealed that doubles the VP value of one good type. Creates volatile demand swings that reward flexible conversion chains over committed pipelines.

---

## 10. Designer Notes — B3 and C4 Documentation

*Required per HOP-017 notes: B3 and C4 must be explicitly documented.*

**B3 Conversion Chain Depth (anchor; targeted earn):** The Farm→Mill→Warehouse→Market pipeline is a genuine depth-4 resource-conversion chain. Each stage has a distinct cost (Cart, Grain, Warehouse space, Market stall), a distinct output type, and a distinct scarcity constraint (Farmland fertility, Mill access, Warehouse capacity, Market stall availability). Converting from raw Grain to sold Bread requires committing to all 4 stages over multiple rounds. B3's earning condition: the chain is mechanically load-bearing — removing any stage would collapse the game's economic logic.

**C4 Engine-Garden Dependency (targeted earn — monitoring window):** The warehouse upgrade system is the game's engine-garden. Players place Preservation, Spice-Rack, and Cooling tiles onto their Warehouse board across 4 seasons. A warehouse with all 3 tile types produces Bread semi-automatically (Spice converts Flour → Bread; Preservation prevents spoilage; Cooling extends immunity). The dependency: removing a single upgrade tile breaks the engine. This is the "garden" — once you've planted the Spice-Rack in column 2 and placed Flour in the adjacent space, you depend on that adjacency relationship for the rest of the game. C4's earning condition: the warehouse architecture creates an architectural dependency that must survive across all 4 seasons. If it does, C4 earns canonically.

---

## 11. Collision Adjacency Chart

| Axes | Adjacency type | Reasoning |
|---|---|---|
| **B3 ↔ C3** | OP candidate (primary) | B3 = structural depth of the pipeline (how many stages); C3 = scarcity pressure at each stage (Mill bottleneck, Warehouse limit, Market close). Same supply chain; different analytical registers. |
| **B3 ↔ C4** | co-firing (designed) | The conversion chain (B3) and the warehouse garden (C4) are coupled: a deeper chain run is enabled by a better-upgraded warehouse. B3 is the pipeline; C4 is the engine that powers later pipeline stages. |
| **B1 ↔ B3** | co-firing (existing) | System gearing (B1) and conversion chain depth (B3) both describe the same Farm→Mill→Warehouse→Market cascade — B1 from the production-system perspective; B3 from the pipeline-depth perspective. |
| **D4 ↔ C3** | co-firing (existing) | Warehouse placements lock in (D4); the scarcity at each pipeline node (C3) forces earlier commitment, amplifying the lock-in. |

---

## 12. Open at DESIGN-stage

1. **Mill bottleneck intensity**: one Mill slot per round per site is very tight at 4 players — does this create too much AP waste? Alternative: Mill has 2 slots but second slot costs 1 extra Grain. Needs playtesting.
2. **Warehouse upgrade cost calibration**: Spice-Rack at 2 Flour (conversion cost) may be too steep in early seasons when Flour is scarce. Consider 1 Flour + 1 Grain instead.
3. **Full Chain Bonus trigger**: currently requires Bread sold in a season. Should it instead require that the *same player* completed all 4 stages in sequence (Farm→Mill→Warehouse→Sell), or merely that they sold Bread? The sequential requirement earns more B3 evidence but is harder to track.
4. **Spoilage rate**: 1 Spoilage token per round per unpreserved good is fast — a good spoils in 2 rounds. With 4 rounds per season, an unprepared player can lose goods by round 2. Is this the right catastrophe intensity for B2?
5. **C4 adjacency rules**: Spice-Rack fires on orthogonal adjacency only (not diagonal). Should diagonal also trigger? Diagonal adjacency would make the garden significantly easier to build — B3↔C4 coupling would weaken.
