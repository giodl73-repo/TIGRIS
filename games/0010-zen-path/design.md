---
name: ZEN PATH — Design (Rules)
slug: zen-path-design
game: 0010-zen-path
stage: design
version: 1.0.0
rubric_version: v2.9
bet_version: parliament
author: TIGRIS (user + Claude collaboration)
created: 2026-04-19
updated: 2026-04-19
anchor_persona: knizia
anchor_axis: minimum-score-shape
concept_version: v1.1 (panel-reviewed; 3-validator greenlight)
protagonist_subsystem: aspect-partitioned-steps
---

# ZEN PATH — Rules

## 1. Overview & Object of the Game

ZEN PATH is a 3-4 player race game where samurai traverse a branching path on a journey to the Peak. Movement is not dice-driven — each movement *commits you to an aspect* (Honor, Strength, or Strategy). Your samurai's position at game end is multiplied by your **minimum aspect** to determine final score. Racing alone wins nothing. Neglect costs everything.

**Object:** highest `position × MIN(Honor, Strength, Strategy)` score.

**Win condition:**
```
final_score = track_position × MIN(Honor, Strength, Strategy)
```

**Protagonist subsystem** (Chvátil's validation note): **aspect-partitioned Steps**. Every movement tile commits to one of three aspects. This is the load-bearing mechanical bet — you cannot race without building aspects. The other systems (branching forks, Moon Phase, private hand) support this protagonist.

## 2. Components

| Item | Count | Notes |
|---|---:|---|
| Branching path board | 1 | 40 positions; 2 forks at position 10 and 20. Sumi-e art. |
| Aspect track board | 1 | Three parallel tracks (0-10 each) for Honor, Strength, Strategy. |
| Samurai tokens | 4 | One per player; distinct colors. |
| Aspect markers | 12 | Three per player; track on aspect board. |
| Tile deck | 51 | 12 Step + 12 Ascend + 6 Focus + 10 Ladder/Chute + 4 Duel + 3 Moon + 4 pre-placed path markers = 51 |
| Tile bag | 1 | Opaque, for draw actions |
| Hand screens | 4 | Private hand display (one per player) |
| Moon cycle tracker | 1 | Counts rounds; triggers Moon Phase every 5 rounds |
| Player aid cards | 4 | Quick reference: scoring equation + action menu + aspect legend |
| Rulebook | 1 | This document |

**Tile distribution** (51 total):

| Type | Count | Per-tile |
|---|---:|---|
| **Honor-Step** | 4 | Move 1-3 squares; +1 Honor |
| **Strength-Step** | 4 | Move 1-3; +1 Strength |
| **Strategy-Step** | 4 | Move 1-3; +1 Strategy |
| **Honor-Ascend** | 4 | +1 Honor (no movement) |
| **Strength-Ascend** | 4 | +1 Strength |
| **Strategy-Ascend** | 4 | +1 Strategy |
| Focus | 6 | +2 to one chosen aspect |
| Ladder | 5 | Place on path; trigger +moves on condition |
| Chute | 5 | Place on path; trigger -moves on condition |
| Duel | 4 | Aspect comparison with adjacent player |
| Moon | 3 | Trigger Moon Phase early |

## 3. Setup

1. Place the branching path board at center; place Aspect Track board beside it.
2. Set each player's three aspect markers to 0 on the Aspect Track (Honor, Strength, Strategy).
3. Each player places their samurai token at **Origin** (position 0).
4. Shuffle the 51 tiles into the bag.
5. Deal each player 3 tiles into hand (private).
6. Reveal 3 tiles face-up in the public market.
7. Set the Moon Cycle tracker to 0 (Round 1 begins).
8. Determine first player randomly.

Setup time: <5 minutes.

## 4. Turn structure

Each turn, the active player performs exactly **2 actions**. Actions are resolved sequentially; cascade/trigger effects resolve immediately after each action.

After 2 actions:
1. Check if any player has reached Peak (position 40). If yes, enter final round.
2. Increment Moon Cycle. If Moon Cycle hits 5, trigger **Moon Phase** (see §8).
3. Play passes clockwise.

Hand size cap: **5 tiles** at 3-4p; **4 tiles** at 2p.

## 5. Actions in detail

Each action is ONE of the following (pick any 2 per turn; same action can repeat):

### 5.1 DRAFT — 1 action

Take one of the 3 face-up tiles from the market into your private hand. Refill the market slot by drawing from the bag. If the bag is empty, slot stays empty.

Cannot DRAFT if your hand is full (5 tiles at 3-4p).

### 5.2 DRAW — 1 action

Draw one tile from the bag directly into your hand (opponents don't see it). If the bag is empty, skip (no tile gained).

Cannot DRAW if your hand is full.

### 5.3 PLAY — 1 action

Play one tile from your hand. Apply its effect. Place the tile in the discard pile (or on the path, for Ladder/Chute tiles).

See §6 for tile effects.

## 6. Tile types — detailed

### 6.1 Step tiles (12; aspect-partitioned — PROTAGONIST)

Each Step tile is dedicated to ONE aspect.

- **Honor-Step** (×4): Move your samurai **1-3 squares forward** (tile shows a specific number: 2 at 1-space, 1 at 2-space, 1 at 3-space). Also +1 Honor on your aspect track.
- **Strength-Step** (×4): same pattern, Strength aspect.
- **Strategy-Step** (×4): same pattern, Strategy aspect.

**Design note (Knizia fix)**: movement is inseparable from aspect-commitment. You cannot move without building an aspect. The race and the min-score shape are geometrically coupled.

### 6.2 Ascend tiles (12)

Each Ascend tile is dedicated to ONE aspect. Play to gain **+1 to that aspect**. No movement.

- Honor-Ascend (×4), Strength-Ascend (×4), Strategy-Ascend (×4).

Use to top up an aspect without advancing position.

### 6.3 Focus tiles (6)

Play to gain **+2 to any chosen aspect** (your choice at time of play). Rarer; stronger.

### 6.4 Ladder tiles (5)

**Place on any unoccupied path square** as a new ladder. When played:
- You pick which aspect-condition to print on the ladder from these 3 options:
  - "Honor ≥ N: +3 squares"
  - "Strength ≥ N: +3 squares"
  - "Strategy ≥ N: +3 squares"
- Where N is the level you choose (3, 5, or 7).

When a samurai lands on the ladder, if they meet the aspect-condition, they advance +3 squares immediately. If not met, no effect.

### 6.5 Chute tiles (5)

**Place on any unoccupied path square** as a new chute. When played:
- Pick aspect-condition and threshold from:
  - "Honor < N: -2 squares"
  - "Strength < N: -2 squares"
  - "Strategy < N: -2 squares"

When a samurai lands, if they FAIL the aspect-condition, retreat -2 squares.

### 6.6 Duel tiles (4)

Tile is printed with one of the 3 aspects (resolution-on-card per Vaccarino's note). Play to trigger:
- Choose an **adjacent** player (within 3 squares on the path).
- Compare your aspect (as printed on the Duel tile) to theirs.
- Loser retreats 2 squares.

### 6.7 Moon tiles (3)

Play to **trigger a Moon Phase immediately** (see §8). Tactical timing — you can force the obligation on a player about to benefit from their lead.

### 6.8 Pre-placed path markers (5-6)

The path board has pre-printed ladder and chute markers at fixed positions (these are not in the tile deck). Each has a printed aspect-condition (e.g., "Honor ≥ 5: +3"). Functions as permanent features.

## 7. Branching forks

The path is NOT a single line. It splits at two forks:

### 7.1 Fork 1 (position 10)

Path splits into:
- **Mountain Path** (10 squares): rugged; ladders favor Honor/Strength; chutes penalize Strategy.
- **River Path** (10 squares): flowing; ladders favor Strategy; chutes penalize Strength.

Rejoin at position 20.

When a samurai reaches position 10, they must declare which fork to enter before their next action. Cannot backtrack.

### 7.2 Fork 2 (position 20)

Path splits into:
- **Temple Path** (10 squares): honor-centric; ladders favor Honor.
- **Market Path** (10 squares): market-centric; ladders favor Strategy/Strength.

Rejoin at Peak (position 40).

### 7.3 Fork choice strategy

Choose forks based on your aspect mix. A Honor-heavy samurai thrives on Mountain + Temple. A Strategy-heavy samurai thrives on River + Market. Mixed aspects have flexibility.

Other players' forks don't block yours — each player traverses their own choices. But: ladder/chute placements are shared across forks (you can place a new ladder on an opponent's fork).

## 8. Moon Phase

Every 5 rounds (tracked by Moon Cycle), a Moon Phase triggers before the next player's turn. Moon Phase also triggers immediately when any player plays a Moon tile.

When Moon Phase fires, each player (starting with the current player, clockwise) picks ONE:

- **REFLECT** — remove 1 point from your LOWEST aspect. (Min-score pressure.)
- **RETREAT** — move your samurai back 2 squares on the path.
- **PURGE** — discard 2 tiles from your hand face-down into the discard pile.

Skipping Moon Phase: **−1 VP at endgame per skip**. Default penalty for passivity.

Moon Phase does NOT trigger cascades or additional effects.

After Moon Phase resolves for all players, reset Moon Cycle to 0 and continue play.

## 9. Scoring / end condition

### 9.1 End triggers

Game ends at end of the current round when any of:
- A samurai reaches Peak (position 40).
- Tile bag is empty AND market has ≤ 2 tiles.
- All players have declared a Moon Phase Retreat 3+ consecutive times (stall-protection).

### 9.2 Final scoring

For each player:
1. Calculate **MIN(Honor, Strength, Strategy)** — your lowest aspect value.
2. Multiply by your **track position**.
3. **Subtract 1 VP per skipped Moon Phase**.

`final_score = position × MIN(H, S, St) − skipped_moons`

### 9.3 Tiebreakers

1. Second-lowest aspect (tied: use third-lowest).
2. Highest track position.
3. Shared victory.

## 10. Edge cases & clarifications

- **Aspect cap**: aspects cap at 10. Excess Ascend/Focus plays go to waste (no rebate).
- **Aspect floor**: aspects cannot go below 0. Moon REFLECT on a 0 aspect has no further effect.
- **Ladder/chute stacking**: each path square holds at most ONE player-placed tile (plus the pre-printed marker, if any). If both trigger, resolve pre-printed first, then player-placed.
- **Retreat past Origin**: cannot go below position 0. Retreat lands you at 0.
- **Duel with no adjacent player**: Duel tile wastes (no effect); discard.
- **Samurai at Peak**: stays at Peak; future moves discarded. Game end-trigger check follows.
- **Market empty**: DRAFT action is unavailable; DRAW and PLAY remain.
- **Hand full + must DRAW (e.g., tile effect)**: discard 1 tile from hand (your choice) first.
- **Fork choice mid-turn**: you declare fork BEFORE your next action; cannot change once declared.
- **Multiple Ladder tiles on one square**: not allowed; rule prevents stacking.
- **Moon tile during Moon Phase**: Moon tile played during Moon Phase is wasted (cannot trigger a Moon within a Moon).

## 11. Collision Adjacency Chart (ZEN PATH-specific candidates)

Potential new adjacency pairs surfaced by ZEN PATH's mechanics:

- **C2 Min-Score Shape ↔ B3 Conversion Chain Depth** — already in chart (row 9). ZEN PATH reaffirms.
- **D2 Spatial-Interaction Presence ↔ D4 Late-Game Lock-in** — branching forks commit samurai to paths; spatial choice = lock-in. Possible new adjacency.
- **A3 Interaction ↔ C5-retired** — Duel tiles were anti-catch-up in spirit; C5 retired in v2.6. If the retired C5 had remained, adjacency-candidate here.

If ZEN PATH earns D2 strongly (branching forks), log D2↔D4 as I-zen-01 candidate for chart amendment.

## 12. 2-player variant

- **Path length**: 30 squares (Origin → Fork 1 at pos 8 → rejoin 15 → Fork 2 at 22 → Peak at 30).
- **Tile pool**: 40 tiles (reduce proportionally — 8 Steps, 8 Ascends, 4 Focus, 8 Ladder/Chute, 3 Duel, 2 Moon, 7 misc).
- **Hand cap**: 4 tiles.
- **Moon Phase**: every 4 rounds.
- **Expected length**: 25-35 min.
- **Aspects**: still 3 (Honor, Strength, Strategy); scoring equation unchanged.

## 13. Solo mode (sketched — to finalize in v1.1)

### 13.1 Automa: The Master

- Play against a scripted opponent called **The Master**.
- The Master uses a 20-card action deck (separate from the tile deck):
  - 8 Move cards (Master advances 1-3 squares)
  - 6 Ascend cards (Master gains +1 in a specified aspect)
  - 3 Duel cards (Master initiates a Duel with the solo player)
  - 2 Fork-choice cards (Master picks fork based on state)
  - 1 Master's Wisdom (Master gains +1 to all aspects)
- Each round: solo player takes their turn, then flip top Master card.

### 13.2 Win condition

Solo player wins by scoring higher than The Master. The Master's final aspects: {Honor: 6, Strength: 6, Strategy: 6} fixed. Solo must beat 6 × position.

### 13.3 Difficulty levels

- Easy: Master takes 1 card per round.
- Normal: Master takes 2 cards per round.
- Hard: Master takes 3 cards per round.

Solo specifics to be finalized after first solo playtests.

## 14. Designer notes

### 14.1 Why ZEN PATH

The design cross — Chutes & Ladders × Samurai — was the user's commissioning prompt. The central idea: replace C&L's dice with Samurai's min-score shape, turning random jumps into aspect-gated events.

### 14.2 The protagonist subsystem (Chvátil's validation note)

Chvátil flagged v1.1's compound bet: "weirdness distributed across 4 ribs instead of 1 spine." The spine, after design analysis, is **aspect-partitioned Step tiles**.

Every move is an aspect commitment. This single mechanical choice IS the bet:
- It makes race + one-aspect dominance arithmetically impossible (Knizia arithmetic).
- It creates the gearing between movement and scoring (Lacerda system-gearing).
- It justifies the other subsystems (branching forks reward aspect-choice; Moon Phase punishes aspect-neglect; private hand protects aspect-strategy).

If one system were removed without replacement, aspect-partitioned Steps would still produce a playable game. Remove aspect-partitioned Steps, and the game reverts to v1.0's dominant-strategy problem.

This is the protagonist. Others are supporting cast.

### 14.3 Inheritance from Knizia

ZEN PATH is built on Knizia's minimum-score shape. Samurai (1998) is the direct ancestor. Unlike Samurai's tile-influence-on-cities, ZEN PATH's aspects live on per-player tracks — a simpler UI for the same math. Tigris & Euphrates, Ingenious, and Taj Mahal are spiritual siblings.

### 14.4 What UNFOLD taught us

UNFOLD (TIGRIS original #1) introduced the concept→panel-review→revise→validate→design workflow. ZEN PATH applied the same workflow with sharper panel feedback (5 revisions vs UNFOLD's 4). Both originals pass their validation passes.

### 14.5 Open questions to resolve in playtest

Listed in §15.

## 15. Open at DESIGN-stage — to resolve post-playtest

1. **Step-tile movement values** — 1-space : 2-space : 3-space ratio. Proposed 2:1:1; may need 1:2:1.
2. **Ascend/Focus reservoir balance** (Knizia's v1.1 note) — do Ascend tiles stay fungible enough? Tune count.
3. **Ladder/Chute threshold levels** — 3/5/7 proposed; may need 2/4/6 for lighter games.
4. **Moon Phase cycle** — every 5 rounds at 3-4p, every 4 at 2p. Maybe scale.
5. **Fork aspect bias** — balanced so neither fork dominates.
6. **Hand size** — 5 at 3-4p may need 6 to avoid forced PURGE penalties.
7. **2p tile ratio** — 40 tiles scaled; balance check.
8. **Duel aspect assignment** — printed on card (v1.1 approved per Vaccarino); variety.
9. **Tie breakers** — second-lowest aspect first; may need multiple tier-based rules.
10. **Solo difficulty scaling** — 1/2/3 Master cards/round; validate each works.

## Compliance checks (per /tigris-design)

- ✅ §5 action menu = 3 types (DRAFT, DRAW, PLAY). Well within 6 ceiling.
- ✅ §9 end condition and scoring both specified.
- ✅ §6 covers every tile type; §3 setup covers all components.
- ✅ Anchor declarations (Knizia / C2) reflected throughout; §14.3 explicit.
- ✅ §11 new-adjacency candidates flagged.
- ✅ §15 open questions present.
- ✅ Protagonist subsystem identified (aspect-partitioned Steps, per Chvátil).

## Status

**Design v1.0 complete — 2026-04-19.** Ready for TIGRIS panel review via `/tigris-panel` game-review mode. First physical playtest deferred to post-panel-review.

The protagonist bet: movement is aspect commitment. Win depends on balance. Panel decides whether this earns B5 at 8+ now.
