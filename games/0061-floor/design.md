---
name: FLOOR — Design
slug: floor-design
game: 0061-floor
stage: design
version: 1.0.0
rubric_version: v2.23.32
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: knizia
anchor_axis: C2
---

# FLOOR — Design

## Overview

FLOOR is a 2–4 player Euro where every player begins with a non-losable 10 VP Foundation. All play adds to this floor. Game ends after 4 market rounds; highest total VP wins.

**Weight:** 2.5. **Length:** 60–75 min. **Players:** 2–4.

---

## Components

- 40 Foundation tokens (10 per player; gold-colored; permanent)
- 60 Resource tiles (3 types: Stone, Timber, Grain; 20 each)
- 40 Product tiles (4 types: Tools, Cloth, Bread, Luxury; 10 each)
- 20 Market stall cards (5 per round)
- VP track

---

## Foundation mechanic (C2 design)

At setup, each player takes 10 Foundation tokens and places them on their player board. These tokens:
- **Cannot be lost** (no negative effects can remove Foundation tokens)
- **Count as VP** at game end (each Foundation token = 1 VP; permanently held)
- **Can be spent** to activate Market Multiplier: spending 1 Foundation token = double VP from next market sale (token is permanently removed when spent; this is the only way to lose Foundation VP)

**Why this earns C2:** The floor is 10 VP (minimum guaranteed); spending Foundation tokens is always a trade of guaranteed VP for a multiplied future gain. A player who spends no Foundation tokens ends with exactly 10 + earned VP. A player who spends all Foundation tokens risks finishing with only earned VP (potentially less than 10 if market rounds go poorly). The designed floor creates a meaningful minimum that players can choose to risk.

---

## Turn structure (per market round)

Each of 4 market rounds:

1. **Reveal:** Flip 5 Market stall cards (each shows a Product requirement and VP value).
2. **Produce (2 actions each):** Each player takes 2 actions from: Gather Resource (take 2 tiles of 1 type), Process (convert 3 Resource tiles → 1 Product tile), or Inspect (look at a hidden market stall).
3. **Sell:** Each player may sell up to 2 Products to available market stalls (matching type). Earn VP printed on stall. If Foundation Multiplier active (from spending Foundation token): double VP for this sale.
4. **Replenish:** Discard unsold stall cards; deal fresh stalls for next round.

---

## B1 pipeline

Foundation token (start) → Gather Resource → Process → Sell to Market → VP. 4 stages where each depends on prior (can't sell without product; can't product without resource; can't multiply without foundation token spend decision).

## B6 mechanism

Foundation token spend → Sale VP ×2. The Foundation multiplier IS the B6 mechanism: Foundation tokens are themselves VP (base scoring) AND the activation cost for the multiplier (multiplicative bonus on sale VP). Stacking Foundation spends across multiple market stalls creates multiplicative endgame scoring.
