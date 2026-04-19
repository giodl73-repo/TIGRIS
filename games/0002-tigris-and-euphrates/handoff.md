---
name: Tigris & Euphrates — Handoff
slug: te-handoff
game: 0002-tigris-and-euphrates
stage: handoff
version: 1.0.0
rubric_version: v2.2
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# T&E — Handoff (end of cycle)

## Locked (cycle-permanent)

- **T&E passed cleanly.** All four v2.1 success criteria met (Parliament missed criterion 1).
- **Two adoptions.** C2 Minimum-Score Shape and B4 Information-Transparency-Cost are now permanent pool members. First v2.x adoption events.
- **Rubric version: v2.2.** Bump reflects adoption; v2.1 protocol (FORCED-ENGAGEMENT) remains.
- **v2.1 FORCED-ENGAGEMENT fix validated.** Parliament had 48% ignore rate; T&E had 0% silent-retire.
- **Bet validated again.** Five earned stakes, two clean collisions, two new-adjacency candidates. The factory is doing what it promised.
- **Anchor held.** Knizia's C2 Min-Score Shape anchor earned its LAST-CALL. The anchor-persona recognized his own signature in his own game.

## Pending (needs decision before game #3)

- **Rate-limit pick for next cycle** — after game #3 (completing 2-game cycle), one amendment adopts. Recommendation: **I-parliament-03** (anchor-axis adjacency GATE check). Two instances already clustered; game #3 would be the third.
- **Adjacency chart amendment** — add B2 Catastrophe Pressure ↔ C5 Anti-Catch-up Pressure (I-te-01). Low-risk, low-controversy; could adopt as a v2.2.1 patch outside the rate limit if we treat adjacency-chart edits as not counting toward the rubric-amendment quota.
- **Identical-drafts investigation** — Parliament and T&E produced the same 21-axis draft. Is this a stability feature or a rigidity bug? Game #3 with a different game-type (e.g., party/light-weight) may naturally force persona-preference reshuffling. Or we could explicitly require 1 of each persona's preferred_axes to be re-ordered before game #3.

## Next session brief

**Candidate games for #3:**

- **Scythe (Stegmaier, 2016)** — tests I-parliament-02 (Scoring Multiplier Dependency — Scythe's end-game is a 7-factor multiplier). Also tests Stegmaier-persona on his own game (analogous to Knizia on T&E). Count-Robustness and First-Turn Compression should earn.
- **Wingspan (Hargrave / Stegmaier Games, 2019)** — another scoring-multiplier test. Tests Architectural Novelty (the bird-card engine is unusual) and Emergence/Replayability (169 bird cards).
- **Spec-review of TIGRIS v2.1** — recursive review of the factory under its own v2.1 procedure. High value for self-validation but risk of infinite regress. Not recommended yet.
- **Agricola (Rosenberg, 2007)** — tests Rosenberg on his own game. Should earn C3 Scarcity Bite, C4 Engine-Garden Dependency, D4 Late-Game Lock-in — which would REVERSE C3/C4's queued-for-retirement status.

**Recommendation: Agricola.** Triggers Rosenberg's retirement-reversal evidence; likely triggers B2/C4 adoption via second earning; tests the ledger's adoption/retirement symmetry. A Knizia-T&E / Rosenberg-Agricola pairing is the strongest calibration sequence for Euro design.

## Active amendments in flight

- **I-parliament-01** — ADOPTED in v2.1; validated in T&E via zero silent-retires.
- **I-parliament-02** — candidate (Scoring Multiplier); 1 instance. Awaiting Scythe or Wingspan.
- **I-parliament-03** — proposed-amendment (anchor-adjacency); 2 instances clustered. Scheduled for next cycle adoption.
- **I-te-01** — candidate (B2↔C5 adjacency); low-risk, adoptable outside rate limit.
- **I-te-02** — candidate (identical drafts); observational, not yet proposing change.
- **I-te-03** — ADOPTED (log entry, not protocol change) — v2.1 validated.

## What changed this session

- Rubric bumped v2.1 → v2.2.
- 2 axes adopted.
- Axis-pool ledger updated with T&E evidence.
- 4 new innovations logged (3 candidates + 1 validation).
- 0 silent-retires — v2.1 fix confirmed.

## What the factory learned about itself this game

1. **The rubric can recognize masterworks.** Knizia's own persona earned his own anchor on his own game. If this hadn't happened, the rubric would be broken.
2. **Retirement is appropriately slow.** Two games, zero retirements despite several refutation candidates. The rubric errs on the side of keeping axes, which is correct — premature retirement would destroy axis diversity.
3. **Drafts need perturbation.** Parliament and T&E produced identical drafts. Game #3 may naturally break this pattern if the game-type is different enough (e.g., Agricola's worker-placement focus would force Rosenberg-preferences into play harder).
4. **LAST-CALL-STAKE is powerful.** Three of five earned stakes in T&E came via LAST-CALL. The mechanism is a promotion path when other engagement paths don't materialize — suggests it may need a cap (max N last-calls per game) to prevent abuse.

## Proposed session 4 agenda

1. Adopt **I-te-01** (B2↔C5 adjacency) as a v2.2.1 patch (non-rubric-amendment).
2. Scaffold `games/0003-agricola/`.
3. Run Agricola through v2.1+ pipeline.
4. Amendment pass should trigger at least 2 more adoptions (B1, B2, D2 candidates) + potentially first retirement events.
5. At end of game #3, promote I-parliament-03 to adopted (rate-limit cycle completes).

Game #3 will complete the first 2-game cycle of v2.1 and trigger the second rubric amendment event.
