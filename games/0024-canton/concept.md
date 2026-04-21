---
name: CANTON — Concept
slug: canton-concept
game: 0024-canton
stage: concept
version: 1.0.0
rubric_version: v2.20
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: kramer-kiesling
anchor_axis: D2
counter_pressure_primary: A7
counter_pressure_note: >
  A7 Emergence-Replayability is in contested-watch (9E/1R) after Pandemic Legacy #22
  refuted it. GARNER #23 completed counter-pressure cycle 2 game #1 via variable-contract-deck
  structural replayability. CANTON is counter-pressure cycle 2 game #2, using a
  DIFFERENT mechanism: asymmetric faction identity. The argument:
  "Playing as Faction A is a qualitatively different experience from playing as Faction B.
  The same 2 players with factions A+B face a different strategic landscape than A+C
  or B+C. With 6 factions and 2–4 players, no two faction combinations produce
  identical strategic dynamics. Replayability is intrinsic to player identity, not
  merely to external setup variables."
  This is mechanically distinct from GARNER's mechanism (setup variability → different
  demands on the player). Here the player BECOMES a different strategic entity each game.
---

# CANTON — Concept

## Premise

CANTON is a 2–4 player territorial Euro in which players control asymmetric factions competing for influence across a modular map of canton territories. Over three eras, players deploy markers, trigger faction abilities, and score territory networks — but every game, the roster of factions is different, and so is the map.

The game's structural replayability claim: with six distinct factions and combinations of two to four players, no two games share the same strategic landscape. A player who has mastered the Merchant faction's passive-income engine faces a completely different optimization problem when playing the Engineer — not just a different context, but a different mode of thought. CANTON's replayability is not circumstantial; it is built into the game's identity system.

Factions range from the straightforward (the Farmer: no active ability, strong passive territory scoring) to the complex (the Diplomat: conditional active ability requiring memory of opponents' control patterns). This gradient makes CANTON accessible to newcomers and absorbing for experts on the same board.

## Players and length

- **Players:** 2–4
- **Length:** 60–90 min
- **Weight:** 3.0

## Anchor mechanic

**Spatial Territorial Control.** The map is a modular hex arrangement (12 tiles drawn from a deck of 20, each with 2–4 territory spaces and connection edges). Players place Control markers on territory spaces, establishing adjacency chains. At each era end, players score controlled territories based on network size (contiguous control chains) and faction-specific bonuses. The spatial map is load-bearing: adjacency determines whether a faction's network is valuable, whether an opponent can block, and which territories are worth fighting over.

## Artifact-as-story

**The Faction Cards** (6 cards, 2 abilities each). Each faction defines:
- **Active ability**: an action available on your turn (costs 1–2 of your AP budget)
- **Passive ability**: always in effect; changes how you count territory or interact with others
- **Era bonus**: a unique scoring condition applied at each era end

The faction card is the game's physical identity token. Holding the Merchant card means you are playing a fundamentally different game than the player holding the Engineer card — not a small variation, but a different strategic persona.

Secondary artifact: **the Hex-tile map deck** (20 tiles; 12 drawn per game). Produces a different physical layout each play, further compounding the faction-combination variety.

## Inspiration / lineage

- **Kramer/Kiesling / Tikal (1999)** — action-point allocation + territory control + modular map excavation. CANTON borrows the AP economy and territorial scoring structure directly.
- **Kramer/Kiesling / Torres (2000)** — building-height spatial control; per-player asymmetric scoring. CANTON borrows the "players want different things from the same map" dynamic.
- **Laukat / Root (2018)** — asymmetric faction design as primary replayability mechanism. CANTON explicitly cites Root as the proof-of-concept that asymmetric factions can make the SAME board feel like different games for different players.
- **Kiesling / Azul (2017)** — clean rule-count for accessible weight-3 play. CANTON targets Azul's teach-time (under 15 min) while supporting the complexity gradient that Tikal enables.

CANTON's architectural claim is not the hex map or the action points — those are borrowed from Tikal/Torres. The claim is: **asymmetric faction identity as the primary cross-game replayability mechanism**, documented as A7 counter-pressure.

## Target review in the TIGRIS pipeline

**Earn-candidates:**
- **D2 Spatial-Interaction Presence (anchor; K-K)** — hex-tile map + territory adjacency chains + blocking and counter-blocking. D2's canonical territory: spatial control that matters. Earn.
- **A7 Emergence-Replayability (counter-pressure game #2)** — faction asymmetry × map variability. Different faction combinations produce qualitatively different strategic landscapes. This is the second of two required counter-pressure originals for v2.18 cycle 2. Needs collision-vote confirmation.
- **D1 Family-to-Expert Scaling (K-K secondary)** — faction gradient: Farmer (simple, no active ability) to Diplomat (complex conditional ability). Beginners choose Farmer; experts choose Diplomat. The same game serves both.
- **A3 Interaction (Chvátil)** — territory blocking, control-chain denial, faction-ability targeting. Direct spatial interaction is high.
- **B2 Catastrophe Pressure (Feld)** — map can be "blocked off" in late eras; losing territory to an aggressive faction is a catastrophe scenario requiring immediate response.
- **C7 Action-Menu Clarity (K-K)** — action-point menu with clear options: Place marker / Activate faction ability / Pass. Minimal lookup.
- **A1 Elegance (Knizia)** — rule-count/depth ratio. 3 era structure + AP economy + faction abilities is complex but manageable.
- **D4 Late-Game Lock-in (Rosenberg)** — territory placements are permanent; early-era control decisions lock in late-era scoring lines.

**Collision candidate:**
- **D2↔A7** (K-K vs Lacerda): K-K's spatial mechanism (D2) and Lacerda's A7 counter-pressure. Are they orthogonal (D2 = spatial mechanics within a play; A7 = faction identity across plays) or does D2 subsume A7 (the map variation already provides sufficient replayability without needing faction asymmetry)? Expect OP or CR — either outcome is load-bearing for the counter-pressure argument.

**Retire-explicit candidates:**
- C6 Point-Salad Incommensurability — territory scoring is multi-factor but not incommensurable in Feld's sense; all paths lead to the same VP metric.
- B5 Architectural Novelty — no novelty claim (Tikal lineage explicitly acknowledged).
- C3 Scarcity Bite — AP scarcity is present but not Rosenberg-canonical starvation.

## Non-goals

- No legacy elements. No permanent between-game modification. CANTON is explicitly anti-one-shot.
- No deck-building (that's Vaccarino's territory; CANTON stays in K-K's spatial tradition).
- No hidden information beyond the usual fog-of-war in tile draw (faction cards are always public once selected).
- CANTON does not claim architectural novelty — the Tikal lineage is acknowledged.

## Open questions (to resolve during DESIGN)

1. **Faction count**: is 6 factions (15 possible 2p combinations) the right pool? More factions = more replayability; fewer = more balance-testing feasibility. Current: 6.
2. **Map tile count**: 12 of 20 tiles per game. Is the map variation sufficient to compound the faction variety meaningfully, or does map variability get drowned out by faction dominance?
3. **Era length**: 3 eras. Each era ends when the action-point pool is depleted (track-based). Is this the right pacing mechanism?
4. **Faction selection**: random assignment vs. player-draft vs. player-choice? Random maximizes replayability variety; player-draft adds strategic depth in faction selection.
5. **AP budget per turn**: Tikal gives 5 APs; CANTON currently plans 3 APs per turn (faster turns) with faction-specific AP modifiers. Right balance?
