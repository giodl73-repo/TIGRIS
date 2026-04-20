---
name: Rewild — Concept
slug: rewild-concept
game: 0021-rewild
stage: concept
version: 1.0.0
rubric_version: v2.18
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: rosenberg
anchor_axis: C3
sources:
  - ideas/hopper.md HOP-004 (promoted 2026-04-20)
---

# Rewild — Concept

## Premise

Rewild is a 2-5 player deck-builder with an inverted engine. Every player begins owning the same industrial landscape: 15 cards representing roads, pipelines, clear-cuts, drainage channels, monoculture fields, and other extraction infrastructure. The engine these cards form is productive — each card generates Gold per round — but ecologically dead. The game asks: can you dismantle this engine fast enough to restore a functioning ecosystem?

Each round, players draw 5 cards from their personal deck and play them. Industrial cards generate Gold. Gold is spent on two things: (1) **acquiring Wildlife** from a public Riverboard, (2) **removing** industrial cards from your deck. Wildlife cards add to your deck but score differently from industrial cards — they pay Biome points, not Gold. Each wildlife species has a biome affinity (Forest, Wetland, Grassland, Tundra, Chaparral) and a niche-category (predator, herbivore, pollinator, detritivore, aquatic). Scoring at game-end is: **(biome diversity × wildlife populations) − (remaining industrial footprint)**.

The tension is genuine. Every industrial card you remove loses its future Gold generation. Every wildlife card you add takes deck-slot but doesn't pay Gold. The optimal strategy is NOT clear: do you strip down fast and pay an early Gold cost for long-term ecosystem score? Or run industrial engines for 4-5 rounds first, stockpile Gold, then rewild late? The calibration of the 80-card Wildlife deck and the 15-card starting Industrial deck determines which strategy arc wins.

**The design bet**: combinatorial emergence via randomized Wildlife deck deal × biome-season events × personal industrial-starting-state × Riverboard timing produces **structural diversity of optimal strategy arcs** across plays. This is the A7 counter-pressure target (paired with Covenstat as 2nd different counter-pressure game for v2.18 retirement-reversal).

Secondary experience: Rosenberg's scarcity-bite applied outside farming. The Riverboard has 5 face-up Wildlife slots for up to 5 players; by mid-game, the best wildlife is gone and the scraps fight over niche plays. This is the C3 anchor case.

## Players and length

- **Players:** 2–5
- **Length:** 60–90 min
- **Weight:** 2.8 (target)

## Anchor mechanic

**Deck-un-building via inverted economy.** Each player's personal deck starts large (15 industrial cards) and should end small (~8-12 cards, mostly wildlife). Gold generation from industrial cards funds both acquisition and removal. Industrial-card-removal is a cost, not a benefit — it shrinks future income but enables higher ecosystem score. Emergence: the trade-off calibration varies game-to-game based on Wildlife deck shuffle, biome-season events, and opponents' draft timing. The 80-card Wildlife deck has 5 biomes × 16 species per biome; any given game sees only a fraction, ensuring different games emphasize different biomes.

## Artifact-as-story

**The Wildlife deck (80 cards, 5 biomes × 16 species).** Load-bearing because: if the deck is too small, emergent strategy space collapses (Halifax Hammer risk). If too large, teach-time ballons. 80 is target: sufficient for 5 biomes of 16 each; in any 3-play session, only ~50 cards surface. Each species has: biome affinity, niche category, Gold cost (1-6), Biome point value (1-4), Population capacity (1-4), and a unique trigger text.

The Industrial starting deck (identical across players) anchors fairness. Everyone starts the same; divergence comes entirely from play.

## Inspiration / lineage

- **Donald X. Vaccarino — Dominion (2008)** — deck-building mechanical ancestor.
- **Martin Wallace — A Few Acres of Snow (2011)** — deck-building + area-control fusion; Rewild borrows the "location-as-card" idea for Biome cards.
- **Elizabeth Hargrave — Wingspan (2019)** — nature-theme + tableau-engine reference.
- **Matt Leacock — Daybreak (2023)** — climate/environmental theme with card-removal mechanics.
- **Mark Gerrits — Terraforming Mars (2016)** — environment-restoration framing via card play.

## Target review in the TIGRIS pipeline

**Anchor: Rosenberg on C3 Scarcity Bite (adopted).** Riverboard scarcity + Gold tension. Expected: canonical earn.

**Secondary high-stakes: A7 Emergence-Replayability (adopted, contested-watch 5E/1R).**
- **v2.18 counter-pressure target.** Rewild is explicitly designed as 2nd counter-pressure game for A7 (paired with Covenstat). If panel confirms A7 earns canonically with collision-vote support, A7 becomes eligible for retirement-reversal trigger per A-v2.18-01.
- Key design features for A7: 80-card Wildlife deck diversity, 5-biome scoring matrix, per-game deck-shuffle determining hot biomes, personal-starting-state dissimilarity emerges from play divergence.

**Other expected earnings:**
- **C4 Engine-Garden Dependency (Vaccarino)** — inverted engine is still engine; Dominion-territory. Expected earn; potential collision with C3 on "which axis is primary."
- **B3 Conversion Chain (Rosenberg)** — Gold → Wildlife + Removal → Biome score; 3-stage chain. Earn.
- **A4 Variance Calibration (Vaccarino)** — Wildlife deck + Riverboard + event timing.
- **B6 Scoring Multiplier (Chvátil)** — biome diversity × wildlife populations IS canonical multiplier.
- **C6 Point-Salad (Feld)** — 5-biome scoring + industrial-footprint penalty + niche bonuses.

**Retire-candidates:**
- **B2 Catastrophe (Feld)** — no catastrophes.
- **D2 Spatial-Interaction (K-K)** — no map.
- **D1 Family-to-Expert (K-K)** — 80-card catalog + deck-un-building introspection is moderate-expert, not family.
- **C1 Tension Budget (Knizia)** — gentle escalation, not catastrophic-tension.
- **C8 First-Turn Compression (Stegmaier)** — deck-shuffle variance blocks compression.

## Non-goals

- **Realistic ecology modeling** — species are gameplay abstractions, not biological simulation.
- **Climate-disaster mechanics** — covered in Daybreak; Rewild stays focused on restoration, not fighting disasters.
- **Asymmetric starting decks** — every player starts identical 15 industrial. Fairness baked in.
- **Legacy / campaign** — one-shot. Replayability via deck shuffle + biome variability.
- **Custom minis** — markdown-only per TIGRIS conventions.

## Open questions (to resolve during DESIGN)

- **Wildlife deck size** — 60 / 80 / 100? Target 80 balances variety against teach-burden.
- **Starting industrial deck** — 15 cards, all generating 1 Gold, all identical? Or 15 varied (3 each of 5 industrial types)? Lean varied for strategic texture.
- **Removal cost** — flat (pay 3 Gold to remove 1 card), or scaling (first removal costs 2; second costs 3; third costs 4)? Scaling punishes late bulk-removal.
- **Biome-season events** — should each round trigger a biome-specific event (Forest year: Forest wildlife scores double), or are biomes equal? Lean event-driven for emergence.
- **Riverboard refill rate** — fixed (1 per round), or variable? Fixed for teachability; variable for texture.
- **Industrial card Gold generation** — does Gold reduce over time (ecological collapse simulation), or stay constant? Stay constant (simpler; trade-off remains).
- **End-trigger** — round 10 fixed? Or triggered when someone fully rewilds? Lean fixed-round to prevent rushing-rewild dominated strategy (Halifax Hammer risk).
- **Does A7 actually earn canonically?** The whole bet. If 3 sessions produce same arc, A7 refutes; Rewild fails as counter-pressure.
