---
name: Jamey Stegmaier review of TIGRIS Factory Design Blueprint
slug: stegmaier
stage: panel
version: 1.0.0
rubric_version: v1.0
author: stegmaier
artifact_under_review: docs/specs/2026-04-19-tigris-design.md
created: 2026-04-19
updated: 2026-04-19
---

# Stegmaier review — TIGRIS Factory Design Blueprint

## Opening verdict

I keep thinking of this spec the way I think of a Scythe player-mat: the dense version ships with the icons already doing the talking, and if you hand it to a new player who has never opened the rulebook, they can still act on their first turn. TIGRIS does not pass that test yet. The architecture is coherent, the stage pipeline is legible, and there is a real attempt at a frontmatter contract that could play the role iconography plays in my own titles. But the spec asks a new contributor to read five sibling-project inheritances (marathon, puzzlehunt, chronicle, rmm, artisan) before they can produce a panel review within one hour, and the directory layout is the reference sheet rather than being superseded by the components. On teachability — my highest-weighted axis at 1.8 — the first-turn experience for a fresh contributor is guided, not genuine. I want this factory to ship. I want to make the rulebook teach in ten minutes before it does.

## Three greenlights

**G1 — The YAML frontmatter contract plays the iconography role on the elegance axis.** §9 defines a single frontmatter shape (`name`, `slug`, `game`, `stage`, `version`, `rubric_version`, `author`, `created`, `updated`, `sources`) that every artifact carries. This is the closest thing a markdown factory has to a Scythe player-mat: the component carries its own legend. A downstream skill, or a reviewer, can traverse the tree and know what kind of object they are looking at without reading the body. Elegance axis per §6: this is one rule producing many behaviours (filtering, re-scoring, version-locking, cluster detection all fall out of it). That is the ratio I want.

**G2 — The stage pipeline teaches its own sequence on the downtime/pacing axis.** CONCEPT → DESIGN → TIER-A → GATE → TIER-B → PANEL → INNOVATE → HANDOFF (§3, §7) is a seven-step arc where every station has a named output artifact. A contributor who sees `tierA-sim.md` in a game directory knows what came before and what comes next without a tutorial. This is the compressed first-turn experience ported to a process: the pipeline starts teaching the moment you open any game folder. In my titles I would call this the action-board analogue — the shape of the turn is visible before the first action resolves.

**G3 — The anchor-game discipline (§10) is a genuine player-count robustness move on the teachability axis.** Shipping the Tigris & Euphrates review end-to-end before building Phase 2 skills is exactly the pattern I use across solo / 2p / 3p / 4p / 5p tuning: you do not trust the 4p experience until you have played 4p. TIGRIS does not trust its own pipeline until it has run a full cycle at N=1. The success criterion in §15.2 — "a knowledgeable reader recognizes the game from the review alone" — is a tactile test, not a procedural one. That is the right hurdle for a v1.0.

## Three red flags

**R1 — Teachability axis: first-hour contributor onboarding fails at the `inherits_from` block.** The spec frontmatter cites five sibling projects (marathon, puzzlehunt, chronicle, rmm, artisan) as inheritance sources, and terms like "CERES Tier-A matrix format" (§3), "Anchor Rule (chronicle)" (§10), and "puzzlehunt's NATO phonetic roster" (§13) appear without in-spec definition. A fresh contributor reading this spec at cold-start cannot produce a useful panel review within one hour because the vocabulary assumes four sibling-repo readings first. Per my diagnostic question #2 — can a contributor with the spec and no external files act correctly by turn 3 — the answer at 2026-04-19 is no. The iconography (YAML frontmatter) is present; the legend key is hosted off-board.

**R2 — Elegance axis: the `skills/` directory lists twelve slugs, four of which are Phase 1, and the reader has to hold the phasing in their head.** §4 renders all twelve skill directories inline — `tigris-tierA`, `tigris-panel`, `tigris-innovate`, `tigris-handoff`, `tigris-concept`, `tigris-design`, `tigris-tierB`, `tigris-tierC`, `tigris-resume`, `tigris-status` — with "Phase 2" / "Phase 3 (deferred)" as inline comments. §11 then re-enumerates the same list with the phasing attached. A Scythe player-mat never shows a power you cannot yet access; the action spaces that unlock later are not pre-drawn. Here the rule count is high because the rulebook narrates future rules. This is the exact failure mode my red-flags list calls out: iconography that requires a reference card beyond the first turn. Cut the inline list to Phase 1 and link forward.

**R3 — Thematic integration axis: the spec says "Euro-tradition" but the factory itself reads as a release-management system.** §2 declares the anchor tradition is Euro / Knizia, and §6's rubric pulls in the right vocabulary (elegance, decision density, interaction, thematic integration). But the spec's own artifact shape — seven stages, gate thresholds, tracker rows, tiered simulation stack — reads as a CI/CD pipeline in markdown. A newcomer reading the spec top-to-bottom will not feel they are entering a design studio; they will feel they are entering a release pipeline. Per §6 axis 4, mechanics-are-the-theme means the system feels like what it claims to be. The stage names (TIER-A, GATE, HANDOFF) are load-bearing for the process but carry zero theme weight. In Apiary I would not name the worker-advancement phase "GATE."

## Amendment candidates

- **Rubric axis candidate: "First-hour contributor reach."** Teachability axis 7 is scored on rules-to-first-turn friction for a player. For a factory spec, the analogous measure is spec-to-first-useful-review friction for a contributor. I propose the innovation log capture this as a spec-review-specific sub-metric on axis 7 so that spec reviews do not coast on rubric definitions written for games. Dimension: teachability. Scope: pattern-across-artifacts (applies to every spec review this factory produces about itself). Supporting reviewer: Stegmaier.

- **Process amendment candidate: every SKILL.md must include a 60-second quickstart.** The `tigris-panel/SKILL.md` I just read has preconditions, procedure, invariants, and mode differences — all excellent — but no "what a first-time caller types and what they get back." A fresh contributor cannot make their first useful invocation from the SKILL.md alone. Dimension: teachability + elegance. Scope: universal across the skills/ directory.

- **Persona amendment candidate: a "Fresh-Contributor" lens for spec-reviews.** The current player-lens roster (engine-builder, interactionist, ap-prone, thematic, fresh-face) does not apply to spec-reviews per §5.2 and the SKILL.md mode-differences table. But spec-reviews lose the Fresh-Face lens at exactly the moment they need it most: onboarding a new author. Dimension: teachability. Scope: universal.

## Scoring (Stegmaier weights)

| # | Axis | Score | Justification | Weight | Weighted |
|---|---|---|---|---|---|
| 1 | Elegance | 6.5 | Frontmatter contract (§9) and stage pipeline (§3) are high-leverage single rules; twelve-slug skill list (§4) and dual enumeration (§4 + §11) inflate rule count above concept count. | 1.2 | 7.80 |
| 2 | Decision Density | 7.0 | The GATE thresholds (§7.4) create genuine per-stage decisions — revise vs. proceed is a live choice every pipeline run, not a formality. | 0.8 | 5.60 |
| 3 | Interaction | 5.5 | Personas interact via the panel synthesis step (§7.6) and cluster-triggered amendments (§7.7); contributor-to-contributor interaction across games is not yet defined (§13 defers callsigns). | 0.8 | 4.40 |
| 4 | Thematic Integration | 5.0 | Rubric language is in-tradition (Knizia, elegance, tension) but pipeline vocabulary (GATE, TIER-A, HANDOFF) reads as release-engineering — theme rhymes loosely per axis-anchor 3–5. | 1.0 | 5.00 |
| 5 | Variance Calibration | 7.0 | The forward-only rubric versioning (§6, §7.7) locks prior scores against their rubric_version, which is the single strongest anti-variance-noise move in the spec. | 1.0 | 7.00 |
| 6 | Downtime/Pacing | 7.5 | Tier A at 30–60 min / Tier B at 2–4 hours / Tier C opt-in (§8) matches pacing to cost; no stage waits on a stage it does not need. | 1.0 | 7.50 |
| 7 | Teachability | 4.5 | Five sibling-project inheritances (frontmatter + §3 + §10 + §13) plus twelve-slug skill list make the first-hour contributor experience guided rather than genuine — per my diagnostic #5, the first turn is not a real decision. | 1.8 | 8.10 |
| 8 | Emergence/Replayability | 7.5 | Cluster-triggered amendments (§7.7) plus the forward-only rubric (§6) give this factory a genuine meta-evolution loop — across 10+ games the rubric itself will have moved, not just the scores. | 0.4 | 3.00 |

**Weighted aggregate: 48.40 / 80 = 6.05 / 10** → `marginal` per §6.

The aggregate sits in the marginal band because teachability — my heaviest axis at 1.8 — lands at 4.5 and drags the vector down. Elegance, pacing, and variance calibration are all above 7 on their own; the spec knows what it wants to be. It does not yet teach a fresh contributor what it is within the first hour, and until it does, I cannot cell-verdict it `win`. Cut the skills list to Phase 1, inline the sibling-project glossary, and add a quickstart to every SKILL.md, and the teachability axis moves to 7+ and the aggregate clears 7.0. The fix is local, not structural.
