---
name: HEARTH — Design (Rules)
slug: hearth-design
game: 0029-hearth
stage: design
version: 1.0.0
rubric_version: v2.23
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: stegmaier
anchor_axis: A6
---

# HEARTH — Rules

## 1. Overview & Object of the Game

HEARTH is a 1–5 player cooperative campaign game played across 8 sessions. Players work together to maintain a frontier settlement threatened by an escalating Threat track. Each session, players win by keeping the Threat at bay until a cooperative goal is met (varies per session). If the Threat reaches 8, the session is lost.

The game begins with 6 rules (Session 1). Each subsequent session, players open one sealed envelope that adds exactly 1 new Action and 1 new Role to the game. By Session 8, the full ruleset is in play.

**The core design promise:** every rule you learn in HEARTH arrives at the moment the game has created a felt need for it. You never read ahead; the game teaches you.

## 2. Components

**Always in play (Session 1):**
- 1 Settlement Board (shared; 5×5 hex grid with starting 4 terrain hexes revealed)
- 6 Action cards (Move, Gather, Build, Rest, Threat-Response, End-Trigger)
- 2 starting Role cards (Scout, Keeper)
- 1 Threat track (0–8 counter)
- 1 Event deck (20 cards; shuffle each session)
- Resource tokens: Wood ×20, Food ×20, Stone ×10
- 4 Structure tiles (Shelter, Storage, Watchtower, Hearth Stone)

**In sealed envelopes B–H (one per session 2–8):**
- Envelope B: Craft action card + Craftsperson role card + Rule insert B + Narrative card B
- Envelope C: Defend action card + Defender role
- Envelope D: Trade action card + Trader role + Trade token set
- Envelope E: Explore action card + Explorer role + new terrain hex tiles ×3
- Envelope F: Heal action card + Healer role + Ailment cards ×6
- Envelope G: Lead action card + Leader role + Morale track
- Envelope H: Commune action card + Elder role + Legacy Hearth Stone tile

## 3. Setup (Session 1)

1. Place the Settlement Board. Reveal 4 of the 25 hex spaces (the starting terrain): 2 Forest, 1 River, 1 Plains.
2. Lay out the 6 Session-1 Action cards face-up (these are the only actions available).
3. Assign 1 Role card to each player (Scout or Keeper; duplicate if >2 players).
4. Set Threat track to 0.
5. Shuffle Event deck; place face-down.
6. Give each player starting resources: 2 Food, 1 Wood.
7. Place the Hearth Stone structure tile in the center hex.

**Between sessions:** leave the board as-is; record resource totals on the Session Log; seal remaining envelopes. The next session begins from the saved state.

## 4. Turn Structure

Each session consists of rounds until the session's Win Condition or Lose Condition is met.

Each round:

### Phase 1 — Event
Draw 1 card from the Event deck. Apply its effect (usually: +1 Threat, or reveal a terrain hex, or add a resource). If the deck is exhausted, reshuffle.

### Phase 2 — Actions
In any order, each player takes **2 Actions** (chosen from available Action cards on the table). Players may discuss freely before acting. Actions:

| Action (Session 1) | Cost | Effect |
|---|---|---|
| **Move** | 0 | Move your pawn 1 hex in any direction. |
| **Gather** | 0 | Take 1 resource token from the hex you occupy (type = terrain type). |
| **Build** | 2 Wood + 1 Stone | Place a Structure tile on your current hex (one tile per hex). |
| **Rest** | 0 | Recover 1 resource from the supply; reduce Threat by 1 (min 0). |
| **Threat-Response** | 1 Food | Reduce Threat by 2. Can only be taken if Threat ≥ 3. |
| **End-Trigger** (cooperative) | — | Declare at start of your turn. All players vote: if unanimous, resolve win-condition check immediately. If met: Win. If not met: this action slot is forfeit. |

### Phase 3 — Threat Check
After all players have taken their actions: if Threat ≥ 8, the session is lost. If the Win Condition is met, the session is won.

## 5. Session Win Conditions (per session)

| Session | Win condition |
|---|---|
| 1 | Build 2 Structure tiles before Threat reaches 8. |
| 2 | (Revealed from Envelope B — involves Crafting) |
| 3–8 | (Revealed from envelopes C–H as sessions are played) |

## 6. Envelope Unlock Triggers

Each envelope has a **trigger condition** — a game-state the players recognize, after which they open the next envelope and read its Rule insert aloud before continuing.

| Envelope | Trigger condition | New rule summary |
|---|---|---|
| B (before Session 2) | Win Session 1 | Craft action: convert 2 resources → 1 Crafted good. Craftsperson: +1 resource when Crafting. |
| C (during Session 2) | Threat reaches 5 for the first time | Defend action: pay 1 Stone → reduce Threat by 3. Defender: Defend costs 0 Stone. |
| D (before Session 3) | Win Session 2 | Trade action: give 1 resource to another player; both take 1 extra resource. Trader: Trade triggers Gather. |
| E (during Session 3) | A player Gathers from every terrain type in one session | Explore action: reveal a hidden terrain hex adjacent to occupied hex. Explorer: Explore costs 0 actions. |
| F (before Session 4) | Win Session 3 | Heal action: remove 1 Ailment from a player. Healer: Heal costs 1 Food (not 2). |
| G (before Session 5) | Win Session 4 | Lead action: give your remaining action to another player. Leader: Lead can target any player, not just adjacent. |
| H (before Session 8) | Complete sessions 5, 6, and 7 | Commune action: trade resources with the Hearth Stone as a communal pool. Elder: Commune costs 0 resources. |

## 7. Edge Cases & Clarifications

- **Roles**: if a player holds a duplicate role (more than 2 players in sessions 1–2), the duplicate role's ability is inert for the player who already has that role (the second copy of Scout grants no additional Scout ability).
- **Non-destructive sealing**: envelopes B–H may be resealed after opening. The trigger condition printed on the exterior uses iconography only (not text); the full trigger text is on the Condition card inside. A new group using the same copy will not learn trigger conditions by reading the envelope exterior.
- **Session loss recovery**: a lost session is replayed from the session start state (resources reset to session-start log entry; Threat reset to 0). No permanent progress is lost on a loss — HEARTH does not punish failure permanently.
- **Solo play**: 1 player controls 2 Role cards simultaneously. All solo rules are identical to multiplayer except the player takes their 2 actions per role per round (4 actions total at session 8).
- **Maximum Threat**: if Threat reaches 8, read "The settlement falls" — the session is lost. Reset and replay.

## 8. Designer Notes — A6 Teach-by-Doing Documentation

**A6 Teachability** is HEARTH's anchor because the game's architecture IS a teachability delivery system:

Session 1 teach: "You have 6 actions. Move to gather resources. Build structures. Keep the Threat down. Win by building 2 structures." Under 5 minutes.

The incremental unlock sequence ensures that no rule arrives before the game has created a context for it:
- Envelope C (Defend) triggers only after Threat has reached 5 — meaning players have already experienced the threat of losing to high Threat before learning the primary defense mechanism. The rule lands with urgency.
- Envelope E (Explore) triggers only after players have touched every terrain type — meaning they've felt the constraint of a limited map before learning to expand it.

**A6's canonical earning condition**: the game can be taught to new players in under 5 minutes (Session 1); players who have never seen a rulebook can start playing immediately. This is HEARTH's strongest A6 claim.

**The comparison to Pandemic Legacy**: Pandemic Legacy is non-teachable mid-campaign (Session 8 requires the full campaign history). HEARTH is always teachable from Session 1 (start a new group from Session 1 any time). A6 and A7 are both stronger in HEARTH than in Pandemic Legacy.

## 9. Collision Adjacency Chart

| Axes | Adjacency | Reasoning |
|---|---|---|
| **A6 ↔ D1** | co-firing (strong) | A6 asks "can new players learn this?" (always yes; Session 1 is minimal). D1 asks "does expertise grow across play?" (yes; Session 8 is significantly more complex). Both fire on the same unlock sequence — candidate for OP or co-firing CR. |
| **A6 ↔ A7** | co-firing (candidate) | A6 fires on session-1 teachability; A7 fires on cross-campaign replayability (non-destructive envelopes mean the campaign can be replayed). Temporal-register OP candidate: A6 = entry teachability; A7 = cross-play replayability. |
| **B2 ↔ D1** | co-firing (existing) | Catastrophe pressure (B2: Threat escalation) is introduced progressively — Session 1 has mild threat; Session 8 has compound threat from multiple systems. D1's expert complexity is partially driven by B2's multi-level threat. |

## 10. Open Questions at Design-Stage

1. **Session 4–7 win conditions**: only Sessions 1–3 have explicit win conditions in this document. Sessions 4–7 use conditions revealed in envelopes F and G. Placeholder; will need full content development.
2. **Resource balance**: Session 1 starts players with 2 Food + 1 Wood. Is this enough? The first Build costs 2 Wood + 1 Stone — players need at least 1 more Wood before building. Does this create too much front-loaded gathering?
3. **Threat calibration**: Threat reaches 8 = loss. The Event deck has 20 cards; each Event averages +0.5 Threat. A session with 20 rounds would accumulate ~10 Threat without mitigation. Rest (−1) and Threat-Response (−2) are the main mitigation tools. Calibration needed.
4. **Envelope H timing**: Session 8 is described as "Complete sessions 5, 6, and 7." Is this the right trigger for the final envelope, or should it fire earlier?
