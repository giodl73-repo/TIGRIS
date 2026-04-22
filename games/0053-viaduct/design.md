---
name: VIADUCT — Design
slug: viaduct-design
game: 0053-viaduct
stage: design
version: 1.0.0
rubric_version: v2.23.24
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: lacerda
anchor_axis: B1
---

# VIADUCT — Design

## Overview

VIADUCT is a 3–4 player hex-based aqueduct-engineering Euro. Players build private pipe networks feeding into a shared city grid. Each player has hidden Capacity cards defining their network's maximum flow rate. The shared city grid has 12 districts; revealed districts can be delivered to for VP. Pipe placement is permanent; district delivery is permanent. Game ends when any player exhausts their Flow tokens (8 per player) or the city grid is fully delivered.

**Weight:** 3.5. **Length:** 75–100 min. **Players:** 3–4.

---

## Components

- 1 hex map board (4×6 grid; 24 hexes total; 12 district hexes + 12 terrain hexes)
- 80 Pipe segments (20 per player; 4 colors)
- 12 District tiles (face-down at start; 6 pre-revealed; 6 hidden)
- 4 Spring tiles (1 per player; placed randomly at setup)
- 4 Capacity card decks (10 cards each; 1 deck per player)
- 32 Flow tokens (8 per player)
- 4 Upgrade cubes (1 per player)
- VP track

---

## Setup

1. Place the hex map. Reveal 6 of 12 district tiles (random). Leave 6 face-down.
2. Each player chooses a Spring tile and places it on an edge hex (at least 3 hexes from any other spring).
3. Deal each player 5 Capacity cards face-down (private hand). The remaining 5 go face-down as the player's draw pile.
4. Each player takes 8 Flow tokens.

**Growing-session variant (canonical for weight-3.5 play):** On session 2+, additional district tiles may be pre-revealed based on the prior session's log. See §10.

---

## Turn structure

On your turn, take **2 actions** from the following menu:

### Survey
Reveal one face-down district tile on the hex map. The district's VP value and resource requirement become public. You may Survey any face-down hex (not just adjacent ones).

### Lay Pipe
Place 1 pipe segment on an unreserved terrain hex adjacent to your existing network (or adjacent to your Spring tile on the first placement). Pipe segments are permanent and belong to you; no other player may use your pipe.

*Restriction:* You may not lay pipe on a hex already occupied by another player's pipe.

### Flow
Spend 1 Flow token. Deliver water from your Spring through your pipe network to one connected district tile. To Flow to a district, you must have a continuous pipe path from your Spring to that district's hex. Collect VP equal to the district's value. Flip the district tile to its scored side.

*Restriction:* Each district may only be scored once (first player to Flow to it scores it; no subsequent scoring).

### Inspect
Spend 1 action. Look at one of your opponents' Capacity cards (they choose which card they show you). The card is immediately returned face-down. You may not Inspect the same player twice in the same round.

### Upgrade
Discard one Capacity card from your hand face-up. Your pipe network's capacity increases by 1 (tracked by your Upgrade cube). Upgrades are permanent. At capacity 2+, you may Flow to 2 districts in a single Flow action.

### Pass
Take no action this turn (unusual but legal in last-gasp situations).

---

## Capacity cards

Each player's 5-card hand represents their network's private flow constraints. Capacity cards are either **High Flow (HF)** or **Restricted Flow (RF)**:
- HF card: your network can Flow to a district up to 4 hexes from your Spring in a single path.
- RF card: your network can only Flow to a district up to 2 hexes from your Spring in a single path.

When you need to Flow, you must have at least 1 HF card in hand (or the relevant path length must be ≤ 2 hexes, satisfying RF). Cards are not spent on Flow — they're permanent hand information. The hand IS the capacity limit.

**Why this matters (B4 mechanism):** Opponents can see how many cards you hold but not their types. An opponent with 3 HF + 2 RF can reach distant districts; an opponent with 1 HF + 4 RF is limited to nearby districts. Inspecting reveals one card's type.

---

## Scoring

- **Delivered district:** VP printed on tile (1–5 VP)
- **Longest network bonus:** player with most pipe segments on board at game end earns +3 VP
- **Undelivered district penalty:** each district tile that remained face-down (never Surveyed) at game end: no penalty. Each district Surveyed but not delivered by game end: -1 VP to the player who Surveyed it (you revealed it, so you're accountable for not delivering).

---

## End condition

Game ends when:
- Any player has placed all 8 Flow tokens (exhausted), OR
- All 12 district tiles have been delivered (all face-up scored).

All remaining players take one final turn, then score.

---

## §10 — C8 Design Note: Growing Session Map

This section documents the C8 First-Turn Compression targeted-earn mechanism.

**Session 1:** 6 of 12 hexes pre-revealed. Turn 1 routing decision: limited complexity. Players choose a pipe direction with imperfect knowledge of the full map. C8 present but not dominant.

**Session 3+ (canonical weight-3.5 play state):** 9–10 hexes revealed based on session log. Turn 1 routing decision: players must plan routes through a dense known topology. Multiple viable paths compete; the first player to claim a key junction (a terrain hex that two players both need) makes a strategic commitment that shapes the entire game. Turn 1 routing IS the key decision.

**Design intent:** The growing-session map converts VIADUCT from a game of discovery (session 1) into a game of precision routing (session 3+). At the canonical play state, first-turn pipe placement requires immediate strategic engagement — no guided warm-up. C8 earns at this play state because the first-turn decision is non-guided and load-bearing.

**Session log mechanism:** A small session-log card tracks which district tiles were revealed in each play. Before session N+1, reveal all districts that appeared in any prior session. This creates growing map density without full legacy commitment.
