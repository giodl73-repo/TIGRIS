---
name: CANTON — Design (Rules)
slug: canton-design
game: 0024-canton
stage: design
version: 1.0.0
rubric_version: v2.20
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: kramer-kiesling
anchor_axis: D2
counter_pressure_primary: A7
---

# CANTON — Rules

## 1. Overview & Object of the Game

CANTON is a 2–4 player territorial Euro played over three Eras. Players control asymmetric factions competing for influence across a modular hex-tile map of canton territories. At the end of each Era, players score points for the territories they control — but what territories are worth, and how they are scored, depends on the faction you are playing.

The game ends after Era 3. The player with the most VP wins. Ties broken by most territories controlled; further ties by faction-specific tiebreak ability (stated on faction card).

**The game's core tension:** the action-point budget per Era is shared — the Era ends when a fixed AP pool is exhausted, meaning every action you take advances the game toward scoring. Playing aggressively (many actions, burning APs) scores your current territory lead but ends the era before opponents can react. Playing conservatively risks opponents reaching the scoring threshold first. The faction you hold changes which choice is optimal.

## 2. Components

- **20 Hex-map tiles** (each with 2–4 Territory spaces and edge connections)
- **6 Faction cards** (one per faction; double-sided for faction-ability reference)
- **6 Faction markers** (×12 Control Markers per faction color; 72 total)
- **1 Era track board** (3 eras; AP pool tracker; scoring reminder)
- **1 AP pool token** (shared; advances each time a player spends APs)
- **1 VP score board** (0–60 range)
- **6 VP markers** (one per faction color)
- **20 Blocked Territory tokens** (used by some faction abilities)

## 3. The Six Factions

Each faction is defined by an **Active ability** (optional action, AP cost), a **Passive ability** (always in effect), and an **Era bonus** (extra scoring at era end).

| Faction | Active ability | Passive ability | Era bonus |
|---|---|---|---|
| **Farmer** | None (extra 1 AP per turn instead) | Scores +1 VP per controlled territory at era end (base) | +2 VP per fully-surrounded territory (all edges controlled or blocked) |
| **Merchant** | Pay 2 AP: gain 1 VP immediately (income action) | Scores territories in chains of 2+ for +1 VP bonus per chain | +3 VP per pair of non-adjacent controlled territories (long-range network) |
| **Soldier** | Pay 1 AP: remove 1 opponent's marker from an adjacent territory (contest) | May place markers in already-contested territories (ignores contest lock) | +4 VP to the player who contested (successfully removed) the most opponent markers this era |
| **Engineer** | Pay 2 AP: place a Road token on a map edge (extends adjacency by 2 edges) | Road tokens you placed count as your control for adjacency scoring | +2 VP per Road token on the board at era end |
| **Diplomat** | Pay 1 AP: copy one territory scored by another player this era (scores it a second time) | Cannot be targeted by Soldier's contest action on turns when you take the Diplomat action | +5 VP if you controlled at least 1 territory in every map quadrant at era end |
| **Scholar** | Pay 2 AP: place a Monument on a territory you control (×3 monuments total) | Monuments count as 2 territory spaces for scoring | +3 VP per Monument still on the board at era end |

## 4. Setup

1. **Map construction**: Draw 12 hex tiles from the shuffled map deck. Arrange them randomly in a roughly circular layout (each tile adjacent to 1–3 others). All Territory spaces start empty.

2. **Faction selection**: Shuffle all 6 faction cards. Deal 1 faction card face-up to each player, then deal each player a second card face-up. Each player chooses 1 to keep; the rest are returned to the box. *(Alternative: all 6 face-up, players draft in reverse turn order.)*

3. **Starting placement**: In turn order, each player places 2 Control markers on any empty Territory spaces on the map.

4. **AP pool**: Set the AP pool token to the starting position (12 APs at 2-player; 16 APs at 3-player; 20 APs at 4-player).

5. **Era marker**: Place on Era 1.

## 5. Turn Structure

Each Era proceeds until the AP pool is exhausted. On your turn, you have **3 Action Points** (modified by faction passive if applicable — Farmer gets 4 APs per turn).

Spend APs on any combination of:

### Actions (per-turn menu)

| Action | AP Cost | Effect |
|---|---|---|
| **Place marker** | 1 AP | Place 1 Control marker on any empty or contested Territory space on the map. |
| **Reinforce** | 1 AP | Place a second marker on a Territory you already control (it becomes Fortified; requires 2 opponent markers or 1 Soldier-contest to dislodge). |
| **Activate faction ability** | 1–2 AP (see faction card) | Use your faction's Active ability once per turn. |
| **Pass** | 0 AP | Voluntarily end your turn; all remaining APs are added to the AP pool (speeds the era). |

After you spend your APs, advance the AP pool tracker by the number of APs you spent. If the pool reaches 0, the current Era ends immediately (even mid-turn).

## 6. Era Scoring

When the AP pool reaches 0, score all factions simultaneously:

1. **Territory control**: count each Territory space you control (a space is controlled if you have the most markers on it; ties give 0 to all tying players).
2. **Faction passive**: apply your faction's passive ability modifier.
3. **Era bonus**: apply your faction's era bonus.
4. **Record VP**: update the score track.

**Between Eras**: remove no markers (territory control persists). Refill the AP pool. Advance Era marker. If Era 3 was just scored, proceed to Final Scoring.

## 7. Final Scoring (after Era 3)

No additional scoring beyond Era 3 results. The VP total from all three era-scoring events is the final score. Winner is the player with the highest VP. Ties broken as stated in §1.

## 8. Edge Cases & Clarifications

- **Fortified territories**: a Fortified territory requires 2 opponent markers to contest (not 1). Only the Soldier's Active ability can remove a Fortified marker with 1 contest action (ignores fortification).
- **Tie-controlled territory**: if two players have equal markers on a Territory, no one controls it. The territory does not score for anyone.
- **Diplomat ability timing**: the Diplomat copies a territory scored "this era." The copy is applied at era end, not during the era. The Diplomat may not copy the same territory twice.
- **Scholar's Monuments**: if a Monument is on a territory, it counts as 2 spaces for the Scholar's scoring only. Other factions score the territory as 1 space.
- **Road tokens (Engineer)**: edges with Road tokens count as adjacent edges for all adjacency calculations. Roads belong to the Engineer who placed them; they score for the Engineer at era end regardless of who controls the adjacent territories.
- **AP overspend**: you may not spend more APs than the pool contains. If the pool has 1 AP and you want to spend 2, you may only spend 1 (and the era ends after your action).
- **Faction ability once per turn**: each faction's Active ability may be used once per turn, not once per AP block. Farmer's passive (+1 AP/turn) applies before the turn begins; Farmer effectively gets 4 APs regardless of faction ability use.

## 9. Variants

**Expert draft**: instead of 2-deal-choose-1, lay all 6 faction cards face-up. Players draft in reverse turn order (last player picks first). Adds pre-game strategic depth; recommended after 3+ plays.

**Closed map**: instead of random tile draw, use a fixed canonical map arrangement (12 tiles in a predetermined layout). Reduces per-game setup variety but eliminates map variability as an explanatory factor for strategy differences — useful for pure faction-strategy testing.

**Longer campaign**: play 2 games back-to-back, with players selecting a DIFFERENT faction for game 2. The player with the highest combined VP wins. Tests whether faction mastery compounds.

## 10. Designer Notes — A7 Counter-Pressure Documentation

*Required for v2.18 retirement-reversal eligibility. Documents A7 Emergence-Replayability as an explicit counter-pressure design target.*

**Counter-pressure target: A7 Emergence-Replayability** — contested-watch at 9E/1R after Pandemic Legacy #22 refuted via the no-replayability argument. GARNER #23 completed counter-pressure cycle 2 game #1 via variable-contract-deck structural replayability. CANTON is cycle 2 game #2, using a **different mechanism**: asymmetric faction identity.

**CANTON's counter-pressure mechanism — faction identity replayability:**

GARNER's A7 argument was: "the game demands different things from you each play because the scoring landscape changes." CANTON's A7 argument is: "you ARE a different strategic entity each play because your faction is different."

With 6 factions and 2–4 players, the number of distinct faction combinations is:
- 2-player: C(6,2) = 15 unique matchups
- 3-player: C(6,3) = 20 unique matchups
- 4-player: C(6,4) = 15 unique matchups

Each combination produces a qualitatively different game because faction abilities interact with each other: the Diplomat is much stronger when a Scholar is present (Monuments are valuable targets to copy); the Soldier is less useful against an Engineer (Roads are harder to contest than markers). These faction synergies and counters create a strategic meta-game that unfolds differently each session.

Additionally: 12-of-20 hex tiles per game provides compounding variability. A Farmer on a compact map (few adjacency edges) plays differently than a Farmer on a sprawling map (many edges, hard to surround territories).

**For v2.18 counter-pressure eligibility, the panel argument must confirm both:**
- A7 earns via the faction-identity replayability mechanism (≥1 defend mark, citing specific faction interactions)
- A7 earns with at least one collision-vote confirmation (OP or CR against an adjacent axis)

The designed collision candidate: **D2↔A7** (K-K anchor vs. Lacerda's A7). Does spatial territorial control (D2) subsume faction-based replayability (A7), or are they orthogonal? D2 asks "does the map create meaningful spatial interaction within a play?" A7 asks "does faction identity make each play feel like a different experience?" These may be OP (D2 = within-play spatial mechanics; A7 = cross-play identity change) or CR depending on the panel's reading.

## 11. Collision Adjacency Chart

| Axes | Adjacency type | Reasoning |
|---|---|---|
| **D2 ↔ A7** | temporal-register (candidate) | D2 = spatial mechanics within a single play (adjacency, control, blocking); A7 = faction identity across multiple plays. Both are active in CANTON but at different temporal scales. Similar to C6↔A7 at GARNER and A6↔C8 at Pandemic Legacy. |
| **D2 ↔ A3** | co-firing (existing) | Spatial control creates direct interaction (blocking, contesting); D2 and A3 fire simultaneously on territorial disputes. |
| **D1 ↔ C7** | co-firing (existing) | Faction gradient (D1: Farmer accessible; Diplomat complex) maps onto action-menu clarity gradient (C7: simpler factions have simpler menus). |
| **B2 ↔ A3** | co-firing (existing) | Territory catastrophe (losing a critical chain to blocking) is a direct interaction event. |

## 12. Open Questions at Design-Stage

1. **Faction balance**: 6 factions is the minimum for meaningful combination variety (15 unique 2p matchups). But balance-testing 6 asymmetric factions is non-trivial. Known risk: Scholar's Monument (counts as 2 spaces) may be too powerful in compact maps.
2. **AP pool scaling**: 12/16/20 APs at 2/3/4p. Does this produce equivalent era lengths, or do higher player counts feel too short (APs drained faster per player)?
3. **Farmer's extra AP**: giving Farmer 4 APs/turn compensates for having no active ability. Is this the right compensation, or should Farmer have a weak active ability instead?
4. **Diplomat copy timing**: copying an opponent's territory at era end is powerful but creates a "wait-and-see" passive playstyle. Should the copy be limited to territories scored above a threshold?
5. **Map tile variety**: 12 of 20 tiles gives C(20,12) = 125,970 possible map configurations. Is 20 the right deck size, or would 16 tiles (12 used; C(16,12) = 1,820 configurations) be more manageable to balance?
