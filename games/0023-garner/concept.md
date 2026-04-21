---
name: GARNER — Concept
slug: garner-concept
game: 0023-garner
stage: concept
version: 1.0.0
rubric_version: v2.20
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: feld
anchor_axis: C6
counter_pressure_primary: A7
counter_pressure_note: >
  A7 Emergence-Replayability re-entered contested-watch at game #22 (Pandemic Legacy).
  Refutation argument: "legacy mechanism explicitly defeats replayability by design — zero
  replayability after the campaign ends." GARNER counter-pressures by making replayability
  a first-class structural design principle: variable contract deck (5 of 20 contracts per
  game) guarantees that no two plays share the same scoring landscape. Emergence within
  each game is high; replayability across games is structural and infinite.
---

# GARNER — Concept

## Premise

GARNER is a 2-4 player Euro tile-placement game about warehouse masters competing to fulfill merchant contracts across four trading seasons. Each game, a shuffled subset of five contracts — drawn from a deck of twenty — determines which commodity combinations are worth hoarding and which are liabilities. No two games share the same contract landscape.

Each player inherits a modest warehouse grid (4×4 tiles) and a starting hand of commodity tokens. Over four rounds (Autumn, Winter, Spring, Summer), players draft commodity tiles, arrange them into their warehouse grid, and spend adjacency bonuses to convert raw goods into processed stock. At season's end, the five active contracts resolve simultaneously: every player scores against the same public criteria, but the criteria change each game.

The game's central promise — borrowed from Feld's Castles of Burgundy and extended — is that the scoring landscape cannot be pre-memorized. The five contracts in play this game are different from last game's five. A player who has played GARNER ten times still faces a novel optimization problem on play eleven. This is not a side effect of complexity; it is the explicit design bet.

## Players and length

- **Players:** 2–4
- **Length:** 60–90 min
- **Weight:** 3.0

## Anchor mechanic

**Variable Contract Scoring.** Twenty contracts exist in the box; five are randomly selected each game and placed face-up at the start. Contracts specify scoring conditions of the form: "3 VP per adjacent pair of Grain+Barley," "7 VP for each completed column of processed stock," "2 VP per Grain minus 1 VP per unsold Chaff." At game start, no single contract dominates — every game, two or three contracts will be in tension (a player maximizing Contract 3 will sacrifice Contract 1). The optimal strategy is freshly derived each play.

Within a game, the contract deck drives emergence: players pursuing different contracts diverge quickly from a shared start state. A warehouse optimized for Contract 3 looks nothing like one optimized for Contract 1. By round 3, no two players' grids are alike.

## Artifact-as-story

**The Contract Deck.** Twenty contract cards, each with a unique scoring formula. Five selected randomly each game. This is the physical load-bearing element: without the contract deck, GARNER is a deterministic engine-building puzzle solvable by memorization. With it, the game is a fresh optimization problem each play. The deck is GARNER's structural anti-legacy element: unlike Pandemic Legacy, which permanently consumes its components, the contract deck is reshuffled and redrawn each game. Nothing is expended. Nothing is unlocked once and gone.

Secondary artifact: **4×4 personal warehouse grids** (player boards). Tile placement is the game's visible action surface; the contract deck is what gives placement meaning.

## Inspiration / lineage

- **Feld / Castles of Burgundy (2011)** — tile placement into personal board; draw from supply; adjacency bonuses. GARNER borrows the personal-grid + draw mechanism directly.
- **Feld / In the Year of the Dragon (2007)** — time-pressure × action-selection × variable event sequence. GARNER borrows the "each round, a different set of things are important" feel.
- **Gerdts / Concordia (2013)** — variable scoring deck (Concordia cards define what scores); GARNER extends this by making the deck the *per-game* setup variable, not a player-choice variable.
- **Rosenberg / Agricola (2007)** — warehouse/storage as resource-scarcity engine; GARNER borrows the "unsold stock becomes a liability" design.

GARNER's architectural claim is not the tile-placement or the adjacency bonuses — those are borrowed. The claim is: **variable contract deck as the primary replayability mechanism**, explicitly designed so that the scoring landscape is unrepeatable by structural guarantee.

## Target review in the TIGRIS pipeline

**Earn-candidates:**
- **C6 Point-Salad Incommensurability (anchor; Feld)** — five contracts with incommensurable scoring dimensions are the game's core. "Is Grain more valuable than Barley this game?" is unanswerable in the abstract; the answer depends entirely on which five contracts are active. Canonical C6.
- **A7 Emergence-Replayability (counter-pressure primary)** — by design: high in-game emergence (warehouse states diverge from round 1) + structural infinite replayability (contract deck reshuffled each game; no consumable elements). Direct counter-pressure to Pandemic Legacy's B5↔A7 refutation argument. If A7 earns here, it demonstrates that emergence + replayability coexist when the game explicitly designs for both. Needs collision-vote confirmation for v2.18 counter-pressure eligibility.
- **A2 Decision Density (Feld)** — per-turn: tile to draft, placement in grid, adjacency-bonus conversion, contract-tracking. 4-step per-turn tree.
- **B2 Catastrophe Pressure (Feld)** — unsold Chaff at season-end costs VP (liability mechanism). The accumulation pressure — warehouse fills up; Chaff is a terminal bad state — is catastrophe-adjacent.
- **D4 Late-Game Lock-in (Rosenberg)** — tile placements in the grid are permanent; the grid configuration locks in early rounds. Classic D4.
- **C3 Scarcity Bite (Rosenberg)** — warehouse capacity (16 tiles max) is the primary scarcity; commodity supply draft is the secondary. Rosenberg's canonical domain.

**Refute-candidates:**
- **B5 Architectural Novelty** — GARNER uses borrowed mechanisms (tile-placement, personal board, draft). No architectural novelty claim. Retire-explicit expected.
- **A3 Interaction** — limited direct interaction (draft competition; no direct player-to-player). Retire-explicit expected for standard Feld Euro.

**Collision candidate:**
- **C6↔A7** (anchor ↔ counter-pressure) — are incommensurable scoring paths (C6) and structural replayability (A7) the same thing or different dimensions? C6 is about *within-game* scoring incommensurability; A7 is about *across-game* divergence. If the panel identifies these as genuinely orthogonal registers of the contract deck's role, an OP-collision would confirm both. If the panel argues C6 subsumes A7 (variable contracts are just a C6 device, not an A7 device), then CR resolves in favor of C6. Either outcome is load-bearing for the counter-pressure argument.

## Non-goals

- No legacy elements. No permanent between-game modification. No sealed content. GARNER is explicitly anti-one-shot by design.
- No player elimination.
- No spatial interaction between player boards (players compete for tiles from a shared supply but their grids do not interact).
- Not a game about "the village" — the warehouse theme is functional, not narrative.
- GARNER does not claim architectural novelty. The tile-placement DNA is from Feld/Castles, the variable scoring DNA is from Gerdts/Concordia. What GARNER claims is that this specific combination, deployed as a counter-pressure argument, earns A7 structurally.

## Open questions (to resolve during DESIGN)

1. **Contract quantity calibration**: 20 contracts with 5 active per game gives C(20,5) = 15,504 possible game configurations. Is 20 the right pool size? Too many may dilute the contract landscape; too few reduces variation. Tension: more contracts = more replayability, but harder to balance each contract against the others.
2. **Chaff penalty calibration**: at what VP-per-tile does the Chaff penalty create genuine catastrophe pressure (B2) without dominating the game?
3. **Adjacency bonus depth**: should adjacency bonuses be simple (tiles of same type score together) or complex (specific combinations trigger conversion chains for B3 eligibility)?
4. **Player count scaling**: how does the shared commodity draft scale from 2p (slower drain, bigger hands) to 4p (faster draft, tighter scarcity)?
5. **A7 collision visibility**: should the design doc explicitly state that the contract deck is the A7 mechanism? This is the counter-pressure documentation requirement for v2.18 eligibility. Recommend: yes, state it explicitly in DESIGN §A7-counter-pressure.
