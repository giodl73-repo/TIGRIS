---
name: Panel Summary — TIGRIS Factory Design Blueprint
slug: SUMMARY
artifact_under_review: docs/specs/2026-04-19-tigris-design.md
mode: spec-review
stage: panel
version: 1.0.0
rubric_version: v1.0
author: TIGRIS synthesis
reviewers:
  - knizia
  - rosenberg
  - feld
  - lacerda
  - chvatil
  - kramer-kiesling
  - stegmaier
created: 2026-04-19
updated: 2026-04-19
---

# Panel Summary — TIGRIS Factory Design Blueprint (v1.0)

Seven designer personas reviewed the factory spec. Five player lenses abstained (spec-review mode per `tigris-panel`).

## Panel verdict

**All seven cells: `marginal`. Zero `win`, zero `fail`. Weighted aggregate range 5.38–6.73, unweighted mean 6.08.** By the spec's own §7.4 GATE rules (≥50% marginal-or-better, no axis ≤3 across ≥3 personas at target count), the factory passes. By its own implicit ambition — produce reviews that *collide* — it does not. The panel produced an unusually coherent diagnosis: five of seven reviewers named **persona / rubric-axis orthogonality** as the central structural flaw, from three different angles. This is a cluster-trigger on the innovation log's own first use.

Loudest dissent: **Chvátil** (5.38, lowest). His objection — TIGRIS is a port of marathon + puzzlehunt + chronicle + artisan with no novel architectural bet — is an architectural claim, not a polish complaint. It will not be resolved by the punchlist below. The panel's other six voices treat the port-inheritance as acceptable given the YAGNI roadmap. Chvátil does not.

Highest scorer: **Kramer-Kiesling** (6.73). They also rate the spec highest on Elegance (7) and Teachability (tied low, 6), calling the structural shape "bones right; teach is the work."

## Per-axis consensus

| # | Axis | Kz | Ros | Fld | Lac | Chv | K-K | Stg | Mean | Spread | Status |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | Elegance | 6 | 6 | 6 | 6 | 6 | 7 | 6.5 | 6.21 | 1.0 | tight |
| 2 | Decision Density | 5 | 7 | 8 | 5 | 4 | 7 | 7.0 | 6.14 | 4.0 | **contested** |
| 3 | Interaction | 4 | 5 | 5 | 6 | 5 | 6 | 5.5 | 5.21 | 2.0 | tight (low) |
| 4 | Thematic Integration | 8 | 8 | 4 | 4 | 5 | 8 | 5.0 | 6.00 | 4.0 | **contested** |
| 5 | Variance Calibration | 6 | 5 | 7 | 6 | 7 | 6 | 7.0 | 6.29 | 2.0 | tight |
| 6 | Downtime / Pacing | 7 | 6 | 6 | 5 | 6 | 7 | 7.5 | 6.36 | 2.5 | tight |
| 7 | Teachability | 6 | 6 | 6 | 7 | 7 | 6 | 4.5 | 6.07 | 2.5 | tight (Stegmaier low) |
| 8 | Emergence / Replayability | 7 | 6 | 8 | 7 | 4 | 8 | 7.5 | 6.79 | 4.0 | **contested** |
| — | **Weighted aggregate** | **5.95** | **6.15** | **6.625** | **5.68** | **5.38** | **6.73** | **6.05** | **6.08** | 1.35 | cell: marginal |

Three axes contested (spread ≥ 4):

- **Decision Density (4–8):** Structural readers (Rosenberg, Feld, Kramer-Kiesling, Stegmaier) see GATE + cluster-trigger + frontmatter as real decisions. Depth-skeptics (Knizia, Lacerda, Chvátil) see linear file-producers with ~1 live call per stage.
- **Thematic Integration (4–8):** Surface readers (Knizia, Rosenberg, Kramer-Kiesling) see a Euro-patron framing that predicts the mechanism — theme earns the math. Depth readers (Feld, Lacerda, Chvátil) see a generic process-design rubric that could be re-skinned as software product reviews without loss.
- **Emergence / Replayability (4–8):** The amendment mechanism divides the panel. Six reviewers (scores 6–8) see it as the factory's strongest diversifying feature. Chvátil (4) says identical axes across personas cap the strategy space — after 10 reviews, a veteran predicts review 11 by turn 3.

## Greenlights consensus

Appeared in ≥ 2 reviews:

1. **Forward-only rubric versioning (§6, §7.7)** — named by **all 7 reviewers**. The strongest consensus in the panel. Variously framed as a sealed-bid auction (Knizia), a Le Havre ledger (Rosenberg), a Lisboa clock (Lacerda), historical honesty (Feld), and anti-retro-drift discipline (Chvátil, Kramer-Kiesling, Stegmaier). This is the rule that makes the archive comparable.
2. **Anchor Rule / T&E discipline (§10)** — named by **6 of 7** (all except Kramer-Kiesling, who praised it implicitly through "anchor-game discipline"). Framed as: calibration against a known masterwork (Knizia, Rosenberg), honorary vs. structural homage (Feld), forced embodiment test (Chvátil), tactile success hurdle (Stegmaier), teachability anchor (Kramer-Kiesling).
3. **GATE as plague-phase with three orthogonal checks (§7.4)** — named by **4 voices** (Knizia, Rosenberg, Feld, Kramer-Kiesling). The three-check structure (≥50% marginal, anchor-cell `win`, no ≤3 cluster) punishes the hedging equilibrium the aggregate would launder.
4. **Frontmatter contract as iconography (§9)** — named by **3 voices** (Stegmaier, Kramer-Kiesling, Rosenberg). One rule producing many downstream behaviours (traversal, re-scoring, version-locking, clustering).
5. **YAGNI discipline in skill roadmap (§11)** — named by **2 voices** (Knizia, Stegmaier). Compute tension budget prevents the maintainer from drowning in Phase 3 skills before Phase 1 is proven.

## Red flags consensus (must-fix)

Appeared in ≥ 2 reviews. Ordered by number of voices:

### RC1 — Persona / rubric-axis orthogonality is unchecked (5 voices)

**Knizia (RF3):** "7 designers × 5 lenses × 8 axes. The ratio of voices to axes is the wrong direction for a factory that wants distinct signal."
**Rosenberg (RF1):** "Per-persona `rubric_weights` vectors have no stated orthogonality check. If three designer vectors correlate at 0.9, the panel has three voices but one axis of opinion."
**Feld (RF1):** "Axes 1 (Elegance) and 2 (Decision Density) are partially substitutable. I would expect correlation > 0.7 across any serious game set."
**Chvátil (RF3):** "All seven designer personas use the same 8-axis rubric. Their only difference is the weight vector. That's a costume, not an asymmetry. As currently specified, the reviewers will produce a chorus, not a parliament."
**Kramer-Kiesling (RF3):** "Eight axes with flat naming is too many to hold in one head; several axes overlap — Elegance and Decision Density both penalize dead-rule space; Variance Calibration and Emergence both measure strategic range."

This is the dominant structural finding of the panel. Five independent reviewers flagged it from three different angles (persona-weight collinearity, axis covariance, and axis-rubric monoculture). **Must be addressed before Phase 2.**

### RC2 — Tier-A matrix has no selection pressure or inter-rater calibration (3 voices)

**Knizia (RF1):** "48 cells with verdicts {win / marginal / fail} will drift toward a **dominant strategy for the scoring agent**: hedge to marginal everywhere, fail nowhere, and the GATE passes."
**Rosenberg (RF3):** "~38 seconds per cell, carrying 8 axis scores plus justifications. Either cells will be shallow or the model will template them. Neither failure mode is flagged."
**Lacerda (RF3):** "At 75 seconds per cell, each diagnostic question gets six seconds of reasoning. Lisboa cannot be scored in 75 seconds by anyone. A factory whose GATE is computed from fast matrix cells will pass designs whose depth lives in surface features and fail designs whose depth lives in across-turn dependency."

### RC3 — INNOVATE cluster-trigger is under-specified at the edges (3 voices)

**Rosenberg (RF2):** "'2+ innovations in the same dimension across ≥2 games → propose amendment.' Dimensions are the 8 rubric axes, which are coarse; almost any friction will name one. After three games, you will have clusters on every axis."
**Lacerda (RF1):** "Clustering becomes string-matching on axis names rather than recognition of a recurring pattern. The PANEL → INNOVATE seam does not gear; it concatenates."
**Knizia (amendment A3):** Orthogonality audit must precede clustering; otherwise "cluster" is a bucket label for any score drift.

### RC4 — Rubric is generic, re-skinnable without loss (2 voices, structural)

**Lacerda (RF2):** "Strip 'board game' from the eight axes and re-skin them for software product reviews. The rubric still works. A rubric *about Euro games* should have axes that cannot survive re-skinning — catastrophe-pressure, conversion-chain depth, residual-VP density, information-transparency cost."
**Chvátil (RF1):** "Zero novel architectural bet. The subject matter changed — board games instead of adventures, puzzles, or novels — but subject-matter substitution is a re-skin, not an architectural move."

Two voices, but both are structural claims that would force a rubric redesign. Not a polish item.

### RC5 — First-hour contributor onboarding fails at the sibling-repo inheritances (2 voices)

**Kramer-Kiesling (RF1):** "The `inherits_from` block cites five sibling projects, and terms like 'CERES Tier-A matrix', 'Anchor Rule (chronicle)', 'artifact-as-story' appear without in-spec definition. A fresh contributor cannot read this spec cold and act."
**Stegmaier (RF1):** "A fresh contributor cannot produce a useful panel review within one hour because the vocabulary assumes four sibling-repo readings first. The iconography (YAML frontmatter) is present; the legend key is hosted off-board."

Both teachability-heaviest personas. Not contested.

### RC6 — Anchor-cell declaration is under-specified (2 voices)

**Knizia (RF2):** "'The persona-count cell the designer declared must be `win`.' This is the most load-bearing rule in GATE and the spec gives it one line. Who declares? When? Can it be revised after a failed gate? As written, it is a rule without strategic consequence."
**Feld (RF3):** "The anchor-cell requirement has no plague mitigation. GATE currently zeroes out. This is too spiky; the anchor declaration becomes a single point of catastrophic failure, which pushes designers toward conservative anchor picks."

### RC7 — 8-axis rubric size + overlap (2 voices)

**Feld (RF1):** Elegance/Decision-Density covariance.
**Kramer-Kiesling (RF3):** "Eight axes with flat naming is too many to hold in one head; Variance/Emergence also overlap. The 8-flat is tournament-depth without family-tier scaffolding."

Related to but distinct from RC1 (RC1 is about persona vectors; RC7 is about the axes themselves).

### RC8 — CONCEPT is a dead stage for reviews (1 voice, kept for specificity)

**Feld (RF2):** "§7.1 explicitly says CONCEPT is originals-only; for reviews it is a single-paragraph note saying 'imported.' That is a dead turn. Either CONCEPT does real work for reviews — e.g., stating the reviewer's anchor-persona declaration up front, which GATE then tests against — or it should be folded into DESIGN's frontmatter."

### RC9 — SKILL.md files lack a 60-second quickstart (1 voice)

**Stegmaier (amendment):** "`tigris-panel/SKILL.md` has preconditions, procedure, invariants, and mode differences, but no 'what a first-time caller types and what they get back.'" Referenced against his own `Scythe` player-mat standard.

## Amendment candidates → innovation log

Nine candidates surface. Clustering after panel:

| ID | Proposed amendment | Dimension | Voices | Scope | Feeds |
|---|---|---|---|---|---|
| **A1** | **Persona / axis orthogonality audit before Phase 2.** Compute pairwise Pearson correlation of persona `rubric_weights` vectors; pairs at |r| > 0.85 merge or retire. Separately: compute axis-axis correlation across cells at anchor completion; pairs at |r| > 0.7 are consolidation candidates. | persona roster + rubric structure | Knizia, Rosenberg, Feld, Chvátil, K-K (5) | universal | rubric v1.1 |
| **A2** | **Scoring selection rule — penalize hedging.** Weighted aggregate reduced by standard deviation across axes, so {7,7,7,7,7,7,7,7} beats {10,10,10,10,4,4,4,4}. Forces breadth with teeth. | rubric formula | Knizia | universal | rubric v1.1 |
| **A3** | **Sub-axis taxonomy + rate-limit on amendments.** Each innovation log entry must record a `trigger_pattern` (machine-readable predicate) in addition to dimension. Max one amendment adopted per rubric version. | INNOVATE stage + log schema | Rosenberg, Lacerda, Knizia (3) | universal | rubric + innovation log schema |
| **A4** | **Tier-A cell-time budget + inter-rater spot-check.** Declare explicit per-cell time budget; each persona cross-checks ≥1 other persona's cell per game; track agreement rate as process-quality metric. Optionally split Tier-A into fast-wide + deep-anchor passes. | Tier-A protocol | Knizia, Rosenberg, Lacerda (3) | universal | pipeline §7.3 + §8 |
| **A5** | **Euro-specific axis replacement or addition.** Candidate successor axes — *system gearing* (multiplicative subsystem dependency), *catastrophe pressure*, *conversion-chain depth*, *information-transparency cost* — anchored to Euro games, resistant to domain re-skin. | rubric axis content | Lacerda, Chvátil (2) | universal | rubric v1.1 |
| **A6** | **Anchor-cell lock protocol.** Declared at CONCEPT, immutable once Tier-A begins. For spec-reviews where anchor-cell doesn't apply, define spec-review-specific GATE. Optional: one-time re-declaration permitted if two non-anchor personas exceed the declared anchor. | GATE protocol | Knizia, Feld (2) | universal | pipeline §7.4 |
| **A7** | **Sibling-repo glossary in-spec + SKILL.md quickstart contract.** Every inherited term gets ≤ 50-word definition at first mention. Every SKILL.md carries a 60-second quickstart section. First-hour contributor reach becomes a measured sub-metric on axis 7. | teachability | K-K, Stegmaier (2) | universal | CLAUDE.md + SKILL.md standard |
| **A8** | **Persona axis-set asymmetry (optional per persona).** Each designer may declare up to 2 replacement axes specific to their tradition (e.g., Chvátil's "structural novelty"). Converts chorus to parliament. Cost: cross-persona aggregation becomes non-trivial. | persona architecture | Chvátil (1) | bold amendment, defer to post-anchor | future rubric major rev |
| **A9** | **Dead-stage retrospective.** Post-anchor, for each pipeline stage ask: "what would have been lost if this were omitted?" Stages scoring "nothing material" get revision. Especially CONCEPT for reviews. | pipeline architecture | Feld (1) | universal | post-anchor retrospective |

Top three amendments for the innovation log (cluster-trigger satisfied: 3+ independent voices):
- **A1** (orthogonality audit) — 5 voices — HIGH priority
- **A3** (sub-axis taxonomy) — 3 voices — HIGH priority
- **A4** (Tier-A calibration) — 3 voices — HIGH priority

A2, A5, A6, A7 each have 2 voices — cluster-trigger threshold of 2+ met per spec §7.7. Promote all four to innovation-log `proposed-amendment` status.

## Punchlist (ordered)

Before starting the Tigris & Euphrates anchor review, address:

1. **[A1 / RC1]** Add an orthogonality-audit procedure to §5 or §7.3. Minimum: state that persona weight vectors and axis scores will be correlated after the anchor and above-threshold pairs must be justified or merged.
2. **[A3 / RC3]** Add a `trigger_pattern` field to the innovation-log schema (`personas/playtest-innovations.md`). Add a rate limit: one amendment adopted per rubric version.
3. **[A4 / RC2]** Add explicit per-cell time budget to §7.3 and define an inter-rater spot-check protocol (one cross-check per game minimum).
4. **[A6 / RC6]** Tighten §7.4 GATE: who declares the anchor cell, when it locks, what happens under spec-review mode where no cell applies.
5. **[A7 / RC5]** Add sibling-repo glossary section to CLAUDE.md or as §2.1 of the spec. Add 60-second quickstart to `skills/tigris-panel/SKILL.md` and make it a standard for future skills.
6. **[A5 / RC4]** Open a design question for v1.1: add Euro-specific axis candidates (system gearing, catastrophe pressure, information-transparency cost). Defer resolution to post-anchor retrospective.
7. **[A9 / RC8]** Either bind CONCEPT to anchor-cell declaration for reviews, or fold it into DESIGN.
8. **[RC9]** Add 60-sec quickstart to SKILL.md template.

Punchlist items 1–5 are **gating** — address before the T&E anchor. Items 6–8 feed the post-anchor retrospective.

## Cell verdicts

| Reviewer | Weighted aggregate | Verdict |
|---|---:|---|
| Reiner Knizia | 5.95 | marginal |
| Uwe Rosenberg | 6.15 | marginal |
| Stefan Feld | 6.625 | marginal |
| Vital Lacerda | 5.68 | marginal |
| Vlaada Chvátil | 5.38 | marginal |
| Kramer-Kiesling | 6.73 | marginal (upper) |
| Jamey Stegmaier | 6.05 | marginal |
| **Panel mean** | **6.08** | **marginal** |

## GATE check (applied to the spec itself)

Spec §7.4 GATE thresholds:
- ✅ **≥50% cells marginal-or-better**: 100% (7/7).
- ➖ **Anchor-cell must be `win`**: inapplicable — no anchor cell was declared for spec-review mode. See RC6/A6. This omission is itself an innovation candidate.
- ✅ **No axis ≤ 3 across ≥3 personas at target count**: minimum axis score is 4 (Chvátil on Decision Density; Feld, Lacerda on Thematic Integration; Chvátil on Emergence; Knizia on Interaction; Stegmaier on Teachability at 4.5). No cluster of ≥3 personas at ≤3 on any axis.

**Formal GATE: pass.** Material GATE: conditional on addressing the top-3 amendments before Phase 2.

## Next step

1. **Log** all 9 amendment candidates to `personas/playtest-innovations.md` with the cluster information above.
2. **Ratify** the top-5 amendments (A1, A3, A4, A6, A7 — all with 2+ voices) with the user before Phase 2 opens. Emit a **rubric v1.1 proposal** that incorporates them.
3. **Address** punchlist items 1–5 as spec amendments to `docs/specs/2026-04-19-tigris-design.md` or a new `2026-04-XX-tigris-v1.1-amendments.md`.
4. **Run** the Tigris & Euphrates anchor review under the amended rubric. The anchor will surface whether the amendments land.
5. **Retrospective** after the anchor: do punchlist items 6–8 in light of what the anchor revealed.

The panel produced exactly the kind of collision the spec hoped for. The first innovation cluster is already on the log.
