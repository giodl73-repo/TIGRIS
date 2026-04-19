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

## Band B — Euro-specific (5 axes, new in v2.0)

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

As games complete TIER-C, this section records adoption/retirement history. For v2.0 launch: all 24 axes begin `live`. After Parliament runs, expect 3–6 axes to be marked `adopted` and 0–2 `retired` (first-game adoption patterns based on marathon's similar v1.0→v1.1 transition).

| Axis | First drafted by | Adopted | Retired | Contested | Note |
|---|---|---|---|---|---|
| A1 Elegance | — | — | — | — | carried from v1.0 |
| A2 Decision Density | — | — | — | — | carried from v1.0 |
| A3 Interaction | — | — | — | — | carried from v1.0 |
| A4 Variance Calibration | — | — | — | — | carried from v1.0 |
| A5 Downtime / Pacing | — | — | — | — | carried from v1.0 |
| A6 Teachability | — | — | — | — | carried from v1.0 |
| A7 Emergence / Replayability | — | — | — | — | carried from v1.0 |
| B1 System Gearing | — | — | — | — | new in v2.0 |
| B2 Catastrophe Pressure | — | — | — | — | new in v2.0 |
| B3 Conversion Chain Depth | — | — | — | — | new in v2.0 |
| B4 Information Transparency Cost | — | — | — | — | new in v2.0 |
| B5 Architectural Novelty | — | — | — | — | new in v2.0 |
| C1 Tension Budget | — | — | — | — | new in v2.0 |
| C2 Minimum-Score Shape | — | — | — | — | new in v2.0 |
| C3 Scarcity Bite | — | — | — | — | new in v2.0 |
| C4 Engine-Garden Dependency | — | — | — | — | new in v2.0 |
| C5 Anti-Catch-up Pressure | — | — | — | — | new in v2.0 |
| C6 Point-Salad Incommensurability | — | — | — | — | new in v2.0 |
| C7 Action-Menu Clarity | — | — | — | — | new in v2.0 |
| C8 First-Turn Compression | — | — | — | — | new in v2.0 |
| D1 Family-to-Expert Scaling | — | — | — | — | new in v2.0 |
| D2 Spatial-Interaction Presence | — | — | — | — | new in v2.0 |
| D3 Count-Robustness | — | — | — | — | new in v2.0 |
| D4 Late-Game Lock-in Point | — | — | — | — | new in v2.0 |

## Changelog

- **v2.0.0** — 2026-04-19 — Pool introduced. 24 axes. `Thematic Integration` from v1.0 retired pre-launch (panel consensus). Forward-only versioning reset: v2.0 is a bet change, not a rubric change; all v1.0 scores remain locked against v1.0 and are NOT re-scored against v2.0. The Pool is a new rubric, not an amended one.
