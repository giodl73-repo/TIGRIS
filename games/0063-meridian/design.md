---
name: MERIDIAN — Design
slug: meridian-design
game: 0063-meridian
stage: design
version: 1.0.0
rubric_version: v2.23.34
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: kramer-kiesling
anchor_axis: D2
---

# MERIDIAN — Design

## Overview

MERIDIAN is a 2–4 player road-network Euro on a shared hex grid. Players build permanent road segments; connected towns score VP at round end. Blocking is direct and permanent. The game is architecturally minimal: two player actions, complete strategic depth.

**Weight:** 2.0. **Length:** 45–60 min. **Players:** 2–4.

---

## Components

- 1 shared hex-grid board (hex grid with 37 hexes; 12 town hexes marked)
- 72 road segment tokens (18 per player; 4 colors; placed on hex edges = 6 edges per hex)
- 1 round track (5 rounds)
- VP tokens

---

## Two actions (complete ruleset)

**Action 1: Place Road Segment.** Claim one hex edge on the grid and place one of your road tokens on it. A claimed edge cannot be claimed by another player. You may place only on edges adjacent to your existing network OR on any unclaimed edge (to start a new branch).

**Action 2: Score Adjacency.** At the end of each round, each player scores 1 VP per pair of towns connected by an unbroken path of their road segments. A path is unbroken if every hex edge in the path carries that player's road token.

**Why this earns A1:** The entire strategic space — blocking, detour calculation, route optimization, network topology management — emerges from these two sentences. No exceptions, no special cases.

---

## Spatial interaction mechanics (D2 design)

The hex grid creates the following direct spatial coupling mechanisms:

1. **Direct blocking:** claiming a hex edge denies that edge to all others permanently.
2. **Network interference:** opponent networks can force detours (extending path length, adding required segments).
3. **Town competition:** two players may route toward the same town; whoever reaches it first creates the connection.
4. **Adjacency compression:** at higher player counts, the hex grid becomes contested faster — D2 scales with density.

---

## Scoring example

End of round 3: P1 has a connected network linking Town A — Town B — Town D (2 connected pairs: A-B, A-D, B-D = 3 pairs). P1 scores 3 VP. P2 blocked P1's direct A-C route, forcing P1 to detour through D. The block costs P1 the A-C connection (2 VP lost, as P1 couldn't complete by round end).

---

## Design notes (§10 — D2 monitoring)

D2 Spatial-Interaction Presence last earned at VIADUCT #53 (9 games before FRACTURE #62; 10 games before MERIDIAN #63). Retire-explicit accumulation at non-spatial games: Ra, Ledger, Scroll, Brass-Lancs, Hanabi, Chorus, Viticulture, Floor = 8 retire-explicits. MERIDIAN is the targeted D2 earn. The spatial claim: hex-grid permanent road placement is canonical D2 — direct blocking, adjacency coupling, permanent topology. The A1 co-claim: two rules produce a complete strategic game. D2↔A1 OP is the designed collision: spatial coupling (D2 = the architecture) vs. rule elegance (A1 = the design economy).
