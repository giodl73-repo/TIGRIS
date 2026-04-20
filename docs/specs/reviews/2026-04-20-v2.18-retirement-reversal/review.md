---
name: v2.18 Retirement-Reversal — Parliament Review
slug: v2.18-retirement-reversal-review
stage: panel
version: 1.0.0
rubric_version: v2.17
bet_version: parliament
artifact_under_review: docs/specs/2026-04-20-v2.18-retirement-reversal-design.md
personas_seated: [knizia, rosenberg, feld, lacerda, chvatil, kramer-kiesling, stegmaier, vaccarino]
review_type: meta-review (TIGRIS-internal protocol amendment)
anchor_persona: lacerda
anchor_axis: B1
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# v2.18 Retirement-Reversal — Parliament Review

Spec-review of `docs/specs/2026-04-20-v2.18-retirement-reversal-design.md`. 8 designers, no lenses (meta-review). Anchor: Lacerda on B1 System Gearing — retirement-reversal is a gearing question between ledger and argument-record.

## Drafts

### Knizia
1. **A1 Elegance.** 7/10. "Amendment is compact — 1 trigger, 1 effect, 1 rate-limit rule. Clean. Earn-leaning."
2. **C2 Min-Score.** 1/10. "N/A. Retire-explicit."
3. **B2 Catastrophe.** 2/10. "Spec enumerates failure cases in §open questions. Earn-adjacent."

### Rosenberg
1. **B3 Conversion Chain.** 6/10. "Clear pipeline: panel earnings → counter-pressure documentation → collision-vote → refute-weight reduction. 4-step. Earn."
2. **C3 Scarcity.** 1/10. "N/A. Retire-explicit."
3. **D4 Late-Game Lock-in.** 3/10. "Amendment is forward-only except one-time retroactive. Weak lock-in. Hold."

### Feld
1. **A2 Decision Density.** 5/10. "User decisions: identify counter-pressure game, confirm collision-vote, apply refute-weight reduction manually. 3-step density."
2. **B2 Catastrophe Pressure.** 5/10. "Failure modes enumerated in §open questions. Partial earn."
3. **C6 Point-Salad.** 1/10. "N/A to protocol amendment. Retire-explicit."

### Lacerda
1. **B1 System Gearing (anchor).** **8/10.** "Retirement-reversal gears the rubric-ledger to the argument-record correctly. Counter-pressure design was previously unreachable by the ledger; now it's reachable via the documented amendment trigger. Earn-canonical."
2. **B4 Info-Transparency-Cost.** 7/10. "Counter-pressure events must be documented in concept.md / design.md — retired-weight reduction is transparent. Earn."
3. **A7 Emergence-Replayability.** 2/10. "N/A to protocol. Retire-explicit."

### Chvátil
1. **A3 Interaction.** 1/10. "N/A. Retire-explicit."
2. **B5 Architectural Novelty.** **6/10.** "Amendment introduces a retirement-reversal protocol not seen in other adversarial-review systems (only one-way accumulate-or-explicit-clear patterns). Moderate novelty. Hold-leaning earn."
3. **B6 Scoring Multiplier.** 1/10. "N/A. Retire-explicit."

### Kramer-Kiesling
1. **C7 Action-Menu Clarity.** 6/10. "Amendment has clear trigger + effect + rate-limit. User step explicitly documented. Earn."
2. **D1 Family-to-Expert.** 4/10. "Protocol readable by fresh TIGRIS contributor after reading v2.0 spec + v2.14 amendment. Moderate."
3. **D2 Spatial-Interaction.** 1/10. "N/A. Retire-explicit."

### Stegmaier
1. **A6 Teachability.** 6/10. "Spec is 4 sections + §open questions + §success criteria. Readable in ~8 min. Earn."
2. **D3 Count-Robustness.** 1/10. "N/A. Retire-explicit."
3. **C8 First-Turn Compression.** 1/10. "N/A. Retire-explicit."

### Vaccarino
1. **A4 Variance Calibration.** 2/10. "N/A. Retire-explicit."
2. **A5 Downtime-Pacing.** 1/10. "N/A. Retire-explicit."
3. **C4 Engine-Garden.** 1/10. "N/A. Retire-explicit."

## Argument

### Round 1 — the retroactive application question

- **Lacerda (B1 anchor) defends**: "The amendment correctly applies the reversal to A3 (2 counter-pressure games: Vigil + Covenstat; collision-vote confirmed) but holds A7 at 1R because Covenstat is only 1 game (2 sessions ≠ 2 games). The '≥ 2 different games' requirement is strict and sensible. +1 defend on B1." → B1 at 1 defend.
- **Stegmaier (A6) agrees**: "The asymmetry between A3 (eligible) and A7 (not-yet-eligible) in the same commit is clear evidence the protocol's ≥ 2 games requirement is being enforced, not waived for convenience. +1 defend on A6." → A6 at 1 defend.
- **Knizia (A1)**: "Elegance: the amendment doesn't hand-wave A7's case to make the story cleaner. That kind of discipline is elegant. +1 defend on A1." → A1 at 1 defend.

### Round 2 — the collision-vote requirement

- **Lacerda**: "§Trigger item 2 requires a collision-vote confirming canonical earning. This prevents soft-earn gaming (where a counter-pressure game earns on a technicality rather than an adversarial test). Strong gate."
- **Chvátil (B5) partial attack**: "Requiring collision-vote may be too strict. A counter-pressure earning without collision may still be load-bearing. What if no collision naturally arises in the counter-pressure game? Do we artificially manufacture one? Soft refute."
- **Vote on Chvátil's concern**: Feld → Lacerda (collision-vote gates against soft-earns; good); Rosenberg → Chvátil (manufactured collisions are bad); Stegmaier → Lacerda (A3 and A7 counter-pressure games both naturally produced collisions, so the bar is achievable); Knizia → Lacerda (discipline); K-K → Lacerda (traceable standard); Vaccarino → Chvátil (strictness may block good earnings).
- **Result: 4-2 for Lacerda B1. Collision-vote requirement holds.**

### Collision — elegance vs rigor

- **Chvátil (A1)**: "Amendment would be simpler without the collision-vote clause. Cleaner spec."
- **Lacerda (B1)**: "But then refute-weight would reduce from ordinary re-earnings, defeating the counter-pressure-specific intent. Rigor > brevity."
- **Vote**: Stegmaier → Lacerda; Feld → Lacerda; K-K → Lacerda; Rosenberg → Chvátil; Vaccarino → Chvátil; Knizia → Lacerda.
- **Result: 4-2 for Lacerda. Rigor wins.** Both axes preserved (orthogonal-preservation pattern holds).

### Round 3 — Feld's failure-mode concerns

- **Feld (B2)**: "§Open question 2 asks what happens if a counter-pressure design ALSO refutes another axis. Spec proposes 'axis accounting is independent' but doesn't work an example. Vigil actually did refute A6 partial — so the example is concrete and should be IN THE SPEC, not in open questions."
- **Stegmaier seconds**: "Agreed — Vigil's A6 partial-refute is already-known evidence; moving it from open-questions to main text would demonstrate the protocol handles multi-axis impact."
- **Vote**: Feld → refute; Stegmaier → partial refute; consensus partial refute.
- **Result: B2 soft-refute.** Amendment required: move Vigil example into main spec.

## Earned stakes (5)

| Persona | Axis | Status |
|---|---|---|
| **Lacerda (anchor)** | **B1 System Gearing** | Earned canonical (gearing ledger to argument-record) |
| Lacerda | B4 Info-Transparency-Cost | Earned (transparent event-logging requirement) |
| Knizia | A1 Elegance | Earned (compact spec, disciplined application) |
| Stegmaier | A6 Teachability | Earned (readable in 8 min) |
| Rosenberg | B3 Conversion Chain | Earned (4-step pipeline) |

## Refuted stakes (1 soft-refute)

| Persona | Axis | Status |
|---|---|---|
| **Feld** | **B2 Catastrophe Pressure** | Soft refute — multi-axis refute example (Vigil A6 partial) should be in main spec, not open questions |

## Retire-explicits (11)

All N/A-to-protocol axes. Adopted preservations +0.5r each.

## Silent-retire: 0

## Success criteria

| # | Criterion | Result |
|---|---|---|
| 1 | ≥ 5 earned | PASS (5) |
| 2 | ≥ 2 on Band B/C | PASS (B1, B3, B4) |
| 3 | ≥ 1 clean collision | PASS (A1↔B1 4-2 for B1 rigor) |
| 4 | Spec-amendment | **PASS — 1 required spec amendment (A-spec-2.18-01)** |

## Required amendment before adoption

### A-spec-2.18-01 — Move multi-axis-refute example into main spec

§Open question 2 currently asks about multi-axis-refute; this question has already-answered evidence (Vigil's A6 partial-refute while earning A3 counter-pressure). Move the resolved answer into the main spec as a worked example under a new subsection "§Worked example: Vigil's dual-axis outcome."

Remove from open questions once moved.

## Verdict

**Spec approved for v2.18 adoption after A-spec-2.18-01 applied.** Core protocol (counter-pressure earnings → collision-vote-gated refute-weight reduction) is architecturally sound. Retroactive application correctly identifies A3 as eligible and A7 as not-yet-eligible, demonstrating the protocol's discipline.

Recommended action: apply A-spec-2.18-01 amendment, bump spec to v1.1, then formally adopt v2.18 and apply retroactive refute-weight reduction on A3.
