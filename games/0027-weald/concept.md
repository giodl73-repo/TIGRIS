---
name: WEALD — Concept
slug: weald-concept
game: 0027-weald
stage: concept
version: 1.0.0
rubric_version: v2.22.2
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: kramer-kiesling
anchor_axis: D2
counter_pressure_primary: A3
counter_pressure_note: >
  A3 Interaction re-entered contested-watch at NEST #26 (8E/1R) via
  C6↔A3 CR 5-1 (parallel-board multiplayer-solitaire refutation).
  WEALD is counter-pressure cycle 3 game #1. Counter-pressure
  mechanism: DISPLACEMENT — any player may place a building on any
  hex, even if an opponent occupies it, by paying +1 resource beyond
  the base cost. The displaced player's building is returned to hand.
  This makes every hex a live contest: placing first is not safe;
  opponents can always overwrite you at a premium. No player can
  ignore opponents because ignoring them means losing your best hexes
  to displacement. Interaction is MANDATORY by mechanism, not by
  theme or discussion.
---

# WEALD — Concept

## Premise

WEALD is a 2–4 player hex-map development game about clearing and claiming land across a shared forest landscape. Players spend resources to clear hexes and build structures: Farms, Workshops, Mills, and Manors. Each cleared hex can hold exactly one structure, but no position is ever permanent — any player may displace an existing structure by paying one extra resource of the required type.

The displacement mechanism makes WEALD's interaction mandatory and structural: the best hexes (river-adjacent, hilltop, road-junction) will always be contested. Placing first doesn't secure a position; it stakes a claim that others can challenge. Players who ignore opponents' positions will lose their prime hexes to displacement. Every turn requires tracking not only your own development plan but also which of your positions opponents are likely to challenge.

At game end, players score VP for the structures they still hold on the board — displacement means you can build something and lose it before it scores.

## Players and length

- **Players:** 2–4
- **Length:** 60–90 min
- **Weight:** 2.5

## Anchor mechanic

**Displacement Development.** Each hex on the shared WEALD map has a terrain type (Forest, River, Hill, Plains, Road-junction) with an associated base resource cost to develop. On your turn, you spend the required resources to place a structure on any undeveloped hex OR to displace an opponent's existing structure on any hex by spending the base cost +1.

When your structure is displaced, you retrieve it to your hand immediately. The displacing player's structure is placed; they pay the premium; the displaced player is not otherwise penalized (no resource loss). This means: displacement is always an option; it costs you 1 extra resource; it returns the opponent's piece to their hand without hurting them beyond position.

The result: no position is safe without resource investment to defend it (by developing adjacent positions that are lower-priority for opponents, creating a "defensive perimeter" of less-valuable hexes that buffer your key positions).

## Artifact-as-story

**The shared WEALD map** — a hex grid of 40 terrain hexes (some pre-marked as impassable). All players develop the same landscape. The map is the contest: every hex has a value determined by adjacency to rivers, hills, and roads, and every good hex will be contested. The map's physical layout — which hexes are worth fighting over — is the game's argument surface.

## Inspiration / lineage

- **Kramer/Kiesling / Tikal (1999)** — action-point allocation + shared archaeological hex map + contested excavation sites. WEALD borrows the shared-map contest structure.
- **Knizia / Ra (1999)** — players bid for limited tiles; the highest-value tiles are contested; turn-order creates real displacement pressure. WEALD borrows the "shared pool is worth fighting over" economics.
- **Wallace / Brass: Birmingham (2018)** — building on shared network; other players' connections limit your options. WEALD borrows the "other players directly constrain your choices" architecture.

WEALD's architectural claim: **displacement as first-class interaction mechanism** — not blocking (you can't go here), not auction (we bid for it), but displacement (I can take it from you at a premium). This is a specific inter-player action type that makes A3 Interaction structurally mandatory.

## Target review in the TIGRIS pipeline

**Earn-candidates:**
- **D2 Spatial-Interaction Presence (anchor; K-K)** — shared hex map; direct placement competition; displacement creates spatial adjacency pressure. Canonical D2.
- **A3 Interaction (counter-pressure primary)** — displacement mechanism makes opposition actions directly affect your board state. Every good hex is actively contested. Mandatory interaction by mechanism.
- **B2 Catastrophe Pressure (Feld)** — displacement threat creates catastrophe risk: your best structure can be overwritten before scoring. Late-game displacement of a high-value Manor is a catastrophic event.
- **D4 Late-Game Lock-in (Rosenberg)** — developed hexes that survive to scoring are permanently locked; the final positions are the scoring record.
- **A2 Decision Density (Feld)** — per-turn: which hex to develop; whether to displace an opponent; which opponent position to target; whether to defend your own positions.
- **C1 Tension Budget (Knizia)** — displacement threat creates a tension clock on every structure: when will an opponent decide to displace yours?
- **D1 Family-to-Expert Scaling (K-K secondary)** — accessible entry (place a building, spend resources) with expert depth (optimal displacement timing, defensive perimeter building).

**Collision candidate:**
- **D2↔A3** — the anchor itself and the counter-pressure axis. Are spatial interaction (D2: map mechanics) and direct player interaction (A3: player-to-player actions) orthogonal (OP candidate) or is D2 the mechanism and A3 the consequence? If OP: D2 describes the spatial landscape; A3 describes the social dynamic of displacing opponents. If CR: spatial interaction IS the interaction; D2 subsumes A3.

I expect CR (D2 wins): the displacement mechanism is spatial, and the "interaction" (A3) is a consequence of the spatial competition. HOWEVER — for v2.18 counter-pressure purposes, A3 must EARN with collision-vote confirmation. If D2↔A3 is CR with D2 winning, A3 doesn't earn the collision credit. Solution: design the argument so A3 earns first (via independent defends on the displacement mechanism's mandatory interaction) and the collision (if it occurs) is OP (D2 and A3 are orthogonal registers: spatial landscape vs. social contest).

The designed argument: A3 earns independently through displacement-as-mandatory-interaction evidence, then the D2↔A3 OP fires to confirm the earn with collision-vote.

## Non-goals

- No legacy elements.
- No hidden information — all hexes visible; all opponent resources visible.
- No player elimination — displacement doesn't hurt the displaced player beyond losing a position.
- WEALD does not claim B5 Architectural Novelty (displacement is borrowed from the Diplomacy/area-control tradition).

## Open questions (to resolve during DESIGN)

1. **Displacement cost calibration**: base cost +1. Is this right? Too cheap → constant displacement, never stable. Too expensive → displacement never happens, game plays like parallel development. Target: displacement should happen 3-5 times per game per player.
2. **Map density**: 40 hexes for 2-4 players with 4 structure types × up to 6 per player = 24 structures max. Is 40 hexes dense enough for real competition?
3. **Resource types**: current plan is 4 resource types (Wood, Stone, Food, Gold) with terrain-specific costs. How many resource types before the economic layer dominates the spatial layer?
4. **Scoring**: structures score differently by type (Manor = 5 VP; Farm = 2 VP; Workshop = 3 VP; Mill = 4 VP) and by adjacency (Hill = +1 VP; River = +1 VP). Should adjacency bonuses create more spatial pressure?
