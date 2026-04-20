---
name: Vigil — Design (Rules)
slug: vigil-design
game: 0018-vigil
stage: design
version: 1.0.0
rubric_version: v2.15
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: stegmaier
anchor_axis: A6
---

# Vigil — Rules

## 1. Overview & Object of the Game

Vigil is a cooperative horror game for 3–6 players with exactly one hidden Traitor. Players are **Watchers** holding the line against a **Monster** that wants to breach a threshold. Each round is a simultaneous-reveal action round; at the end of each round, the Monster acts. A **Haunting Track** (0–7) measures how close the Monster is to breaching.

**Watchers win if** (a) the Haunting Track reaches 0 at any point — the Monster is banished, **or** (b) round 8 resolves without the Haunting Track reaching 7.

**Traitor wins if** the Haunting Track reaches 7 at any point — the Monster breaches.

The core tension: the Watchers know one of them is compromised, but not who. The Traitor knows they are compromised — and knows their own actions will sabotage specific plays at specific Haunting states, without any choice about *when*. The game is pattern-reading under collective pressure, with a 10-minute teach.

## 2. Components

- **24 Base Action cards** (4 sets of 6, all identical faces): **Investigate**, **Ward**, **Flee**, **Pray**, plus 2 duplicates per deck. 1 base deck per player.
- **1 Watcher Variant deck (72 cards)** — shared pool, 18 cards per action type, each a small modifier ("+1 Investigate clue this round", "Ward counts twice if another Ward revealed", etc.).
- **1 Sabotage Variant deck (24 cards)** — same action-type backs as the Watcher variant deck; 6 cards per type, each with a **conditional sabotage clause** ("if Haunting ≥ 4, this Ward counts as 0", "if Clue ≥ 3, this Investigate adds +1 noise instead of +1 clue").
- **Haunting Track** — 8 spaces (0 through 7), 1 marker.
- **Clue Track** — 6 spaces (0 through 5), 1 marker. Milestones at 2, 4, 5.
- **Monster Script deck (12 cards)** — one drawn per round; dictates Haunting advance and special effects.
- **Clue Info deck (6 cards)** — face-up reveals at Clue milestones (Monster-script preview, Traitor-hint, banishment token, etc.).
- **Role cards** — 1 Traitor card + 5 Watcher cards; dealt secretly at setup.
- **Pray Penalty tokens** (1 per player) — track whether Pray is available this round.
- **Round counter** (1–8).

## 3. Setup

1. Place the Haunting Track at **1**. Place the Clue Track at **0**. Place the Round Counter at **1**.
2. Shuffle the Monster Script deck; place face-down.
3. Shuffle the Clue Info deck; place face-down.
4. Each player takes a set of 4 Base Action cards (Investigate / Ward / Flee / Pray).
5. Shuffle the Role deck: 1 Traitor card + (players − 1) Watcher cards. Deal one to each player secretly. Each player reads their role and returns the card to an envelope (do not reveal).
6. The Watcher Variant deck is shuffled and placed face-down as the **variant draw pile**. The Sabotage Variant deck is placed face-down beside it.
7. **The Traitor**, silently and out-of-turn (e.g., during a "lights-off" moment before round 1), draws 15 cards from the Watcher Variant deck, sets them aside face-down, and seeds the top of the Watcher Variant deck with 15 Sabotage cards shuffled in. The 15 removed Watcher variants go under the Sabotage deck. *(Ritualized Traitor-seeding; described as "the dark moment" in flavor. Mechanism ensures the Traitor's sabotage is randomly distributed through the round's draws without any Watcher being certain.)*
8. Round 1 begins.

## 4. Turn structure

Each round has 5 sub-phases, fixed order:

### 4.1 Draw phase
Each player draws **3 variant cards** from the (now-mixed) Watcher Variant deck into their hand. (Hand is private. Variants drawn by non-Traitor players are overwhelmingly Watcher-type; occasionally a Sabotage card surfaces.)

### 4.2 Select phase
Each player secretly selects exactly **1 Base Action card** + **0 or 1 variant card** to attach. The variant must match the base action type (Ward variants attach to Ward base, etc.). Place face-down in front of you. Discard the 2 unused variants face-down to a shared discard pile.

### 4.3 Reveal phase
All players flip their base + variant simultaneously. No discussion during reveal.

### 4.4 Resolve phase (fixed priority: Investigate → Ward → Flee → Pray)
1. **Total Investigate** across all plays (base + variant modifiers). Add that to Clue Track.
2. **Total Ward**. Cap the Monster's coming advance at (current Haunting − Ward total). If total Ward ≥ 3 **and** Clue Track ≥ 2, a **Banishment** fires: Haunting reduces by 2 immediately, before Monster phase.
3. **Total Flee**. Monster Script's Haunting advance this round is reduced by 1 per Flee (minimum 0).
4. **Total Pray**. Each Pray places 1 **stall token** on the top Monster Script card. Each player who played Pray places a Pray Penalty token on their own board — they cannot play Pray in round `current + 1`.

Sabotage cards resolve as part of the variant attached to their base. They do not announce themselves — they just fire per their printed clause. The Traitor may not voluntarily activate or deactivate them.

### 4.5 Monster phase
Reveal top of Monster Script deck. Apply its Haunting advance (after Flee reduction; after any stall-token cancellation). Apply its special text. Move Round Counter forward.

### 4.6 Discussion (60 seconds, untimed if 3p, 90s if 6p)
Open discussion. Accusations, pattern claims, suggestions for next round. Mandatory — the discussion phase is load-bearing for A3 Interaction. After the timer, play proceeds to next round.

### Win-check (continuous)
- If Haunting ≥ 7 at any moment: **Traitor wins**, game ends.
- If Haunting ≤ 0 at any moment: **Watchers win** (Monster banished), game ends.
- If round 9 would begin (8 rounds complete) and neither end-state reached: **Watchers win** by survival.

## 5. Actions

Four action types. The action menu's clarity is load-bearing for the A6 Teachability anchor.

| Action | Icon | Base effect | Total across players |
|---|:---:|---|---|
| **Investigate** | 🔎 | +1 Clue | Sum → Clue Track advance |
| **Ward** | 🛡 | Caps Monster advance | Sum ≥ 3 + Clue ≥ 2 → Banishment (Haunting -2) |
| **Flee** | 🏃 | Reduces Monster advance by 1 | Sum → line-of-sight reduction |
| **Pray** | 🙏 | Stall Monster 1 round + self-penalty | Sum → Monster delay tokens |

Each variant card modifies these numerically or conditionally. Variants cannot change action *type*, only magnitude/condition.

## 6. Scoring / end condition

**Binary.** No points. Watchers win or Traitor wins.

End-triggers:
- **Traitor win**: Haunting reaches 7.
- **Watcher win (banishment)**: Haunting reaches 0.
- **Watcher win (survival)**: 8 rounds complete without Haunting hitting 7.

The end-condition deliberately has **no tie-breaker** and **no score** — this is binary pressure, Min-Score-Shape (C2) negative-case, and Point-Salad (C6) negative-case.

## 7. Edge cases & clarifications

- **Sabotage-card detection**: Watchers cannot inspect a played variant after the round. Sabotage fires silently at resolution. Accusation is based on inference across rounds, not forensic inspection.
- **Clue Track at 5**: triggers the **Final Banishment** — Haunting immediately reduces by 3 (not 2). Clue Track resets to 0.
- **Pray Penalty stacking**: if you play Pray two rounds in a row somehow (e.g., via a Sabotage that forces it), you place a second penalty token and miss Pray for two rounds.
- **Variant deck exhaustion**: when the shared variant draw pile empties, reshuffle the discard pile to form a new draw pile. The remaining Sabotage cards are shuffled in proportionally.
- **3-player mode**: draws only 2 variants per round (not 3) to keep round duration similar to 4p.
- **Monster Script runs out**: can only happen at round 12+; game ends in round 8 regardless.

## 8. Special subsystems

### Clue Track

- Milestone 2: reveal Clue Info card 1 — top 3 Monster Script cards previewed face-up.
- Milestone 4: reveal Clue Info card 2 — one variant-deck sub-type is publicly revealed (e.g., "the Ward-inverter variant exists in this game's Sabotage deck"). Narrows Traitor inference.
- Milestone 5: triggers Final Banishment (Haunting −3, Clue resets to 0).

### Monster Script

12 unique cards. Each card has:
- **Haunting advance**: +1, +2, or +3.
- **Special effect**: "target one player — their next Investigate fails" / "Haunting cannot be Warded this round" / "duplicate top card for next round" / etc.

Script escalates over the deck's top-to-bottom order — early cards are mild, late cards are brutal. Deck is shuffled at setup to prevent memorization, but a bounded shuffle: cards 1–4 drawn from the top third, cards 5–8 from the middle, cards 9+ from the bottom.

### Pray Penalty tokens

Pray is powerful (stalls Monster) but self-costing. Penalty tokens enforce the cost without requiring other players to track it for you.

## 9. Variants

- **3-player variant**: as described in §7. Draw 2 variants per round.
- **5-player variant**: standard rules; 1 Traitor.
- **6-player variant**: 1 Traitor still; discussion phase extended to 90s.
- **Solo variant** (out of scope for v1.0): one Watcher + automated Traitor-AI deck that plays sabotage moves based on Haunting state. Deferred.
- **Expert variant**: after 3+ successful cooperative wins, enable "Double Compromise" — 2 Traitors drawn from role deck, but they do not know each other. Haunting Track extends to 9.

## 10. Designer notes

**The load-bearing bet is A6 Teachability under a hidden-role layer.** Every Stonemaier-lineage game in the top 100 earns A6 — Scythe, Wingspan, Viticulture — by making the teach-curve shallow. Vigil's hypothesis is that a 4-icon action menu + binary win condition + one hidden role still fits that curve, despite horror's genre-reputation for complexity. If the 10-minute teach fails, the anchor fails.

**A3 Interaction is the high-stakes secondary axis.** Post-Wingspan, A3 is at 2E/1R (adopted-contested-watch). Multiplayer-solitaire designs have been undermining it. Vigil's design decisions that specifically enforce A3:
- **Mandatory discussion phase** — 60-second untimed collaborative analysis per round. You cannot skip it.
- **Clue milestone info-drops** — shared information that forces collective reasoning.
- **Simultaneous reveal** — no order-of-play advantage; everyone reads the same public state.
- **Sabotage detection requires pattern inference** — cannot be solved individually; must be discussed.

If Vigil earns A3, it's a counter-pressure game against the Wingspan refute. If it doesn't — if discussion collapses into chat and the Traitor wins by noise — A3 takes a second refute-game and formalizes adopted-contested.

**Why Haunting starts at 1, not 0**: starting at 0 means the first round could win the game via a Banishment combo. Starting at 1 ensures at least 2 rounds of play before banishment is possible. Calibration choice.

**Why 4 actions, not 5**: K-K's C7 Action-Menu Clarity axis rewards ≤ 6 actions. We chose 4 because the icon set is memorable (🔎🛡🏃🙏), each action is single-sentence legible, and the variant layer provides depth. Each action also has a clear thematic role (learn / defend / hide / endure).

## 11. Collision Adjacency Chart

Vigil's mechanics create two candidate novel adjacencies:

- **A6 Teachability ↔ C7 Action-Menu Clarity** — for Vigil, earning A6 depends on the action menu's clarity; earning C7 depends on the teach-time. They are not orthogonal in traitor-coop designs; they gear together. Candidate for adjacency-chart update.
- **A3 Interaction ↔ B2 Catastrophe Pressure** — the Traitor's sabotage activates at Haunting state thresholds (conditional clauses). Interaction is *triggered* by catastrophe pressure; they are linked in this design, not merely adjacent.

Submit both to `personas/adjacency-chart.md` for I-te-01-style adjacency-cluster review after panel.

## 12. Open at DESIGN-stage, to resolve post-review

- **Haunting Track length (7 vs 8 vs 9)**: target is "haunting = 4 by round 4 at 4p is balanced." Panel likely tests by simulating rounds.
- **Variant deck total size (72 Watcher + 24 Sabotage = 96)**: with 6p × 3 draws × 8 rounds = 144 draws max, deck exhausts mid-round-6. Is the reshuffle disruptive or acceptable?
- **Sabotage deck's 24 cards**: 6 per action type. Is that too predictable for experienced players? Should it be 4 per type (16 total) for more detection difficulty?
- **Discussion phase duration**: 60s fixed for 3-5p, 90s for 6p. Should it scale smoothly with player count? Or be always 60s?
- **Pray Penalty severity**: skip-next-round only, or also impose a clue penalty (−1 Clue if all players have a penalty token)?
- **Clue milestones (2/4/5)**: should milestone 5 be a reliable Final Banishment, or should the Traitor have a sabotage that interferes with it?
- **Traitor's "quiet victory" path**: currently Traitor only wins at Haunting ≥ 7. Should they also have a victory if the game ends in round 8 with Haunting ≥ 5? This tightens the Traitor role but may make pure-survival wins impossible for Watchers.
- **Role card return vs keep**: Role cards currently returned to envelope at setup. Alternatively, kept-face-down in front of each player with a reveal trigger ("you may reveal your role at end of round 4 for a one-time effect"). Would this increase or decrease A3?
- **Does the Clue Track reset after Final Banishment?** Currently yes. But this caps mid-game banishments at 1 per game. Should it be a 1-time effect or renewable?
