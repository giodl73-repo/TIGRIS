---
name: QUORUM — Design
slug: quorum-design
game: 0041-quorum
stage: design
version: 1.0.0
rubric_version: v2.23.12
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# QUORUM — Design

## Overview

QUORUM is a 2–5p voting game where influence tokens are the tension engine. 10 rounds; each round has 5 Proposals. Players simultaneously commit tokens from their hand (hidden behind a screen), then reveal. Proposals reaching Quorum (35% of tokens in 3p, 25% in 5p) pass — all backers score that Proposal's VP. Failing Proposals return 0; spent tokens gone.

## Setup

- Each player: 8 influence tokens (3p), 10 (4p), 12 (5p).
- Each round: draw 5 Proposal cards face-up. Each Proposal shows: VP value (2–8), Quorum threshold (printed as a % of total tokens cast), and a special effect (if any).
- All players have tokens replenished to hand-limit at round start.

## Core turn

1. **Commitment phase** (simultaneous): secretly distribute any number of tokens across any Proposals. Place behind screen.
2. **Reveal**: all players reveal simultaneously.
3. **Resolution**: for each Proposal, count total tokens committed. If total ≥ Quorum threshold, Proposal passes. All backers score VP proportional to their contribution (tokens contributed / total tokens × Proposal VP, rounded to integer, minimum 1 if Proposal passes).
4. **Return**: failing Proposal tokens lost. Passing Proposal tokens returned to owners.

## Special Proposals

- **Exclusive Proposal** (1-2 per round): only the player with the most tokens committed to it scores; others lose tokens.
- **Veto Proposal**: if this Proposal reaches super-quorum (50%), all other Proposals in the round fail.
- **Chain Proposal**: passing this Proposal carries over a token-bonus to next round for all backers.

## Tension design

The 8-token budget ensures FOMO across all 5 proposals. With 3 players each having 8 tokens, total pool = 24. Quorum threshold (35%) = ~8.4 tokens needed. A single player can force any 1 proposal through alone but that spends the entire budget. Splitting across 2 proposals risks both failing (4 tokens each = below threshold). Standard solution: coordinate implicitly on 2-3 "consensus" proposals — but Exclusive Proposals make this dangerous.

## Scoring

10 rounds × average 2.5 proposals passing per round × average 3.5 VP = ~87 VP typical winner score. Knizia minimum-score tension: last-place player is typically only 15–20 VP behind — the budget compression prevents runaway leaders.

## Player count scaling

- 2p: 6 tokens each; Quorum = 30% of 12 total = 3.6 tokens. Tense from turn 1.
- 5p: 12 tokens each; Quorum = 25% of 60 total = 15 tokens. Requires 2+ players aligning.
