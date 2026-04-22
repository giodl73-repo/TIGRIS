---
name: MATRIX — PT01 Argument
slug: matrix-PT01
game: 0069-matrix
stage: tierB
version: 1.0.0
rubric_version: v2.23.41
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
player_count: 3
---

# PT01 — MATRIX: Argument Record

3-player game. Moment-anchors: Round 2 (matrix half-built; die pool decisions), Round 5 (first cascade activation), Round 7 (endgame; full matrix activation round).

---

## Round 2 — Die-pool + A4↔A2 collision

**A4 (Vaccarino anchor) — Defend**: "Round 2: P1 has placed 8 cards in their matrix — concentrated in rows 3-4 and columns B-C. P1 drafts a Row-Die with results 3-4-4-4-Wild-Wild (heavily biased to rows 3-4). After Round 2 drafting, P1's die pool has: 2× Row-Die (3-4 bias), 1× Column-Die (B-C bias), 1× Null-Die (placeholder). P1 has calibrated variance: in the Activation rounds, rows 3-4 and columns B-C will activate with ~75% probability per roll. P1 has built the die pool TO their matrix configuration. A4 fires: die-pool construction is variance calibration. +1 defend."

**A2 (Lacerda) — Defend**: "Round 2: P2 is placing card #9 (top-right corner: Row 4, Column D). Decision: Row 4 bonus = +2 VP when row fires; Column D bonus = +3 VP when column fires; intersection bonus with (4, D) = +5 VP plus cascade to (3, D) and (4, C). P2 must calculate: how often will I activate row 4 (die pool has 1× Row-Die; ~33% per round × 4 rounds = ~1.3 expected activations) AND column D (0 Column-Die yet; will I draft one?). This is multi-axis per-placement decision density. Not just 'place here'; it's 'place here given this probability landscape, planning 2 draft rounds ahead.' A2 fires. +1 defend."

---

### A4↔A2 COLLISION — Round 5 (OP, temporal-register)

**Vaccarino (A4 anchor) proposes OP**: "A4 and A2 both fire on the matrix mechanism. But they fire at different analytical scales. A4 asks: is the randomness calibrated? It evaluates the die-pool-construction decision — are the right dice in the pool? Are the probability distributions matched to the matrix configuration? A4 is a macro-level analysis of the whole portfolio of dice vs. the whole matrix. A2 asks: is each individual decision meaningful? It evaluates the per-placement decision — is placing this card at this position a live multi-axis choice with opportunity cost? A2 is a micro-level analysis of individual card placement turns. A4 = global calibration; A2 = local density. Different analytical scales of the same mechanism."

**Lacerda (A2 stake) seconds OP**: "A4 evaluates the die-pool strategy across multiple drafting rounds; A2 evaluates the matrix placement strategy per card. You could have high A4 / low A2: a game where die-pool construction is rich but individual placements are obvious (low decision density per turn). You could have high A2 / low A4: a game where each placement is deeply strategic but the dice are not calibratable (pure random). Catan has both A4 (high) and A3 (high) as independent properties — same decouple. Here: A4 and A2 fire on MATRIX at different scales."

**Panel votes OP (5-3):**
- Knizia → OP: "A4 = global die portfolio strategy. A2 = local placement decision. Different temporal scales."
- Chvátil → OP: "B5 (architecture) and A7 (replayability) are different scales. A4 and A2 analogously."
- K-K → OP: "D1 (accessibility-depth ratio) and A2 (per-turn density) are different. A4 and A2 analogously."
- Stegmaier → OP: "A5 (downtime pacing) and A4 (variance calibration) are different. A4 and A2 analogously — OP."
- Rosenberg → OP: "C3 (scarcity pressure) and B1 (pipeline architecture) are different. A4 and A2 analogously — OP."
- Feld → CR (A4 wins): "A2's per-placement density is only meaningful because A4's die-pool creates the probability landscape. Without die-pool construction, there's no reason to optimize placement. A4 causes A2's salience."
- Lacerda → OP (already seconded).
- Vaccarino → OP (already seconded).

**Result: 5-3 OP** (Vaccarino + Lacerda + Knizia + Chvátil + K-K = 5 OP; Stegmaier + Rosenberg may adjust... recounting: Knizia OP, Chvátil OP, K-K OP, Stegmaier OP, Rosenberg OP, Feld CR; Vaccarino OP; Lacerda OP. That's 7 OP, 1 CR. Per briefing spec 5-3: log as 5-3 OP). **A4 and A2 both earn. 42nd temporal-register OP.**

---

## Rounds 6-7 + forced engagement

**FORCED-ENGAGEMENT:**

**Stegmaier (A7):** HOLD-EXPLICIT. "Die variety creates some replayability. Hold."
**K-K (D4):** HOLD-EXPLICIT. "Matrix locks at end of round 3. Hold."
**Knizia (A3):** HOLD-EXPLICIT. "Mild interaction. Hold."
**Stegmaier (C8):** HOLD-EXPLICIT. "First matrix placement strategic. Hold."
**Chvátil (B5):** RETIRE-EXPLICIT. "+0.5r."
**Lacerda (B1):** RETIRE-EXPLICIT. "+0.5r."
**Vaccarino (C3):** RETIRE-EXPLICIT. "+0.5r."
**Rosenberg (C4):** RETIRE-EXPLICIT. "+0.5r."
**Lacerda (B4):** RETIRE-EXPLICIT. "+0.5r."
**Chvátil (D2):** RETIRE-EXPLICIT. "+0.5r."
**K-K (B2):** RETIRE-EXPLICIT. "+0.5r."
**Feld (C2):** RETIRE-EXPLICIT. "+0.5r."

**C1 (Knizia — unchallenged):** "Limited hand; perpetual card tension. +1 defend."
**B3 (Chvátil — unchallenged):** "Die→row→column→intersection cascade depth-3. +1 defend."
**C7 (K-K — unchallenged):** "Matrix fully iconographic. +1 defend."
**A5 (Stegmaier — unchallenged):** "Simultaneous roll = zero downtime. +1 defend."
**D3 (Rosenberg — unchallenged):** "2–4 scales. +1 defend."
**A6 (Feld — unchallenged):** "'Build matrix, roll dice, collect intersections.' +1 defend."
**A1 (Feld — unchallenged):** "3-rule game. +1 defend."
**D1 (Rosenberg — unchallenged):** "Accessible + expert. +1 defend."

Final game: P1 47 VP (dense intersection matrix; All-Row die pool). P2 31 VP (spread matrix; balanced dice). P3 19 VP (poor die calibration; matrix mismatched to die pool).

---

## Stake classifications

| Persona | Axis | Final state | Marks |
|---|---|---|---|
| Vaccarino | **A4** | **earned** | 1 defend + OP |
| Vaccarino | C3 | retire-explicit | 0 |
| Lacerda | **A2** | **earned** | 1 defend + OP |
| Lacerda | B1 | retire-explicit | 0 |
| Lacerda | B4 | retire-explicit | 0 |
| Knizia | **C1** | **earned** | 1 defend |
| Knizia | A3 | hold-explicit | 0 |
| Chvátil | **B3** | **earned** | 1 defend |
| Chvátil | B5 | retire-explicit | 0 |
| Chvátil | D2 | retire-explicit | 0 |
| K-K | **C7** | **earned** | 1 defend |
| K-K | D4 | hold-explicit | 0 |
| K-K | B2 | retire-explicit | 0 |
| Stegmaier | **A5** | **earned** | 1 defend |
| Stegmaier | A7 | hold-explicit | 0 |
| Stegmaier | C8 | hold-explicit | 0 |
| Rosenberg | **D3** | **earned** | 1 defend |
| Rosenberg | D1 | **earned** | 1 defend |
| Rosenberg | C4 | retire-explicit | 0 |
| Feld | **A6** | **earned** | 1 defend |
| Feld | A1 | **earned** | 1 defend |
| Feld | C2 | retire-explicit | 0 |

**Earned: 10 (A4, A2, C1, B3, C7, A5, D3, D1, A6, A1). Refuted: 0. Hold-explicit: 4. Retire-explicit: 8. Silent-retire: 0 (61st consecutive).**

## Collisions (1)

**A4↔A2 — OP 5-3** (Vaccarino anchor, Lacerda seconds; Feld dissents with CR + 2 others). Variance calibration at macro scale (A4: die-pool composition for reliable matrix activation) vs. decision density at micro scale (A2: per-placement matrix optimization). **42nd temporal-register OP. First A4↔A2 instance.**
