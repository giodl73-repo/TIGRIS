---
name: Parliament — Tier-C Amendments
slug: parliament-tierC-amendments
game: 0001-parliament
stage: tierC
version: 1.0.0
rubric_version: v2.0
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# Tier-C Amendment — Parliament (game #1)

Deterministic pass. Reads TIER-B argument record and updates the Rubric Ledger in `personas/axis-pool.md`. Nothing promoted to `adopted` or `retired` yet — thresholds require ≥ 2 games of evidence. This game's counts contribute.

## Reviewer-stake ledger update (game #1)

| Axis | Slug | Band | Primary advocate | Game #1 state | Defend marks | Refute marks | Collision wins |
|---|---|---|---|---|---:|---:|---:|
| Minimum-Score Shape | C2 | C | Knizia | earned | 3 | 0 | 0 |
| Architectural Novelty | B5 | B | Chvátil | earned | 2 | 0 | 1 |
| Information-Transparency-Cost | B4 | B | Lacerda | earned | 1 | 0 | 1 |
| Elegance | A1 | A | (shared) | contested | 1 | 1 | 0 |
| Tension Budget | C1 | C | Knizia | ignored | 0 | 0 | 0 |
| Scarcity Bite | C3 | C | Rosenberg | ignored | 0 | 0 | 0 |
| Engine-Garden Dependency | C4 | C | Rosenberg | refuted | 0 | 2 | 0 |
| Late-Game Lock-in | D4 | D | (shared) | ignored | 0 | 0 | 0 |
| Point-Salad Incommensurability | C6 | C | Feld | ignored | 0 | 0 | 0 |
| Anti-Catch-up Pressure | C5 | C | Feld | refuted (self) | 0 | 2 | 0 |
| Decision Density | A2 | A | (shared) | ignored | 0 | 0 | 0 |
| System Gearing | B1 | B | Lacerda | weak-defended | 1 | 0 | 1 |
| Emergence-Replayability | A7 | A | (shared) | contested | 0 | 1 | 0 |
| Interaction | A3 | A | (shared) | ignored | 0 | 0 | 0 |
| Catastrophe Pressure | B2 | B | Chvátil | ignored | 0 | 0 | 0 |
| Action-Menu Clarity | C7 | C | Kramer-Kiesling | weak-defended | 0 | 0 | 1 |
| Family-to-Expert Scaling | D1 | D | Kramer-Kiesling | ignored | 0 | 0 | 0 |
| Spatial-Interaction Presence | D2 | D | Kramer-Kiesling | ignored | 0 | 0 | 0 |
| Count-Robustness | D3 | D | Stegmaier | weak-defended | 1 | 0 | 0 |
| First-Turn Compression | C8 | C | Stegmaier | ignored | 0 | 0 | 0 |
| Teachability | A6 | A | (shared) | contested | 0 | 1 | 0 |

21 drafted axes. Undrafted in this game (not scored): A4 Variance Calibration, A5 Downtime-Pacing, B3 Conversion Chain Depth.

## Promotion / retirement check

- **Adopted this session**: 0. (Threshold requires ≥ 2 earned states across ≥ 2 games. Three axes are positioned for adoption after one more game of evidence.)
- **Retired this session**: 0. (Threshold requires ≥ 2 refuted states across ≥ 2 games.)

**Axes queued for adoption** (earned in game #1; second earning in any future game triggers `adopted`):
- C2 Minimum-Score Shape
- B5 Architectural Novelty
- B4 Information-Transparency-Cost

**Axes queued for retirement** (refuted in game #1; second refutation in any future game triggers `retired`):
- C4 Engine-Garden Dependency
- C5 Anti-Catch-up Pressure

**Axes with 3+ ignored states** (silence penalty candidates):
- Tension Budget (Knizia ignored his own drafted axis beyond the in-game collision)
- Scarcity Bite (Rosenberg)
- Late-Game Lock-in (Rosenberg)
- Point-Salad Incommensurability (Feld)
- Decision Density (Feld)
- Interaction (Chvátil)
- Catastrophe Pressure (Chvátil)
- Family-to-Expert Scaling (Kramer-Kiesling)
- Spatial-Interaction Presence (Kramer-Kiesling)
- First-Turn Compression (Stegmaier)

**10 of 21 drafted axes were ignored.** This is the biggest signal of the session — nearly half the stakes were inert. The Argument protocol is too permissive.

## Rubric version bump

**v2.0 → v2.0.1.** No `adopted` or `retired` events, but the ledger has real entries now. Bump reflects that the rubric has moved from pristine (all axes `live`, no history) to active (21 axes have first-game records).

## Innovations surfaced

### I-parliament-01 — Ignored-stake protocol is too permissive

- **Dimension:** process (Argument phase)
- **Trigger pattern:** `ignored_stake_over_threshold`
- **Source:** games/0001-parliament/playtests/PT01-argument.md
- **Supporting voices:** Stegmaier (flagged via success-criteria miss), Knizia (several of his own stakes went ignored despite active session), Feld (same)
- **Scope:** universal
- **Status:** candidate
- **Note:** 10 of 21 drafted axes (48%) were ignored in the full 4p+3p session. Success criterion 1 (≥ 5 earned) failed because argument energy concentrated on ~5 axes while the rest sat idle. Proposal: add a "forced engagement" micro-phase after R3 where each persona must HOLD, ATTACK, DEFEND, or COLLIDE on any still-ignored stake of theirs OR retire that stake to "explicit-ignore" (stronger penalty than silent-ignore). Alternative: raise the silence penalty from 1 pt/session to 2 pts per ignored stake above a threshold (e.g., 3 ignored = -6 pts, which makes ignoring material to score).

### I-parliament-02 — Scoring-multiplier as distinct axis

- **Dimension:** new axis candidate
- **Trigger pattern:** `axis_not_in_pool`
- **Source:** end-of-game reviewer remark (Rosenberg)
- **Supporting voices:** Rosenberg (originator), Knizia (anchor axis is this axis, implicitly)
- **Scope:** pattern-across-games (likely applies broadly — games with endgame VP multipliers)
- **Status:** candidate
- **Note:** Rosenberg's Engine-Garden Dependency was refuted because Parliament doesn't have engine-gardens, but Parliament DOES have something structurally similar — the scoring multiplier that creates inter-axis multiplicative dependency. This is a new axis candidate: **Scoring Multiplier Dependency** (definition sketch: "Do endgame points depend multiplicatively on choices made across multiple subsystems, forcing balance across them?"). If added in v2.1, this axis would give Rosenberg a live place for his lens in future games.

### I-parliament-03 — Anchor-axis vulnerability when collision partner undrafted

- **Dimension:** draft protocol
- **Trigger pattern:** `anchor_axis_without_collision_partner`
- **Source:** TIER-A draft record — Knizia passed on Conversion Chain Depth (B3), leaving Minimum-Score Shape (C2) without its adjacency partner
- **Supporting voices:** Knizia (anchor persona)
- **Scope:** universal
- **Status:** candidate
- **Note:** The anchor axis is the designer's most load-bearing stake. If the draft leaves it without a collision partner, the anchor can't participate in collision wins — it must earn purely by direct defense. This worked in Parliament's case (Knizia earned 3 defends without a collision) but is fragile. Proposal: GATE check should additionally verify that the anchor axis has at least one adjacent axis drafted by another persona, OR flag this as an anchor-vulnerability for the session.

### I-parliament-04 — Argument-protocol success on architectural novelty claim

- **Dimension:** bet validation
- **Trigger pattern:** `novel_architecture_defended`
- **Source:** Chvátil's Architectural Novelty stake earned 3 marks (2 defends + 1 collision) across the session
- **Supporting voices:** Chvátil (stake-holder), Knizia (voted for Novelty collision), Lacerda (acknowledged gearing via draft)
- **Scope:** bet
- **Status:** adopted (2026-04-19) — this is direct evidence that the Parliament bet survives its own review. The novelty claim was tested and earned. This is not a new amendment; it's a validation log entry.
- **Note:** Chvátil-reviewer's v1.0 critique ("zero novel architectural bet") triggered the v2.0 bet change. In v2.0's first use, the same Chvátil-persona's Architectural Novelty stake earned the second-most marks of any reviewer stake (3 marks, behind Knizia's 3-defend Minimum-Score). The adversarial bet produced evidence for itself. Log this as the strongest bet-validation signal so far.

## Success criteria outcome

| # | Criterion | Result | Note |
|---|---|---|---|
| 1 | ≥ 5 earned stakes across panel | **FAIL** (3 earned) | Flagged as I-parliament-01 (ignored-stake protocol) |
| 2 | ≥ 2 earned on Band B or C | **PASS** (C2, B5, B4) | All three earned stakes in Euro-specific or persona-signature bands |
| 3 | ≥ 1 clean collision resolution | **PASS** (4 resolved) | K-K/Knizia, Chvátil/Lacerda, Stegmaier/Lacerda, Lacerda/Rosenberg |
| 4 | ≥ 1 axis promoted OR retired | **CONDITIONAL** | Nothing can promote/retire in game #1 alone; 3 axes queued for adoption, 2 queued for retirement after game #2 |

**Overall: Parliament anchor PARTIALLY PASSES v2.0 success criteria.** Three of four pass directly; criterion 4 is conditional on game #2; criterion 1 fails and produces its own amendment candidate.

**Verdict on the bet:** The Parliament architecture (adversarial stakes + collision resolution + deterministic amendment) produced:
- Three earned stakes on Euro-specific / persona-signature axes
- Four clean collisions with real semantic content
- One self-refutation (Feld refuting his own TIER-A stake based on evidence the adjacency chart generated — the factory surfaced an insight about itself)
- One amendment candidate directly addressing a protocol weakness
- One new-axis candidate (Scoring Multiplier Dependency)

The bet survives its first game. The rubric (the factory) is sharpening.

## Handoff to v2.1 planning

v2.0 has passed its anchor in a conditional sense. The miss on ≥5-earned means v2.1 should include:

- **Ignored-stake penalty strengthening** (I-parliament-01) — either forced-engagement micro-phase or 2x penalty per over-threshold ignore.
- **Scoring Multiplier Dependency axis** (I-parliament-02) — added to Band B with Rosenberg as primary advocate.
- **Anchor adjacency guarantee** (I-parliament-03) — GATE check enhancement.

v2.1 should ship before game #2 (Tigris & Euphrates) begins. Rate limit: 1 adopted amendment per 2-game cycle, so at most one of these three becomes `adopted` in v2.1. The other two stay `candidate` until game #2 provides cluster evidence.

Proceed to HANDOFF.
