---
name: GARNER — Design (Rules)
slug: garner-design
game: 0023-garner
stage: design
version: 1.0.0
rubric_version: v2.20
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: feld
anchor_axis: C6
counter_pressure_primary: A7
---

# GARNER — Rules

## 1. Overview & Object of the Game

GARNER is a 2–4 player warehouse management Euro set across four trading seasons. Each player is a warehouse master purchasing commodities on the open market, storing them efficiently, and converting the best lots into processed goods before the merchant caravans arrive at Summer's end.

At the start of every game, five Merchant Contracts are drawn from a deck of twenty and placed face-up in the town square. These contracts define the entire scoring landscape: what combinations of goods are worth hoarding, what surplus becomes a liability, and which conversions are worth their cost. A completed row may be worth three VP per contract — or worthless. A rack of Flour might be the game's best line — or a trap. The answer changes with every shuffle.

Players win by reading the contract landscape accurately, positioning their warehouse grids to score heavily on two or three active contracts while avoiding the worst liabilities. The player with the most VP after Summer scores wins. Ties broken by fewest Chaff tiles; further ties by turn-order proximity to the first player.

**The game's central tension:** your warehouse grid has sixteen spaces. You will fill roughly twelve of them with commodity tiles across four rounds. Which five contracts are active this game determines which twelve tiles to want — and the contracts were different last time.

## 2. Components

- **20 Contract cards** (the scoring deck; 5 drawn per game; remainder boxed)
- **80 Commodity tiles** in four raw types:
  - Grain (22 tiles, golden yellow)
  - Barley (22 tiles, amber)
  - Hops (18 tiles, green)
  - Flax (18 tiles, pale blue)
- **20 Chaff tokens** (straw-brown; shared penalty pool)
- **40 Processed Stock tokens** in two types:
  - Flour (20 tokens; round, white)
  - Malt (20 tokens; hexagonal, amber)
- **4 Warehouse boards** (personal 4×4 grid, 16 spaces each; notched edges for adjacency tracking)
- **1 Market board** (shared draft area; 4 Market Stalls labeled I–IV)
- **1 Round track** (Season marker; 4 positions: Autumn / Winter / Spring / Summer)
- **1 VP track board** (separate scoring board; 0–60 range)
- **4 VP markers** (one per player)
- **1 First-player token** (sheaf of wheat)

## 3. Setup

1. **Contracts**: Shuffle the 20-card Contract deck. Draw 5 contracts. Place them face-up in the Town Square (the center of the table). Return the remaining 15 to the box unseen — they are not used this game.

2. **Commodity bag**: Place all 80 commodity tiles in an opaque bag. Shake.

3. **Market board**: Draw 3 tiles from the bag and place them in each Market Stall (12 tiles total across 4 stalls). Tiles within a stall are placed face-up in a stack (visible to all players).

4. **Warehouse boards**: Each player takes a Warehouse board. All 16 spaces start empty.

5. **Starting stock**: Each player draws 2 tiles from the bag. These may be placed immediately into their Warehouse (player's choice of placement) or held until the first Warehousing Phase.

6. **Season marker**: Place on Autumn.

7. **First player**: Determined randomly. Hand the First-player token to that player.

## 4. Turn Structure

The game lasts **4 rounds** (one per Season). Each round has 5 phases, in order:

### Phase 1 — Market Restock
Draw 3 tiles from the commodity bag and add them to **each** Market Stall that currently has fewer than 4 tiles. Stalls already at 4 tiles are not restocked. (Overflow: a stall can never exceed 4 tiles — if drawing would exceed 4, draw only until the stall reaches 4.)

### Phase 2 — Selection (snake draft)
Starting with the First-player and proceeding clockwise, each player takes one Market Stall of their choice (all tiles in that stall) and places the tiles in front of them as a **draft hand**. Continue clockwise until every player has selected exactly one stall. Players who have already selected skip their turn. Remaining stalls (not selected) are **left in place** — their tiles carry into the next round's restock.

_Stall scarcity note: with 4 players, all 4 stalls are taken each round (no carry-over). With 3 players, one stall carries over. With 2 players, two stalls carry over (these accumulate quickly by Winter, creating large-stall pressure in Spring/Summer)._

### Phase 3 — Warehousing
All players simultaneously place tiles from their draft hand into their Warehouse board. Rules:
- Tiles must be placed one per space; no stacking.
- Players may choose any empty space (no adjacency requirement at placement).
- A player may place fewer tiles than their hand contains — unplaced tiles from the draft hand become **Chaff**: return them to the communal Chaff token pile and take 1 Chaff token per tile discarded.
- If a player's Warehouse is completely full (no empty spaces), all remaining draft-hand tiles become Chaff.

### Phase 4 — Conversion (optional, sequential)
Starting with the First-player and going clockwise, each player may perform **up to 2 conversions**:

- **Flour conversion**: Remove 2 Grain tiles from the same **row** of your Warehouse. Gain 1 Flour token (place beside your Warehouse board).
- **Malt conversion**: Remove 2 Barley tiles from the same **column** of your Warehouse. Gain 1 Malt token (place beside your Warehouse board).

Converted tiles leave empty spaces in the Warehouse. Empty spaces may be filled in later rounds. A player may decline all conversions.

### Phase 5 — Season Advance
Advance the Season marker. Pass the First-player token clockwise. If the marker just moved to Summer (round 4 complete), the game ends — proceed to Scoring. Otherwise, return to Phase 1.

## 5. Actions

Per-turn action menu for a player in Phase 2–4:

| Action | Phase | Cost | Effect |
|---|---|---|---|
| **Draft stall** | Phase 2 | 1 action (mandatory; once per round) | Take all tiles from one Market Stall into draft hand |
| **Warehouse tile** | Phase 3 | Free (simultaneous) | Place 1 tile from draft hand into empty Warehouse space |
| **Discard to Chaff** | Phase 3 | 1 Chaff token per tile | Discard an unplaceable or unwanted draft tile; take Chaff penalty |
| **Convert Flour** | Phase 4 | 2 Grain in same row | Remove 2 Grain tiles; gain 1 Flour token |
| **Convert Malt** | Phase 4 | 2 Barley in same column | Remove 2 Barley tiles; gain 1 Malt token |
| **Pass conversion** | Phase 4 | None | Decline all conversions this round |

_Action-menu clarity note: all actions are explicitly available or unavailable at any moment. No hidden costs or conditional unlocks beyond the Warehouse-full condition at Phase 3._

## 6. Scoring & End Condition

**End trigger:** After Summer's Phase 5 (4th round complete).

**Scoring:** Score all 5 active Contracts simultaneously. Each player calculates their own score for each Contract independently, then sums. Contracts always score the current state of the Warehouse board plus all Processed Stock tokens adjacent to it.

**Contract anatomy:** Each contract card states:
- A **scoring condition** (a rule that awards positive VP)
- Optionally, a **liability clause** (a rule that deducts VP for a surplus or deficit)

Examples from the 20-card Contract deck:

| Contract # | Scoring condition | Liability clause |
|---|---|---|
| C01 | 3 VP per fully-completed row (all 4 spaces occupied) | — |
| C02 | 2 VP per Flour token | −1 VP per Grain remaining in Warehouse |
| C03 | 5 VP per Flour+Malt pair | — |
| C04 | 4 VP per column containing ≥ 1 Hops and ≥ 1 Flax | — |
| C05 | 1 VP per tile in Warehouse | −2 VP per Chaff token you hold |
| C06 | 3 VP per pair of Grain in the same row | −1 VP per Flour token (punishes conversion) |
| C07 | 2 VP per Barley; +3 VP bonus if most Barley at table | — |
| C08 | 6 VP if Warehouse has zero empty spaces | — |
| C09 | 2 VP per Hops; +5 VP if most Hops at table | − |
| C10 | 4 VP per Malt token | −2 VP per Barley remaining in Warehouse |
| C11 | 3 VP per Flax tile | −1 VP per empty space |
| C12 | 10 VP to the player with the most Processed Stock tokens | 0 VP for all others |
| C13 | 1 VP per tile in Warehouse, doubled for tiles in top row | — |
| C14 | 3 VP per Flax+Hops pair in same row | — |
| C15 | 5 VP if no Chaff; 0 VP otherwise | — |
| C16 | 2 VP per tile; −3 VP per column with only 1 tile | — |
| C17 | 4 VP per completed column (all 4 spaces occupied) | — |
| C18 | 3 VP per Malt token; −1 VP per Barley in Warehouse | — |
| C19 | 2 VP per Grain+Barley adjacent pair in Warehouse | — |
| C20 | 1 VP per Chaff token (Chaff has value — but only if this contract is active) | — |

**VP totaling:** Sum all 5 active contract scores per player. Apply Chaff liabilities (from contracts that penalize Chaff; Chaff tokens held without a Chaff-penalizing contract are harmless). Record on the VP track.

**Tiebreak:** Most Chaff tokens loses (counterintuitive but consistent with liability framing). Further tie: player seated most clockwise from the first-player marker.

## 7. Edge cases & clarifications

- **Full Warehouse in Phase 3**: if your Warehouse has zero empty spaces and your draft hand has tiles, every tile in your hand becomes Chaff (take 1 Chaff token per tile). You cannot place into occupied spaces.
- **Conversion and scoring**: Processed Stock tokens (Flour, Malt) score under contracts that mention them; raw commodity tiles in the Warehouse score under contracts that mention raw tiles. Flour is not Grain; Malt is not Barley — they are distinct scoring categories.
- **C20 active**: if Contract C20 is drawn, Chaff tokens have positive VP. Chaff is no longer a pure liability under this configuration — this is intentional and creates a unique contract landscape. Players who habitually avoid Chaff may be disadvantaged.
- **C12 majority tiebreak**: if two or more players are tied for most Processed Stock tokens when C12 is active, neither scores the 10 VP (strict majority required).
- **Empty spaces at scoring**: empty Warehouse spaces are neutral unless a contract specifically mentions them (C11 and C16 penalize empty spaces).
- **Stall carry-over at 2 players**: two stalls accumulate across rounds. By Spring at 2 players, a stall may hold 6-9 tiles — valuable but may overfill a Warehouse. Players must manage overflow risk.
- **Bag empty**: if the commodity bag empties mid-restock, restock with what remains. No emergency reshuffling. Remaining rounds have reduced market availability — scarcity intensifies.

## 8. Special Subsystem — Adjacency Scoring (contracts C04, C14, C19)

Three contracts in the deck score specifically against **adjacent pairs** in the Warehouse grid. "Adjacent" means orthogonally neighboring (not diagonal). The 4×4 grid has up to 24 orthogonal adjacency edges. Players who target adjacency-scoring contracts must plan placement strategically.

_C04 example_: a column with Hops in row 1 and Flax in row 3 is **not** a Hops+Flax column (they must both be in the same column; C04 does not require adjacency, only same-column co-presence).

_C14 example_: a row with Flax in column 2 and Hops in column 3 earns the Flax+Hops pair (orthogonally adjacent, same row). A row with Flax in column 1 and Hops in column 3 does NOT earn — not adjacent.

## 9. Variants

**2-player variant**: Use a 3×4 Warehouse grid (12 spaces instead of 16). Reduce starting tiles from 2 to 1. The smaller grid forces harder placement decisions earlier and prevents the "easy-fill-everything" path that 2-player's slower draft can otherwise permit.

**Learning variant**: Draw only 3 contracts instead of 5. Reduces scoring complexity at the cost of C6 incommensurability depth. Not recommended after first play.

## 10. Designer Notes — A7 Counter-Pressure Documentation

_This section is required for v2.18 retirement-reversal eligibility. It documents A7 Emergence-Replayability as an explicit counter-pressure design target._

**Counter-pressure target: A7 Emergence-Replayability** — re-entered contested-watch after Pandemic Legacy (game #22) refuted A7 via the no-replayability argument (B5↔A7 CR 5-2 for Chvátil): "Legacy mechanism explicitly defeats replayability by design — zero replayability after the campaign ends."

**GARNER's counter-pressure mechanism:**

The 20-card Contract deck produces C(20,5) = 15,504 distinct contract combinations. Each combination creates a different scoring landscape with different optimal paths. The counter-pressure claim has two components:

1. **Replayability (structural guarantee):** No consumable elements. Nothing is unlocked once and gone. No stickers. No permanent alterations. Every play uses the same 80 tiles, the same 4 Warehouse boards, and a freshly drawn 5-of-20 contract selection. The game is as replayable on play 100 as on play 1 — structurally equivalent to a shuffled deck game like Dominion. This directly refutes the Pandemic Legacy argument: GARNER cannot be "consumed."

2. **Emergence (within each play):** Player warehouse states diverge significantly from round 1. A player targeting C03 (Flour+Malt pairs) places tiles differently from a player targeting C01 (completed rows) — they draft different stalls, place tiles in different positions, and convert at different rates. By Winter, two players' warehouses look architecturally different. The emergent play-state divergence is produced by the contract landscape, not by pre-programmed events.

**For v2.18 counter-pressure eligibility, the panel argument must confirm both:**
- A7 earns via the contract-deck replayability mechanism (≥1 defend mark in argument)
- A7 earns with at least one collision-vote confirmation (OP or CR collision against an adjacent axis, confirming canonical earning)

The designed collision candidate is **C6↔A7**: are incommensurable within-game scoring paths (C6) and structural across-game replayability (A7) orthogonal registers of the same mechanism (the contract deck) — or does C6 subsume A7? If the panel votes OP (temporal registers: within-game vs. across-game), both earn and the OP serves as the collision-vote confirmation needed for v2.18. If the panel votes CR in favor of C6 and A7 loses, the counter-pressure design has failed for this game — a second designed original targeting A7 explicitly would then be required.

## 11. Collision Adjacency Chart

New adjacency candidates surfaced by GARNER's mechanics:

| Axes | Adjacency type | Reasoning |
|---|---|---|
| **C6 ↔ A7** | temporal-register (new — candidate) | C6 fires on within-game scoring incommensurability; A7 fires on across-game replayability. Both driven by the same mechanism (variable contract deck) at different temporal scales. OP-candidate per I-pandemic-03 temporal-register type. |
| **C6 ↔ A2** | co-firing (existing) | Incommensurable contracts create dense per-turn decisions; the contract landscape directly multiplies A2's decision surface. |
| **D4 ↔ C3** | co-firing (existing) | Tile placements lock in permanently (D4) while Warehouse scarcity limits available space (C3); these pressure the same decisions from opposite directions. |
| **B2 ↔ C3** | co-firing (existing) | Chaff accumulation (catastrophe build-up, B2) is driven by Warehouse scarcity (C3). Bag-empty condition reinforces. |

## 12. Open Questions at Design-Stage

1. **Contract count calibration**: is 20 contracts (15,504 combinations) the right pool? Alternatives: 15 contracts (C(15,5) = 3,003 combinations; faster to balance but less variety) or 24 (harder to balance but richer). Current design: 20.

2. **Conversion limits per round**: the current rules allow up to 2 conversions per round per player. Is this too generous (removes scarcity from conversion) or appropriate (allows Flour/Malt strategies to be viable)? Panel may challenge C3 earning if conversion is too easy.

3. **C20 Chaff-has-value contract**: this contract inverts the liability. Some players find it jarring; others find it a crucial part of the C6 incommensurability thesis (if Chaff can be valuable, even "wasted" tiles are strategically meaningful). Keep or cut?

4. **2-player Warehouse size**: is 3×4 (12 spaces) the right reduction? Too small may make the game too deterministic (every tile matters too much); too large may make 2-player feel like 4-player with slower drafts.

5. **Snake draft order**: currently Phase 2 is a simple snake (1-2-3-4 / 4-3-2-1 alternating). A simultaneous draft (all players pick simultaneously, resolve conflicts by turn order) might reduce downtime but loses the strategic communication of watching where others draft.
