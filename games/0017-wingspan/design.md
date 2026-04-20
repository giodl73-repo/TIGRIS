---
name: Wingspan — Design Import
slug: wingspan-design
game: 0017-wingspan
stage: design
version: 1.0.0
rubric_version: v2.14
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
bgg_id: 266192
bgg_url: https://boardgamegeek.com/boardgame/266192/wingspan
canonical_edition: 1st (2019, Stonemaier Games)
game_designer: Elizabeth Hargrave
game_first_published: 2019 (Stonemaier Games)
review_type: imported-review
sources:
  - Wingspan rulebook (Stonemaier 2019)
  - BGG 266192
---

# Wingspan — Design Import (fair-use rules summary)

North American bird-watching engine-builder. 1-5 players (with Automa solo). ~70 min. BGG weight 2.43.

## Core loop

**4 rounds** of decreasing action counts (8, 7, 6, 5 turns). Each turn: take 1 of 4 actions:

1. **Play a bird** (pay food + egg cost; place in habitat row)
2. **Gain food** (forest row; dice from bird feeder tower)
3. **Lay eggs** (grassland row; place on existing birds)
4. **Draw bird cards** (wetland row; 1 face-up or from deck)

When activating a row, trigger each played bird's "when activated" power from right → left.

## Bird cards (170 in base set)

Each bird has:
- **Food cost** (1-5 food tokens of specific types).
- **Egg capacity** (1-6; eggs = VP + resources).
- **VP value** (0-9).
- **Habitat assignment** (Forest / Grassland / Wetland).
- **Power**: when-played (one-time), when-activated (repeating), once-between-turns (reactive), end-of-round. ~90 unique power types.

## Scoring dimensions

End-of-game sum of:
- **Bird point values** played.
- **Bonus card** (secret goal assigned at setup; 27 unique).
- **End-of-round goals** (variable per round; 14 possible).
- **Eggs** on birds (1 VP each).
- **Cached food** on birds (1 VP each).
- **Tucked cards** on birds (1 VP each).
- **Nectar** (expansion; 1-2 VP per in habitats).

**6+ scoring dimensions** per game.

## Asymmetry

- **Bonus cards**: each player has a secret scoring objective (e.g., "most cavity-nesting birds").
- **Round-end goals**: revealed at setup; chosen from pool (most eggs in grassland, most birds in forest, etc.).
- Automa solo mode available.

## Design signatures

- **Bird card as dual resource**: each played bird is a scoring target AND a recurring engine component.
- **Habitat rows as activation order**: right-to-left triggering produces chain-effects.
- **Food-dice-tower**: passive randomizer for food availability.
- **Multiplayer-solitaire critique**: players largely build parallel tableaus; primary interactions limited to:
  - Card-market drafting (shared face-up card pool).
  - End-of-round goals (scoring comparisons).
  - Very few bird powers target opponents.

## Family-to-expert engineering

- **Automa** for solo (Stonemaier house style).
- **Teach**: core loop is 4 actions; powers are card-by-card.
- **Expert**: bonus card synergy + combo engines + habitat efficiency.
- **Swan Song variant**: alternative round structure for advanced play.

## The multiplayer-solitaire question

Wingspan's A3 Interaction refute rests on structural evidence:
- 90% of bird powers affect only the owner's tableau.
- Card market + round goals are the only meaningful interactive layers.
- At 3-5p, players report low influence-on-each-other.

This is the canonical test for **adopted-axis A3 Interaction**: can Chvátil (A3's own primary advocate) self-refute on structural grounds?

Proceeding to TIER-A.
