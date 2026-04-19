---
name: Dominion — Handoff
slug: dominion-handoff
game: 0004-dominion
stage: handoff
version: 1.0.0
rubric_version: v2.3
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# Dominion — Handoff (cycle 2 close)

## Locked (cycle-permanent)

- **Dominion is anchor #4, strongest review to date.** All 4 success criteria pass by wide margins. 8 earned stakes (record). 5 axis adoptions in one session (record). 3 clean collisions.
- **7 axes adopted in the Pool** (up from 2): C2, B4, B5, A6, A2, B1, D4. 29% of the Pool is now permanent.
- **Rubric v2.3.** Major version bump consolidating 5 adoptions + I-parliament-03 protocol amendment (anchor-adjacency GATE check).
- **Retirement reversal logged** — C4 Engine-Garden Dependency moved from queued-for-retirement back to live via Dominion earning. First reversal in TIGRIS history.
- **0% silent-retire for 3rd consecutive game.** v2.1 FORCED-ENGAGEMENT is absolutely stable.
- **Cycle 2 rate-limit consumed** (I-parliament-03 adopted). Cycle 3 begins with game #5.

## Pending (needs decision before game #5)

- **I-facets-03 — Systematically undrafted axes (A4, A5, B3).** Now 4-instance cluster (4 games, 0 drafts). Scheduled for cycle 3 adoption. Options:
  - **Option A (recommended)**: Add "Vaccarino" persona (8th designer-persona) with preferences including A4/A5/B3. Addresses both I-facets-03 AND I-dominion-04.
  - **Option B**: Retire A4, A5, B3 from the Pool. Thins the rubric but is cleaner.
  - **Option C**: Accept as "aspirational" pool members; no amendment. Keeps options open for future personas.

- **I-dominion-04 — Vaccarino persona**. Justified by Dominion review — Chvátil's lens didn't perfectly match Vaccarino's design emphasis (card-design, combo engineering, per-card elegance). A Vaccarino persona with preferred_axes [B5, C4, C7, A1] could cover deck-building-style games authentically.

- **Combining options**: Option A+I-dominion-04 together — single amendment adds Vaccarino whose preferences include A4/A5/B3. Solves both clusters at once. Strongly recommended.

## Next session brief

**Candidate games for #5:**

- **Scythe (Stegmaier, 2016)** — tests I-parliament-02 (Scoring Multiplier Dependency); Stegmaier-on-Stegmaier; adoption triggers on A7, D1, C4 likely.
- **Agricola (Rosenberg, 2007)** — Rosenberg-on-Rosenberg; tests C3 Scarcity Bite + C4 Engine-Garden decisively. Could trigger first retirement via actively refuting C5 or C3.
- **7 Wonders (Bauza, 2010)** — draft-genre; different genre for rubric breadth. Tests C6 Point-Salad (draft games often use incommensurable scoring).
- **Spirit Island (Reed, 2017)** — cooperative game; tests B1/B2 in unfamiliar (co-op) territory.

**Recommendation: Scythe.** Three reasons:
1. Stegmaier-on-Stegmaier, paralleling Knizia-on-T&E's calibration test.
2. Scythe's 7-factor end-game multiplier directly tests I-parliament-02 (Scoring Multiplier Dependency candidate axis). Provides 2nd instance for cluster promotion.
3. Likely triggers 2-3 more adoptions (A7 second earning; D1 second earning; C4 second earning; all queued-for-adoption).

**Before Scythe: add Vaccarino persona.** Enables A4/A5/B3 advocacy + adds authentic deck-building lens to future reviews.

## Active amendments in flight

- **I-parliament-01 → v2.1** — ADOPTED. Validated 3 consecutive games.
- **I-parliament-02** — candidate (1 instance). Awaiting Scythe (2nd instance would cluster-promote).
- **I-parliament-03 → v2.3** — ADOPTED. Applies forward-only from game #5.
- **I-parliament-04** — adopted log entry.
- **I-te-01** — candidate (B2↔C5 adjacency — low-risk patch).
- **I-te-02** — closed observational (pattern broke in Dominion).
- **I-te-03** — adopted log entry.
- **I-facets-01 / 02** — candidates (adjacency chart updates, low-risk patches).
- **I-facets-03** — proposed-amendment (4 instances). Cycle 3 candidate.
- **I-facets-04** — adopted log entry.
- **I-dominion-01 / 02** — adopted log entries (validations).
- **I-dominion-03** — observational candidate.
- **I-dominion-04** — candidate (Vaccarino persona).

## Summary of v2.x era so far (4 games)

| Metric | Parliament | T&E | FACETS | Dominion | Total / Mean |
|---|---:|---:|---:|---:|---:|
| Earned stakes | 3 | 5 | 5 | 8 | **21 / 5.25 avg** |
| Refuted (full) | 0 | 1 | 1 | 0 | 2 |
| Retire-explicit | — | 3 | 4 | 4 | 11 |
| Collisions | 4 | 2 | 3 | 3 | 12 |
| Silent-retire | 10 (48%) | 0 | 0 | 0 | 10 total (pre-v2.1) |
| Adoptions this session | 0 | 2 | 0 | 5 | **7 total** |
| Retirement reversals | — | — | — | 1 | 1 |
| Success criteria pass | 3/4 | 4/4 | 3/4 | 4/4 | 14/16 |
| Rubric version | v2.0.1 | v2.2 | v2.2.1 | **v2.3** | 5 bumps |

**The bet is rigorously validated.** The factory produces evidence, updates the ledger, evolves the rubric — and no game has failed to produce meaningful amendment action.

## What changed this session

- Dominion scaffolded using `/tigris-concept` + `/tigris-design` (review-mode) skills.
- Full pipeline run with v2.1 FORCED-ENGAGEMENT.
- `/tigris-amendment` skill triggered 5 adoptions in one pass (deterministic, clean).
- Rubric v2.3 with new GATE check.
- 7 adopted axes; 4 active queued-for-adoption; 3 queued-for-retirement; 1 reversal logged.

## What the factory learned about itself

- **Retirement reversal works.** C4's trajectory is proof the ledger is context-aware. An axis can fail in 3 games, earn in the 4th, and get rehabilitated.
- **Anchor collisions are powerful.** First cross-player anchor collision (B5↔A7) in 4 games; outcome was sharp and informative. Worth the GATE check.
- **Multi-adoption events happen.** Dominion alone adopted 5 axes. Pattern: diverse games produce fast rubric evolution; similar games produce slow curation.
- **Persona gaps exist.** Vaccarino's lens was proxied by Chvátil. Adding specific personas for specific game-design traditions would expand coverage — especially with A4/A5/B3 undrafted for 4 games.
- **The bet is robust.** 4 games, 7 adoptions, 1 reversal, 0 retirements, 0 silent-retires (under v2.1). The architecture produces informative evidence every session.

Cycle 2 complete. Factory ready for game #5 (recommend: add Vaccarino persona + run Scythe).
