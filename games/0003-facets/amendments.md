---
name: FACETS — Tier-C Amendments
slug: facets-tierC-amendments
game: 0003-facets
stage: tierC
version: 1.0.0
rubric_version: v2.2.1
bet_version: parliament
author: TIGRIS (via /tigris-amendment skill procedure)
created: 2026-04-19
updated: 2026-04-19
skill_invocation: "/tigris-amendment on games/0003-facets/"
---

# Tier-C Amendment — FACETS (game #3)

First amendment pass authored via the new `/tigris-amendment` skill. Output is deterministic given the stakes + argument record; no creative generation applied.

## Stake classification (per v2.1 rules)

| Persona | Axis | Defend | Refute | Collision-win | Collision-loss | FORCED-ENGAGEMENT | State |
|---|---|---:|---:|---:|---:|---|---|
| Feld | C6 Point-Salad (anchor) | 2 | 0 | 1 | 0 | — | **earned** |
| Feld | C5 Anti-Catch-up | 0 | 0 | 0 | 1 | — | contested (weak) |
| Feld | A2 Decision Density | 1 | 0 | 0 | 1 | LAST-CALL success | **earned** (via LAST-CALL) |
| Stegmaier | D3 Count-Robustness | 1 | 0 | 0 | 0 | hold-explicit | weak-defended |
| Stegmaier | C8 First-Turn Compression | 0 | 0 | 0 | 0 | hold-explicit | hold-explicit |
| Stegmaier | A6 Teachability | 1 | 0 | 1 | 1 | — | **earned** (2 marks net) |
| K-K | C7 Action-Menu Clarity | 1 | 0 | 0 | 0 | hold-explicit | weak-defended |
| K-K | D1 Family-to-Expert | 0 | 0 | 0 | 0 | hold-explicit | hold-explicit |
| K-K | D2 Spatial-Interaction | 0 | 0.5 | 0 | 0 | retire-explicit | retire-explicit |
| Rosenberg | C3 Scarcity Bite | 1 | 0 | 0 | 0 | — | weak-defended |
| Rosenberg | C4 Engine-Garden | 0 | 0.5 | 0 | 0 | retire-explicit | retire-explicit |
| Rosenberg | D4 Late-Game Lock-in | 1 | 0 | 1 | 0 | — | **earned** |
| Knizia | C1 Tension Budget | 0 | 0 | 0 | 0 | hold-explicit | hold-explicit |
| Knizia | C2 Min-Score Shape (ADOPTED) | 3 | 0 | 0 | 0 | — | **earned** (diagnostic-low stake earns defends anyway) |
| Knizia | A1 Elegance | 0 | 0 | 0 | 0 | hold-explicit | hold-explicit |
| Lacerda | B1 System Gearing | 0 | 0.5 | 0 | 0 | retire-explicit | retire-explicit |
| Lacerda | B4 Info-Transparency-Cost (ADOPTED) | 0 | 0 | 0 | 0 | hold-explicit | hold-explicit |
| Lacerda | A7 Emergence-Replayability | 0 | 0 | 0 | 0 | hold-explicit | hold-explicit |
| Chvátil | B5 Architectural Novelty | 0 | 0 | 0 | 0 | hold-explicit | hold-explicit |
| Chvátil | A3 Interaction | 0 | 1 | 0 | 0 | — | **refuted (self)** |
| Chvátil | B2 Catastrophe Pressure | 0 | 0.5 | 0 | 0 | retire-explicit | retire-explicit |

**Summary counts**:
- Earned: **5** (C6, A2, A6, D4, C2)
- Refuted (full): 1 (Chvátil A3 self-refuted)
- Retire-explicit: 4 (D2, C4, B1, B2)
- Hold-explicit: 7
- Weak-defended: 3 (D3, C7, C3)
- Contested (weak): 1 (C5)
- **Silent-retire: 0** — v2.1 FORCED-ENGAGEMENT holds for second consecutive game

## Rubric Ledger — cumulative after 3 games

Updated from `personas/axis-pool.md` after applying FACETS game evidence. Entries shown with per-game earnings/refutations cumulated.

| Axis | Earned (cum) | Refuted (cum) | Contested (cum) | Status | Trajectory |
|---|---:|---:|---:|---|---|
| A1 Elegance | 0 | 0 | 1 | live | P contested → T&E hold → FACETS hold |
| A2 Decision Density | 1 | 0 | 0 | queued-for-adoption | P ignored → T&E hold → FACETS earned |
| A3 Interaction | 0 | 1 | 1 | live (contested cross-game) | P ignored → T&E contested → FACETS self-refuted |
| A4 Variance Calibration | 0 | 0 | 0 | live (never drafted) | undrafted 3× — I-facets-03 candidate |
| A5 Downtime-Pacing | 0 | 0 | 0 | live (never drafted) | undrafted 3× — I-facets-03 candidate |
| A6 Teachability | 1 | 0 | 1 | queued-for-adoption | P contested → T&E hold → FACETS earned |
| A7 Emergence | 0 | 0 | 1 | live | P contested → T&E hold → FACETS hold |
| B1 System Gearing | 1 | 0.5 | 0 | queued-for-adoption (mixed) | T&E earned; FACETS retire-explicit |
| B2 Catastrophe Pressure | 1 | 0.5 | 0 | queued-for-adoption (mixed) | T&E earned; FACETS retire-explicit |
| B3 Conversion Chain Depth | 0 | 0 | 0 | live (never drafted) | undrafted 3× — I-facets-03 candidate |
| B4 Info-Transparency-Cost | 2 | 0 | 0 | **ADOPTED** | v2.2 event; still earning (1 hold-explicit in FACETS) |
| B5 Architectural Novelty | 1 | 0.5 | 0 | live | P earned → T&E retire-explicit → FACETS hold |
| C1 Tension Budget | 0 | 0 | 0 | live | ignored/weak/hold — 0 active engagement in any game |
| C2 Min-Score Shape | 3 | 0 | 0 | **ADOPTED** | All 3 games earned (P 3 defends, T&E LAST-CALL, FACETS 3 defends diagnostic-low) |
| C3 Scarcity Bite | 0 | 1 | 0 | queued-for-retirement | T&E refuted; FACETS weak-defended |
| C4 Engine-Garden Dependency | 0 | 1.5 | 1 | queued-for-retirement | P refuted + T&E contested + FACETS retire-explicit — close to retirement threshold |
| C5 Anti-Catch-up Pressure | 0 | 1 | 2 | queued-for-retirement | P self-refuted; T&E+FACETS contested via collision losses |
| C6 Point-Salad Incommensurability | 1 | 0.5 | 0 | queued-for-adoption | T&E retire-explicit; FACETS earned (vindicated anchor after T&E retreat) |
| C7 Action-Menu Clarity | 0 | 0 | 0 | live | Weak across 3 |
| C8 First-Turn Compression | 0 | 0.5 | 0 | live | T&E retire-explicit; FACETS hold |
| D1 Family-to-Expert Scaling | 0 | 0 | 0 | live | Hold across 3 |
| D2 Spatial-Interaction Presence | 1 | 0.5 | 0 | queued-for-adoption (mixed) | T&E earned; FACETS retire-explicit |
| D3 Count-Robustness | 0 | 0 | 0 | live | Weak/hold across 3; first active defend in FACETS |
| D4 Late-Game Lock-in Point | 1 | 0 | 0 | queued-for-adoption | FACETS earned (first real defense); one more earning triggers |

## Promotion / retirement triggered

### ADOPTED (this session): 0

No new adoptions. Three candidates (A6, D4, A2) have 1 earning and await second. Four candidates (B1, B2, D2, C6) have mixed earned-with-retire-explicit patterns.

### RETIRED (this session): 0

No retirements. Retirement protocol (conservative) holds:
- C4 closest at 1.5 refutes (needs 2).
- C3 at 1 refute.
- C5 at 1 refute + 2 contested.

One more unambiguous refutation in any future game would retire C4.

### Ledger status changes

| Axis | Previous | New | Reason |
|---|---|---|---|
| A6 Teachability | live | queued-for-adoption | First earning in FACETS |
| A2 Decision Density | live | queued-for-adoption | First earning in FACETS |
| D4 Late-Game Lock-in | live | queued-for-adoption | First earning in FACETS |
| C6 Point-Salad | live | queued-for-adoption | First earning (Feld anchor vindication) |
| B1 System Gearing | queued-for-adoption | queued-for-adoption (stalled) | Earned T&E but retire-explicit FACETS |
| B2 Catastrophe Pressure | queued-for-adoption | queued-for-adoption (stalled) | Earned T&E but retire-explicit FACETS |
| D2 Spatial-Interaction | queued-for-adoption | queued-for-adoption (stalled) | Earned T&E but retire-explicit FACETS |
| C4 Engine-Garden | queued-for-retirement | queued-for-retirement (1.5 refutes) | Cumulative weight moves closer to retirement |

## Rubric version bump

No adoptions, no retirements. Per `/tigris-amendment` skill: "If no adoption/retirement: bump patch version."

**v2.2 → v2.2.1.** Ledger-update-only bump. Rubric structure unchanged.

Next triggering event (likely game #4):
- If game #4 earns A6 or D4 a second time → adoption event → v2.3.
- If game #4 refutes C4 → retirement event → v2.3.
- If the user overrides rate limit to adopt I-parliament-03 (anchor-adjacency, now 3 cluster instances) → v2.3.

## Silent-retire count

**0** (second consecutive game at 0). The v2.1 FORCED-ENGAGEMENT fix continues to hold. Sample size: Parliament (48% → 0% after fix) + T&E (0%) + FACETS (0%) = 3 games, fix stable.

I-te-03 (FORCED-ENGAGEMENT validation) status upgraded from "1-game log entry" to "3-game validation": the fix is robust.

## New innovations logged

### I-facets-01 — New adjacency: C6 Point-Salad ↔ A6 Teachability

- **Dimension:** adjacency chart
- **Trigger pattern:** `new_adjacency_surfaced`
- **Source:** FACETS Turn 8 collision
- **Supporting voices:** Feld (C6 stake-holder), Stegmaier (A6 stake-holder), plus non-colliding voters who validated the semantic adjacency
- **Scope:** universal
- **Status:** candidate (high-adoption likelihood, low-risk patch)
- **Note:** Incommensurable scoring requires teachable rules — the two axes are semantically linked. Proposal: add C6↔A6 to Parliament design.md §9 adjacency chart.

### I-facets-02 — New adjacency: A2 Decision Density ↔ A6 Teachability

- **Dimension:** adjacency chart
- **Trigger pattern:** `new_adjacency_surfaced`
- **Source:** FACETS Turn 4 collision
- **Supporting voices:** Feld (A2), Stegmaier (A6)
- **Scope:** universal
- **Status:** candidate
- **Note:** Dense decisions require clear rules; inverse also true. Proposal: add A2↔A6 to adjacency chart.

### I-facets-03 — Systematically undrafted axes (A4, A5, B3)

- **Dimension:** pool curation
- **Trigger pattern:** `axes_never_drafted`
- **Source:** 3-game draft history
- **Supporting voices:** observational
- **Scope:** universal
- **Status:** candidate
- **Note:** A4 Variance Calibration, A5 Downtime-Pacing, and B3 Conversion Chain Depth have NEVER been drafted across Parliament + T&E + FACETS despite being in the Pool. Either:
  1. Retire them from the Pool (they have no advocate).
  2. Reassign their advocacy to a persona whose preferences prioritize them (requires persona edit).
  3. Add a new persona (e.g., "The Referee" or "The Tournament Player") whose preferences include A4/A5/B3.
  Proposal: Option 3 would expand the panel and give these axes advocates. Option 1 is simpler but thins the rubric.

### I-facets-04 — Self-refutation validates factory self-correction

- **Dimension:** bet validation
- **Trigger pattern:** `self_refutation_by_evidence`
- **Source:** Chvátil A3 Interaction self-refutation at Turn 8
- **Supporting voices:** Chvátil (self-refuter); I-parliament-04, I-te-03 context
- **Scope:** bet
- **Status:** adopted (log entry)
- **Note:** Second self-refutation in TIGRIS history (after Feld's C5 in Parliament). Personas updating stakes based on in-game evidence is the strongest single signal that the bet (adversarial, evidence-driven review) works. Feld and Chvátil both demonstrated this — the most voting-heavy personas are also the most willing to update. v2.0 bet continues to validate.

### I-te-02 cluster confirmation (identical drafts)

- **Status:** candidate → **proposed-amendment**
- **Note:** Third consecutive game with identical 21-axis drafts. Pattern is stable. Proposals:
  - Accept as feature (stable preferences = stable panel identity)
  - Force perturbation (shuffle preference order for one persona per game)
  - Context-sensitive preferences (Parliament-vs-T&E-vs-FACETS should preference differently)
  - User decision needed for adoption — this is a bet-level question, not a protocol patch.

### I-parliament-03 cluster confirmation (anchor-without-partner)

- **Status:** proposed-amendment (3 instances cluster) → **ready-for-adoption**
- **Note:** Feld's C6 anchor at FACETS joins Knizia's C2 at Parliament and T&E as the third instance. Rate-limit note: cycle 2 (FACETS + next game) has not yet triggered an adoption; I-parliament-03 is the prime candidate. Recommendation: adopt immediately before game #4, OR wait for game #4 to complete the cycle naturally.

## Success criteria (v2.1)

| # | Criterion | FACETS result |
|---|---|---|
| 1 | ≥ 5 earned stakes | **PASS (5)** |
| 2 | ≥ 2 earned on Band B or C | **PASS (2)** — C6, C2 |
| 3 | ≥ 1 clean collision | **PASS (3)** |
| 4 | ≥ 1 axis promoted or retired | **FAIL (0)** |

3 of 4 PASS. Criterion 4 fails this session because thresholds require second-game evidence and FACETS is the first earning for A6, A2, D4, C6. All four are queued for adoption at game #4. This is a transient failure — the rubric is actively moving but no axis crossed the threshold this round.

## Verdict

Three-game bet-validation sequence (Parliament → T&E → FACETS) shows:
- 0% silent-retire rate after v2.1 patch (sustained across 2 games)
- Adoptions firing (C2, B4 in T&E; A6, D4, A2, C6 queued for game #4)
- Retirement protocol correctly conservative (no premature retirements despite 6 refute events)
- Self-refutation pattern present across 2 personas (Feld, Chvátil) — the bet produces evidence that updates personas, not just scores games
- **Point-Salad Incommensurability vindicated** — Feld's anchor axis earned its first game after being retire-explicit in T&E. The axis survives; the design-specific application is what varies.

## Next step

HANDOFF. Game #4 should either:
- Trigger adoption of I-parliament-03 (anchor-adjacency) as the cycle-2 amendment.
- Be a game that naturally earns A6 or D4 a second time → axis-adoption event.
- Be a game where the panel faces a C4/C5 refutation decision → first retirement event.

Recommended: **Agricola** (Rosenberg-on-Rosenberg) — likely earns C3 Scarcity Bite and C4 Engine-Garden Dependency (both currently queued-for-retirement), providing reversal evidence against both and the test of the retirement protocol.
