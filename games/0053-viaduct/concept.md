---
name: VIADUCT — Concept
slug: viaduct-concept
game: 0053-viaduct
stage: concept
version: 1.0.0
rubric_version: v2.23.24
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: lacerda
anchor_axis: B1
hopper_source: HOP-015
---

# VIADUCT — Concept

## Premise

VIADUCT is a 3–4 player aqueduct-engineering Euro where each player manages a private pipe network feeding into a shared city grid. Players know their own network's flow capacity (a set of hidden Capacity cards) but cannot inspect opponents' cards without spending an action. Optimal routing requires inferring what rivals need before they cut off your water supply. Every pipe laid is permanent; every district delivered earns VP; every undelivered district at game end scores against you.

The city grid is a shared hex map with 12 districts demanding water. Each player has a private spring location (random at setup) and must route pipes through the public grid to reach the most valuable districts. As more pipe networks are revealed each session, the initial routing calculation grows longer — players must plan further ahead because the revealed topology is denser.

## Players and length

- **Players:** 3–4
- **Length:** 75–100 min
- **Weight:** 3.5

## Anchor mechanic

**System Gearing (B1).** The aqueduct pipeline is explicitly 4-gear: Survey (reveal a hex) → Lay Pipe (connect hex to your network) → Flow (deliver water to a city district) → VP (score the district). Each gear depends on the prior: you cannot Flow without Pipe; you cannot Pipe without Survey. Remove any stage and the scoring machine collapses. Lacerda's B1 canonical architecture on a hydraulic substrate.

## Artifact-as-story

**The hex map grows each session.** At setup, the map has only 6 of 12 districts revealed; the other 6 are face-down. When a player Surveys a hex, that district is revealed permanently. The revealed topology creates the routing puzzle — and because previous plays leave hexes already revealed on the board (tracked via a session log sticker system), each play of VIADUCT features a denser network. The game is designed so that the 4th or 5th play of the same copy is the "richest" play — more revealed topology means longer initial routing calculations before the first Lay Pipe action. **This is the C8 targeted-earn mechanism: First-Turn Compression increases each session as the revealed map grows.**

## Inspiration / lineage

- **Lacerda / Lisboa (2017)** — multi-stage production with private hidden objectives and public shared scoring. VIADUCT borrows the hidden-capacity / public-grid tension.
- **Rosenberg / Le Havre (2008)** — resource chains with permanent action availability. VIADUCT borrows the "permanent pipe = permanent capability" model.
- **Simmons / Ticket to Ride (2004)** — route claiming on a shared map. VIADUCT borrows the spatial blocking via first-mover pipe placement.

## Target review in the TIGRIS pipeline

**Earn-candidates:**
- **B1 System Gearing (anchor; Lacerda)** — Survey → Lay Pipe → Flow → VP. 4-gear pipeline.
- **B4 Information Transparency Cost (Lacerda secondary)** — Capacity cards are hidden. Inspect action costs 1 action to peek at one opponent's card. Information asymmetry is an active economy.
- **C8 First-Turn Compression (CRITICAL TARGETED EARN)** — Session setup grows each play. §10 documents: by session 3+, the revealed map creates a Turn-1 routing calculation that is immediately strategic (multiple viable paths compete; player must commit before knowing opponents' routes). C8 earns when the initial routing choice is non-guided. Targeted earn after 8 games (last earned Gloomhaven #45).
- **D2 Spatial-Interaction Presence** — Hex pipe adjacency creates direct spatial interaction; a player who lays pipe through a key junction cuts off all rivals who need that junction. Blocking is structural, not incidental.
- **A3 Interaction** — Route blocking via pipe placement; opponents can cut off your water supply. Mandatory indirect competition.
- **A2 Decision Density** — Per-turn: which hex to Survey, which pipe segment to lay, which district to Flow to, whether to Inspect opponent capacity. High density.
- **C1 Tension Budget** — Flow tokens are limited (8 per player total). Budget creates perpetual scarcity.
- **A6 Teachability** — 6 actions (Survey, Lay Pipe, Flow, Inspect, Upgrade, Pass); each has a clear hydraulic metaphor.
- **D3 Count-Robustness** — 3–4 players; hex map scales via revealed district count.
- **D4 Late-Game Lock-in** — Committed pipe routes lock strategy by game midpoint (you can't un-lay pipe).
- **C7 Action-Menu Clarity** — 6 actions with clear costs.

**Collision candidate:**
- **B1↔B4 OP** — pipeline production process (B1: the 4-gear chain that defines how water moves) vs. information cost of hidden capacity (B4: what you don't know about the chain your rivals are running). B1 = the architecture of the machine; B4 = the cost of the information about rivals' machines. Different analytical registers of the same aqueduct mechanism.

## §10 — C8 First-Turn Compression Design Note

**This section is explicitly required per CLAUDE.md monitoring protocol.**

C8 was last earned at Gloomhaven #45 (8 games ago by #53). Dormancy-watch had been cleared at #45; by game #53 that's 8 games since the last earn, putting C8 back in monitoring range.

VIADUCT is designed to earn C8 via the growing-session-map mechanism:
- Session 1 (fresh copy): 6 of 12 hexes revealed. Turn 1 routing decision has limited complexity. C8 marginal.
- Session 3+: 9-10 hexes revealed. Turn 1 routing decision requires planning through multiple revealed junctions. The first routing choice is a live strategic commitment (which spring to prioritize, which junction to claim before opponents). Turn 1 IS the game's key decision.
- Session 5+: all 12 hexes revealed (or nearly). Turn 1 routing rivals Brass Birmingham's opening-turn complexity.

C8 earns specifically on sessions 3+ (which represent the canonical play state for weight-3.5 games in the TIGRIS corpus). Design §10 documents this as the intended C8 earn mechanism.

## Non-goals

- No hidden-role or traitor mechanics.
- No randomness after setup (all variance is from pipe topology and Capacity card distribution).
- VIADUCT does not claim B5 Architectural Novelty (route-claiming hex-pipe is established; see Ticket to Ride, Pipeline the game).
- Not a cooperative game.

## Open questions (to resolve during DESIGN)

1. **Upgrade action scope**: should Upgrade increase a single pipe's capacity (per-segment) or the player's entire network capacity (global)? Per-segment is more granular and strategic but adds bookkeeping.
2. **Inspect cost calibration**: 1 action for 1 opponent's capacity card. Is this the right cost? Too cheap = information asymmetry collapses; too expensive = B4 never matters because players never Inspect.
3. **District value distribution**: how many high-value (5 VP) vs. low-value (1 VP) districts should there be? More spread = more competition for key districts; flatter = less blocking incentive.
4. **Flow token count**: 8 per player for 3–4 players, 75–100 min. Enough to sustain meaningful decisions without the game ending before routing is established?
5. **Session-sticker legacy**: the growing-session-map mechanism requires some tracking between sessions. Should this use stickers (lightweight legacy) or a session log card?
