---
name: Uwe Rosenberg review of TIGRIS Factory Design Blueprint
slug: rosenberg
stage: panel
version: 1.0.0
rubric_version: v1.0
author: rosenberg
artifact_under_review: C:\src\tigris\docs\specs\2026-04-19-tigris-design.md
created: 2026-04-19
updated: 2026-04-19
---

# Uwe Rosenberg — Review of the TIGRIS Factory Design

## Opening verdict

I spent an hour walking the pipeline before I formed an opinion, because that is what the spec asks of me: trace the engine before judging it. What TIGRIS describes is a seven-stage action economy with a hard gate in the middle, and my instinct — the same instinct I apply to Agricola's feeding phase — is to ask whether each stage *bites*. Most of them do. CONCEPT forces a declared anchor mechanic and an artifact, so the designer cannot begin without committing. DESIGN forces a rulebook-shaped output. TIER-A forces a matrix — the designer cannot hand-wave past a cell that reads `fail`. GATE is the feeding phase: a periodic obligation that converts abundance (a pile of cells) into shortage (you must have 50% marginal-or-better, and your anchor cell must be `win`, or you starve back to DESIGN). That structure has teeth. Where I grow quieter is further downstream — INNOVATE and HANDOFF are well-shaped on paper but I cannot yet tell whether they will diversify the rubric across ten games or merely annotate it. The factory is engineered with the patience I respect; my concerns are about whether the scarcity is calibrated, and whether the engine-garden of personas really gears together.

## Three greenlights

1. **The GATE stage is a true feeding phase** (elegance, decision_density). Spec §7.4 imposes three independent obligations: 50% cells marginal-or-better, anchor-persona cell must be `win`, and no axis ≤ 3 across ≥ 3 personas at the target count. Those are not a single threshold in three disguises — they trigger on different structural failures, and the third catches a flaw the aggregate would launder. This is the same pattern I use when I separate food from wood from reed: one upkeep that checks three scarcities at once. A design reaching GATE with a clever aggregate but a Decision-Density cluster-fail gets sent back. That bites.

2. **Forward-only rubric versioning preserves the archive's integrity** (emergence_replayability, variance_calibration). Spec §6 and §7.7 lock prior scores to their rubric version and never retro-adjust. This is how you make a decade of reviews comparable. It is also how you keep the INNOVATE stage honest — if amendments were retroactive, every cluster would be an excuse to re-litigate old games, and the log would drift. The archive becomes a ledger in the sense Le Havre's ledger is: the accumulation of prior rounds is what gives the current round its meaning.

3. **The anchor-game discipline** (thematic_integration, teachability). Spec §10 commits TIGRIS to running Tigris & Euphrates through the whole pipeline before building Phase-2 skills. This is the correct ordering. It calibrates the rubric against a known masterwork — if Knizia-AI fails to recognize his own triangle, the persona is wrong, not the game. It also provides a teach-reference: a new contributor can read the anchor's artifacts and learn what "good output" looks like without reading the spec cold. A rules teach that collapses into a one-page player aid after the cliff; here the anchor review is that aid.

## Three red flags

1. **The per-persona `rubric_weights` vectors have no stated orthogonality check** (elegance, decision_density). My vector up-weights decision_density (1.4) and emergence_replayability (1.4) and down-weights thematic_integration, variance, pacing, teachability (0.8 each). Feld's vector presumably up-weights decision_density as well. Lacerda's up-weights elegance-of-interconnection. If three designer vectors correlate at 0.9, the panel has three voices but one axis of opinion, and a game can pass or fail by appealing to that single shared bias. Spec §5 and §6 do not require the roster's weight vectors to span the 8-dimensional space, and they do not report the pairwise correlations. That is a hidden dominant-engine problem — like Agricola with grain before occupations were tuned.

2. **The INNOVATE cluster trigger is under-specified at the edges** (variance_calibration, emergence_replayability). Spec §7.7 says "2+ innovations in the same dimension across ≥ 2 games → propose amendment." But dimensions are the 8 rubric axes, which are coarse; almost any friction in a review will name one of them. After three games, you will have clusters on *every* axis. The trigger fires constantly, the user approves or rejects, and the rubric either sprawls (every amendment adopted — rules inflation without payload) or stalls (user rejects most — the mechanism becomes decorative). I do not see a sub-axis taxonomy, a minimum specificity requirement, or a rate limit on amendments per rubric version. Without those, the INNOVATE stage risks becoming a turn that auto-resolves.

3. **The Tier-A matrix is a 48-cell scoring exercise with no declared inter-rater calibration** (variance_calibration, interaction). Spec §7.3 promises 7 designers × 5 lenses × 4 counts at roughly 30–60 minutes of model time. That is approximately 38 seconds per cell, each of which carries 8 axis scores plus a one-sentence justification. Either (a) cells will be shallow — in which case the gate is noisy and marginal games will ricochet across revisions due to scoring variance rather than design defects, or (b) the model will economize by templating cells, at which point the diversity of the roster collapses. Neither failure mode is flagged. In my games the equivalent is when harvest is too fast and nobody accounts for the reed they just used — the accounting is present but unenforced, and the game runs on vibes. This is my largest concern about whether each pipeline stage bites.

## Amendment candidates

- **Persona vector orthogonality axis.** Add a diagnostic to §5 requiring the roster's `rubric_weights` vectors to be checked for pairwise correlation. If two persona weights correlate above some threshold, one of them is redundant as a panel voice. This is the Interaction axis applied to the persona roster itself.
- **Sub-axis taxonomy under each of the 8 rubric axes.** Right now an innovation in "Decision Density" could be about pip-scarcity, about dominant-action kill, or about turn-structure dead zones. These are different diagnoses. The INNOVATE cluster trigger should require dimension + sub-dimension before it counts toward a cluster of 2.
- **Cell-time budget and calibration protocol for Tier-A.** Declare the per-cell time budget, require that each persona cross-check at least one other persona's cell per game for baseline agreement (inter-rater spot-check), and track the agreement rate as a rubric quality metric. This is the variance-calibration axis applied to the scoring process.
- **Rate-limit rubric amendments.** At most one amendment per rubric version, or the version becomes unstable and the ledger of locked scores loses meaning.

## Scoring

Each axis 0–10, with my personal `rubric_weights` (elegance 1.0, decision_density 1.4, interaction 1.0, thematic_integration 0.8, variance_calibration 0.8, downtime_pacing 0.8, teachability 0.8, emergence_replayability 1.4; sum = 8.0).

| # | Axis | Score | Justification |
|---|---|---|---|
| 1 | Elegance | 6 | Seven stages plus sub-artifacts plus two personas rosters is a rules cliff, but the stages each carry distinct payload; rule-count is high, depth-per-rule is acceptable, but not compressed. |
| 2 | Decision Density | 7 | CONCEPT, DESIGN, GATE, INNOVATE each force real design commitments with opportunity cost; TIER-A cells are dense on paper but see red-flag 3. |
| 3 | Interaction | 5 | Designer personas interact at PANEL and at INNOVATE clustering, but there is no required cross-persona contest — each reviewer writes independently; SUMMARY.md averages. Multiplayer-adjacent, not multiplayer. |
| 4 | Thematic Integration | 8 | The Euro-design theme is not paint: the pipeline imitates a designer's own workflow (concept → rules → simulate → playtest → review), so the factory's mechanics *are* its theme. |
| 5 | Variance Calibration | 5 | No calibration protocol for model output variance across Tier-A cells or across re-scorings (red-flag 3); forward-only versioning controls variance across *time* but not within a single pipeline run. |
| 6 | Downtime / Pacing | 6 | Pipeline stages are sequenced with phase-gates; however, a game's handoff-to-next-game cadence is not specified, and §7.8 HANDOFF is thin — the factory may sit idle between commissions. |
| 7 | Teachability | 6 | The spec itself is long but the directory layout in §4 and bootstrapping checklist in §12 serve as the player aid; a contributor can follow the checklist without absorbing all 15 sections first. |
| 8 | Emergence / Replayability | 6 | The innovation + amendment mechanism *could* diversify the rubric over 10+ games, but the cluster trigger is under-specified (red-flag 2); without sub-axis taxonomy I expect convergence on the same 2–3 amendments repeatedly. |

**Weighted aggregate:**
(6×1.0 + 7×1.4 + 5×1.0 + 8×0.8 + 5×0.8 + 6×0.8 + 6×0.8 + 6×1.4) / 8.0
= (6.0 + 9.8 + 5.0 + 6.4 + 4.0 + 4.8 + 4.8 + 8.4) / 8.0
= 49.2 / 8.0
= **6.15 — marginal.**

The factory has the bones of a deep system — GATE bites, versioning is honest, the anchor plan is the right first move — but three structural gaps (persona-weight orthogonality, innovation cluster specificity, Tier-A calibration) need amendment before I would call this ready to build against. Fix those and I expect the aggregate climbs above 7.0 on re-review.
