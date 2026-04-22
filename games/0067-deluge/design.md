---
name: DELUGE — Design
slug: deluge-design
game: 0067-deluge
stage: design
version: 1.0.0
rubric_version: v2.23.38
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: rosenberg
anchor_axis: C3
---

# DELUGE — Design

## Overview

DELUGE is a 2–4 player flood-management Euro. Players farm a shared river valley; floods advance each round along tributaries, permanently removing farmland. Players must harvest before their fields flood, then process and sell for VP.

**Weight:** 2.5. **Length:** 60–90 min. **Players:** 2–4.

---

## Components

- 1 shared river valley board (hex grid; 48 farmland tiles + 12 river tributary hexes)
- 80 flood markers (permanent; placed on farmland when flooded)
- 60 Crop tokens (4 types: Wheat, Barley, Root, Flax)
- 30 Harvest tiles (processed goods)
- 20 Store tokens (stored goods)
- 15 Sell cards (market demand)
- 1 Rainfall deck (18 cards; determines flood advance per round)
- VP tokens

---

## The flood system (C3 design)

At the start of each round, draw 1 Rainfall card. The card specifies:
- Which tributaries flood (1-3 tributaries activate)
- Flood advance distance (1-3 hexes along the tributary)

**Flood markers are permanent.** Once placed on a farmland tile, that tile is removed from the game. Any Crops on that tile are destroyed. Players cannot harvest flooded tiles.

By round 4 (of 7), 30-40% of the board will typically be flooded. By round 7, 60-70% is flooded. The pressure escalates continuously.

**Why C3 earns:** The scarcity is structural and irreversible. Neglecting a flood zone doesn't merely cost you resources this round — it permanently eliminates that region's future potential. This is harsher than Agricola's starvation (which resets each round); DELUGE's scarcity is cumulative and terminal.

---

## The production pipeline (B1/B3 design)

**Farm → Harvest → Store → Sell**

1. **Farm:** Plant Crop tokens on unflooded farmland tiles (1 action: plant 2 Crops)
2. **Harvest:** Collect planted Crops before floods arrive (1 action: harvest 1 tile)
3. **Store:** Process Crops into Harvest tiles at a Mill building (1 action: convert 2 Crops → 1 Harvest tile)
4. **Sell:** Convert Harvest tiles to VP via Sell cards (1 action: fulfill 1 Sell card for VP)

Four steps from planting to VP. Each step requires a separate building/action. This is depth-4 conversion: Farm (raw resource) → Harvest (collected) → Store (processed) → Sell (VP). B3 earns alongside B1.

---

## Storage upgrades (C4 design)

Players can build Mill upgrades that increase Harvest tile conversion rates:

- **Basic Mill:** 2 Crops → 1 Harvest tile
- **Improved Mill:** 2 Crops → 1.5 Harvest tiles (effective)
- **Master Mill:** 3 Crops → 3 Harvest tiles (1:1 ratio)

Master Mill requires Improved Mill (built in prior rounds). Building dependency creates mild C4 garden.

---

## Design notes (§10)

DELUGE targets two monitoring items:
- **C3** anchor: Rosenberg's own-anchor. C3 earns via permanent flood scarcity (farmland permanently removed).
- **B3** secondary: depth-4 Farm→Harvest→Store→Sell. B3 earns via depth confirmation (Le Havre #65 established this round's B3 canonical pattern; DELUGE confirms with a different mechanism).
