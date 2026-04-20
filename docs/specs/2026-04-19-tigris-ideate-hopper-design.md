---
name: TIGRIS Ideate-Hopper
slug: tigris-ideate-hopper-design
version: 1.0.0
rubric_version: v2.15
bet_version: parliament
author: TIGRIS (human + claude co-design)
created: 2026-04-19
updated: 2026-04-19
sources: []
---

# TIGRIS Ideate-Hopper — Design (v1.0)

A source-of-ideas system for TIGRIS original designs. Hybrid static pool + generator skill. Feeds the `/tigris-concept` pipeline.

## Purpose

After 17 reviews and 2 originals (UNFOLD, ZEN PATH), TIGRIS needs a disciplined way to generate candidate original-design ideas. The existing `/tigris-concept` skill turns an idea into a concept document — but until now, ideas have been produced ad-hoc (ZEN PATH came from a single user prompt: "chutes and ladders × samurai"). The hopper formalizes idea-generation.

**Non-goal:** the hopper does not replace human judgment. It produces candidates; the user promotes or retires them.

## Architecture

Moderate complexity. Five pool files + one skill. Each pool corresponds to one idea-source type.

```
ideas/
  README.md        — overview + usage
  primitives.md    — catalog of ~40-50 themes, mechanisms, table-dynamics
  mashups.md       — 20-30 seed pairs (theme × mechanism)
  gaps.md          — live TIGRIS gaps (axes/personas/BGG-domains)
  frustrations.md  — external frustration catalog (designer interviews, BGG threads)
  hopper.md        — generated candidates queue
  hopper-archive.md — retired entries after hopper > ~200
```

```
skills/tigris-ideate/
  SKILL.md        — the /tigris-ideate skill definition
```

## Pool schemas

### primitives.md

Hand-curated catalog of atomic design primitives. Each entry:

```markdown
### P-<NNN> — <short name>
- **Kind:** theme | mechanism | table-dynamic
- **Tag:** <short tag, e.g. "nature", "deck-building", "cascade">
- **One-liner:** <single sentence describing the primitive>
- **Examples:** <2-4 existing games that use it>
- **Notes:** <optional>
```

Seeded with ~40-50 entries across three kinds:
- **Themes** (~15): nature, space, history, abstract, horror, farming, exploration, legacy, economic, mythology, urban, maritime, underworld, cosmic, domestic.
- **Mechanisms** (~20): deck-building, worker placement, area control, tile laying, bidding, trick-taking, bag-building, roll-and-write, rondel, pick-up-and-deliver, engine-building, programming, auction, dice drafting, set collection, hidden roles, push-your-luck, route building, legacy, campaign.
- **Table-dynamics** (~10): simultaneous-action, asymmetry, variable-setup, solo-capable, team-based, traitor, escalating-pressure, lock-in, tableau-growth, compression-then-release.

### mashups.md

Seed pairs — pre-imagined theme × mechanism combinations. Each entry:

```markdown
### M-<NNN> — <pair slug>
- **A:** <primitive tag or freeform>
- **B:** <primitive tag or freeform>
- **Tension hypothesis:** <what argument makes this work or fail?>
- **Source:** curator | auto-generated | external
```

Seeded with ~20-30 pairs. Examples: "deck-building × conservation" / "roll-and-write × horror" / "rondel × cosmic" / "legacy × gardening" / "escalating-pressure × domestic".

### gaps.md

Live TIGRIS state snapshot. Manually refreshed from axis-pool, review record, BGG scan. Sections:

- **Axes underexplored** — adopted axes with few canonical anchors across persona diversity.
- **Personas without recent anchor** — designers who haven't anchored in last N games.
- **BGG domains missing** — theme clusters from BGG Top 100 that TIGRIS hasn't reviewed (horror, dexterity, party, legacy, etc.).
- **Mechanisms never-reviewed** — primitives with zero representation in games/0001-…/0017-*.
- **Adopted-contested-watch** — axes at risk of de-adoption (A3, A7 currently).

Each section is a list of one-liners tagged with current-state evidence.

### frustrations.md

External-source catalog, hand-curated. Each entry:

```markdown
### F-<NNN> — <short problem slug>
- **Problem:** <what's missing, broken, or unexplored>
- **Source:** <link or citation>
- **Date seen:** YYYY-MM-DD
- **Notes:** <why it's interesting>
```

Bootstrap with ~10-15 entries. Grows as the user encounters frustrations.

### hopper.md

Generator output queue. Each entry:

```markdown
### HOP-<NNN> — <working title>
- **One-liner:** <2-3 sentence pitch>
- **Anchor guess:** <persona>/<axis>
- **Sources:** <which source entries this candidate combined, with IDs>
- **Tension hypothesis:** <what argument would the panel have about this game?>
- **Status:** fresh | promoted | consumed | retired
- **Created:** YYYY-MM-DD
- **Updated:** YYYY-MM-DD
- **Notes:** <user notes; backlink to games/NNNN-<slug>/ if consumed>
```

## `/tigris-ideate` skill

### Invocation

```
/tigris-ideate [--count N] [--sources <list>] [--format brief|full] [--theme <tag>]
```

### Defaults

- `--count 3`
- `--sources mashup,gap,mining,frustration,primitives` (all 5)
- `--format brief` (3-sentence pitch)
- `--theme` — unset (no tag constraint)

### Behavior

1. **Read** pool files (primitives, mashups, gaps, frustrations), `personas/axis-pool.md` (for ledger state), `personas/playtest-innovations.md` (for mining), and existing `ideas/hopper.md` (for dedup).
2. **For each candidate:**
   - Select source(s) from `--sources` (uniform random across the chosen list; each candidate uses 1-2 sources).
   - **Mashup**: pick a pre-seeded pair from `mashups.md` (distinct from **Primitives** source, which creates fresh pairings from `primitives.md` atoms).
   - **Gap**: pick a gap from gaps.md; design to fill it.
   - **Mining**: pick an observational or candidate innovation from the log; turn the observation into a design prompt.
   - **Frustration**: pick a problem from frustrations.md; design to solve it.
   - **Primitives**: sample 2-3 primitives across kinds; invent a pair/mashup on the fly.
   - Generate: working title + one-liner + anchor guess (persona/axis) + tension hypothesis.
   - **Dedup**: compare against hopper.md recent entries (last 20); regenerate if near-duplicate by title OR same anchor+sources.
3. **Append** each candidate to `ideas/hopper.md` with `fresh` status and new HOP-NNN ID.
4. **Report** the list of generated IDs + one-liners to stdout.

### Write discipline

`/tigris-ideate` writes ONLY to `ideas/hopper.md`. Never modifies pools. Pools are stable knowledge base.

## Lifecycle

### States

| State | Meaning | How entered |
|---|---|---|
| fresh | generated, not yet reviewed | by `/tigris-ideate` |
| promoted | user flagged as interesting | manual edit of hopper.md |
| consumed | became a real game | automatic via `/tigris-concept HOP-NNN` |
| retired | rejected or stale | manual |

### Transitions

- `fresh → promoted | retired` — manual.
- `promoted → consumed` — automatic when `/tigris-concept HOP-NNN` runs.
- `promoted → retired` — manual.
- `consumed` — terminal.

### Integration with `/tigris-concept`

`/tigris-concept HOP-NNN`:
1. Reads hopper entry HOP-NNN.
2. Creates `games/NNNN-<slug>/concept.md` seeded from entry pitch + anchor guess.
3. Marks hopper entry `consumed`, adds backlink to `games/NNNN-<slug>/`.

(This requires a small edit to the existing `/tigris-concept` skill to accept a `HOP-NNN` argument.)

### Archive

When `hopper.md` exceeds ~200 entries, retired entries migrate to `ideas/hopper-archive.md` by manual housekeeping. YAGNI auto-archive.

## Initial bootstrap

As part of implementing this design:

1. Create `ideas/` directory.
2. Seed `primitives.md` with ~40-50 entries (hand-curated).
3. Seed `mashups.md` with ~20-30 pairs.
4. Seed `gaps.md` from current TIGRIS state (derive during bootstrap):
   - **Axes underexplored** — derive from `axis-pool.md` by counting distinct persona-anchors per axis; any adopted axis with only one primary advocate is underexplored.
   - **Personas without recent anchor** — derive from games 0001…0017 anchor history; personas who haven't anchored in last 5 games.
   - **BGG domains missing** — horror (Nemesis/Eldritch/Arkham), dexterity (Crokinole), party (Codenames), legacy (Pandemic Legacy), hidden-roles (BSG), abstract (Azul/Patchwork heavyweights).
   - **Mechanisms never-reviewed** — derive from game mechanics column of BGG Top 100 snapshot (Rakowski data) minus any mechanisms present in reviewed 0001…0017 games.
   - **Adopted-contested-watch** — A3 (multiplayer-solitaire), A7 (solved-strategy).
5. Seed `frustrations.md` with ~10-15 entries (user-curated; can start with TIGRIS-internal observations).
6. Write `ideas/README.md` with usage notes.
7. Create empty `ideas/hopper.md` with schema header.
8. Write `skills/tigris-ideate/SKILL.md`.
9. Edit `skills/tigris-concept/SKILL.md` to accept `HOP-NNN` argument.

## Success criteria for v1.0 release

- `/tigris-ideate` produces 3 coherent candidates on first run.
- Each candidate names a plausible anchor persona + axis.
- User can flag a `fresh` entry as `promoted` with a single text edit.
- `/tigris-concept HOP-NNN` converts a promoted entry into a game concept cleanly.
- Pool files stay stable; skill reads but does not write to them.

## Open questions

1. **Auto-refresh `gaps.md`**? Current plan: manual refresh. Future option: `/tigris-ideate --refresh-gaps` regenerates from axis-pool + innovations + BGG scan. Deferred.
2. **Rating system**? Current plan: none. Future option: promoted entries accumulate notes / half-page sketches. Deferred.
3. **Cross-hopper cadence**? Current plan: on-demand only. Future option: scheduled generation. YAGNI.
4. **Hopper-to-mashup feedback**? When a consumed game is interesting, does its mashup get promoted into mashups.md for re-use? Deferred; observational only.

## Non-goals

- Auto-designing games (no end-to-end automation from idea → concept.md without user review).
- Replacing `/tigris-concept` (this is upstream, not a replacement).
- Rating/ranking candidates (no quality signal; user judgment is the filter).
- External API integration (no auto-scraping of BGG/Kickstarter in v1.0).

## Changelog

- **v1.0.0** (2026-04-19) — initial design; hybrid 5-pool + 1-skill architecture adopted after brainstorming.
