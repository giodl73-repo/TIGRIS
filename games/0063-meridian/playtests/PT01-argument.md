---
name: MERIDIAN — PT01 Argument
slug: meridian-PT01
game: 0063-meridian
stage: tierB
version: 1.0.0
rubric_version: v2.23.34
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
player_count: 3
---

# PT01 — MERIDIAN: Argument Record

3-player game. Moment-anchors: Round 1 (first blocking established), Round 3 (network interference mid-game), Round 5 (topology committed; final scoring).

---

## Round 1 — Spatial coupling established

**D2 (K-K anchor) — Defend**: "Round 1: P1 places a road segment on the edge between hex A3 and B3, connecting to Town 1. P2 immediately claims the A3-B4 edge — the only other natural path from P1's starting position toward Town 4. P1 must detour through C3, adding 2 segments to their route. This is canonical direct spatial coupling: P2's placement denies P1 a hex edge permanently, forcing a topological decision. D2 fires from Round 1. +1 defend."

**A1 (Knizia) — Defend**: "P1 faces exactly the decision the two-rule ruleset promises: which edge to place on (maximize route connectivity) vs. which edge P2 will claim (minimize damage from blocking). The entire strategic complexity of a network game emerges from 'place road, score connection.' No exception cases, no power cards, no phase rules beyond those two sentences. A1 fires on the ratio: 2 rules → complete strategic depth. +1 defend."

**A3 (Chvátil) — Defend**: "P2's block of P1's natural route is a deliberate interactive move. P2 calculated: 'if I place here, P1 needs 2 more segments to reach Town 4 — that delays their scoring by 1 round.' Direct player-to-player impact. A3 earns. +1 defend."

---

### D2↔A1 COLLISION — Round 2 (OP, temporal-register)

**K-K (D2 anchor) proposes OP**: "D2 and A1 both fire on the road-placement mechanism but from different analytical registers. D2 asks: how direct is the spatial coupling? It evaluates the game's topology — permanent hex claims, blocking architecture, adjacency geometry. D2 is a question about the game's spatial structure. A1 asks: how much depth do the rules produce? It evaluates the game's design economy — rule count to strategic complexity ratio. A1 is a question about the game's rule architecture. I can describe a game that scores high on D2 but low on A1 (chess: maximally spatial but ~infinite rules); and a game that scores high on A1 but low on D2 (Poker: minimal rules, profound depth, zero spatial coupling). D2 and A1 decouple."

**Knizia (A1 stake) seconds OP**: "I drafted A1 specifically because it fires on a different property than D2. D2 = the spatial coupling is direct (permanent hex edges, adjacency scoring). A1 = the game achieves this with two sentences. A game could have highly direct spatial coupling via many complex rules (low A1, high D2), or it could have minimal rules but weak spatial coupling (high A1, low D2). Meridian is rare: both score high. They're orthogonal."

**Panel votes OP (6-2):**
- Chvátil → OP: "Spatial architecture (D2) and rule economy (A1) are distinct analytical properties. Confirmed — same decouple as B5 (mechanism novelty) vs. A7 (mechanism generativity)."
- Rosenberg → OP: "D2 is about what the spatial structure IS (blocking, adjacency, topology). A1 is about how many rules it takes to create that structure. Different questions."
- Lacerda → OP: "Analogously: B1 (pipeline architecture complexity) and A1 (rule elegance) are different. You can have complex gearing stated in minimal rules or simple gearing requiring many rules."
- Stegmaier → OP: "A6 (teachability) and D2 (spatial depth) are different. A1 and D2 analogously."
- Vaccarino → CR (A1 wins): "Minimal rules produce spatial depth IN THIS GAME because of the spatial mechanism. A1 causes D2. They're nested, not orthogonal."
- Feld → CR (A1 wins): "The elegance (A1) is what makes the spatial coupling (D2) work. They reinforce each other — CR, A1 wins as the deeper claim."

**Result: 6-2 OP** (K-K + Knizia + Chvátil + Rosenberg + Lacerda + Stegmaier = 6 OP; Vaccarino + Feld = 2 CR). **D2 and A1 both earn. 36th temporal-register OP. First D2↔A1 instance — new pair.**

---

## Rounds 3-5 + forced engagement

**FORCED-ENGAGEMENT:**

**Stegmaier (A5):** HOLD-EXPLICIT. "Sequential turns; moderate downtime. Hold."
**K-K (A7):** HOLD-EXPLICIT. "Board randomization provides some replayability. Hold."
**Chvátil (C3):** HOLD-EXPLICIT. "Mild segment scarcity. Hold."
**Stegmaier (C8):** HOLD-EXPLICIT. "First placement declares direction. Hold."
**Lacerda (B1):** RETIRE-EXPLICIT. "+0.5r."
**Rosenberg (C4):** RETIRE-EXPLICIT. "+0.5r."
**Chvátil (B5):** RETIRE-EXPLICIT. "+0.5r."
**Knizia (C2):** RETIRE-EXPLICIT. "+0.5r."
**K-K (B2):** RETIRE-EXPLICIT. "+0.5r."
**Rosenberg (B3):** RETIRE-EXPLICIT. "+0.5r."
**Lacerda (B4):** RETIRE-EXPLICIT. "+0.5r."
**Knizia (A4):** RETIRE-EXPLICIT. "+0.5r."
**Vaccarino (B6):** RETIRE-EXPLICIT. "+0.5r."

**C1 (Rosenberg — unchallenged):** "Limited segments each round. Budget tension perpetual. +1 defend."
**A2 (Lacerda — unchallenged):** "Per-placement density confirmed through rounds 3-5. +1 defend."
**A6 (Stegmaier — unchallenged):** "5 min teach confirmed. +1 defend."
**D3 (Vaccarino — unchallenged):** "3p confirmed scale. +1 defend."
**C7 (Feld — unchallenged):** "Two actions on board. +1 defend."
**D1 (Feld — unchallenged):** "Accessible + expert depth. +1 defend."
**D4 (Vaccarino — unchallenged):** "Route topology locked by round 3. +1 defend."
**A3 (Chvátil — already 1 defend):** "Direct route denial confirmed. +1 defend (total 2)."

Final game: P1 21 VP (connected 7 town pairs across network). P2 17 VP (6 pairs; P1 blocked north route). P3 11 VP (2 disconnected network segments; P2 and P1 both cut routes).

---

## Stake classifications

| Persona | Axis | Final state | Marks |
|---|---|---|---|
| K-K | **D2** | **earned** | 1 defend + OP |
| K-K | A7 | hold-explicit | 0 |
| K-K | B2 | retire-explicit | 0 |
| Knizia | **A1** | **earned** | 1 defend + OP |
| Knizia | C2 | retire-explicit | 0 |
| Knizia | A4 | retire-explicit | 0 |
| Chvátil | **A3** | **earned** | 2 defend |
| Chvátil | B5 | retire-explicit | 0 |
| Chvátil | C3 | hold-explicit | 0 |
| Rosenberg | **C1** | **earned** | 1 defend |
| Rosenberg | C4 | retire-explicit | 0 |
| Rosenberg | B3 | retire-explicit | 0 |
| Lacerda | **A2** | **earned** | 1 defend |
| Lacerda | B1 | retire-explicit | 0 |
| Lacerda | B4 | retire-explicit | 0 |
| Stegmaier | **A6** | **earned** | 1 defend |
| Stegmaier | A5 | hold-explicit | 0 |
| Stegmaier | C8 | hold-explicit | 0 |
| Vaccarino | **D3** | **earned** | 1 defend |
| Vaccarino | D4 | **earned** | 1 defend |
| Vaccarino | B6 | retire-explicit | 0 |
| Feld | **C7** | **earned** | 1 defend |
| Feld | D1 | **earned** | 1 defend |

**Earned: 10 (D2, A1, A3, C1, A2, A6, D3, D4, C7, D1). Refuted: 0. Hold-explicit: 4. Retire-explicit: 9. Silent-retire: 0 (55th consecutive).**

## Collisions (1)

**D2↔A1 — OP 6-2** (K-K anchor, Knizia seconds; Vaccarino + Feld dissent). Spatial network mechanics (D2: permanent hex-grid blocking; adjacency topology) vs. rule elegance (A1: two rules produce complete strategic depth). **36th temporal-register OP. First D2↔A1 instance — new pair established. D2 TARGETED EARN: 10 games since VIADUCT #53.**
