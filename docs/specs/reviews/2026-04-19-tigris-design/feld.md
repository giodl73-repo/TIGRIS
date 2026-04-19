---
name: Stefan Feld review of TIGRIS design blueprint
slug: feld
stage: panel
version: 1.0.0
rubric_version: v1.0
author: feld
artifact_under_review: docs/specs/2026-04-19-tigris-design.md
created: 2026-04-19
updated: 2026-04-19
---

# Stefan Feld reviews the TIGRIS factory

## Opening verdict

A factory is a game you play once per design cycle, and by that measure TIGRIS has the shape I admire: seven stages, each generating something a player (the designer) converts into something else. CONCEPT yields a premise; DESIGN yields rules; TIER-A yields a matrix cell; GATE yields a bit of structural intelligence; TIER-B yields narrative evidence; PANEL yields scored critique; INNOVATE yields rubric evolution; HANDOFF yields next-session state. That is eight outputs across seven stages — a decent point-salad, except the salad is not yet incommensurable enough. Several of the rubric's eight axes will converge on the same underlying judgment in practice, and two of the pipeline's stages risk becoming what I would flag in my own designs as dead turns. The weighted aggregate lands at 6.6 — marginal by my own verdict thresholds, which is exactly where a spec should land before its anchor playthrough. Not a redesign. A revise.

## Three greenlights

**1. The GATE is a legitimate plague phase (axis 5: Variance Calibration; axis 2: Decision Density).** §7.4 forces three orthogonal checks: ≥50% marginal cells, anchor-cell `win`, and no three-persona axis ≤3 cluster at the target count. This is structurally what my Notre Dame plague, my In the Year of the Dragon event track, and my Macao plague cube do: they punish the player who optimized a single dimension without hedging. A designer who stacks their design toward one persona's taste will clear the aggregate threshold but fail the fail-cluster check. That is the right shape. Neglect costs you re-design time, which is the scarcest resource in the factory — exactly how my penalty phases work. I would score this axis of the spec at 8.

**2. The rubric is genuinely multi-track (axis 8: Emergence).** Eight axes, each with five-point anchors, weighted seven ways by seven designer personas, reshaped further by five player lenses — that is a scoring surface where no two games should produce identical cell patterns. Castles of Burgundy scores VP from ships, livestock, buildings, workers, and end-game bonuses precisely because none of those tracks is directly substitutable. TIGRIS's axes aspire to the same property. A review that scored Agricola and Tigris & Euphrates identically would be a failure; the spec gives the rubric enough dimensions to avoid that.

**3. Forward-only rubric versioning (axis 1: Elegance).** §7.7 and §6's version note: prior scores stay locked at their rubric_version, amendments bump the version, scores are not retro-adjusted. This is the rule that prevents the factory from rewriting its own history to flatter its latest taste. It is also cheaper than the alternative, which would require re-scoring every prior game on every amendment — a cost that would swamp the factory by game five. One rule, two strategic consequences: historical honesty and computational tractability. That is the elegance ratio I reward.

## Three red flags

**1. Axes 1 and 2 are partially substitutable (axis 1: Elegance; axis 2: Decision Density).** Elegance is defined as rule-count-to-depth ratio. Decision Density is defined as meaningful choices per turn. These are not independent: a high-elegance game is almost always also a high-density game, because the depth that rules generate shows up as per-turn choices. I would expect a correlation coefficient above 0.7 across any serious game set. When two axes covary that strongly, one of them is a dead axis — it never distinguishes a game from its neighbors, it just reinforces the other axis's call. My point-salad works because ships and buildings and livestock pay in genuinely different VP curves. TIGRIS needs to prove axes 1 and 2 are not just two names for the same judgment. Either tighten Elegance to something like "rule surface area per session" (measurable in rulebook pages or teach-minutes) or fold it into axis 7 Teachability and promote a new axis from the innovation log.

**2. CONCEPT is a dead stage for reviews (axis 2: Decision Density).** §7.1 explicitly says CONCEPT is originals-only; for reviews, it is a single-paragraph note saying "imported." That is a dead turn in pipeline terms. A review pipeline that starts at DESIGN will eventually ask: why does the originals pipeline need CONCEPT at all, given that DESIGN will re-encode the premise, artifact, and inspiration fields anyway? Either CONCEPT does real work for reviews (e.g., stating the reviewer's anchor-persona declaration up front, which GATE then tests against) or it should be folded into DESIGN's frontmatter. The spec hints at anchor-persona declaration in §7.4 but doesn't bind it to CONCEPT. Bind it, or collapse the stage.

**3. The anchor-cell requirement has no plague mitigation (axis 5: Variance Calibration).** §7.4's second rule says the designer-declared anchor-cell must be `win`. But the spec nowhere describes what happens when the anchor-persona is the wrong anchor — when the design turned out to serve Rosenberg's lens rather than the Feld lens it was pitched as. In my games, a bad worker placement gets you a bonus tile as consolation; you are never zeroed out. GATE currently zeroes out. This is too spiky; the anchor declaration becomes a single point of catastrophic failure, which pushes designers toward conservative anchor picks (Knizia at any count, effectively the null hypothesis). Add an anchor-revision rule: if two non-anchor personas score higher than the declared anchor, the designer may re-declare once per pipeline run. That preserves the penalty while mitigating the variance.

## Amendment candidates

- **A1 — Axis covariance audit (dimension: elegance × decision_density).** After the anchor review (Tigris & Euphrates), compute the pairwise correlation across all 56 cell-axis entries. Any pair with |r| > 0.7 is a candidate for consolidation. Scope: universal.
- **A2 — Dead-stage test for reviews (dimension: process elegance).** Retrospectively ask, for each of the seven stages at the anchor's completion: what would have been lost if this stage were omitted? Any stage where the answer is "nothing material" gets a revision proposal. Scope: pipeline-architectural.
- **A3 — Anchor re-declaration rule (dimension: variance_calibration).** Permit one anchor-persona re-declaration per pipeline run, triggered when two non-anchor personas score higher than the declared anchor at the target count. Scope: GATE protocol.
- **A4 — Catch-up / plague axis (dimension: variance).** §13 already flags this as an open question. The rubric has Variance Calibration, but no axis specifically for how the game treats a trailing player. I have built my career on this distinction; it deserves its own axis or an explicit sub-anchor in axis 5.

## Scoring (8 axes, 0–10)

| # | Axis | Score | Justification |
|---|---|---|---|
| 1 | Elegance | 6 | Seven stages, clear contracts per §9, but axes 1 and 2 covary — the rulebook is slightly longer than the strategic concept space it generates. |
| 2 | Decision Density | 8 | Every stage produces a named artifact consumed by the next; the GATE's three orthogonal checks in §7.4 force multi-axis decisions at the one moment the pipeline could have collapsed into rubber-stamping. |
| 3 | Interaction | 5 | Personas contest the same matrix cells and the cluster-trigger in §7.7 forces cross-game dialogue, but most review work is independent-lane — indirect competition for rubric attention, not direct confrontation. |
| 4 | Thematic Integration | 4 | The "factory" and "pipeline" framing is paint over a directed-acyclic-graph of markdown transformations; the Tigris & Euphrates homage in §10 is honorary rather than structural. |
| 5 | Variance Calibration | 7 | The GATE functions as a genuine plague phase punishing single-axis optimization; the anchor-cell rule over-spikes the variance for declared anchors without a mitigation, costing a point. |
| 6 | Downtime / Pacing | 6 | Stage time estimates in §7.3 and §8 are honest (30–60 min Tier-A, 2–4 hours Tier-B), but no stage can execute in parallel with the next — a strictly serial pipeline has downtime baked in. |
| 7 | Teachability | 6 | §4's directory layout and §9's frontmatter contract teach the factory shape quickly; the rubric's eight axes and five-point anchors require one thorough read before a persona can score, which matches axis 7's anchor-5 definition. |
| 8 | Emergence / Replayability | 8 | The innovation log and forward-only versioning in §7.7 produce a rubric that provably evolves; success criterion 5 in §15 demands at least one amendment from the anchor, meaning replay of the factory generates genuinely different arcs. |

**Weighted aggregate** (Feld weights: elegance 0.8, decision_density 1.6, interaction 0.8, thematic 0.6, variance 1.0, pacing 1.0, teachability 0.8, emergence 1.4; sum = 8.0):

(6 × 0.8) + (8 × 1.6) + (5 × 0.8) + (4 × 0.6) + (7 × 1.0) + (6 × 1.0) + (6 × 0.8) + (8 × 1.4)
= 4.8 + 12.8 + 4.0 + 2.4 + 7.0 + 6.0 + 4.8 + 11.2
= 53.0 / 8.0 = **6.625**

**Cell verdict: marginal.** A point-salad factory with one genuinely incommensurable dimension (emergence via innovation log), one dead stage (CONCEPT for reviews), and one pair of covariant axes (elegance / decision density). Proceed to anchor; use Tigris & Euphrates' 56-cell data to test amendments A1 and A2 before Phase 2.
