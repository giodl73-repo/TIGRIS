---
name: TORRENT — Design
slug: torrent-design
game: 0047-torrent
stage: design
version: 1.0.0
rubric_version: v2.23.18
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# TORRENT — Design

## Overview

TORRENT is a 2–4p water-routing game across 8 rounds on a hex grid. Players place Water tiles (sources, channels, dams, reservoirs) to route fresh water to their territories. Territories with water generate resources; resources buy VP and additional tiles. The hex grid uses elevation (printed on each hex): water flows to adjacent lower hexes.

## Setup

- 37-hex grid (roughly hexagonal shape). Elevation values: 1–5 (mountain hexes at center = 5; valley hexes at edges = 1).
- 8 Mountain hexes (elevation 4–5): potential source placements.
- 24 Valley hexes (elevation 1–2): territory hexes where VP scoring occurs.
- Each player controls 6 territory hexes (pre-assigned by color, adjacent clusters).

## Tile types

- **Source tile:** placed on mountain hex; generates 1 Water unit per round flowing downhill.
- **Channel tile:** placed on any hex; routes Water in one specific direction.
- **Dam tile:** placed on any hex; blocks downstream flow; creates a reservoir (+3 Water storage but blocks 100% of downstream).
- **Reservoir:** channel-adjacent; stores Water for emergency use.
- **Aqueduct:** bridges gaps over high hexes; expensive (3 VP cost).

## Water flow mechanic

Each round, Water flows from each active Source tile through any connected Channel tiles, following elevation gradients. Water stops at: a Dam, a territory hex (delivers resources), a hex with no channel. Players can share water paths (both benefit) or block each other (Dam cuts off all downstream).

## Resources and scoring

- **Territory with water:** generate 1 resource per water unit reaching it.
- **Resources:** spend 3 = buy 1 tile; spend 1 = 1 VP at game end.
- **Water delivery VP:** 1 VP per territory receiving water per round.
- **Final scoring:** territory VP bonuses (based on water delivered total across all rounds).

## C3 Scarcity note

Source hexes: 8 available. Players need Sources to generate water. 4 players × typically 2 Sources each = all 8 Sources claimed. Late-source-claiming players face significant disadvantage.

## Player count

- 2p: 18-hex sub-grid; 12 territory hexes split 6/6.
- 3p: 28-hex grid; 18 territory hexes split 6/6/6.
- 4p: 37-hex full grid; 24 territory hexes split 6/6/6/6.
