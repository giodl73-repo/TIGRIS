---
name: HAUL — Concept
slug: haul-concept
game: 0051-haul
stage: concept
version: 1.0.0
rubric_version: v2.23.22
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: rosenberg
anchor_axis: B3
hopper_source: HOP-017
counter_pressure_note: >
  B3 Conversion Chain Depth is at 3E/4.5R with 18 games since last earn
  (WELLSPRING #32). HAUL is a targeted B3 earn: the Farm→Mill→Warehouse→
  Market chain is documented as the explicit design target. Additionally,
  C4 Engine-Garden Dependency (last earned Brass #37, 13 games ago) must
  earn here via the warehouse-upgrade garden mechanic before its 8-game
  monitoring window closes at approximately game #58.
---

# HAUL — Concept

## Premise

HAUL is a 2–4 player harvest-and-transport Euro about building seasonal supply chains across a shared countryside. Players are hauliers: they plant crops, process them at shared mills, store surplus in personal warehouses, and sell goods to markets that close at season's end.

Goods move through four conversion stages — **Farm → Mill → Warehouse → Market** — each step transforming raw produce into higher-value processed goods. Grain becomes Flour at the Mill; Flour becomes Bread in the Warehouse (when stored adjacent to the Spice tile); Bread commands premium prices at the Market. The four-step chain is the game's spine: players who cut corners and sell raw grain earn less; players who run the full chain earn more but risk the goods spoiling before season's end.

Resource scarcity bites at every node. The Mill is shared — only one haulier can process at the Mill per season-round, creating a race for the conversion bottleneck. Warehouse capacity is limited: you can store four goods, not five, which means every purchase requires a discard. The Market closes after all four seasonal markets have been resolved; goods unsold are worth nothing.

The warehouse is the game's engine-garden: players upgrade their warehouse with Preservation tiles (goods don't spoil), Spice adjacencies (raw goods convert to premium goods in storage), and Cooling rooms (extend sell windows). A well-upgraded warehouse becomes a self-sustaining conversion engine. A bare warehouse is a liability.

## Players and length

- **Players:** 2–4
- **Length:** 75–100 min
- **Weight:** 3.0

## Anchor mechanic

**Conversion Chain Depth (B3).** The Farm→Mill→Warehouse→Market pipeline is a genuine depth-4 resource-conversion chain. Each stage has a distinct cost, a distinct output type, and a distinct scarcity pressure. Converting all the way from raw Grain to premium Bread requires committing to a full 4-stage plan over multiple rounds — and opponents competing for the Mill and Market windows make that plan fragile.

The tension hypothesis: Rosenberg owns B3 (the conversion pipeline is Rosenberg's natural territory — Agricola, Le Havre, A Feast for Odin all demonstrate it). The collision is B3↔C3 OP: **B3 = the architectural design of the conversion chain**; **C3 = the scarcity pressure that makes each node in the chain costly**. Different analytical registers of the same supply chain.

## Artifact-as-story

**The shared Countryside board** — a central grid of terrain hexes (Farmland, Mill sites, Market stalls, River crossings, Warehouse locations). Players place their goods tokens, carts, and warehouse tiles on this shared surface. The board's physical layout — which Mill is adjacent to which Farm, which Market closes first — determines the urgency of the conversion race.

Secondary: **Personal Warehouse boards** (4-slot grid; upgradeable with Preservation, Spice, Cooling tiles). The warehouse becomes the game's engine-garden: players invest in its architecture across 4 seasons, creating a personal conversion engine that interacts with the shared countryside.

## Inspiration / lineage

- **Rosenberg / Le Havre (2008)** — multi-step resource processing; shared production buildings; starvation mechanic as pressure. HAUL borrows the conversion-chain spine and the building-competition dynamic.
- **Rosenberg / A Feast for Odin (2016)** — warehouse/storage as a spatial puzzle; goods placed adjacently trigger bonus effects. HAUL borrows the warehouse-as-garden mechanic (adjacency between stored goods creates conversion bonuses).
- **Wallace / Brass: Birmingham (2018)** — route-building on shared geography; era-based lock-in. HAUL borrows the shared-board competition and the permanent commitment of placed infrastructure.

HAUL's architectural claim: not novelty (Le Havre precedent acknowledged) but **depth-4 canonical conversion chain** as the game's primary design statement.

## Target review in the TIGRIS pipeline

**Earn-candidates (mandatory):**
- **B3 Conversion Chain Depth (anchor; Rosenberg)** — Farm→Mill→Warehouse→Market. Depth-4. This is the game's explicit design target. *18 games since last earn — B3 must earn here.*
- **C4 Engine-Garden Dependency (targeted)** — Warehouse upgrades (Preservation + Spice + Cooling tiles) create an architectural dependency: the conversion bonuses depend on having the right tiles placed adjacently. C4 earns via the warehouse-garden mechanic. *C4 monitoring window: game #58 is the last safe game.* Must document in concept.md and §Designer Notes.
- **C3 Scarcity Bite (Rosenberg primary)** — Mill bottleneck (shared, 1-player-per-round), Warehouse capacity (4 slots), Market close-out (seasonal). Three scarcity nodes on the same supply chain.
- **D4 Late-Game Lock-in** — Warehouse tile placements are permanent. Season 3 conversions are shaped by Season 1 warehouse architecture.

**Earn-candidates (secondary):**
- **B1 System Gearing** — the Farm→Mill→Warehouse→Market gear cascade is the game's production system.
- **A2 Decision Density** — per-action: which good to convert, which node to use, when to sell vs. store.
- **C1 Tension Budget** — Market close-out creates a seasonal tension clock.
- **B2 Catastrophe Pressure** — goods spoil if not sold before Market closes; a warehouse full of unsellable goods is a catastrophe.

**Collision candidate:**
- **B3↔C3 OP** — conversion chain depth (B3: structural pipeline design) vs. scarcity bite (C3: pressure at each pipeline node). B3 asks "how deep is the chain?" C3 asks "how scarce is each step?" Same supply chain; different analytical registers.

## Non-goals

- No hidden information (all goods and tile placements visible).
- No legacy elements.
- HAUL does not claim B5 Architectural Novelty (Le Havre / A Feast for Odin precedent explicitly acknowledged).

## Open questions (to resolve during DESIGN)

1. **Mill bottleneck model**: single shared Mill (only 1 player processes per round) vs. multiple Mill sites (competition via placement). Single Mill creates more acute bottleneck; multiple Mills soften but allow blocking.
2. **Warehouse upgrade cost**: how expensive should Preservation/Spice/Cooling tiles be? Too cheap = everyone builds the same warehouse; too expensive = C4 garden never materialises.
3. **Season length**: 4 seasons × 4 rounds = 16 total rounds. Right pacing for a weight-3 game?
4. **Spoilage rule calibration**: goods that aren't sold by Market close become waste tokens (−1 VP each). Is −1 VP the right penalty to create B2 catastrophe pressure without dominating the game?
5. **B3 depth confirmation**: does the Mill-to-Warehouse conversion count as a separate stage from Warehouse-to-Market? Currently: Farm (grow) → Mill (process) → Warehouse (store+upgrade) → Market (sell) = 4 stages. Confirm each stage has a distinct mechanical identity before TIER-A.
