---
name: FACETS — Design (Rules)
slug: facets-design
game: 0003-facets
stage: design
version: 1.0.0
rubric_version: v2.2
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
anchor_persona: feld
anchor_axis: point-salad-incommensurability
---

# FACETS — Rules

## 1. Overview & Object of the Game

You are a gem-trader stocking a display of six jewel types, each valued by a different merchant. Over ~15 turns, you draft jewels from a shared market, building a collection that scores under six simultaneously-applied functions. Each type has its own math — sum linearly, cap at a ceiling, reward pairs, reward triples, act as wild, or punish duplicates. Winning requires balance across incommensurable value curves.

**Object:** collect jewels to maximize total score across six scoring functions. Highest total at game end wins.

**Core tension:** chasing one type alone yields capped score (~15 pts). Only multi-type commitment wins. But every jewel you take is one you must *score under its own rule* — there is no conversion between types except via Diamond (the wild).

## 2. Components

| Item | Count | Notes |
|---|---:|---|
| Ruby tokens | 10 | Red; linear scoring |
| Emerald tokens | 10 | Green; capped scoring |
| Sapphire tokens | 10 | Blue; pairs scoring |
| Topaz tokens | 10 | Yellow; triples scoring |
| Diamond tokens | 10 | Clear; wild, assigned at endgame |
| Pearl tokens | 10 | White; singleton scoring |
| Jewel bag | 1 | Opaque drawstring; holds all 60 tokens |
| Market strip | 1 | Cardboard or cloth with 5 face-up slots |
| Player trays | 1 per player | Segmented into 6 compartments (one per type) |
| Scoring pad | 1 | For endgame scoring |
| Rulebook | 1 | This document |

**60 jewel tokens total. 10 of each type — distribution is fixed and known to all players.**

Prototype substitution: glass beads, poker chips, or colored cubes all work. The game is medium-agnostic.

## 3. Setup

1. Place all 60 jewel tokens in the bag.
2. Draw 5 tokens blindly and place face-up in the 5 market slots.
3. Give each player an empty tray.
4. Determine first player randomly. Play proceeds clockwise.

Total setup time: <1 minute.

## 4. Turn structure

On your turn, choose ONE action from the menu in §5. Then the turn passes clockwise.

## 5. Actions

### Action A — TAKE

Select any one jewel from the market and move it to your tray's matching compartment. Then draw one token blindly from the bag and place it in the now-empty market slot. If the bag is empty, the slot stays empty for the rest of the game.

A single action. No cost. No limit on how many of a type you can accumulate.

### Action B — PASS

Take no jewel this turn. Draw 2 tokens from the bag and add them to empty market slots (if any). If the market is full, the 2 tokens go to the side and the first empty slot refills from them.

The PASS action accelerates endgame for everyone. It is rarely optimal for the passing player individually, but useful when no market jewel suits your collection strategy.

**Exactly 2 action types.** No combinations. No free sub-actions.

## 6. Scoring / end condition

### End condition

The game ends at the end of the round in which:
- The bag is empty, AND
- The market has fewer than 5 tokens remaining.

Complete the current round (each player takes one more turn), then score.

### Scoring functions (applied simultaneously)

For each player, apply each of the six functions to the tokens in the matching tray compartment:

| Type | Function | Max single-type score |
|---|---|---:|
| **Ruby** | +1 per Ruby | 10 |
| **Emerald** | min(Emerald count, 5) × 2 | 10 |
| **Sapphire** | floor(Sapphire count / 2) × 3 | 15 (5 pairs) |
| **Topaz** | floor(Topaz count / 3) × 5 | 15 (3 triples) |
| **Diamond** | Wild — see Diamond rule below | 15 (best assignment) |
| **Pearl** | If Pearl count == 1: +3. Else: +3 − 2 × (count − 1) (can go negative) | +3 (exactly 1) |

### Diamond rule

At scoring, declare how you assign each Diamond token. Each Diamond counts as one token of any other type — Ruby, Emerald, Sapphire, Topaz, or Pearl. Diamonds may be split across types (2 as Ruby, 3 as Topaz, 5 as Emerald, etc.). Once declared, Diamonds are scored under the target type's function **as if they were native tokens of that type**.

**Worked example:** You have 3 Topazes and 4 Diamonds. Declare all 4 Diamonds as Topaz. Effective Topaz count = 7. Score = floor(7/3) × 5 = 2 triples × 5 = 10 pts. Declare as Emerald instead: min(4+0, 5) × 2 = 8 pts for Emerald plus original Topaz (floor(3/3) × 5 = 5) = 13 total. Depends on your other jewels.

### Total score

Sum the six function outputs. Highest total wins.

### Tiebreak

1. Most types with a positive score contribution.
2. Most total tokens.
3. Shared victory.

## 7. Edge cases & clarifications

- **Bag empties mid-turn.** If the bag empties when you're about to refill after a TAKE action, the market slot stays empty. You continue play; the end condition triggers when the market falls below 5.
- **Every market slot is full on your PASS.** Excess tokens from the PASS go to a "pending" side-stack. When a slot empties (via someone's TAKE), refill from pending first, then from bag.
- **Tied first-slot choice.** If two players want to take the same jewel but are separated by other players' turns, each must wait for their turn. No simultaneous action.
- **Zero tokens at the end.** A player with 0 tokens scores 0 from every function (Pearl rule: 0 pearls = 0, not −2).
- **Single Pearl.** +3 exactly. Two Pearls: +3 − 2×1 = +1. Three Pearls: +3 − 4 = −1. Four: −3. Pearl is actively punished.
- **All tokens drawn but market still at 5.** Edge case: the bag has exactly enough tokens to fill the market plus your TAKE. The game continues for one more round after the bag empties.

## 8. Special subsystems

None. FACETS is intentionally single-system — one market, one action menu, one scoring pass. This is why it teaches in 8 minutes.

## 9. Variants

### 2-player variant
- Setup: draw 4 tokens for market (smaller market).
- End condition: bag empty AND market < 3.
- Scoring: same functions.
- Expected play time: 12–15 min.

### Tournament variant
- Setup: 7-slot market (more visibility).
- End condition: stricter — bag empty AND market < 5.
- Scoring: Pearl penalty doubled (−4 per Pearl beyond 1).
- Produces more aggressive market-blocking.

### Draft variant (untested)
- Draft 3 jewels from initial market before play begins (snake draft 1p → Np → 1p).
- Each player begins with 3 tokens; market refills from bag.
- Faster onset of scoring specialization.

## 10. Designer notes

**The scoring multiplier isn't here — and that's the design.** Parliament uses `earned × adopted` (Knizia multiplier). T&E uses MIN across colors (Knizia minimum-score). FACETS uses **functional incommensurability** as the shape instead. Six functions cannot be compared; you play all of them in parallel.

This is the design's testable claim: **you can force breadth without a multiplier or minimum-score**, by giving each track its own math. If the claim holds, the panel's earning on C6 Point-Salad Incommensurability validates it. If the claim fails, FACETS joins the counter-example pile alongside T&E.

The Pearl mechanism (negative scoring for duplicates) is the design's second novelty attempt. Most games reward accumulation; Pearl punishes it. Combined with the Ruby (pure linear) and Sapphire (pairs only), the scoring space rewards *precise* holding patterns over greedy accumulation.

## 11. Collision Adjacency Chart (FACETS-specific)

New adjacency candidates surfaced by this game, for possible adoption into the global chart:

| Left | Right | Theme |
|---|---|---|
| C6 Point-Salad Incommensurability | A2 Decision Density | Both measure per-turn choice richness |
| C6 Point-Salad Incommensurability | A6 Teachability | Incommensurable scoring *requires* clear teach |
| A2 Decision Density | A6 Teachability | Many meaningful choices, plus simple menu |
| A4 Variance Calibration | A6 Teachability | Random bag + simple rules |

These are C6-centered candidates. If FACETS earns C6 and triggers one or more of these collisions in play, the global adjacency chart should amend to include these pairs (I-facets-01).

Existing global adjacencies that apply to FACETS's likely drafts:
- C6 Point-Salad Incommensurability ↔ A2 Decision Density (Parliament §9 row 4 — confirmed)

## 12. Open at DESIGN-stage, to resolve post-review

1. **Scoring balance.** Sapphire ×3 and Topaz ×5 may over-pay vs Ruby ×1. First play should record each player's max-by-type and calibrate.
2. **Pass action behavior.** If passes dominate player strategy, redesign the accelerate-endgame rule (e.g., PASS = draw 1 token instead of 2).
3. **Market size at 2p.** If 4-slot market at 2p creates insufficient variety, revert to 5-slot.
4. **Diamond-split rule.** Allow-split (as drafted) vs all-same-type. Split is more strategic; same-type is simpler. Recommend split for v1.0, re-evaluate.
5. **Pearl-3 starting buff.** Is +3 the right base value for "exactly 1 Pearl"? Could be +5 (stronger singleton reward) or +2 (weaker). Test.
6. **Count-robustness flags.** 2p with 4-slot variant may play very differently from 4p with 5-slot. Does Stegmaier's Count-Robustness stake engage?

All six resolvable via first-play calibration + panel review feedback.

## Compliance checks (per `/tigris-design` skill invariants)

- ✅ Section §4 action menu ≤6 actions: FACETS has exactly 2. Well within Action-Menu Clarity target.
- ✅ §6 specifies end-trigger AND scoring: both present.
- ✅ §5 covers every piece in §2: TAKE handles all token acquisition; PASS covers non-acquisition turns.
- ✅ Anchor declarations (Feld / C6) reflected: §10 designer notes explicitly call out the C6 claim.
- ✅ §11 adjacency chart surfaces FACETS-specific candidates for I-facets-01 amendment.
- ✅ §12 open questions present.

Ready for TIER-A STAKES.
