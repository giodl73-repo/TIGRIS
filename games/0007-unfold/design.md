---
name: UNFOLD — Design (Rules)
slug: unfold-design
game: 0007-unfold
stage: design
version: 1.0.0
rubric_version: v2.6
bet_version: parliament
author: TIGRIS (user + Claude collaboration)
created: 2026-04-19
updated: 2026-04-19
anchor_persona: chvatil
anchor_axis: architectural-novelty
concept_version: v1.1 (panel-reviewed, 3-validator greenlight)
---

# UNFOLD — Rules

## 1. Overview & Object of the Game

UNFOLD is a 2-4 player tile-placement game set on a shared island. Players represent cultures — Feng Shui, Medieval Village, Permaculture, Modernist Grid — each with its own aesthetic, tools, and ideal level of development. The island starts as pristine forest. Over roughly 20 rounds, players place and flip pop-up tiles, developing the land through buildings, roads, and temples — while the **Development Level** of the island rises.

**Central tension:** each culture has a different IDEAL Development Level. A Modernist thrives in a dense built island; a Permaculture player thrives in a mostly-preserved one. The shared track forces collision: every tile placement affects everyone's distance-from-ideal. On top of that, every placement or flip triggers a **forced-fold cascade** — one adjacent tile must state-change in the opposite direction (open one, close a neighbor).

**Object:** highest cultural score, adjusted by Development Alignment multiplier.

**Win condition:**
```
final_score = cultural_score × dev_alignment_multiplier + spirit_bonus − dusk_skip_penalty
```

## 2. Components

| Item | Count | Notes |
|---|---:|---|
| Pop-up tiles | 60 | 6 types × 10 each. Each tile has 3 printed states (closed-left / open-up / closed-right). Accordion-fold cardboard or 3-layer-stack simulation. |
| Island board | 1 | Bounded, irregular coastline; ~55 tile positions. Variable scenarios possible. |
| Starting forest tiles | 15 | Tree-tiles pre-placed in open-up state at setup (from the 60). |
| Culture boards | 4 | One per culture (Feng Shui, Medieval, Permaculture, Modernist); scoring, tool, ideal Dev Level. |
| Spirit tokens | 4 | One per culture; distinct visual (compass, knight-banner, sapling, pylon). |
| Development Level track | 1 | 0-40 scale; single communal marker. |
| Round / turn tracker | 1 | Tracks turn order; marks Dusk thresholds. |
| Dusk event cards | 3 | One per Dev Level threshold (10, 20, 30); flipped when triggered. |
| Scoring pad | 1 | For endgame. |
| Rulebook | 1 | This document. |
| Player aid cards | 4 | One per culture; iconography-first quick reference. |

**Tile type inventory (60 tiles):**

| Type | Count | Closed-Left (◀) | Open-Up (▲) | Closed-Right (▶) |
|---|---:|---|---|---|
| Tree | 18 (15 pre-placed + 3 in draw) | grass/meadow | big tree | road |
| Water | 8 | closed canal | bridge | waterway |
| Cottage | 12 | cottage garden | cottage (3D) | courtyard path |
| Temple | 6 | shrine-ground | temple (3D, Spirit-hostable) | stone plaza |
| Market | 8 | empty square | market stalls | trading road |
| Stone | 8 | pond | standing stone | stone wall |

Total: 60.

**Tile state semantics (Dev Level contribution):**

- Open-up **building** (Cottage, Temple, Market): **+1 Dev**
- Open-up **infrastructure** (Water/bridge): **+0.5 Dev**
- Open-up **nature** (Tree, Stone): **0 Dev**
- Closed-right (any type): **+0.5 Dev**
- Closed-left (any type): **0 Dev**

## 3. Setup

1. Place the island board in the center of the table.
2. Place the Development Level marker at **0** on the track.
3. Pre-place 15 Tree-tiles in **open-up state** on the island per scenario setup (3-4 scenario configurations; pick randomly or by agreement). Starting Dev Level = 0 (trees are nature).
4. Each player selects a culture board and takes the matching culture's player aid card, Spirit token, and tool reference.
5. Each player places their Spirit token in reserve (off-board).
6. Shuffle the remaining 45 tiles (60 total minus 15 starting trees) in the opaque tile bag.
7. Reveal 5 tiles from the bag face-up in the market slots.
8. Place the 3 Dusk event cards face-up beside the track.
9. Determine first player randomly (or agreed-upon).

Setup time: <5 minutes.

## 4. Turn structure

Each turn, the active player takes **exactly 2 actions**. Actions are resolved one at a time, with any cascade resolving immediately after the triggering action. After your 2 actions, check whether the Development Level has crossed a Dusk threshold (10, 20, 30); if so, resolve Dusk Phase before the next player's turn.

Turn ends; play passes clockwise.

## 5. Actions in detail

Each action is ONE of the following (4 types; pick any 2 per turn; same type can be taken twice).

### 5.1 PLACE — 1 action

1. Draft a tile from the market (5 visible).
2. Choose its state (closed-left / open-up / closed-right).
3. Place it adjacent (edge-touching) to an existing placed tile on the island. Starting-forest trees are existing tiles.
4. **Update Development Level** per §2 tile state semantics.
5. **Trigger forced-fold cascade** (see §6) on one adjacent existing tile.
6. Refill market slot: draw a replacement from the bag and place face-up.

If the bag is empty, market doesn't refill. Game end-triggers when market falls below 3 tiles remaining (see §10).

### 5.2 FLIP — 1 action

1. Choose any tile already placed on the island (yours, opponent's, starting forest, or Temple hosting a Spirit — see caveats below).
2. Cycle its state **one step** in the direction of your choice:
   - Closed-left → open-up
   - Open-up → closed-right
   - Closed-right → closed-left
   - (Or reverse direction: closed-right → open-up → closed-left → closed-right)
3. Update Development Level accordingly.
4. Trigger forced-fold cascade (see §6) on one adjacent existing tile.

**Flipping a Temple tile hosting a Spirit:** The Spirit is displaced; it returns to that player's reserve. No additional cost. The flipping player did NOT pay extra, but they've broken that culture's claim on the temple zone.

### 5.3 MOVE SPIRIT — 1 action

Either:
- **Place your Spirit from reserve onto any empty Temple** (Temple tile in open-up state with no other Spirit present), OR
- **Move your Spirit from its current Temple to another empty Temple**, provided a **connected path of placed tiles** exists between the two (tiles in any state connect).

Only one Spirit per Temple at any time. A Temple that's been flipped out of open-up state cannot host a Spirit.

### 5.4 USE TOOL — 1 action

Apply your culture's unique Tool. Specific effects per culture (§8).

## 6. Forced-fold cascade

**After every PLACE or FLIP action**, you must trigger a state change on one adjacent tile of your choice.

### 6.1 Cascade direction rule

- If your action set the triggering tile to **open-up**: you must cause an adjacent tile to **CLOSE**. That adjacent tile moves to **closed-left OR closed-right** (your choice).
- If your action set the triggering tile to **closed** (either left or right): you must cause an adjacent tile to **OPEN** (move to **open-up**).

**Why:** pop-up books conserve energy. One panel opens, another must close.

### 6.2 Choosing the cascading tile

You pick any one tile that shares an edge with the triggering tile. It must be a placed tile (starting forest tree counts). You cannot cascade onto a Temple hosting a Spirit unless you're willing to displace it (Spirit returns to reserve per §5.2).

### 6.3 No chain reactions

The cascading tile state-change does NOT trigger a further cascade. Single-step ripple only. This keeps resolution fast.

### 6.4 Edge cases

- **No adjacent tile exists**: if the triggering tile is on the edge of the island with no adjacent placed tile, skip the cascade. Rare — usually avoidable by placement choice.
- **All adjacent tiles are locked** (e.g., all are Temples hosting Spirits and you don't want to displace any): same as above — cascade is skipped. (This is unusual.)
- **Cascading tile was just modified by the previous action this turn**: allowed. The pop-up re-springs.

### 6.5 Update Dev Level after cascade

The cascaded tile's new state changes the Dev Level per §2. Cumulate with the triggering action's Dev Level change.

**Example:**
- You PLACE a Cottage in open-up state (+1 Dev) adjacent to a Tree in open-up state.
- Your action is "open-up," so cascade direction is **close**.
- You choose to close the Tree to closed-right (becomes a road: +0.5 Dev).
- Net Dev change this action: +1 (cottage) + 0.5 (road) = **+1.5 Dev**.

## 7. Dusk Phase

Triggered when the Development Level **crosses** 10, 20, or 30 (first crossing only; each threshold is a one-time event). Resolved immediately after the crossing action (before the next player's turn).

### 7.1 Dusk sequence

1. Starting with the player whose action triggered the crossing, each player in turn order picks ONE Dusk action.
2. Dusk actions don't count against your turn's 2-action budget; they're free this round.
3. Mark the Dusk card as triggered; it won't fire again.

### 7.2 Dusk action options

Each player picks one:

- **REWILD** — flip one closed-right tile anywhere on the island to closed-left. (Dev decreases; nature returns; roads become grass/meadow.)
- **CONSOLIDATE** — move your Spirit to another Temple for free (no action cost), OR place a Wall token between two adjacent tiles (Medieval-only; see §8).
- **ADVANCE** — your culture's Tool can be used **twice** during your next turn (bonus action economy).

### 7.3 Skipping Dusk

A player may skip Dusk (take no option). Record this — **−1 VP at endgame per skipped Dusk**. Discourages passivity without blocking it.

### 7.4 Dusk cascades?

Dusk actions (REWILD, CONSOLIDATE, ADVANCE effects) do NOT trigger forced-fold cascades. Dusk is outside the normal action economy.

## 8. The Four Base Cultures

Each culture has: **Aesthetic lens** (scoring formula), **Tool** (1-action modifier), **Spirit token** (visual), **Ideal Dev Level** (range for Dev-Alignment multiplier).

### 8.1 Feng Shui (Eastern Directional Harmony)

- **Ideal Dev Level:** 20–30 (balance)
- **Spirit:** Compass / Lantern
- **Tool:** **REALIGN** — when you PLACE a tile, you may rotate it 90° for free (aligning it to cardinal axes). Does not change state; just orientation. (Note: rotation is normally locked at placement. Feng Shui's tool permits rotational choice at no extra action.)
- **Scoring (per endgame):**
  - Cardinal row bonus: +2 VP per row of 3+ tiles aligned on N-S or E-W axis through your Temple (Spirit-adjacent bonus 2×).
  - Water-flow pairs: +3 VP per (Water closed-left → open-up-bridge → Water closed-right) chain.
  - Curved paths: +2 VP per curving road run of 3+ closed-right tiles that don't form a straight line.
  - **Penalty**: −2 VP per sharp-angle sightline passing through your Temple (straight line of 3+ identical-state tiles, including the Temple).

### 8.2 Medieval Village (European Fortified)

- **Ideal Dev Level:** 25–35 (developed, not sprawling)
- **Spirit:** Knight banner
- **Tool:** **WALL** — when you PLACE a tile, you may also add one Wall token on an edge between two tiles (yours or shared). Walls have tactical effects: a walled edge BLOCKS forced-fold cascade from crossing it. (Only 6 Wall tokens per game; limited resource.)
- **Scoring:**
  - Road length: +1 VP per tile in your longest connected road network (closed-right roads). Maximum 12 VP cap per network.
  - Buildings adjacent to Temple: +2 VP per Cottage/Market adjacent to a Temple in open-up state. Double if YOUR Spirit occupies the Temple.
  - Walled perimeter: +5 VP if a closed ring of Wall tokens encloses ≥3 Cottage/Market tiles.
  - **Penalty**: −1 VP per detached Cottage (no road connection to any Temple).

### 8.3 Permaculture (Regenerative Eco-Design)

- **Ideal Dev Level:** 10–25 (preserved with settlements)
- **Spirit:** Sapling / Seedling
- **Tool:** **FREE SEED** — once per turn, when you PLACE a tile in open-up state that is TREE type, that placement costs 0 actions (not 1). You can still cascade.
- **Scoring:**
  - Tree + Water adjacency: +2 VP per Tree tile (open-up or closed-left) adjacent to any Water tile.
  - Buildings-on-forest-edge: +2 VP per Cottage/Temple/Market adjacent to at least one Tree in open-up state.
  - Layered greens: +5 VP per 3-tile run of Tree tiles where at least one is closed-left and at least one is open-up (showing both forest floor and canopy).
  - **Penalty**: −2 VP per cleared zone of 4+ adjacent tiles containing no Tree tile.

### 8.4 Modernist Grid (Urban Planning)

- **Ideal Dev Level:** 30–40 (dense)
- **Spirit:** Pylon / Modern sculpture
- **Tool:** **GRID EXTEND** — when you PLACE a tile in closed-right (pathway/road) state, you may immediately place 1 additional closed-right tile adjacent to it at 0 additional actions. The additional tile also triggers a cascade.
- **Scoring:**
  - Dense building cluster: +3 VP per group of 3+ adjacent Cottages/Markets in open-up state.
  - Road-network efficiency: +1 VP per tile in your connected road network, bonus +5 VP if network connects to ≥3 buildings.
  - **Penalty**: −2 VP per Tree (closed-left or open-up) with no adjacent building within 2 tiles.

## 9. Development Level track

### 9.1 Dev Level calculation

Per §2 tile-state semantics. Dev Level = sum across all placed tiles:

- Building open-up (Cottage, Temple, Market): +1
- Infrastructure open-up (Water/bridge): +0.5
- Nature open-up (Tree, Stone): 0
- Any closed-right: +0.5
- Any closed-left: 0

### 9.2 Dev-Alignment multiplier

At endgame, each player calculates: |actual final Dev Level − your culture's ideal Dev Level range|. If actual is within the ideal range, distance is 0.

Multiplier table:

| Distance from ideal | Multiplier |
|---|---|
| 0 (within range) | ×1.5 |
| 1-3 | ×1.2 |
| 4-8 | ×1.0 |
| 9-15 | ×0.8 |
| >15 | ×0.5 |

**Example:** Feng Shui player's ideal is 20-30. Game ends at Dev Level 34. Distance = 4. Multiplier = ×1.0. Their cultural score is unchanged.

**Example:** Permaculture player's ideal is 10-25. Game ends at Dev Level 34. Distance = 9. Multiplier = ×0.8. Their cultural score is reduced by 20%.

### 9.3 Spirit bonus

If your Spirit occupies a Temple at endgame, the 6 tiles adjacent to that Temple score **DOUBLE** for your culture's criteria. Spirit-adjacent tiles are computed per the 6-neighbor hex rule (each Temple has 6 adjacent positions; count only placed tiles).

## 10. Scoring / end condition

### 10.1 End triggers (game ends at end of current round after any of):

- Development Level reaches 40.
- Tile bag is empty AND market has < 3 tiles.
- Island is fully tiled (all positions filled).

### 10.2 Final scoring

For each player:
1. Calculate **cultural_score** per your culture's §8 formula (apply all bonuses and penalties).
2. Apply **Spirit bonus** to adjacent tiles (§9.3).
3. Calculate **Dev-Alignment multiplier** (§9.2).
4. **final_score = cultural_score × dev_alignment_multiplier**.
5. Subtract **1 VP per skipped Dusk phase** (§7.3).

### 10.3 Winner

Highest final_score wins.

### 10.4 Tiebreaker

1. Most adopted tiles of your culture's signature type (e.g., Feng Shui counts cardinal-rows; Medieval counts roads; etc.).
2. Closest to ideal Dev Level.
3. Shared victory.

## 11. Edge cases & clarifications

- **Starting-forest tree flipped**: starting trees are ordinary tiles once placed. They can be flipped per normal rules (costs 1 action + cascade).
- **Spirit in displaced Temple**: Spirit returns to its owner's reserve; player can move it to another Temple on a later turn.
- **Dev Level drops below threshold**: threshold events are one-time triggers. Dev going back below 10 after crossing 10 does NOT re-trigger Dusk.
- **Tie on Dusk trigger**: if two Dusk thresholds are crossed by a single action (e.g., Dev jumps from 9 to 11, crossing 10), only the 10 threshold triggers that turn; the 20 is still pending.
- **Multiple cascades-worthy targets**: active player picks freely; no obligation to cascade on a specific tile.
- **Flip cycle choice**: when flipping, active player chooses direction (forward or reverse through the state cycle).
- **Market refill failure**: if bag is empty, market slot stays empty. End-trigger checks each turn.
- **Tool-and-normal-action in same turn**: allowed. Spend 1 action on Tool, 1 action on PLACE/FLIP/etc.

## 12. Collision Adjacency Chart (UNFOLD-specific, candidates for global)

New adjacencies UNFOLD may surface during TIGRIS review:

- B5 Architectural Novelty ↔ D2 Spatial-Interaction (pop-up physicality + shared board — already in global chart via §9 row 4 A3↔D2 adjacent)
- A7 Emergence-Replayability ↔ C6 Point-Salad (25 culture/seed combinations × 4-culture scoring — not yet on chart; candidate)
- B1 System Gearing ↔ C7 Action-Menu Clarity (multi-system gearing with tight menu — candidate)

If UNFOLD's review surfaces clear adjacency cases, log as I-unfold-XX for future global chart updates.

## 13. Variants

### 13.1 2-Player variant

- Island: smaller (same board; use only "core" scenario seed; 40 tile positions instead of 55).
- Cultures: each player picks 2 of 4 cultures; at endgame, you score each culture and take the BETTER score. (Asymmetric lens-selection per session.)
- Turn: 2 actions per turn (unchanged).
- Tiles: use 45 tiles (reduced pool).
- Expected length: 30-40 min.

### 13.2 3-Player variant

- Island: standard.
- Cultures: each player picks 1 of 4 cultures; 4th is unused this session.
- Turn: 2 actions per turn.
- Tiles: use all 60.
- Expected length: 45 min.

### 13.3 4-Player variant (design target)

- Island: standard (55 tile positions).
- Cultures: each player picks 1 of 4 cultures; all 4 in play.
- Turn: 2 actions per turn.
- Tiles: all 60.
- Expected length: 60 min.

### 13.4 Nature Cultures Expansion (sketch, not base game)

- Adds Wabi-Sabi and Indigenous Steward cultures.
- Wabi-Sabi Tool: **CONCEAL** — temporarily hide an open-up tile under a closed-left tree-tile for 2 rounds.
- Indigenous Steward Tool: **PROTECT** — mark a tile as sacred; opponents pay +2 actions to flip it.
- Both cultures have their own Spirit tokens and ideal Dev Levels (Wabi-Sabi: 15-30; Indigenous: 5-15).

### 13.5 Solo mode (sketched; deferred to v1.1)

- Single player vs "Island Spirit" Automa.
- Automa places tiles per a simple state-machine: pick nearest-to-market slot, state chosen by a small card deck.
- Automa-scoring: sum across all 4 cultures, averaged.
- Expected length: 30 min.
- **Full solo rules deferred to v1.1.**

## 14. Designer notes

UNFOLD's architectural bet: **physical pop-up tiles (3 states) + shared island + per-culture scoring + Development Level shared track + forced-fold cascade** produces a game where every action is a 4-system decision (tile state × cascade × cultural lens × shared track).

The panel review of the v1.0 concept flagged:
- Parallel scoring (cultures didn't collide mechanically) → solved by Dev Level track + Dev-Alignment multiplier.
- Decorative pop-up (3D didn't pay rent) → solved by forced-fold cascade (pop-up-book physicality as mechanic).
- Action slack (3 actions were too loose) → solved by 2 actions + Dusk-Phase obligation.
- Culture overlap (6 was too many) → cut to 4 base cultures; 2 deferred to expansion.

Validation pass confirmed fixes landed (Chvátil B5 5→8, Knizia C1 4.5→7.5, Lacerda B1 6→8.5).

**Testable claim:** UNFOLD will earn B5 Architectural Novelty strongly (8+) in TIGRIS's own review. If the review surfaces this as the game's signature, the design holds. If the review scores B5 below 6 and names decorative-pop-up as the critique, we've failed the bet.

**Secondary testable claim:** the Dev-Alignment multiplier creates genuine cross-culture collision. Playtest evidence: if players' strategies diverge significantly (Modernist rushes Dev Level, Permaculture rewilds, Feng Shui balances, Medieval stabilizes mid-high), the shared track is working.

## 15. Open at DESIGN-stage, to resolve post-playtest

1. **Scoring formula calibration** — specific VP values per condition. Needs 3-5 playtests to tune.
2. **Dev Level formula calibration** — is +1 per building and +0.5 per closed-right appropriately balanced across cultures? Test.
3. **Dusk action costs** — is −1 VP per skipped Dusk the right penalty? Test.
4. **Cascade chain or single?** — Chvátil flagged single-step as possibly too tidy. Playtest: compare single-step to 2-step cascades.
5. **Starting forest seed** — 3-4 scenarios; need to design them. First-play with a "standard" 15-tree configuration.
6. **Spirit movement: path requirement** — "connected path of placed tiles" is intentionally loose; tighten if abuse emerges.
7. **Wall token count** — 6 walls per game for Medieval; tune after playtest.
8. **Market size** — 5 slots at 4p; consider reducing to 4 at 3p, 3 at 2p.
9. **Culture balance** — first playtest will show if any culture dominates. Expect Modernist to feel strongest (Dev Level grows naturally); Permaculture may need buffing.
10. **Component engineering** — real pop-up tiles vs simulated-with-tokens is a manufacturing decision. For design, simulated-tokens is fine.

## Compliance checks (per /tigris-design skill invariants)

- ✅ §4 action menu has 4 action types (≤6 ceiling; C7 Action-Menu Clarity target met).
- ✅ §6 end-trigger and §10 scoring both specified.
- ✅ §5 covers every tile type and state via PLACE + FLIP; cascade handles secondary effects.
- ✅ Anchor declarations (Chvátil / B5) reflected: §6 forced-fold cascade is the architectural novelty claim.
- ✅ §12 collision adjacency chart candidates flagged for global update.
- ✅ §15 open questions present.

## Status

**Design v1.0 complete — 2026-04-19.** Ready for TIGRIS panel review (`/tigris-panel` — game-review mode) before first physical playtest.

Recommended next step: dispatch TIGRIS panel to review this design.md. If v2.4 pipeline passes, commission first physical playtest and proceed to TIER-A STAKES → TIER-B ARGUMENT → TIER-C AMENDMENT.

The adversarial architecture applied to our own design will surface what we've missed.
