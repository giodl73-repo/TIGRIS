---
name: FURROW — Concept
slug: furrow-concept
game: 0028-furrow
stage: concept
version: 1.0.0
rubric_version: v2.22.3
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: chvatil
anchor_axis: B5
hopper_source: HOP-001
counter_pressure_primary: A3
counter_pressure_note: >
  A3 Interaction is in contested-watch (9E/1R). FURROW is counter-pressure
  cycle 3 game #2. Counter-pressure mechanism: trick-taking lead/follow.
  In every trick, all players MUST respond to the leader's card. Following
  suit is mandatory if you hold matching cards. Reneging is a penalty.
  Every trick is a collective mandatory interaction: the lead card defines
  the terrain of the trick; every other player's choice is constrained by
  the leader's choice and by their own hand composition. This is mandatory
  interaction at the collective (all-players-simultaneously) register
  rather than WEALD's targeted (I-act-on-you) register. Both demonstrate
  A3 earning: WEALD shows displacement = direct player-to-player action;
  FURROW shows lead/follow = collective mandatory engagement. Together
  they satisfy the two-mechanism requirement for v2.18 cycle 3 completion.
---

# FURROW — Concept

## Premise

FURROW is a 2–4 player trick-taking game about a farming community dividing the harvest across four seasons. Players hold cards representing crop varieties (Grain, Barley, Hops, Flax — mirroring GARNER's commodity vocabulary but in card form) and compete to win tricks that earn harvest tokens. At season's end, players convert their harvest tokens into VP by planting them in their personal field strips, where adjacency bonuses create a secondary scoring layer.

The game's structural replayability claim (A7 secondary): no two deals are alike, and the optimal trick-play strategy depends on your hand, the other players' visible play history, and which harvest tokens have already been taken. The game is architecturally novel (B5 anchor) because the trick-taking mechanism IS the farming action-selection: winning a trick selects which crop you harvest, not just how many points you earn.

## Players and length

- **Players:** 3–4 (best at 4)
- **Length:** 45–60 min
- **Weight:** 2.0

## Anchor mechanic

**Trick-as-Action-Selection.** Standard trick-taking in most games determines who scores points. In FURROW, the suit of the winning card determines WHICH crop you harvest: Grain tricks yield Grain tokens; Barley tricks yield Barley tokens; Hops tricks yield extra tokens (wild); Flax tricks yield VP directly. The lead suit doesn't just set the rules for following — it sets the economic stakes of the trick. Winning a Flax trick with a high Flax card scores directly; winning a Grain trick grows your planting options.

This creates a novel tension: do you follow suit to win a valuable Grain trick (even with a card you'd rather save), or do you discard a low-value off-suit card and let an opponent win? Every card play is both a trick decision and an economic decision.

## Artifact-as-story

**The 48-card hand deck** — 12 cards per suit (Grain: 1–12; Barley: 1–12; Hops: 1–12; Flax: 1–12). Standard trick-taking deck, but each suit is color-coded to a harvest type. The deck IS the farm: crops in your hand are crops waiting to be harvested.

Secondary: **the personal field strip** (4 spaces per player; tiles placed left-to-right representing seasonal plantings). At season-end, your field's tile adjacency determines adjacency bonuses.

## Inspiration / lineage

- **Chvátil (standard)** — B5 Architectural Novelty: FURROW's trick-as-action-selection structure is novel in the trick-taking genre. Prior trick-taking games determine who scores; FURROW determines what you score. This is a structural distinction.
- **Rosenberg / Bohnanza (1997)** — planting mechanic + crop variety management. FURROW borrows the "crops have different values at different quantities" economy.
- **Knizia / Ra (1999)** — trick-taking-adjacent mechanics (tiles you collect determine your scoring path). FURROW borrows the "your collection defines your scoring" design.

## Target review in the TIGRIS pipeline

**Earn-candidates:**
- **B5 Architectural Novelty (anchor; Chvátil)** — trick-as-action-selection: winning a trick chooses what you harvest, not just who scores. Novel in the trick-taking genre.
- **A3 Interaction (counter-pressure game #2)** — trick-taking lead/follow = mandatory collective interaction. Every player must respond to the leader's card on every trick. You cannot ignore the trick; you must engage. Different register from WEALD's displacement (targeted vs. collective-mandatory).
- **A7 Emergence-Replayability** — no two deals identical; play history reveals opponents' hand compositions; strategic path varies per game.
- **A4 Variance Calibration** — deck shuffle randomizes hands; trick outcomes calibrated by hand composition.
- **C1 Tension Budget** — end-of-season scoring creates late-trick tension: every trick in season 4 may determine who wins.
- **A1 Elegance** — trick-taking is one of the most elegant mechanisms in the history of card games; FURROW adds one structural layer (what-you-harvest) without breaking the base mechanism.

**Collision candidate:**
- **B5↔A7** — Chvátil's architectural-novelty claim (B5) vs. across-play replayability (A7). Same temporal-register OP pattern as Codenames #25 (B5↔A7 8-0). FURROW's trick-taking structure is the same every game (B5 = timeless mechanism); the deal variance is what changes (A7 = per-game content). Expecting Decisive OP if the panel recognizes the pattern.

**Retire-explicit candidates:** B2 (no escalating catastrophe in trick-taking), B3 (no conversion chain), C6 (no point-salad incommensurability), D2 (no spatial interaction), D4 (no late-game lock-in — trick outcomes are immediate), C4 (no engine-building).

## Non-goals

- No legacy elements.
- Not a pure trick-taking game — FURROW adds a planting layer. But the trick-taking IS the core mechanism.
- FURROW does not attempt to close the A3 contested-watch by itself — combined with WEALD (game #27), it completes the v2.18 cycle 3 two-game requirement.

## Open questions (to resolve during DESIGN)

1. **Trump suit**: should FURROW have a trump suit? Hops (the wild/bonus suit) could function as trump. This increases B5's novelty claim (trump = Hops = bonus = familiar concept deployed unusually).
2. **Field strip scoring**: how many adjacency bonuses? Current: +1 VP for same-crop adjacent tiles, +2 VP for a full strip of one crop type. Need calibration vs. trick earnings.
3. **Season length**: 4 seasons × 12 tricks = 48 tricks total for 4 players (full deck dealt each season?). Or shorter seasons (8 tricks) with 3 seasons?
4. **2-player viability**: standard trick-taking degrades at 2 players. Is 2-player in scope?
