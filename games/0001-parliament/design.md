---
name: Parliament — Design (Rules)
slug: parliament-design
game: 0001-parliament
stage: design
version: 1.0.0
rubric_version: v2.0
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
anchor_persona: knizia
anchor_axis: minimum-score-shape
---

# Parliament — Rules

## 1. Overview & Object of the Game

You are a game designer with strong opinions. Over four rounds, you will argue with 2–3 other designers about what makes a hypothetical game good — planting stakes on the axes you drafted, challenging opponents' weak stakes, and colliding with them on shared concerns.

The winner is the designer whose stakes *earned their defense* **and** whose axes *shaped the rubric* for the next session. The scoring multiplier rewards balance between the two:

```
final_score = (raw_stake_points + collision_points)  ×  (1 + adopted_axes_count)
```

A designer who earns many stakes but adopts no axes scores only their raw total. A designer who adopts 3 axes but earns nothing scores zero. Only balance wins.

## 2. Components

- **24 Axis cards** (the Axis Pool deck). Each card: axis name, definition, anchor descriptors 0/5/10, band marker (A/B/C/D), and a blank sticker area for adoption/retirement markers.
- **8 Designer persona boards**: Knizia, Rosenberg, Feld, Lacerda, Chvátil, Kramer-Kiesling, Stegmaier, and one Guest seat.
- **32 Subject cards**: each names a fictional game premise and its rough weight (e.g., "A 3–5p trading game about medieval guilds. Mid-weight Euro.")
- **36 stake tokens** (9 per seat at 4p; 12 per seat at 3p).
- **8 collision markers.**
- **Rubric Ledger sheet** (pencil-marked between games to record adoption/retirement).
- **Turn-order card.**
- **Rulebook.**

## 3. Setup

1. Shuffle the Axis Pool deck. If the Rubric Ledger marks any axes `adopted`, those cards are placed face-up in the center (they are drafted first in round 0; see §4.0). If any are `retired`, they are set aside (see §4.1).
2. Deal 12 Axis cards from the Pool face-up in the center. These are the draft board for this game.
3. Shuffle the Subject deck; draw 4 Subject cards face-up beside the Pool. These are the 4 rounds of argument.
4. Each player chooses a Designer persona board (or is dealt one randomly) and takes the matching persona's **preferred_axes** reference card.
5. Each player takes 9 stake tokens (12 at 3p).
6. Determine first-player randomly.

## 4. Phase 1 — Draft (5–10 min)

### 4.0 Round 0 — Adopted axes (skipped in first-ever session)

If any axes are `adopted` in the Ledger, each player in turn order picks 0 or 1 adopted axis at no cost (up to one free adopted draft). Adopted axes count toward the player's 3-axis limit.

### 4.1 Round 1 — standard snake draft

Turn order: P1 → P2 → P3 → P4 → P4 → P3 → P2 → P1. Each player picks 1 axis from the face-up draft board.

If a player picks a **retired** axis (from the set-aside stack), they pay 2 draft slots — they skip their next draft.

### 4.2 Round 2 — continue snake (reverse)

Same order as Round 1, reversed: P1 → P2 → P3 → P4 → P4 → P3 → P2 → P1. Each player picks 1 axis.

### 4.3 Round 3 — clockwise

Turn order: P1 → P2 → P3 → P4. Each player picks 1 axis.

At end of Draft, each player has 3 axes (or 2 if they spent a retired-axis slot).

### 4.4 Reveal preferred_axes

Each player turns their persona's preferred_axes card face-up. Draft is **public at all times**; the preferred list is revealed at draft-end as a tactical signal — if you drafted an axis outside your persona's preferences, opponents know you're likely weaker there.

Any undrafted cards (4 at 4p, more at 3p) return to the Pool for future sessions.

## 5. Phase 2 — Argument (4 rounds, 8–10 min each)

Each round plays against one Subject card, revealed in order. Each round has five steps:

### 5.1 STAKE (simultaneous)

Each player selects one of their 3 drafted axes and privately (face-down behind a screen, or on a notepad) stakes 1, 2, or 3 tokens on that axis. They may also pass (stake 0) — but a round with a pass counts against the persona.

A single axis may be staked at most once per game. Staking on a previously-staked-but-defended axis is allowed only if that defense resulted in `earned` status; otherwise the axis is exhausted for this player.

### 5.2 REVEAL

All stake commitments are revealed simultaneously. Token counts are placed on the axis cards in front of each player.

### 5.3 COLLISION CHECK

Consult the Collision Adjacency Chart (see §9). For any pair of axes in this round's stakes that are adjacent:

- A **collision marker** is placed between the two players.
- Both players gain a defense credit on the collided axis.
- The stake tokens remain (collision does not cost tokens).
- The axis with the better score match to the Subject card earns the `collision-win` (+2 endgame points). The other takes a `single-moment refutation` (no endgame point change, but the moment is logged).

Collision match-to-subject is resolved by table vote: each non-colliding player votes for which axis fits the Subject better. Ties favor the earlier-drafted axis.

### 5.4 ATTACK

In turn order (starting with the current round's first-player, rotating each round), each player may:

- **HOLD** — no action. Their staked axis remains as placed.
- **CHALLENGE** another player's stake. Pay 1 stake token from reserve (not from the staked axis) to the central pool.

If challenged, the defender must:
- **DEFEND** — pay 2 stake tokens from their reserve to the central pool. The stake survives with full tokens plus a "defended" mark. If the axis accumulates 2 `defended` marks over the game, it becomes `earned`.
- **CONCEDE** — half of the staked tokens (rounded up) are discarded. Gain 1 `refuted` mark on the axis. If the axis accumulates 2 `refuted` marks over the game, it becomes `refuted`.

A player may challenge at most once per round. A player may be challenged multiple times per round (each from a different attacker).

### 5.5 END OF ROUND

- Collision markers stay on the board for endgame scoring.
- Defend/concede marks stay on the axis cards.
- All token reserves remaining with players carry into the next round.

Proceed to the next Subject card.

## 6. Phase 3 — Amendment & Scoring (5 min)

### 6.1 Classify each axis per player

For each axis each player drafted:

- **Earned** — ≥ 2 `defended` marks and 0 `refuted`.
- **Contested** — has both `defended` and `refuted` marks.
- **Refuted** — ≥ 2 `refuted` marks.
- **Ignored** — 0 marks (neither challenged nor defended). Silent failure mode.

### 6.2 Compute raw points per player

- Each **earned** axis: 3 points + 1 per stake token still on it.
- Each **contested** axis: 1 point + 0.5 per stake token (round down).
- Each **refuted** axis: −1 point × number of refuted marks.
- Each **ignored** axis: 0 points AND 1 "silence penalty" mark on your persona (3+ cumulative across sessions → persona retirement candidate).
- Each **collision you participated in, collision-win side**: +2 points.
- Each **collision you participated in, refuted side**: 0 points (moment-refuted, no session-refutation).

### 6.3 Determine adopted / retired axes (session-wide, not per player)

- **Adopted (this session):** any axis marked `earned` by ≥ 2 players in this game. Axis advances to `adopted` status in the Rubric Ledger.
- **Retired (this session):** any axis marked `refuted` by ≥ 2 players. Axis advances to `retired` status.
- **Contested:** any axis with both earned and refuted states across players. Recorded as `contested` in the Ledger with full play record.

Most 4-player games produce 1–3 adopted axes and 0–1 retired.

### 6.4 Apply scoring multiplier

For each player:

```
final_score = (raw_points + collision_points)  ×  (1 + your_adopted_axes_count)
```

Your adopted-axis count = axes YOU drafted that became adopted this session.

A player who earns many stakes but drafted no adopted axes: final_score = raw_points × 1 = raw_points.
A player who drafts all adopted axes but earns none: final_score = 0 × anything = 0.
A player who earns 8 points and had 2 adopted axes: final_score = 8 × 3 = 24.
A player who earns 15 points and had 1 adopted axis: final_score = 15 × 2 = 30.

### 6.5 Winner

Highest final_score wins. Ties broken by (a) most adopted axes, (b) most collision wins, (c) most earned stakes.

### 6.6 Update the Rubric Ledger

After scoring, mark the Ledger sheet with:
- Each adopted axis (gold sticker).
- Each retired axis (black sticker).
- Each contested axis (pencil note of earned/refuted counts).

This Ledger persists. Next session's Pool uses the updated state.

## 7. Turn structure at a glance

```
SETUP
  ├── Shuffle Pool (respect prior Ledger)
  ├── Deal 12-card draft board
  ├── Draw 4 Subject cards
  └── 9 tokens per player

DRAFT (5-10 min)
  ├── Round 0: adopted axes (if any)
  ├── Round 1: snake draft (P1→P4→P1)
  ├── Round 2: reverse snake
  └── Round 3: clockwise

ARGUMENT × 4 rounds (8-10 min each)
  ├── STAKE (simultaneous)
  ├── REVEAL
  ├── COLLISION CHECK
  ├── ATTACK (turn order)
  └── END ROUND

AMENDMENT (5 min)
  ├── Classify axes
  ├── Compute raw points
  ├── Determine session-adopted / retired
  ├── Apply ×(1+adopted) multiplier
  ├── Declare winner
  └── Update Ledger
```

## 8. Edge cases & clarifications

- **Two players challenge the same defender in one round.** Resolved sequentially by turn order. Each challenge costs the attacker 1 token and requires a separate defense.
- **A player has no tokens to defend.** They must concede (auto-refuted).
- **The same axis is attacked twice in one round (from different rounds' stakes).** An axis may only accumulate one defended-mark or one refuted-mark per round, regardless of challenge count.
- **A player stakes 0 (passes) in a round.** The round produces no stake for them. After 2 passes in a game, the player loses 1 point per subsequent pass.
- **Subject cards run out in first-play session.** Deal 4 more; ignore any "requires prior adoption" flavour text.
- **4-player draft board runs out of adjacency candidates.** Mid-draft, if no face-up axis is adjacent to any already-drafted axis, shuffle-replace 4 cards from the Pool into the draft board.
- **All players' stakes are non-adjacent in a round.** No collision. ATTACK proceeds normally.
- **A player's persona has preferred_axes overlapping another's top drafts.** The earlier drafter takes the axis; the later drafter picks from remaining preferred or falls back.

## 9. Collision Adjacency Chart

Pairs of axes that can collide (semantic proximity creates argument-space):

| Left | Right | Collision theme |
|---|---|---|
| Tension Budget (C1) | Anti-Catch-up Pressure (C5) | Both manage pressure across the arc |
| Tension Budget (C1) | Scarcity Bite (C3) | Both about resource-under-want |
| Elegance (A1) | Action-Menu Clarity (C7) | Both about tight rule sets |
| Decision Density (A2) | Point-Salad Incommensurability (C6) | Both about per-turn richness |
| Interaction (A3) | Spatial-Interaction Presence (D2) | Direct board-coupling |
| Emergence / Replayability (A7) | Architectural Novelty (B5) | Strategy-space diversity |
| System Gearing (B1) | Engine-Garden Dependency (C4) | Subsystem interdependence |
| Information Transparency Cost (B4) | Teachability (A6) | Inverse — high cost = low teach |
| Conversion Chain Depth (B3) | Minimum-Score Shape (C2) | Both about scoring non-linearity |
| Variance Calibration (A4) | Catastrophe Pressure (B2) | Reversal events management |
| Late-Game Lock-in Point (D4) | Anti-Catch-up Pressure (C5) | End-game structure |
| Count-Robustness (D3) | Family-to-Expert Scaling (D1) | Access across audiences |
| First-Turn Compression (C8) | Teachability (A6) | Onboarding mechanics |
| Downtime / Pacing (A5) | Action-Menu Clarity (C7) | Flow management |

(14 adjacencies across 24 axes. A first play typically surfaces 3–6 collisions.)

## 10. Variants

- **Tournament variant (90 min, 4 rounds → 5 rounds):** add one Subject card and one extra draft round. Raise stake token count to 12 per seat.
- **Solo research variant (asynchronous, 2 hours):** no Attack phase. Player seats multiple personas serially; stakes are auto-evaluated against a pre-prepared Subject commentary. Used for TIGRIS spec-reviews where no argument partner is available. This is what `/tigris-panel spec-review` mode does.
- **Legacy campaign (5+ sessions):** track adoption/retirement across sessions. A persona whose drafts consistently land on `adopted` axes earns a Legacy Star; 3 stars and the persona is "canonized" (+1 starting token for future sessions in this campaign).

## 11. Designer notes

Parliament is built on three structural claims:

1. **The scoring multiplier (earned × adopted) is the Knizia-shape anchor.** Lose balance, lose the game. This is T&E's four-aspect minimum-score in new dress.
2. **Collision-via-adjacency is the architectural novelty claim.** No prior game I know forces players to anticipate semantic proximity between scoring axes. Chvátil's vote on whether this is a real novel move will decide the anchor review.
3. **Legacy at axis level is Rob Daviau scaled down.** Adopted/retired stickers are persistent but reversible. The game is legacy-lite, not legacy.

The game is designed to *be reviewed by TIGRIS* — its mechanics test each band of the rubric. Band A: Elegance, Decision Density, Interaction, Variance, Pacing, Teachability, Emergence. Band B: Architectural Novelty, System Gearing (the collision chart is a gearing chart), Information Transparency Cost (preferred_axes are revealed late; staking is hidden). Band C: Minimum-Score Shape, Tension Budget, Point-Salad Incommensurability. Band D: Count-Robustness, First-Turn Compression.

If Parliament passes its own TIGRIS review, the rubric has tested itself on an artifact specifically built to test it. That is the anchor's claim.

## 12. Open at DESIGN-stage, to resolve post-review

- The adjacency chart may be wrong. First play will reveal which pairs don't activate and which should have been listed.
- Silence penalty (ignored stake) may be too soft. A persona with 4/12 ignored stakes in one session should feel a sharper penalty.
- Adopted-axis multiplier of `1 + count` may be too shallow at 4p; `1 + 1.5×count` is a tournament variant candidate.
- Collision-win tie-break via table vote is a social mechanism that may degrade with player count imbalance (3p vs 4p). Worth examining.
