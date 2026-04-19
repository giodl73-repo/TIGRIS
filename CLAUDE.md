# TIGRIS — House Rules for Claude

A Claude-Code-driven factory for Euro-tradition board games. Designs new games and reviews existing ones through the same pipeline. Sibling to marathon (D&D), puzzlehunt, chronicle, rmm, artisan.

Spec: `docs/specs/2026-04-19-tigris-design.md` (read it before doing anything non-trivial).

## The pipeline

```
CONCEPT → DESIGN → TIER-A → [GATE] → TIER-B → PANEL → INNOVATE → HANDOFF
         (skip for reviews)          ↑
                                     └─ gate fail → revise DESIGN, re-run TIER-A
```

Reviews (of existing games or of TIGRIS itself) start at TIER-A or at PANEL depending on what's being evaluated. The pipeline is the same; originals run every stage.

## Frontmatter contract

Every generated file starts with YAML frontmatter:

```yaml
---
name: <human-readable title>
slug: <hyphenated>
game: <NNNN-slug>                  # omit for non-game artifacts (rubric, personas, spec)
stage: concept|design|tierA|tierB|panel|innovate|handoff
version: <semver>
rubric_version: v1.0               # the rubric version scored against
author: <persona-slug or human>
created: YYYY-MM-DD
updated: YYYY-MM-DD
sources: []
---
```

## Forbidden vocabulary

See `personas/forbidden-words.md`. Short version: no "fun", "well-designed", "solid", "great game". Every claim must name (a) which rubric axis, (b) which persona/lens, (c) which player count or game-state.

## Rubric

Eight axes, v1.0. See `personas/playtest-rubric.md`. Forward-only versioning: when amended, old scores lock at their rubric version.

| # | Axis | Short definition |
|---|---|---|
| 1 | Elegance | Rule-count-to-depth ratio |
| 2 | Decision Density | Meaningful choices per turn |
| 3 | Interaction | Player-to-player impact |
| 4 | Thematic Integration | Mechanics ↔ theme coherence |
| 5 | Variance Calibration | Luck scaled to game length |
| 6 | Downtime / Pacing | Wait between turns |
| 7 | Teachability | Rules → first-turn friction |
| 8 | Emergence / Replayability | Strategy space diversity |

Per-cell verdicts on weighted aggregate: `win` ≥ 7.0 · `marginal` 4.0–6.9 · `fail` < 4.0.

## Personas

Seven designer voices + five player lenses. Full roster in `personas/designers/` and `personas/player-lenses/`. Each persona has a `rubric_weights` vector (sum = 8.0) that reshapes the 8-axis scoring.

## Directory map

- `docs/specs/` — design docs (this project's spec, future amendments)
- `games/NNNN-<slug>/` — one directory per game (new or review)
- `personas/` — designer roster, player lenses, rubric, innovation log, forbidden words
- `skills/` — Claude Code skills (`tigris-panel`, `tigris-innovate`, etc.)
- `reference/` — cached rules, designer research, community archives
- `scripts/` — seeded RNG and other deterministic utilities

## Session discipline

- Read the latest `handoff.md` (game-level) or `TRACKER.md` (project-level) before starting work.
- End every substantive session with a handoff document for the game worked on.
- The innovation log is append-only. Amendments to the rubric bump its version; old scores never re-score.
- YAGNI aggressively. Don't build a skill until you've manually done its job twice and felt the friction.

## Anchor rule

Before any Phase 2 skill is built, Tigris & Euphrates must have a complete review in `games/0001-tigris-and-euphrates/` with all pipeline stages executed. This is the calibration lap — it's how we know the rubric and personas are doing real work.
