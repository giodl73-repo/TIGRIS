---
name: The Castles of Burgundy — Handoff
slug: castles-of-burgundy-handoff
game: 0012-castles-of-burgundy
stage: handoff
version: 1.0.0
rubric_version: v2.10
bet_version: parliament
author: TIGRIS (burgundy-reviewer)
created: 2026-04-19
updated: 2026-04-19
---

# The Castles of Burgundy — Handoff

## Locked (cycle-permanent within this review)

- **First Feld-on-Feld published anchor.** C6 Point-Salad Incommensurability re-earns in the game cited in its own axis definition.
- **15 earned stakes** — highest single-game earning count in factory history (tied with TtA's 12+, exceeds it).
- **2 proposed adoptions** (game-local): A5 Downtime-Pacing (Famiglia + CoB = 2 earnings → adoption trigger), D3 Count-Robustness (Scythe + ZEN PATH + CoB = 3 earnings → adoption trigger).
- **A4 Variance Calibration first-ever earning** across 11 prior games — canonical calibrated-variance case located. Queued-for-adoption.
- **5 retire-explicits on adopted axes** (B2, A3, D2, C2, B4) — each precisely diagnoses Castles' structural absences. Adopted status preserved for all.
- **3 clean collisions** (A3↔D2, A4↔B2, C6↔A6) — primary collision-candidates from Tier-A all fired.
- **0 silent-retire** (10th consecutive game under v2.1).
- **Rubric version baseline**: v2.10 (user integrates proposed adoptions → v2.11 candidate).

## Artifact inventory (all in `games/0012-castles-of-burgundy/`)

| File | Purpose |
|---|---|
| `concept.md` | BGG 84876 frontmatter + anchor declaration (Feld on C6) + expected-earnings roadmap |
| `design.md` | Fair-use rules summary (alea/Ravensburger 2011 edition) |
| `tierA-stakes.md` | 24 drafts (8 personas × 3) + stake tables per count + GATE pass + collision candidates |
| `playtests/PT01-argument.md` | 4p + 3p narrated playthrough with moment-anchors, FORCED-ENGAGEMENT, earned/refuted/collision record |
| `amendments.md` | Game-local classification; proposed adoption triggers; innovation log (I-cob-01..06) |
| `panel/SUMMARY.md` | Panel summary, success criteria check, earned/refuted/hold tables, state projection |
| `handoff.md` | This file |

## Success criteria (from `personas/playtest-rubric.md`)

| # | Criterion | Result |
|---|---|---|
| 1 | >= 5 earned stakes | **PASS (15)** |
| 2 | >= 2 earned on Band B or C | **PASS (8 across B+C)** |
| 3 | >= 1 clean collision resolution | **PASS (3 clean)** |
| 4 | >= 1 axis adopted or retired | **PASS** (2 proposed adoptions + 1 first-earning queued) |

All 4 PASS.

## Proposed pool changes for user integration

| Axis | Pre-CoB state | CoB result | Post-CoB proposed |
|---|---|---|---|
| **A5 Downtime-Pacing** | live; 1 earning (Famiglia LAST-CALL) | 2 defends (4p + 3p) | **adopted** (21st adoption) |
| **D3 Count-Robustness** | live; 2 earnings (Scythe + ZEN PATH) | 2 defends (4p + 3p) = 3rd earning | **adopted** (22nd adoption) |
| **A4 Variance Calibration** | live; 0 earnings in 11 games | 2 defends + collision win = first earning | **queued-for-adoption** (pending 2nd earning) |

**If user integrates both adoptions**: Pool at **22 of 25 adopted (88%)**; Band D fully adopted; only A4 (queued) + C8 (0 earnings) remain live among non-retired.

## Proposed innovation log entries

Six innovations recorded in `amendments.md`:
- **I-cob-01** — First Feld-on-Feld published anchor
- **I-cob-02** — A5 Downtime-Pacing adopts via Feld canonical pacing
- **I-cob-03** — D3 Count-Robustness triple-earns → adoption
- **I-cob-04** — A4 Variance Calibration first earning ever
- **I-cob-05** — Multiplayer-solitaire critique operationalizes as A3 retire-explicit
- **I-cob-06** — C6 ↔ A6 adjacency both-and case resolved

## Feld anchor pattern (6 of 8 designers anchored own work)

- Knizia: T&E (#2) + ZEN PATH (#10 original)
- Feld: FACETS (#3 original) + **CoB (#12 published — first Feld-on-published-own-game)**
- Stegmaier: Scythe (#5)
- Chvátil: UNFOLD (#7 original) + TtA (#8)
- **Pending**: Rosenberg, Lacerda (Lisboa #11 in-progress by teammate), K-K (Tikal #13 completed per task list), Vaccarino-published (Kingdom Builder)

## Notable findings for user attention

1. **C6 reference-case validation.** The axis-pool definition names Castles of Burgundy. Reviewing at Feld's anchor produced C6's strongest possible validation: re-earn + anchor-adjacency collision win. This is the canonical adoption-reinforcement pattern.
2. **A4 dormant-axis pattern.** A4 Variance Calibration was the longest-dormant axis after C1 was rehabilitated at PR. CoB's calibrated variance (dice + workers + silver + knowledge flexibility) fulfills A4's axis definition literally. Parallels C1's PR stress-test (#6).
3. **Multiplayer-solitaire critique captured structurally.** The BGG community's most-cited Castles critique maps exactly to A3 Interaction retire-explicit. The rubric operationalizes an established community complaint without retiring A3 (adopted protection holds).
4. **Both-and adjacency resolution.** C6 ↔ A6 adjacency (from I-facets-01) was previously tested as tension in FACETS. CoB demonstrates adjacencies can resolve as co-earning when rules iconography bridges incommensurable scoring with teachability.
5. **Feld's signature A5 adopts.** Feld's stated philosophy "every turn should give you something" (persona file line 24) formally adopts into the rubric after 2 canonical earnings. Designer philosophy → rubric axis → adoption lifecycle demonstrates.
6. **Diagnostic-low precision.** The 5 retire-explicits (B2/A3/D2/C2/B4) collectively form a structural portrait of Castles: dice-driven point-salad with no plague, no interaction, no hidden info, no min-score, no shared board. This is exactly the game Feld designed — rubric maps reality faithfully.

## Notes on hard constraints honored

- ✅ Wrote ONLY to `games/0012-castles-of-burgundy/`.
- ✅ Did NOT modify `personas/*`, `TRACKER.md`, `CLAUDE.md`, or any shared state.
- ✅ BGG ID 84876 in frontmatter.
- ✅ Forbidden-vocabulary discipline enforced (no "fun", "well-designed", "solid", "great game"; all claims axis-grounded, persona-attributed, count-anchored).
- ✅ No git commits.
- ✅ Fair-use posture on design.md (summary, not reproduction).

## Next session brief (user picks)

Backlog candidates:
- **Tikal / Torres** (K-K-on-K-K first time; BGG 54) — appears to be completed per task list
- **Kingdom Builder** (Vaccarino published; BGG 107529)
- **Agricola / Le Havre** (Rosenberg-on-Rosenberg; user previously excluded Agricola)
- **Bruges, Bora Bora, Troyes** — Feld dice-games, candidates for A4 second-earning
- **Third TIGRIS original design**

Recommendation: if user wants to close A4's queued-for-adoption, Bruges (BGG 136888) is the cleanest 2nd-earning candidate — same dice-menu + worker-mitigation architecture. Alternatively, K-K-on-K-K via Tikal/Torres completes the designer-on-own pattern to 7/8.

## State summary (game-local, projected post-integration)

| Metric | Value |
|---|---:|
| Game number | 12 |
| Earned stakes | 15 |
| Retire-explicit | 5 (all adopted axes preserved) |
| Hold-explicit | 4 |
| Silent-retire | 0 |
| Collisions | 3 clean |
| Adoptions proposed | 2 (A5, D3) |
| First-earning-ever | 1 (A4) |
| Proposed Pool post-integration | 22/25 (88%) |
| Proposed rubric version | v2.10 → v2.11 |
| Band D | fully adopts (4/4) |

Task complete per user brief. No further action from burgundy-reviewer.
