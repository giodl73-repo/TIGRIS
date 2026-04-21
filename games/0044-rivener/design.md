---
name: RIVENER — Design
slug: rivener-design
game: 0044-rivener
stage: design
version: 1.0.0
rubric_version: v2.23.15
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# RIVENER — Design

## Overview

RIVENER is a 2–4p canal-routing game across 8 rounds. Players extend personal canal networks on a shared hex-grid map (48 hexes: towns, countryside, hills, locks). Canal segments once placed cannot move. Cargo tokens flow through completed networks each round, scoring VP for connected towns. The game ends after 8 rounds; final scoring rewards network length + cargo delivered + town-tier bonuses.

## Setup

- Hex grid: 12 named towns (VP values 1–5), 4 locks (allow elevation change), 8 hill hexes (impassable), 24 flat hexes.
- Each player starts with a Home Dock (fixed, random placement) and 15 canal segment tiles.
- 3 Cargo types (Grain, Coal, Textile) assigned to towns randomly.

## Core turn

1. **Place 1–2 canal segments** from hand (≤2 per turn; must connect to own network). Once placed: permanent.
2. **Route cargo** (if your network connects a production town to a consumption town this round): score cargo VP × route length.
3. **Draw** 2 canal tile refills from a limited common pool (scarcity mechanism).

## Canal network rules

- Segments are hexagonal; each connects adjacent hex sides.
- Locks allow uphill connection (1 elevation change per lock).
- Your network cannot cross another player's unless at a junction hex (shared locks).
- Junction hexes are contested: whoever builds the junction first claims access.

## Scoring

- **Cargo VP:** each cargo delivered = 1 VP × number of segments in routing chain.
- **Town connection bonus:** connecting a town = that town's tier value (1–5 VP one-time).
- **Network length:** 1 VP per segment in your network at game end.
- **Longest route bonus:** player with longest single connected chain: +8 VP.

## Lock-in design

The permanent placement rule is non-negotiable. Players cannot undo, sell, or remove segments. If P1 places on the only viable path between two towns, P2 must detour (if possible) or abandon that cargo route. By Round 4 of 8, the topology is effectively determined. Final rounds are cargo-optimization within fixed geometry.

## C3 Scarcity

Canal tile pool: 60 tiles for 4 players × 15 each = exactly consumed. No surplus. If the pool runs dry mid-game, players cannot build more (rare but dramatic). Tile draw from limited pool = scarcity engine.

## Player count scaling

- 2p: 20 hexes per player area; less competition. 60 minutes.
- 3p: Tighter; junction hexes contested earlier. 80 minutes.
- 4p: Highly contested; junction hex control is critical. 100 minutes.
