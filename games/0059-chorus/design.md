---
name: CHORUS — Design
slug: chorus-design
game: 0059-chorus
stage: design
version: 1.0.0
rubric_version: v2.23.30
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: stegmaier
anchor_axis: D3
---

# CHORUS — Design

## Overview

CHORUS is a 2–6 player cooperative scoring game. Players collectively build a Harmony score by activating Voices. Each Voice performs one action per round. The game ends after 6 rounds; total Harmony score wins (cooperative against a threshold score).

**Weight:** 2.0. **Length:** 30–45 min. **Players:** 2–6 (identical rules).

---

## Modular Voice System (D3 mechanism)

| Player count | Voices per player | Total voices |
|---|---|---|
| 2p | 3 Voices each | 6 total |
| 3p | 2 Voices each | 6 total |
| 4p | 1-2 Voices each (alternate) | 6 total |
| 5p | 1-2 Voices each | 6 total |
| 6p | 1 Voice each | 6 total |

Always 6 Voices active regardless of player count. Identical rules: each Voice takes 1 action per round; Harmony is scored identically at all counts.

---

## Actions (per Voice, per round — simultaneous)

All players simultaneously select actions for their Voice(s):

### Compose
Add 1 Note card from the central Chord pile to your Voice's sheet. Notes come in 4 types (Soprano, Alto, Tenor, Bass).

### Conduct
Play 1 Note from your Voice's sheet to the Harmony board. Matching the current Chord type = Harmony +2; mismatching = Harmony +1 but Dissonance +1.

### Sustain
Hold a chord: all matching Notes currently played to the Harmony board score +1 additional Harmony for each identical Note.

### Rest
Recover 1 action token. Does not advance Note collection but stabilizes the next round.

---

## Harmony scoring

At end of each round: if total Harmony this round ≥ target (6), score +1 Chorus point. Chorus points × round multiplier = final cooperative score.

**D3 mechanism proof:** At 2p, each player coordinates 3 Voices — the strategic complexity is high (managing 3 action selections). At 6p, each player manages 1 Voice — coordination with the group is the skill. At 4p, 2 Voices each — intermediate complexity. The game is architecturally designed so that no count is "easier" or "harder" — just differently coordinated.
