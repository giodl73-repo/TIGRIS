---
name: LEDGER — Concept
slug: ledger-concept
game: 0055-ledger
stage: concept
version: 1.0.0
rubric_version: v2.23.26
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: knizia
anchor_axis: C1
hopper_source: HOP-018
---

# LEDGER — Concept

## Premise

LEDGER is a 2–5 player Vickrey sealed-bid auction game for commodity tiles across 4 market rounds. All players simultaneously commit sealed bids; highest bidder wins but pays only the second-highest price. The bid IS the tension budget: you must exceed the threshold without knowing where it is. A minimally elegant design — 3 action types, 4 rounds, pure auction tension.

## Anchor mechanic

**C1 Tension Budget.** The Vickrey second-price mechanism is the sharpest tension budget in auction design: every bid is a clock tick where you don't know the threshold. Bidding too high wastes money; bidding too low loses the lot. The perpetual tension is structural: you can never know if you've bid enough without risking overbidding.

## Target earn-candidates

- **C1** (anchor; Knizia) — Vickrey tension is structural perpetual scarcity
- **A2** — per-bid calibration density; each round = calculate lot value vs. likely threshold
- **A1** — minimal 3-action ruleset; elegant Knizia shape
- **B4** — sealed bids = hidden information economy (you don't see rivals' bids until reveal)
- **A6** — taught in 10 min; sealed-bid metaphor grounds rules
- **D3** — 2–5 scales via bid token count
- **D1** — family-accessible premise; expert bidding strategy
- **A3** — bidding war = direct competition
- **A4** — tile distribution variance calibrated across 4 rounds
- **C7** — 3 clear actions (Bid, Pass, Score)

## Collision candidate

**C1↔B4 OP** — tension of bid reveal (C1: the moment when sealed bids are revealed and you find out if you won) vs. information cost of hidden bids (B4: the entire sealed-bid economy before reveal). C1 = the tension of the outcome; B4 = the architecture of the hiding. Different temporal registers: B4 = during bidding; C1 = at reveal.
