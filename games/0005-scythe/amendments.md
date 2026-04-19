---
name: Scythe — Tier-C Amendments
slug: scythe-tierC-amendments
game: 0005-scythe
stage: tierC
version: 1.0.0
rubric_version: v2.5
bet_version: parliament
author: TIGRIS (via /tigris-amendment skill)
created: 2026-04-19
updated: 2026-04-19
---

# Tier-C Amendment — Scythe (game #5)

## Stake classification (v2.1 rules, 24 drafted stakes)

**Earned (9)**: D3 (Stegmaier anchor), D1 (K-K LAST-CALL), D2 (K-K), C4 (Rosenberg), D4 (Rosenberg; already adopted), A7 (Lacerda), B1 (Lacerda; already adopted), A2 (Knizia LAST-CALL; already adopted), B5 (Chvátil LAST-CALL; already adopted).

**Retire-explicit (3)**: C2 (Knizia; adopted axis but Scythe uses multiplier), C5 (Feld), B2 (Chvátil). Each +0.5 refute.

**Hold-explicit (9)**: C8, C7, C1, C6, A1, B3, C3, B4, A6.

**Weak-defended (3)**: A4 (Vaccarino 1d), A5 (Feld 1d), A3 (Chvátil 1 collision loss).

**Silent-retire: 0** (4th consecutive game).

## Rubric Ledger updates (cumulative after 5 games)

Key changes:

| Axis | Before Scythe | After Scythe | Change |
|---|---|---|---|
| A4 Variance Calibration | 0/0/0/0 never drafted | 0/0/0/0 weak-defended | First active engagement ever |
| A5 Downtime-Pacing | 0/0/0/0 never drafted | 0/0/0/0 weak-defended | First engagement ever |
| B3 Conversion Chain Depth | 0/0/0/0 never drafted | 0/0/0/0 hold-explicit | First engagement ever |
| A7 Emergence | 1/0/1 (Dominion earn) | **2/0/1 → ADOPTED** | 2nd earning |
| D1 Family-to-Expert | 1/0/0 (Dominion earn) | **2/0/0 → ADOPTED** | 2nd earning |
| D2 Spatial-Interaction | 1/1.0/0 (T&E earn + 2 retire-explicits) | **2/1.0/0 → ADOPTED** | 2nd earning |
| C4 Engine-Garden | 1/1.5/1 (reversed) | **2/1.5/1 → ADOPTED** | 2nd earning; retirement-queue permanently cleared |
| D3 Count-Robustness | 0/0/0 | 1/0/0 | First earning |

## Adoptions this session

**4 new adoptions:**
1. **A7 Emergence-Replayability** — Dominion earned + Scythe earned → 8th adopted axis
2. **D1 Family-to-Expert Scaling** — Dominion earned + Scythe earned (LAST-CALL) → 9th
3. **D2 Spatial-Interaction Presence** — T&E earned + Scythe earned → 10th
4. **C4 Engine-Garden Dependency** — Dominion earned + Scythe earned → 11th (retirement reversal fully confirmed)

**Rubric Pool now has 11 adopted axes out of 24 (46%).**

## Retirements this session

**Zero.** C2, C5, B2 each took 0.5 refute via retire-explicit but none hit the 2.0 cumulative threshold.

## Rubric version bump

**v2.4 → v2.5.** 4 adoptions.

## New innovations

### I-scythe-01 — Scoring Multiplier Dependency (I-parliament-02 second instance)

- **Dimension:** new axis candidate
- **Trigger pattern:** `scoring_multiplier_present`
- **Source:** Scythe end-game popularity-banded multiplier
- **Supporting voices:** observational (panel didn't explicitly collide on this; but the axis clearly applies — Parliament's multiplier + Scythe's multiplier = 2 instances)
- **Status:** candidate → **promoted** based on I-parliament-02's prior instance. **Cluster now at 2 genuine multiplier cases.** Eligible for cycle 4 adoption.

### I-scythe-02 — Pool saturation pattern

- **Dimension:** ledger dynamics
- **Trigger pattern:** `pool_fully_drafted`
- **Source:** Scythe draft record (24 drafts cover 24 axes)
- **Status:** observational
- **Note:** First game where every axis is advocated. With 8 personas × 3 = 24 drafts and 24-axis Pool, future games likely sustain full coverage. Pattern for future: when Pool expands (new axis adoptions or candidates), revisit draft count vs. Pool size.

### I-scythe-03 — A4/A5/B3 first engagement validation

- **Dimension:** bet validation (for I-facets-03 adoption)
- **Trigger pattern:** `resolved_cluster_produces_evidence`
- **Status:** adopted (log entry)
- **Note:** Vaccarino's addition (v2.4) immediately produced first engagements on A4 (drafted + defended), B3 (drafted + hold-explicit), A5 (drafted by Feld fallback + defended). The amendment worked as designed — covering previously-undrafted axes. Validates I-facets-03's resolution.

### I-scythe-04 — Adopted axes can re-earn

- **Dimension:** ledger dynamics
- **Trigger pattern:** `adopted_axis_reearns`
- **Status:** observational (log entry)
- **Note:** Adopted axes (B5, A2, B1, D4) continued to earn in Scythe. Adoption isn't terminal — axes keep accumulating evidence. This is correct protocol behavior. Future ledger analyses may want to distinguish "adopted + X re-earnings" vs "adopted with 0 re-earnings" to track which adoptions are holding vs stale.

## Cluster status updates

- **I-parliament-02 (Scoring Multiplier Dependency)** — 1 instance (Parliament) + 1 instance (Scythe I-scythe-01) = **2 cluster instances**. Proposed-amendment status. Eligible for cycle 4 adoption (games 6-7 cycle closing).
- **I-facets-03 + I-dominion-04 (Vaccarino persona)** — fully resolved; log entries closed.

## Success criteria

| # | Criterion | Scythe |
|---|---|---|
| 1 | ≥ 5 earned | **PASS (9)** |
| 2 | ≥ 2 earned on Band B or C | **PASS (5 — B5, B1, C4, A7 in bands; A2 adopted)** |
| 3 | ≥ 1 clean collision | **PASS (2 — A3↔D2, A7↔B5)** |
| 4 | ≥ 1 axis promoted/retired | **PASS (4 adoptions)** |

All 4 PASS.

## Verdict

Scythe is the second-strongest review to date (Dominion was 8 earned, 5 adoptions; Scythe is 9 earned, 4 adoptions). The 8-persona roster produces richer argument; the full-Pool coverage eliminates a class of undrafted failures.

## State after 5 games

- **11 adopted axes** out of 24 (46% of Pool)
- **0 retirements** (protocol conservative; 1 reversal; 3 queued)
- **4 protocol amendments** (v2.1 FORCED-ENGAGEMENT, v2.3 anchor-adjacency GATE, v2.3.1 adjacency chart patch, v2.4 Vaccarino persona)
- **0% silent-retire** across 4 games under v2.1
- **Mean earned per game**: (3+5+5+8+9)/5 = 6.0 stakes/game

## Next cycle (4)

Game #6 eligible for:
- I-parliament-02 adoption (Scoring Multiplier axis) — 2-instance cluster, cycle 4 candidate.
- Tracking A4, A5, B3 toward second earnings (they need to earn in another game to adopt).
- First retirement event? (C3, C5 still queued with 1-1.5 refute each; next game with active refutation could trigger first retirement.)

Proceeding to HANDOFF.
