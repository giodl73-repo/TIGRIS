---
name: Tigris & Euphrates — Tier-A Stakes
slug: te-tierA-stakes
game: 0002-tigris-and-euphrates
stage: tierA
version: 1.0.0
rubric_version: v2.1
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
anchor_persona: knizia
anchor_axis: minimum-score-shape
draft_order_round_1: [stegmaier, kramer-kiesling, rosenberg, feld, lacerda, chvatil, knizia]
draft_order_round_2: [knizia, chvatil, lacerda, feld, rosenberg, kramer-kiesling, stegmaier]
draft_order_round_3: [stegmaier, kramer-kiesling, rosenberg, feld, lacerda, chvatil, knizia]
---

# Tier-A Stakes — Tigris & Euphrates

Same 7-persona panel as Parliament. Anchor: Knizia on C2 Minimum-Score Shape (T&E is the canonical instance). Snake order places Knizia last in round 1 per anchor-persona protocol.

Persona preferences are fixed; drafts resolve identically to Parliament because no snipes forced fallbacks. This is a known property of stable preference rosters — two games in close succession with the same seated personas produce identical drafts.

## Draft record

| Round | Order | Persona | Axis | Band |
|---|---|---|---|---|
| 1 | 1 | Stegmaier | count-robustness | D3 |
| 1 | 2 | Kramer-Kiesling | action-menu-clarity | C7 |
| 1 | 3 | Rosenberg | scarcity-bite | C3 |
| 1 | 4 | Feld | point-salad-incommensurability | C6 |
| 1 | 5 | Lacerda | system-gearing | B1 |
| 1 | 6 | Chvátil | architectural-novelty | B5 |
| 1 | 7 | **Knizia (anchor)** | tension-budget | C1 |
| 2 | 1 | Knizia | **minimum-score-shape** (anchor axis) | C2 |
| 2 | 2 | Chvátil | interaction | A3 |
| 2 | 3 | Lacerda | information-transparency-cost | B4 |
| 2 | 4 | Feld | anti-catch-up-pressure (*queued-for-retirement from Parliament; staking here is a claim that T&E vindicates the axis*) | C5 |
| 2 | 5 | Rosenberg | engine-garden-dependency (*also queued-for-retirement; staking here is a similar claim*) | C4 |
| 2 | 6 | Kramer-Kiesling | family-to-expert-scaling | D1 |
| 2 | 7 | Stegmaier | first-turn-compression | C8 |
| 3 | 1 | Stegmaier | teachability | A6 |
| 3 | 2 | Kramer-Kiesling | spatial-interaction-presence | D2 |
| 3 | 3 | Rosenberg | late-game-lock-in-point | D4 |
| 3 | 4 | Feld | decision-density | A2 |
| 3 | 5 | Lacerda | emergence-replayability | A7 |
| 3 | 6 | Chvátil | catastrophe-pressure | B2 |
| 3 | 7 | Knizia | elegance | A1 |

Band coverage: **A(5) · B(4) · C(8) · D(3)**. 4/4 bands present.

Undrafted: A4 variance-calibration, A5 downtime-pacing, B3 conversion-chain-depth. Same as Parliament — the anchor adjacency problem (I-parliament-03) recurs: Knizia's Minimum-Score Shape again lacks its Conversion-Chain-Depth collision partner. Without adoption of I-parliament-03 (still `candidate`), GATE doesn't block this; it just gets logged as a second instance of the same pattern. Cluster evidence for I-parliament-03 triggers after this game.

## Stakes

### Knizia (anchor; T&E is his game)

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| C1 Tension Budget | 10 | 10 | 4 leaders × 4 colors; only one leader per kingdom per color. Every placement starves another. §4 turn economy: 2 actions forces tradeoff between placement, conflict, and tile-cycling. The definition of tension budget. |
| **C2 Minimum-Score Shape** (anchor) | 10 | 10 | §10 final scoring: VP = MIN across 4 colors. This is the canonical instance of this axis — the axis was named from T&E. A score below 10 would be self-refuting. |
| A1 Elegance | 9 | 9 | 30-min teach produces ~20-turn games with tournament-level depth across decades. 153 tiles, 4 colors, 4 conflict types, 2 actions/turn — everything pays rent. Not 10 because the conflict-resolution rules (§5) require careful rereading on first play. |

### Rosenberg

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| C3 Scarcity Bite | 6 | 7 | Tile bag is finite (153); hand cap is 6. Leaders-per-kingdom cap creates placement scarcity. But no explicit feeding phase — nobody starves. 4p tightens bag draw. |
| C4 Engine-Garden Dependency | 6 | 6 | Leaders → tiles → kingdoms → monuments → passive VP. That's an engine chain. But unlike Le Havre, the pieces don't *gear conditionally* — monuments score passively once built. Looser dependency than a true engine-garden. Staking here claims that T&E has more engine-garden than Parliament did; the axis may earn even though it's queued-for-retirement from Parliament. |
| D4 Late-Game Lock-in Point | 7 | 7 | Lock occurs around turn 15 of ~20 — when last 2 treasures claim-triggers the end condition per §9. Players commit to final kingdom shapes ~2/3 through the arc. Right on the Rosenberg-canonical lock point. |

### Feld

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| C6 Point-Salad Incommensurability | 4 | 4 | T&E has ONE VP metric (MIN across 4 colors). The 4 colors are structurally the same thing — they're not incommensurable tracks, they're partitions of the same scoring surface. This is a low-scoring stake deliberately; T&E is the counter-example to point-salad. |
| C5 Anti-Catch-up Pressure | 8 | 8 | Catastrophes (§6) target leaders who are doing well; leading players draw aggression. External wars (§5) redistribute tiles from winners to losers in the conflict resolution. The mechanic LITERALLY names itself "catastrophe." Staking here to claim T&E vindicates this axis even though Parliament refuted it. |
| A2 Decision Density | 8 | 9 | 2 actions × 5+ action types × ~20 turns × 4 players = lots of live decisions. Every turn you pick among place-tile, place-leader, catastrophe, trade. No dead turns possible per §4. Higher at 4p due to interaction density. |

### Lacerda

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| B1 System Gearing | 9 | 9 | Remove any subsystem — catastrophes (§6), monuments (§7), treasures (§8) — and T&E becomes fundamentally different. Not additive: the subsystems multiply. Catastrophes *activate* monument-hunting *because* monument-destruction is possible *because* of catastrophe-breaking kingdoms. The Lisboa test passes. |
| B4 Information Transparency Cost | 8 | 8 | Opponents' 6-tile hands are hidden (§3). Treasure values are revealed on claim. Catastrophe timing is a guessing game. You pay for knowledge through scouting actions and pattern recognition. Strong information asymmetry that drives strategy. |
| A7 Emergence/Replayability | 9 | 10 | Every game's board configuration is unique. Tournament players still find new openings after 100+ plays. 29 years after publication, T&E is still a tournament staple — emergent space is effectively unbounded. |

### Chvátil

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| B5 Architectural Novelty | 7 | 7 | In 1997, min-score scoring was genuinely novel; now it's imitated (Samurai, Ingenious, Taj Mahal). T&E still does it cleanest. 7 not 10 because the rubric judges novelty-today, not novelty-at-publication — and the move has been borrowed often enough to be part of Euro standard vocabulary. |
| A3 Interaction | 9 | 10 | Conflicts (§5) are direct and frequent. Kingdom adjacency forces spatial negotiation. Catastrophe targeting is pure aggression. Every turn someone's options change because someone else's action reshaped the map. Multiplayer solitaire is structurally impossible in T&E. |
| B2 Catastrophe Pressure | 10 | 10 | 9 catastrophe tiles per game; they are the game's most famous mechanic. The axis "Catastrophe Pressure" was NAMED partly in honor of T&E. A score below 10 would invalidate the axis definition. Same absolute-score confidence as Knizia on minimum-score. |

### Kramer-Kiesling

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| C7 Action-Menu Clarity | 7 | 7 | §4 turn: 5 action types. Just above the 6-action ceiling for family clarity. Iconography is moderate — leader types are distinct, tile colors are distinct, but conflict resolution requires rulebook lookup for first-play teachers. |
| D1 Family-to-Expert Scaling | 7 | 7 | Teachable in 30 min; tournament-level depth well-established. 29 years of tournament play testifies to the scaling. Not 10 because the teach is above the 15-min Azul threshold. |
| D2 Spatial-Interaction Presence | 9 | 9 | The board IS the game. Kingdom adjacency, leader placement, catastrophe targeting, treasure routes — all spatial. Direct board-state coupling per §5 conflict rules: your placement changes my kingdom. |

### Stegmaier

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| D3 Count-Robustness | 7 | 8 | 3p and 4p both play well; 2p is explicitly supported but weaker (most players agree 4p is optimal, 3p is strong, 2p is a variant). 2-point gap between weakest and strongest counts. |
| C8 First-Turn Compression | 4 | 4 | Turn 1 in T&E is often "place a leader near a starting temple" — meaningful but not dramatic. A new player isn't making a strategic choice on turn 1 that differs materially from any other reasonable opener. Below Scythe-anchor. |
| A6 Teachability | 5 | 5 | 30–45 min teach for a 90-min game. The conflict rules (§5) require walking-through with examples. Below the family-tier threshold; acceptable for hobbyist-tier. |

## Stakes not drafted (undrafted axes)

Three axes returned to the Pool: A4 Variance Calibration, A5 Downtime-Pacing, B3 Conversion Chain Depth. Notable absence: T&E's tile-bag mechanic is a prime example of Variance Calibration, but no persona drafted A4. This is a *systematic undraft* — A4 went undrafted in Parliament too. Amendment candidate: A4 should be promoted (via draft-preference update) or retired (if no persona wants to advocate it). Queued as a future I-te-NN candidate.

## GATE check (v2.1)

1. **Anchor stake viable** — Knizia's C2 Minimum-Score Shape was not pre-staked by any other persona. His stake of 10/10 is the strongest stake on record. Anchor viable. ✓
2. **Draft coverage** — 4/4 bands. ✓
3. **≥ 1 collision candidate** — 9 active adjacency pairs (same as Parliament, identical drafts). ✓

**GATE: PASS.**

Anchor adjacency observation: Knizia's C2 again lacks its B3 collision partner. This is the 2nd instance of I-parliament-03's `anchor_axis_without_collision_partner` trigger pattern. Per innovation-log clustering rules (≥ 2 instances of same trigger pattern → propose amendment), I-parliament-03 should cluster after this game. Since I-parliament-01 already consumed this cycle's rate-limit, I-parliament-03 cannot adopt immediately but becomes a stronger candidate for the next cycle.

Proceeding to TIER-B ARGUMENT with v2.1 FORCED-ENGAGEMENT.
