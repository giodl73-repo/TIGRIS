---
name: Codenames — PT01 Argument
slug: codenames-PT01
game: 0025-codenames
stage: tierB
version: 1.0.0
rubric_version: v2.22
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
player_count: 6
teams: [Red (3), Blue (3)]
---

# PT01 — Codenames: Argument Record

6-player game (3v3). Moment-anchors: Setup, Round 1, Round 3 (mid-game), Round 5 (end-game), Scoring.

Grid sample (25 words): PITCH / MERCURY / BOOT / LONDON / CRANE / MARCH / ROME / SHADOW / STAR / WELL / GLASS / IRON / SPRING / CROWN / BANK / WAVE / DIAMOND / MINE / BOLT / CROSS / DUCK / BEAR / SCALE / FIRE / BRUSH

Red agents: MERCURY, CRANE, SHADOW, STAR, WELL, GLASS, IRON, MINE (8 + Double Agent = 9)
Blue agents: BOOT, LONDON, MARCH, ROME, SPRING, CROWN, BANK, WAVE (8)
Assassin: DIAMOND
Bystanders: PITCH, BEAR, SCALE, FIRE, BRUSH, BOLT, CROSS, DUCK

---

## Setup

**A6 (Stegmaier) — Defend**: "Full rules in under 5 minutes: lay 25 cards, draw key card, explain Spymaster role (one word + count, can't be a grid word), explain Operative role (touch to guess, stop when unsure). Under 5 min from box-open to first round. No game in the 25-game corpus teaches faster at this depth. +1 defend on A6."

**B5 (Chvátil anchor) — Hold**: "Architecture is present but the novelty claim requires seeing it in play. Deferring B5 defense to Round 1 when the Spymaster constraint becomes visible."

**C2 (Knizia) — RETIRE-EXPLICIT (immediate)**: "Binary win/lose. No scoring floor. C2 dormancy-watch window game #1. As predicted: Codenames cannot earn C2. +0.5r. Dormancy-watch clock advances: 1 of 3 window games used."

---

## Round 1

**Red Spymaster gives:** "orbit 2" (targeting MERCURY + STAR)
**Red Operatives guess:** STAR (correct, covered Red). MERCURY (correct, covered Red). Turn ends.

**Blue Spymaster gives:** "law 3" (targeting MARCH + CROWN + BANK)
**Blue Operatives discuss:** BANK (correct). CROWN (correct). Debate: MARCH — could be "march" as a military parade or "march" as a legal proceeding. Operatives argue for 30 sec. Touch MARCH (correct). Turn ends.

---

**B5 (Chvátil anchor) — Defend**: "Round 1 demonstrates the architecture: Red Spymaster needed 'orbit' to simultaneously span MERCURY (planet) and STAR (celestial) without touching STAR's other meanings (entertainment, performance) which could have led to non-target words. The Spymaster is solving a semantic constraint satisfaction problem. No prior party game requires this. +1 defend on B5."

**A3 (Chvátil) — Defend**: "Blue Operatives' 30-second debate on MARCH shows the mandatory interaction: the team MUST negotiate meaning aloud, using each other's readings to converge. No individual operative can win alone; the discussion IS the mechanism. +1 defend on A3."

**C1 (Knizia) — Defend**: "The key-card layout: DIAMOND is the Assassin. DIAMOND is adjacent in meaning to GLASS, MINE, IRON — all Red agents. Red Spymaster must avoid any clue that could touch DIAMOND. Every Red clue is now Assassin-constrained. The tension budget is live from Round 1. +1 defend on C1."

**B4 (Lacerda) — Defend**: "Blue Operatives cannot see the Key card. Their debate on MARCH demonstrates the information cost: 'Is MARCH Red or Blue or Bystander or Assassin?' The information is hidden; the clue is the only channel. B4 earns because the hidden Key card creates a genuine information-cost structure that shapes every decision. +1 defend on B4."

**A1 (K-K) — Defend**: "Round 1 complete: 2 rules applied (give clue, touch word), infinite depth of play visible. The rule count is minimal; the depth requires full attention. K-K's canonical A1 pattern. +1 defend on A1."

---

## Round 3 (mid-game)

**Current state**: Red has 5 covered; Blue has 5 covered. Red Spymaster needs to push ahead.

**Red Spymaster gives:** "reflect 3" (targeting GLASS + SHADOW + WELL — reflection, water well mirror)
**Red Operatives**: GLASS (correct). SHADOW (correct, SHADOW reflects light). Debate: WELL — "well as a noun = a well; how does it 'reflect'?" One operative: "reflection metaphor — well of knowledge." Another: "no, this feels wrong." Operatives stop at 2, saving WELL.

**Blue Spymaster gives:** "waterway 2" (targeting WAVE + SPRING)
**Blue Operatives**: WAVE (correct). SPRING (correct).

---

**A2 (Feld) — Defend**: "Red Spymaster's 'reflect 3' demonstrates the decision density: choosing a clue that spans GLASS (literal reflection) + SHADOW (metaphorical reflection) + WELL (well-deep metaphor) requires modeling all 3 target interpretations simultaneously against the Assassin (DIAMOND — could DIAMOND reflect? Yes, dangerously). The Spymaster spent ~45 seconds constructing this clue. Dense decision under pressure. +1 defend on A2."

**A7 (Rosenberg) — Defend**: "Round 3 shows the emergence: the Red Spymaster found a 3-word clue that worked through METAPHORICAL REFLECTION (SHADOW, WELL) — a reading path that would be completely different in a grid without SHADOW + WELL adjacent to GLASS. The specific word combination produced a unique strategic path. Different word grid → different clue-strategies emerge. +1 defend on A7."

**B2 (Feld) — fires**: "The Assassin DIAMOND is adjacent to Red's IRON, MINE, GLASS — all potential Red-clue targets. Red Spymaster must reject several natural clues ('carbon 3', 'mine 2') because they touch DIAMOND. The Assassin creates persistent catastrophe pressure on the dominant party. Earn — but contested."

---

### B5↔A7 COLLISION — Round 3 (temporal-register OP)

**Rosenberg (A7) proposes OP**: "B5 asks 'is the one-word-spanning-targets architecture unprecedented as a mechanism?' A7 asks 'does the game produce a different experience across plays due to word-grid variety?' B5 fires on the mechanism's structural identity (invariant across all plays); A7 fires on what changes between plays (the word grid). These are different temporal registers: B5 is a timeless structural claim about how the mechanism works; A7 is a per-session claim about what experience emerges."

**Chvátil (B5) seconds OP**: "The mechanism (B5) is the same in every game — one-word clue spanning semantic targets. The specific targets (A7) vary infinitely. Same structure, infinite instantiations."

**Vote on OP:**
- Knizia → OP: "C1 tension budget is same-structure-every-game; A7's across-game variety is the word grid. Orthogonal."
- Feld → OP: "A2 decision density fires on the mechanism; A7 fires on the content. Orthogonal."
- Lacerda → OP: "B4 information-cost is structural (Key card hidden); A7 is content-driven (what words appear). Orthogonal."
- K-K → OP: "A1 elegance is structural; A7 is experiential across plays. Same pattern as prior temporal-register OPs."
- Stegmaier → OP: "A6 is teachable-once for the structure; A7 is why you play again. Classic temporal register."
- Rosenberg → OP (proposer).
- Vaccarino → OP.
- Chvátil → OP (anchor; seconds).

**Result: 8-0 Decisive OP.** Unanimous. B5 and A7 both earn. Fourth temporal-register OP in the corpus; first Decisive OP on temporal register. Both earn defend marks.

---

## Round 5 (end-game)

Red has 7 covered (needs 2 more). Blue has 7 covered (needs 1 more). Blue is one correct guess from winning.

**Blue Spymaster gives:** "march 1" — wait, MARCH is already covered. Blue Spymaster: "ROME 1" — one remaining Blue agent.
**Blue Operatives**: ROME (correct). **Blue wins.**

**D3 (Stegmaier) — Defend (end-game)**: "6-player game completed in 22 minutes. Turn overhead at 6 players was minimal — Spymasters deliberate; Operatives discuss simultaneously without waiting. Count-robust. +1 defend on D3."

**A4 (Vaccarino) — Defend (end-game)**: "The word WELL earned via metaphorical connection ('well of knowledge'); SHADOW earned via physical reflection; ROME + MARCH earning via category (European cities or historical periods). The word grid's semantic richness produced unexpected connections the Spymaster found and the Operatives interpreted. Managed variance: the randomness was productive, not chaotic. +1 defend on A4."

---

## FORCED-ENGAGEMENT

Stakes at 0: B2 (Feld), B1 (Lacerda), D1 (K-K), A5 (Knizia), D4 (Rosenberg), C7 (Vaccarino), C8 (Stegmaier).

**Feld (B2):** LAST-CALL. "Assassin as catastrophe — single binary terminator. Testing whether it qualifies as B2."
**Lacerda (B1):** RETIRE-EXPLICIT. "Information pipeline exists but it's a propagation chain, not a conversion chain. +0.5r."
**K-K (D1):** HOLD-EXPLICIT. "Accessible to all ages; expert depth present. Not canonical D1 scaling."
**Knizia (A5):** HOLD-EXPLICIT. "Discussion overhead is not mechanism-based. Hold."
**Rosenberg (D4):** RETIRE-EXPLICIT. "No meaningful late-game lock-in point — the game ends by covering the last agent. +0.5r."
**Vaccarino (C7):** HOLD-EXPLICIT. "Clear menu but not the game's design achievement."
**Stegmaier (C8):** RETIRE-EXPLICIT. "Trivial setup. +0.5r."

**Feld (B2 LAST-CALL):** "The Assassin operates as a constant catastrophic threat: every clue is evaluated against the possibility of an instant-loss. The game's tension budget (C1) is partially determined by the Assassin's proximity to target words. B2 as 'ever-present catastrophe preventing optimal play' earns here in a narrow sense. +1 defend on B2."

---

## Stake classifications

| Persona | Axis | Final state | Marks |
|---|---|---|---|
| Knizia | C1 Tension Budget | **earned** | 1 defend |
| Knizia | C2 Min-Score Shape | **retire-explicit** | 0 (dormancy-watch game #1) |
| Knizia | A5 Downtime | hold-explicit | 0 |
| Rosenberg | A7 Emergence-Replayability | **earned** | 1 defend + Decisive OP |
| Rosenberg | C3 Scarcity Bite | retire-explicit | 0 |
| Rosenberg | D4 Late-Game Lock-in | retire-explicit | 0 |
| Feld | A2 Decision Density | **earned** | 1 defend |
| Feld | B2 Catastrophe Pressure | **earned** | 1 defend (LAST-CALL) |
| Feld | C6 Point-Salad | retire-explicit | 0 |
| Lacerda | B4 Info-Transparency-Cost | **earned** | 1 defend |
| Lacerda | B1 System Gearing | retire-explicit | 0 |
| Lacerda | B3 Conversion Chain | retire-explicit | 0 |
| K-K | D1 Family-to-Expert | hold-explicit | 0 |
| K-K | A1 Elegance | **earned** | 1 defend |
| K-K | D2 Spatial-Interaction | retire-explicit | 0 |
| Stegmaier | A6 Teachability | **earned** | 1 defend |
| Stegmaier | D3 Count-Robustness | **earned** | 1 defend |
| Stegmaier | C8 First-Turn Compression | retire-explicit | 0 |
| Vaccarino | A4 Variance Calibration | **earned** | 1 defend |
| Vaccarino | C7 Action-Menu Clarity | hold-explicit | 0 |
| Vaccarino | B6 Scoring Multiplier | retire-explicit | 0 |
| Chvátil (anchor) | B5 Architectural Novelty | **earned** | 1 defend + Decisive OP |
| Chvátil | A3 Interaction | **earned** | 1 defend |
| Chvátil | C4 Engine-Garden | retire-explicit | 0 |

**Earned: 11.** **Refuted: 0.** Hold-explicit: 4. Retire-explicit: 9. Silent-retire: 0 (22nd consecutive).

## Collisions (1)

**B5↔A7** — **Decisive OP**, 8-0 (unanimous). Temporal-register: B5 = structural mechanism identity (timeless); A7 = word-grid variety across plays. Fourth temporal-register OP in corpus; first unanimous temporal-register Decisive OP. Both earn defend marks.
