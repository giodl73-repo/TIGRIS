---
name: Covenstat — Concept
slug: covenstat-concept
game: 0019-covenstat
stage: concept
version: 1.0.0
rubric_version: v2.16
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: lacerda
anchor_axis: A7
sources:
  - ideas/hopper.md HOP-003 (promoted 2026-04-20)
---

# Covenstat — Concept

## Premise

Covenstat is a 2–5 player horror roll-and-write in which every player is an investigator of the same occult phenomenon — but each sees a different piece of it. At the center of the table, 8 "fate dice" are rolled each round. Every player looks at the same dice. But each player's **private objective sheet** — a 3-column personal dossier of what-you're-tracking — turns those dice into something different.

Where one investigator reads the dice as a pattern of lunar alignments, another reads them as blood types at a crime scene, and a third reads them as fragments of a summoning phrase. Each player marks their own sheet. Shared dice. Private meaning.

There are 12 distinct objective-sheet types (Coven, Detective, Occultist, Medium, Alienist, Priest, Skeptic, Heir, Witness, Cartographer, Archivist, Relative). Each sheet has a unique scoring rubric, a unique end-trigger, and a unique "collapse" state (sanity breaks differently for each role). Players draft sheets at setup.

The design bet: **combinatorial emergence via private-rubric-on-shared-dice can defeat the Halifax Hammer failure mode.** No single "optimal" strategy can dominate because the optimal play depends entirely on which sheet you hold. And even for a specific sheet, the dice permutations across 8-12 rounds mean no two plays converge on the same arc. Halifax Hammer fails because there is no universal deck; each player is playing a structurally different game with the same inputs.

Secondary bet: the **investigation feels cooperative even when scoring is individual.** Shared dice force shared attention. Players watch each other's sheets — not to know scores but to know *what each other is tracking*. A Priest marking blessings ≠ a Skeptic marking inconsistencies, and that asymmetry is legible because sheets are public.

No Monster, no haunting, no traitor. The horror is interpretive: 5 people looking at the same 8 dice and seeing 5 different nightmares.

## Players and length

- **Players:** 2–5
- **Length:** 45–75 min
- **Weight:** 2.8 (target)

## Anchor mechanic

**Shared-dice / private-objective roll-and-write.** Each round: 8 fate dice rolled centrally. Players in turn order claim dice from the pool (drafting: first player takes 2, next takes 2, then 1-1-1-1 depending on count) and mark their private sheets. Each sheet has 3 columns representing 3 tracking dimensions; each die claimed goes in one column per the sheet's printed rules. End-triggers are sheet-specific (Coven fills their summoning phrase; Detective closes the case; Skeptic proves all alternative explanations). Game ends when any player hits their end-trigger OR round 12 completes.

Scoring: each sheet has a unique 0–60 rubric combining column totals, pattern bonuses, and "collapse" penalties. No cross-player scoring normalization — scores are not directly comparable, but the end-trigger gate creates implicit race pressure.

## Artifact-as-story

**The 12 objective sheets.** Each is a folded A5 tri-fold: front panel has flavor + end-trigger; inside panel has the 3 tracking columns with mark-boxes; back panel has scoring rubric. Sheets are physical, public-facing (everyone can see everyone's sheet at all times), but only the owner marks their own.

Load-bearing: if the 12 sheets don't produce genuinely different optimal-strategy trees, A7 Emergence fails. If they do, the game has 12C5 × permutation × dice-order ≈ 10^8 meaningfully-distinct opening-states. The sheet catalog IS the anti-Halifax-Hammer move.

## Inspiration / lineage

- **Wolfgang Warsch — That's Pretty Clever (Ganz Schön Clever) (2018)** — roll-and-write with shared public dice and individual sheets; core mechanical ancestor.
- **Various (Welcome To…, Railroad Ink, Railroad Ink Deep Blue)** — shared-decision roll-and-write lineage.
- **Arkham Horror LCG (French/Newman 2016)** — horror-investigator framing; per-scenario divergent play.
- **The Mind (Warsch 2018)** — "no direct communication, still cooperative" spirit.
- **Nemesis (Kwapiński 2018)** — private-agenda layer on shared threat; Covenstat inherits the private-agenda idea without the traitor axis.
- **Tarot / divination games** — interpretation-of-shared-symbols as gameplay.

## Target review in the TIGRIS pipeline

**Anchor: Lacerda on A7 Emergence-Replayability (adopted, contested-watch at 2E/1R).**
- **Counter-pressure target.** Vigil just demonstrated that contested-watch axes CAN be defended by ledger-aware design. Covenstat mirrors the pattern for A7.
- Expected earning: 12-sheet catalog × dice-permutation × drafting-order produces vast strategy space. If A7 earns 3rd canonical time, contested-watch qualitatively reduces (same pattern as Vigil→A3).

**Other expected earnings:**
- **A4 Variance Calibration (Vaccarino)** — shared dice + private interpretation is canonical variance-as-resource. Likely earn.
- **B4 Information-Transparency-Cost (Lacerda)** — public sheets but hidden strategy computation; opponents see what you track but not what you *value*. Moderate earn.
- **A3 Interaction (Chvátil)** — **high-stakes secondary.** Public sheets force attention to opponents, but individual scoring may collapse to multiplayer-solitaire. Test: does Covenstat preserve A3's counter-pressure gains from Vigil, or regress?
- **A5 Downtime-Pacing (Vaccarino)** — drafting in turn order can cause waiting. Moderate earn/hold.
- **C6 Point-Salad (Feld)** — 12 scoring rubrics across players; individual rubrics have 3+ dimensions. Earn.

**Expected refutes / retire-explicits:**
- **C3 Scarcity Bite (Rosenberg)** — 8 dice for 2-5 players is not canonical scarcity (enough to go around).
- **B2 Catastrophe Pressure (Feld)** — no catastrophes; only personal "collapse" penalties.
- **B3 Conversion Chain (Rosenberg)** — no conversion pipeline; dice mark sheets directly.
- **C4 Engine-Garden (Vaccarino)** — no personal engine; sheets are fixed at setup.
- **D2 Spatial-Interaction (K-K)** — no map.
- **D4 Late-Game Lock-in (Rosenberg)** — marks are permanent but arc is short.

## Non-goals

- **Monster mechanic.** Covenstat has no NPC antagonist. The horror is interpretive.
- **Traitor / hidden role.** Already covered by Vigil. Covenstat is transparent-competitive-roll-and-write.
- **Legacy / campaign.** One-shot play. Scenario variety via sheet-draft + dice-shuffle.
- **Custom miniatures.** Markdown-only.
- **Deckbuilding.** No cards at all; the sheets are the persistent state.

## Open questions (to resolve during DESIGN)

- **Sheet count (12 vs 10 vs 15)?** Each added sheet adds emergence but also teach-cost. Target: 12 — enough for replayability (12C5 = 792 role-draw combinations), not so many that teach fails.
- **Dice pool size (6 vs 8 vs 10)?** 8 is the target (4 standard + 4 fate-colored), but calibration depends on player count. 2p may use 6 dice; 5p may use 10.
- **Drafting order (turn order vs stat-based vs all-simultaneous)?** Simultaneous would be fastest but may cause dice-conflict; turn-order adds downtime. Start with strict turn-order and test.
- **End-trigger race pressure**: if any player's trigger fires → game ends. Does this mean the "easiest" sheet always wins? Or does each sheet have comparable-difficulty triggers calibrated to ~round 9? Need playtest data.
- **Collapse penalties**: each sheet has a "collapse" state when a tracking column overflows. Should collapse end the game for that player only (eliminated), or impose a permanent −15 VP (damaged)? Lean toward −15 (no elimination).
- **Shared-dice draft order within round**: does first-player rotate? If static, first-player gets best pick advantage across 12 rounds. Rotate per round.
- **Does anyone know anyone else's scoring rubric?** Sheets are public, so the rubric is visible — but computing opponent's optimal play requires actively reading their sheet. Test at 3p: do players waste time computing opponent scores or just play their own?
