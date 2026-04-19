---
name: Parliament — Handoff
slug: parliament-handoff
game: 0001-parliament
stage: handoff
version: 1.0.0
rubric_version: v2.0.1
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# Parliament — Handoff (end of cycle)

## Locked (game-permanent)

- **Parliament passed its anchor review conditionally.** 3/4 success criteria met; criterion 1 (≥ 5 earned) failed and produced amendment candidate I-parliament-01.
- **The Parliament bet validates itself.** Architectural Novelty earned 3 marks — the strongest evidence the adversarial architecture is doing what v2.0 claimed.
- **Three axes queued for adoption**: C2 Minimum-Score Shape, B5 Architectural Novelty, B4 Information-Transparency-Cost. One more game's earning triggers `adopted`.
- **Two axes queued for retirement**: C4 Engine-Garden Dependency, C5 Anti-Catch-up Pressure.
- **Rubric version: v2.0.1.** Three amendment candidates logged (I-parliament-01/02/03); one validation log entry (I-parliament-04).
- **Anchor persona + axis confirmed.** Knizia on C2 Minimum-Score Shape. The scoring multiplier IS a minimum-score shape, as claimed in CONCEPT. Non-colliding reviewers voted for it in R2's collision despite dissent.
- **Parliament's self-refutation worked.** Feld's own Anti-Catch-up stake was refuted by evidence the adjacency chart generated in-play. The factory surfaced an insight about its own game.

## Pending (needs decision)

- **Ignored-stake protocol weakness.** 48% ignore rate this session. Before game #2, pick a fix: forced-engagement micro-phase, 2× silence penalty, or declare-defendable stake at draft-end.
- **Anchor adjacency rule.** Add GATE enhancement or accept fragility. Knizia passed on B3 Conversion Chain Depth, which would have been his C2 collision partner. He earned without it — but shouldn't always have to.
- **New axis: Scoring Multiplier Dependency.** Add to v2.1? It would give Rosenberg a place in future games and is distinct from C4 Engine-Garden Dependency.
- **Rate limit pick.** Only one amendment can be adopted before game #2. Recommend I-parliament-01 (strongest cluster signal, most load-bearing for future reviews).

## Next session brief

**Option A — Patch v2.1 then run game #2 (T&E).** Adopt I-parliament-01 (ignored-stake fix). Scaffold `games/0002-tigris-and-euphrates/`. Run full pipeline. Expected outcome: T&E evidence triggers first real adoption/retirement events.

**Option B — Run game #2 immediately without patch.** Accept 48% ignore rate risk. T&E should produce more argument naturally (known masterwork, well-discussed in design community) which may mitigate the ignore problem empirically.

**Option C — Spec-review Parliament's design under v2.0 procedure.** Recursive but useful: take Parliament's design.md through `/tigris-panel spec-review` to validate the game design independently of the playthrough. This would be the strictest test of v2.0: Parliament reviewing Parliament-the-document.

**Recommendation: A.** Option B delays the protocol fix that every subsequent game will need. Option C is rigorous but risks infinite regress. Option A moves the factory forward one productive step.

## Active amendments in flight

- I-parliament-01 (ignored-stake protocol) — candidate, awaiting ratification for v2.1.
- I-parliament-02 (scoring multiplier axis) — candidate, awaiting game #2 cluster evidence.
- I-parliament-03 (anchor adjacency GATE check) — candidate, awaiting second instance.

## What changed this session

- v2.0 anchor game complete.
- Axis pool ledger has first 21 entries.
- Innovation log has 4 new entries (3 candidates + 1 validation).
- `personas/axis-pool.md` ledger updated.
- Success criteria check logged for future comparison.

## What the factory learned about itself

Parliament the game demonstrated what Parliament the procedure claimed: when personas draft exclusive axes and argue under pressure, they produce evidence that (a) some stakes are demonstrably stronger than others, (b) cross-persona disagreement is the signal, and (c) the factory can surface insights about itself that no single persona had going in.

The architectural bet is not a polish item. It's the core move, and it just earned its first defense in play.
