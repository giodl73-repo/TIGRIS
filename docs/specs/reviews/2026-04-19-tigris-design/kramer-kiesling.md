---
name: Wolfgang Kramer & Michael Kiesling review of TIGRIS Factory Design Blueprint
slug: kramer-kiesling
stage: panel
version: 1.0.0
rubric_version: v1.0
author: kramer-kiesling
artifact_under_review: tigris\docs\specs\2026-04-19-tigris-design.md
created: 2026-04-19
updated: 2026-04-19
---

# Kramer & Kiesling — review of the TIGRIS design blueprint

## Opening verdict

A spec is a rulebook for the people who will build the factory. We read it the way we read a 4-page rulebook for a family-to-expert title: can a contributor who has never opened marathon, chronicle, puzzlehunt, rmm, or artisan sit down, read §§1-9, and play their first turn without calling a friend? The answer here is *almost*. The seven-stage pipeline is a clean action-point allowance — CONCEPT, DESIGN, TIER-A, GATE, TIER-B, PANEL, INNOVATE, HANDOFF — each stage a named cost, each stage producing a named artifact. That is exactly the shape we reach for in Tikal and Coal Baron: a fixed menu of moves, each with its output, each with its price. Where the spec falters is at the seams: the five sibling repositories in the `inherits_from` header are cited without summary, and a contributor who has not visited them will stall at the first unfamiliar term ("Anchor Rule", "CERES Tier-A matrix", "signal-artifact discipline"). A 9-year-old cannot play this factory. A graduate student who has read all five ancestors can. That is the gap we would flag before Phase 2.

## Three greenlights

**G1. The seven-stage pipeline is an action-point allowance system.** (Elegance; Teachability.) Each stage has a fixed cost (artifact produced), a fixed adjacency rule (which stage may follow), and a named transition (GATE). This is Tikal's action-point spine ported from table to process. A contributor who reads §7 once can recite the stage sequence — CONCEPT → DESIGN → TIER-A → GATE → TIER-B → PANEL → INNOVATE → HANDOFF — the same way a Tikal player can recite "dig, reveal, claim, guard, move, ascend." That is tight spec design.

**G2. Continuous scoring triggers via GATE and cluster-amendments.** (Decision Density; Variance Calibration.) Azul scores row-completion mid-round; Torres scores at end-of-round; we reject games that concentrate all feedback at the end. The spec's GATE at §7.4 is a mid-play scoring trigger — the designer learns whether their CONCEPT holds before committing three sessions to TIER-B. The cluster-trigger in §7.7 (2+ innovations in one dimension → amendment proposal) is a second mid-play trigger: the rubric itself scores continuously. Neither the designer nor the rubric waits until end-of-game to learn.

**G3. Direct spatial interaction between artifacts.** (Interaction.) The §9 YAML frontmatter contract is spatial play on the file-system board. Every file knows which game it belongs to (`game:`), which stage it sits in (`stage:`), which rubric it was scored against (`rubric_version:`). A downstream skill can traverse this geography the way a Mexica player reads canals. The forward-only versioning rule — prior scores lock at their rubric_version and are not retro-adjusted — is the same discipline as Azul's completed rows: once placed, immutable. That is direct structural interaction between stages, not multiplayer-solitaire pipeline stages that ignore each other.

## Three red flags

**R1. The rules sheet requires five supplementary rulebooks.** (Teachability, hard miss.) The frontmatter's `inherits_from` lists marathon, puzzlehunt, chronicle, artisan, and rmm, and the body refers to "marathon's `adventures/` convention" (§4), "CERES Tier-A matrix format" (§3), "chronicle Anchor Rule" (§3), "puzzlehunt's NATO phonetic roster" (§13), and "marathon 'artifact-as-story' port" (§7.1) — without ever defining them in-text. A contributor who has not read those five repositories will stall on the first page. For a family-to-expert target this is the equivalent of a Torres rulebook that says "see Tikal for action-point rules." We want §3 or an appendix to carry a 150-word capsule per ancestor: *what* each pattern is, *why* TIGRIS inherits it, *where* the reader can confirm. Teachability drops from what could be an 8 to a 6 on this one omission.

**R2. The 8-axis rubric uses hobby-fluent anchors.** (Elegance; Teachability.) Axis 4's anchor-10 names "T&E's balance of the four aspects"; Axis 1's anchor-10 names "T&E's catastrophe triggers"; Axis 3's anchor-10 cites "auction games, T&E conflicts" (per §6 of the rubric file). A contributor who has not played Tigris & Euphrates cannot calibrate anchor-10 on three of the eight axes. This is the rubric asking its user to own a copy of the anchor game — a gate that no family-tier rulebook should place in its scoring reference. We would cut the T&E-specific anchors from the rubric proper and move them into a separate calibration appendix (`reference/anchor-examples.md`), leaving the axis definitions themselves game-agnostic. The rubric must teach without requiring hobby fluency, or it fails at the job of letting a fresh contributor score their first cell.

**R3. Eight axes with flat naming is too many to hold in one head.** (Elegance; Decision Density at the reviewer's turn.) Tikal's action menu has six actions. Azul's has three phases. Coal Baron has four. Our own designs converge on 3–6 moves because that is the number a player can hold live. The 8-axis rubric is above the working-memory line; worse, several axes overlap (Elegance and Decision Density both penalize dead-rule space; Variance Calibration and Emergence both measure strategic range over repeated play). A reviewer scoring eight axes on 7 designers × 5 lenses × 4 counts = 1,120 scores in a single TIER-A pass, per §7.3's "48 cells × 8 axes." The blueprint should either (a) collapse to 5–6 axes for v1.1 once the T&E anchor surfaces actual redundancy (flagged as an open question in §13 already — good), or (b) promote 2–3 axes to primary and demote the rest to secondary, scored only when primary scores leave uncertainty. The current 8-flat is tournament-depth without family-tier scaffolding.

## Amendment candidates

- **Glossary-in-spec requirement.** Any term inherited from a sibling repo gets a ≤50-word definition at first mention. Dimension: teachability (process). Scope: universal. Feeds a new §2.1 "Vocabulary" or front-matter appendix.
- **Rubric anchor discipline: no game-specific exemplars in axis anchors.** Axis anchors describe the *shape* of a score; specific games go in a separate calibration file. Dimension: elegance of the rubric itself. Scope: universal, affects v1.1 rubric.
- **Axis count audit after anchor.** The spec already flags this (§13 open question). We amend to make the audit a required deliverable of the T&E retrospective, not optional. Dimension: elegance. Scope: universal. Trigger: §7.7 cluster mechanism should produce a v1.1 proposal before Phase 2 opens.
- **Teach-time target for the spec.** Add a stated teach-time target ("a new contributor reaches first artifact within N minutes of reading") to §15 Success Criteria. Makes teachability measurable, not aspirational.

## Scoring

| Axis | Score | Justification |
|---|---|---|
| Elegance | 7 | Seven-stage pipeline compresses well; 8-axis rubric inflates the rule sheet beyond what the depth requires. |
| Decision Density | 7 | GATE at §7.4 + cluster-trigger at §7.7 produce meaningful contributor decisions mid-pipeline, not just at end-of-game. |
| Interaction | 6 | Artifacts cross-reference via frontmatter (§9) and rubric amendments feed back into scoring — direct structural interaction — but the stage-to-stage flow is sequential, not interleaved. |
| Thematic Integration | 8 | Naming, frontmatter, and pipeline vocabulary (TIGRIS = Tigris & Euphrates; stages named after process verbs) carry the board-game-factory theme through every artifact. |
| Variance Calibration | 6 | Forward-only rubric versioning (§6) prevents retro-scoring drift; YAGNI skill roadmap (§11) controls scope creep; but no mechanism scales rubric-weight variance as the innovation log matures. |
| Downtime / Pacing | 7 | Per-stage time budgets in §7.3 (30-60 min) and §8 (Tier A / B / C model-time table) prevent contributor dead time; Phase 2/3 gating in §11 prevents front-loading. |
| Teachability | 6 | Pipeline stages teach in under 10 minutes; the five-ancestor prerequisite pushes total first-contribution time past any family-tier threshold. Fixable with a vocabulary appendix. |
| Emergence / Replayability | 8 | Forward-only versioning + cluster-triggered amendments produce a genuinely evolving rubric across games; no solved state after the T&E anchor. |

**Weighted aggregate:** (7×1.4 + 7×1.0 + 6×1.2 + 8×0.8 + 6×0.8 + 7×1.0 + 6×1.4 + 8×0.4) / 8.0 = (9.8 + 7.0 + 7.2 + 6.4 + 4.8 + 7.0 + 8.4 + 3.2) / 8.0 = **53.8 / 8.0 = 6.73 — marginal (upper band).**

Cell verdict: `marginal`. One vocabulary appendix and one rubric-anchor revision move this to `win` before Phase 2 opens. The bones are right; the teach is the work.
