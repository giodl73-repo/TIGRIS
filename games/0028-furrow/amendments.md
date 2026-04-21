---
name: FURROW — Tier-C Amendments
slug: furrow-tierC-amendments
game: 0028-furrow
stage: tierC
version: 1.0.0
rubric_version: v2.23
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# Tier-C Amendment — FURROW (game #28)

## Headline

**A3 Interaction formally recovered via v2.23 retirement-reversal cycle 3.** FURROW's lead/follow trick-taking mechanism is counter-pressure cycle 3 game #2. B5↔A3 OP (6-1) provides the collision-vote confirmation; combined with WEALD's D2↔A3 OP (6-1), the v2.18 two-game requirement is satisfied. **A3: 10E/0R. Contested-watch CLEARED.** Additionally: **B5↔A7 Decisive OP 8-0** (6th temporal-register OP; 2nd unanimous B5↔A7). **HOP-001 consumed** — trick-taking gap closed; pipeline 5-for-5. Rubric v2.22.3 → **v2.23**. 13 earned / 0 refuted / 25th 0-silent-retire.

---

## ★ A3 RETIREMENT-REVERSAL CYCLE 3 — v2.23 Application

**Axis:** A3 Interaction
**Refute-weight before:** 1R (contested-watch entered at NEST #26)
**Refute-weight after:** 0R — contested-watch CLEARED

### Trigger verification (v2.18 A-v2.18-01)

**Condition 1: ≥2 counter-pressure earnings across ≥2 different games.**
- WEALD #27: A3 earns via displacement mechanism (documented in concept.md as counter-pressure target; mandatory mechanical direct interaction). ✓
- FURROW #28: A3 earns via trick-taking lead/follow (documented in concept.md as counter-pressure target; mandatory collective interaction). ✓
- Two different games, two different mechanisms. ✓
- **Condition 1 SATISFIED.**

**Condition 2: ≥1 collision-vote confirmation per game.**
- WEALD #27: D2↔A3 OP 6-1 (spatial landscape vs. adversarial contest). ✓
- FURROW #28: B5↔A3 OP 6-1 (mechanism identity vs. social dynamics). ✓
- **Condition 2 SATISFIED.**

**Rate-limit:** retirement-reversal does not consume the 2-game cycle amendment rate-limit. ✓
**Cycle restriction:** "once per contested-watch-cycle." A3's second contested-watch cycle began at NEST #26; this is its first reversal event. ✓

**Result:** Retirement-reversal fires. A3: 10E/1R → 10E/0R. A3 exits contested-watch. Returns to standard adopted status.

### Counter-pressure mechanisms (cycle 3 documentation)

| Cycle | Games | Mechanism | Interaction register |
|---|---|---|---|
| Cycle 1 (cleared v2.18) | Vigil #18 + Covenstat #19 | Mandatory discussion + public-sheet draft-denial | Social |
| Cycle 2 (NEST entered contested-watch) | — | NEST not a counter-pressure game | — |
| **Cycle 3 (cleared v2.23)** | WEALD #27 + FURROW #28 | Displacement (targeted) + Lead/follow (collective) | **Mechanical** |

Cycle 3's mechanisms are qualitatively stronger than Cycle 1: mechanical mandatory interaction (displacement, follow-suit) is harder to game or design around than social mechanisms (discussion, public-sheet). The corpus now demonstrates that A3 earns robustly across both registers.

---

## Rubric version: v2.22.3 → v2.23

A3 retirement-reversal is a consequential protocol event warranting a minor-version bump (precedent: v2.19, v2.21 for A7 recoveries; v2.22.2 for A3 entry).

---

## Retirement-Reversal Log update

| Date | Axis | Refute-weight change | Counter-pressure games | Collision-vote | Trigger |
|---|---|---|---|---|---|
| 2026-04-20 | A3 Interaction | 1 → 0 (cycle 3) | WEALD #27 (displacement; D2↔A3 OP 6-1) + FURROW #28 (lead/follow; B5↔A3 OP 6-1) | WEALD D2↔A3 OP 6-1; FURROW B5↔A3 OP 6-1 | v2.23 A-v2.18-01 cycle 3 application |

---

## Other status changes

### B5 Architectural Novelty: 4E/3.0R → 5E/3.0R (monitoring; earned 0 games ago; clock reset)

B5 earns via trick-as-action-selection novelty. Monitoring clock reset from game #28.

### Earn-mark increases (13 total)

| Axis | Before | After |
|---|---|---|
| A1 Elegance | 7E | 8E |
| A2 Decision Density | 8E | 9E |
| **A3 Interaction** | 9E/1R | **10E/0R (cycle 3 retirement-reversal)** |
| A4 Variance Calibration | 7E | 8E |
| A6 Teachability | 8E | 9E |
| A7 Emergence-Replayability | 13E | 14E |
| B1 System Gearing | 7E | 8E |
| B5 Architectural Novelty | 4E | 5E |
| B6 Scoring Multiplier | 4E | 5E |
| C1 Tension Budget | 8E | 9E |
| C6 Point-Salad Incommensurability | 5E | 6E |
| D1 Family-to-Expert Scaling | 5E | 6E |
| D3 Count-Robustness | 8E | 9E |

### Retire-explicit contributions (+0.5r each)

| Axis | Before | After | Note |
|---|---|---|---|
| B2 Catastrophe Pressure | 2.5R | 3.0R | monitoring |
| B3 Conversion Chain | 0.5R | 1.0R | |
| C2 Min-Score Shape | 3.0R | 3.5R | cleared; ordinary |
| C3 Scarcity Bite | 2.5R | 3.0R | monitoring (earned #27) |
| C8 First-Turn Compression | 2.5R | 3.0R | monitoring |
| D2 Spatial-Interaction | 3.0R | 3.5R | monitoring (earned #27) |

## Collisions (2)

1. **B5↔A3** — OP 6-1 (Vaccarino dissents). Mechanism identity vs. social dynamics. A3 cycle 3 game #2 collision-vote confirmation.
2. **B5↔A7** — **Decisive OP 8-0** (unanimous). 6th temporal-register OP; 2nd B5↔A7 8-0. Timeless mechanism structure vs. per-game deal variance.

## Innovations

### I-furrow-01 — A3 v2.23 retirement-reversal cycle 3 complete

- **Dimension:** A3 Interaction; v2.18 counter-pressure
- **Trigger pattern:** retirement_reversal_cycle_3_fires
- **Source:** games/0028-furrow + games/0027-weald (combined)
- **Scope:** pattern-across-games
- **Status:** adopted (retirement-reversal applied; v2.23 bump)
- **Note:** Cycle 3 complete with two different mandatory-interaction mechanisms: displacement (targeted mechanical, WEALD) + lead/follow (collective mechanical, FURROW). Cycle 3 is mechanically stronger than Cycle 1 (social mechanisms) because mechanical mandatory interaction is categorically inescapable. The factory has now demonstrated A3 counter-pressure across three cycles and three interaction-register types: social (Cycle 1), spatial-contesting (CANTON intermediate), and mechanical (Cycle 3).

### I-furrow-02 — 6th temporal-register OP; B5↔A7 8-0 second unanimous instance

- **Dimension:** OP corpus; temporal-register sub-type
- **Trigger pattern:** temporal_register_op_sixth_instance
- **Source:** games/0028-furrow (B5↔A7 Decisive OP 8-0)
- **Scope:** pattern-across-games
- **Status:** adopted
- **Note:** Six temporal-register OP instances now confirmed. The B5↔A7 temporal-register (mechanism structure timeless vs. per-game content variance) has now occurred twice unanimously (Codenames #25 and FURROW #28). When B5 claims architectural novelty, A7 claims across-play variety from the same mechanism — these axes are reliable OP partners.

### I-furrow-03 — HOP-001 consumed; trick-taking gap closed; pipeline 5-for-5

- **Dimension:** hopper pipeline; corpus coverage
- **Trigger pattern:** hopper_consumed_5th; coverage_gap_closure
- **Source:** games/0028-furrow (HOP-001 Furrow → game #28)
- **Scope:** pipeline-architectural
- **Status:** adopted
- **Note:** 5th hopper-converted original (Vigil, Covenstat, Rewild, NEST, FURROW). Pipeline remains 5-for-5. Trick-taking mechanism gap closed. HOP-006 (Hearth) is the last remaining fresh hopper entry.

## Success criteria

| # | Criterion | Result |
|---|---|---|
| 1 | ≥ 5 earned | PASS (13) |
| 2 | ≥ 2 on Band B/C | PASS (B5, B1, B6, C1, C6) |
| 3 | ≥ 1 collision | PASS (2 collisions: B5↔A3 OP 6-1 + B5↔A7 Decisive OP 8-0) |
| 4 | Pool update | **PASS (A3 retirement-reversal cycle 3; v2.23 bump; zero contested-watch again)** |

## Next step

HANDOFF.
