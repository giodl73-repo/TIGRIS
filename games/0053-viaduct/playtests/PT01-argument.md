---
name: VIADUCT — PT01 Argument
slug: viaduct-PT01
game: 0053-viaduct
stage: tierB
version: 1.0.0
rubric_version: v2.23.24
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
player_count: 4
---

# PT01 — VIADUCT: Argument Record

4-player game. Session 3 of this copy (canonical play state: 9 hexes pre-revealed). Moment-anchors: Turn 1 (routing commitment), Round 3 (first Flow deliveries), Round 5 (junction conflict), Round 8 (endgame lock-in).

---

## Turn 1 — Routing commitment (C8 targeted earn)

**C8 (Stegmaier anchor stake) — Defend**: "Session 3. Nine hexes revealed: the city grid shows Districts 1, 2, 4, 5, 7, 8, 9, 11 and a high-value Junction-6. P1's Spring is in the northwest corner; three viable pipe routes to the high-value districts compete immediately. Route Alpha (northwest → Junction-6 → District-9, 5 VP) requires 4 segments before first Flow; Route Beta (northwest → District-2 direct, 2 VP) delivers faster but surrenders District-9. P1 must commit on Turn 1 — no guided entry, no catch-up. The revealed topology makes Turn 1 a non-guided strategic bet. C8 earns at session 3+ canonical play state. +1 defend."

**B1 (Lacerda anchor) — Defend**: "Turn 1: P2 plays Survey (reveals District-3, 3 VP) → Lay Pipe (segment placed adjacent to P2's Spring). The 4-gear pipeline fires from turn 1: Survey reveals the destination; Pipe begins the route; Flow and VP follow in later turns but depend causally on these first two actions. B1's multiplicative dependency is live from the opening move. +1 defend."

**D2 (K-K) — Defend**: "Turn 1: P1 and P3 both need Hex-7 (a terrain junction that sits between both their springs and the high-value districts). P1 acts first — lays pipe through Hex-7. P3 is now cut off from the direct route to District-9. P3 must re-route around, adding 2 extra segments and losing 2 turns. Direct spatial blocking from Turn 1. +1 defend."

---

## Rounds 3-5 — First deliveries + junction conflict

**A3 (Chvátil) — Defend**: "Round 4: P2 has established a 3-segment route to District-4 (3 VP). P4 has a competing route that's 1 segment shorter but passes through Hex-12 — which P2 just claimed in their pipe. P4 cannot route through Hex-12; P4 must reroute around via Hex-14, arriving later. The pipe placement created mandatory competitive re-routing. A3 fires as direct structural competition. +1 defend."

**B4 (Chvátil) — Defend**: "Round 4: P3 has been routing toward District-11 (5 VP; highest value). P1 knows P3 has some Capacity cards but doesn't know how many are HF vs. RF. P1 Inspects P3 (spends 1 action). P3 reveals: it's an RF card. Suddenly P1 knows P3 can only Flow to districts within 2 hexes — District-11 is 4 hexes away. P3 was bluffing with their routing commitment; they can't actually deliver to District-11 without at least 2 HF cards. P1 pivots to claim District-11's approach junction before P3. The information cost (1 action to Inspect) yielded real routing advantage. B4 earns canonically. +1 defend."

**C1 (Knizia) — Defend**: "Round 5: P2 has spent 4 of their 8 Flow tokens. P2 calculates: 4 tokens remain; 3 more districts reachable; optimal sequencing matters because the first player to exhaust tokens triggers game-end. P2 holds back a token to avoid triggering end-game prematurely. The budget scarcity is live and actionable. +1 defend."

**A2 (Feld) — Defend**: "Round 5: P4 faces a 3-way decision: (a) Survey Hex-13 to reveal a hidden district (possible high-value; might reorient entire strategy), (b) Lay Pipe to extend toward District-8 (already known, 3 VP), or (c) Inspect P1 to see if P1's claimed route to District-11 is actually executable. Each option has live opportunity cost; P4 calculates for 45 seconds. Decision density is high and multi-axis. +1 defend."

---

### B1↔B4 COLLISION — Round 5 (OP, temporal-register)

**Lacerda (B1 anchor) proposes OP**: "B1 fires on the pipeline architecture — the causal chain Survey→Pipe→Flow→VP that defines how the game's scoring machine operates. B1 is the structure of production. B4 fires on what is hidden about a rival's production machine — the information cost of understanding opponents' capacity. B1 = how the machine works; B4 = what you don't know about their machine. These are different analytical questions: one about mechanism design, one about information economy."

**Chvátil (B4 stake) seconds OP**: "When I'm defending B4, I'm talking about the cost of knowing P3's flow capacity — the information economics. When Lacerda defends B1, he's talking about the causal dependency of production stages. They're asking different questions about the same physical pipe network: B1 asks 'how does it work?'; B4 asks 'what's hidden about how it works?' Genuinely orthogonal."

**Panel votes OP (5-3):**
- Knizia → OP: "C1 fires on the resource budget (Flow tokens); B1 fires on the production architecture; B4 fires on the information about that architecture. Three different questions."
- Rosenberg → OP: "D4 lock-in is a consequence of B1's pipeline commitment; B4 is the cost of knowing what lock-in your rivals have made. Orthogonal registers."
- Stegmaier → OP: "C8 fires on first-turn routing; B1 fires on the full production chain; B4 fires on info asymmetry in that chain. Distinct."
- Feld → CR (B1 wins): "B4's information economy serves B1's pipeline optimization. They're the same mechanism from two angles — you need B4 information to operate B1 effectively. CR: B1."
- K-K → CR (B1 wins): "D2 spatial blocking and B4 inspection are both tools for understanding the pipeline state. B4 exists to serve B1; they're not independent."
- Vaccarino → CR (B1 wins): "The Capacity cards are a B1 mechanism — they're part of the pipeline architecture. B4 is just describing one B1 component as 'hidden.'"

**Result: 5-3 OP** (Lacerda proposes, Chvátil seconds = 5 for OP; Feld, K-K, Vaccarino dissent). **B1 and B4 both earn. 26th temporal-register OP. 5-3 vote.** Lacerda own-anchor confirmed (19 games overdue).

---

## Rounds 7-9 — Endgame routing + forced engagement

**FORCED-ENGAGEMENT (between R7 and R8):**

**Vaccarino (A7):** HOLD-EXPLICIT. "Map topology + session growth creates variety. Hold — not canonical."
**Rosenberg (C3):** HOLD-EXPLICIT. "Flow tokens are scarce but not punishing neglect in the Rosenberg feeding-phase sense. Hold."
**Knizia (A1):** HOLD-EXPLICIT. "Moderate elegance at weight 3.5. Hold."
**Feld (A4):** HOLD-EXPLICIT. "Capacity card variance is present but light. Hold."
**K-K (D1):** HOLD-EXPLICIT. "Accessible/deep gradient present. Hold."
**Stegmaier (A5):** HOLD-EXPLICIT. "Moderate downtime. Hold."
**Rosenberg (C4):** HOLD-EXPLICIT. "Light garden-like dependency. Hold."
**Vaccarino (B5):** RETIRE-EXPLICIT. "+0.5r."
**Feld (C6):** RETIRE-EXPLICIT. "+0.5r."
**Lacerda (B6):** RETIRE-EXPLICIT. "+0.5r."
**Chvátil (B2):** RETIRE-EXPLICIT. "+0.5r."
**Knizia (B3):** RETIRE-EXPLICIT. "+0.5r."

**A6 (Stegmaier — unchallenged):** "6 clear actions; hydraulic metaphor. Taught in 14 min. +1 defend."

**C7 (K-K — unchallenged):** "6 actions; clear costs. +1 defend."

**D3 (Lacerda — unchallenged):** "3–4 players; hex scales via route density. +1 defend."

**D4 (Rosenberg — unchallenged):** "Pipe routes lock strategy by Round 6 (can't un-lay pipe). +1 defend."

**Final scoring**: P1 wins with 24 VP (early District-9 delivery + longest network bonus). P3 (bluff-routed to District-11) finishes 3rd despite high-value target — capacity cards didn't support delivery distance.

---

## Stake classifications

| Persona | Axis | Final state | Marks |
|---|---|---|---|
| Knizia | C1 | **earned** | 1 defend |
| Knizia | A1 | hold-explicit | 0 |
| Knizia | B3 | retire-explicit | 0 |
| Rosenberg | D4 | **earned** | 1 defend |
| Rosenberg | C3 | hold-explicit | 0 |
| Rosenberg | C4 | hold-explicit | 0 |
| Lacerda | **B1** | **earned** | 1 defend + OP |
| Lacerda | D3 | **earned** | — (unchallenged) |
| Lacerda | B6 | retire-explicit | 0 |
| Chvátil | A3 | **earned** | 1 defend |
| Chvátil | **B4** | **earned** | 1 defend + OP |
| Chvátil | B2 | retire-explicit | 0 |
| K-K | D2 | **earned** | 1 defend |
| K-K | C7 | **earned** | — (unchallenged) |
| K-K | D1 | hold-explicit | 0 |
| Stegmaier | **C8** | **earned** | 1 defend (targeted) |
| Stegmaier | A6 | **earned** | — (unchallenged) |
| Stegmaier | A5 | hold-explicit | 0 |
| Vaccarino | A7 | hold-explicit | 0 |
| Vaccarino | B5 | retire-explicit | 0 |
| Feld | A2 | **earned** | 1 defend |
| Feld | C6 | retire-explicit | 0 |
| Feld | A4 | hold-explicit | 0 |

**Earned: 11.** **Refuted: 0.** Hold-explicit: 7. Retire-explicit: 5. Silent-retire: 0 (45th consecutive).

## Collisions (1)

**B1↔B4 — OP 5-3** (Lacerda proposes, Chvátil seconds; Feld/K-K/Vaccarino dissent). Pipeline production architecture (B1: Survey→Pipe→Flow→VP structural dependency) vs. information cost of hidden capacity (B4: Capacity cards hidden; Inspect economy). **26th temporal-register OP. Lacerda own-anchor confirmed (19 games overdue).** C8 targeted earn confirmed.
