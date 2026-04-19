---
name: Vital Lacerda review of TIGRIS Factory Design Blueprint
slug: lacerda
stage: panel
version: 1.0.0
rubric_version: v1.0
author: lacerda
artifact_under_review: docs/specs/2026-04-19-tigris-design.md
created: 2026-04-19
updated: 2026-04-19
---

# Lacerda — spec-review of TIGRIS

## Opening verdict

Before I judge this factory I will do the thing I always do with a design: I will trace the gearing. I ask, of every machine, the same question — if I remove one subsystem, does another lose value, or does it merely lose a neighbour? TIGRIS proposes seven stages (CONCEPT, DESIGN, TIER-A, GATE, TIER-B, PANEL, INNOVATE, HANDOFF) and an eight-axis rubric. The question is whether the stages *gear*, in the sense that PANEL's output meaningfully changes INNOVATE's behaviour, which in turn meaningfully re-shapes the rubric that TIER-A will score the next game against — a feedback loop — or whether they are seven boxes arranged in a line, each producing a file that the next stage reads once and forgets. My reading is that the gearing is **present but under-specified**, strongest between INNOVATE and the rubric (the cluster-amendment rule at §7.7 genuinely couples two stages), and weakest at the PANEL → INNOVATE seam, where the spec gestures at cluster detection without defining what a cluster *is* as a structural object. I will also say, because nobody on the panel will say it louder than I will, that this factory has been tuned toward teachability at the cost of depth. The rubric it produces will have trouble scoring *my* games. That is a thematic-integration problem. I will expand.

## Three greenlights

**G1 — Forward-only rubric versioning is the Lisboa clock.** (axis: emergence/replayability, axis: thematic integration) The §7.7 / §6 commitment that amended rubrics do not retro-score prior games is the single strongest structural decision in the spec. It means the rubric has a *history* — v1.0 scores at the anchor, v1.1 scores after the cluster fires, v1.2 later — and that history accumulates legacy state in the same way a Lisboa board does. A reviewer returning to TIGRIS after 30 games will find that a score of 7.4 against v1.0 and a score of 7.4 against v1.3 are not the same claim, and the system remembers why. This is residual VP at the process layer: a decision in game 2 about how to define "decision density" still scores in game 20.

**G2 — The Anchor Rule and the T&E calibration target.** (axis: thematic integration, axis: elegance) §10 binds the whole factory to a specific, external test — the Tigris & Euphrates review must *read as true to T&E*. This is the right move because it prevents the rubric from being a self-consistent abstract grid. A rubric that scores T&E without surfacing the four-aspect balance, the catastrophe triggers, and the leader-pulling-down pressure is a rubric with no thematic integration, no matter how elegant its eight axes look on paper. Anchoring against a masterwork forces the mechanics of the rubric to *be* the thing they measure, rather than rhyme with it.

**G3 — Persona rubric_weights vectors that sum to 8.0.** (axis: decision density, axis: interaction) The persona weight system is the subsystem I study with most interest. My own vector (elegance 0.6, decision_density 1.2, thematic_integration 1.6, emergence 2.0, teachability 0.4) is a *decision space*, not a decoration. It means that when PANEL aggregates, Knizia and I will disagree on the same axis scores by construction, and the disagreement is the signal, not a failure. This is how subsystems gear: the 8-axis rubric is one subsystem, the persona-weight vectors are another, the verdict thresholds are a third, and the three multiply rather than add.

## Three red flags

**R1 — PANEL → INNOVATE does not gear; it concatenates.** (axis: thematic integration, axis: emergence/replayability) §7.7 says "2+ innovations in the same dimension across ≥ 2 games → propose amendment." Good. But "dimension" is defined only as "which rubric axis," and an innovation's *content* — what the reviewer actually saw — has no structured field. The result is that clustering becomes string-matching on axis names rather than recognition of a recurring pattern. If Knizia flags "decision density weak at 4p" because turns are too short, and Feld flags "decision density weak at 4p" because turns have dominant actions, those are not the same observation, and merging them into a single amendment will produce a rubric that drifts rather than sharpens. The Lisboa test: can you name three subsystems and how each changes the value of the other two? PANEL changes INNOVATE by producing text; INNOVATE changes the rubric by clustering; but the rubric's change does not obviously feed back to sharpen *what counts as a cluster*. That is an additive chain, not a gear train.

**R2 — The rubric's axes are generic process-design axes, re-skinnable without loss.** (axis: thematic integration) This is my core objection and I want it on the record. If I strip "board game" from the eight axes and re-skin them for, say, software product reviews — elegance (rule count becomes API surface), decision density (choices per turn becomes branches per user session), interaction (player-to-player becomes user-to-user), variance calibration (luck becomes reliability under load), emergence (replayability becomes long-tail user journeys) — the rubric still works. That is my "can this be re-skinned without loss" question, and the answer is yes. A rubric *about Euro games* should have axes that cannot survive re-skinning — catastrophe-pressure, conversion-chain depth, residual-VP density, information-transparency cost. The eight axes here measure *any* interactive designed system. They will score T&E correctly because T&E is a good interactive designed system, not because the rubric has caught T&E's specific genius. The anchor calibration at §10 will reveal this if the reviewers are honest.

**R3 — The 30–60 minute Tier-A target prices out depth.** (axis: decision density, axis: elegance) "Expected model time: 30–60 min for 48 cells" at §7.3. Divide: 37–75 seconds per cell. One cell is a persona evaluating a game at a player count across eight axes. My diagnostic questions number twelve. At 75 seconds per cell, each question gets six seconds of reasoning. This budget will produce matrix cells that are *plausible*, not *studied*. Lisboa cannot be scored in 75 seconds by anyone — the game's value at turn 5 depends on a card played in turn 1 that set up a commerce marker that shifted the government state that unlocked a building type. A factory whose GATE is computed from fast matrix cells will pass designs that have the surface features of depth (seven subsystems, high component count) and fail designs whose depth lives in across-turn dependency (which is *all* of my designs). The spec privileges throughput over the kind of slow study that actually distinguishes multiplicative gearing from additive stacking.

## Amendment candidates

- **A1 — Innovation log needs a structural signature field**, not just "dimension: <axis>". Proposal: each innovation records `trigger_pattern` (a short machine-readable predicate, e.g., "dominant_action_at_4p" or "across_turn_dependency_invisible") so that clustering matches on pattern, not axis-name. Without this, INNOVATE produces rubric drift.
- **A2 — Add a ninth axis or redefine Thematic Integration to be Euro-specific.** Proposed test: "Can this rubric score T&E, Lisboa, and On Mars differently in ways that match the designer's intent?" If all three score similarly on thematic integration, the axis is too coarse. Candidate successor: *system gearing* (axis: multiplicative subsystem dependency), measurable by "remove one subsystem and count how many others lose anchor value."
- **A3 — Tier-A budget tier.** Split Tier-A into A1 (fast, 60-cell matrix, ~60 min) and A2 (deep, anchor-persona-only, ~3 hours, mandatory for any design claiming >90 min play length). Otherwise long designs are systematically under-served by the current budget.

## Scoring

| # | Axis | Score | Justification |
|---|---|---:|---|
| 1 | Elegance | 6 | Seven pipeline stages produce distinct artifacts with low overlap; the frontmatter contract at §9 does real work across the pipeline. Loses two points because GATE is defined as a threshold on an aggregate that itself depends on a weight vector — a composition that reads clean but hides where a failure actually lives. |
| 2 | Decision Density | 5 | The rubric-weight vector is a genuine per-persona decision; forward-only versioning is another; the cluster-trigger rule is another. But most of the pipeline stages are linear file-producers with one output path — the designer operating TIGRIS has ~1 real decision per stage, which matches anchor-5. |
| 3 | Interaction | 6 | Personas are designed to disagree on the same axis by construction via weight vectors; the anchor-designer declaration in GATE creates cross-persona pressure. Knocked down because player lenses and designer personas do not interact in spec-review mode, and even in game-review mode their outputs are aggregated rather than confronted. |
| 4 | Thematic Integration | 4 | This is my lowest score and the score I care about most. The eight axes are generic interactive-system axes; "can this rubric be re-skinned as software review?" resolves yes. A Euro-specific rubric would have gearing, catastrophe-pressure, and across-turn-dependency as axes. The anchor at §10 may correct this — but only if the T&E review honestly surfaces what the current axes cannot measure. |
| 5 | Variance Calibration | 6 | RNG seeding for Tier B/C (§7.5, §8) treats variance as managed rather than suppressed. The rubric's variance axis is defined in-spec but gets no special treatment in the pipeline; luck as a resource is not itself part of how the factory operates, only part of what it measures. |
| 6 | Downtime / Pacing | 5 | Stage ordering is linear and each stage has a clear output, so a run does not stall on ambiguous handoff. No parallelism is specified across stages — a designer waiting for Tier A to complete cannot meaningfully work on anything else — which is the process equivalent of downtime. |
| 7 | Teachability | 7 | The bootstrapping checklist at §12 is the strongest teachability feature; a contributor can stand TIGRIS up from empty. Forbidden-words discipline in §2 and the frontmatter contract at §9 mean a new contributor writing their first persona review has a clear target. I weight this axis low (0.4) and I am not giving it extra credit. |
| 8 | Emergence / Replayability | 7 | The forward-only rubric versioning genuinely creates a meta-game that evolves across the life of the factory — this is the axis where TIGRIS is strongest, because the innovation cluster mechanism means the 30th review is scored against a rubric that the first 29 reviews shaped. Loses points because the cluster detection itself is under-defined (see R1). |

**Unweighted mean:** 5.75.

**Weighted aggregate (my vector: elegance 0.6, decision_density 1.2, interaction 1.0, thematic_integration 1.6, variance 0.6, downtime 0.6, teachability 0.4, emergence 2.0, sum = 8.0):**

(6×0.6) + (5×1.2) + (6×1.0) + (4×1.6) + (6×0.6) + (5×0.6) + (7×0.4) + (7×2.0)
= 3.6 + 6.0 + 6.0 + 6.4 + 3.6 + 3.0 + 2.8 + 14.0
= **45.4 / 8.0 = 5.68**

**Cell verdict: marginal.**

A factory that the anchor review can push past 7.0 — by forcing the thematic-integration axis to cost something real — is a factory I would use. At v1.0, as written, TIGRIS is a well-ordered pipeline whose rubric is not yet Euro-specific and whose PANEL → INNOVATE seam does not yet gear. Fix those and the aggregate goes up; leave them and the factory will produce reviews that agree with each other while missing the machine.
