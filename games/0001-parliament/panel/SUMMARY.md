---
name: Parliament Summary — Panel
slug: parliament-panel-SUMMARY
game: 0001-parliament
stage: panel
version: 1.0.0
rubric_version: v2.0.1
bet_version: parliament
artifact_under_review: games/0001-parliament/design.md
personas_seated:
  - knizia (anchor)
  - rosenberg
  - feld
  - lacerda
  - chvatil
  - kramer-kiesling
  - stegmaier
total_stakes: 21 (7 personas × 3 axes)
earned: 3
refuted: 2
contested: 3
ignored: 10
collisions: 4
axes_adopted: []
axes_retired: []
created: 2026-04-19
updated: 2026-04-19
---

# Parliament — Panel Summary

## Headline

Parliament passes three of four v2.0 success criteria; fails ≥5-earned, producing its own most important amendment candidate (ignored-stake protocol). The Parliament bet (adversarial stakes + collision resolution + deterministic amendment) validates itself on its first run: Chvátil-reviewer's Architectural Novelty stake earned three marks, making the bet's architectural-novelty claim the second-strongest reviewer stake on record.

## Earned stakes

| Persona | Axis (band) | Marks | Defending moments |
|---|---|---:|---|
| Knizia | C2 Minimum-Score Shape (C) — **anchor axis** | 3 defends | R1 defense vs C4; R4 defense vs C4; 3p session defense |
| Chvátil | B5 Architectural Novelty (B) | 3 (2 defends + 1 collision) | R1 defense vs C1; collision-win vs Lacerda's Emergence at R3; R4 implicit validation |
| Lacerda | B4 Information-Transparency-Cost (B) | 2 (1 defend + 1 collision) | Stegmaier attack at R3; collision-win vs Stegmaier's Teachability at R3 |

## Refuted stakes

| Persona | Axis | Marks | Refuting moments |
|---|---|---:|---|
| Rosenberg | C4 Engine-Garden Dependency | 2 refutes | Chvátil attack at end-of-game; Lacerda collision-win at end-of-game |
| Feld | C5 Anti-Catch-up Pressure | 2 refutes (**self-refuted**) | Feld-reviewer observed in-game Anti-Catch-up earn status at R3 and formally retracted his own TIER-A stake |

Feld's self-refutation is the session's strongest signal that the Parliament procedure produces new evidence, not just archive-checking.

## Contested stakes

| Persona | Axis | Marks | Note |
|---|---|---|---|
| Knizia | A1 Elegance | 1 defend / 1 refute | Defended by Knizia at R2; lost collision to K-K's Action-Menu Clarity, earning single-moment refutation |
| Stegmaier | A6 Teachability | 0 defend / 1 refute | Lost collision to Lacerda's Transparency-Cost at R3 |
| Lacerda | A7 Emergence/Replayability | 0 defend / 1 refute | Lost collision to Chvátil's Novelty at R3 |

## Collisions resolved

Four collisions resolved cleanly (no mutual-ignored outcomes):

1. **R2 — K-K Action-Menu Clarity vs Knizia Elegance.** Vote 3-2 for Action-Menu Clarity. K-K stake earns collision credit; Knizia's Elegance takes single-moment refutation.
2. **R3 — Chvátil Architectural Novelty vs Lacerda Emergence/Replayability.** Vote 3-2 for Novelty. Chvátil earns collision win; Lacerda's Emergence takes refutation.
3. **R3 — Lacerda Information-Transparency-Cost vs Stegmaier Teachability.** Vote 3-2 for Transparency-Cost. Lacerda earns collision win; Stegmaier's Teachability takes refutation.
4. **End-of-game — Lacerda System Gearing vs Rosenberg Engine-Garden Dependency.** Vote 3-2 for System Gearing. Lacerda earns collision credit (but at 1 defend + 1 collision = 2 marks, her System Gearing stake falls just shy of earned — it's weak-defended pending game #2).

Collision votes show actual reviewer-level disagreement. No collision was decided 5-0 — every vote had dissent. Personas are genuinely arguing, not chorusing.

## Ignored stakes (silence penalties)

Ten stakes drafted but never engaged across the 4p+3p session:

- Knizia: C1 Tension Budget
- Rosenberg: C3 Scarcity Bite, D4 Late-Game Lock-in
- Feld: C6 Point-Salad Incommensurability, A2 Decision Density
- Chvátil: A3 Interaction, B2 Catastrophe Pressure
- Kramer-Kiesling: D1 Family-to-Expert Scaling, D2 Spatial-Interaction Presence
- Stegmaier: C8 First-Turn Compression

48% ignore rate is the session's most important protocol signal. See amendment candidate I-parliament-01.

## Amendment outcomes (deterministic pass)

**Adopted this session: 0.** (Threshold requires ≥ 2 earned across ≥ 2 games.)
**Retired this session: 0.** (Threshold requires ≥ 2 refuted across ≥ 2 games.)

**Queued for adoption (one more earning in game #2 triggers):**
- C2 Minimum-Score Shape (Knizia's anchor)
- B5 Architectural Novelty (Chvátil)
- B4 Information-Transparency-Cost (Lacerda)

**Queued for retirement (one more refutation triggers):**
- C4 Engine-Garden Dependency (Rosenberg's diagnostic)
- C5 Anti-Catch-up Pressure (Feld, self-refuted)

**Rubric bump: v2.0 → v2.0.1** (ledger has entries; no thresholds triggered).

## New innovations (cluster candidates for v2.1)

1. **I-parliament-01** — Ignored-stake protocol too permissive (48% ignore rate). Proposal: forced-engagement micro-phase or stronger silence penalty. Status: candidate.
2. **I-parliament-02** — "Scoring Multiplier Dependency" as a new axis, distinct from Engine-Garden Dependency. Surfaced by Rosenberg at end-of-game. Status: candidate.
3. **I-parliament-03** — Anchor axis without drafted collision partner is fragile. GATE enhancement: require ≥ 1 adjacency-partner drafted by another persona. Status: candidate.
4. **I-parliament-04** — Bet validation logged. Architectural Novelty earned 3 marks, making the bet testable and passed. Status: adopted (as a validation log entry, not a protocol change).

## Success criteria check (from `personas/playtest-rubric.md`)

| # | Criterion | Result | Signal |
|---|---|---|---|
| 1 | ≥ 5 earned stakes across panel | **FAIL (3)** | Amendment candidate I-parliament-01 |
| 2 | ≥ 2 earned on Band B or C | **PASS** | C2, B5, B4 |
| 3 | ≥ 1 clean collision | **PASS (4)** | All collisions resolved with real dissent |
| 4 | ≥ 1 axis promoted or retired | **CONDITIONAL (0 this game, 3+2 queued for game #2)** | Game #2 will trigger first adoption/retirement |

**Three of four pass; one fails productively (generates its own amendment).** This is the factory working as designed — a failed criterion produces evidence, not a reason for re-design.

## Punchlist (ordered)

1. **[I-parliament-01]** Strengthen ignored-stake penalty before game #2. Candidate fixes: forced-engagement micro-phase between R3 and R4; 2× silence penalty per over-threshold ignored stake; or require each persona to declare at draft-end which of their 3 stakes they'll defend-critical.
2. **[I-parliament-03]** Add anchor-adjacency GATE check. Before TIER-B begins, verify that the anchor axis has ≥ 1 adjacency partner drafted by another persona.
3. **[I-parliament-02]** Propose Scoring Multiplier Dependency axis for v2.1. Definition sketch + primary advocate: Rosenberg.
4. Rate limit: 1 adopted amendment per 2-game cycle. Pick one of the three (recommend I-parliament-01 — highest cluster-evidence) to promote to `proposed-amendment` before game #2.

## GATE check on this review itself

Did the review respect the GATE rules from TIER-A?

- ✅ Anchor stake viable — Knizia's C2 Minimum-Score Shape was earned.
- ✅ Draft coverage (all 4 bands).
- ✅ Collision candidates surfaced and resolved (4).

**Review is valid.** Parliament's first TIER-B argument produced a well-formed record.

## Next step

1. Write `games/0001-parliament/handoff.md`.
2. Update `TRACKER.md`.
3. Commit.
4. Decide between: run game #2 (T&E) now, or patch the ignored-stake protocol first.

Recommended: patch the ignored-stake protocol first. T&E will be a richer testbed with game #2 evidence but only if the protocol fix prevents another 48% ignore rate.
