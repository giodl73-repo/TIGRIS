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

---

## Rubric Ledger (live state)

As games complete TIER-C, this section records adoption/retirement history. **Parliament (game #1) complete — 21 axes have first-game records.** Thresholds: ≥ 2 earned across ≥ 2 games → `adopted`; ≥ 2 refuted across ≥ 2 games → `retired`. Game #2 (expected: Tigris & Euphrates) will trigger first adoptions/retirements for queued axes.

Columns: **Earned / Refuted / Contested / Ignored** counts are cumulative across games (value = game-count in that state).

| Axis | Earned | Refuted | Contested | Ignored | Status | Note |
|---|---:|---:|---:|---:|---|---|
| A1 Elegance | 0 | 0 | 2 | 0 | live | v1.0 carry; contested P + Dominion |
| A2 Decision Density | 2 | 0 | 0 | 1 | **adopted** | FACETS earned + Dominion earned (LAST-CALL). |
| A3 Interaction | 0 | 1 | 2 | 1 | live | Self-refuted FACETS; contested P+Dominion |
| A4 Variance Calibration | 0 | 0 | 0 | 0 | live (advocate: Vaccarino) | Scythe first draft ever (Vaccarino 1d — weak-defended). |
| A5 Downtime / Pacing | 0 | 0 | 0 | 0 | live | Scythe first draft ever (Feld fallback 1d — weak-defended). |
| A6 Teachability | 2 | 0 | 1 | 0 | **adopted** | FACETS earned + Dominion earned (LAST-CALL). |
| A7 Emergence / Replayability | 2 | 0 | 1 | 0 | **adopted** | Dominion + Scythe earned. 8th adoption event (v2.5). |
| B1 System Gearing | 0 | 0 | 0 | 0 | live | weak-defended in Parliament (1 defend, not 2) |
| B2 Catastrophe Pressure | 0 | 0 | 0 | 1 | live | ignored in Parliament |
| B3 Conversion Chain Depth | 0 | 0 | 0 | 0 | live | undrafted in Parliament (Knizia passed) |
| B1 System Gearing | 2 | 0.5 | 0 | 0 | **adopted** | T&E earned + Dominion earned (LAST-CALL). FACETS retire-explicit doesn't block. |
| B2 Catastrophe Pressure | 1 | 1.0 | 0 | 1 | live (contested cross-game) | Earned T&E; retire-explicit FACETS + Dominion. |
| B3 Conversion Chain Depth | 1 | 0 | 0 | 0 | queued-for-adoption | **PR first earning** (Vaccarino, 2 defends). Corn→Indigo→Sugar→Tobacco→Coffee chain canonical. |
| B4 Information Transparency Cost | 2 | 0 | 0 | 0 | **adopted** | Still-adopted; Dominion hold-explicit. |
| B5 Architectural Novelty | 2 | 0.5 | 0 | 0 | **adopted** | P earned + Dominion earned (anchor vindication). T&E retire-explicit doesn't block. |
| C1 Tension Budget | 1 | 0 | 0 | 2 | queued-for-adoption | Puerto Rico first earning (Knizia anchor, 3 defends). Dormant 5 games prior. |
| C2 Minimum-Score Shape | 3 | 0.5 | 0 | 0 | **adopted** | Adopted; Dominion retire-explicit (diagnostic-low) adds minor refute |
| C3 Scarcity Bite | 1 | 1 | 0 | 1 | queued-for-adoption | **PR first earning** (3 defends) — reverses queued-for-retirement. Mixed history. |
| C4 Engine-Garden Dependency | 2 | 1.5 | 1 | 0 | **adopted** | Dominion + Scythe earned. 11th adoption (v2.5). Retirement reversal permanently confirmed. |
| C5 Anti-Catch-up Pressure | 0 | 3 | 2 | 0 | **RETIRED v2.6** | **First retirement event.** PR retire-explicit + self-refute. Cumulative weight 3.0. |
| C6 Point-Salad Incommensurability | 1 | 1.0 | 0 | 1 | live (contested) | Dominion retire-explicit adds refute to FACETS earning |
| C7 Action-Menu Clarity | 1 | 0 | 0 | 0 | queued-for-adoption | PR LAST-CALL earning (K-K, 9/9 roles-clear). |
| C8 First-Turn Compression | 0 | 0.5 | 0 | 1 | live | Dominion hold-explicit |
| D1 Family-to-Expert Scaling | 2 | 0 | 0 | 2 | **adopted** | Dominion + Scythe (LAST-CALL) earned. 9th adoption (v2.5). |
| D2 Spatial-Interaction Presence | 2 | 1.0 | 0 | 1 | **adopted** | T&E + Scythe earned. 10th adoption (v2.5). |
| D3 Count-Robustness | 1 | 0 | 0 | 0 | queued-for-adoption | Scythe first earning (Stegmaier anchor). |
| D4 Late-Game Lock-in Point | 2 | 0 | 0 | 1 | **adopted** | FACETS earned + Dominion earned. |

## Changelog

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
