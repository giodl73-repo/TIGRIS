---
name: FURROW — Design (Rules)
slug: furrow-design
game: 0028-furrow
stage: design
version: 1.0.0
rubric_version: v2.22.3
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: chvatil
anchor_axis: B5
counter_pressure_primary: A3
---

# FURROW — Rules

## 1. Overview & Object of the Game

FURROW is a 3–4 player trick-taking game across four seasons. Each season, players are dealt hands from a 48-card deck (4 suits × 12 cards: Grain 1–12, Barley 1–12, Hops 1–12, Flax 1–12). Players play tricks; winning a trick earns harvest tokens of the leading suit's crop type. At season's end, each player plants one harvest token into their personal 4-space Field Strip and scores adjacency bonuses.

The player with the most VP after four seasons wins. Ties broken by most remaining harvest tokens; further ties by reverse turn order.

## 2. Components

- **48-card deck** (Grain 1–12 yellow; Barley 1–12 amber; Hops 1–12 green; Flax 1–12 blue)
- **40 Harvest tokens**: Grain ×12, Barley ×12, Hops ×8, Flax ×8
- **4 Field Strips** (1 per player; 4 spaces; printed with adjacency scoring legend)
- **20 Crop tiles** (5 per player; placed on Field Strip when planting; printed with crop type)
- **1 VP score board**
- **4 VP markers**
- **1 Season track** (4 positions: Spring / Summer / Autumn / Winter)

## 3. Setup

1. Shuffle the 48-card deck. Deal 12 cards to each player (full deck at 4 players).
2. Set Season track to Spring.
3. Each player takes their Field Strip (empty) and set of 5 Crop tiles.
4. Determine first player (dealer). First player leads the first trick.

## 4. Trick-Taking Rules

**Standard trick-taking rules apply with one structural modification.**

### Playing a trick

1. The **leader** plays any card face-up.
2. All other players, in clockwise order, must **follow suit** (play a card of the same suit as the led card) if able. If unable, they may play any card.
3. The player who played the **highest card of the led suit** wins the trick.
4. **Hops exception**: Hops cards are **trump** — any Hops card beats any non-Hops card, regardless of value. If multiple Hops cards are played, the highest Hops wins. If only one Hops card is played among non-Hops followers, it wins the trick regardless of value.

### Earning harvest tokens

The **winner** of the trick takes a harvest token of the **led suit's crop type** from the supply:
- Lead suit was Grain → winner takes 1 Grain token
- Lead suit was Barley → winner takes 1 Barley token
- Lead suit was Hops → winner takes 2 Hops tokens (leading Hops earns a bonus)
- Lead suit was Flax → winner scores **2 VP immediately** (Flax tricks don't yield tokens; they score directly)

The winner of each trick leads the next trick.

### Season end

A season ends when all players have played their full 12-card hand (12 tricks per season at 4 players).

## 5. Planting Phase (after each season)

After each season, each player **plants exactly one harvest token** from their supply onto their Field Strip:
- Place the chosen Crop tile on the leftmost empty space of your Field Strip.
- The crop type of the Crop tile matches the harvest token type spent.
- Once planted, Crop tiles are permanent — no repositioning.
- Unplanted harvest tokens carry over to future seasons.

**Players who have no harvest tokens at planting phase** plant nothing this season and gain 1 wild VP instead (consolation; prevents zero-token players from being locked out).

## 6. Field Strip Scoring (after all four seasons)

After Winter (season 4), score all players' Field Strips:

| VP per Crop tile | Base VP | Adjacency bonus |
|---|---|---|
| Grain | 2 VP | +1 VP per adjacent Grain tile |
| Barley | 2 VP | +1 VP per adjacent Barley tile |
| Hops | 3 VP | No adjacency bonus |
| Flax | 1 VP | +2 VP if all 4 Field Strip spaces are occupied |

**Plus**: immediate Flax-trick VP accumulated during seasons (already scored; not re-scored here).

## 7. Edge Cases & Clarifications

- **Hops as lead**: if the leader leads a Hops card, all players must follow with Hops if able. Hops has no trump status in this context — it leads normally. The trump rule only applies when Hops is played off-suit (as a discard when unable to follow another suit).
- **Leading Hops for 2 tokens**: leading a Hops card earns 2 Hops tokens when the trick is won. Following Hops off-suit to win a Grain-led trick earns 1 Grain token (the led suit), not a Hops bonus.
- **Renege**: playing off-suit when you hold the led suit is a renege. At season end, reneging player loses 3 VP. (Honor system; revealed by hand inspection if disputed.)
- **Field Strip full**: after season 4, players with a full 4-tile Field Strip do not plant. Any remaining harvest tokens score 0 VP.
- **3-player variant**: deal 12 cards to 3 players (36 cards dealt; 12 cards removed randomly and set aside face-down). 3 seasons instead of 4.

## 8. Designer Notes — A3 Counter-Pressure Documentation

*Required for v2.18 retirement-reversal eligibility.*

**Counter-pressure target: A3 Interaction** — contested-watch at 9E/1R. FURROW is counter-pressure cycle 3 game #2 (WEALD was game #1). Combined, WEALD + FURROW satisfy the v2.18 two-game requirement.

**FURROW's counter-pressure mechanism — trick-taking lead/follow:**

NEST's refutation argument: "parallel life-boards = multiplayer-solitaire; one player's action doesn't affect another's board state."

FURROW's counter-argument: the lead/follow obligation is collective mandatory interaction. On every single trick in FURROW:
1. The leader's card choice (suit and value) directly constrains all other players' choices — you must follow suit if able.
2. You cannot opt out of a trick — passing is not a rule.
3. Your card choice affects who wins the trick, which affects who leads next, which affects what the next trick's suit will be.

This is mandatory interaction at the **collective register**: every player's decision is simultaneously constrained by and constrains every other player's decision on every trick. No player can act as if others don't exist.

The interaction register differs from WEALD's:
- WEALD: targeted (I displace YOUR specific structure)
- FURROW: collective (I lead a suit; ALL players must respond)

Both are mandatory. Together they demonstrate that A3 earns across different interaction registers.

**For v2.18 cycle 3 eligibility:**
- A3 must earn with ≥1 defend mark in argument (lead/follow = mandatory collective interaction)
- A3 must earn with a collision-vote confirmation (B5↔A7 OP or similar)
- Combined with WEALD #27 (game #1): 2 different mechanisms across 2 different games → v2.18 trigger fires

## 9. Collision Adjacency Chart

| Axes | Adjacency type | Reasoning |
|---|---|---|
| **B5 ↔ A7** | temporal-register (confirmed; 6th OP) | B5 = trick-as-action-selection mechanism structure (timeless); A7 = deal variance (per-game). Pattern confirmed from Codenames #25 (B5↔A7 8-0 unanimous). |
| **A3 ↔ A1** | co-firing (candidate) | Mandatory interaction (A3) is the reason trick-taking has elegant rule-count-to-depth: the follow-suit rule creates the depth from one simple constraint. |
| **C1 ↔ B5** | co-firing (candidate) | Tension budget (C1) is heightened by the trick-as-action-selection structure (B5): you can't just play conservatively — the suit you lead determines what's at stake. |

## 10. Open Questions at Design-Stage

1. **Season length at 3 players**: 12-card hands deal 36 of 48 cards; 12 removed. Does this create hand-composition imbalance (some suits may be over/underrepresented in any 3-player deal)?
2. **Flax direct VP vs. token model**: Flax tricks scoring 2 VP directly (instead of tokens) creates a different strategy than Grain/Barley/Hops. Does this create point-salad incommensurability (C6) — can C6 earn here?
3. **Field Strip adjacency calibration**: is +1 VP for same-crop adjacent tiles too weak vs. the trick-earning rate? If a player wins 6 Grain tricks per season and plants Grain every season, their Field Strip gives 3 VP per planted Grain (base 2 + +1 adjacency) — 12 VP from Field Strip. Is this competitive?
4. **2-player**: excluded from this design; standard trick-taking degrades badly at 2. FURROW is 3–4 players only.
