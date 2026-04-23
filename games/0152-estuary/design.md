---
name: ESTUARY — Design (Rules)
slug: estuary-design
game: 0152-estuary
stage: design
version: 1.0.0
rubric_version: v2.24.83
bet_version: parliament
author: TIGRIS
created: 2026-04-23
updated: 2026-04-23
anchor_persona: lacerda
anchor_axis: B1
players: 3-5
weight_estimate: 3.8
duration_estimate: 120
---

# ESTUARY — Rules

## 1. Overview & Object of the Game

Estuary is a complex port-trading game for 3–5 players set in a 17th-century river delta.
Each player commands a merchant house competing to fulfill a private cargo manifest — a
sealed grid specifying exactly which processed goods must reach which delivery ports, and
in what quantities. Other players can see what your ships are carrying, but not what your
manifest requires. That asymmetry is the game.

Victory goes to the first player to complete 6 of their 12 manifest cells (delivering
enough processed goods to a matched port). When a player reaches 6 completions, all
remaining players finish the current round, then final scoring applies.

The game operates as a five-stage pipeline: goods must be loaded onto ships, sailed to
processing harbors, transformed into manifest-eligible cargo, sailed again to the correct
delivery port, and matched against the manifest. Each stage is conditional on the previous
one; the whole chain must be navigated before a cell can score. Harbor capacity is finite,
tides shift the navigable channels every round, and the manifest is the only thing that
tells you whether all this effort was worth it.

Information is itself a resource. A player who knows that an opponent needs 4 Salt to
South Quay can saturate that harbor to extract toll income, or load Salt aggressively to
compete for that delivery window. Gaining that knowledge costs mana. Every revealed cell
stays public for the rest of the game. The decision to pay for information — or to reveal
your own in trade — is as live as any build action.

## 2. Components

**Board & Tracks**
- 1 Estuary board: hex-grid delta, 42 hexes
  - 20 navigable water channels
  - 8 processing harbors (two per goods type)
  - 8 delivery ports (two per goods type, plus River Gate × 2 and Open Sea × 2)
  - 4 supply depots (one per goods type)
  - Fault lines: 6 contested hexes that block both players' ships if simultaneously occupied
- 1 Tide Wheel (circular track, 8 positions, central to the board)
- 1 Tide Marker

**Player Components (×5 per color)**
- 1 player board (showing fleet capacity, mana reserves, harbor license slots)
- 3 ships
- 1 manifest screen (folds to conceal a 4×3 grid)
- 12 manifest tiles (hidden face-down behind screen; values 1–5 units required)
- 10 mana discs (2 per goods type + 2 wild)

**Shared Supply**
- 80 goods cubes: Timber (20, brown), Grain (20, yellow), Salt (20, white), Cloth (20, purple)
- 8 harbor tiles (double-sided: neutral / licensed)
- 24 harbor improvement tiles (3 types × 8)
- 30 toll markers
- 1 VP track + 5 VP markers
- 1 round marker + 1 round track (12 rounds maximum)

**Manifest Tiles (construction)**
Each player's 12 manifest tiles are drawn blind from a shared distribution bag at setup.
The bag contains tiles for each [Goods × Port] combination at values 1, 2, 3, 4, and 5
(one each). Each player draws 12 tiles without looking, then arranges them face-down in
their 4×3 manifest grid. Before the game starts, each player looks at their own tiles but
not others'. Total cells: 3-player = 36 face-down tiles; 5-player = 60 face-down tiles.

## 3. Setup

1. Place the Estuary board in the center. Set the Tide Marker on the Tide Wheel at position 1.
2. Place the 4 supply depots on their designated board hexes. Seed each with 4 goods cubes
   of the matching type.
3. Shuffle and place one harbor tile face-up on each harbor hex. All harbors start neutral
   (unlicensed); licensing happens via the Build action.
4. Distribute player components. Each player takes their ships, board, screen, and mana discs.
5. **Manifest construction:** Separate the manifest tile bag into four suits (one per goods
   type). Each player draws 3 tiles from each goods-suit (3 Timber, 3 Grain, 3 Salt,
   3 Cloth) and places them face-down in their manifest grid, arranged in the 4×3 layout
   (rows = goods types, columns = North Quay / South Quay / River Gate). Look at your
   own tiles only.
6. Starting positions: players place 1 ship each at the supply depot nearest their player
   color's corner. Place 2 remaining ships in reserve.
7. Starting mana: each player takes 3 mana discs (1 per non-wild type of choice + 1 wild).
8. First player: player with the most complex manifest (highest sum across all tiles);
   ties broken by youngest player.

## 4. Turn Structure

Each round proceeds in four phases:

### Phase 1 — Tide Advance (automatic)
Advance the Tide Marker 1 step clockwise on the Tide Wheel. Consult the Tide Chart:
certain channel hexes become navigable or impassable based on the current position.
Update channel markers (navigable = blue face up; impassable = grey face up).

Ships stranded in newly impassable channels remain stranded until the tide shifts again.
Stranded ships may take no movement actions but may still Process at an adjacent harbor
if docked.

### Phase 2 — Player Actions
In turn order, each player takes **3 actions**. Players may take the same action multiple
times. With 4+ players, simultaneous resolution is recommended: all players declare all 3
actions simultaneously, then resolve conflicts (harbor capacity, supply depot limits) in
turn order.

### Phase 3 — Harbor Resolution
All ships currently docked at a harbor with a valid license attempt to process. Processing
succeeds if:
- The ship carries the harbor's goods type.
- The harbor has remaining capacity (1 ship per round per harbor; 2 if harbor has a
  Expansion improvement).
- The player paid the harbor toll (2 mana of matching type) at the start of this phase.

Goods successfully processed are flipped to their processed-goods side (light-bordered
cube face). Unprocessed goods remain raw.

### Phase 4 — Delivery Check
Any ship docked at a delivery port with processed goods aboard: the player declares which
manifest cell they are targeting. If the goods type and port match, AND the quantity
delivered ≥ the manifest cell value, the cell is completed. Reveal that manifest tile
publicly (stays face-up for all to see). Score VP immediately (see §6).

Over-delivery scores at 50% rounded down (delivered more units than the manifest
requires). Under-delivery scores 0 and does NOT reveal the tile — the player may retry.

---

## 5. Actions

Six actions are available. Each costs 1 action to take (some have additional mana costs).

### 1. Sail
Move one of your ships through up to 3 navigable channel hexes. Cannot enter impassable
channels (tide-blocked). Cannot enter a channel containing 2+ other ships (capacity = 2
ships per hex). **Cost:** 1 mana (any type).

Ships may end in a supply depot hex, harbor hex, or delivery port hex.

### 2. Load
Pick up goods cubes from a supply depot or from a harbor's surplus pile (goods left
unprocessed by other players are placed in the harbor's surplus). Load up to your ship's
capacity (3 cubes per ship standard; 4 with Expanded Hull improvement). **Cost:** 0. Depot
goods are free. Harbor surplus goods cost 1 mana (matching type) per cube.

### 3. Process
Dock your ship at a harbor hex. Pay the harbor toll (printed on the harbor tile; default
2 mana of matching type; owner earns this as toll income). Declare processing intent.
Resolution occurs in Phase 3. **Cost:** harbor toll (paid now; processing resolves in
Phase 3).

Note: You may Process without the matching license, but the harbor processes your goods
last (after licensed players). If capacity is exhausted, your goods are queued to the
next round automatically — you do not need to take this action again.

### 4. Deliver
Move your ship (if not already there) to a delivery port and attempt to score. Declare
the target manifest cell. Check: does the port match? Do goods type and quantity meet or
exceed the cell value?

- **Match + sufficient quantity:** Cell completed. Reveal tile. Score VP (see §6).
- **Match + insufficient quantity:** Nothing. Tile stays hidden.
- **No match:** Nothing. Wasted move.

**Cost:** 0 (movement to the delivery port is NOT included — use Sail for that).

### 5. Observe
Pay 3 mana (any type; wild counts) to reveal one cell of any opponent's manifest.
The target player announces the value on that cell aloud. Flip the tile face-up.
It stays face-up permanently (even if the player later completes that cell). **Cost:** 3 mana.

*Note on partial information:* You choose which [Goods × Port] combination to ask about.
You cannot ask "what is your highest cell?" — you must specify the exact cell.

### 6. Build
Place one harbor improvement tile on any harbor hex where you hold a license (or on an
unlicensed harbor to claim it). To claim an unlicensed harbor: pay 4 mana of the harbor's
goods type. The harbor is now licensed to you. You earn all future toll income from it.

To build an improvement on a harbor you already hold, pay 3 mana (any type).

**Harbor improvements (3 types):**
- **Expansion** — Harbor now processes 2 ships per round (default = 1). Costs 3 mana.
- **Express Line** — Ships processing at this harbor need not pay the toll (you still earn
  toll from opponents). Costs 3 mana.
- **Survey Post** — Once per round, you may observe one opponent manifest cell for free
  (1 mana instead of 3) when a ship processes at this harbor. Costs 3 mana.

Maximum 2 improvements per harbor. **Cost:** 4 mana (license claim) or 3 mana (improvement
on held harbor).

---

## 6. Scoring / End Condition

### Cell Completion VP
When a manifest cell is completed (Deliver action succeeds), score immediately:

| Cell value (units required) | VP scored |
|---|---|
| 1 | 2 |
| 2 | 4 |
| 3 | 6 |
| 4 | 9 |
| 5 | 12 |

**Over-delivery:** If delivered quantity > cell value, score floor(full VP / 2). Example:
cell value 3 (6 VP); deliver 5 units → score 3 VP.

### Bonus VP (scored at game end)
- **Goods Sweep:** Complete all 3 cells for a single goods type (all 3 ports): +5 VP.
- **Port Sweep:** Complete all 4 cells for a single delivery port (all 4 goods): +5 VP.
- **Harbor Majority:** Most harbor licenses at game end: +3 VP. Tie: +1 VP each.
- **Residual Mana:** 1 VP per 3 mana discs remaining.

### End Trigger
The round in which any player completes their 6th manifest cell is the final round.
All players finish the current round, then final scoring applies.

If no player reaches 6 completions after 12 rounds, the game ends after round 12.
Final scoring applies; whoever has the most VP wins.

### Tiebreaker
Most completed manifest cells → most remaining mana → most harbor licenses → first player
in turn order from the triggering player (anti-clockwise).

---

## 7. Edge Cases & Clarifications

**Q: Can I observe a cell that has already been revealed?**
No. You cannot spend 3 mana to observe a face-up tile. Revealed tiles are public; no
further payment is needed or possible.

**Q: Can I deliver to a cell I've already completed?**
No. Completed cells (face-up) cannot be scored again. You may over-load goods to that
port, but they earn 0 VP.

**Q: What happens to goods aboard a stranded ship?**
They remain on the ship until the tide frees the channel. Goods do not spoil.

**Q: Can I build on a harbor I don't have a license for?**
Only via the license claim action (costs 4 mana of matching type). You cannot improve an
unlicensed harbor.

**Q: Does Harbor surplus accrue indefinitely?**
No. Surplus resets at the end of each round. Unprocessed surplus is returned to the
supply depot at round-end.

**Q: If two players target the same manifest cell in the same Delivery Check phase?**
Delivery is resolved in turn order. The first player who meets quantity requirements
claims the cell; the second player's delivery attempt earns 0 VP (the cell is already
completed). Goods are returned to the player's ship unscored.

**Q: Can a ship carry mixed goods types?**
Yes, up to 3 cubes total per ship regardless of type. A ship can carry 2 Timber + 1 Grain.

**Q: Fault Line rule (contested hexes):**
If two players' ships simultaneously occupy a Fault Line hex at the end of Phase 2, both
ships produce no mana and cannot Sail or Process in Phase 2 of the *next* round (they are
"stuck in dispute"). This resolves automatically at start of Phase 2 (both ships are freed
to act normally).

---

## 8. Special Subsystems

### The Tide Wheel

The Tide Wheel has 8 positions. After each round, the Tide Marker advances 1 step. The
Tide Chart (printed on the board) specifies which of the 20 channel hexes are navigable
at each of the 8 positions:

- Positions 1–3: High Tide. All channels navigable. Supply depots accessible from all sides.
- Position 4: Ebb. 6 upper-delta channels impassable. North supply depot isolated.
- Positions 5–6: Low Tide. 10 central channels impassable. Only coastal harbors accessible.
- Position 7: Flood. 6 lower-delta channels impassable. South delivery ports isolated.
- Position 8: High Tide (cycle resets to Position 1 next round).

The Tide Wheel cycles every 8 rounds. In a 12-round game, it completes 1.5 cycles — the
low-tide phase hits twice. This creates two planning horizons: players see the first tide
restriction coming and can plan around it; the second arrives during the game's final
third, forcing adaptation.

**Veil's Eye equivalent:** The Estuary Lighthouse map feature (1 hex at the delta's
mouth) allows the controller to hold the Tide Wheel back by 1 step for one round per
control (once per game). This is a minor arc-extension mechanic mirroring CODEX's
Veil control.

### Manifest Tile Distribution

The 12-tile manifest setup requires careful balancing across player counts:

| Player count | Tiles per player | Total manifest cells |
|---|---|---|
| 3 | 12 | 36 |
| 4 | 12 | 48 |
| 5 | 12 | 60 |

The tile distribution bag guarantees each player draws exactly 3 tiles per goods type.
Value distribution across all tiles in the bag: values 1–5 appear equally (each goods
type × port combination gets one tile of each value in the 5-player bag). For 3–4 player
games, the bag is pre-seeded per the §9 count-specific setup.

### Harbor License Economy

Harbors are the game's contested resource. Only 8 harbors exist for 3–5 players. At 5p,
each player can theoretically license at most 1–2 harbors before competition is fierce.
The license economy creates C1 tension: the harbor you need is always slightly less
accessible than you'd like.

**Toll income:** When an opponent processes goods at your harbor, they pay the toll and
you gain those mana discs (placed in your reserve immediately). Harbor licensing is a
secondary income stream, not a primary one — but at 3–4 harbors controlled, toll income
can equal the VP from a late manifest completion.

---

## 9. Variants

### 3-Player Setup Adjustment
At 3p, reduce goods cubes per supply depot to 3 (from 4). Harbors have capacity 1
(unchanged). The scarcity differential is intentional — with fewer players, harbor queues
are shorter, so depot scarcity compensates.

### 4-Player Standard
No adjustments. The game is tuned for 4p. Harbor competition is most live at this count.

### 5-Player Extended
At 5p, add 2 fault-line hexes to the board (cover 2 standard channel hexes with fault-line
markers at setup). This increases contested interactions and compensates for the wider
information environment. Duration note: 5p adds ~20 minutes.

### Simultaneous Declaration (recommended at 4-5p)
All players declare all 3 actions simultaneously (write on a pad or declare behind a
screen). Reveal and resolve in turn order. Reduces inter-player wait time significantly.
Conflict resolution (harbor capacity, supply depot limits) still follows turn order.

### Short Manifest (introductory)
Use 6 manifest tiles per player instead of 12 (draw 1 per goods-type for 4 goods + draw
2 extras of player's choice). End trigger: 4 completions (not 6). Reduces weight to ~2.5
and duration to ~60 minutes. Not recommended for competitive play.

---

## 10. Designer Notes

**Why obligation asymmetry, not capability asymmetry?**
VIADUCT's B4 claim was about hidden capability (what your pipes can carry). ESTUARY's B4
claim is about hidden obligation (what you *must* deliver). The distinction produces
different player behavior: in VIADUCT, information cost is about inferring whether an
opponent can block you; in ESTUARY, it's about inferring what they *need* from the same
goods supply you need. The same Salt supply hex is valuable to two players for different
reasons — and neither player knows the other's exact reason unless they pay to find out.

**Why partial information rather than full concealment?**
Making ships' cargo visible (which goods types, how many) creates a middle layer of
legible information. You see the *what* (4 Timber cubes) but not the *why* (which manifest
cell drives it). This intermediate visibility is more interesting than full concealment:
the inference task is tractable (you can reason about likely manifest structures) but not
trivial (you don't know the exact value). Observation then becomes a precision tool rather
than a necessity.

**Why the Tide Wheel over a doom-clock?**
A doom-clock (like CODEX's Veil) creates pressure through scarcity of time. The Tide
Wheel creates pressure through scarcity of *access* — the same resource (channels,
harbors) is available at some moments and closed at others. The distinction is
architectural: a doom-clock ends the game; a tide wheel reshapes the game's geography
periodically. Both earn B2 (Catastrophe Pressure) and C1 (Tension Budget) but via
different registers. ESTUARY earns D4 (Late-Game Lock-in) because the Tide's position
at the game's two-thirds mark typically forecloses specific delivery routes permanently
for the remaining rounds.

**The 6-of-12 end trigger:**
Completing 50% of manifest cells is the end trigger — not all 12. This means a player
can win with a focused strategy (dominate 6 specific high-value cells) or a broad strategy
(complete many low-value cells quickly for tempo). The design deliberately makes both
paths viable, which feeds A7 (Emergence) and A30 (Strategy Ergodicity): the correct
manifest-completion strategy varies with what your opponents are doing and what your
manifest distribution is.

---

## 11. Collision Adjacency Chart

**Predicted new OP pair (2nd corpus instance):**

| Pair | Register distinction | Source |
|---|---|---|
| B1↔B4 | Pipeline architecture (B1: the 5-stage chain) vs. information economy (B4: cost to learn what the chain is working toward). Both fire on the cargo-manifest mechanism at different analytical scales — B1 describes the structure; B4 describes what animates it. | Established at VIADUCT #53 (26th TP, 5-3 vote). ESTUARY as 2nd corpus instance would confirm reliability. |

**Additional predicted adjacencies (new or confirmed):**

| Pair | Prediction | Confidence |
|---|---|---|
| B1↔D4 | Pipeline lock-in: committing a ship route into a harbor queue forecloses that ship from the Tide-closed route. Established pair (Brass #57, 30th TP). Would confirm. | High |
| B4↔A2 | Information cost × decision density: the decision to Observe is a compounding calculation (what do I learn, what does it cost, what can I do with it). New pair candidate. | Medium |
| C1↔B4 | Tension budget × information cost: manifest concealment means you're always slightly uncertain whether the mana cost of Observe is worth it. Adjacent registers (both fire on the "is this worth it?" moment). | Medium |

**Adjacency chart note:** The B1↔B4 OP is ESTUARY's primary claim. If the panel votes OP
here for the second time, it becomes a confirmed reliable pair and may enter the adjacency
chart at `personas/adjacency-chart.md` as a standard adjacency.

---

## 12. Open Questions at DESIGN Stage

1. **Manifest tile value distribution:** The current bag gives each goods×port combination
   one tile of each value 1–5. This means the total manifest value range is 12–60 per
   player (12 cells × 1 to 12 cells × 5). Average is 36 VP in potential manifest value.
   **Question:** Is 36 VP potential too high relative to the harbor bonus VP? Target:
   manifest VP should be ~65% of total game VP. Calibrate at first playtest.

2. **Observation cost calibration:** 3 mana feels right for a 3-mana-economy game, but
   at 5p with more mana flowing, 3 mana might be too cheap. **Question:** Should
   observation scale with player count (3p: 3 mana, 4p: 3 mana, 5p: 4 mana)?

3. **Harbor license claim cost:** 4 mana for a harbor license is expensive early (when
   mana is scarce) and cheap later (when toll income compounds). **Question:** Should the
   cost scale with how many harbors the player already controls? (1st harbor: 3 mana,
   2nd: 4 mana, 3rd: 5 mana, etc.) This would prevent one player from monopolizing
   harbors early.

4. **Fault Line hex count:** 2 fault lines for 3-4p (standard), 4 fault lines for 5p
   (variant). **Question:** Are fault lines interesting or just an irritant? Test with
   0 fault lines in first playtest; add if spatial interaction is insufficient.

5. **Deliver action cost:** Currently 0 (free to declare delivery if already docked).
   Should delivery cost 1 mana (matching goods type) to model unloading fees? This would
   add another tension layer but might over-complicate the late-game.

6. **B3 depth risk:** Current design has depth-2 transformation (raw goods → processed
   goods at one harbor). If a playtest reveals that players naturally route through two
   harbors in sequence (possible via the channel layout), depth-3 chains might emerge
   emergently. **Question:** Should the board layout explicitly enable OR prevent
   multi-harbor chains?

7. **End-trigger pacing:** 6-of-12 completions. At 3p with fewer manifest interactions,
   is this too fast (game ends in 7-8 rounds, Tide Wheel doesn't cycle through Low Tide)?
   **Question:** Should 3p end trigger be 5-of-12?

8. **Wild mana usage:** Wild mana discs currently count as any type. Should wild mana
   be restricted from Observe (to prevent a "just use 3 wilds" solution)? This would
   make information costs more type-specific and require building the correct mana economy
   for the information you need.
