---
name: CODEX v2 Amendments (TIER-C on v0.5)
slug: 0151-codex-amendments-v2
game: 0151-codex
stage: tierC
version: 2.0.0
rubric_version: v2.24.82
bet_version: parliament
author: TIGRIS
created: 2026-04-22
updated: 2026-04-22
review_type: delta-amendment
prior_amendments: amendments.md (v0.1)
---

# CODEX v2 Amendments — Deterministic Pass on v0.5 Argument Record

Inputs: `tierA-stakes-v2.md` + `playtests/PT15-v05-argument.md`. Outputs: per-axis state classification + Rubric Ledger updates.

## Per-stake classification (24 stakes)

| # | Persona | Axis | v0.1 state | v0.5 state | Amendment contribution |
|---|---|---|---|---|---|
| 1 | Feld | B1 | earned | **earned** | +1E toward B1 maintenance |
| 2 | Lacerda | D4 | earned | **earned** | +1E toward D4 maintenance |
| 3 | Vaccarino | A7 | earned | **earned** | +1E toward A7 maintenance |
| 4 | Rosenberg | C4 | earned | **earned** | +1E toward C4 maintenance |
| 5 | K-K | C7 | contested | **earned** (resolves) | +1E toward C7 maintenance; clears v1 contested |
| 6 | Stegmaier | A6 | contested | **earned** (resolves) | +1E toward A6 maintenance; clears v1 contested |
| 7 | Chvátil | B2 | earned | **earned** | +1E toward B2 maintenance (softened score) |
| 8 | Knizia | A1 | hold-explicit | **earned** (resolves) | +1E toward A1 maintenance; hold-explicit clears via M6 CR 5-2 |
| 9 | Knizia | C1 | earned | **earned** | +1E toward C1 maintenance |
| 10 | Chvátil | B5 | earned | **earned** | +1E toward B5 maintenance |
| 11 | Stegmaier | C8 | earned | **earned** | +1E toward C8 maintenance |
| 12 | K-K | D2 | earned | **earned** | +1E toward D2 maintenance |
| 13 | Rosenberg | A2 | earned | **earned** | +1E toward A2 maintenance |
| 14 | Vaccarino | A4 | earned | **earned** | +1E toward A4 maintenance |
| 15 | Lacerda | A28 | earned | **earned** | +1E toward A28 maintenance |
| 16 | Feld | C6 | earned | **earned** | +1E toward C6 maintenance |
| 17 | Feld | A5 | contested | **earned** (resolves) | +1E toward A5 maintenance; clears v1 contested |
| 18 | Lacerda | B4 | retire-explicit | **retire-explicit-confirmed** | +0.5R (inapplicability confirmed) |
| 19 | Vaccarino | B6 | not-staked | **earned** (new) | +1E toward B6 maintenance; first CODEX earn |
| 20 | Rosenberg | C3 | earned | **earned** | +1E toward C3 maintenance |
| 21 | K-K | A1 (collision loss) | — | **refute-single** (collision loser) | +1R-moment on A1 (but Knizia defends, so no net refute) |
| 22 | Stegmaier | C7 (collision redundant) | — | **earned** (same axis as #5) | absorbed into #5 |
| 23 | Chvátil | A3 | earned | **earned** | +1E toward A3 maintenance |
| 24 | Knizia | C2 (refute-reconfirm) | refuted | **refute-confirmed** | +1R formal (Knizia-primary self-refute) |

**Summary:** 21 earned, 1 refute-confirmed (C2), 1 retire-explicit-confirmed (B4), 1 collision-loss (K-K A1 counter — absorbed into the A1 collision resolution, not a standalone refute).

## Collisions resolved

1. **M2 B1 Feld ↔ C4 Rosenberg Decisive OP 7-1** — 124th total-pool TP (temporal-register OP); second B1↔C4 instance (first at Parliament v1). Both earn defend credit.
2. **M3 A5 Feld ↔ A6 Stegmaier Standard OP 6-2** — 125th TP; new pair at CODEX v0.5 (downtime-pacing vs teachability resolved as orthogonal registers). Both earn defend credit.
3. **M6 A1 Knizia ↔ A1 K-K CR 5-2 for Knizia** — **first intra-axis collision in TIGRIS corpus** (both personas staked the same axis with opposite verdicts). Knizia earns; K-K takes single-moment refutation on the attack stake but not on A1 the axis.

## Rubric Ledger updates (axis-pool.md)

Based on 21 earned stakes at CODEX v0.5 (2nd review of the same game — these are *maintenance earns* toward existing adoption counts, not first-earns):

| Axis | v0.5 earn count increment | New running total (pre-batch: 150-game corpus) |
|---|---|---|
| A1 Elegance | +1E | 72E → 73E (career high maintained) |
| A2 Decision Density | +1E | 84E → 85E (career high) |
| A3 Interaction | +1E | 88E → 89E (career high) |
| A4 Variance Calibration | +1E | 46E → 47E |
| A5 Downtime/Pacing | +1E | 37E → 38E |
| A6 Teachability | +1E | 99E → 100E (**first axis to reach 100E** — post-D3's 104E) |
| A7 Emergence/Replayability | +1E | 49E → 50E (milestone: 50 career earns) |
| A28 Cognitive Load Profile | +1E | 12E → 13E |
| B1 System Gearing | +1E | 54E → 55E |
| B2 Catastrophe Pressure | +1E | 35E → 36E |
| B5 Architectural Novelty | +1E | 19E → 20E (milestone: 20 career earns) |
| B6 Scoring Multiplier Dep. | +1E | 19E → 20E (milestone: 20 career earns; CODEX is 20th B6 earn) |
| C1 Tension Budget | +1E | 85E → 86E (career high) |
| C3 Scarcity Bite | +1E | 33E → 34E |
| C4 Engine-Garden Dependency | +1E | 21E → 22E |
| C6 Point-Salad Incommensurability | +1E | 32E → 33E |
| C7 Action-Menu Clarity | +1E | 80E → 81E |
| C8 First-Turn Compression | +1E | 12E → 13E |
| D2 Spatial-Interaction Presence | +1E | 31E → 32E |
| D4 Late-Game Lock-in Point | +1E | 46E → 47E |
| C2 Minimum-Score Shape | +1R formal | 11E/26R → 11E/27R (**monitoring escalated; highest retire-explicit AND highest formal-refute**) |
| B4 Info Transparency Cost | +0.5R (retire-explicit) | 31E/11.5R → 31E/12R |

## Adoption / retirement / dormancy-watch triggers

**All 21 axes earning here are already `adopted`.** No first-earns → no new adoptions triggered.

**C2 Monitoring:**
- Pre-batch: 11E/26R (highest formal-refute count in pool; Knizia-primary maintains).
- v0.5: +1R → 11E/27R.
- Still `adopted (monitoring — high retire-explicit)`.
- Not dormancy-watch yet — C2 *earns* periodically (11 career earns) so the axis remains active despite high refute count. A C2-refuted Knizia review is a known-shape pattern; no protocol change triggered.

**B4 Retire-explicit accumulation:**
- 31E/11.5R → 31E/12R.
- Not dormancy-watch — B4 has earned at Kanban EV, Vinhos, Whitechapel, Weather Machine, On Mars, etc. recent-adjacent. Active axis.

**A1 hold-explicit resolution:**
- v0.1 Knizia held A1 hold-explicit. Under v2.1 stake states, hold-explicit is neutral (+0 to adoption/retirement).
- v0.5 resolved A1 to earned.
- **The v0.1 hold-explicit stake is retroactively recorded as a *paid* prestige token — the token was spent at v0.1 and the persona vindicated at v0.5.** No ledger retro-edit; the v0.1 entry stays hold-explicit. v0.5 entry is earned.

## Rubric version bump

No axis crossed an adoption/retirement threshold at CODEX v0.5. No protocol amendment triggered.

**Proposed bump:** v2.24.82 → **v2.24.83** (patch-level bump for delta-review on CODEX).

Rationale for the patch bump (not a minor bump):
- No new axis adopted, no axis retired.
- No new protocol amendment (no new collision type, no new stake state).
- The **first intra-axis collision** (M6 A1 Knizia ↔ A1 K-K) is a precedent but doesn't require protocol change — current CR rules handle it cleanly.
- CODEX v0.5 becomes the **first TIGRIS game with two complete Parliament reviews** (v1 + v2). This is a protocol precedent for *delta review* as a review-mode but doesn't require amendment — the procedure simply ran.

**Changelog addition (append to axis-pool.md):**
```
- **v2.24.83** — 2026-04-22 — **CODEX delta-review v0.5 (game #151, 2nd review).** First TIGRIS double-reviewed game; first intra-axis collision (A1 Knizia↔K-K CR 5-2 for Knizia). 21 earned (all maintenance), 3 v1-contested resolved (A1 hold-explicit→earned, A6 contested→earned, C7 contested→earned, A5 contested→earned = 4 v1-contested resolved). A6 reaches 100E (2nd axis to 100E after D3). C2 +1R formal (Knizia re-confirm; 27R career high). B4 +0.5R retire-explicit-confirmed. B6 20th earn milestone. B5 20th earn milestone. A7 50E milestone.
```

## Innovations logged (append to playtest-innovations.md)

**I-codex-v2-01: Intra-axis adversarial collision.** Two personas staking the same axis with opposite verdicts can resolve via CR; the attacker's stake counts as refute-moment on the defender's position but is absorbed into the collision, not scored as a standalone refute against the axis. Precedent: M6 A1 Knizia ↔ A1 K-K. Trigger pattern: same-axis re-stake during a delta review where a v1 hold-explicit must resolve.

**I-codex-v2-02: Delta-review procedure.** Second Parliament on a game that iterated through multiple versions. Personas argue deltas, not whole-game. Evidence base is prior playtests + rule-change diff. Lower draft overhead (4 v1-contested resolved in 1 round), higher collision yield per stake. Suggests `review_type: delta-review` frontmatter + halved-prestige-cost for re-staking previously-earned axes.

**I-codex-v2-03: Hold-explicit-resolution via collision.** A v1 hold-explicit stake can resolve at v2 either by (a) direct defense + no attack, (b) direct attack → defense earning (CR collision outcome), or (c) direct attack → refute earning. CODEX v0.5 demonstrates (b). This is the first hold-explicit-to-earned transition in TIGRIS corpus — previously hold-explicit stakes expired unresolved.

## Flags / follow-up

- **C2 monitoring escalated** (27R career). C2 is now ~2.5× any other axis's refute count. Dormancy-watch not triggered (still earning), but the axis has become functionally diagnostic — the C2 refute *is* the Knizia-signature verdict for games that reward specialization. Consider protocol amendment to recognize "diagnostic-refute axes" that accumulate high R without triggering retirement because the refute itself is load-bearing.

- **Intra-axis collision precedent.** First instance. If this pattern recurs in future delta reviews, consider formalizing as a named collision type (IAC = intra-axis collision). Currently absorbed into CR.

- **CODEX v0.5 balance concerns (from playtest evidence, not argument):**
  1. Tide's TM tokens are strippable jackpots (PT14). Option: cap TM banked tokens at 5.
  2. Flame's SE rarely triggers in corner slates (PT13, PT14). Option: broaden SE trigger.
  3. Iron's VP is 100% mana (PT11). Option: mana-to-VP step function.
  4. Earth's end-game scoring doctrines are Edict-proof against token strip (PT13). Option: Edict also blocks future VP from end-game doctrines.

  These are *design* flags, not *rubric* flags. CODEX design team owns them. Parliament v2 does not adjudicate design fixes — only rubric state.

## Anchor completion

Knizia anchored A1 at v0.5. Outcome: A1 earned via M6 CR 5-2 collision win. **First successful anchor recovery from v1 hold-explicit in TIGRIS corpus.** Knizia's v1 prestige token is redeemed at v0.5. Protocol note: this validates the hold-explicit-as-reserved-prestige mechanism from v2.1 FORCED-ENGAGEMENT — holds *can* be defended at a future review, not just expire.

## Amendment pass status: COMPLETE

21 earned, 0 new adoptions, 0 retirements. 4 v1-contested resolutions (A1, A5, A6, C7). 3 collisions. 1 formal refute (C2). 1 retire-explicit confirmed (B4). Rubric version → v2.24.83.
