---
name: Rewild — Design (Rules)
slug: rewild-design
game: 0021-rewild
stage: design
version: 1.0.0
rubric_version: v2.18
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: rosenberg
anchor_axis: C3
---

# Rewild — Rules

## 1. Overview & Object of the Game

2-5 players. Each begins with an identical 15-card industrial deck. Over 10 rounds, players acquire Wildlife from a public Riverboard and **remove** Industrial cards from their own decks. Scoring at round 10: **(biome diversity × wildlife populations) − (remaining industrial footprint)**. Highest score wins.

Core tension: industrial cards generate Gold (the economy); wildlife cards pay Biome points (the win condition). Every industrial removal reduces future Gold. Every wildlife acquisition fills a deck slot without Gold return. Players calibrate a trade-off that shifts across the 10-round arc.

## 2. Components

- **Industrial Starter Deck** (15 cards × 5 copies for 5-player max):
  - 3x Road (Gold 1, Footprint 1)
  - 3x Pipeline (Gold 2, Footprint 2)
  - 3x Clear-cut (Gold 1, Footprint 2)
  - 3x Drainage (Gold 2, Footprint 1)
  - 3x Monoculture (Gold 3, Footprint 3)
- **Wildlife Deck** (80 cards: 5 biomes × 16 species each). Shuffled at setup.
- **Riverboard** (5 face-up slots for Wildlife).
- **Biome-Season deck** (10 cards; one drawn per round — shifts scoring weights).
- **Gold tokens** (supply; ~60 total).
- **Biome Point tokens** (tracked on scoring track per biome).
- **Round Counter** (1-10).
- **Reference cards** (removal costs, biome definitions, niche types).

## 3. Setup

1. Each player takes Industrial Starter Deck (15 cards identical across players). Shuffle; place face-down as personal Draw pile. Discard pile starts empty.
2. Shuffle Wildlife Deck; deal 5 cards face-up to Riverboard.
3. Shuffle Biome-Season deck.
4. Each player starts with 2 Gold.
5. Round Counter at 1. First-player randomly chosen. Round 1 begins.

## 4. Turn structure

Each round has 4 phases:

### 4.1 Season phase
Reveal top Biome-Season card. It announces this round's **Hot Biome** (e.g., "Forest: Forest wildlife populations score 2× this round" or "Wetland: Wetland acquisitions cost −1 Gold"). Effects apply this round only.

### 4.2 Draw phase
Each player draws 5 cards from their personal Draw pile (reshuffle discard if needed).

### 4.3 Play phase (simultaneous; no draft order)
Each player plays all 5 cards face-up in front of them. Sum Gold generated (industrial cards) and count any card-trigger effects. Wildlife in hand are placed in personal Tableau but generate Biome points only at end-game; not played-and-discarded.

### 4.4 Action phase (clockwise starting with first-player)
On your turn in the Action phase, you may take any combination of actions costing Gold, up to your Gold supply:

- **Acquire Wildlife** (cost = species Gold cost, modified by season): take a Wildlife card from Riverboard; place in your Tableau. Refill Riverboard from deck.
- **Remove Industrial** (cost = scaling: 2 Gold for first removal this game, 3 for 2nd, 4 for 3rd, 5 for 4th+): choose an Industrial card from your discard pile and permanently trash it (return to box).
- **Trade** (cost 1 Gold): draw top Wildlife card from deck; look; keep or return to Riverboard top.

Unspent Gold is discarded end-of-round; Gold does not persist.

After all players have acted, used cards move from play area to discard pile. Round Counter advances.

### 4.5 End-trigger check
After round 10 completes, game ends. Proceed to scoring.

## 5. Actions

3 action types:

| Action | Cost | Effect |
|---|---|---|
| Acquire Wildlife | species Gold cost (1-6, season-modified) | Add to Tableau |
| Remove Industrial | 2/3/4/5 Gold (scaling) | Permanently trash 1 Industrial from discard |
| Trade | 1 Gold | Draw Wildlife; keep or return |

3-action menu is **deliberately narrow** per A6 lineage (Vigil's 4-icon menu). Complexity lives in the 80-card Wildlife catalog and season variability.

## 6. Scoring / end condition

**End condition**: round 10 completes. No early-end trigger (to prevent rushing-rewild dominant strategy).

**Scoring formula** (computed after round 10):

```
Score = (Biome Diversity × Wildlife Population Sum) − (Remaining Industrial Footprint)
```

Where:
- **Biome Diversity** = number of distinct biomes with ≥ 1 wildlife in Tableau (0-5).
- **Wildlife Population Sum** = sum of Population values on all wildlife in your Tableau.
- **Remaining Industrial Footprint** = sum of Footprint values on Industrial cards still in your deck + discard + hand.

Plus:
- **Niche bonuses** (+3 per complete niche set of 5 different niches within a biome).
- **Biome dominance** (+5 for having most Population in a single biome across all players; ties share 5).

Highest total wins. Ties broken by: most niche bonuses → most wildlife cards → fewest industrial cards remaining.

## 7. Edge cases & clarifications

- **Deck exhaustion**: when personal Draw pile empties, reshuffle discard to form new Draw pile. This happens ~2x per game at most.
- **Riverboard empty**: if Wildlife Deck is empty and Riverboard empties, no more acquisitions this game.
- **Season effects stack**: if season modifies cost and a card has cost-reduction trigger, apply both (minimum 1 Gold for acquisition).
- **Biome assignment**: each Wildlife card has exactly 1 biome. Species with "multi-biome" text in expansions count for each listed biome but only take up 1 slot.
- **Industrial card in hand at round 10**: counts as Footprint for scoring. You cannot remove cards from hand during round 10 without having drawn them in round 10.
- **Trading with Trade action**: if kept, the drawn Wildlife goes to your Tableau (not hand); cost of acquiring is still paid. If returned, it goes to top of Wildlife Deck, visible to next Trade action.

## 8. Special subsystems

### Biome-Season deck (10 cards)

Each card shifts scoring weight for 1 round. Examples:
- "Forest Year": Forest wildlife populations score ×2 this round (affects acquisition, not endgame).
- "Mass Extinction": all players must remove 1 Wildlife from Tableau.
- "Lush Year": all acquisitions cost −1 Gold.
- "Industrial Boom": all Gold from industrial cards doubles; all removal costs +1.
- "Quiet Year": no special effect.

Shuffled at setup, drawn once per round. 10 rounds × 1 per round; deck runs out just as game ends.

### Scaling Removal Costs

First removal: 2 Gold. Second: 3. Third: 4. Fourth+: 5 each. Prevents single-turn industrial-dump strategies. Encourages phased rewilding.

### Niche categories

5 niches per biome: Predator / Herbivore / Pollinator / Detritivore / Aquatic. Niche bonuses reward ecological completeness, not just raw population.

## 9. Variants

- **2-player variant**: Riverboard reduced to 3 slots; starting Gold increased to 4.
- **5-player variant**: Riverboard expanded to 6 slots; Wildlife Deck shuffled at 2 points (middle refresh).
- **Speed variant**: game ends round 7 (2-session evening play).
- **Expert variant**: after 3+ plays, enable "Keystone Species" — 5 extra cards with multi-biome effects and cross-niche triggers.

## 10. Designer notes

**Load-bearing A7 counter-pressure bet.** Rewild is explicitly designed as the 2nd counter-pressure game for A7 Emergence-Replayability (pair with Covenstat for v2.18 trigger). Specific design choices for A7:
- **80-card Wildlife deck** — vast enough that no single session uses more than ~50 cards; 3 sessions sample different card subsets.
- **5 biome × per-game weights** — Biome-Season deck shuffle means different games emphasize different biomes, forcing different optimal strategies.
- **Personal-starting-state dissimilarity emerges from play divergence** — every player starts identical, but by round 3, players have diverged enough that strategic arcs are distinct.
- **No dominant-strategy trap** — starting with identical decks and acquiring from a shared pool prevents "everyone converges on deck X" that broke AFoS (Halifax Hammer).

**Why anchor is Rosenberg C3 Scarcity Bite, not A7:** C3 is the moment-to-moment experience (Riverboard competition, Gold economy tension). A7 is the cross-game signature (emergence across plays). The panel will test both; Rosenberg's C3 is the primary stake, A7 is the ledger-aware secondary.

**Inverted engine — is it still engine-garden?** This will be a Vaccarino C4 collision. Vaccarino's territory is deck-building. Rewild is deck-UN-building. Vaccarino will test whether removing cards counts as engine-garden. Covenstat's orthogonal-preservation pattern suggests this collision resolves to OP (both axes can be true: it is an engine, AND it inverts).

**No catastrophes.** Unlike Vigil or Covenstat or Nemesis, Rewild doesn't rely on catastrophe pressure. Tension comes from scarcity + trade-off calibration. B2 will retire-explicit honestly.

## 11. Collision Adjacency Chart

Novel adjacencies surfacing from Rewild:
- **C3 Scarcity ↔ C4 Engine-Garden** (inverted) — scarcity of wildlife and scarcity of "engine diminishment" are two sides of the same calibration. Candidate OP collision.
- **A7 Emergence ↔ B6 Scoring Multiplier** — emergence via biome-diversity multiplier. Scoring formula `(diversity × populations)` IS the multiplier-dependence. In nature-scoring games these gear together.
- **C3 ↔ A4 Variance Calibration** — scarcity of wildlife cards depends on Riverboard deal variance. Tight conditional adjacency.

## 12. Open at DESIGN-stage, to resolve post-review

- **Wildlife deck size**: 80 vs 60 vs 100? 80 chosen; test emergence at 60 (smaller catalog) to see if A7 still earns.
- **Starting industrial card Gold generation**: constant or degrading? Constant for now; test degrading in expert variant.
- **Removal cost scaling**: 2/3/4/5 vs 3/3/3/3 vs 1/2/4/8? Scaling 2/3/4/5 chosen.
- **Biome-Season effects**: 10 cards enough? Need more texture, or more abstract weights?
- **Niche bonus +3**: right magnitude? Too much = complete-the-set dominates; too little = niche-ignored.
- **Is 10 rounds the right length**? Too long risks boredom; too short risks rushing-rewild dominant strategy. 10 chosen with 6-round Speed variant as escape.
- **Does A7 earn?** The bet.
