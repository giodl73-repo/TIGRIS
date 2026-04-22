---
name: Fast-Track Record Schema
slug: fast-track-schema
stage: spec
version: 1.0.0
rubric_version: v2.24
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
sources: []
---

# Fast-Track Record Schema — v1.0

## Purpose

The fast-track format enables data collection for the 150-game PCA (Principal Component Analysis) research program. Full pipeline records average 6,500 words; fast-track records target 300 words. The compressed format sacrifices argument narration for axis coverage breadth.

Fast-track records still count toward:
- 0-silent-retire streak (any silent-retire breaks the streak)
- Axis earn/retire ledger counts
- OP collision registry (if a collision is noted)
- Rubric patch versioning (each fast-track game = one minor patch)

Fast-track records do NOT produce:
- tierA-stakes.md (no full STAKES phase)
- playtests/PT*-argument.md (no full ARGUMENT narration)
- amendments.md (no formal AMENDMENT document)
- panel/SUMMARY.md

The record.md file IS the complete artifact.

## Classification criteria

Use **full pipeline** when:
- The game is an original TIGRIS design (full pipeline always required)
- The game is a milestone review (every 10th game, every OP pair first-instance)
- The anchor persona has never anchored this game type before
- A clock-reset or targeted-earn is required (B4, B5, C2, C6, C8 monitoring)
- The game is in the TIGRIS anchor sequence (parliament, T&E, Scythe, etc.)

Use **fast-track** when:
- The game is a published review
- The game's primary value is data-point coverage (weight bracket, genre gap)
- No active monitoring axis is the anchor
- The rubric has ≥ 5 earned instances of all key axes in the game's genre cluster

## Complete record.md template

```yaml
---
name: <Full Game Name>
slug: <NNNN-slug>
game: <NNNN-slug>
stage: handoff
version: 1.0.0
rubric_version: v2.24.<patch>
bet_version: parliament
author: fast-track
type: review | original | fast-track-original
bgg_id: <integer>               # required for reviews
bgg_url: https://boardgamegeek.com/boardgame/<id>
canonical_edition: <edition>
game_designer: <name>
game_first_published: <year> (<publisher>)
review_type: fast-track
weight: <BGG weight float>
anchor: <persona-slug>/<axis-id>
rubric: v2.24.<patch>
date: 2026-04-21
earned: <count>
silent_retire: 0
streak: <Nth consecutive 0-silent-retire>
---
```

## 28-Axis Table Format

```markdown
| Axis | Score | State | Note |
|---|---|---|---|
| A1 Elegance | 7 | earn | <one sentence anchored to specific mechanic> |
| A2 Decision Density | 6 | earn | <one sentence> |
| A3 Interaction | 4 | hold | <one sentence> |
| A4 Variance Calibration | 5 | hold | <one sentence> |
| A5 Downtime/Pacing | 8 | earn | <one sentence> |
| A6 Teachability | 7 | earn | <one sentence> |
| A7 Emergence/Replayability | 5 | hold | <one sentence> |
| B1 System Gearing | 3 | retire-explicit | <one sentence> |
| B2 Catastrophe Pressure | 2 | retire-explicit | <one sentence> |
| B3 Conversion Chain Depth | 4 | hold | <one sentence> |
| B4 Information Transparency Cost | 3 | hold | <one sentence> |
| B5 Architectural Novelty | 5 | hold | <one sentence> |
| B6 Scoring Multiplier Dependency | 2 | retire-explicit | <one sentence> |
| C1 Tension Budget | 7 | earn | <one sentence> |
| C2 Minimum-Score Shape | 1 | retire-explicit | <one sentence> |
| C3 Scarcity Bite | 2 | retire-explicit | <one sentence> |
| C4 Engine-Garden Dependency | 3 | retire-explicit | <one sentence> |
| C5 Anti-Catch-up Pressure | — | RETIRED | Axis retired v2.6; skip. |
| C6 Point-Salad Incommensurability | 4 | hold | <one sentence> |
| C7 Action-Menu Clarity | 8 | earn | <one sentence> |
| C8 First-Turn Compression | 5 | hold | <one sentence> |
| D1 Family-to-Expert Scaling | 7 | earn | <one sentence> |
| D2 Spatial-Interaction Presence | 2 | retire-explicit | <one sentence> |
| D3 Count-Robustness | 6 | earn | <one sentence> |
| D4 Late-Game Lock-in Point | 5 | hold | <one sentence> |
| A25 Emotional Arc | 3 [est] | hold | <one sentence> |
| A26 Social Contract Dependency | 5 [est] | hold | <one sentence> |
| A27 Physical Affordance | 2 [est] | retire-explicit | <one sentence> |
| A28 Cognitive Load Profile | 5 [est] | hold | <one sentence> |
```

States: `earn` | `hold` | `retire-explicit` | `silent-retire` (avoid — breaks streak). C5 is always `RETIRED`.

A25-A28 scores marked `[est]` until parliament-confirmed (full pipeline with argument narration).

## Collision section

```markdown
## Collision

**OP: <axisX>↔<axisY> <vote> (<Nth TP>)**
<one sentence on why these axes address genuinely different registers>
Both earn defend marks.
```

If no collision detected: `No collision identified.`

## Amendment section

```markdown
## Amendment

Rubric: v2.24.<patch>
Streak: <Nth> consecutive 0-silent-retire.
Earned: <count> axes.
Silent-retire: 0.

Axis ledger updates:
- <axis> +1 earn (cumulative: <total>E)
- <axis> +0.5 retire-explicit (cumulative: <total>R)
```

## Retroactive A25-A28 estimation for existing games

For games #1-#69 already in the corpus, A25-A28 scores can be estimated retroactively from:

1. **A25 Emotional Arc**: Infer from game type — legacy/campaign games score 7-9; cooperative games with fear/dread mechanics score 4-6; pure abstracts and euros score 0-3; dungeon crawls/narrative games score 5-8.

2. **A26 Social Contract Dependency**: Infer from genre — social deduction games (Hanabi, The Mind) score 8-10; bluffing/negotiation games score 6-8; most euros score 2-4; pure abstracts score 0-2.

3. **A27 Physical Affordance**: Infer from component type — dexterity games score 9-10; games with tactile components (tile-laying, heavy wood) score 4-6; card games and dice games score 1-3; fully digitizable abstracts score 0-1.

4. **A28 Cognitive Load Profile**: Map primary demand — roll-and-move=0; spatial (hive, blokus)=3; probabilistic (push-your-luck, auctions)=5; deductive (hanabi, mastermind)=7; narrative (dixit, mysterium)=8; novel-combined (specific axis combos)=10.

Retroactive estimates carry `[estimated]` marker and go in `games/NNNN-<slug>/retroactive_axes.md`.

## Design efficiency formula

For PCA analysis, efficiency = `(earned_count × op_bonus_multiplier) / weight`

Where `op_bonus_multiplier = 1.0 + (op_vote_strength - 5) × 0.1` for decisive OPs (≥7), else 1.0.

High-efficiency games punch above their weight in axis discrimination.
