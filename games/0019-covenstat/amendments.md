---
name: Covenstat — Tier-C Amendments
slug: covenstat-tierC-amendments
game: 0019-covenstat
stage: tierC
version: 1.0.0
rubric_version: v2.17
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# Tier-C Amendment — Covenstat (game #19)

## Headline

**Second successful counter-pressure defense — A7 Emergence earned CANONICAL TWICE** (2E/1R → 4E/1R). **A3 Interaction counter-pressure from Vigil HELD** (3E/1R → 4E/1R). **Both adopted-contested-watch axes now carry decisive positive-weight evidence.** 10 earned (incl. 1 partial) / 0 refuted / 7 retire-explicit / 1 collision / 0 silent-retire (16th consecutive). **TIGRIS originals can now repeatably drive contested-watch axes back to healthy adopted state via counter-pressure design.** First such dual-axis success. Rubric v2.16 → v2.17.

## Status changes

- **A7 Emergence-Replayability**: 2E/1R → **4E/1R** (Covenstat earned canonically twice — S2 round 8 "zero convergence" + S3 round-12 "three different game shapes"). Formal refute weight still 1 game. Qualitative status: **strongly recovered from contested-watch.**
- **A3 Interaction**: 3E/1R → **4E/1R** (Covenstat S2 + S3 earned). Counter-pressure from Vigil HELD in a different mechanical context.
- **A6 Teachability**: 10th earning partial (Covenstat S3 + experienced-group re-teaches). Teach-cliff at first-play remains a documented cost.
- **A4, C6, B4, A2, C7, D3, D1**: canonical re-earnings or re-earnings (routine).
- All retire-explicit adopted axes: +0.5r each. Preserved.

## Stake classification

**Earned (10)**: A7 (Lacerda anchor, canonical x2), A3 (Chvátil, counter-pressure hold), A4 (Vaccarino, canonical), C6 (Feld, canonical), A2 (Feld), B4 (Lacerda, canonical), C7 (K-K), D3 (Stegmaier), D1 (K-K, LAST-CALL), A6 (Stegmaier, partial).

**Refuted (0)**.

**Retire-explicit (7)**: C2 (Knizia), C3 B3 (Rosenberg), B2 (Feld), D2 (K-K), **A5 (Vaccarino — first A5 retire-explicit! 5p clockwise-draft downtime real)**, C4 (Stegmaier).

**Hold-explicit (6)**: A1 C1 (Knizia), D4 (Rosenberg), B1 (Lacerda), B5 B6 (Chvátil), C8 (Stegmaier).

**Silent-retire: 0 (16th consecutive).**

## Rubric version: v2.16 → v2.17

Minor bump for:
1. **Dual counter-pressure success** (both A3 and A7 reinforced) — first instance of designing counter-pressure for two contested-watch axes in a single game.
2. **First A5 Downtime-Pacing retire-explicit**: A5 previously always-earn or hold. Covenstat's 5p clockwise-draft downtime is genuine. A5 ledger now at 3E/0.5R.

No protocol amendments (rate-limit not consumed).

## Innovations

### I-covenstat-01 — Second successful counter-pressure defense (A7)

- **Dimension:** rubric meta
- **Trigger pattern:** `counter_pressure_defense_success`
- **Source:** games/0019-covenstat/ (A7 earned canonical x2 via shared-dice / private-rubric design)
- **Supporting voices:** Lacerda (anchor); Vaccarino, Feld, Chvátil, K-K (collision vote)
- **Scope:** universal
- **Status:** adopted (log entry)
- **Note:** Confirms I-vigil-01 pattern generalizes: ledger-aware design can move contested-watch axes back toward healthy adopted state. Two successful counter-pressures across two different adopted-axis refutes (A3 via Vigil discussion; A7 via Covenstat sheet-catalog). Pattern stable.

### I-covenstat-02 — A3 counter-pressure from Vigil HOLDS across mechanical contexts

- **Dimension:** rubric meta
- **Trigger pattern:** `counter_pressure_defense_generalizes`
- **Source:** Vigil (A3 via mandatory discussion) → Covenstat (A3 via shared-dice opponent-denial)
- **Supporting voices:** Chvátil (A3 advocate across both)
- **Scope:** universal
- **Status:** adopted (log entry)
- **Note:** A3 Interaction earned in two different counter-pressure mechanics (Vigil's discussion; Covenstat's public-sheet draft-denial). Post-Wingspan refute has been effectively overridden by 2 earnings across 2 different mechanical implementations. Qualitatively, A3 contested-watch status reconsidered; formal refute weight still 1 game.

### I-covenstat-03 — First A5 Downtime-Pacing retire-explicit

- **Dimension:** pool dynamics
- **Trigger pattern:** `adopted_axis_first_retire_explicit`
- **Source:** Covenstat 5p clockwise-draft session; Vaccarino (A5 primary) recorded it
- **Supporting voices:** Vaccarino (self-retire); observational
- **Scope:** pool
- **Status:** adopted (log entry)
- **Note:** A5 has been earned 3 times (Famiglia, Tikal-LAST-CALL, CoB) and held in ~10 other games. Covenstat's 5p clockwise-draft is the first design where A5 retire-explicit is honest-stake rather than defensive. Suggests A5 boundary conditions now better-understood: synchronous-draft mechanics at 5p threaten A5 even with simultaneous-marking mitigation.

### I-covenstat-04 — A7 ↔ A4 gear-lock adjacency in roll-and-write

- **Dimension:** adjacency chart
- **Trigger pattern:** `new_adjacency_surfaced`
- **Source:** Covenstat argument (Vaccarino A4 + Lacerda A7 fired together; both canonical)
- **Supporting voices:** Lacerda, Vaccarino (self-observed)
- **Scope:** universal (within roll-and-write subgenre)
- **Status:** candidate → **adopted log entry**
- **Note:** In roll-and-write designs with sheet-catalog variation, A7 Emergence and A4 Variance Calibration gear-lock: variance IS the emergence substrate. Parallels I-vigil-02's A6↔C7 gear-lock in traitor-coop. Both are conditional adjacencies — context-dependent axis pairings.

### I-covenstat-05 — Second hopper-consumed original; pipeline scales

- **Dimension:** pipeline architectural
- **Trigger pattern:** `hopper_pipeline_scales`
- **Source:** ideas/hopper.md HOP-003 → games/0019-covenstat/
- **Status:** adopted log entry
- **Note:** Second hopper-consumed original in consecutive game slots (Vigil #18 from HOP-002, Covenstat #19 from HOP-003). Ideate-hopper pipeline produces design-quality candidates at rate-limited cadence. Four remaining fresh candidates in hopper (HOP-001 Furrow, HOP-004 Rewild, HOP-005 Nest, HOP-006 Hearth).

### I-covenstat-06 — A3 + A7 both reinforced in single game

- **Dimension:** rubric meta
- **Trigger pattern:** `dual_counter_pressure`
- **Source:** Covenstat single-game dual-axis reinforcement
- **Supporting voices:** observational
- **Status:** observational
- **Note:** Unprecedented: a single game earned both contested-watch axes (A3 + A7) canonically, reinforcing both simultaneously. Suggests emergence-heavy + interaction-heavy games can defend multiple adopted-contested axes in one design. Future counter-pressure designs may bundle multiple contested axes.

## Hopper impact

- **HOP-003 Covenstat**: fresh → promoted → **consumed** (commit forthcoming). Backlink to `games/0019-covenstat/`.
- Remaining fresh: HOP-001 Furrow, HOP-004 Rewild, HOP-005 Nest, HOP-006 Hearth.

## State after 19 games

- **24 adopted axes / 25 Pool (96%)** — unchanged.
- 1 retired (C5).
- 3 salvages (C4, C3, B2).
- **Both previously-contested-watch axes decisively recovered**:
  - A3: 4E/1R (Vigil + Covenstat dual-reinforcement).
  - A7: 4E/1R (Covenstat canonical x2).
  - Formal refute weight on both still 1 game each; qualitative status recovered.
- **8/8 designer-on-own-anchor tally** preserved.
- **16 consecutive games 0% silent-retire.**
- 6 protocol amendments.
- **200+ total earned stakes.**
- **4 TIGRIS originals** (UNFOLD, ZEN PATH, Vigil, Covenstat); 15 reviews.
- **2 hopper-consumed originals** (Vigil, Covenstat).
- Rubric v2.17.

## Success criteria

| # | Criterion | Covenstat |
|---|---|---|
| 1 | ≥ 5 earned | PASS (10) |
| 2 | ≥ 2 on Band B/C | PASS (B4, C6, C7) |
| 3 | ≥ 1 clean collision | PASS (A7↔A6 4-2) |
| 4 | ≥ 1 adoption/retirement OR ledger movement | **PASS** — A7 counter-pressure earned canonical x2; A3 Vigil counter-pressure held across mechanical contexts. First dual-axis contested-watch recovery. |

4/4 PASS.

## Verdict

Covenstat is TIGRIS's **first dual-axis counter-pressure success** and confirms that the ledger-aware design pattern (demonstrated by Vigil on A3) generalizes. Two originals in a row have moved contested-watch axes back to healthy adopted state via specific design choices: mandatory discussion (Vigil) and shared-dice-with-private-rubric (Covenstat).

A7 and A3 are now both firmly re-adopted qualitatively, though the v2.14 A-v2.14-01 amendment holds formal refute weight until explicit retirement-reversal. The factory now has an operational practice: **contested-watch axes produce hopper candidates, which become designed counter-pressures, which (if successful) reinforce adoption via earned evidence**.

The only remaining formal constraint on "healing" contested-watch is: does the v2.14 protocol need an amendment to allow positive-earnings-with-counter-pressure-justification to reduce formal refute weight? Flagging as candidate for future protocol amendment; not included in v2.17 (rate-limit holds).

Proceeding to HANDOFF.
