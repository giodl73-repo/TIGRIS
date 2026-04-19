---
reviewer: Reiner Knizia
game: 0007-unfold
stage: concept-review-v2
rubric_version: v2.6
pass_type: validation
prior_review: knizia.md (v1.0, C1=4.5)
my_primary_axis: C1 Tension Budget
created: 2026-04-19
---

# Knizia — UNFOLD v1.1 validation

## Did the fixes land?

Yes. Both of my objections were answered in the form I asked for, and one of them was answered better than I proposed.

### Concern 1 — Action economy slack (was 4.5 ceiling)

The turn is now **2 actions**, and each action is atomic (PLACE=1, FLIP=1, MOVE=1, TOOL=1). Run the arithmetic against C1:

- v1.0 budget: 3 actions, PLACE=1, FLIP=2 → a player laid a tile AND still had 2 actions spare. Slack.
- v1.1 budget: 2 actions, PLACE=1, FLIP=1 → a player who PLACEs can do exactly one other thing. PLACE + FLIP is now the *whole turn*, not two-thirds of it.

Critically, the forced-fold cascade makes each action cost **two tile-state changes** (the action + the obligatory neighbor flip in opposite direction). So a 2-action turn is actually a 4-state-change turn, and each of those four changes is coerced by the previous one. A player who PLACEs open-up must close a neighbor; the closed neighbor cancels someone's Permaculture adjacency; the second action now has to *fix that* or advance the Dev track, not both. The choice between "build mine" and "break yours" is now exclusive, as requested. Slack eliminated.

### Concern 2 — No cross-lens forcing rule (the bigger miss)

The Development Level track + Dev-Alignment multiplier is the forcing rule. Check the math:

- Four cultures, four non-overlapping ideal zones (10-25, 20-30, 25-35, 30-40).
- Multiplier range: 0.5× to 1.5×. That is a **3:1 swing** on total score, not a tiebreaker.
- The zones cannot be simultaneously satisfied. Permaculture ideal ceiling (25) < Modernist ideal floor (30). Any shared island ending above 27 is *mathematically hostile* to Permaculture and vice versa.

This is superior to my proposed Island Coherence track because it is **asymmetrically weighted per player**. Same board state, opposite outcomes — this is the T&E-catastrophe trick in a new register: a shared public number whose meaning is private. Every placement is now a vote in a four-way election where each voter scores on their own curve. That is the cross-lens forcing rule. It imports the spirit of C2 (minimum-score) through a different door: you cannot ignore the Dev track because the multiplier eats half your score.

The Dusk Phase at thresholds 10/20/30 adds three forced commitment points — the "one turn before" drama fires *three times*, not once.

## New scores

**C1 Tension Budget:** **7.5/10** (up from 4.5). 2 actions + forced cascade + 3:1 multiplier swing = every turn, the next-best action is unreachable. Held under 8 only because tile-bag depletion rate is unspecified (open question #5) — without a declared clock, the budget could still stretch.

**B5 Architectural Novelty:** **8.5/10** (up from 8.0). Forced-fold cascade promotes the pop-up from *flavor* to *mechanic*. Pop-up physics now has rule-weight: energy conservation as state law.

## Remaining concerns

1. Dev-Alignment thresholds (≤3, 4-8, 9-15, >15) need playtest calibration — a 3-point band is tight against a 40-point track.
2. End-trigger still "first of three" (open Q#5). Pick one. Bag-depletion remains the correct Knizia choice.

## Verdict

**Greenlight.** Proceed to design.md. The math bites.

Word count: 478.
