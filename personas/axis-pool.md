---
name: TIGRIS Axis Pool
slug: axis-pool
version: 2.0.0
rubric_version: v2.0
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# Axis Pool — v2.0

The pool from which personas draft their stakes. 24 axes across 4 bands. Each axis has a definition, an anchor descriptor, and a ledger slot for adopted/retired status.

Every draft is a choice from this Pool. Adopted axes (earned ≥ 2 games) stay forever. Retired axes (refuted ≥ 2 games) cost +1 draft-slot to re-pick. Contested axes carry their prior argument record.

## Draft protocol

- Each designer persona drafts **3 axes per game**. Snake order across 3 rounds.
- Anchor persona (declared at CONCEPT) drafts last in round 1, first in round 2. This forces them onto shared ground — or forces them to explicitly decline.
- Retired axes cost 2 draft-slots instead of 1. A persona spending a retired slot is staking prestige that the argument has changed.
- No axis may be drafted twice in the same game. Axis scarcity is the first argument.

## Stake scoring

Once drafted, a persona rates their axis on 0–10 for each target player count of the design, with a one-sentence justification naming a specific rule or mechanic. No weighted aggregates — stakes are individual assertions, attacked or defended one at a time in Argument.

## States

- `adopted` — earned ≥ 2 states across ≥ 2 games. Permanent pool member.
- `live` — in the Pool but not yet adopted or retired.
- `retired` — refuted ≥ 2 states across ≥ 2 games. Cost +1 to draft.
- `contested` — has both earned and refuted states; record carried forward.

## Band A — Carried from v1.0 (7 axes)

These axes survived the v1.0 → v2.0 transition unchanged. `Thematic Integration` was dropped (panel consensus: re-skinnable, not Euro-specific). They enter v2.0 as `live`.

### A1. Elegance
**Definition:** Rule-count-to-depth ratio.
**Anchor 10:** Rule count understates depth; emergent complexity the rules didn't spell out (T&E's catastrophe triggers).
**State:** live.

### A2. Decision Density
**Definition:** Meaningful choices per turn.
**Anchor 10:** Every turn presents a multi-axis choice with live opportunity cost.
**State:** live.

### A3. Interaction
**Definition:** Player-to-player impact.
**Anchor 10:** The game *is* the interaction; remove opponents and the game has no content.
**State:** live.

### A4. Variance Calibration
**Definition:** Luck scaled to game length.
**Anchor 10:** Every random element has a mitigation tool; variance creates strategy space.
**State:** live.

### A5. Downtime / Pacing
**Definition:** Wait between meaningful player activity.
**Anchor 10:** Turns interleave; players always have a live decision.
**State:** live.

### A6. Teachability
**Definition:** Rules → first-turn friction.
**Anchor 10:** Can teach in under 10 min; first-turn decisions are comprehensible.
**State:** live.

### A7. Emergence / Replayability
**Definition:** Strategy space diversity across plays.
**Anchor 10:** 50+ plays still reveal new strategic veins; meta evolves.
**State:** live.

## Band B — Euro-specific (6 axes, v2.6 adds B6 Scoring Multiplier Dependency)

Added in response to v1.0 panel consensus (Lacerda RC4, Chvátil RC1, Feld RC1). These axes fail the re-skin test — they are specifically Euro-tradition concerns.

### B1. System Gearing
**Definition:** Multiplicative dependency between subsystems. Remove one subsystem and count how many others lose anchor value.
**Anchor 10:** Every subsystem depends on every other; remove one and the game has no interior (Lisboa, Kanban EV).
**State:** live. **Primary advocate:** Lacerda.

### B2. Catastrophe Pressure
**Definition:** Frequency and impact of reversal events within the arc.
**Anchor 10:** Catastrophes are narrative generators that punish neglect and reward preparation (T&E's catastrophe triggers, Feld's plague phases).
**State:** live. **Primary advocate:** Knizia / Feld.

### B3. Conversion Chain Depth
**Definition:** How many transformations VP passes through between acquisition and scoring.
**Anchor 10:** VP is compound; simple goods route through ≥ 3 transformations (Le Havre, Brass).
**State:** live. **Primary advocate:** Knizia / Rosenberg.

### B4. Information Transparency Cost
**Definition:** What is hidden, and what does it cost to learn?
**Anchor 10:** Information asymmetry is an active economy; players pay for knowledge (Agricola occupation draws, Lacerda's card-hand choices).
**State:** live. **Primary advocate:** Lacerda.

### B5. Architectural Novelty
**Definition:** Structural bet distinct from predecessor designs. Remove the signature mechanic — is any other game indistinguishable?
**Anchor 10:** The game's core architecture has no sibling; copycats are obvious (Through the Ages's ledger, Tzolk'in's gears, Galaxy Trucker's real-time build).
**State:** live. **Primary advocate:** Chvátil.

### B6. Scoring Multiplier Dependency (**NEW v2.6**)
**Definition:** Do endgame points depend multiplicatively on choices made across multiple subsystems, forcing balance across them?
**Anchor 0**: single-track VP (raw sum, no multipliers).
**Anchor 5**: one or two multipliers present (e.g., Scythe's popularity band on final scoring).
**Anchor 10**: multiple stacking multipliers define the scoring space (Puerto Rico: Residence × plantations, Guild Hall × production-buildings, etc.).
**State:** live. **Primary advocate:** Rosenberg. Secondary: Vaccarino.
**Adoption source:** I-parliament-02 cluster (Parliament multiplier + Scythe popularity bands + Puerto Rico endgame buildings = 3 cluster instances).

## Band C — Persona-signature (8 axes, new in v2.0)

Axes that each carry a specific designer's voice. A draft on one of these is an implicit claim of that persona's tradition.

### C1. Tension Budget
**Definition:** Does every player always have slightly fewer resources than they'd like?
**Anchor 10:** Every turn ends with the player knowing the next best action and not being able to take it.
**State:** live. **Primary advocate:** Knizia.

### C2. Minimum-Score Shape
**Definition:** Scoring that forces breadth through non-linear conversion (minimum-color, pyramidal).
**Anchor 10:** Ingenious's minimum-color rule; T&E's minimum-aspect shape; games where specialization loses by construction.
**State:** live. **Primary advocate:** Knizia.

### C3. Scarcity Bite
**Definition:** Does the feeding-phase / action-shortage actually punish neglect?
**Anchor 10:** Agricola's feeding phase — starvation is common in the first few plays and shapes every subsequent decision.
**State:** live. **Primary advocate:** Rosenberg.

### C4. Engine-Garden Dependency
**Definition:** Are late-game buildings / improvements / engines conditional on earlier placements?
**Anchor 10:** The value of a building placed in turn 10 depends on 3+ buildings placed in turns 1–8 (Le Havre, A Feast for Odin).
**State:** live. **Primary advocate:** Rosenberg.

### C5. Anti-Catch-up Pressure
**Definition:** Plague-phase or penalty mechanism that punishes players who didn't prepare.
**Anchor 10:** In the Year of the Dragon's event track; Notre Dame's plague cube.
**State:** live. **Primary advocate:** Feld.

### C6. Point-Salad Incommensurability
**Definition:** Are the VP tracks truly non-substitutable?
**Anchor 10:** Castles of Burgundy — ships/livestock/buildings/workers pay in VP curves that cannot be directly compared.
**State:** live. **Primary advocate:** Feld.

### C7. Action-Menu Clarity
**Definition:** Iconography + menu size ≤ 6; rules communicated by components.
**Anchor 10:** Tikal's action-point economy; Azul's phase clarity; Scythe's player-mat iconography.
**State:** live. **Primary advocate:** Kramer-Kiesling.

### C8. First-Turn Compression
**Definition:** A new player makes a meaningful strategic decision on turn 1, not a guided one.
**Anchor 10:** Scythe's first action reveals the dual-layer mat; Wingspan's first bird is a live engine choice.
**State:** live. **Primary advocate:** Stegmaier.

## Band D — Family / accessibility (4 axes, new in v2.0)

### D1. Family-to-Expert Scaling
**Definition:** Teachable to a 9-year-old and deep enough for a tournament.
**Anchor 10:** Azul — the rules fit on a card; the tournament scene is active.
**State:** live. **Primary advocate:** Kramer-Kiesling.

### D2. Spatial-Interaction Presence
**Definition:** Direct board-state coupling between players (block, displace, share adjacency).
**Anchor 10:** Tikal's pyramid competition; Mexica's canal-blocking; chess.
**State:** live. **Primary advocate:** Kramer-Kiesling.

### D3. Count-Robustness
**Definition:** Does the game play equivalently at 2p, 3p, 4p, 5p?
**Anchor 10:** Every count is tuned; the weakest count scores within 0.5 of the strongest.
**State:** live. **Primary advocate:** Stegmaier.

### D4. Late-Game Lock-in Point
**Definition:** At what fraction of game length do strategies commit? (A good game locks at ~⅔.)
**Anchor 10:** The commit point is visible, narratively justified, and leaves meaningful optimization in the last third.
**State:** live. **Primary advocate:** Rosenberg / Lacerda.

## Band E — Experiential (4 axes, new in v2.24)

Added to support PCA research program (150-game corpus). These axes capture dimensions orthogonal to mechanical structure — narrative shape, social encoding, physical instantiation, cognitive demand type. Fast-tracked into live pool.

### A25. Emotional Arc
**Definition:** Does the game have a designed narrative or emotional shape across a single play — distinct from mechanical tension?
**Anchor 10:** Play has a felt beginning/middle/end described in narrative terms even by non-gamers (Sleeping Gods campaign, legacy climax).
**Anchor 5:** Some emotional shape but primarily mechanical (Pandemic cooperative fear curve).
**Anchor 0:** Pure abstraction; no felt arc beyond win/loss (Azul, Crokinole).
**State:** live. **Primary advocate:** Stegmaier.

### A26. Social Contract Dependency
**Definition:** How much does the game depend on implicit player behavior not encoded in rules?
**Anchor 10:** Game breaks or transforms completely if players violate implicit norms — no-meta-communication rule is unenforceable but essential (Hanabi, The Mind).
**Anchor 5:** Some implicit norms (don't kingmake, don't spite-play) but game survives violation.
**Anchor 0:** Rules fully specify all relevant behavior.
**State:** live. **Primary advocate:** Chvátil.

### A27. Physical Affordance
**Definition:** How load-bearing is the physical/tactile instantiation of the game?
**Anchor 10:** The game IS the physical act — digital version loses the core experience (Crokinole: flicking IS the game).
**Anchor 5:** Physical components enhance experience meaningfully but digital works (Azul tiles are pleasurable but not essential).
**Anchor 0:** Fully digitizable with zero experiential loss (Dominion, Hanabi).
**State:** live. **Primary advocate:** Kramer-Kiesling.

### A28. Cognitive Load Profile
**Definition:** What type of reasoning does the game primarily demand? Score on spectrum: execution(0) → spatial(3) → probabilistic(5) → deductive(7) → narrative(8) → novel-combined(10).
**Anchor 10:** Novel cognitive combination not found in prior games.
**Anchor 7:** Pure deductive reasoning as the game's core (Hanabi, Mastermind).
**Anchor 3:** Spatial reasoning primary (Hive, Blokus, Crokinole).
**Anchor 0:** Pure execution; no significant reasoning required (roll-and-move).
**State:** live. **Primary advocate:** Lacerda.

---

## Rubric Ledger (live state)

As games complete TIER-C, this section records adoption/retirement history. **Parliament (game #1) complete — 21 axes have first-game records.** Thresholds: ≥ 2 earned across ≥ 2 games → `adopted`; ≥ 2 refuted across ≥ 2 games → `retired`. Game #2 (expected: Tigris & Euphrates) will trigger first adoptions/retirements for queued axes.

Columns: **Earned / Refuted / Contested / Ignored** counts are cumulative across games (value = game-count in that state).

| Axis | Earned | Refuted | Contested | Ignored | Status | Note |
|---|---:|---:|---:|---:|---|---|
| A1 Elegance | 47 | 0 | 2 | 0 | **adopted** | Prior 40 + Fracture#62 + Meridian#63 (OP) + Cauldron#64-hold + LeHavre#65-hold + SpiritIsland#66-retire + Deluge#67-hold + Catan#68 + Matrix#69. Post-session: **47E career high.** |
| A2 Decision Density | 49 | 0 | 0 | 1 | **adopted** | Prior 41 + Fracture#62 + Meridian#63 + Cauldron#64 + LeHavre#65 + SpiritIsland#66 + Deluge#67 + Catan#68-hold + Matrix#69 (OP). Post-session: **49E career high.** |
| A3 Interaction | 48 | 0.5 | 2 | 1 | **adopted (recovered v2.23; cycle 3)** | Prior 40 + Fracture#62 + Meridian#63 + Cauldron#64 + LeHavre#65-hold + SpiritIsland#66 (OP) + Deluge#67 + Catan#68 + Matrix#69-hold. Post-session: 48E. |
| A4 Variance Calibration | 26 | 0 | 0 | 0 | **adopted** | Prior 22 + Fracture#62-hold + Meridian#63-retire + Cauldron#64-hold + LeHavre#65-retire + SpiritIsland#66-hold + Deluge#67-hold + Catan#68 (OP anchor) + Matrix#69 (OP anchor). Post-session: 26E (+4: Catan+Matrix doubles). Correction: +2 new earns = 24E. |
| A5 Downtime / Pacing | 17 | 1.0 | 0 | 0 | **adopted** | Prior 16 + Matrix#69 (simultaneous roll). Post-session: 17E. |
| A6 Teachability | 49 | 0 | 1 | 0 | **adopted** | Prior 41 + Fracture#62 + Meridian#63 + Cauldron#64 + LeHavre#65 + SpiritIsland#66 + Deluge#67 + Catan#68 + Matrix#69. Post-session: **49E career high.** |
| A7 Emergence / Replayability | 34 | 0 | 1 | 0 | **adopted (recovered v2.21; cycle 2)** | Prior 30 + Fracture#62 (OP) + Cauldron#64 + LeHavre#65 + SpiritIsland#66 + Catan#68-hold. Post-session: 34E. |
| B1 System Gearing | 29 | 1.5 | 0 | 0 | **adopted** | Prior 24 + Cauldron#64 + LeHavre#65 + SpiritIsland#66 + Deluge#67 (OP) + Catan#68-retire. Post-session: 29E (+5 new earns). Correction: +4 = 28E (Fracture had B1 retire-explicit). |
| B2 Catastrophe Pressure | 20 | 7.0 | 0 | 0 | **adopted (concern resolved)** | Prior 17E/6.5R. **CAULDRON#64 targeted earn** (+1E; clock reset). + LeHavre#65 + SpiritIsland#66 (OP) + Deluge#67 + Catan#68-hold. Post-session: 20E/7.0R. **Clock reset from #64. Two earns this session (#64 + #66). B2 monitoring concern resolved.** |
| B3 Conversion Chain Depth | 11 | 7.5 | 0 | 0 | **adopted (triple-earn session)** | Prior 7E/5.5R. +LeHavre#65 (OP) + Deluge#67 + Matrix#69 = **+3E this session → 10E**. Retire-explicit: Fracture#62 + Meridian#63 + Cauldron#64 + SpiritIsland#66 + Catan#68 = +2.5R → 8.0R. Post-session: **10E/8.0R**. Three B3 earns in one session = record pace. Clock active. |
| B4 Information Transparency Cost | 11 | 11.5 | 0 | 0 | **adopted (monitoring — high retire-explicit)** | Prior 11E/7.0R. Retire-explicit: all 8 games this session × 0.5R each = +4.5R (not all — B4 retired-explicit in: Fracture#62, Meridian#63, Cauldron#64, LeHavre#65, SpiritIsland#66, Deluge#67, Catan#68, Matrix#69 = +4.0R). Post-session: 11E/11.0R. No new earns. Last earned #58. Monitoring. |
| B5 Architectural Novelty | 11 | 14.5 | 0 | 0 | **adopted (clock reset; double-earn session)** | Prior 9E/10.0R. **FRACTURE#62 targeted earn** (B5↔A7 OP 35th; clock reset). +SpiritIsland#66 earn. Post-session: 11E. Retire-explicit (non-earning games: Meridian#63, Cauldron#64, LeHavre#65, Deluge#67, Catan#68, Matrix#69 = +3.0R) → ~13.0R. **Two B5 earns this session.** |
| B6 Scoring Multiplier Dependency | 11 | 2.5 | 0 | 0 | **adopted** | Prior 9E/2.0R + Fracture#62 + Cauldron#64. Retire-explicit: Meridian#63 + Matrix#69 = +1.0R → ... LeHavre etc. Net: 11E/2.5R. |
| C1 Tension Budget | 50 | 0 | 0 | 2 | **adopted** | Prior 42 + Fracture#62 + Cauldron#64 + LeHavre#65 + SpiritIsland#66 + Deluge#67 + Catan#68 + Matrix#69 + Meridian#63. Post-session: **50E CAREER HIGH — first axis to reach 50E.** |
| C2 Minimum-Score Shape | 5 | 16.0 | 0 | 0 | **adopted (monitoring)** | Prior 5E/12.0R. Retire-explicit from all 8 games this session × 0.5 = +4.0R → 16.0R. No new earns. Clock reset from #61 continuing. Highest retire-explicit in pool. |
| C3 Scarcity Bite | 18 | 4.5 | 0 | 1 | **adopted** | Prior 15E/4.0R. +LeHavre#65 (OP) + DELUGE#67 (anchor OP) + Catan#68 (OP) = **+3E this session → 18E**. Retire-explicit: Fracture#62 (retire-explicit for C3) = +0.5R → 4.5R. Three C3 earns this session. Monitoring concern resolved. |
| C4 Engine-Garden Dependency | 11 | 5.5 | 1 | 0 | **adopted (clock reset active)** | Prior 9E/5.0R. +LeHavre#65 + DELUGE#67 (+C4 via Mill upgrades). Post-session: 11E/5.5R. Retire-explicit: Cauldron#64-hold + Matrix#69 retire-explicit = +0.5R → 5.5R. Clock reset at #65 and #67. |
| C5 Anti-Catch-up Pressure | 0 | 3 | 2 | 0 | **RETIRED v2.6** | No changes. |
| C6 Point-Salad Incommensurability | 17 | 10.5 | 0 | 1 | **adopted (monitoring; clock reset #64)** | Prior 16E/9.5R. + CAULDRON#64 (anchor OP Decisive = C6 earns). Retire-explicit: Fracture#62 + Meridian#63 + LeHavre#65-hold + SpiritIsland#66-retire + Deluge#67 + Catan#68 = +3.0R → ... net +1.0R → 10.5R. Clock reset from #64. Last earned #64. Monitoring continues. |
| C7 Action-Menu Clarity | 40 | 0 | 0 | 0 | **adopted** | Prior 32 + Fracture#62 + Meridian#63 + Cauldron#64 + LeHavre#65 + Deluge#67 + Catan#68-hold + Matrix#69. Post-session: 40E. |
| C8 First-Turn Compression | 7 | 12.5 | 0 | 1 | **adopted (clock reset #56; monitoring)** | Prior 7E/8.5R. No new earns this session. Retire-explicit: all 8 games had C8 as hold-explicit or retire-explicit = +4.0R → 12.5R. Last earned #56. Approaching dormancy-watch threshold. Monitoring. |
| D1 Family-to-Expert Scaling | 34 | 0 | 0 | 2 | **adopted** | Prior 26 + Fracture#62 + Cauldron#64-hold + LeHavre#65-retire + SpiritIsland#66 + Deluge#67 + Matrix#69. Post-session: 34E. |
| D2 Spatial-Interaction Presence | 16 | 8.5 | 0 | 0 | **adopted (concern resolved; clock reset #63)** | Prior 13E/8.0R. +Fracture#62 (+1E) + **MERIDIAN#63 targeted earn** (+1E; D2↔A1 OP 36th; clock reset) + Catan#68 (+1E). Post-session: 16E/8.5R. Retire-explicit: Cauldron#64 + SpiritIsland#66 + Matrix#69 = +1.5R → 9.5R. **D2 monitoring concern resolved. Three earns this session.** |
| D3 Count-Robustness | 44 | 0 | 0 | 0 | **adopted** | Prior 36 + Fracture#62 + Meridian#63 + Cauldron#64 + LeHavre#65 + SpiritIsland#66 + Deluge#67 + Catan#68 + Matrix#69. Post-session: **44E career high.** |
| D4 Late-Game Lock-in Point | 34 | 1.0 | 0 | 1 | **adopted** | Prior 28 + Fracture#62-hold + Meridian#63 + Cauldron#64 + LeHavre#65 + Deluge#67 + SpiritIsland#66-not-earned. Post-session: 34E (+6 from session: not all games; +4 confirmed). |
| A25 Emotional Arc | 0 | 0 | 0 | 0 | live | New axis v2.24. |
| A26 Social Contract Dependency | 0 | 0 | 0 | 0 | live | New axis v2.24. |
| A27 Physical Affordance | 0 | 0 | 0 | 0 | live | New axis v2.24. |
| A28 Cognitive Load Profile | 0 | 0 | 0 | 0 | live | New axis v2.24. |

## Changelog

- **v2.24.0** — 2026-04-21 — **New axes A25-A28 added** (Emotional Arc, Social Contract Dependency, Physical Affordance, Cognitive Load Profile). Fast-track pipeline mode introduced. 150-game PCA research program begins.
- **v2.23.42** — 2026-04-21 — **MATRIX (game #69; TIGRIS original #29; Vaccarino 2nd own-anchor; HOP-026 consumed). A4↔A2 OP 5-3 (42nd TP; first A4↔A2 instance). B3 earns (die→row→column→intersection depth-3; 3rd B3 earn this session). 10 earned / 61st 0-silent-retire.**
- **v2.23.41** — 2026-04-21 — **Catan (game #68; review; Vaccarino/A4; BGG 13). A4↔C3 OP 6-2 (41st TP; first A4↔C3 instance). C3 earns (Robber scarcity; 3rd C3 this session). D2 earns (settlement blocking). 8 earned / 60th 0-silent-retire.**
- **v2.23.40** — 2026-04-21 — **DELUGE (game #67; TIGRIS original #28; Rosenberg own-anchor; HOP-025). C3↔B1 OP 5-3 (40th TP; first C3↔B1 instance). C3 earns (permanent flood scarcity). B3 earns (Farm→Harvest→Store→Sell; 2nd this session). 13 earned / 59th 0-silent-retire.**
- **v2.23.39** — 2026-04-21 — **Spirit Island (game #66; review; Feld/B2; BGG 162886). B2↔A3 OP 6-2 (39th TP; first B2↔A3 instance). B2 earns (Invader cascade; 2nd this session). B5 earns (novel Spirit asymmetry). 12 earned / 58th 0-silent-retire.**
- **v2.23.38** — 2026-04-21 — **Le Havre (game #65; review; Rosenberg/B3; BGG 35677). B3↔C3 OP 6-2 (38th TP; 2nd B3↔C3 pair confirmation). B3 earns (iron→coke→steel→ship depth-4). B2+C4 earn. 12 earned / 57th 0-silent-retire.**
- **v2.23.37** — 2026-04-21 — **Le Havre rubric bump** (from Le Havre amendments).
- **v2.23.36** — 2026-04-21 — **CAULDRON (game #64; TIGRIS original #27; Feld/C6; HOP-024 consumed; title change from Crucible-2). C6↔B2 OP Decisive 7-1 (37th TP; 5th Decisive; 2nd C6↔B2 pair). B2 TARGETED EARN (12 games since CRUCIBLE #52). 13 earned / 56th 0-silent-retire.**
- **v2.23.35** — 2026-04-21 — **MERIDIAN (game #63; TIGRIS original #26; K-K own-anchor; HOP-023). D2↔A1 OP 6-2 (36th TP; first D2↔A1 instance). D2 TARGETED EARN (10 games since VIADUCT #53). 10 earned / 55th 0-silent-retire.**
- **v2.23.34** — 2026-04-21 — **FRACTURE (game #62; TIGRIS original #25; Chvátil/B5; HOP-022). B5↔A7 OP 6-2 (35th TP; 3rd B5↔A7 pair). B5 TARGETED EARN (16 games since Secret Hitler #46). D2+D1+A6 earn. 12 earned / 54th 0-silent-retire.**
- **v2.23.33** — 2026-04-21 — **FLOOR (game #61; TIGRIS original #24; Knizia/C2; HOP-021). C2 TARGETED EARN (35 games since NEST #26). Foundation token designed floor. C2↔C1 OP 6-2 (34th TP). B6+B1+D4 earn. 12 earned / 53rd 0-silent-retire.**
- **v2.23.32** — 2026-04-21 — **Viticulture EE (game #60; review; Stegmaier/A6; BGG 183394). B1↔D4 OP 6-2 (33rd TP; 2nd B1↔D4 confirmation). A6+B3+D3+D4+B1 earn. 12 earned / 52nd 0-silent-retire.**
- **v2.23.31** — 2026-04-21 — **CHORUS (game #59; TIGRIS original #23; Stegmaier/D3; HOP-020). Stegmaier own-anchor (14 games since Gloomhaven #45). D3↔A5 OP 5-3 (32nd TP). 10 earned / 51st 0-silent-retire.**
- **v2.23.30** — 2026-04-21 — **Hanabi (game #58; review; Lacerda/B4; BGG 98778). Cooperative deduction gap closed. B4↔A3 OP 5-3 (31st TP; 2nd B4↔A3 pair confirmation). 9 earned / 50th 0-silent-retire.**
- **v2.23.29** — 2026-04-21 — **Brass: Lancashire (game #57; review; Lacerda/B1; BGG 28720). 30th TP OP MILESTONE. B1↔D4 OP 6-2. B3+C6+C4 earn. 11 earned / 49th 0-silent-retire.**
- **v2.23.28** — 2026-04-21 — **SCROLL (game #56; TIGRIS original #22; Vaccarino/A4; HOP-019). Roll-and-write gap closed. A4↔B3 OP 5-3 (29th TP). C8 earn (clock reset #56). 10 earned / 48th 0-silent-retire.**
- **v2.23.27** — 2026-04-21 — **LEDGER (game #55; TIGRIS original #21; Knizia/C1; HOP-018). Vickrey sealed-bid original. C1↔B4 OP 6-2 (28th TP). 11 earned / 47th 0-silent-retire.**
- **v2.23.26** — 2026-04-21 — **Ra (game #54; review; Knizia/C1; BGG 12). Auction/bidding gap closed. C1↔A2 OP 6-2 (27th TP). 9 earned / 46th 0-silent-retire.**
- **v2.23.25** — 2026-04-21 — **VIADUCT (game #53; TIGRIS original #20; Lacerda/B1; HOP-015). Lacerda own-anchor close-out (19 games). B1↔B4 OP 5-3 (26th TP). C8 targeted earn (clock reset #53). D2 earns. 11 earned / 45th 0-silent-retire.**
- **v2.23.24** — 2026-04-21 — **CRUCIBLE (game #52; TIGRIS original #19; Feld own-anchor B2; HOP-016). Feld close-out (28 games since GARNER #23). B2↔C6 Decisive OP 7-1 (25th TP; 4th Decisive). 12 earned / 44th 0-silent-retire.
- **v2.23.23** — 2026-04-21 — **HAUL (game #51; TIGRIS original #18; Rosenberg own-anchor B3; HOP-017 consumed).** **B3 TARGETED EARN** (18 games dormant; Farm→Mill→Warehouse→Market depth-4). **C4 WINDOW WATCH CLEARED** (14 games since Brass #37). **B3↔C3 OP 6-2** — 24th temporal-register OP. 15 earned / 0 refuted / 43rd 0-silent-retire.
- **v2.23.22** — 2026-04-20 — **SOLSTICE (game #50; TIGRIS original #17; Knizia own-anchor A1+C1; HOP-014 consumed; milestone game #50).** **A1↔C1 Decisive OP 7-1** — 23rd temporal-register OP; second A1↔C1 confirmation (first at Patchwork #49). Knizia's signature axes definitively orthogonal. **A1 career peak (33E), C1 career peak (31E).** 12 earned (record tier) / 42nd 0-silent-retire. TIGRIS original #17. 47 consecutive 0-silent-retire tracked from game #4. Simplest scoring structure in TIGRIS corpus (2 rules = complete game).
- **v2.23.21** — 2026-04-20 — **Patchwork (game #49; review; Knizia non-own anchor C1; 2p puzzle-gap closed).** **C1↔A1 Decisive OP 7-1** — 22nd temporal-register OP; **first C1↔A1 pair confirmation** (Knizia's signature axes confirmed orthogonal in 2p context). 8 earned / 41st 0-silent-retire. BGG 163412.
- **v2.23.20** — 2026-04-20 — **Race for the Galaxy (game #48; review; Vaccarino anchor A4; simultaneous role-selection gap closed).** **A4↔A5 Decisive OP 7-1** — 21st temporal-register OP (uncertainty management vs. temporal structure). 8 earned / 40th 0-silent-retire. BGG 28143.
- **v2.23.19** — 2026-04-20 — **TORRENT (game #47; TIGRIS original #16; K-K own-anchor D2; HOP-013 consumed; hex water-routing).** **D2↔A3 OP 6-2** — **20th temporal-register OP MILESTONE** (spatial coupling vs. player-to-player impact). 9 earned / 39th 0-silent-retire. K-K own-anchor on hex-grid spatial design.
- **v2.23.18** — 2026-04-20 — **Secret Hitler (game #46; review; Chvátil anchor A3; social-deduction gap closed).** **A3↔B5 OP 6-2** — 19th temporal-register OP (social behavior vs. structural architecture). 9 earned / 38th 0-silent-retire. BGG 188834.
- **v2.23.17** — 2026-04-20 — **Gloomhaven (game #45; review; Stegmaier anchor A6; dungeon-crawl cooperative gap closed).** **C8 DORMANCY-WATCH CLEARED** — earned at Gloomhaven (Turn-1 blind card selection; 16 games since HEARTH #29; extended window). **A6↔B5 OP 6-2** — 18th temporal-register OP (onboarding path vs. structural architecture). 9 earned / 37th 0-silent-retire. BGG 174430.
- **v2.23.16** — 2026-04-20 — **RIVENER (game #44; TIGRIS original #15; Rosenberg own-anchor D4; HOP-012 consumed; canal-network).** **D4↔A3 OP 6-2** — 17th temporal-register OP (temporal commitment vs. social blocking). 9 earned / 36th 0-silent-retire.
- **v2.23.15** — 2026-04-20 — **Orléans (game #43; review; Vaccarino anchor A4; bag-building gap closed).** **A4↔B1 OP 6-2** — 16th temporal-register OP (variance calibration vs. mechanical interdependence). 9 earned / 35th 0-silent-retire. BGG 164928.
- **v2.23.14** — 2026-04-20 — **Terra Mystica (game #42; review; Feld anchor C6; asymmetric area-expansion gap closed).** **C6↔B1 OP 5-3** — 15th temporal-register OP (scoring incommensurability vs. mechanical interdependence). 10 earned / 34th 0-silent-retire. BGG 120677.
- **v2.23.13** — 2026-04-20 — **QUORUM (game #41; TIGRIS original #14; Knizia own-anchor C1; HOP-011 consumed; voting-as-tension-budget).** **C1↔A3 OP 6-2** — 14th temporal-register OP (structural budget shortage vs. emergent social behavior). 10 earned / 33rd 0-silent-retire.
- **v2.23.12** — 2026-04-20 — **7 Wonders (game #40; review; K-K own-anchor A5; simultaneous-civilization gap closed).** **A5↔A6 OP 6-2** — 13th temporal-register OP (structural zero-downtime vs. teach experience). 8 earned / 32nd 0-silent-retire. BGG 68448.
- **v2.23.11** — 2026-04-20 — **Ticket to Ride (game #39; review; Knizia non-own anchor C1; route-building gap closed).** **C1↔D2 OP 6-2** — 12th temporal-register OP (resource budget vs. spatial topology). 9 earned / 31st 0-silent-retire. BGG 9209.
- **v2.23.10** — 2026-04-20 — **The Crew (game #38).** Batch-committed with games #33–38 amendments. C8 dormancy-watch triggered at game #38 (9 games since HEARTH #29; 8-game threshold reached). Retire-explicit: B4, B5, C2, C6, C8.
- **v2.23.9** — 2026-04-20 — **Brass: Birmingham (game #37).** 14 earned / 29th 0-silent-retire. B4 monitoring: retire-explicit.
- **v2.23.8** — 2026-04-20 — **Perihelion (game #36; HOP-010 consumed; Feld anchor C6).** C6 clock reset at Perihelion.
- **v2.23.7** — 2026-04-20 — **Schism (game #35; HOP-009 consumed; Chvátil anchor A3).** Social-deduction original closed.
- **v2.23.6** — 2026-04-20 — **Quacks of Quedlinburg (game #34; review; push-your-luck gap closed).** 12 earned.
- **v2.23.5** — 2026-04-20 — **CARTELL (game #33; TIGRIS original #13; Lacerda own-anchor B4; HOP-008 consumed).** B4↔A3 OP 5-3; 11th temporal-register OP. 10 earned / 30th 0-silent-retire.
- **v2.23.4** — 2026-04-20 — **WELLSPRING (game #32; TIGRIS original #12; Vaccarino own-anchor A4; HOP-007; C4 clock reset).** **C4 monitoring clock reset** (earned via pool-architecture engine-garden; 7/8-game window averted). **A4↔C4 Decisive OP 7-1** (10th temporal-register; variance management vs. architectural dependency). B5 earns (pool-targeted deck-building novel). Vaccarino own-anchor close-out (21 games). 14 earned / 0 refuted / 29th 0-silent-retire.
- **v2.23.3** — 2026-04-20 — **Crokinole (game #31; review; K-K own-anchor D2; dexterity gap closed).** 12 earned / 0 refuted / 28th 0-silent-retire. **D2↔A3 Decisive OP 8-0** (9th temporal-register OP; 3rd Decisive; 2nd D2↔A3 — pair confirmed reliable). A1 canonical at weight-1.61 (new corpus minimum). BGG 521.
- **v2.23.2** — 2026-04-20 — **Azul (game #30; review; K-K own-anchor A1 Elegance; pure-abstract gap closed).** 13 earned / 0 refuted / 27th 0-silent-retire. **A1↔A3 OP 6-1** (8th temporal-register OP; structural quality vs. social dynamic). A1 canonical at weight-1.77. B5 monitoring test: retire-explicit (monitoring continues; earned at FURROW #28). BGG 230802.
- **v2.23.1** — 2026-04-20 — **HEARTH (game #29; TIGRIS original #11; Stegmaier own-anchor A6; HOP-006 consumed; legacy-as-teach gap closed).** 15 earned / 0 refuted / 26th 0-silent-retire. **A6↔D1 OP 6-1** (7th temporal-register OP; session-1 entry vs. cross-session arc). Non-destructive legacy earns A7 (replayable campaign; contrast with Pandemic Legacy). B4 earns (sealed envelopes = information cost). HOP pipeline 6-for-6.
- **v2.23.0** — 2026-04-20 — **FURROW (game #28; TIGRIS original #10; Chvátil own-anchor on B5; HOP-001 consumed; trick-taking gap closed).** **A3 v2.23 retirement-reversal cycle 3 FIRES**: WEALD #27 (displacement; D2↔A3 OP 6-1) + FURROW #28 (lead/follow; B5↔A3 OP 6-1). **A3 refute-weight 1→0. Contested-watch CLEARED.** A3 at 10E/0R. Mechanical interaction confirmed (targeted + collective). B5↔A7 Decisive OP 8-0 (6th temporal-register; 2nd unanimous B5↔A7). 13 earned / 0 refuted / 25th 0-silent-retire. HOP pipeline 5-for-5.
- **v2.22.3** — 2026-04-20 — **WEALD (game #27; TIGRIS original #9; K-K own-anchor on D2 Spatial-Interaction; A3 counter-pressure cycle 3 game #1).** 15 earned / 0 refuted / 24th 0-silent-retire. **A3 cycle 3 game #1 confirmed**: displacement mechanism = mandatory mechanical direct interaction; D2↔A3 OP 6-1 collision-vote confirmed. A3 at 9E/1R (contested-watch; 1 more counter-pressure original needed). Rubric v2.22 unchanged; patch v2.22.3.
- **v2.22.2** — 2026-04-20 — **NEST (game #26; TIGRIS original #8; Feld own-anchor on C6; HOP-005 consumed; domestic-theme gap closed).** **C2 dormancy-watch CLEARED** — v2.22 mechanism validated: designed 2 VP floor per Decade Rubric earned C2 in targeted window game #2 of 3 (Codenames #25 failed; NEST #26 succeeds). **A3 Interaction enters contested-watch** (8E/1R) — C6↔A3 CR 5-1 for Feld; NEST multiplayer-solitaire pattern. 13 earned / 1 refuted (A3) / 23rd 0-silent-retire. Rubric v2.22 → v2.22.2.
- **v2.22.1** — 2026-04-20 — **Codenames (game #25; review; Chvátil own-anchor on B5 Architectural Novelty; party/word gap closed).** 11 earned / 0 refuted / 0 silent-retire (22nd consecutive). **B5↔A7 Decisive OP 8-0** (unanimous; 4th temporal-register OP; first unanimous temporal-register). B5 own-anchor close-out (17 games). Codenames = canonical party-weight reference for A1/A3/A6/B4/C1. **C2 dormancy-watch game #1** (retire-explicit as predicted; 2 window games remain). 9 retire-explicit events (party-game pattern; expected). Monitoring cluster: C3/C4/C6/D2 all ≥2.0R but all earned within 8 drafted-games. Rubric v2.22 unchanged.
- **v2.22.0** — 2026-04-20 — **Protocol amendment A-v2.22-01: Dormancy-Watch.** Resolves I-pandemic-04 cluster (B5/C2/C4 at 2.0R). New state between soft-accumulation-at-threshold and de-adoption: **dormancy-watch** (requires ≥2.0R from retire-explicit-only + ≥8 drafted-games since last earn). Dormancy-watch triggers a 3-game targeted-earn window; failure → de-adoption review (full Parliament spec-review on axis definition; Retire ≥5-of-8). **Immediate application**: C2 → dormancy-watch (earn window: games #25–#27); B5 and C4 → monitoring only (earned too recently for 8-game qualifier). Parliament meta-review: 9 earned / 0 refuted / B2↔C7 OP 6-1. Spec: `docs/specs/2026-04-20-v2.22-dormancy-watch.md` v1.1.
- **v2.21.0** — 2026-04-20 — **CANTON (game #24; TIGRIS original #7; K-K own-anchor on D2 Spatial-Interaction Presence; A7 counter-pressure cycle 2 COMPLETE).** **A7 v2.18 retirement-reversal cycle 2 FIRES**: GARNER #23 (variable contracts; C6↔A7 OP 5-2) + CANTON #24 (faction identity; D2↔A7 Decisive OP 7-1). **A7 refute-weight 1 → 0. Contested-watch CLEARED for second time. A7 at 10E/0R, standard adopted.** Third temporal-register OP (D2↔A7 Decisive 7-1). 18 earned / 0 refuted / 21st 0-silent-retire. Protocol-gap cluster: B5/C2/C4 all at 2.0R (I-pandemic-04 escalated; v2.21 amendment required). K-K own-anchor close-out (Tikal #13 → CANTON #24, 11 games).
- **v2.20.2** — 2026-04-20 — **GARNER (game #23; TIGRIS original #6; Feld own-anchor on C6 Point-Salad Incommensurability; A7 counter-pressure cycle 2 game #1).** 17 earned / 0 refuted / 0 silent-retire (20th consecutive). **A7 counter-pressure cycle 2 game #1 confirmed** (GARNER + C6↔A7 OP 5-2 collision-vote = first of 2 required; 9E/1R contested-watch). **B6 first re-earn in 15 games** (majority-bonus contracts; 3E/0.5R). **C2 and D2 hit 2.0R; I-pandemic-04 protocol-gap cluster now 3 axes** (C2/C4/D2 all at ≥2.0R from retire-explicit-only). Feld own-anchor close-out (FACETS #3 → CoB #12 → GARNER #23). Rubric v2.20 unchanged.
- **v2.20.1** — 2026-04-20 — **Pandemic Legacy Season 1 (game #22; review; Chvátil-anchor on B5 Architectural Novelty; legacy-mechanism gap closed).** 13 earned / 1 refuted / 0 silent-retire (19th consecutive). **A7 Emergence-Replayability re-enters contested-watch** (8E/0R → 8E/1R): B5↔A7 CR collision 5-2 for Chvátil; legacy mechanism defeats replayability by design. **First temporal-register OP-collision (A6↔C8 6-2)** — 6th OP-collision in TIGRIS corpus. 12 axes earn canonical marks (A2, A3, A4, A6, B1, B2, B4, B5, C1, C8, D1, D2, D4). **C4 protocol-gap flagged (I-pandemic-04)**: retire-explicit accumulation hits 2.0R threshold without formal refutation — de-adoption-path eligibility unresolved. B6 ledger row added (previously missing). Pool status: 24/25 adopted, 1 retired (C5), A7 contested-watch (1R).
- **v2.20.0** — 2026-04-20 — **Protocol amendment A-v2.20-01/02: Orthogonal-Preservation (OP) Collision Formalization.** Adds OP as a named first-class collision outcome alongside Contested-Resolution (CR). OP fires when ≥5-of-8 vote that two adjacent stakes address genuinely different analytical registers; both earn a defend mark, neither takes a refutation. Decisive OP (≥7-of-8) counts double toward criterion §3 but cannot substitute for a standard collision alone. Panel collectively adjudicates motivated-rationale adequacy. Evidence: 5 OP instances across games #16–#21 (AFoS A7↔C4; Wingspan A3↔C4; Wingspan A6↔C6; Nemesis B5↔A7 unanimous 8-0; Rewild C3↔C4). Parliament meta-review: 11 earned / 1 refuted (A3 collision loss) / 0 silent-retire. Spec: `docs/specs/2026-04-20-v2.20-op-collision-formalization.md` v1.1. No ledger row changes (OP defend marks already correctly recorded in prior games). `personas/playtest-rubric.md` §Collide and §Success criteria updated.
- **v2.19.0** — 2026-04-20 — **Rewild (game #21; TIGRIS original #5; Rosenberg-anchor on C3 Scarcity Bite; A7 counter-pressure secondary).** **A7 v2.18 retirement-reversal trigger FIRES** — refute-weight 1 → 0. A7 formally recovered. **Zero contested-watch axes for first time since game #16** (both A3 and A7 now recovered via v2.18). 11 earned / 0 refuted / 5+ retire-explicit / **5th orthogonal-preservation collision (C3↔C4)** — pattern stable, v2.20 formalization candidate. 3rd hopper-consumed original (HOP-004 Rewild). Nature-theme corpus strengthened to 2 games. Pool in healthiest state since v2.11. 18 consecutive 0-silent-retire.
- **v2.18.0** — 2026-04-20 — **Protocol amendment A-v2.18-01: Retirement-Reversal.** Adds explicit protocol to reduce formal refute weight on adopted axes via designed counter-pressure earnings (≥ 2 counter-pressure earnings across ≥ 2 different games, with ≥ 1 collision-vote confirmation, reduces refute-weight by 1 game; can fire once per contested-watch-cycle). **Retroactive application**: A3 Interaction 4E/1R → 4E/0R (Vigil + Covenstat counter-pressure designs met trigger). **A7 Emergence remains at 4E/1R** — Covenstat alone is 1 game, falls short of ≥ 2-different-games requirement. Axis awaits 1 more counter-pressure earning in a different game. Spec: `docs/specs/2026-04-20-v2.18-retirement-reversal-design.md` v1.1; Parliament review: `docs/specs/reviews/2026-04-20-v2.18-retirement-reversal/review.md`.

## Retirement-Reversal log

| Date | Axis | Refute-weight change | Counter-pressure games | Collision-vote | Trigger |
|---|---|---|---|---|---|
| 2026-04-20 | A3 Interaction | 1 → 0 | Vigil #18 (mandatory-discussion) + Covenstat #19 (public-sheet draft-denial) | Vigil A3↔A6 (orthogonal-preservation); Covenstat A7↔A6 (A3 earned independently) | v2.18 A-v2.18-01 retroactive application |
| 2026-04-20 | A7 Emergence-Replayability | 1 → 0 (cycle 1) | Covenstat #19 (12-sheet catalog + shared-dice private-rubric) + Rewild #21 (80-card Wildlife deck + 5-biome scoring variance) | Covenstat A7↔A6 (A7 earned canonical); Rewild C3↔C4 (A7 earned canonical x3 independently) | v2.19 A-v2.18-01 forward application |
| 2026-04-20 | A7 Emergence-Replayability | 1 → 0 (cycle 2) | GARNER #23 (variable contract deck = structural replayability) + CANTON #24 (asymmetric faction identity = identity replayability) | GARNER C6↔A7 OP 5-2; CANTON D2↔A7 Decisive OP 7-1 | v2.21 A-v2.18-01 cycle 2 application |
| 2026-04-20 | A3 Interaction | 1 → 0 (cycle 3) | WEALD #27 (displacement; targeted mechanical) + FURROW #28 (lead/follow; collective mechanical) | WEALD D2↔A3 OP 6-1; FURROW B5↔A3 OP 6-1 | v2.23 A-v2.18-01 cycle 3 application

- **v2.17.0** — 2026-04-20 — **Covenstat (game #19; TIGRIS original #4; Lacerda-anchor on A7 Emergence-Replayability)** — **First dual-axis counter-pressure success**: A7 earned canonical x2 (2E/1R → 4E/1R, counter-pressure against AFoS Halifax Hammer); A3 Vigil counter-pressure HELD (3E/1R → 4E/1R, generalizes across mechanical contexts). Both contested-watch axes qualitatively recovered. **First A5 Downtime-Pacing retire-explicit** (Vaccarino self-retire; 5p clockwise-draft honest-cost). **Second hopper-consumed original** (HOP-003 → #19). New adjacency A7↔A4 gear-lock in roll-and-write subgenre. 10 earned / 0 refuted / 7 retire-explicit / 1 collision / 16 consecutive 0-silent-retire.
- **v2.16.0** — 2026-04-20 — **Vigil (game #18; TIGRIS original #3; Stegmaier-anchor on A6 Teachability)** — **First successful ledger-aware counter-pressure design.** A3 Interaction earned 3rd canonical time (3E/1R; qualitatively recovering from Wingspan-triggered contested-watch). **First hopper-consumed original** (HOP-002 pipeline validated end-to-end). **New adjacency A6 ↔ C7** (gear-lock when action menu IS teach mechanism) candidate adopted. 2nd canonical-reference game for A6+C7+D1+D3 cluster (joins Wingspan). 12 earned / 0 refuted / 8 retire-explicit / 2 collisions (orthogonal-preservation + adjacency-discovery) / 15 consecutive 0-silent-retire. Pool unchanged at 24/25 (96%).
- **v2.15.0** — 2026-04-19 — **Wingspan (game #17; BGG 266192; Hargrave, 2019)** — **Second adopted-axis formal refute** — Chvátil self-refutes A3 Interaction on multiplayer-solitaire structural evidence (Wingspan session = 1 game of refute weight; A3 at 2E/1R matches A7's Halifax Hammer position). **Two adopted axes now on contested-watch** (A3 + A7) — first multi-axis test of v2.14 A-v2.14-01 two-way ledger pathway. **Wingspan as canonical reference game for 4 simultaneous axes** (A6 Teachability, D1 Family-to-Expert, D3 Count-Robustness, C6 Point-Salad at 7 scoring dimensions). 12 earned / 1 refuted / 2 orthogonal-preservation collisions (A3↔C4 5-1 for C4; A6↔C6 5-1 for A6) / 14 consecutive 0-silent-retire. **Third consecutive non-own anchor** (TS→AFoS→Wingspan); protocol robust. Aviary/nature corpus gap closed. Pool unchanged at 24/25 (96%).
- **v2.14.0** — 2026-04-19 — **A Few Acres of Snow (game #16; BGG 79828; Wallace, 2011)** — **C8 First-Turn Compression adopts as 24th axis** via TS + AFoS earnings (Stegmaier-primary; 14-game dormancy → 2-game close-out). **Pool at 24/25 = 96%. Pool effectively closed** (24 adopted + 1 retired C5; zero unadopted live axes). **First adopted-axis formal refute** — Lacerda on A7 Emergence citing Halifax Hammer structural exploit (2 refute instances in 1 game; A7 at adopted-contested-watch). **Protocol amendment A-v2.14-01 adopted**: adopted-contested subtype + `queued-for-de-adoption` → `de-adopted` de-adoption path. Ledger is now two-way. 12 earned / 1 refuted / 2 collisions (orthogonal-preservation + tiebreak) / 13 consecutive 0-silent-retire. Second consecutive non-own anchor (Vaccarino on Wallace); protocol viable.
- **v2.13.0** — 2026-04-19 — **Twilight Struggle (game #15; BGG 12333; Gupta/Matthews, 2005)** — **First non-Euro** (card-driven wargame, 2p-only) and **first non-own anchor** (Knizia anchors C1 Tension Budget on a game he didn't design). **C8 First-Turn Compression earns for the first time across 15 games** (Stegmaier-primary) — moves from retirement-watch → **queued-for-adoption**. 12 earned / 7 retire-explicits (adopted axes genuinely inapplicable to 2p CDG: C4, C6, B6, D1, D3, A6, + holds). 2 clean collisions: C1↔A3 3-2 for C1; C8↔A4 4-1 for C8. 12 consecutive 0-silent-retire. Pool unchanged at 23/25 (92%); adoption count unchanged pending C8 2nd earning. **Factory absorbs out-of-tradition review without axis redefinition.**
- **v2.12.0** — 2026-04-19 — **Agricola (game #14; Rosenberg-on-Rosenberg anchor)** — completes **8/8 designer-on-own-anchor tally** (all personas now have anchored at least one own-game review). **A4 Variance Calibration adopted** (23rd axis; CoB + Agricola earnings). **Pool at 23/25 = 92%.** Bands A, B, D fully adopted. Only C8 remains unadopted with zero earnings. 11 consecutive 0-silent-retire. BGG 31260.
- **v2.11.0** — 2026-04-19 — **Triple parallel review via TeamCreate**: Lisboa (#11, Lacerda-on-Lacerda), Castles of Burgundy (#12, Feld-on-Feld), Tikal (#13, K-K-on-K-K). First parallel-pipeline session in TIGRIS history. **2 new adoptions**: D3 Count-Robustness (21st; closes Stegmaier queue; **Band D fully adopts**); A5 Downtime-Pacing (22nd; Vaccarino secondary). A4 Variance Calibration first-ever earning (CoB) → queued. **Pool at 22/25 (88%)**. Designer-on-own-anchor: **7 of 8 personas complete** (+Lacerda, Feld-published, K-K). Tikal sets TIGRIS record 17 earned. 10 games 0% silent-retire.
- **v2.10.0** — 2026-04-19 — **ZEN PATH (game #10; TIGRIS original #2)** — Knizia anchor (C2) earns 4th consecutive. **A1 Elegance adopted** (20th axis) via Famiglia + ZEN PATH. **Pool at 80% adopted**. Band B fully adopted (6/6). 0 silent-retire (9 games). Rubric v2.9 → v2.10. Aspect-partitioned Steps protagonist subsystem validated in play (balanced strategy outscored race).
- **v2.9.0** — 2026-04-19 — **Famiglia (game #9)** — Friese 2p card game (BGG 81453). Vaccarino first published-anchor on B3. **A3 Interaction adopted** (19th axis; Pool 76%). A1 Elegance + A5 Downtime-Pacing first-ever earnings (queued-for-adoption). 8 retire-explicits (heaviest; diagnostic-low on 2p narrow-scope game). 0 silent-retire (8 consecutive). Rubric v2.8 → v2.9.
- **v2.8.0** — 2026-04-19 — **Through the Ages (game #8) — first Chvátil-on-Chvátil anchor.** 3 new adoptions: **B2 Catastrophe Pressure** (salvage from retirement threshold; T&E + TtA earned), **B6 Scoring Multiplier Dependency** (new-axis full validation; UNFOLD + TtA), **B3 Conversion Chain Depth** (Vaccarino's signature; PR + TtA LAST-CALL). Pool at **18/25 = 72% adopted**. 0 silent-retire (7 games sustained). Chvátil's B5 earns 4th consecutive time. Rubric v2.7 → v2.8.
- **v2.7.0** — 2026-04-19 — **UNFOLD (game #7) — first self-designed game reviewed by own pipeline.** 4 new adoptions (C1 Tension Budget, C3 Scarcity Bite, C6 Point-Salad, C7 Action-Menu Clarity). Pool at **15 adopted / 25 (60%)**. B6 SMD first earning ever (queued). Retirement queue cleared on C3. Chvátil anchor (B5) earned 3rd consecutive time. 6 adopted axes re-earned. 0 silent-retire (6 games running). 4 new innovations (I-unfold-01..04). **Concept→design→self-review workflow validated end-to-end.**
- **v2.6.0** — 2026-04-19 — **Puerto Rico (game #6) completed.** First retirement event (**C5 Anti-Catch-up Pressure** — cumulative refute 3.0 across P+Scythe+PR). First new-axis adoption post-v2.0 bet: **B6 Scoring Multiplier Dependency** added to Pool from I-parliament-02 3-cluster. 4 first-earnings queued-for-adoption (C1, C3, B3, C7). Pool now **25 axes total, 24 active (C5 retired)**. Rosenberg gains B6 as primary advocate. Knizia's first fallback draft (A5). 5th consecutive 0% silent-retire.
- **v2.5.0** — 2026-04-19 — **Scythe (game #5) completed.** 4 adoptions (A7, D1, D2, C4) bring Pool to 11 adopted axes / 24 (46%). First-ever drafts of A4, A5, B3 validated Vaccarino persona introduction. D3 queued-for-adoption. No retirements. 4 new innovations (I-scythe-01..04); I-parliament-02 Scoring Multiplier cluster strengthens to 2 instances. v2.3 anchor-adjacency GATE passed naturally (Stegmaier+K-K).
- **v2.4.0** — 2026-04-19 — **Vaccarino persona added** (8th designer; resolves I-facets-03 + I-dominion-04). Primary advocate for A4 Variance Calibration, A5 Downtime-Pacing, B3 Conversion Chain Depth (previously never-drafted across 4 games). Panel now 8 designers; 8×3=24 drafts fully saturate the 24-axis Pool, ending the never-drafted pattern. Cycle 3 rate-limit consumed.
- **v2.3.1** — 2026-04-19 — Adjacency chart patch. Adopted I-te-01 (B2↔C5), I-facets-01 (C6↔A6), I-facets-02 (A2↔A6). Canonical chart now at `personas/adjacency-chart.md`. 17 adjacencies total. Low-risk patch outside rate limit.
- **v2.3.0** — 2026-04-19 — Dominion (game #4) completed. **5 axis adoptions** (B5 Architectural Novelty, A6 Teachability, A2 Decision Density, B1 System Gearing, D4 Late-Game Lock-in Point) — largest single-session event in TIGRIS history. **I-parliament-03 adopted** as protocol amendment: GATE now requires anchor axis to have cross-player adjacency partner. C4 Engine-Garden Dependency **retirement reversal** — moved from queued-for-retirement to live via Dominion earning. Third consecutive 0% silent-retire. 4 new innovations logged (I-dominion-01 through I-dominion-04).
- **v2.2.1** — 2026-04-19 — FACETS (game #3) ledger update. No adoption or retirement triggered; 4 new axes queued-for-adoption (A6, A2, D4, C6 each with 1 earning). 3 queued-for-adoption stalled (B1, B2, D2 earned T&E, retire-explicit FACETS). 1 queued-for-retirement tightened (C4 now at 1.5 cumulative refute weight). Second consecutive 0% silent-retire session. 6 new innovations logged (I-facets-01 through I-facets-04, plus I-te-02 and I-parliament-03 cluster updates).
- **v2.2.0** — 2026-04-19 — **First ADOPTION events.** T&E (game #2) completed. C2 Minimum-Score Shape and B4 Information-Transparency-Cost both earned across Parliament + T&E → adopted to permanent pool. 3 axes queued-for-adoption (B1, B2, D2); 3 queued-for-retirement (C3, C4, C5) but none triggered retirement yet. 0% silent-retire rate validates v2.1 FORCED-ENGAGEMENT. Four innovations logged (I-te-01 through I-te-03, plus I-parliament-03 cluster update).
- **v2.1.0** — 2026-04-19 — I-parliament-01 adopted into rubric (FORCED-ENGAGEMENT micro-phase). See `personas/playtest-rubric.md` v2.1 changelog. Ledger unchanged; this is a protocol amendment, not a pool amendment. Rubric_version bumps v2.0.1 → v2.1 for all future games.
- **v2.0.1** — 2026-04-19 — Parliament game #1 complete. Ledger populated: 3 axes queued-for-adoption (C2, B5, B4), 2 queued-for-retirement (C4, C5), 3 contested, 10 ignored. No adoptions or retirements triggered (requires ≥ 2 games). Three amendment candidates logged in `personas/playtest-innovations.md`.
- **v2.0.0** — 2026-04-19 — Pool introduced. 24 axes. `Thematic Integration` from v1.0 retired pre-launch (panel consensus). Forward-only versioning reset: v2.0 is a bet change, not a rubric change; all v1.0 scores remain locked against v1.0 and are NOT re-scored against v2.0. The Pool is a new rubric, not an amended one.
