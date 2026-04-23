---
name: CODEX — Amendments
slug: 0151-codex-amendments
game: 0151-codex
stage: tierC
version: 1.0.0
rubric_version: v2.24
bet_version: parliament
author: TIGRIS
created: 2026-04-22
updated: 2026-04-22
---

# Tier-C Amendments — CODEX (0151)

Deterministic ledger pass based on PT01 argument record.

## Per-axis delta for this game

| Axis | Final state | Delta | Notes |
|---|---|---|---|
| A1 Elegance | hold-explicit | 0E / 0R | Token paid; no play resolution. |
| A2 Decision Density | earned | +1E | Rosenberg defend × 2 at R3 + Unraveling. |
| A3 Interaction | earned | +1E | Chvátil Decisive OP + final defend. Edict phase load-bearing. |
| A4 Variance Calibration | earned | +1E | Vaccarino 3 defends across arc. |
| A5 Downtime/Pacing | contested | 0E / 0R | Feld mixed evidence (R3 cascade vs R5 Unraveling). |
| A6 Teachability | contested | 0E / 0R | Stegmaier hold-explicit; no play evidence either way. |
| A7 Emergence/Replayability | earned | +1E | Vaccarino arc-defend (500M configs). |
| A28 Cognitive Load Profile | earned | +1E | Lacerda hold-explicit converts via arc-defend (4 register stacking). |
| B1 System Gearing (ANCHOR) | **earned** | **+1E** | Feld anchor earn. OP at R1 collision. Multiple defends. |
| B2 Catastrophe Pressure | earned | +1E | Chvátil last-call defend at Doctrinal Schism R4. |
| B3 Conversion Chain Depth | retire-explicit | 0E / +0.5R | Vaccarino self-retire; 2-step chains insufficient. |
| B4 Information Transparency Cost | retire-explicit | 0E / +0.5R | Lacerda self-retire; CODEX refuses info asymmetry. |
| B5 Architectural Novelty | earned | +1E | Chvátil Convocation-as-law defended. |
| C1 Tension Budget | earned | +1E | Knizia storage-cap defend + CR-win on B2 collision. |
| C2 Minimum-Score Shape | **refuted** | 0E / +1R | Knizia last-call failed; specialization wins (P4 Earth). |
| C3 Scarcity Bite | earned | +1E | Rosenberg last-call defend at Doctrinal Schism + R4 bite. |
| C4 Engine-Garden Dependency | earned | +1E | Rosenberg garden gearing. OP with B1 at R1. |
| C6 Point-Salad Incommensurability | earned | +1E | Feld final scoring defend (6+ VP surfaces). |
| C7 Action-Menu Clarity | contested | 0E / 0R | K-K hold-explicit; Doctrine text parsing issue. |
| C8 First-Turn Compression | earned | +1E | Stegmaier retrospective defend (Convocation → arc cascade). |
| D1 Family-to-Expert Scaling | retire-explicit | 0E / +0.5R | K-K self-retire; CODEX declines the axis. |
| D2 Spatial-Interaction Presence | earned | +1E | K-K Decisive OP = 2 collision credits + multiple defends. |
| D3 Count-Robustness | retire-explicit | 0E / +0.5R | Stegmaier self-retire; 3-4p asymmetry didn't fire in this play. |
| D4 Late-Game Lock-in Point | earned | +1E | Lacerda R3 lock confirmed at R5. |

## Collisions resolved (4 events)

1. **R1 B1↔C4 (Feld vs. Rosenberg)** → OP 6-2; both earn defend.
2. **R2 B2↔C1 (Chvátil vs. Knizia)** → CR 5-3 for Knizia; Chvátil B2 takes single-moment refute (recovered via last-call at R4).
3. **R3 D2↔A3 (K-K vs. Chvátil)** → Decisive OP 7-1; both earn defend; counts as 2 collision credits.

Total collision credits: 4 (1 OP + 1 CR + 2 from Decisive OP).

## Summary counts

- **Earned: 15** (B1 anchor, C6, D4, A28, B5, B2, A3, C1, C3, C4, A2, D2, C8, A4, A7).
- **Refuted: 1** (C2).
- **Retire-explicit: 4** (D3, B3, D1, B4) → +0.5R each = +2.0 total retire-weight distributed.
- **Hold-explicit: 1** (A1).
- **Contested: 3** (A5, A6, C7).
- **Silent-retire: 0.** (FORCED-ENGAGEMENT prevented all silent-retires.)

## Success criteria

- ≥ 5 earned: **PASS (15).**
- ≥ 2 on Band B/C: **PASS (9 earned on Band B/C).**
- ≥ 1 clean collision: **PASS (4 collisions including Decisive OP).**
- ≥ 1 adoption/retirement trigger: pool-level thresholds track across games; C2 +1R continues retirement trajectory (currently at 11E/26R with retire-explicit high). This game contributes +1.0 formal refute weight to C2. No single-game adoption triggers fire at this game alone.

## Ledger delta for axis-pool.md

Per-axis cumulative update (apply to ledger):

- A2 +1E, A3 +1E, A4 +1E, A7 +1E, A28 +1E
- B1 +1E, B2 +1E, B3 +0.5R, B4 +0.5R, B5 +1E
- C1 +1E, C2 +1R, C3 +1E, C4 +1E, C6 +1E, C7 contested, C8 +1E
- D1 +0.5R, D2 +1E, D3 +0.5R, D4 +1E

## Rubric version

No new protocol amendments this game. Rubric bump: v2.24 → **v2.24.82** (patch; game-record update).

## Anchor performance

**Feld on B1 anchored successfully.** 3 defends across arc (R1, R3, Unraveling) plus an OP credit. The anchor-adjacency partners drafted (B5 Chvátil, C4 Rosenberg, D4 Lacerda) all earned — the anchor neighborhood functioned as designed.
