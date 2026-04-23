---
name: ESTUARY — Concept
slug: estuary-concept
game: 0152-estuary
stage: concept
version: 1.0.0
rubric_version: v2.24.83
bet_version: parliament
author: TIGRIS
created: 2026-04-23
updated: 2026-04-23
anchor_persona: lacerda
anchor_axis: B1
---

# ESTUARY — Concept

## Premise

A 17th-century port-trading game set in a branching river delta. Three to five merchant
houses compete to fulfill private cargo manifests — sealed lists of which goods must reach
which port cities, and in what quantities. The delta's ports transform raw goods into
finished cargo: timber becomes planks becomes hulls; grain becomes flour becomes hardtack.
Ships carry goods through channels; tides constrain when channels are navigable; harbors
charge tolls that become your opponents' income.

The game's central claim: **information asymmetry is an economy**. Each player knows
exactly what goods other houses are trading — the cargo crates are visible on the ship —
but not what their manifests *require*. A competitor loading six barrels of salted fish
might need exactly that to complete a high-value delivery, or might be filling a channel
just to block yours. You can learn their exact quantities, but only by paying: observation
costs mana, and information revealed stays revealed. Every decision to share or conceal
partial manifest knowledge is a gamble with compounding stakes.

Where Lacerda's previous designs (VIADUCT, the Gallerist, On Mars) placed hidden
information inside a subsystem, ESTUARY makes revelation itself the primary action
economy. The manifest is not a puzzle to be solved — it is a currency to be spent.

## Players and length

- **Players:** 3–5
- **Length:** 100–120 min (3p), 120–140 min (5p)
- **Weight:** 3.8 (BGG-estimate)

## Anchor mechanic

The cargo-manifest pipeline: Ships deliver raw goods to Processing Harbors, which
transform them into manifest-eligible cargo, which then travel to Delivery Ports for
scoring. Every step of the chain is conditional on the prior step having been completed
and the channel being navigable at this tide. The pipeline is: **Goods → Ship → Harbor →
Processed Cargo → Manifest Match → Delivery → VP**. Remove any stage and the chain
collapses. Each stage costs a different resource (mana type); each harbor has limited
capacity; each delivery window closes when the seasonal tide changes. B1 System Gearing
fires on the multiplicative dependency between the five pipeline stages.

## Artifact-as-story

**The Cargo Manifest.** Each player holds a private 4×4 grid: four goods types (Timber,
Grain, Salt, Cloth) × four delivery ports (North Quay, South Quay, River Gate, Open Sea).
Each cell holds a hidden quantity (0–6 units needed). The manifest is load-bearing:
it determines what the player *actually* values, which no one else can see. Its physical
form — a pad of sealed sheets that players mark, peek at, and guard — is the component
that generates the game's tension. Delivering 4 Timber to North Quay scores only if your
manifest cell for that combination is non-zero. Over-delivering generates no VP.

Secondary artifact: **the Tide Wheel**. A shared ratchet that rotates each round,
changing which channels are navigable (and which harbors are accessible from which sides).
The Tide Wheel is ESTUARY's pace-clock — no doom track, but a rotation that forecloses
delivery windows permanently if you miss them. It earns D4 Late-Game Lock-in (routes
commit when the Tide passes the window) and C1 Tension Budget (you can always see the
window closing but never quite make it in time).

## Inspiration / lineage

- **Lacerda — The Gallerist (2015):** Load-bearing information asymmetry (market-value
  hidden information); multi-stage production pipeline; mana-type scarcity. ESTUARY
  extends the information-cost mechanism from market-value to manifest-requirement.
- **Lacerda — VIADUCT (TIGRIS #20, 2026):** Established the B1↔B4 OP pair; aqueduct
  pipeline as system gearing. ESTUARY claims the same adjacency from the sea rather than
  the highlands — a second corpus instance of the same OP pair.
- **Rosenberg — Le Havre (2008):** Depth-4 conversion chain (iron→coke→steel→ship)
  as structural reference for B3. ESTUARY's harbor transformation chains are deliberately
  depth-3 (not depth-4) to avoid competing directly with Le Havre's B3 claim.
- **Lacerda — Kanban EV (2020):** Player-board sub-economy that gears into shared
  board events; harsh information asymmetry; long arc. ESTUARY inherits the arc length
  and information model, not the automotive theme.

Novelty claim: **No TIGRIS game (and no corpus game reviewed) puts manifest-requirement
asymmetry as the primary action economy.** VIADUCT had hidden capacity (what your pipes
can carry). ESTUARY has hidden demand (what you *must* deliver). The distinction is that
VIADUCT's asymmetry is about capability; ESTUARY's is about obligation. Obligation
asymmetry is unreviewed in the 150-game corpus.

## Target review in the TIGRIS pipeline

**Predicted earned:**
- **B1 System Gearing (anchor):** Pipeline stages multiply dependency (Goods → Ship →
  Harbor → Processed → Manifest → Delivery). Remove Harbor capacity and both Ship and
  Delivery collapse. Remove Tides and the Harbor access economy collapses.
- **B4 Information Transparency Cost:** Manifest-requirement hiding IS the game.
  Observation costs 2 mana (any type); partial reveal stays public. This is the
  highest possible B4 score: information cost is not incidental but load-bearing.
- **A2 Decision Density:** Per-turn choices: which goods to load, which harbor to queue,
  whether to observe an opponent's manifest, which delivery window to target. 10+ live
  options each action.
- **D4 Late-Game Lock-in:** Tide Wheel forecloses delivery windows; committed ship
  routes become irreversible when the tide turns. Lock-in visible and narratively
  justified at ~⅔ game length.
- **C1 Tension Budget:** Ship capacity and mana to activate harbors always undersupply
  demand. Knizia-territory: every turn ends with the best unmade move visible.
- **A28 Cognitive Load Profile (Lacerda-primary):** Five pipeline stages + manifest
  deduction + tide-window planning + partial-information inference = compound cognitive
  register. Expected A28=8.

**Predicted refuted:**
- **D3 Count-Robustness:** 3–5p range is narrow; the game is explicitly tuned for 3–4p
  (5p is possible but longer). Retire-explicit expected.
- **A6 Teachability:** Lacerda games are not fast teaches. Retire-explicit expected.
  Not a design failure — weight target is 3.8, not a gateway game.

**Predicted OP collision:**
- **B1↔B4 OP** — system gearing (pipeline architecture) vs. information transparency
  cost (manifest-requirement hiding). Established pair from VIADUCT #53 (26th TP at 5-3).
  Second instance would confirm pair reliability.

## Non-goals

- Not a negotiation game (deals are not formally structured; information revelation is
  unilateral and costly, not bilateral and mutual).
- Not a family-weight game. Complexity is intentional.
- Not a war game — conflict is economic (blocking delivery windows, saturating harbors),
  not martial (no combat mechanics).
- Not a solo game. Manifest deduction requires ≥3 players to generate meaningful
  information asymmetry.
- Not a replica of Le Havre. Conversion chain depth is B3-lite (depth-3 max), not
  Rosenberg's depth-4.

## Open questions (to resolve during DESIGN)

1. **Manifest format:** 4×4 grid (4 goods × 4 ports) or simpler 4×2 (4 goods × 2
   delivery types)? 4×4 gives higher A2/B4; 4×2 lowers weight. Design target: 4×3.
2. **Observation cost calibration:** 2 mana to observe one cell, or 3 mana for any
   quantity in a row (all ports for one good type)? Row observation is more strategically
   interesting (partial but actionable reveals).
3. **Harbor capacity:** Fixed slots (Brass-style) or variable (worker placement)? Fixed
   is more elegant (A1) but variable is more interactive (A3). Design target: fixed with
   1 overload slot per harbor (tension in the overload slot).
4. **Tide Wheel intervals:** Every round (fast clock, tight windows) or every 2 rounds
   (longer arc, more planning)? Test at 1 round first; calibrate at playtests.
5. **5p balance:** Does the manifest grid need to be adjusted at 5p to prevent manifest
   collision (two players needing the same good type to the same port)? Or is that
   collision itself a design feature (scarcity drives conflict)?
6. **B3 depth decision:** Should harbors support depth-3 chain (Goods → Harbor A →
   Harbor B → Delivery Port) or depth-2 (single transformation)? Depth-3 risks competing
   with Le Havre as B3 corpus reference. Design target: depth-2 transformation, earn B1
   on multiplicative dependency not B3 on chain depth.
