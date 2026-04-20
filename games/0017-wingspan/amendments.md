---
name: Wingspan — Tier-C Amendments
slug: wingspan-tierC-amendments
game: 0017-wingspan
stage: tierC
version: 1.0.0
rubric_version: v2.15
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# Tier-C Amendment — Wingspan (game #17)

## Headline

**Second adopted-axis formal refute (A3 Interaction via multiplayer-solitaire critique).** Chvátil self-refutes his own primary axis. **Two adopted axes now on contested-watch** (A3 + A7). **First multi-axis test of v2.14 adopted-contested pathway.** 12 earned / 1 refuted / 2 collisions (both orthogonal-preservation) / 14 consecutive 0-silent-retire. **A6 Teachability + D1 Family-to-Expert + D3 Count-Robustness canonical-caliber re-earns** (Wingspan as reference game for all three). **Rubric v2.14 → v2.15.**

## Status changes

- **A3 Interaction**: 2E/1R/2C/1I → **2E/1R/2C/1I (refute weight increased by 1 game of Wingspan self-refute)**. Currently at contested-watch. Needs 1 more refute in a different game to trigger adopted-contested formal status.
- **A7 Emergence-Replayability**: 2E/1R/1C/0I → **2E/1R/1C/0I (Wingspan defense adds 1 earning but refute state unchanged; A7 still at contested-watch, AFoS Halifax Hammer refute standing)**. Counterweight defense slows the refute trajectory.
- **C8 First-Turn Compression**: Wingspan hold-explicit (variance-rich setup inapplicable). No ledger change.
- All other adopted axes: canonical re-earnings or retire-explicits (+0.5r each).

## Stake classification

**Earned (12)**: C4 (Vaccarino anchor), A6 D3 (Stegmaier canonical), C6 A2 (Feld), D1 C7 (K-K), A7 B4 (Lacerda), B3 (Rosenberg), A4 (Vaccarino), A1-LAST-CALL (Knizia).

**Refuted (1 axis, 2 in-session instances)**: A3 Interaction (Chvátil self-refute; multiplayer-solitaire structural).

**Retire-explicit (5)**: C1 C2 (Knizia), C3 (Rosenberg), B2 (Feld), D2 (K-K). +0.5r each. Preserved.

**Hold-explicit (6): D4 (R), B1 (L), B5 B6 (Ch), C8 A5 (S+V).**

**Silent-retire: 0 (14th consecutive).**

## Rubric version: v2.14 → v2.15

Minor bump for two-adopted-axis-refute pathway progression. No new protocol amendments (A-v2.14-01 from last session is the governing framework).

## Innovations

### I-wingspan-01 — Second adopted-axis formal refute: A3 Interaction

- **Dimension:** rubric meta
- **Trigger pattern:** `multi_axis_adopted_refute`
- **Source:** games/0017-wingspan/ (Chvátil self-refute at Round 3 + 2p confirmation)
- **Supporting voices:** Chvátil (primary; A3's own advocate); Knizia (collision vote supporting refute)
- **Scope:** universal
- **Status:** adopted (log entry)
- **Note:** Two adopted axes now carry formal refute weight (A3 via multiplayer-solitaire, A7 via Halifax Hammer). Per v2.14 A-v2.14-01 amendment, a 2nd refute in a DIFFERENT game would push either axis to `adopted-contested` formal status. **First time in TIGRIS that the two-way ledger is actively tracking multiple at-risk adopted axes.** Self-refute by the axis's own primary advocate is a strong structural signal — Chvátil, who pushed A3 across 12 games, is refuting his own territory based on Wingspan's structural evidence.

### I-wingspan-02 — Canonical reference game pattern for 3 adopted axes

- **Dimension:** pool dynamics
- **Trigger pattern:** `canonical_reference_game_multi_axis`
- **Source:** Wingspan earned 10/10 or 9/10 canonical-caliber defends on A6, D1, D3, C6 simultaneously
- **Supporting voices:** Stegmaier (A6 + D3), K-K (D1), Feld (C6)
- **Scope:** pool
- **Status:** observational
- **Note:** A single game serving as the canonical reference for 4 simultaneous adopted axes is unprecedented in TIGRIS. Previous reference-games were single-axis (CoB for C6, Tikal for C7+D2). Wingspan's family-to-expert / teachability / count-robustness / point-salad simultaneity suggests a cluster of Band-A and Band-D axes are correlated in well-designed family Euros. Observation only — no protocol change.

### I-wingspan-03 — Orthogonal-preservation collision pattern solidifies

- **Dimension:** argument protocol
- **Trigger pattern:** `collision_orthogonal_preservation`
- **Source:** Wingspan Collision 1 (A3↔C4, 5-1 for C4) + Collision 2 (A6↔C6, 5-1 for A6)
- **Supporting voices:** 5 of 8 personas voted orthogonal-preservation in both cases
- **Scope:** universal
- **Status:** observational — could become adopted protocol clarification if pattern persists
- **Note:** Three orthogonal-preservation collisions now recorded (AFoS A7↔C4 + Wingspan × 2). Pattern is stable: when two stakes address genuinely orthogonal dimensions of the same game, collision votes preserve both. Protocol clarification candidate — but rate-limit unconsumed; wait for a 4th instance.

### I-wingspan-04 — Counter-pressure defense pattern

- **Dimension:** argument protocol
- **Trigger pattern:** `counter_pressure_defense`
- **Source:** Lacerda's A7 defense at Round 3 (direct counterweight to AFoS Halifax Hammer refute)
- **Supporting voices:** Lacerda (primary)
- **Status:** observational
- **Note:** First instance of a persona specifically defending an adopted axis to counter a prior game's refute evidence. Lacerda's A7 defense on 170-bird-emergence was deliberately framed as counter-pressure against AFoS. Structurally this is rational — if A7 is at risk of adopted-contested, finding positive canonical cases slows the trajectory. Pattern may formalize as "ledger-aware stake strategy."

### I-wingspan-05 — Third consecutive non-own anchor

- **Dimension:** anchor protocol
- **Trigger pattern:** `non_own_anchor_consecutive_3`
- **Source:** TS (Gupta/Matthews) → AFoS (Wallace) → Wingspan (Hargrave)
- **Supporting voices:** observational
- **Status:** observational
- **Note:** Third consecutive game where no panel persona authored the target. All three earned their anchor axis. Non-own-anchor protocol is robust across 3 consecutive applications. Suggests TIGRIS can sustainably review games outside the 8-persona roster.

### I-wingspan-06 — Aviary coverage gap closed

- **Dimension:** corpus representativeness
- **Trigger pattern:** `coverage_gap_closure`
- **Source:** BGG Top 100 scan identified aviary underrepresentation in TIGRIS corpus pre-Wingspan
- **Supporting voices:** TIGRIS-internal
- **Scope:** pipeline-architectural
- **Status:** observational
- **Note:** Pre-Wingspan, TIGRIS had 0 aviary / nature-theme games. Post-Wingspan, 1. Still underrepresented vs Wingspan + Ark Nova + Wyrmspan + Earth (current top-25 nature-cluster), but coverage started. No protocol change.

## State after 17 games

- **24 adopted axes / 25 Pool (96%)** — unchanged.
- 1 retired (C5).
- 0 unadopted live axes.
- **2 adopted-contested-watch** (A3 + A7) — first multi-axis test of v2.14 pathway.
- 3 salvages (C4, C3, B2).
- **8/8 designer-on-own-anchor tally** preserved.
- **14 consecutive games 0% silent-retire.**
- 6 protocol amendments.
- **170+ total earned stakes.**
- **3 consecutive non-own anchors** (TS + AFoS + Wingspan).
- Rubric v2.15.

## Success criteria

| # | Criterion | Wingspan |
|---|---|---|
| 1 | ≥ 5 earned | PASS (12) |
| 2 | ≥ 2 on Band B/C | PASS |
| 3 | ≥ 1 clean collision | PASS (2 orthogonal-preservation) |
| 4 | ≥ 1 adoption/retirement | **PASS — A3 refute advances adopted-contested pathway (2nd axis on watch)** |

4/4 PASS.

## Verdict

Wingspan delivered the strongest canonical-reference-game signal in TIGRIS history (4 simultaneous canonical-caliber earnings) AND activated the v2.14 two-way ledger mechanism for a second adopted axis. The factory now has two adopted axes on contested-watch and a clear trajectory toward first-ever adopted-axis de-adoption if a third refute game materializes for either A3 or A7.

Proceeding to HANDOFF.
