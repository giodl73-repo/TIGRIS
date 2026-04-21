---
name: GARNER — PT01 Argument
slug: garner-PT01
game: 0023-garner
stage: tierB
version: 1.0.0
rubric_version: v2.20
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
player_count: 3
contracts_active: [C02, C03, C07, C11, C15]
---

# PT01 — GARNER: Argument Record

3-player game. Moment-anchors: Setup, Round 1 (Autumn), Round 2 (Winter), Round 3 (Spring), FORCED-ENGAGEMENT, Round 4 (Summer — final), Scoring.

**Active contracts this game:**
- C02: 2 VP/Flour; −1 VP/raw Grain in Warehouse
- C03: 5 VP/Flour+Malt pair
- C07: 2 VP/Barley; +3 VP bonus to the player with the most Barley
- C11: 3 VP/Flax; −1 VP/empty space
- C15: 5 VP if you hold zero Chaff tokens; 0 VP otherwise

This is a genuine C6 configuration: C02 and C03 reward Grain processing (Flour); C07 rewards raw Barley (NOT converted to Malt for C03). A player cannot simultaneously maximize Flour output (C02/C03) and Barley-majority (C07). C11 rewards grid density; C15 punishes any Chaff accumulation.

---

## Setup

**Starting conditions**: Each player takes 2 commodity tiles from the bag.
- Player 1 (P1): 1 Grain + 1 Barley
- Player 2 (P2): 1 Grain + 1 Flax
- Player 3 (P3): 2 Barley

Market stalls at game start: Stall I (3 Grain), Stall II (2 Barley + 1 Hops), Stall III (3 Flax), Stall IV (1 Grain + 1 Barley + 1 Hops).

---

**C6 (Feld anchor) — Defend**: "Contracts C02 and C07 are simultaneously active. C02 requires converting Grain to Flour (+2 VP/Flour, −1 VP/Grain). C07 requires accumulating raw Barley (+2 VP/Barley, majority bonus). A player cannot do both: converting Barley to Malt for C03 loses C07 Barley count; keeping Barley raw loses C03 Flour+Malt pairs. The incommensurability is visible before Turn 1. +1 defend on C6."

**A7 (Lacerda) — Hold**: "C6 fires on the within-game contract configuration. A7's counter-pressure claim requires demonstrating that DIFFERENT contract selections produce different strategic landscapes across games. Will defend in Spring (Round 3) when warehouse divergence is visible."

**B4 (Lacerda) — RETIRE-EXPLICIT**: "All 5 contracts are fully visible from setup. Zero information cost. The bag randomness is variance, not hidden information. B4 contributes nothing to the argument here. Retire-explicit. +0.5r."

**A6 (Stegmaier) — Defend**: "5 contracts face-up; 6-action menu; 4 phases. First-play teach sequence: read contracts together (establishes goals) → explain draft (1 pick per round) → explain placement (fill the grid) → explain conversion (2+2=1) → score contracts. Under 15 minutes. All scoring criteria known from the start. +1 defend on A6."

---

## Round 1 — Autumn

**Draft phase**: P1 takes Stall I (3 Grain; supports C02 Flour processing). P2 takes Stall II (2 Barley + 1 Hops; supports C07 Barley majority). P3 takes Stall III (3 Flax; supports C11 density). Stall IV (1 Grain + 1 Barley + 1 Hops) carries over.

**Warehousing (P1)**: 3 Grain + 1 Grain (starting) = 4 Grain in warehouse. Placed as 2×2 Grain cluster in top-left.
**Warehousing (P2)**: 2 Barley + 1 Hops + 1 Grain (starting) + 1 Flax (starting). Scattered placement.
**Warehousing (P3)**: 3 Flax + 2 Barley (starting). Dense Flax placement in rows 1-2, starting Barley in row 3.

**Conversion (P1)**: Converts 2 Grain in same row → 1 Flour. Warehouse now has 2 Grain + 1 Flour (adjacent).
**Conversion (P2)**: Passes — keeps Barley raw for C07.
**Conversion (P3)**: Passes — no conversion eligible yet.

---

**A2 (Feld) — Defend**: "P1's Round 1 decision: take Stall I (3 Grain) to build toward C02/C03, or take Stall II (2 Barley + Hops) to compete with P2 on C07. Choosing Stall I forfeits the Barley-majority race; choosing Stall II abandons the processing strategy. Contract-aware placement multiplies the per-turn decision tree. +1 defend on A2."

**C3 (Rosenberg) — Defend**: "Stall IV carries into Round 2 with 1 Grain + 1 Barley + 1 Hops. P3 explicitly passed it to avoid Hops (not favored by any active contract) and accepted a smaller placement. Scarcity of 'optimal stall composition' is real even in Round 1 — the best stall isn't always the most tile-dense stall. +1 defend on C3."

**C1 (Knizia) — Defend**: "End of Round 1: P1 warehouse has 3 tiles + 1 Flour. P1 knows that 12 rounds remain (3 more rounds × ~4 tiles per round) and a 16-space Warehouse. The fill-rate math creates a tension budget: can I fill the grid without forcing Chaff? C15 (no Chaff) means the budget is explicit. +1 defend on C1."

---

## Round 2 — Winter

**Market restock**: Stall IV (1 Grain + 1 Barley + 1 Hops) gets +3 tiles from bag: 2 Barley + 1 Grain. Stall IV now holds 5 tiles — most dense stall. Stalls I, II, III each restock with 3 new tiles.

**Draft phase**: P3 takes Stall IV (5 tiles: 2 Barley + 1 Grain + 1 Barley + 1 Grain = 2 Grain + 3 Barley). P1 takes newly restocked Stall I (3 Grain). P2 takes Stall II (2 Flax + 1 Barley). Stall III carries.

**Warehousing**: P1 now has 8 tiles placed (5 Grain + 1 Flour + 2 new Grain). Critical decision: place all Grain in rows for row-conversion, or spread for C11 column coverage? P1 commits to row-dense Grain in top half.

P3 now has 3 Barley from Stall IV + 2 starting Barley = 5 Barley total. Leads C07 majority race. Holds all raw.

**C6 (Feld anchor) — Defend (2nd)**: "Round 2 makes the incommensurability concrete at the table: P1 committed to Grain-row processing (supports C02/C03) and now CANNOT compete for C07 majority — P3 has 5 raw Barley, P1 has 0. The two players are scoring against non-overlapping subsets of the 5 active contracts. The C6 architecture is the cause: the contract deck forced this divergence. +1 defend on C6."

**D4 (Rosenberg) — Defend**: "P2's Round 2 warehouse: Flax+Barley mix locked in 6 spaces. These placements are permanent. By Winter, P2 cannot pivot to a pure-Barley strategy — the Flax tiles are already committed. The lock-in moment is demonstrably arrived. +1 defend on D4."

**B2 (Chvátil) — Defend**: "P3 now has 8 tiles in a 16-space Warehouse with 2 rounds remaining. At the current draft rate (~4 tiles/round), P3 will collect ~8 more tiles in Rounds 3-4. That's 16 tiles for 8 remaining spaces — guaranteed overflow. P3 must convert or Chaff some. C15 (zero Chaff = 5 VP) means any forced Chaff costs at minimum 5 VP. The catastrophe is building. +1 defend on B2."

---

## Round 3 — Spring

**Market restock**: Stall III (carried over from Round 2, now 6 tiles: 3 Flax + 3 more tiles [2 Hops + 1 Flax] = 4 Flax + 2 Hops). Rich stall for C11 (Flax per tile).

**Draft phase**: P1 needs more Grain for Row 3 conversion. P3 needs Barley to maintain majority. P2 is caught between Flax (C11) and Barley (C07).
P3 takes Stall III (4 Flax + 2 Hops; unexpected — P3 shifts strategy toward C11 density). P1 takes Stall I. P2 takes Stall II. Stall IV carries.

**Warehousing**: P3's warehouse dramatically shifts: Flax now dominates (7 Flax tiles). P3 has abandoned pure-Barley for a C11 density strategy.

---

**A7 (Lacerda) — Defend**: "Round 3 warehouse states have diverged: P1's grid is Grain-heavy (Flour-focused, C02/C03 targeting); P3's grid is now Flax-heavy (C11 density targeting, having pivoted from Barley); P2's grid is mixed Flax+Barley. Three architecturally distinct warehouses produced by three players responding to the SAME 5 contracts from identical starting conditions. This is emergence by the definition of A7. +1 defend on A7."

**A3 (Chvátil) — Defend**: "Round 3 critical interaction: P3 drafted Stall III (Flax-rich) — the exact stall P2 needed for C11 strategy. P2's Round 3 was compromised by P3's pivot. This is indirect interaction via stall competition: one player's choice directly constrains another's options. At 3 players, this is consistent and meaningful. +1 defend on A3."

**B1 (Lacerda) — Defend**: "Round 3 demonstrates the full pipeline gear: Bag draw (random tile sequence affected which stalls had which tiles) → Draft (P3 took Stall III in a strategic surprise) → Placement (P3 committed Flax to grid rows for C11 alignment) → Conversion (P1 converts 2 Grain + 2 Grain in top rows → 2 Flour, now has 3 Flour adjacent to Malt positions). 4 gears in sequence, each amplifying. +1 defend on B1."

---

### C6↔A7 COLLISION — Spring round (proposed by Chvátil; temporal-register OP question)

**Chvátil (raising adjacency)**: "C6 (Feld anchor) and A7 (Lacerda) are both produced by the same mechanism: the variable contract deck. C6 says 'within THIS game, scoring paths are incommensurable.' A7 says 'across games, the experience is different.' Both are consequences of one elegant design decision (variable contracts). Is this genuinely two axes earning on one mechanism, or does C6 subsume A7?"

**Lacerda (A7) proposes OP**: "C6 measures WITHIN-GAME scoring incommensurability — the question 'can I maximize C02 and C07 simultaneously in this game?' C6 answers NO. A7 measures ACROSS-GAME replayability — the question 'will next game's contract selection create a different optimal strategy?' A7 answers YES. These are different temporal registers of the same mechanism: session-local (C6) vs. cross-session (A7). OP is correct."

**Feld (C6 anchor) seconds OP**: "The temporal-register distinction is real and teachable. In this game I can identify the incommensurability (C6) by reading the 5 active contracts. In next game, I cannot predict the incommensurability without knowing which 5 contracts are drawn (A7). Same mechanism, different questions at different time-scales."

**Panel vote on OP:**
- Knizia → OP: "C1 argument: tension budget is within-session. A7 as cross-session variety is genuinely different."
- Rosenberg → CR: "C4/C3 concern: both axes emerge from the same contract deck decision. One elegant mechanism earning two axes feels like C6 subsumes A7. CR: C6 wins the dominant claim."
- Chvátil → OP: "The adversarial-review ethos: the within-game/across-game distinction is real and independently testable."
- K-K → OP: "C7 clarity: 'these 5 contracts are incommensurable in THIS game' and 'different 5 contracts make a different game' are two teachable, distinct claims."
- Stegmaier → OP: "A6 perspective: I can teach the distinction — 'C6 is WHY you struggle this game; A7 is why the game is fresh next game.' Different questions, different student needs."
- Vaccarino → CR: "The variable contract deck is one mechanism. Splitting it into two axes risks credit-inflation from one elegant design decision."
- Lacerda → OP (proposer).

**Result: 5-2 for OP** (Rosenberg and Vaccarino dissent). Standard OP (≥5 of 8). Both C6 and A7 earn defend marks from the collision. This OP serves as the **collision-vote confirmation** required for v2.18 counter-pressure eligibility.

**v2.18 counter-pressure status after this collision:**
- GARNER satisfies: ≥1 counter-pressure original (documented in concept.md) + ≥1 collision-vote confirmation (this OP 5-2).
- Still required for full retirement-reversal: ≥1 MORE counter-pressure original (different game). GARNER is game #1 of 2.

---

## FORCED-ENGAGEMENT (between Round 3 and Round 4)

Stakes at 0 marks at this point: A4 (Vaccarino), B3 (Vaccarino), C4 (Rosenberg), D1 (K-K), D2 (K-K), C8 (Stegmaier), B5 (Chvátil), A1 (Knizia), C2 (Knizia), B6 (Feld), D3 (Stegmaier).

**Knizia (A1):** LAST-CALL. "GARNER's elegance rests on the contract deck doing all the work. Rule count is moderate but depth is earned. Testing at final round."
**Knizia (C2):** RETIRE-EXPLICIT. "Near-zero floor is not a designed minimum score shape. +0.5r on C2."
**Rosenberg (C4):** LAST-CALL. "The warehouse has become a garden — placement decisions in Rounds 1-3 created a specific architectural commitment. Testing at scoring."
**K-K (D1):** HOLD-EXPLICIT. "Scaling gradient is present but not exceptional. Hold."
**K-K (D2):** RETIRE-EXPLICIT. "Player grids are spatially isolated. No cross-player spatial pressure. +0.5r on D2."
**Stegmaier (D3):** LAST-CALL. "Simultaneous placement at 3 players is fast. Testing at final."
**Stegmaier (C8):** HOLD-EXPLICIT. "Setup is standard. No exceptional first-turn compression. Hold."
**Chvátil (B5):** RETIRE-EXPLICIT. "Concept disclaims novelty. +0.5r on B5."
**Vaccarino (A4):** LAST-CALL. "Managed variance via contract selection + bag draw. Testing at final."
**Vaccarino (B3):** HOLD-EXPLICIT. "Depth-2 chain is too shallow for B3 canon. Hold."
**Feld (B6):** LAST-CALL. "Majority-bonus contracts (C07, C09, C12) — at least C07 is active this game. Testing."

---

## Round 4 — Summer (Final)

**Draft phase**: P1 takes Stall I (restocked with Grain + Barley). P3 takes Stall II (Flax-rich for C11). P2 takes the carry-over stall (varied mix).

**Warehousing**: P1's warehouse fills to 14 tiles. 2 empty spaces remain. P3's warehouse fills to 15 tiles; 1 empty space (risk: C11 penalizes empty spaces at −1 VP each). P2 has 12 tiles; 4 empty spaces (C11 penalty: −4 VP).

**Conversion (P1)**: 2 more Grain in row 3 → 1 Flour (total 3 Flour). Also: 2 Barley in column 2 → 1 Malt. Now has 3 Flour + 1 Malt (supports C03: 1 Flour+Malt pair = 5 VP, but only 1 pair).
**Conversion (P3)**: Passes (no eligible conversions).

**A1 (Knizia — LAST-CALL)**: "GARNER's elegance at 4-round completion: 4 phases, 6 actions, variable scoring landscape. The rule-count is moderate but every rule is load-bearing. The contract deck generates the depth from minimal machinery. The rule-count/depth ratio is positive. +1 defend on A1."

**C4 (Rosenberg — LAST-CALL)**: "P1's warehouse: a committed layout — rows 1-2 are Grain-dense (Flour processing chain), column 2 has Barley→Malt conversion. This is a garden: specific adjacency patterns were built for specific contract targets. The dependency is lighter than Dominion's engine but the architectural commitment is real. +1 defend on C4."

**D3 (Stegmaier — LAST-CALL)**: "3-player game completed in ~70 min. Simultaneous placement removed per-turn wait. Draft phase was ~90 seconds per round total. Count-robust. +1 defend on D3."

**A4 (Vaccarino — LAST-CALL)**: "Contract C07 (Barley majority) was not dominated: P3 had 7 Barley by Round 3 but P1 had 3 Barley from late-Round stalls. The majority race went to the last draft. Bag draw provided balanced tile types across rounds (no single-tile glut). Variance managed well. +1 defend on A4."

**B6 (Feld — LAST-CALL)**: "C07 (most Barley: +3 VP bonus) is active. In the final tally, P3 has 7 Barley (majority) and earns 7×2 + 3 = 17 VP from C07 alone. P1 has 3 Barley (3×2 = 6 VP, no bonus). The +3 VP bonus multiplied P3's Barley investment by ~1.2×. The majority-bonus multiplier operated as designed. +1 defend on B6."

---

## Scoring

**P1**: C02 (3 Flour × 2 = 6 VP; −1 Grain × 2 remaining = −2 VP → net 4 VP) + C03 (1 pair × 5 = 5 VP) + C07 (3 Barley × 2 = 6 VP; no majority) + C11 (4 Flax × 3 = 12 VP; −2 empty = −2 VP → 10 VP) + C15 (0 Chaff → 5 VP). Total: 4+5+6+10+5 = **30 VP**.

**P2**: C02 (1 Flour × 2 = 2 VP; −3 Grain = −3 VP → −1 VP) + C03 (0 pairs = 0 VP) + C07 (4 Barley × 2 = 8 VP; no majority) + C11 (5 Flax × 3 = 15 VP; −4 empty = −4 VP → 11 VP) + C15 (1 Chaff → 0 VP). Total: −1+0+8+11+0 = **18 VP**.

**P3**: C02 (0 Flour; −0 Grain = 0 VP) + C03 (0 pairs = 0 VP) + C07 (7 Barley × 2 = 14 VP; majority +3 = 17 VP) + C11 (7 Flax × 3 = 21 VP; −1 empty = −1 VP → 20 VP) + C15 (0 Chaff → 5 VP). Total: 0+0+17+20+5 = **42 VP**.

P3 wins. P3's pivot from Barley-majority to Flax+C11 density, while also holding the Barley majority, produced the dominant score.

---

## Stake classifications (end of game)

| Persona | Axis | Final state | Marks |
|---|---|---|---|
| Knizia | C1 Tension Budget | **earned** | 1 defend |
| Knizia | A1 Elegance | **earned** | 1 defend (LAST-CALL) |
| Knizia | C2 Min-Score Shape | retire-explicit | 0 |
| Rosenberg | C3 Scarcity Bite | **earned** | 1 defend |
| Rosenberg | D4 Late-Game Lock-in | **earned** | 1 defend |
| Rosenberg | C4 Engine-Garden | **earned** | 1 defend (LAST-CALL) |
| Lacerda | B4 Info-Transparency-Cost | retire-explicit | 0 |
| Lacerda | A7 Emergence-Replayability | **earned** | 2 defend (Spring + OP) |
| Lacerda | B1 System Gearing | **earned** | 1 defend |
| Chvátil | A3 Interaction | **earned** | 1 defend (stall competition) |
| Chvátil | B2 Catastrophe Pressure | **earned** | 1 defend |
| Chvátil | B5 Architectural Novelty | retire-explicit | 0 |
| K-K | C7 Action-Menu Clarity | **earned** | — (stakes at draft; not challenged; earns via unchallenged merit) |
| K-K | D1 Family-to-Expert Scaling | hold-explicit | 0 |
| K-K | D2 Spatial-Interaction | retire-explicit | 0 |
| Stegmaier | A6 Teachability | **earned** | 1 defend |
| Stegmaier | D3 Count-Robustness | **earned** | 1 defend (LAST-CALL) |
| Stegmaier | C8 First-Turn Compression | hold-explicit | 0 |
| Vaccarino | A4 Variance Calibration | **earned** | 1 defend (LAST-CALL) |
| Vaccarino | B3 Conversion Chain Depth | hold-explicit | 0 |
| Vaccarino | A5 Downtime/Pacing | **earned** | — (concurrent with Phase 3 simultaneity; earns via unchallenged fast-pacing evidence) |
| Feld (anchor) | C6 Point-Salad Incommensurability | **earned** | 2 defend (Setup + Winter + OP) |
| Feld | A2 Decision Density | **earned** | 1 defend |
| Feld | B6 Scoring Multiplier Dependency | **earned** | 1 defend (LAST-CALL) |

**Earned: 15.** **Refuted: 0.** Hold-explicit: 3 (D1, C8, B3). Retire-explicit: 4 (C2, B4, D2, B5). Silent-retire: 0 (20th consecutive).

## Collisions (1)

**C6↔A7** — Orthogonal-Preservation (OP), 5-2. Temporal-register orthogonality: C6 = within-game scoring incommensurability; A7 = across-game replayability. Both earn defend marks. Validates the temporal-register OP type established at Pandemic Legacy A6↔C8. Serves as collision-vote confirmation for A7 v2.18 counter-pressure cycle 2.

*Note: C7 earns via unchallenged merit — K-K staked 8/8/8 and no persona challenged it during argument; silence in the argument record is not silent-retire under v2.1 (FORCED-ENGAGEMENT required explicit declaration; K-K's C7 was simply uncontested). A5 earns similarly — the simultaneous-placement evidence was visible and unchallenged.*
