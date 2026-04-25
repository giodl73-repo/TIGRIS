---
name: Reiner Knizia review of TIGRIS Factory Design Blueprint
slug: knizia
stage: panel
version: 1.0.0
rubric_version: v1.0
author: knizia
artifact_under_review: tigris\docs\specs\2026-04-19-tigris-design.md
created: 2026-04-19
updated: 2026-04-19
---

# Knizia — review of the TIGRIS factory spec

A note on metaphor. I am asked to score a factory, not a game. I treat the factory's rules as a game's rules: **Elegance** = rule-count-to-depth of the pipeline itself; **Decision Density** = whether each stage forces a live call; **Interaction** = whether the personas meaningfully collide; **Thematic Integration** = whether the euro-patron framing predicts the mechanism; **Variance Calibration** = how the process treats the luck of agent output; **Downtime / Pacing** = whether any stage stalls the line; **Teachability** = whether a new contributor can run a game through in 10 minutes of reading; **Emergence / Replayability** = whether the factory produces materially different reviews over 10, 20, 50 games, or collapses toward a house style.

## Opening verdict

The spec is tight where tightness matters and loose where it should be. Seven stages, eight axes, five lenses, seven designers — the numerology is disciplined, and the forward-only rubric-amendment rule has the shape of a good end-trigger: the system decides, not the author. My concern is the opposite of the one the authors braced for. They feared a factory that would over-produce; the real risk is a factory whose **GATE** rule does its own thinking for it, and whose matrix format will reward compliant scoring over real argument. The T&E anchor is a good choice. If the factory cannot recover the catastrophe rule and the four-aspect minimum-score from first principles, the rubric is wrong, not the game.

## Three greenlights

1. **The GATE rule has a Knizia-triangle shape (§7.4).** Three conditions in conjunction: 50% marginal-or-better, anchor-cell is `win`, no axis scores ≤3 across ≥3 personas at the target count. This forces breadth in the same way the minimum-color rule in *Ingenious* forces breadth — you cannot pass by winning big on one axis. A design that is brilliant on Elegance and broken on Interaction is held back exactly as I would hold it back. The third clause in particular (the critical-fail cluster) pays its rent: it catches structural flaws the aggregate would smuggle through.

2. **Forward-only rubric amendments (§6, §7.7).** Prior scores lock to their `rubric_version`. This is the same principle as a sealed-bid auction: once the bid is in, it is in. The temptation to re-score history every time the rubric moves would destroy comparability across games, and the spec resists that temptation explicitly. The cluster-trigger (2+ innovations in the same dimension across ≥2 games) is a reasonable end-trigger analogue — the system proposes amendments, not the author's mood.

3. **Tier separation with YAGNI gates (§8, §11).** Tier-A runs every game; Tier-B runs only past GATE; Tier-C is commissioned. Skills deferred until used twice manually. This is a **tension budget** imposed on the factory itself: every stage always has slightly less compute than it would like, which forces the work into the cheapest tier that can answer the question. The roadmap refusing to build `/tigris-tierC` and `/tigris-resume` in Phase 1 is the correct form of mercy — mercy for the maintainer, not the designer.

## Three red flags

1. **The 48-cell Tier-A matrix is a point salad without a minimum-score rule.** 7 designers × 5 lenses × 4 counts = 48 cells, each an 8-axis weighted aggregate. This is the failure mode of a rubric that adds rather than constrains. In *Modern Art*, the scoring ruthlessly punishes breadth-without-depth by resetting every round — there is a selection pressure. Here, 48 cells with verdicts {win / marginal / fail} will drift toward a **dominant strategy for the scoring agent**: hedge to marginal everywhere, fail nowhere, and the GATE passes. The spec needs a selection rule — perhaps "a single axis below 4 across any 2 designers at the anchor count is a hard revise" — to punish the hedging strategy. Otherwise the matrix rewards the same blandness it is meant to detect.

2. **Anchor-cell declaration is hand-waved (§7.4).** "The persona-count cell the designer declared (e.g., 'Feld at 4p' for a point-salad game) must be `win`." This is the most load-bearing rule in GATE, and the spec gives it one line. Who declares? When? Can it be revised after a failed gate? In *Ra*, the epoch-end trigger is unambiguous — the sun tile runs out. Here, a designer who declares a weak anchor in turn 1 can re-declare after seeing the matrix, and the rule quietly collapses. Either the anchor is fixed at CONCEPT and locked (like a bid), or this clause provides no constraint at all. As written, it is a rule without strategic consequence.

3. **Five lenses + seven designers overlap on axes the rubric already owns.** Engine-Builder's signature is elegance↑ decision_density↑ interaction↓. Feld's is decision-density authority. Rosenberg's is action-economy. At what point does the Engine-Builder lens express anything a weighted combination of Feld + Rosenberg does not? The spec has 12 voices scoring 8 axes. The **ratio of voices to axes is the wrong direction** for a factory that wants distinct signal. *Tigris & Euphrates* has four aspects and four colors of leaders — the 1:1 is not accidental. I would rather see 4 designer voices that disagree violently than 7 that correlate at 0.8. Cut two designers, or prove their weight vectors are orthogonal.

## Amendment candidates

- **Scoring selection rule.** The rubric needs a compaction mechanism — a rule that penalizes the hedging equilibrium I named above. Candidate: the weighted aggregate is reduced by the standard deviation of the axis scores, so a cell scoring {7,7,7,7,7,7,7,7} beats {10,10,10,10,4,4,4,4}. This is the pyramid rule from *Ingenious*: breadth rewarded, specialization punished, at the scoring level.
- **Anchor-lock protocol.** Add to §7.1 CONCEPT: "The anchor cell (persona × player-count) is declared here, in writing, and cannot be revised after Tier-A begins." Without this, GATE §7.4 clause 2 is ornamental.
- **Designer orthogonality audit.** Before Phase 2, compute the pairwise correlation of rubric_weight vectors across the seven designers. Any pair above 0.85 should merge or one should retire. This is cheaper than the current plan, which is to build all seven and hope.
- **Forbidden-words compliance is a pre-commit check, not a review note.** The spec says /tigris-clean is Phase 3. It should be Phase 1 — vocabulary drift is the quickest route to a factory that produces agreeable mush.

## Scoring

| # | Axis | Score | Justification |
|---|---|---|---|
| 1 | Elegance | 6 | 7 stages, 8 axes, 5 lenses, 7 designers: the rule-count is high, and the spec does not yet demonstrate that each stage produces a concept the others do not. INNOVATE vs. HANDOFF as separate stages, for instance, is on the edge of paying its keep. |
| 2 | Decision Density | 5 | GATE and cluster-trigger are real decisions; CONCEPT and HANDOFF risk being paperwork. A pipeline with two live calls out of seven stages is at the minimum-viable density. |
| 3 | Interaction | 4 | The designers and lenses score independently and are averaged in SUMMARY.md. Real interaction would be cross-persona argument — a dissent protocol where Feld's score must respond to Knizia's. The spec has concurrence, not collision. |
| 4 | Thematic Integration | 8 | The euro-patron framing predicts the mechanism: Knizia's minimum-score shape appears in GATE; forward-only amendments echo sealed-bid auctions; catastrophe-as-narrative-spike appears as the cluster-trigger. Theme earns the math. |
| 5 | Variance Calibration | 6 | Seeded RNG for Tier-B is the correct tool. But agent-output variance (the same persona scoring the same cell differently across runs) is unaddressed. Luck is not yet a resource the factory manages. |
| 6 | Downtime / Pacing | 7 | Tier-A at 30–60 min, Tier-B at 2–4 hr, Tier-C commissioned. The pipeline interleaves compute budgets against scope. Little dead weight in the critical path. |
| 7 | Teachability | 6 | The spec is 472 lines. A new contributor reads CLAUDE.md, the pipeline map, the rubric, the forbidden-words guide, and a persona file before first turn. This is a 20–30 minute teach, not 10. |
| 8 | Emergence / Replayability | 7 | The amendment mechanism is the replayability engine. After 10 games the rubric is no longer v1.0, and the set of proven-orthogonal personas has shrunk or grown. The factory is designed to stop resembling itself, which is the correct answer. |

**Weighted aggregate** (my `rubric_weights`: elegance 1.6, decision_density 1.2, interaction 1.0, thematic_integration 0.6, variance_calibration 1.0, downtime_pacing 0.8, teachability 1.0, emergence_replayability 0.8; sum = 8.0):

(6 × 1.6) + (5 × 1.2) + (4 × 1.0) + (8 × 0.6) + (6 × 1.0) + (7 × 0.8) + (6 × 1.0) + (7 × 0.8)
= 9.6 + 6.0 + 4.0 + 4.8 + 6.0 + 5.6 + 6.0 + 5.6
= **47.6 / 8.0 = 5.95**

**Cell verdict: marginal.** Just under the `win` threshold. The Interaction axis is the single biggest drag; fixing the cross-persona dissent protocol and the hedging-equilibrium problem would move this into `win` territory. As it stands: the factory will produce reviews. Whether those reviews collide hard enough to sharpen the rubric is the open question, and the T&E anchor will answer it.
