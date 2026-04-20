---
name: Twilight Struggle — Tier-C Amendments
slug: twilight-struggle-tierC-amendments
game: 0015-twilight-struggle
stage: tierC
version: 1.0.0
rubric_version: v2.13
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# Tier-C Amendment — Twilight Struggle (game #15)

## Headline

**C8 First-Turn Compression earns for first time across 15 games** — moves from retirement-watch to **queued-for-adoption** (Stegmaier-primary). **First non-Euro game in factory** stress-tests axis Pool. **First game under v2.13.** No new adoptions (C8 needs 2nd game for full adoption); heavy retire-explicit load from 2p-only CDG constraints absorbed without collapse.

## Status changes

- **C8 First-Turn Compression**: 0 earnings → 1 earning (queued-for-adoption). Dormancy broken.
- **A5 Downtime-Pacing**: LAST-CALL re-earns.
- All other adopted axes: canonical re-earnings or retire-explicits (+0.5r each).

## Stake classification

**Earned (12)**: C1 (Knizia anchor), B2 A2 (Feld), B1 A7 B4 (Lacerda), A3 B5 (Chvátil), **C8 (Stegmaier — FIRST EVER)**, A4 D4 A5-LAST-CALL (Vaccarino).

**Retire-explicit (7)**: C4 B3-HOLD (Rosenberg — 1 RE), C6 (Feld), B6 (Chvátil), D1 D2-HOLD (K-K — 1 RE), A6 D3 (Stegmaier), each +0.5r. All adopted; preserved.

**Hold-explicit (7 across personas)**: C2 A1 (Knizia), C3 B3 (Rosenberg), C7 D2 (K-K).

**Silent-retire: 0 (12th consecutive).**

## Rubric version: v2.12 → v2.13

Minor bump. No protocol amendments (rate-limit not consumed). No new axis adoptions (C8 needs 2nd game).

## Innovations

### I-ts-01 — First non-Euro game in TIGRIS factory

- **Dimension:** bet validation
- **Trigger pattern:** `out_of_tradition_stress_test`
- **Source:** games/0015-twilight-struggle/
- **Supporting voices:** all 8 personas (7 retire-explicits absorbed without breaking axis definitions)
- **Scope:** pipeline-architectural
- **Status:** adopted (log entry)
- **Note:** First CDG / wargame / 2p-only game reviewed. Seven adopted axes retire-explicit (C4, C6, B6, D1, D3, A6, B3-hold) — meaning 7 of 23 adopted axes are Euro-specific enough that a 2p CDG doesn't exercise them. Pool still holds: 15 of 23 adopted axes earn or hold cleanly on a non-Euro game. Axis definitions survive out-of-tradition stress test.

### I-ts-02 — C8 First-Turn Compression canonical case found

- **Dimension:** Pool dynamics
- **Trigger pattern:** `dormant_axis_canonical_recovery`
- **Source:** TS Turn 1 + Turn 7 re-defenses by Stegmaier
- **Supporting voices:** Stegmaier (primary); Knizia/Feld/Lacerda/Rosenberg (collision vote)
- **Scope:** pool
- **Status:** candidate (1 earning; awaits 2nd game for adoption)
- **Note:** C8 waited 14 games before TS provided the canonical case. Parallels A4 Variance Calibration (dormant 11 → CoB earning) and A1 Elegance (dormant 8 → Famiglia). **Pattern: dormant axes don't retire automatically — they wait for canonical case.** Retirement-watch policy should extend dormancy tolerance accordingly.

### I-ts-03 — Non-own anchor (no canonical designer) validates anchor protocol

- **Dimension:** anchor protocol
- **Trigger pattern:** `non_own_anchor_review`
- **Source:** TS — Knizia anchors a Gupta/Matthews design
- **Supporting voices:** Knizia (anchor C1 earned 3 defends); protocol adherence observational
- **Scope:** universal
- **Status:** observational
- **Note:** First review where no panel designer authored the game. Anchor protocol works non-adjusted — Knizia's C1 Tension Budget earned canonical re-earn despite Gupta/Matthews having no roster representation. Suggests future non-Euro / non-canonical-designer games can slot into pipeline without persona changes.

### I-ts-04 — Two-collision session despite non-own anchor

- **Dimension:** argument protocol
- **Trigger pattern:** `collision_without_designer_pressure`
- **Source:** TS (C1↔A3 and C8↔A4 both narrated)
- **Supporting voices:** observational
- **Status:** observational
- **Note:** Contrast with Agricola (12 earnings, 0 formal collisions — canonical consensus). TS produced 2 clean collisions despite lacking on-own anchor tension. Suggests non-Euro games force disagreement because persona territory claims don't auto-align with game's identity.

## State after 15 games

- **23 adopted axes / 25 Pool (92%)** — unchanged from v2.12.
- C8 now **queued-for-adoption** (1/2 earnings). If earns in any next game → adoption.
- 1 retired (C5).
- 3 salvages (C4, C3, B2).
- **8/8 designer-on-own-anchor tally** still complete (TS is non-own, doesn't affect).
- **12 consecutive games 0% silent-retire.**
- First non-Euro absorbed cleanly. 7 retire-explicits on adopted axes (2p-only CDG stress).

## Success criteria

| # | Criterion | TS |
|---|---|---|
| 1 | ≥ 5 earned | PASS (12) |
| 2 | ≥ 2 on Band B/C | PASS (C1, B2, B1, B4, B5, C8) |
| 3 | ≥ 1 clean collision | **PASS (2)** |
| 4 | ≥ 1 adoption/retirement | PASS (C8 first-ever earning — retirement-watch → queued-for-adoption) |

4/4 PASS.

## Verdict

Twilight Struggle closes the C8 dormancy dossier (earning-side); adoption pending single 2nd-game earning. Factory successfully reviews a 2p non-Euro CDG wargame with no axis-definition strain. Rubric v2.13 stabilized at 23/25 adopted with C8 queued.

Proceeding to HANDOFF.
