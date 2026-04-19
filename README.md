# TIGRIS

**A board game factory.** Named for Tigris & Euphrates (Knizia, 1997). Sibling to marathon, puzzlehunt, chronicle, rmm, artisan.

Claude Code-driven. Markdown-first. Euro tradition. Produces original game designs **and** reviews of existing games through a single pipeline. Evolves its own design guidelines via an append-only innovation log.

## What's here

- **`docs/specs/`** — design docs. Start with `2026-04-19-tigris-design.md` (the blueprint).
- **`personas/`** — 7 designer voices (Knizia, Rosenberg, Feld, Lacerda, Chvátil, Kramer-Kiesling, Stegmaier) + 5 player lenses + the rubric + the innovation log.
- **`games/`** — one numbered directory per game (originals or reviews). First resident: `0001-tigris-and-euphrates/` (the anchor review).
- **`skills/`** — Claude Code skills that execute the pipeline.
- **`CLAUDE.md`** — house rules: frontmatter contract, forbidden vocabulary, pipeline map.

## The pipeline

Every game, new or existing, runs the same seven stages:

```
CONCEPT → DESIGN → TIER-A → [GATE] → TIER-B → PANEL → INNOVATE → HANDOFF
```

- **Tier A** is a fast rubric-scored matrix (persona × player-count × axis). Cheap, every game.
- **Tier B** is a seeded narrated playthrough. After Tier A passes gate.
- **Tier C** is a multi-seat agent tournament. Commissioned only; not part of the default cycle.

Reviews of published games start at TIER-A with the published rules in `design.md`.

## Status

First commit 2026-04-19. Currently scaffolding personas + panel skill, then running the first panel review (on the TIGRIS spec itself — eating our own dog food before shipping the T&E anchor).

See `TRACKER.md` for the current pipeline state.
