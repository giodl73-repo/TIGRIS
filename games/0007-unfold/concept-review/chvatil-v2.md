---
name: Chvátil v1.1 validation of UNFOLD concept
slug: chvatil-v2
stage: concept-validation
artifact_under_review: games/0007-unfold/concept.md (v1.1)
rubric_version: v2.6
author: chvatil
created: 2026-04-19
---

# Chvátil v1.1 validation

## Does the P2 revision land?

Yes. The forced-fold cascade is the rule I asked for, and it does exactly the work the pop-up was failing to do in v1.0. "Change one tile, an adjacent tile MUST state-change in the opposite direction" is pop-up-book physics expressed as a placement rule: the 3D-ness isn't decorative anymore, because the cascade *is* the conservation law the physical form implies. Every placement is now a two-tile decision, which means the tactile "open this, fold that" moment is load-bearing — remove the cascade and the tiles revert to overproduced rotation tokens. You can't re-skin this game as a Carcassonne variant now; the cascade leaves a ripple-mark in the placement graph that only pop-up-physics explains. Additionally, the Development Level track gives the cascade somewhere to resolve to — an open/close decision is now a vote on a shared axis with per-culture ideals, which gears the pop-up into the scoring lens rather than letting it float next to it. The three systems now multiply; they do not merely coexist.

## B5 Architectural Novelty — revised score

**8/10.** The forced-fold cascade makes the pop-up the load-bearing signature; combined with the Dev-Level track and the culture-lens, UNFOLD now passes the re-skin test (remove any one system and the game collapses).

## Remaining concern (if any)

The cascade is **single-step by rule** — "cascade does NOT trigger a further cascade." I understand the reason (resolution speed), but capping it kills the Galaxy-Trucker moment where a placement rips across half the board and the table laughs. Playtest whether a bounded multi-step cascade (e.g., ripple until a closed-left tile absorbs it) produces the intended "oh no, what did I just do" payoff. Single-step may be too tidy for the architecture you've built.

## Greenlight to DESIGN?

**Yes.** The P2 fix lands. Move to design.md. Flag the single-step-cascade question for first playtest — if the table isn't reacting to placements with visible shock, revisit the ripple rule. Otherwise the architecture is sound.
