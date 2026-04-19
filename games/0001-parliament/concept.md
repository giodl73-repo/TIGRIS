---
name: Parliament — Concept
slug: parliament-concept
game: 0001-parliament
stage: concept
version: 1.0.0
rubric_version: v2.0
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
anchor_persona: knizia
anchor_axis: minimum-score-shape
---

# Parliament — Concept

## Premise

Parliament is a 3–4 player, 45–60 minute Euro about arguing over what makes a game good. Each player is a designer with strong opinions, a limited vocabulary (3 axes drafted from a shared pool of 24), and finite stake tokens to spend defending or attacking. Over four rounds of deliberation on hypothetical game premises, players plant stakes, challenge each other's stakes, collide on adjacent concerns, and at the end of the session propose changes to the vocabulary itself. The winner is the designer who most shaped the next session's rubric while scoring well this session — a Knizia-shape multiplier makes specialization lose.

Played once, Parliament teaches the TIGRIS pipeline. Played across multiple sessions, the game's Axis Pool evolves legacy-style: adopted axes stay; retired axes cost extra to re-pick. The game IS the factory. The factory's first review IS a playable board game. That recursion is the architectural bet made concrete.

## Players and length

- **Players:** 3–4. Designed for 4. 3-player drops the fourth seat (one persona sits out each game).
- **Length:** 45–60 min once rules are learned (budget 90 min for the first play).
- **Weight:** ~3.0/5. Teachable in 15 min (Kramer-Kiesling target).

## Anchor mechanic

**Axis draft + stake-and-challenge resolution + legacy amendment.** Three phases mirror the TIGRIS pipeline 1:1:

1. **Draft (Stakes):** snake-order drafting of 3 axes per player from the Axis Pool.
2. **Argument (4 rounds):** simultaneous stake placement + attack / defend / collide resolution against four Subject cards.
3. **Amendment (Endgame):** stakes classify as earned / refuted / contested / ignored; axis states update the shared Rubric Ledger for the next session.

The scoring multiplier — `earned_axes × adopted_axes` — is the Knizia-shape backbone. A player who scores high on stakes but adopts no axes wins nothing; a player who adopts axes without earning stakes is equally locked out. Balance wins.

## Artifact-as-story

The **Axis Pool deck** is the game's centerpiece. It's 24 oversized cards, each naming an axis (e.g., "Tension Budget", "Architectural Novelty", "Scarcity Bite"), its definition, and its anchor descriptors. The deck persists across sessions — players who play Parliament repeatedly see the Pool evolve. Adopted axes get a gold sticker; retired axes get a black sticker. After three sessions of play, no two play groups have the same Pool anymore. The deck *is* the factory's rubric; the game is played by handling it.

In the first play, the Pool starts in its TIGRIS-seed state (24 axes, all `live`). First-play adoption typically promotes 2–4 axes to `adopted`. The deck is meant to be annotated — a pencil-on-card medium for the adoption stickers.

## Inspiration / lineage

Parliament honors:
- **Reiner Knizia (Modern Art, Ra, Taj Mahal)** — auction-as-moral-instrument; the stake economy borrows directly from his bidding pressure. The scoring multiplier is the Ingenious / T&E minimum-score shape.
- **Stefan Feld (In the Year of the Dragon)** — simultaneous-reveal planning; the four-round structure with Subject-card variation is Year-of-the-Dragon-shaped.
- **Michael Schacht (Zooloretto, Coloretto)** — drafting with exclusivity and risk.
- **Rob Daviau (Pandemic Legacy)** — between-session evolution. Parliament is legacy-lite: adoption/retirement stickers, not torn cards.
- **Chvátil's Codenames** — the adjacency pattern (which stakes can collide) requires players to think laterally about axis-semantic proximity.

Not an imitation of any. The novel move is the **axis draft with exclusive scoring** — no existing game I know of has players draft scoring axes and then play to defend their drafted axes against each other's challenges. That's the architectural bet.

## Target review in the TIGRIS pipeline

Parliament will go through the v2.0 Parliament review procedure (STAKES → GATE → ARGUMENT → AMENDMENT) as anchor game #1. Success criteria (from `playtest-rubric.md`):

- ≥ 5 earned stakes across the 7 designer personas
- ≥ 2 earned stakes on Band B (Euro-specific) or Band C (persona-signature) axes
- ≥ 1 clean collision
- ≥ 1 axis promoted to `adopted`

Anchor persona **Knizia** is declared here. Anchor axis **C2 Minimum-Score Shape** is declared here — the claim is that Parliament's scoring multiplier IS a minimum-score shape, and if Knizia-AI cannot recognize it from the rules, the anchor fails and v2.0 needs revision before we touch T&E.

## Non-goals

- No theme beyond "designers arguing." No fictional world; no narrative beyond the Subject-card premises (which are themselves fictional board games).
- No custom art for v2.0. Component specs described textually; if Parliament earns a v2.1, art scope revisits.
- No solo mode. Parliament is zero-sum argument between ≥ 3 seats.
- No digital companion. The cards are the state.

## Open questions (to resolve during DESIGN)

- Exact `adopted` / `retired` thresholds at the 4-round scale (2 earned = adopted, or 3? 2 refuted = retired, or unique-round refutations?)
- Collision adjacency chart: draft an initial adjacency list, but the first play will reveal which pairs are live and which are cold.
- Tiebreaker order for equal final scores.
- Whether a retired axis can return to `live` if later successfully defended in a future session (probably yes, but defining the mechanic).

All four get settled in `design.md`.
