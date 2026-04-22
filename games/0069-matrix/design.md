---
name: MATRIX — Design
slug: matrix-design
game: 0069-matrix
stage: design
version: 1.0.0
rubric_version: v2.23.41
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: vaccarino
anchor_axis: A4
---

# MATRIX — Design

## Overview

MATRIX is a 2–4 player deck-and-die Euro. Players build personal 4×4 matrix grids of cards over 3 drafting rounds, then play 4 activation rounds where dice determine which rows and columns fire. Intersection bonuses (where an activated row meets an activated column) are the primary VP mechanism.

**Weight:** 2.5. **Length:** 60–75 min. **Players:** 2–4.

---

## Components

- 4 personal 4×4 Matrix boards (16 spaces per player)
- 80 Matrix cards (scored by row, column, or intersection bonus)
- 20 Die tokens (4 types: Row-Die, Column-Die, Both-Die, Null-Die)
- 4 Die pools (personal; starts with 4 basic dice; expands via drafting)
- Intersection bonus tiles (double-sided: single bonus / cascade)
- VP tokens

---

## Die types (A4 design)

**Row-Die (d6, faces 1-4 and two wilds):** activates one row (the face result = row number)
**Column-Die (d6, faces A-D and two wilds):** activates one column
**Both-Die (d8, faces 1-4 and A-D):** activates both a row and a column on a single roll
**Null-Die (d4, 50% chance of activating nothing):** a placeholder die that reduces the pool

**Die-pool construction (A4):** During 3 drafting rounds, players acquire dice to add to their pool. A player building Intersection bonuses in rows 3-4 and columns C-D should draft Row-Die (3-4 bias) and Column-Die (C-D bias). Building the right die pool IS variance calibration — you're shaping the probability distribution of which matrix positions activate.

---

## Turn structure

### Drafting Rounds (Rounds 1-3)

Each drafting round:
1. Draft 1 Matrix card from common pool → place in your 4×4 matrix (choose row + column)
2. Draft 1 Die token → add to your personal die pool

### Activation Rounds (Rounds 4-7)

Each activation round:
1. **Roll:** all players roll their personal die pools simultaneously
2. **Activate rows:** for each Row-Die result, the corresponding matrix row fires (all cards in that row earn their row-bonus VP)
3. **Activate columns:** for each Column-Die result, the corresponding column fires
4. **Intersections:** any matrix position at the intersection of an activated row AND an activated column scores its intersection bonus
5. **Cascade:** intersection positions with Cascade tiles trigger: adjacent positions earn +1 VP even if not activated

---

## The conversion chain (B3 design)

**Die → Row/Column activation → Intersection bonus → Cascade bonus**

A single die roll (step 1) activates a row (step 2), which intersects with an activated column to produce an intersection bonus (step 3), which may cascade to adjacent positions (step 4). This is depth-3 from a single roll event. The cascade is the TIGRIS-canonical B3 pattern applied to a die/grid mechanism.

---

## Matrix placement (A2 design)

Placing a card in the matrix requires calculating:
- **Row bonus:** what does this card earn when its row fires?
- **Column bonus:** what does this card earn when its column fires?
- **Intersection bonus:** given my current die pool, how often will this intersection activate?
- **Future die drafting:** can I draft dice that will activate this intersection reliably?

This is multi-axis per-placement decision density. The matrix is never trivially "full" — each placement forecloses and enables different intersection possibilities.
