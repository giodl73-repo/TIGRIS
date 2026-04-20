---
name: Tikal — Tier-C Amendments (game-local)
slug: tikal-tierC-amendments
game: 0013-tikal
stage: tierC
version: 1.0.0
rubric_version: v2.10
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
note: Game-local amendment record. Shared-state integration (axis-pool ledger, rubric version bump, innovations log, TRACKER) handled by user out-of-band.
---

# Tier-C Amendment — Tikal (game #13)

## Scope

This file records the amendment outcome **for Tikal only**. Shared files (`personas/axis-pool.md`, `personas/playtest-rubric.md`, `personas/playtest-innovations.md`, `TRACKER.md`) are **not modified** by this review — user handles integration across parallel reviews (games #11, #12, #13).

## Headline

**Double-canonical K-K-on-K-K anchor. 2 new adoptions (D3, A5). 17 earned stakes (highest in TIGRIS history). 10th consecutive 0 silent-retire.**

Pool progresses toward stabilization. Tikal's AP-menu + spatial-play is the named canonical case for two adopted axes simultaneously (C7 + D2).

## Stake classification

**Earned (17)**:
- C7 (anchor; adopted re-earn + collision win)
- D2 (adopted re-earn + collision win)
- D1 (adopted re-earn — K-K signature)
- C1 / C3 / C4 / D4 / B1 / A7 / B6 / A2 / A6 (8 adopted re-earns)
- A3 (contested — collision loss refute, still earns via defense)
- A1 (contested — collision loss refute, still earns via defense)
- **D3 Count-Robustness (queued-for-adoption → ADOPTED)** — Scythe + Tikal = 2 earnings across 2 games
- A4 Variance Calibration — first meaningful earning (Scythe was weak-defend only)
- **A5 Pacing (queued-for-adoption → ADOPTED)** — Famiglia + Tikal (LAST-CALL) = 2 earnings across 2 games

**Retire-explicit (5 on adopted axes; +0.5 refute each; adopted status preserved)**:
- C2 Min-Score Shape — Tikal uses commensurable raw-sum VP
- C6 Point-Salad — Tikal VP sources sum directly
- B2 Catastrophe Pressure — no reversal events in Tikal
- B4 Info-Transparency-Cost — zero hidden info
- B3 Conversion Chain Depth — Tikal VP is flat

**Hold-explicit (3)**: A1 (also contested-earned), B5, C8.

**Silent-retire: 0** (10th consecutive game sustained).

## Adoption triggers — 2 new

| Axis | Prior earning | Tikal | Adoption state |
|---|---|---|---|
| **D3 Count-Robustness** | Scythe earned (Stegmaier anchor) | Turn 10 defend + 3p canonical | **ADOPTION-READY** (2 earnings across 2 games) |
| **A5 Downtime/Pacing** | Famiglia earned (Vaccarino LAST-CALL) | Endgame LAST-CALL | **ADOPTION-READY** (2 earnings across 2 games) |

Both axes cross the adoption threshold on Tikal. Shared-ledger integration pending (handled by user to coordinate with parallel reviews).

## Retirement: 0

No axis crosses retirement threshold. 5 retire-explicits on adopted axes each contribute +0.5 refute but do not threaten adopted status (adoption doesn't un-adopt via one game's retire-explicits).

Retire-explicit pattern confirms diagnostic integrity: Tikal's design space doesn't exercise min-score / point-salad / catastrophe / info-asymmetry / conversion-chain — axes measuring those things correctly report low, confirming the axes measure distinct properties.

## Rubric version bump

Tikal-internal record: v2.10 → v2.11 proposed (2 new adoptions). Actual version advancement handled by user out-of-band (parallel reviews may advance version more). Pool count post-Tikal: **22 adopted of 25 (88%)** if D3 + A5 integrate cleanly.

## Innovations — game-local log

### I-tikal-01 — First K-K-on-K-K anchor; double-canonical descriptor validation

- **Trigger:** `first_designer_on_own_anchor` + `double_canonical_anchor`
- **Source:** K-K anchor C7; Tikal named in C7 anchor-10 descriptor. Additionally, K-K drafts D2; Tikal named in D2 anchor-10 descriptor. **Two adopted axes have anchor-10 descriptors pointing at this exact game.**
- **Status:** adopted (log entry — factory-local)
- **Note:** Calibration success for both C7 and D2. The axis-pool descriptors were written correctly — Tikal scores the advertised 10/10 on both. Validates the anchor-descriptor system as evidence-grounded. Pattern parallels: Knizia-T&E (C2 anchor), Feld-FACETS (C6 anchor), Stegmaier-Scythe (C8 anchor), Chvátil-UNFOLD/TtA (B5 anchor), **K-K-Tikal (C7 + D2 anchor)**. First dual-descriptor anchor in TIGRIS history.

### I-tikal-02 — D3 Count-Robustness adoption

- **Trigger:** `stegmaier_signature_adopts_from_queue`
- **Source:** Scythe first earning (Stegmaier anchor, v2.5) + Tikal second earning (cross-anchor defend + 3p canonical).
- **Status:** adopted (pending shared-state integration)
- **Note:** D3 was queued-for-adoption since Scythe (game #5). 7 games elapsed between first and second earning — Tikal is the first game since Scythe to tune 2-4p tightly enough to defend D3. Stegmaier's signature axis validates.

### I-tikal-03 — A5 Downtime/Pacing adoption

- **Trigger:** `vaccarino_second_signature_adopts_from_queue`
- **Source:** Famiglia first earning (Vaccarino LAST-CALL, 2p alternating-turn) + Tikal second earning (4p LAST-CALL + 3p parallel).
- **Status:** adopted (pending shared-state integration)
- **Note:** A5 measures opposite cases — Famiglia's zero-downtime 2p alternating-turn shape AND Tikal's moderate-downtime 4p AP-planning. Axis has demonstrated range across 2 games → adopted. Vaccarino now has B3 (adopted TtA) + A5 (adopted here) as adopted signatures.

### I-tikal-04 — Double collision win for anchor persona on canonical moments

- **Trigger:** `anchor_descriptor_priority_vote_pattern`
- **Source:** K-K won A1 vs C7 collision (T1) and A3 vs D2 collision (T5). Both collision votes cited "anchor-descriptor priority" as the tiebreaker reasoning.
- **Status:** observation (no rule change)
- **Note:** Panel voting appears to favor axes whose anchor-10 descriptor explicitly names the game under review. Observational pattern — could codify in future chart amendment if repeated in Torres/Azul/Mexica reviews. Not bet-altering.

## Retire-explicit accounting (adopted-axis refute weight)

| Axis | Refute increment (Tikal) | Source |
|---|---:|---|
| C2 Min-Score Shape | +0.5 | Commensurable VP |
| C6 Point-Salad | +0.5 | Commensurable VP |
| B2 Catastrophe Pressure | +0.5 | No reversal |
| B4 Info-Transparency | +0.5 | No hidden info |
| B3 Conversion Chain | +0.5 | Flat VP |

Total adopted-axis refute weight added by Tikal: **+2.5**. Each adopted axis preserves status (adoption ≠ reversible via single-game retire-explicits).

## State after Tikal (local record)

- **22 adopted axes / 25 Pool (88%)** — pending shared-state integration
- 1 retired (C5; unchanged)
- 0 new retirements
- 10 consecutive games 0% silent-retire (v2.1 fix stable)
- Tikal earned stakes: **17** (TIGRIS record)
- Dual-canonical anchor validation (C7 + D2)

## Designer-on-own-game pattern — updated

| Persona | Own-anchor game | Game # | Status |
|---|---|---|---|
| Knizia | Tigris & Euphrates | #2 | ✓ |
| Feld | FACETS (original) | #3 | ✓ |
| Stegmaier | Scythe | #5 | ✓ |
| Chvátil | UNFOLD (original) + TtA | #7, #8 | ✓ |
| **K-K** | **Tikal** | **#13** | **✓ (this review)** |

Pending: Rosenberg (Agricola queued), Lacerda (Lisboa / Kanban EV in parallel #11), Vaccarino published (Kingdom Builder).

## Handoff note

This amendment record is **game-local only**. User integrates D3/A5 adoptions, refute-weight accumulations, innovation log entries, and rubric version bumps across parallel reviews (Lisboa #11, Castles of Burgundy #12, Tikal #13) to produce a single coherent shared-state update.

Proceeding to HANDOFF.
