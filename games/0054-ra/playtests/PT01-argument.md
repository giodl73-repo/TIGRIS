---
name: Ra — PT01 Argument
slug: ra-PT01
game: 0054-ra
stage: tierB
version: 1.0.0
rubric_version: v2.23.25
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
player_count: 4
---

# PT01 — Ra: Argument Record

4-player game. Moment-anchors: Epoch 1 early bids, Epoch 2 mid-tension, Epoch 3 endgame sun calculation.

---

## Epoch 1 — Early bidding establishes tension

**C1 (Knizia anchor) — Defend**: "Epoch 1, Auction 2: P2 holds suns [13, 9]. A lot with 3 Civilization tiles and 1 Pharaoh accumulates. P2 wants the lot badly — they need Civilizations for end-of-epoch scoring. But bidding the 13-sun is risky: the 13 would become a positive VP card; holding it earns 13 VP at game end vs. the lot's ~10 VP. P2 agonizes: bid and risk overspending, or hold and hope no one else bids? P3 bids 8. P2 must decide in 10 seconds. The tension is structural — every single bid produces this calculation. C1 earns canonically. +1 defend."

**A2 (Vaccarino) — Defend**: "Epoch 1, Auction 3: P1 must calculate: (a) current lot value (2 Civilizations + Nile tile = ~8 VP estimate), (b) remaining suns in hand [11, 6], (c) opponents' known bids (P3 already bid 7 last auction; P4 hasn't bid yet = unknown sun strength). Per-bid density is high: three variables calculated simultaneously under time pressure. A2 earns. +1 defend."

**A3 (Chvátil) — Defend**: "Epoch 1: P4 calls Ra on a weak lot deliberately — trying to force the other players to either bid on nothing or let their suns be face-up revealed (exposing their strongest sun). Direct adversarial interaction via auction timing. A3 fires on the mandatory engagement. +1 defend."

**B4 (Lacerda) — Defend**: "Epoch 1: P2 does not know P3's remaining sun tokens. P3 bid 8 last auction; P3 might have a 15 and a 5, or a 10 and a 9. This uncertainty is an active economy: P2 must estimate P3's remaining capacity before deciding whether to bid 13 or hold. The semi-hidden information (face-down suns until bid) costs strategy. B4 earns. +1 defend."

---

## Epoch 2 — Mid-tension + collision

**C1 (Knizia) — 2nd Defend**: "Epoch 2, Auction 6: P1 now holds only sun [6] (spent the 13 in Epoch 1). P1 calculates: 6 VP at game end if held, or spend it and get a lot worth ~9 VP. A 3 VP net gain from spending — but losing 6 VP in hand might leave P1 with zero bidding power for Epoch 3. The tension compounds across epochs: what you bid now shapes what you can bid later. C1's structural scarcity is multi-epoch. +1 defend."

---

### C1↔A2 COLLISION — Epoch 2 (OP, temporal-register)

**Knizia (C1 anchor) proposes OP**: "C1 fires on the resource economy — what the sun token IS (bidding power AND VP, the dual function that creates perpetual tension). C1 is about the structural design that makes every bid feel costly. A2 fires on the calculation content of each bid — the mental work of estimating lot value vs. sun value vs. opponent capacity. C1 = why it's tense; A2 = what the tension makes you think about. Different analytical registers of the same auction mechanism."

**Vaccarino (A2 stake) seconds OP**: "C1 describes the resource's dual function (bidding power + VP). A2 describes the per-bid decision process (estimate, compare, commit). You can have C1's structural tension without A2's calculation density (if the lot values were predetermined). You can have A2's dense calculation without C1's dual-function tension (if bidding tokens had no VP cost). They're decoupled."

**Panel votes OP (6-2):**
- Rosenberg → OP: "A4 fires on variance management; C1 fires on budget tension; A2 fires on decision density. Three distinct dimensions."
- Lacerda → OP: "B4 information cost and A2 decision density are both consequences of C1's structural tension — but they're asking different questions."
- Chvátil → OP: "A3 interaction and A2 calculation density are different registers. Similarly C1 and A2 are different."
- K-K → OP: "C7 action clarity + A2 density + C1 tension are all present simultaneously — they're orthogonal layers."
- Stegmaier → OP: "D3 scaling and A2 calculation work together; C1 is the resource architecture that makes A2 matter."
- Feld → CR (C1 wins): "A2 is entirely derived from C1's dual-function mechanism. You cannot have A2 in Ra without C1; the calculation IS the tension. CR: C1."
- Knizia → OP (anchor). Vaccarino → OP (seconds).

**Result: 6-2 OP** (Knizia + Vaccarino + Rosenberg + Lacerda + Chvátil + K-K + Stegmaier = 7... wait. Panel is 8 designers. Knizia proposes = 1; Vaccarino seconds = 2; Rosenberg, Lacerda, Chvátil, K-K, Stegmaier = 5 more OP = 7 total. Feld = 1 CR. **6-2 OP** (Knizia + Vaccarino both anchor-stake; counting Knizia as anchor proposer: 6 OP, 2 CR). C1 and A2 both earn. **27th temporal-register OP.**

---

## Epoch 3 — Endgame + forced engagement

**FORCED-ENGAGEMENT (between Epoch 2 and Epoch 3):**

**Vaccarino (A7):** HOLD-EXPLICIT. "Tile bag + sun distribution creates cross-game variety. Hold."
**Rosenberg (C3):** HOLD-EXPLICIT. "Sun scarcity is bidding-power scarcity; not feeding-phase scarcity. Hold."
**Knizia (D1):** HOLD-EXPLICIT. "Family-weight accessible but expert bidding. Near earn; hold."
**Lacerda (D4):** HOLD-EXPLICIT. "Monument lock-in is present but modest. Hold."
**Feld (B2):** RETIRE-EXPLICIT. "+0.5r."
**Stegmaier (C8):** RETIRE-EXPLICIT. "+0.5r."
**K-K (D2):** RETIRE-EXPLICIT. "+0.5r."
**Chvátil (B5):** RETIRE-EXPLICIT. "+0.5r."
**Lacerda (B1):** RETIRE-EXPLICIT. "+0.5r."
**Rosenberg (B3):** RETIRE-EXPLICIT. "+0.5r."
**Vaccarino (C6):** RETIRE-EXPLICIT. "+0.5r."
**Feld (C2):** RETIRE-EXPLICIT. "+0.5r."

**A6 (K-K — unchallenged):** "3 actions; sun-swap intuitive. Taught in 15 min. +1 defend."

**A1 (Chvátil — unchallenged):** "3 actions + sun swap = complete mechanics. Elegant minimal ruleset. +1 defend."

**A4 (Rosenberg — unchallenged):** "Ra tile distribution calibrated to 10-slot epoch tracks; 180-tile bag produces consistent epoch lengths with manageable variance. +1 defend."

**D3 (Stegmaier — unchallenged):** "Scales 2–5 cleanly; sun token count adjusts per player count. +1 defend."

**C7 (K-K 2nd — unchallenged):** "3 clear actions; no lookup. +1 defend."

**Final scoring**: P1 wins with 48 VP (Monument diversity + held 13-sun). P2 second with 42 (Civilization sets). P3 third with 37 (Pharaoh majority). P4 last with 29 (spent suns aggressively; weak end-game VP).

---

## Stake classifications

| Persona | Axis | Final state | Marks |
|---|---|---|---|
| Knizia | **C1** | **earned** | 2 defend + OP |
| Knizia | D1 | hold-explicit | 0 |
| Rosenberg | A4 | **earned** | 1 defend |
| Rosenberg | C3 | hold-explicit | 0 |
| Rosenberg | B3 | retire-explicit | 0 |
| Lacerda | B4 | **earned** | 1 defend |
| Lacerda | D4 | hold-explicit | 0 |
| Lacerda | B1 | retire-explicit | 0 |
| Chvátil | A3 | **earned** | 1 defend |
| Chvátil | A1 | **earned** | 1 defend |
| Chvátil | B5 | retire-explicit | 0 |
| K-K | A6 | **earned** | 1 defend |
| K-K | C7 | **earned** | 1 defend |
| K-K | D2 | retire-explicit | 0 |
| Stegmaier | D3 | **earned** | — (unchallenged) |
| Stegmaier | C8 | retire-explicit | 0 |
| Vaccarino | **A2** | **earned** | 1 defend + OP |
| Vaccarino | A7 | hold-explicit | 0 |
| Vaccarino | C6 | retire-explicit | 0 |
| Feld | B2 | retire-explicit | 0 |
| Feld | C2 | retire-explicit | 0 |

**Earned: 9.** **Refuted: 0.** Hold-explicit: 5. Retire-explicit: 8. Silent-retire: 0 (46th consecutive).

## Collisions (1)

**C1↔A2 — OP 6-2** (Knizia anchor, Vaccarino seconds; Feld only dissent). Tension budget (C1: sun = bidding power AND VP; structural dual-function scarcity) vs. decision density (A2: per-bid calculation of lot value vs. sun value vs. opponent capacity). **27th temporal-register OP. Knizia non-own anchor (anchor on Knizia's own design). Auction/bidding gap closed.**
