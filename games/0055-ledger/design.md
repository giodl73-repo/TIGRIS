---
name: LEDGER — Design
slug: ledger-design
game: 0055-ledger
stage: design
version: 1.0.0
rubric_version: v2.23.26
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: knizia
anchor_axis: C1
---

# LEDGER — Design

## Overview

LEDGER is a 2–5 player Vickrey sealed-bid auction game. Each of 4 market rounds, players simultaneously commit sealed bids on a central commodity lot. The highest bidder wins the lot but pays only the second-highest price. Bid tokens are limited and recovered after each round; the total VP from won lots determines the winner.

**Weight:** 2.0. **Length:** 45–60 min. **Players:** 2–5.

---

## Components

- 40 Commodity tiles (4 suits × 10 tiles; values 1–10 per suit)
- 5 sets of Bid tokens (10 tokens per player; values 1–10)
- 4 Market Round cards
- VP track

---

## Turn structure (per market round)

1. **Deal lot:** Reveal 3 Commodity tiles face-up as the current lot.
2. **Sealed bid:** Each player secretly selects one Bid token from their hand and places it face-down.
3. **Reveal:** All bids revealed simultaneously.
4. **Resolve:** Highest bidder wins the lot and pays the second-highest bid (return that many tokens to their supply). Ties: re-bid (all tied players bid again).
5. **Score:** Winner takes the 3 Commodity tiles; VP is scored at game end per tile set completion.
6. **Recover bids:** All bid tokens (winner's payment excepted) return to their owners' hands.

## Scoring (end of game)

- **Commodity set bonus:** Complete set of any suit (all 10 tiles) = 20 VP bonus.
- **Partial sets:** VP = sum of tile values held.
- **Bid tokens remaining:** Each unspent Bid token (total payments made = tokens permanently lost) = 0 VP. (You pay cumulatively — payments across rounds are permanent.)

---

## C1 mechanism

The Vickrey mechanism creates structural tension: bid too high (you win but waste the difference) vs. bid too low (you lose the lot). With only 10 bid tokens total and 4 rounds of payments being permanent, the budget compresses as the game progresses. By Round 4, players have spent 1–3 tokens permanently; their remaining bid range is constrained. The perpetual uncertainty about the threshold IS the game.
