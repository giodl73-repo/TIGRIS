---
name: FRACTURE — Design
slug: fracture-design
game: 0062-fracture
stage: design
version: 1.0.0
rubric_version: v2.23.33
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: chvatil
anchor_axis: B5
---

# FRACTURE — Design

## Overview

FRACTURE is a 3–5 player area-control game where each faction has a structurally incompatible scoring architecture. Players draft one faction at setup; all placement occurs on a shared hex grid; VP is earned each round using your faction's unique scoring grammar.

**Weight:** 3.0. **Length:** 75–90 min. **Players:** 3–5 (use 3-of-5 factions for 3p; 4-of-5 for 4p; all 5 for 5p).

---

## Components

- 1 shared hex-grid board (7×7 hexes; 49 territories)
- 5 faction boards (one per faction; defines scoring rule)
- 90 unit tokens (18 per faction; 5 colors)
- 40 placement cards (8 per faction; hand of 3 drawn each round)
- 1 round track (6 rounds)
- VP tokens

---

## Factions

Each faction has exactly **one scoring sentence** on its faction board. The scoring sentences are architecturally incompatible:

### Faction A — The Encirclers
**Scoring:** "At end of each round, score 2 VP per enemy territory completely enclosed by your units (orthogonally surrounded on all sides)."
*Grammar:* encirclement topology; opponent-relative.

### Faction B — The Chainers
**Scoring:** "At end of each round, score 1 VP per hex in your longest unbroken chain of connected controlled territories."
*Grammar:* linear connectivity; self-relative.

### Faction C — The Cascade
**Scoring:** "At any point in your turn, voluntarily remove up to 3 of your units from the board; for each removed unit, score 1 VP × (number of your remaining units adjacent to the removed hex)."
*Grammar:* sacrifice-amplification; density-relative at sacrifice moment.

### Faction D — The Density
**Scoring:** "At end of each round, score VP equal to your highest single-hex unit concentration (max units on any one territory you control)."
*Grammar:* local concentration; self-relative.

### Faction E — The Frontier
**Scoring:** "At end of each round, score 1 VP per of your territories that share a border with at least one enemy-controlled territory."
*Grammar:* border exposure; opponent-relative.

---

## Turn structure

Each of 6 rounds:
1. **Draw**: each player draws 2 placement cards from their personal faction deck.
2. **Place**: in turn order, each player places units from hand cards (card specifies unit count + hex restriction).
3. **Score**: each faction scores using its scoring rule simultaneously.
4. **Cascade check**: any Cascade player who spent units this round triggers cascade scoring mid-round (before end-of-round scores).

---

## Faction draft (setup)

With 5 factions and up to 5 players, draft proceeds clockwise. In 3p games, 2 factions are removed by common agreement before draft. This creates the 10-combination replayability that earns A7.

---

## Design notes (§10 — B5 monitoring)

B5 Architectural Novelty has accumulated 16 retire-explicit marks since last earn at Secret Hitler #46. FRACTURE is the targeted B5 earn. The architectural claim: the five scoring grammars (encirclement, chain, sacrifice-cascade, density, frontier) are not variable starting positions or asymmetric powers — they are genuinely different scoring theories of what area-control means. Remove Faction C's sacrifice-cascade mechanism and no other faction — in this game or in any prior area-control game in the corpus — fills that structural space. This is the B5 argument. The 35th TP collision (B5↔A7) must be well-adjudicated: B5 earns on the timeless structural novelty of the scoring grammar; A7 earns on the emergent strategic variety from faction combination.
