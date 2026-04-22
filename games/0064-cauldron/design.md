---
name: CAULDRON — Design
slug: cauldron-design
game: 0064-cauldron
stage: design
version: 1.0.0
rubric_version: v2.23.35
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: feld
anchor_axis: C6
---

# CAULDRON — Design

## Overview

CAULDRON is a 3–4 player rondel Euro with three orthogonal VP scoring tracks. The rondel determines which track actions are accessible each round. Certain rondel positions trigger Disaster events that damage scoring infrastructure. Players race to optimize along incompatible scoring tracks while managing Disaster risk.

**Weight:** 3.5. **Length:** 90–120 min. **Players:** 3–4.

---

## Components

- 1 rondel board (12 positions; 3 color-coded: Guild=blue, Monument=grey, Harvest=green; 3 Disaster=red)
- 3 scoring track boards (Guilds, Monuments, Harvests; separate VP tracks)
- 60 Worker meeples (15 per player)
- 40 Stone tiles (10 per player)
- 30 Crop tokens (rotating seasonal supply)
- 18 Monument tiles (3 tiers × 6 tiles; scoring multiplier by tier)
- 1 Disaster deck (12 cards: 4 Flood, 4 Fire, 4 Collapse)
- VP tokens

---

## Three scoring tracks (C6 design)

Each track requires completely different resources and has a different scoring grammar:

### Guilds Track
Workers placed in Guild workshops earn Guild VP each round they remain. Removing Workers from workshops ends the Guild VP stream. Guild VP is persistent-while-invested.

### Monuments Track
Stone tiles are placed on Monument sites in sequences. Completing a Monument earns VP = Stone tiles invested × Monument tier. Monument VP is batch-scored on completion. Monument tier 1 = ×1; tier 2 = ×2; tier 3 = ×3 (B6 multiplier).

### Harvests Track
Seasonal crop sequences must be completed within the Harvest season window (3 consecutive rondel passes through the Harvest sector). Crops not completed by season end score 0. Harvest VP requires timing precision.

**Why C6 earns:** Guild VP accumulates steadily; Monument VP front-loads investment for large batches; Harvest VP requires temporal precision. A player investing in Stone for Monuments cannot use that Stone for Guild workshops. The tracks require incompatible investment strategies that cannot be hedged without losing all three.

---

## Rondel structure (12 positions)

- Positions 1-3: Guild sector (3 actions: Place Workers, Advance Guild, Collect Guild VP)
- Positions 4-6: Monument sector (3 actions: Quarry Stone, Build Monument, Complete Monument)
- Positions 7-9: Harvest sector (3 actions: Plant Crops, Tend Crops, Harvest)
- Positions 10-12: Disaster sector (**3 positions trigger Disaster events**)
  - Position 10: FLOOD — destroys all unprotected Crop tokens on player's Harvest board
  - Position 11: FIRE — destroys 2 Workers from player's Guild workshops
  - Position 12: COLLAPSE — destroys 1 Monument tier of player's highest-tier Monument (demotes by 1 tier)

**B2 design:** Players must navigate through the 12-position rondel sequentially. The Disaster sector cannot be skipped cheaply — jumping over it costs 3+ action penalty. Players approaching their scoring tracks from the "near" side must pass through the Disaster sector first.

---

## Turn structure

Each round:
1. Advance rondel token (must advance 1-3 positions; paying 1 resource per extra position)
2. Take the action of your current position
3. Resolve Disaster if on position 10-12 (draw Disaster card of appropriate type; apply effect)

---

## Design notes (§10 — B2 monitoring)

B2 Catastrophe Pressure last earned at CRUCIBLE #52 (12 games before CAULDRON #64). Retire-explicit accumulation has elevated B2's refute weight to 6.5R. CAULDRON is the targeted B2 earn. The catastrophe claim: the Disaster sector on the rondel creates genuine catastrophe events — not mild setbacks but track-infrastructure damage that can eliminate rounds of investment. The Flood that destroys all Harvest crops is not merely a penalty; it erases a seasonal window that cannot be recovered. This is B2: frequency and impact of reversal events within the arc.

The C6↔B2 OP distinction (2nd instance): C6 = which scoring path to pursue (investment architecture, temporal strategy); B2 = what happens when rondel navigation fails (Disaster consequence, reversal pressure). Different register: C6 = the strategic space; B2 = the danger within that space.
