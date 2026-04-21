---
name: CRUCIBLE — Concept
slug: crucible-concept
game: 0052-crucible
stage: concept
version: 1.0.0
rubric_version: v2.23.23
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: feld
anchor_axis: B2
hopper_source: HOP-016
---

# CRUCIBLE — Concept

## Premise

CRUCIBLE is a 3–4 player alchemical production game set in a shared laboratory where players transmute base elements (Earth, Water, Fire, Air) into valuable compounds via a central Reaction Chamber mounted on a rondel. Each round, players commit their elements to the chamber and trigger a reaction — but every reaction carries risk. Mismatched elements cause explosions. Each explosion damages the reactor and permanently increases the cost of all future reactions. Push too hard and the laboratory collapses; play too safe and rivals claim the most valuable compounds.

Players pursue three incommensurable scoring paths:
- **Gold** (low-risk reactions; stable output; slow accumulation)
- **Elixirs** (high-risk reactions; volatile but valuable if the explosion is contained)
- **Artifacts** (sequential reactions; each Artifact requires the previous Artifact as input; the chain takes three successful reactions to complete but scores exponentially)

No single path dominates. A player committing to high-risk Elixirs accelerates the reactor damage for everyone; a player chasing Artifacts must survive three sequential reactions without an explosion. The catastrophe is collective but the scoring paths are individual.

## Players and length

- **Players:** 3–4
- **Length:** 60–90 min
- **Weight:** 3.0

## Anchor mechanic

**Escalating Catastrophe (B2).** The Reaction Chamber is the heart of the game — a shared resource that degrades with use. Each explosion adds a Damage token to the chamber. At 5 Damage tokens, the next reaction automatically explodes regardless of how well-matched the elements were. At 8 Damage tokens, the laboratory collapses and the game ends immediately (all scores locked). Players can spend actions to Repair the chamber (removing 1 Damage token) — but Repair costs elements that could have been used for scoring.

The catastrophe is **escalating and shared**: every player's aggressive reactions damage the chamber that everyone depends on. A player who exploits the shared chamber for quick Elixir scoring imposes costs on rivals who now face a more dangerous reactor. This creates the game's central social tension: each player's risk appetite affects the catastrophe curve for all players.

## Artifact-as-story

**The Reaction Chamber rondel** — a rotating dial with 8 reaction slots, each labeled with a required element combination. Players place their element tokens on slots; the rondel advances; reactions resolve. The physical rondel is load-bearing: its rotation creates timing windows (you can't always get the slot you want), its slots vary in risk (some combinations are safe; some require matching rare elements), and its Damage track is visible to all players as the session's catastrophe meter.

## Inspiration / lineage

- **Feld / In the Year of the Dragon (2007)** — escalating catastrophe events that players race to mitigate; tempo pressure from shared threat. CRUCIBLE borrows the multi-event catastrophe architecture.
- **Gerdts / Concordia (2013)** — rondel action selection; each step forward costs resources. CRUCIBLE borrows the rondel's step-cost structure (adapted to element placement rather than movement).
- **Rosenberg / Agricola (2007)** — resource scarcity under time pressure; starvation analogue. CRUCIBLE borrows the "everything costs something, and costs escalate" design ethos.

CRUCIBLE does not claim B5 architectural novelty (rondel + catastrophe is an established combination; Feld's own In the Year of the Dragon uses similar architecture).

## Target review in the TIGRIS pipeline

**Earn-candidates:**
- **B2 Catastrophe Pressure (anchor; Feld)** — reactor explosions are escalating catastrophe events. Each explosion raises the base cost of all future reactions. The 8-Damage laboratory collapse is the terminal catastrophe. B2 earns canonically via the Damage track's escalating pressure.
- **C6 Point-Salad Incommensurability (Feld secondary)** — Gold (low-risk), Elixirs (high-risk), Artifacts (sequential) are genuinely incommensurable scoring paths. A Gold strategy requires avoiding explosions; an Elixir strategy requires accepting them; an Artifact strategy requires consecutive success. The paths cannot be simultaneously maximized.
- **A2 Decision Density (Feld)** — per-reaction: which slot to target, which elements to commit, whether to Repair vs. produce. High density under catastrophe pressure.
- **B1 System Gearing (Lacerda)** — element gathering → rondel placement → reaction trigger → compound production → scoring. 4-gear pipeline.
- **C1 Tension Budget (Knizia)** — Damage track is the tension budget. Each explosion is a clock tick toward the 8-Damage collapse.
- **D4 Late-Game Lock-in (Rosenberg)** — compound placements in the scoring track lock in mid-game (Artifacts especially, since each requires the prior Artifact).

**Collision candidate:**
- **B2↔C6 OP** — catastrophe pressure (B2 = what happens when you fail; escalating reactor damage) vs. point-salad incommensurability (C6 = which scoring path to commit to; Gold vs. Elixir vs. Artifact). B2 asks "how bad is playing dangerously?"; C6 asks "which safe/dangerous path are you on?" Different analytical registers of the same rondel mechanism.

## Non-goals

- No legacy elements.
- No player elimination (the laboratory collapse ends the game for all).
- CRUCIBLE does not claim B5 Architectural Novelty.
- Not a cooperative game — players compete for scoring paths even though the reactor damage is shared.

## Open questions (to resolve during DESIGN)

1. **Rondel step cost**: how expensive should advancing to a non-adjacent reaction slot be? Too cheap = rondel loses its timing constraint; too expensive = players feel stuck.
2. **Repair vs. produce trade-off calibration**: at what Damage level should Repair become necessary? If Repair is too cheap, the Damage track never escalates; if too expensive, the laboratory collapses too quickly.
3. **Artifact chain length**: 3 sequential successful reactions for an Artifact. Is 3 the right depth? More depth = stronger B3 case but longer game; less depth = Artifacts feel cheap.
4. **Collective vs. individual explosion attribution**: when an explosion occurs, should all players take a penalty (encouraging collective caution) or only the player who triggered it (encouraging individual risk)? Current design: all players see Damage increase (collective); only the triggering player loses their placed elements (individual penalty).
5. **Rondel size**: 8 slots. Right size for 3–4 players across 60–90 min?
