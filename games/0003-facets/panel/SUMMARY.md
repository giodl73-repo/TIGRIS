---
name: FACETS Summary — Panel
slug: facets-panel-SUMMARY
game: 0003-facets
stage: panel
version: 1.0.0
rubric_version: v2.2.1
bet_version: parliament
artifact_under_review: games/0003-facets/design.md
personas_seated: [feld (anchor), stegmaier, kramer-kiesling, rosenberg, knizia, lacerda, chvatil]
total_stakes: 21
earned: 5
refuted_full: 1
retire_explicit: 4
hold_explicit: 7
weak_defended: 3
contested: 1
silent_retire: 0
collisions: 3
axes_adopted: []
axes_retired: []
created: 2026-04-19
updated: 2026-04-19
---

# FACETS — Panel Summary

## Headline

FACETS earns 5 stakes and passes success criteria 1-3; criterion 4 fails transiently because game #3 provides FIRST earnings for 4 axes (A6, A2, D4, C6) — game #4 will trigger the next adoption wave. Feld's anchor Point-Salad Incommensurability (C6) earned 3 marks in the game specifically designed to test it, rehabilitating an axis he himself retire-explicit'd in T&E.

**The rubric self-corrected.** An axis can be retired-explicit in one game and earned in another — the forward-only versioning preserves the evidence without pretending the axis is binary good/bad. This is the adversarial bet at work.

Second consecutive game with **0% silent-retire** — v2.1 FORCED-ENGAGEMENT holds.

## Earned stakes (5)

| Persona | Axis | Band | Marks | Significance |
|---|---|---|---:|---|
| Feld | C6 Point-Salad Incommensurability (anchor) | C | 3 (2d + 1cw) | Anchor vindicated after T&E retire-explicit — axis rehabilitated |
| Knizia | C2 Minimum-Score Shape (ADOPTED) | C | 3 defends | Diagnostic-low stake (4/4) earned via defenses; adopted axis continues to validate |
| Feld | A2 Decision Density | A | LAST-CALL success | Late-game density justified the 7/7 stake |
| Stegmaier | A6 Teachability | A | 1 defend + 1 collision win | First active earning after 2 games of contested-status |
| Rosenberg | D4 Late-Game Lock-in | D | 1 defend + 1 collision win | First active earning; axis active after 2 games hold |

## Retire-explicit (4) — v2.1 clean retreats

Four personas chose explicit retirement rather than defending stakes that didn't fit FACETS:

- **Rosenberg C4 Engine-Garden** — FACETS has no engines. +0.5 refute.
- **Lacerda B1 System Gearing** — no subsystem interdependence. +0.5 refute.
- **Chvátil B2 Catastrophe Pressure** — no reversal events. +0.5 refute.
- **Kramer-Kiesling D2 Spatial-Interaction** — no board. +0.5 refute.

All four are diagnostic: the axes don't apply to FACETS, and the personas honestly mark that. This is the protocol working as designed.

## Self-refutation (1) — second in TIGRIS history

- **Chvátil A3 Interaction** — Chvátil-reviewer attacked his own 3-4 stake at Turn 8, conceding it was too generous given FACETS's market-exclusion-only interaction. +1 full refute.

This is the second self-refutation pattern (first was Feld C5 Anti-Catch-up in Parliament). Self-refutations prove personas are updating stakes based on in-game evidence — the strongest validation signal for the bet. I-facets-04 log entry adopted.

## Collisions resolved (3)

| Round | Collision | Vote | Winner |
|---|---|---|---|
| Turn 4 | A2 Decision Density vs A6 Teachability (new adjacency) | 3-2 | A6 Teachability |
| Turn 8 | C6 Point-Salad vs A6 Teachability (new adjacency) | 3-2 | C6 Point-Salad |
| Turn 12 | D4 Late-Game Lock-in vs C5 Anti-Catch-up | 3-2 | D4 Late-Game Lock-in |

All votes had real dissent. Two new-adjacency pairs surfaced and are candidates for the global adjacency chart (I-facets-01, I-facets-02).

## FORCED-ENGAGEMENT (v2.1) performance

Second consecutive game at **0% silent-retire.** The protocol is validated:

| Game | Drafted | Silent-retire | Ignore rate |
|---|---:|---:|---:|
| Parliament (v2.0) | 21 | 10 | 48% |
| T&E (v2.1) | 21 | 0 | 0% |
| FACETS (v2.1) | 21 | 0 | 0% |

Average 0% across 2 games under v2.1. Fix is robust.

## Axes queued for adoption (awaiting game #4)

Four axes had their FIRST earning this game. Each needs one more game's earning to adopt:
- **A6 Teachability** (Stegmaier)
- **A2 Decision Density** (Feld)
- **D4 Late-Game Lock-in** (Rosenberg)
- **C6 Point-Salad Incommensurability** (Feld anchor vindication)

Three axes had their first earning in T&E but didn't earn in FACETS (retire-explicit instead):
- **B1 System Gearing** (stalled)
- **B2 Catastrophe Pressure** (stalled)
- **D2 Spatial-Interaction** (stalled)

These three need to earn in game #4 to adopt. Otherwise they remain queued-for-adoption indefinitely.

## Axes queued for retirement

- **C4 Engine-Garden Dependency** — 1.5 cumulative refute weight (Parliament 1 + FACETS 0.5). One more refutation triggers retirement.
- **C3 Scarcity Bite** — 1 cumulative refute (T&E).
- **C5 Anti-Catch-up Pressure** — 1 cumulative refute (Parliament self-refute).

Retirement protocol remains conservatively slow; no retirement triggered this session.

## Ledger state

The rubric is now evolving visibly:
- **2 adopted axes** (C2, B4).
- **4 queued-for-adoption** (A6, A2, D4, C6 — first earnings).
- **3 queued-for-adoption stalled** (B1, B2, D2 — earned T&E, retire-explicit FACETS).
- **3 queued-for-retirement** (C4, C3, C5).
- **3 live never-drafted** (A4, A5, B3 — I-facets-03 candidate for pool curation).

Of 24 axes in the Pool, 12 have active evidence (adopted or queued). Half the Pool is in motion. The bet's promise — "rubric evolves by play" — is operational.

## Amendment outcomes

**Adopted this session: 0** (no threshold triggers hit).
**Retired this session: 0** (retirement protocol correctly conservative).
**Rubric version**: v2.2 → **v2.2.1** (ledger-update-only patch).

**Cluster promotions**:
- **I-parliament-03 → ready-for-adoption** (3 instances of anchor-without-partner).
- **I-te-02 → proposed-amendment** (3 instances of identical drafts; awaiting user decision).

**New candidates this session**:
- I-facets-01 — C6↔A6 new adjacency.
- I-facets-02 — A2↔A6 new adjacency.
- I-facets-03 — Systematically undrafted axes (A4, A5, B3).
- I-facets-04 — Self-refutation bet validation (adopted log entry).

## Success criteria check

| # | Criterion | FACETS | Parliament | T&E |
|---|---|---|---|---|
| 1 | ≥ 5 earned stakes | **PASS (5)** | FAIL (3) | PASS (5) |
| 2 | ≥ 2 earned on Band B or C | **PASS (2)** | PASS | PASS (4) |
| 3 | ≥ 1 clean collision | **PASS (3)** | PASS (4) | PASS (2) |
| 4 | ≥ 1 axis promoted OR retired | **FAIL (0)** | FAIL* | PASS (2) |

*Parliament's criterion 4 was conditional — 2 axes queued for adoption, triggered in T&E.

FACETS's criterion 4 failure is **transient** — all four queued-for-adoption axes await second-game earning. Game #4 should trigger at least 1 adoption, restoring criterion 4 to PASS.

## Punchlist (for game #4)

1. **Game #4 should test the stalled-queued axes** (B1, B2, D2) to either advance or decisively retire them. A game with strong system-gearing (Lacerda's territory) or catastrophe-like reversals would test B1/B2 cleanly.
2. **I-parliament-03 ready for adoption** at next rate-limit window (cycle 2 close = game #4 completion). Tightens GATE check.
3. **I-facets-01 / I-facets-02** adjacency-chart updates are low-risk patches; adopt as v2.2.2 outside rate-limit.
4. **I-facets-03 pool curation decision needed** — retire A4/A5/B3 or add a persona advocate.
5. **I-te-02 cluster** confirmed at 3 instances — user decision needed on whether to force draft perturbation or accept stable preferences as a feature.

## What FACETS revealed

- **Feld's C6 came back from retire-explicit to earned.** Axes aren't binary — they earn in the right game and retire-explicit in the wrong one. The ledger captures both as evidence; the forward-only versioning preserves the story.
- **Retire-explicit is the v2.1 achievement.** 4 personas used it cleanly in FACETS. This is healthier than silent-retire (v2.0) or stakes-grinding-on-axes-that-don't-fit. Personas now have an honest path to "this doesn't apply here."
- **Self-refutation is now pattern, not anomaly.** Parliament had 1; FACETS has 1. The adversarial bet produces self-updating personas.
- **The rubric can handle rehabilitation.** Axes that retire-explicit in one game can earn in another. Retirement isn't permanent without cumulative evidence across games.
- **Ignored-axis pattern persists in Pool curation** (A4, A5, B3). This isn't about any one game — it's about whose preferences the Pool represents. Adding a persona or curating the pool addresses it.

## Next step

HANDOFF. Then game #4 — recommended: **Agricola** (Rosenberg-on-Rosenberg, likely triggers stalled-queued axes + retirement tests).
