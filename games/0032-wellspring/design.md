---
name: WELLSPRING — Design (Rules)
slug: wellspring-design
game: 0032-wellspring
stage: design
version: 1.0.0
rubric_version: v2.23.3
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: vaccarino
anchor_axis: A4
---

# WELLSPRING — Rules

## 1. Overview & Object

2–4 player simultaneous deck-builder. Players manage 4 personal pool-stacks (Fire, Water, Earth, Air) of face-down cards. Each round, players simultaneously reveal the top card of one chosen pool and execute its action. The player with the most VP at game end wins.

**The variance bet:** you know your well's composition (the face-up discard adjacent to each pool shows what's been played), but not the card order. Managing WHICH pool you concentrate good cards into is the strategic core.

## 2. Components

- 120 Market cards (5 types × 24: Harvest, Craft, Trade, Scout, Legacy — each with an action and a VP value)
- 20 Starting cards per player (5 × Spark cards, 5 × Stone cards, split between two pools)
- 4 Pool dividers per player (Fire / Water / Earth / Air labels)
- 20 face-up Discard-display holders (one per pool per player; held upright so composition is visible)
- 1 Market row board (6 face-up Market cards + draw deck)
- VP tokens: 1s, 5s, 10s
- Round track (8 rounds)

## 3. Setup

1. Each player places 4 pool-dividers in front of them (forming 4 stacks).
2. Distribute 20 Starting cards: place 5 Sparks face-down into Pool Fire; 5 Stones face-down into Pool Earth; 5 Sparks into Pool Water; 5 Stones into Pool Air.
3. Place 6 Market cards face-up in the Market row; shuffle remainder as draw deck.
4. Set round track to Round 1.

## 4. Turn Structure (per round)

### Phase 1 — Simultaneous Reveal
All players simultaneously choose one of their 4 pools and reveal (flip face-up) its top card. Place it on the discard-display for that pool. Reveal is simultaneous — you cannot see others' choices before committing.

### Phase 2 — Action Resolution (in turn order)
Each revealed card has 1 action. Resolve in clockwise order:

| Card type | Action |
|---|---|
| **Spark** | Gain 1 Gold |
| **Stone** | Gain 1 Material |
| **Harvest** | Gain 2 Gold. If you have ≥4 Harvest cards in this pool's discard, gain +1 VP. |
| **Craft** | Convert 2 Material → 1 VP token. |
| **Trade** | Give 1 Gold to any player; draw 1 Market card to hand. |
| **Scout** | Look at top 3 cards of any of your pools; reorder them freely. |
| **Legacy** | Score this card's VP value immediately (1–5 VP). |

### Phase 3 — Purchase (in turn order)
Each player may spend Gold + Material (as shown on Market cards) to buy 1 Market card from the row. Purchased card is placed face-down on any of your 4 pools (player's choice of which pool). Refill Market row from deck.

### Phase 4 — Pool Cycle
If any pool is empty (all cards in discard), shuffle that pool's discard back face-down as its new pool-stack.

**Engine-Garden rule (C4 designed mechanic):** When a pool is cycled back, if it contains ≥6 cards of the same type: gain 1 bonus VP per same-type card above 5. This rewards concentrated pools — "gardening" one pool with a single card type.

### Phase 5 — End-of-round check
After 8 rounds, the game ends. Score all Legacy cards' face-up VP values plus accumulated VP tokens.

## 5. Actions (summary menu)

| Action | Available | Cost | Effect |
|---|---|---|---|
| **Reveal pool** | Phase 1 | 0 | Flip top card of chosen pool |
| **Execute card action** | Phase 2 | varies | Per card-type action above |
| **Purchase Market card** | Phase 3 | Gold + Material | Gain 1 Market card; place on a pool |
| **Scout** (card action) | Phase 2 | 0 | Reorder top 3 of any your pool |
| **Trade** (card action) | Phase 2 | 0 | Give Gold; draw Market card |

## 6. Scoring

End of Round 8: sum VP tokens + face-up Legacy card VPs + Engine-Garden bonuses (from final pool-cycle if triggered in Round 8).

## 7. Designer Notes — C4 Counter-Pressure

**C4 Engine-Garden Dependency is in monitoring status** (7 games since last earn at CANTON #24). WELLSPRING includes the Engine-Garden rule in §4 Phase 4 explicitly to earn C4: concentrated pools of same-type cards are the "garden"; the cycle-bonus rewards building this architecture.

For C4 to earn canonically, the panel argument must confirm: (1) the pool architecture creates a garden-like dependency where early card placements determine late-game scoring potential, AND (2) the cycle-bonus rewards players who commit to a single card type per pool (dependency on having "planted" the right cards).

## 8. Open Questions

1. **Harvest ≥4 threshold**: is 4 cards the right concentration threshold for the Harvest bonus? Too low = fires too easily; too high = rarely fires.
2. **Pool cycle rate**: 8 rounds × 1 card revealed per round = 8 reveals per player. A starting pool of 10 cards cycles after 10 reveals (~1.25 rounds delay). Does this feel right?
3. **A3 simultaneous interaction**: revealing simultaneously means you can read opponents' choices if you wait (but you can't — it's simultaneous). Does this actually create interaction, or is it just parallel play?
