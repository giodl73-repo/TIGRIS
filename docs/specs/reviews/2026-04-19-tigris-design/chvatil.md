---
name: Vlaada Chvátil review of TIGRIS Design Blueprint
slug: chvatil
stage: panel
version: 1.0.0
rubric_version: v1.0
author: chvatil
artifact_under_review: tigris\docs\specs\2026-04-19-tigris-design.md
created: 2026-04-19
updated: 2026-04-19
---

# Chvátil on TIGRIS

## Opening verdict

If the spec feels like anything I've read before, we failed. It does. TIGRIS reads like a careful transcription of marathon's 7-stage pipeline, puzzlehunt's blind-persona discipline, chronicle's Anchor Rule, and artisan's tiered simulation — each lifted with attribution, none bent into a new shape. The frontmatter even lists its donors. That's honest sourcing, but it's also the entire architectural story: TIGRIS is a port. My diagnostic question #3 — *if you removed the signature mechanic, would any other system be indistinguishable?* — cuts straight through. Remove the "factory meta-pattern" from TIGRIS and you have marathon-for-board-games. Remove the "board-games" and you have marathon. There is no Tzolk'in-gear here, no real-time phase before the turn-based ordeal, no load-bearing weirdness. The tiered simulation stack (Tier A matrix + Tier B narrated + Tier C tournament) is presented as structure but operates as a taxonomy — three cost tiers of the same activity, not three architectures in tension. I wanted a factory that played like a game; I got a factory that reads like a process-improvement memo. The Euro-purity declaration in §2 closes the door on the one move that could have saved it: embracing the review-subject's *own* architectural weirdness as a shaping force on the reviewer.

## Three greenlights

**G1. Anchor Rule as forced embodiment (axis 4: thematic integration).** §10 requires Tigris & Euphrates to pass through the full pipeline before any Phase 2 skill ships. This is the closest TIGRIS gets to a Galaxy Trucker move — build the ship under time pressure with the parts you have, then fly it through the asteroid field and see what breaks. The anchor isn't a demo; it's a stress test where the artifact under review dictates whether the reviewing system is coherent. If Knizia-AI can't recognize his own triangle, the personas are cardboard. That feedback loop is the most theme-shaped thing in the document.

**G2. Forbidden-words discipline forces axis-grounded claims (axis 1: elegance).** §2 and the referenced `forbidden-words.md` ban the evaluative vague-outs ("fun", "well-designed", "solid") and require every claim to name axis + persona + player-count. This is a rule that does more work than its word count suggests — it collapses a whole class of lazy reviews before they can be written. Per axis 1, that's elegance: one rule, several downstream strategic consequences (the innovation log can only cluster on precise critiques, so the amendment mechanism actually has signal to work on).

**G3. Forward-only rubric versioning with locked prior scores (axis 5: variance calibration).** §6 and §7.7 commit that prior scores stay locked at their scoring version; amendments only apply forward. This treats rubric evolution as a resource players manage rather than noise that corrupts the corpus. It's the spec's cleanest structural move: it acknowledges that the reviewing instrument *will* drift and refuses to retro-fit the past to hide the drift. Chronicle earned its credit here.

## Three red flags

**R1. Zero novel architectural bet (axis 1: elegance; axis 8: emergence).** My first diagnostic asks what the novel architectural bet is. I cannot find one. The spec's own §3 describes the architecture as "Marathon-clone + two adaptations"; the two adaptations are a matrix format (artisan) and the Anchor Rule (chronicle). Neither is novel in TIGRIS — they are already running elsewhere. The *subject matter* changed (board games instead of adventures, puzzles, or novels), but in my framework subject-matter substitution is a re-skin, not an architectural move. Tzolk'in's gears would have been gears even if the theme were trains. TIGRIS's gears are marathon's gears. Per axis 8, that caps emergence: what this factory can discover is bounded by what marathon could discover, minus the things board-games-specifically prevent.

**R2. Tiered simulation stack is a cost ladder, not an architecture (axis 2: decision density; axis 3: interaction).** §8 presents Tier A / B / C as an architectural move. Read the table: the three tiers differ only in *compute spent on the same activity* — rubric scoring. Tier A scores fast, Tier B scores with narration, Tier C scores across many seats. There is no structural tension between the tiers. Compare with what a real architectural stack would do: Tier A rules out, Tier B discovers something Tier A cannot by nature (not by budget), Tier C produces a result type the other two cannot produce. Space Alert's real-time-then-resolution works because the two phases operate on *incompatible* cognition modes; one couldn't substitute for the other at higher budget. TIGRIS Tier B could in principle be replaced by a larger Tier A with the same quality loss profile, which means Tier B is a quantity, not a kind. Per axis 3, this also flattens interaction between the tiers — they don't argue with each other, they just agree at different resolutions.

**R3. Euro-purity in §2 and §14 forecloses the one asymmetry that would make the factory play like a game (axis 4: thematic integration; axis 7: teachability).** §2 declares Knizia the spiritual patron. §14 bans cross-tradition coverage until Phase 2+. Together they commit TIGRIS to reviewing Euros with a Euro lens — a monoculture. My diagnostic #5 asks about asymmetries: variable powers, hidden objectives, real-time pressure. There are none between reviewer types — all seven designer personas use the same 8-axis rubric, same cell-verdict thresholds, same prose structure. Their only difference is the weight vector. That's a costume, not an asymmetry. Compare with Mage Knight, where each hero has not just different numbers but a different *strategic grammar*. A Chvátil-persona ought to review on different axes than a Knizia-persona, not the same axes at different weights. As currently specified, the reviewers will produce a chorus, not a parliament, and the novelty-detection capability of the factory is proportional to its parliament breadth.

## Amendment candidates

- **A1. Add an axis-set asymmetry per persona.** Rubric v1.1 candidate: allow each designer persona to declare up to 2 replacement axes specific to their diagnostic tradition (e.g., Chvátil's "structural novelty" as an axis, Rosenberg's "action-economy friction"). Weighted aggregate computed against the persona's own axis set, not a shared 8. Cost: inter-persona score comparison becomes non-trivial. Benefit: reviewers produce heterogeneous signal, which is the only thing that makes the cluster-trigger in §7.7 interesting.
- **A2. Rename Tier C or reframe the stack.** Currently §8 oversells a cost ladder as an architecture. Either commit to a genuine cognitive-mode difference per tier (e.g., Tier B must surface decisions Tier A literally cannot, by design) or demote the language from "architectural stack" to "simulation budgets." Honesty of framing is a teachability issue (axis 7) for future contributors reading the spec.
- **A3. A "designer-of-the-factory" persona seat.** If TIGRIS reviews its own spec (as it is doing now), the panel should include at least one persona whose diagnostic frame is *factory design* rather than *game design*. Otherwise every spec-review is a category error: seven game designers asked to review a build system.

## Scoring

Weights per my `rubric_weights` vector: elegance 1.0, decision density 1.2, interaction 1.0, thematic integration 1.2, variance calibration 1.0, downtime pacing 1.0, teachability 0.6, emergence/replayability 1.0. Sum = 8.0.

| # | Axis | Score | Justification |
|---|---|---|---|
| 1 | Elegance | 6 | The forbidden-words rule and forward-only versioning each pay rent beyond their word count; against that, §7 duplicates structure marathon already proved (7 stages named and numbered), so the rule-count-to-concept ratio is middling. |
| 2 | Decision Density | 4 | Per-stage the spec does not expose meaningful forks for the factory operator — every game runs the same seven stages in the same order; there is no point where the factory decides what *kind* of review this game wants. |
| 3 | Interaction | 5 | Designer personas interact only through the unweighted SUMMARY average; they do not argue across reviews or update each other's scores, so cross-persona influence is indirect competition for shared axis space rather than confrontation. |
| 4 | Thematic Integration | 5 | Board-game theme shows up as persona roster and rubric axes, but the factory's own mechanics (pipeline stages, gate, handoff) are mechanics-agnostic; strip the board-game vocabulary and marathon is indistinguishable. |
| 5 | Variance Calibration | 7 | Forward-only rubric versioning treats evaluative drift as a managed resource rather than noise; the lock-prior-scores rule is the spec's strongest calibration move. |
| 6 | Downtime / Pacing | 6 | Tier A at 30–60 min keeps the operator engaged; Tier B at 2–4 hours without interleaving decisions creates a noticeable observation window, mitigated but not eliminated by the narration requirement. |
| 7 | Teachability | 7 | §3 pipeline diagram and §4 directory layout let a new contributor locate any artifact in under five minutes; the 15-section spec is dense but mostly self-indexing. |
| 8 | Emergence / Replayability | 4 | The factory's strategy space (what reviews it can produce) is bounded by the 8-axis rubric and 7-persona roster; with identical axes across personas, after 10 reviews a veteran can predict the shape of review 11 before turn 3. |

**Weighted aggregate:** (6×1.0 + 4×1.2 + 5×1.0 + 5×1.2 + 7×1.0 + 6×1.0 + 7×0.6 + 4×1.0) / 8.0 = (6.0 + 4.8 + 5.0 + 6.0 + 7.0 + 6.0 + 4.2 + 4.0) / 8.0 = **43.0 / 8.0 = 5.38**.

**Cell verdict:** marginal. The factory will produce reviews. It will not produce a review I would want to sit down and play.
