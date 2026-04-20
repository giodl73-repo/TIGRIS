---
name: Through the Ages — Tier-C Amendments
slug: tta-tierC-amendments
game: 0008-through-the-ages
stage: tierC
version: 1.0.0
rubric_version: v2.8
bet_version: parliament
author: TIGRIS (via /tigris-amendment)
created: 2026-04-19
updated: 2026-04-19
---

# Tier-C Amendment — Through the Ages (game #8)

## Headline

**3 new adoptions + B2 salvage**. Pool reaches **18 adopted / 25 (72%)**. First Chvátil-on-Chvátil validates the anchor. B2 Catastrophe Pressure was at retirement threshold; instead it adopts. Retirement protocol works both directions.

## Stake classification

**Earned (12)**: B5 (adopted re-earn), **B2 (earned → ADOPTS)**, B1/A7/C3/C4/D4/C1/A2 (adopted re-earns), A3 (1st earn; queued), **B6 (earned → ADOPTS)**, **B3 (earned → ADOPTS)**.

**Retire-explicit (6)**: A5 (Vaccarino), D1 (K-K adopted), D2 (K-K adopted), A6 (Stegmaier adopted), C2 (Knizia adopted), C6 (Feld adopted).

**Hold-explicit (4)**: A4, D3, C8, A1, B4.

**Silent-retire: 0** (7th consecutive game).

## Adoption triggers — 3 new

| Axis | Prior earning | TtA | Adoption |
|---|---|---|---|
| **B2 Catastrophe Pressure** | T&E earned | 1 defend + 1 collision = earned | **ADOPTED** (salvaged from retirement) |
| **B6 Scoring Multiplier Dependency** | UNFOLD LAST-CALL | LAST-CALL earned | **ADOPTED** (new axis validated) |
| **B3 Conversion Chain Depth** | PR earned | LAST-CALL earned | **ADOPTED** (Vaccarino's signature) |

**Pool now: 18 adopted of 25 (72%).**

## Retirement: 0

B2 was at retirement threshold (~2.5 refute weight) before TtA. TtA's earning reverses the trajectory permanently — B2 joins C4 (Dominion) and C3 (UNFOLD) as rare retirement-reversal cases.

## Retire-explicit damage to adopted axes

A6 Teachability, D1 Family-to-Expert, D2 Spatial, C2 Min-Score, C6 Point-Salad all got retire-explicit in TtA (each +0.5 refute). Adopted status preserved; diagnostic-low stakes confirmed (TtA is heavy/single-track/etc.).

## Rubric version: v2.7 → v2.8

Major bump: 3 new axis adoptions + 1 retirement salvage.

## Innovations

### I-tta-01 — First Chvátil-on-Chvátil anchor

- **Trigger:** `first_designer_on_own_anchor`
- **Source:** TtA; Chvátil anchored B5 on his own design.
- **Status:** adopted (log entry)
- **Note:** Calibration success. Pattern parallels Knizia-T&E (#2), Stegmaier-Scythe (#5), Feld-FACETS (#3 original), now Chvátil-TtA (#8). Designer-on-own-game is strong validation — the anchor axis scores cleanly 10/10 on the designer's signature work.

### I-tta-02 — B2 Catastrophe Pressure: retirement salvage

- **Trigger:** `retirement_salvage_via_canonical_earning`
- **Source:** B2 accumulated 2.5 refute weight (T&E earning + 5 retire-explicits across FACETS/Dominion/Scythe/PR/UNFOLD). TtA earned B2 with collision win.
- **Status:** adopted (log entry)
- **Note:** Parallels C4 Engine-Garden (Dominion salvage) and C3 Scarcity Bite (UNFOLD salvage). Retirement queue is not fatal — canonical-case game earns an axis out of retirement trajectory. Protocol protects axes from premature cuts.

### I-tta-03 — B6 Scoring Multiplier Dependency adopted (full validation loop)

- **Trigger:** `new_axis_full_validation`
- **Source:** I-parliament-02 proposed axis → v2.6 added to Pool → UNFOLD first earning → TtA second earning → ADOPTED at v2.8.
- **Status:** adopted (log entry)
- **Note:** Complete lifecycle for a proposed axis: cluster evidence (3 instances) → Pool addition → 2 game earnings → adoption. The new-axis-proposal pipeline works end-to-end.

### I-tta-04 — B3 Conversion Chain Depth adopted

- **Trigger:** `vaccarino_signature_adopts`
- **Source:** Vaccarino's B3 earned in PR + TtA (LAST-CALL both times).
- **Status:** adopted (log entry)
- **Note:** B3 was in Vaccarino's preferred_axes at #4 but had zero earnings across games 1-5 (before Vaccarino joined). After Vaccarino joined v2.4, B3 earned its first-ever (PR) and second-ever (TtA), triggering adoption. Validates the v2.4 persona-expansion amendment.

## State after 8 games

- **18 adopted axes / 25 (72% of Pool)**
- 1 retired (C5; no new retirements)
- Pool unchanged size; composition stabilizing
- 5 protocol amendments unchanged
- 7 games 0% silent-retire sustained
- 8 games total, 63+ earned stakes, mean 7.9/game

## Anchor-on-own pattern

Completed designer-on-own-game anchor reviews after TtA:
- Knizia on T&E (game #2) ✓
- Feld on FACETS (game #3, original) ✓
- Stegmaier on Scythe (game #5) ✓
- Chvátil on UNFOLD (game #7, original) ✓
- **Chvátil on Through the Ages (game #8) ✓ — first Chvátil-on-published-own-game**

Still pending: Rosenberg (Agricola queued), Lacerda (Lisboa / Kanban EV queued), K-K (Tikal queued), Vaccarino (Dominion was review without Vaccarino anchor — could re-anchor on Kingdom Builder).

Proceeding to HANDOFF.
