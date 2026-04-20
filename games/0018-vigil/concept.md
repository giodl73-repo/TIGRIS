---
name: Vigil — Concept
slug: vigil-concept
game: 0018-vigil
stage: concept
version: 1.0.0
rubric_version: v2.15
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: stegmaier
anchor_axis: A6
sources:
  - ideas/hopper.md HOP-002 (promoted 2026-04-20)
---

# Vigil — Concept

## Premise

Something is trying to get in. A small group of watchers — call them paranormal investigators, a coven, a night shift — stands between the house and whatever rides the wind outside. Each round of the night, the watchers choose how to hold the line: **Investigate** (learn more about the thing), **Ward** (strengthen the boundary), **Flee** (break line-of-sight to deny it a target), or **Pray** (buy time at personal cost). Everyone reveals their chosen card at the same moment; the house reacts to the combined pattern. If the haunting track ever hits its last step, the house falls and all watchers lose.

One of the watchers is not a watcher. They look like everyone else; they hold the same four action types; but their deck has been *subtly replaced*. Their "Ward" card weakens the boundary in later rounds. Their "Pray" card draws attention instead of deflecting it. The traitor does not know which specific rounds their sabotage will bite — only that their deck is compromised and they must keep the watchers from noticing.

The whole game fits in a 10-minute teach. Each of the four actions has a single icon and a single line of text. The haunting track is a row of 5-7 spaces. There is no scoring — only the binary "did the house hold?" The weight lives in pattern-reading across rounds: when Ward fires and the track advances anyway, someone's Ward wasn't what it said it was.

The experience Vigil is aiming for: Stonemaier-house teachability (Wingspan-caliber on-ramp) with a hidden-role layer that would normally drag teach-time past the 30-minute mark. If it works, it's evidence that a fully traitor-coop game can sit in the teachable-at-10-minutes band.

## Players and length

- **Players:** 3–6 (4 recommended)
- **Length:** 60–90 min
- **Weight:** 2.5 (target)

## Anchor mechanic

**Simultaneous-reveal action selection with asymmetric hidden-role resolution.** Each round, every watcher has a hand of 4 base action cards (Investigate / Ward / Flee / Pray) plus a pool of variant modifiers drawn from a personal deck. All players select one card face-down. Reveal is simultaneous. The Monster script resolves based on totals: total Ward reduces haunting advance; total Investigate feeds a public clue pool; total Flee shifts the line-of-sight state; total Pray stalls the Monster's next-turn action. The traitor's deck looks identical on the backs but their variant modifiers quietly invert effects in specific haunting-track states — so a traitor's Ward might count as Ward in rounds 1–3 and silently count as 0 from round 4 onward. The traitor cannot freely choose to sabotage; their deck sabotages when played at the wrong moment.

## Artifact-as-story

**The action deck is the entire game.** Each watcher starts with 4 base action cards + 3 variant cards drawn from a shared pool. The traitor starts with 4 base action cards that look identical + 3 variant cards whose modifiers are conditional-sabotage (activate at haunting ≥ 4; or invert Ward into counter-Ward once per game; etc.). If the deck is not icon-clear, the 10-minute teach fails. If the sabotage variants are too obvious, traitor is identified in round 1 and the game collapses. If they're too subtle, cooperative strategy can't detect compromise and traitor always wins. The whole design sits on the calibration of this deck.

## Inspiration / lineage

- **Bruno Cathala — Shadows over Camelot (2005)** — traitor-cooperative pioneer; its Loyalty cards are Vigil's ancestor.
- **Jonathan Gilmore & Isaac Vega — Dead of Winter (2014)** — traitor-optional coop with crossroads reveals; Vigil inherits the "hidden per-player agenda" framing but makes traitor-present mandatory and deterministic at setup.
- **Antoine Bauza — 7 Wonders (2010)** — simultaneous action selection eliminates AP; Vigil borrows the "reveal together, resolve in fixed priority" structure.
- **Bruce Glassco / Various — Betrayal at House on the Hill (2004)** — horror-coop with a haunting-reveal; Vigil differs in that the traitor is known-from-start and the haunting is a continuous track, not a phase change.
- **Nate French & MJ Newman — Arkham Horror LCG (2016)** — horror-scenario structure with icon-coded card effects; Vigil borrows the icon-per-action legibility standard.
- **Werewolf / Mafia family** — hidden-role sabotage mechanics, particularly the "traitor's actions look like any other role's until pattern emerges."

## Target review in the TIGRIS pipeline

**Anchor: Stegmaier on A6 Teachability (adopted).** Expected earning: **canonical re-earn** if the 10-minute teach holds despite horror + traitor layers. Vigil aims to join Wingspan as a canonical reference game for A6.

**High-stakes second axis: A3 Interaction (adopted, contested-watch at 2E/1R after Wingspan self-refute).**
- If the hidden traitor creates real inter-player influence (reading each other's plays, adjusting based on suspected sabotage patterns), A3 **earns** — counter-pressure against the Wingspan multiplayer-solitaire refute.
- If the game collapses to "everyone pretends to Investigate, accuses in free time, no one influences play" — A3 takes a **second refute-game** → triggers `adopted-contested` formal status per v2.14 A-v2.14-01. Either way, Vigil moves the A3 ledger.

**Other expected earnings:**
- **B2 Catastrophe Pressure (Feld)** — canonical: haunting-track insta-loss IS catastrophe pressure.
- **A2 Decision Density (Feld)** — per-round 4-action × 3-variant decision is dense for teachable weight.
- **D1 Family-to-Expert (K-K)** — base 4 actions family-accessible; variant-deck enables expert pattern-reading.
- **C7 Action-Menu Clarity (K-K)** — 4 icon-coded actions with single-line effects = canonical clarity.
- **C1 Tension Budget (Knizia)** — haunting track IS the budget meter.
- **A4 Variance Calibration (Vaccarino)** — card-draw variance in variant-deck; sabotage trigger-states add controlled variance.

**Expected refutes / retire-explicits:**
- **C3 Scarcity Bite (Rosenberg)** — no resource scarcity.
- **C6 Point-Salad (Feld)** — binary win/lose; no scoring dimensions.
- **B3 Conversion Chain (Rosenberg)** — no conversion pipeline.
- **C4 Engine-Garden (Vaccarino)** — no personal engine; decks are fixed at setup.
- **B5 Architectural Novelty (Chvátil)** — traitor-coop is 20+ years old; Vigil's novelty is scoped to the conditional-sabotage-deck mechanic specifically.

## Non-goals

- **Campaign / legacy structure.** Vigil is one-shot. Scenario variety handled by Monster-deck + haunting-track length parameters, not persistent game-state.
- **Custom miniatures / asymmetric physical components.** Markdown-only design per TIGRIS spec §13 non-goals. All asymmetry lives in card-deck composition.
- **Deckbuilding layer.** The watcher and traitor decks are fixed at setup. No card acquisition during play.
- **Nemesis-style role diversity.** All watchers are mechanically identical at setup; the only role asymmetry is traitor-vs-watcher. Pursuing multi-role watcher variants is out-of-scope.

## Open questions (to resolve during DESIGN)

- **Haunting-track length**: 5, 6, or 7 steps? Calibration target: insta-loss pressure should bite starting round 4 at 4p, round 3 at 6p.
- **Sabotage variant count**: how many conditional-sabotage variants in the traitor deck? Target: traitor wins ~40-45% at expert play, ~55% at novice (where watchers can't read patterns).
- **Action-card effect precision**: each base action needs a one-sentence effect + one-sentence total-tally rule. Calibrate so the rules fit on a single tuck-box reference card.
- **Traitor knowledge asymmetry**: does the traitor see the full haunting-track state, or only their own action's effect? Lean toward: traitor sees everything (they need information-rich play to sabotage subtly).
- **Reveal-to-resolve priority**: all reveal simultaneously, then resolve Investigate → Ward → Flee → Pray in fixed order? Or in descending-total order? Fixed order is more teachable; descending is more tactically rich.
- **Round cadence**: 3p vs 6p should play at similar wall-clock time. Test: 6p round duration should not exceed 3p round duration by more than 40%.
- **4p teach target**: can 2 experienced players teach 2 newcomers in ≤ 10 minutes, including traitor rules, and have the newcomers play their first round correctly? If not, the anchor A6 claim collapses.
