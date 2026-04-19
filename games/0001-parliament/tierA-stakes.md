---
name: Parliament — Tier-A Stakes
slug: parliament-tierA-stakes
game: 0001-parliament
stage: tierA
version: 1.0.0
rubric_version: v2.0
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

# Tier-A Stakes — Parliament (0001)

Seven designer personas seated. No player lenses (keeping the anchor panel focused; lenses join on game #2+).

Snake draft order in round 1 puts Knizia last per the anchor-persona protocol — he must accept what the others leave. Rounds 2 and 3 rotate; Knizia picks first in round 2.

## Draft record

| Round | Order | Persona | Axis | Pool-band |
|---|---|---|---|---|
| 1 | 1 | Stegmaier | count-robustness | D3 |
| 1 | 2 | Kramer-Kiesling | action-menu-clarity | C7 |
| 1 | 3 | Rosenberg | scarcity-bite | C3 |
| 1 | 4 | Feld | point-salad-incommensurability | C6 |
| 1 | 5 | Lacerda | system-gearing | B1 |
| 1 | 6 | Chvátil | architectural-novelty | B5 |
| 1 | 7 | **Knizia** (anchor) | tension-budget | C1 |
| 2 | 1 | Knizia | minimum-score-shape | C2 — **anchor axis** |
| 2 | 2 | Chvátil | interaction | A3 |
| 2 | 3 | Lacerda | information-transparency-cost | B4 |
| 2 | 4 | Feld | anti-catch-up-pressure | C5 |
| 2 | 5 | Rosenberg | engine-garden-dependency | C4 |
| 2 | 6 | Kramer-Kiesling | family-to-expert-scaling | D1 |
| 2 | 7 | Stegmaier | first-turn-compression | C8 |
| 3 | 1 | Stegmaier | teachability | A6 |
| 3 | 2 | Kramer-Kiesling | spatial-interaction-presence | D2 |
| 3 | 3 | Rosenberg | late-game-lock-in-point | D4 |
| 3 | 4 | Feld | decision-density | A2 |
| 3 | 5 | Lacerda | emergence-replayability (fallback — D4 sniped) | A7 |
| 3 | 6 | Chvátil | catastrophe-pressure | B2 |
| 3 | 7 | Knizia | elegance | A1 |

21 drafts. Band coverage: **A (5) · B (4) · C (8) · D (3)**. All 4 bands present.

Undrafted this game: variance-calibration (A4), downtime-pacing (A5), conversion-chain-depth (B3). Notably, Knizia passed on conversion-chain-depth despite having it in his preferred list — he took elegance instead. This leaves minimum-score-shape without its natural collision partner (B3↔C2 is an adjacency), which will matter in Argument.

## Stakes

Each persona stakes each drafted axis at 3p and 4p. Score 0–10 with a one-sentence justification citing a specific Parliament rule or mechanic.

### Knizia (anchor persona)

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| C1 Tension Budget | 7 | 8 | Token economy (9 tokens/player at 4p; 12 at 3p) enforces scarcity — every challenge costs 1, every defense 2; running out forces auto-concede per §5.4 edge case. |
| **C2 Minimum-Score Shape** (anchor) | 9 | 10 | The scoring multiplier `(raw + collision) × (1 + adopted_axes)` in §6.4 is a minimum-score shape by construction: specializing in raw stakes with 0 adoptions scores raw × 1 = nothing clinched; specializing in adoptions with 0 earnings scores 0. Only balance wins — exactly the Ingenious pattern. |
| A1 Elegance | 7 | 7 | Three phases, 24-axis pool, 14-pair adjacency chart — rule count matches strategic depth with little slack. Adjacency chart is one piece of overhead I'd audit after first play. |

### Rosenberg

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| C3 Scarcity Bite | 6 | 8 | At 4p with 9 tokens across 4 rounds and challenges at cost 1, scarcity bites sharply — a player who challenges in rounds 1–2 risks token-poverty by round 4. At 3p with 12 tokens, scarcity loosens. |
| C4 Engine-Garden Dependency | 4 | 4 | Parliament has no conditional-value system between drafted axes. The multiplier at §6.4 is compositional (adopted × earned) but flat; buildings-gear-on-other-buildings is absent. |
| D4 Late-Game Lock-in Point | 6 | 7 | Strategic commitment locks around round 3 of 4 (⅔ through) once a player has staked on ≥2 of their 3 axes; this matches the canonical lock point for Euros. At 4p the lock is clearer because 4 rounds of rotated-first-attacker more visibly segment the arc. |

### Feld

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| C6 Point-Salad Incommensurability | 8 | 8 | §6.2 generates scores from four non-substitutable sources: earned axes (3 + tokens), collision wins (2 each), refuted penalties (−1 each), adopted-axis multiplier (×1+N). None convert to another; the salad is genuinely incommensurable. |
| C5 Anti-Catch-up Pressure | 3 | 3 | No plague-phase, no downside event, no catch-up mechanism. A leading player after round 2 is not systematically punished in rounds 3–4. This is a concrete v2.1 amendment candidate. |
| A2 Decision Density | 7 | 8 | Each round produces 3 live decisions per player (which axis to stake, how many tokens, whether to challenge/defend); 4 rounds × 3 decisions × 4 players = 48 decisions at 4p. No dead turns per §5. |

### Lacerda

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| B1 System Gearing | 5 | 5 | Draft → Argument → Amendment is a sequential chain; Draft outcomes constrain Argument choices, Argument outcomes dictate Amendment deterministically. But the phases don't multiply — removing Argument collapses the whole game; removing Amendment just removes the endgame; removing Draft makes the game unplayable. That's linear dependency, not multiplicative gearing. |
| B4 Information Transparency Cost | 7 | 7 | §4.4 reveals preferred_axes AFTER draft, creating early-game information asymmetry that's resolved only when all 3 drafts are public. The cost of predicting opponents' drafts is real; §5.1 hidden stake commitment extends this into each round. |
| A7 Emergence/Replayability | 7 | 8 | Adoption/retirement legacy (§6.3, §6.6) means the Pool literally changes between sessions; by session 3–4, no two play groups have the same Pool. 4p has more adoption combinations than 3p (~250% more draft-pairings). Emergent depth is structural. |

### Chvátil

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| B5 Architectural Novelty | 7 | 7 | Drafting scoring axes and then playing to defend them is an unfamiliar architecture. I cannot name a prior tabletop game where player scoring categories are drafted exclusively. That's a genuine bet, not a port. Minus two points because the STAKES → ARGUMENT flow is still tabletop-Euro conventional within each phase. |
| A3 Interaction | 8 | 9 | Attack/defend rotations per §5.4, collision mechanic per §5.3, challenge-as-auction economy — multiplayer solitaire is structurally impossible in Parliament. 4p rotations are richest because the attacker's position rotates by round, distributing offensive pressure. |
| B2 Catastrophe Pressure | 4 | 4 | Challenges are local (one axis per round per attacker), not catastrophic (no reversal events that cascade through multiple axes). A player losing all their reserve tokens in one bad round is the closest to catastrophe; but the game shape doesn't produce that reliably. |

### Kramer-Kiesling

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| C7 Action-Menu Clarity | 8 | 8 | Per §5.4: HOLD / CHALLENGE / DEFEND / CONCEDE — four clear actions, all with stated costs. §5.1 STAKE and §5.2 REVEAL are similarly tight. Action-menu size is 4, well within tournament-target working memory. |
| D1 Family-to-Expert Scaling | 6 | 6 | Rules teach in 15 min (target); adjacency chart pushes complexity into expert territory. A family can play Parliament without mastering adjacencies (only collision scoring is affected), making it playable at mixed-skill tables — but the strategic depth requires adjacency fluency. |
| D2 Spatial-Interaction Presence | 4 | 4 | Parliament has no board. "Adjacency" in §9 is conceptual/semantic, not spatial. Stake tokens sit on axis cards; axis cards are not adjacent to each other on any physical surface. Spatial play is substantially absent. |

### Stegmaier

| Axis | 3p | 4p | Justification |
|---|---:|---:|---|
| D3 Count-Robustness | 6 | 8 | 3p variant exists (12 tokens, 8 draft picks remaining vs 4p's 4 — more axes return to Pool, less contention). 4p is the design-target count. Gap between 3p experience quality and 4p experience quality is ~2 points; too wide for a claim of robust scaling. |
| C8 First-Turn Compression | 6 | 6 | The first draft pick in Round 1 is immediately meaningful — the axis is exclusively yours — but a new player cannot evaluate axis strength without prior-session context (no Ledger at first play). That dilutes the compression. |
| A6 Teachability | 5 | 5 | 15-min teach is aspirational. The multiplier in §6.4, the adjacency chart in §9, and the stake-state classification in §6.1 each add rule-mass; a first-play teach is 25+ min in practice. Below the Azul line. |

## Collision candidates (identified from drafted axes)

Per `design.md` §9, these adjacency pairs are both present:

1. C1 Tension Budget (Knizia) ↔ C5 Anti-Catch-up Pressure (Feld)
2. C1 Tension Budget (Knizia) ↔ C3 Scarcity Bite (Rosenberg)
3. A1 Elegance (Knizia) ↔ C7 Action-Menu Clarity (Kramer-Kiesling)
4. A3 Interaction (Chvátil) ↔ D2 Spatial-Interaction Presence (Kramer-Kiesling)
5. A7 Emergence/Replayability (Lacerda) ↔ B5 Architectural Novelty (Chvátil)
6. B1 System Gearing (Lacerda) ↔ C4 Engine-Garden Dependency (Rosenberg)
7. B4 Information Transparency Cost (Lacerda) ↔ A6 Teachability (Stegmaier)
8. D4 Late-Game Lock-in Point (Rosenberg) ↔ C5 Anti-Catch-up Pressure (Feld)
9. D3 Count-Robustness (Stegmaier) ↔ D1 Family-to-Expert Scaling (Kramer-Kiesling)
10. C8 First-Turn Compression (Stegmaier) ↔ A6 Teachability (Stegmaier — same player, no collision)

**Active collision pairs: 9.** (Item 10 is same-player, excluded.)

Notably absent: **C2 Minimum-Score Shape (Knizia's anchor) has no collision partner** because B3 Conversion Chain Depth went undrafted. Knizia's anchor stake will have to defend itself without an adjacent supporter. This is a Knizia-shaped tension — the anchor persona is alone on their anchor axis. He chose to draft elegance in round 3 instead of conversion-chain-depth, spending his slot to shore up a Band A axis that collides with K-K. Classic Knizia misdirection.

## GATE check (spec §8.4)

1. **Anchor stake viable** — Knizia's anchor axis C2 Minimum-Score Shape was not pre-staked by any other persona. No re-declaration needed. ✓
2. **Draft coverage** — 4/4 bands present (A: 5, B: 4, C: 8, D: 3). Requirement ≥ 3 bands. ✓
3. **≥ 1 collision candidate** — 9 active pairs. ✓

**GATE: PASS. Proceeding to TIER-B ARGUMENT.**
