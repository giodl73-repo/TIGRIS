---
name: Vigil — Tier-C Amendments
slug: vigil-tierC-amendments
game: 0018-vigil
stage: tierC
version: 1.0.0
rubric_version: v2.16
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# Tier-C Amendment — Vigil (game #18)

## Headline

**A3 Interaction earns its 3rd time** — counter-pressure against the Wingspan self-refute. A3 ledger: 3E/1R; remains adopted, contested-watch reduces to single-refute-game from 1. **First successful counter-pressure defense in TIGRIS history.** 12 earned / 0 refuted / 8 retire-explicit / 2 collisions / 0 silent-retire (15th consecutive). **Vigil is a second canonical-reference game** (joining Wingspan) for Stegmaier/K-K/Feld axes. **First TIGRIS original to successfully anchor A6 Teachability** (UNFOLD anchored C6; ZEN PATH anchored C2). Rubric v2.15 → v2.16.

## Status changes

- **A3 Interaction** (adopted): 2E/1R → **3E/1R**. Refute weight unchanged (still 1 game of refute from Wingspan). **Counter-pressure defense succeeded** but does not formally clear contested-watch (per v2.14 A-v2.14-01, refute weight only decreases via explicit retirement-reversal pattern, not earning weight).
- **A6 Teachability**: gains 3rd canonical reference (FACETS + Wingspan + Vigil).
- **C7 Action-Menu Clarity**: gains 3rd canonical reference (PR + UNFOLD + Vigil; Wingspan had it too but via 4-action menu similarity).
- **B2 Catastrophe Pressure**: gains 4th canonical reference (T&E + TtA + TS + Vigil).
- **All other adopted axes**: canonical re-earnings or retire-explicits (+0.5r each).

## Stake classification

**Earned (12)**: A6 (Stegmaier anchor, canonical), C7 (K-K, canonical), D1 (K-K), D3 (Stegmaier, canonical), B2 (Feld, canonical), A2 (Feld), C1 (Knizia, canonical), B4 (Lacerda, canonical), B1 (Lacerda), A3 (Chvátil, **counter-pressure**), A4 A5 (Vaccarino).

**Refuted (0)**: no formal refutes.

**Retire-explicit (8)**: C2 (Knizia), C3 C8 (Rosenberg), C6 (Feld), B6 (Chvátil), D2 (K-K), C4 (Stegmaier), B3 (Vaccarino). +0.5r each. Preserved.

**Hold-explicit (4)**: A1 (Knizia), D4 (Rosenberg), B5 (Chvátil), A7 (Lacerda — contested-watch).

**Silent-retire: 0 (15th consecutive).**

## Rubric version: v2.15 → v2.16

Minor bump for:
1. **A3 counter-pressure event** — first instance of an adopted-contested-watch axis gaining positive earning-weight from a designed counter-pressure target. Observational pattern (see Innovations).
2. **Adjacency chart candidate adopted** — A6 ↔ C7 gear-lock pair formalizes.

No protocol amendments (rate-limit not consumed).

## Innovations

### I-vigil-01 — First successful counter-pressure defense of a contested-watch axis

- **Dimension:** rubric meta
- **Trigger pattern:** `counter_pressure_defense_success`
- **Source:** games/0018-vigil/ (A3 Interaction, 3rd earning via designed counter-pressure)
- **Supporting voices:** Chvátil (primary); Lacerda, Feld, K-K (collision vote)
- **Scope:** universal
- **Status:** adopted (log entry) — first positive instance of the pattern I-wingspan-04 predicted
- **Note:** Vigil was designed explicitly with A3 counter-pressure features (mandatory discussion, Clue milestones, suspicion-driven play adjustment). A3 earned canonically. Pattern confirmed: ledger-aware design can strengthen contested-watch axes. Refute weight does NOT automatically decrease from positive earnings, but the contested-watch classification may be reviewed qualitatively after 2+ counter-pressure earnings.

### I-vigil-02 — A6 ↔ C7 gear-lock adjacency confirmed

- **Dimension:** adjacency chart
- **Trigger pattern:** `new_adjacency_surfaced`
- **Source:** Vigil collision 2; design §11 pre-declaration
- **Supporting voices:** Stegmaier, K-K (self-observed); Feld, Lacerda (voted)
- **Scope:** universal
- **Status:** candidate → **adopted into adjacency-chart** (v2.16 patch)
- **Note:** When a game's action menu IS its teach mechanism, A6 Teachability and C7 Action-Menu Clarity gear together — not orthogonal. Joins the adjacency chart at `personas/adjacency-chart.md` if that chart is to be updated. For now, observational log entry.

### I-vigil-03 — First TIGRIS original to anchor A6

- **Dimension:** original-design corpus
- **Trigger pattern:** `original_design_anchors_adopted_axis`
- **Source:** Vigil concept-and-design pair
- **Supporting voices:** observational
- **Status:** adopted log entry
- **Note:** TIGRIS has produced 3 originals (UNFOLD #7 anchoring C6 Point-Salad; ZEN PATH #10 anchoring C2 Min-Score; Vigil #18 anchoring A6 Teachability). Each original anchors a different previously-adopted axis, demonstrating that the factory can produce originals targeting any adopted axis. A6 was the canonical-reference-game axis (Wingspan reference); Vigil becomes the second A6 reference game.

### I-vigil-04 — First hopper-consumed original (HOP-NNN pipeline validated end-to-end)

- **Dimension:** pipeline architectural
- **Trigger pattern:** `hopper_pipeline_validated`
- **Source:** ideas/hopper.md HOP-002 → games/0018-vigil/
- **Supporting voices:** TIGRIS-internal
- **Status:** adopted log entry
- **Note:** First game consumed from the ideate-hopper. `/tigris-ideate` generated HOP-002, user promoted it, `/tigris-concept HOP-002` seeded the game with backlink and single-commit discipline held. Pipeline works end-to-end: ideate → promote → concept → design → panel. Ideate-hopper v1.0 fully validated.

### I-vigil-05 — Mandatory-discussion mechanism as A3 defensive feature

- **Dimension:** design pattern
- **Trigger pattern:** `mandatory_discussion_enforces_A3`
- **Source:** Vigil design §4.6 + argument record
- **Supporting voices:** Chvátil, Lacerda
- **Scope:** design pattern (observational; not a pool amendment)
- **Status:** observational
- **Note:** Mandatory 60-90s discussion phase per round proved load-bearing for A3 Interaction. Observational pattern: future TIGRIS originals that need to defend A3 can use mandatory-discussion as a design primitive. Candidate for future primitives.md entry.

## Hopper impact

- **HOP-002 Vigil**: fresh → promoted → consumed (single commit `b1bf3dc`). Backlink established.
- **HOP-001 Furrow, HOP-003 Covenstat, HOP-004 Rewild, HOP-005 Nest, HOP-006 Hearth**: remain fresh in hopper for future consumption.

## State after 18 games

- **24 adopted axes / 25 Pool (96%)** — unchanged.
- 1 retired (C5).
- 3 salvages (C4, C3, B2).
- 1 adopted-contested-watch **reduced**: A3 now at 3E/1R (positive-weight counter-pressure applied; refute weight still 1 game).
- 1 adopted-contested-watch **still active**: A7 Emergence-Replayability (2E/1R; Vigil held-explicit, no new evidence either direction).
- **8/8 designer-on-own-anchor tally** preserved.
- **15 consecutive games 0% silent-retire.**
- 6 protocol amendments.
- **180+ total earned stakes.**
- **3 TIGRIS originals** (UNFOLD, ZEN PATH, Vigil); 15 reviews.
- **Ideate-hopper pipeline validated end-to-end** via HOP-002 consumption.
- Rubric v2.16.

## Success criteria

| # | Criterion | Vigil |
|---|---|---|
| 1 | ≥ 5 earned | PASS (12) |
| 2 | ≥ 2 on Band B/C | PASS |
| 3 | ≥ 1 clean collision | PASS (2: A6↔A3 orthogonal-preservation + A6↔C7 adjacency discovery) |
| 4 | ≥ 1 adoption/retirement OR ledger movement on contested | **PASS** — A3 counter-pressure earning (3rd time); adjacency chart candidate adopted |

4/4 PASS.

## Verdict

Vigil is the **first TIGRIS original to successfully execute a ledger-aware design bet**: A3 Interaction was declared in concept as a high-stakes counter-pressure target, and the design specifically enforced it via mandatory discussion. The panel confirmed the mechanism works — A3 earned canonically as the 3rd instance, reducing (though not formally clearing) contested-watch pressure.

A6 Teachability anchor canonical: 9-minute teach with traitor-coop held across 3 sessions. Vigil joins Wingspan as a canonical-reference game for A6.

Proceeding to HANDOFF.
