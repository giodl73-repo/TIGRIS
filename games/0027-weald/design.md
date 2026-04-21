---
name: WEALD — Design (Rules)
slug: weald-design
game: 0027-weald
stage: design
version: 1.0.0
rubric_version: v2.22.2
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: kramer-kiesling
anchor_axis: D2
counter_pressure_primary: A3
---

# WEALD — Rules

## 1. Overview & Object of the Game

WEALD is a 2–4 player land-development game set in a shared forest. Players spend resources to develop terrain hexes, placing structures (Farms, Workshops, Mills, Manors) on the WEALD map. No position is permanent: any player may **displace** an occupied structure by paying the development cost +1 resource, returning the displaced piece to its owner's hand.

Players score VP for structures they hold on the board at game end. The player with the most VP wins. Ties broken by most remaining resource tokens; further ties by turn-order proximity to first player.

The game's core argument: the displacement mechanism creates mandatory inter-player interaction — every contested hex requires attention; ignoring opponents means your best positions will be overwritten before scoring.

## 2. Components

- **1 WEALD board** (hex grid; 40 developable terrain hexes + 8 impassable Rock hexes; printed with terrain types and roads)
- **40 Terrain markers** (one per developable hex; face-down until cleared; flip to reveal terrain type)
  - River hexes (10) — base development cost: 2 Food
  - Hill hexes (10) — base development cost: 2 Stone
  - Plains hexes (12) — base development cost: 1 Wood + 1 Food
  - Road-junction hexes (8) — base development cost: 1 Gold
- **4 Structure sets** (1 per player; each set contains):
  - 6 Farm tokens (brown)
  - 5 Workshop tokens (gray)
  - 4 Mill tokens (white)
  - 3 Manor tokens (purple)
- **Resource tokens**: Wood ×30, Stone ×30, Food ×30, Gold ×20
- **1 Supply board** (shared resource market; 4 stalls, one per resource type; refilled at round start)
- **1 Round track** (5 rounds)
- **1 VP score board**
- **4 VP markers**

## 3. Setup

1. Lay the WEALD board. All terrain hexes start face-down (hidden terrain type).
2. Each player takes their full set of 18 structures.
3. Starting resources (2p: 5 tokens each type; 3p: 4 each; 4p: 3 each).
4. Supply board: place 4 tokens of each resource type in each stall.
5. Determine first player randomly. Round track to Round 1.

## 4. Turn Structure

Each of the 5 rounds:

**Round start**: Refill each supply stall with 2 tokens of its resource type (up to a maximum of 6).

**Player turns** (clockwise, each player takes one action, until all have passed):

On your turn, choose ONE action:

### Actions

| Action | Cost | Effect |
|---|---|---|
| **Gather** | 0 | Take up to 3 resources from the supply board (any combination; no single resource type above 2 per Gather). |
| **Clear** | 1 Wood | Flip 1 adjacent face-down hex to reveal its terrain type. The hex is now available for development. (May only develop cleared hexes.) |
| **Develop** | Base terrain cost (see §2) | Place 1 of your structures on any cleared, unoccupied hex. |
| **Displace** | Base terrain cost +1 of any resource type | Remove an opponent's structure from a cleared hex; place your structure there. The opponent retrieves their piece to hand; you pay the premium. (Cannot displace your own structure.) |
| **Reinforce** | Base terrain cost | Pay the base cost again to mark a structure as Reinforced (place a token on it). A Reinforced structure costs +2 (not +1) to displace. Reinforcement is removed when the structure is displaced. |
| **Pass** | Free | End your turn for this round. Once all players have passed, the round ends. |

## 5. Scoring (game end, after Round 5)

Score all structures currently on the board simultaneously:

| Structure | Base VP |
|---|---|
| Farm | 2 VP |
| Workshop | 3 VP |
| Mill | 4 VP |
| Manor | 5 VP |

**Adjacency bonuses** (applied per structure):
- +1 VP if the hex is adjacent to any River hex (including diagonals)
- +1 VP if the hex is a Hill hex
- +2 VP if the hex is a Road-junction hex

**No VP for structures in hand** — only on-board structures score.

## 6. Edge Cases & Clarifications

- **Displacement chain**: if you displace an opponent, you may not then be displaced by the same opponent on their very next turn as a response (1-turn displacement moratorium per hex, preventing ping-pong). After 1 full round, any hex may be displaced again.
- **Reinforcement**: pays the base terrain cost again (same amount as original development). Reinforcement tokens are single-use; a displaced Reinforced structure loses its reinforcement.
- **First-time clear**: a hex that has never been flipped cannot be developed in the same action — Clear is a separate action from Develop.
- **Impassable hexes**: Rock hexes cannot be cleared, developed, or displaced. They are permanent obstacles.
- **Round 5 late-game pressure**: in Round 5, the Displace action is the most common; players race to displace opponents' high-value structures (especially Manors on Road-junctions) before scoring.
- **Running out of structures**: if your entire structure set is on the board, you cannot Develop more without displacing or being displaced to free up pieces.

## 7. Designer Notes — A3 Counter-Pressure Documentation

*Required for v2.18 retirement-reversal eligibility.*

**Counter-pressure target: A3 Interaction** — contested-watch at 8E/1R after NEST #26 refuted it via C6↔A3 CR (parallel-board multiplayer-solitaire). WEALD is counter-pressure cycle 3 game #1.

**WEALD's counter-pressure mechanism — displacement as mandatory interaction:**

NEST's refutation argument: "parallel life-boards produce multiplayer-solitaire — player A's tile placement doesn't affect player B's board." WEALD directly refutes this: **every hex is shared**. Player A's Manor on a Road-junction hex is a target Player B must address or accept losing the hex. There are no "my board" and "your board" — there is only THE BOARD.

The displacement mechanism makes interaction mandatory by structure:
1. **You cannot develop optimally without interacting**: the best hexes (Road-junction, Hill-River adjacency) will always be targeted by multiple players. Placing first doesn't secure a position — opponents will displace you unless you reinforce.
2. **Ignoring opponents costs you directly**: if you focus only on your own development and ignore opponents' growing Manor networks, they will outscore you without you having any recourse at game end.
3. **Displacement is always available**: unlike blocking (where opponents might not be able to reach you) or auctions (where opponents might run out of bids), displacement is always viable at +1 resource. The threat is constant.

**For v2.18 counter-pressure eligibility:**
- A3 must earn with ≥1 defend mark in the argument record (displacement mechanism = mandatory interaction evidence)
- A3 must earn with a collision-vote confirmation (OP or CR; the designed collision is D2↔A3)

**Designed collision: D2↔A3 (OP candidate)**
- D2 Spatial-Interaction Presence: the hex map creates spatial pressure — where you place matters because of physical adjacency to rivers, hills, and roads.
- A3 Interaction: the displacement mechanism creates social/adversarial pressure — what you place matters because opponents can take it from you.
- OP argument: D2 is about the LANDSCAPE (timeless spatial geometry of the map); A3 is about the CONTEST (turn-by-turn adversarial decisions). Different registers of the same mechanism. If the panel votes OP 5+, both earn and the collision-vote confirmation is satisfied.

## 8. Collision Adjacency Chart

| Axes | Adjacency type | Reasoning |
|---|---|---|
| **D2 ↔ A3** | temporal-register (candidate) | D2 = spatial geometry of the map (timeless); A3 = adversarial displacement contest (per-turn). Different registers of the hex-map mechanism. OP candidate. |
| **B2 ↔ D4** | co-firing (existing) | Catastrophe from displacement threat (B2) fires when high-value structures are at risk; lock-in (D4) fires when structures survive to scoring. |
| **A2 ↔ A3** | co-firing (existing) | High decision density (A2) is partly driven by the displacement calculation (A3): every action requires evaluating opponent displacement risk. |

## 9. Open Questions at Design-Stage

1. **Displacement cost calibration**: +1 of any resource type. Should the premium be type-specific (must match one of the resources used in the base cost) or free-type? Current: free-type (any 1 resource) — makes displacement accessible regardless of resource situation.
2. **Moratorium rule**: 1-round displacement moratorium prevents ping-pong. Is 1 round the right length? Too short → unstable; too long → displacement becomes less viable late-game.
3. **Reinforcement cost**: currently same as base development cost (a second payment). Should reinforcement be cheaper (½ base cost) to make it more common, or kept at full base cost to make it a meaningful commitment?
4. **Map reveal**: face-down terrain hexes add variance — you don't know if the adjacent hex is a high-value River or a Plains until you clear it. This creates tension between "clear now to know" and "save the Clear action for later." Good mechanism but may need balance testing.
5. **Structure count**: 6 Farms, 5 Workshops, 4 Mills, 3 Manors per player = 18 structures for 40 developable hexes at 4 players = 72 structures competing for 40 hexes. Overcrowded by design? May cause constant displacement in later rounds.
