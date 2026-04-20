# TIGRIS — House Rules for Claude (v2.0)

A Claude-Code-driven factory for Euro-tradition board games. Designs new games and reviews existing ones through the same pipeline. Sibling to marathon (D&D), puzzlehunt, chronicle (LUCIA), rmm, artisan (CERES).

**Source of truth:** `docs/specs/2026-04-19-tigris-v2.0-design.md` (the Parliament restructure). Read before doing anything non-trivial.

**v1.0 spec** at `docs/specs/2026-04-19-tigris-design.md` is superseded. v1.0 scores are locked and not re-scored against v2.0.

## Session resume — "lets continue"

When the user says **"lets continue"**, **"continue"**, **"resume"**, **"pick up where we left off"**, or any equivalent, do this before anything else:

1. **Read the latest portfolio handoff** — newest file in `docs/handoff/` (sort by filename; filenames are dated `YYYY-MM-DD-<slug>.md`). This is the authoritative resume point. It snapshots the full factory state: Pool health, recent decisions, hopper, open items, and **ranked Next-session priorities**.
2. **Skim the most recent per-game handoff** — `games/<highest-NNNN>/handoff.md`. This adds per-game context the portfolio handoff can't carry.
3. **Confirm Pool state hasn't drifted** — glance at `personas/axis-pool.md` Changelog top entry and `TRACKER.md` top row. If they match what the portfolio handoff claims is current, state is stable.
4. **Report to user**: one sentence naming the current rubric version + most recent game + the top 1-3 Next-session priorities from the handoff. Ask the user to confirm which priority to start on (or `go` / `1` for the top one).
5. **Do NOT re-read CLAUDE.md**, TRACKER.md in full, axis-pool.md in full, or individual game files until the chosen priority requires them. The handoff is designed to carry the context.

End of every substantive session: run `/tigris-handoff <slug>` to create a fresh resume point **before context is cleared**. This is part of session discipline.

## The architectural bet

**TIGRIS is an adversarial review system.** Designer personas don't converge on consensus — they plant incompatible stakes from a shared Pool and the factory's output is the record of their argument. Amendments to the rubric are *won*, not proposed.

Three consequences:
- No shared rubric at scoring time. Each persona drafts 3 axes per game from a 24-axis Pool (`personas/axis-pool.md`).
- Three phases are cognitively distinct: **STAKES** (drafting+assertion) → **ARGUMENT** (attack/defend/collide under narrated play) → **AMENDMENT** (deterministic adoption/retirement).
- The rubric evolves via play record, not user approval.

## The pipeline

```
CONCEPT → DESIGN → TIER-A [STAKES] → [GATE] → TIER-B [ARGUMENT] → TIER-C [AMENDMENT] → HANDOFF
         (skip for reviews)          ↑
                                     └─ gate fail → revise DESIGN or re-draft
```

Reviews of existing games start at TIER-A (DESIGN imports published rules). Reviews of TIGRIS-internal artifacts (specs, skills, persona additions) run the same Parliament procedure with designers only (no player lenses on processes).

## Frontmatter contract (v2.6)

Every generated file:

```yaml
---
name:
slug:
game: <NNNN-slug>                 # omit for non-game artifacts
stage: concept|design|tierA|tierB|tierC|handoff|panel
version: <semver>
rubric_version: v2.6
bet_version: parliament
author: <persona-slug or human>
created: YYYY-MM-DD
updated: YYYY-MM-DD
sources: []

# Review-mode games only (imported published games):
bgg_id: <integer>                 # REQUIRED for review-mode; see reference/bgg.md
bgg_url: https://boardgamegeek.com/boardgame/<id>
canonical_edition: <edition>
game_designer: <name>
game_first_published: <year> (<publisher>)
review_type: imported-review
---
```

**BGG is the TIGRIS bible for published games.** Every review-mode game carries a `bgg_id`. Full reference + IDs for ~50 candidate games in `reference/bgg.md`. Originals designed by TIGRIS omit `bgg_id` until published.

## Forbidden vocabulary

See `personas/forbidden-words.md`. No "fun", "well-designed", "solid", "great game". Every claim names (a) axis, (b) persona, (c) player count or moment-anchor.

**v2.0 additions:** Stakes are described only as *earned*, *refuted*, *contested*, *ignored*, or *collided*. "Consensus" as an evaluative frame is banned — the factory produces well-formed disagreement.

## Rubric (v2.0)

**24 axes in a Pool**, 4 bands (A/B/C/D). See `personas/axis-pool.md` for full definitions. Each persona has a `preferred_axes` list in their frontmatter; drafting forces contention where preferences overlap.

No weighted aggregate. Stakes are individual assertions attacked or defended one at a time.

| Band | Theme | Count |
|---|---|---|
| A | Carried from v1.0 | 7 |
| B | Euro-specific (System Gearing, Catastrophe Pressure, etc.) | 5 |
| C | Persona-signature (Tension Budget, Scarcity Bite, etc.) | 8 |
| D | Family / accessibility | 4 |

## Personas

**8 designers** (v2.4): Knizia, Rosenberg, Feld, Lacerda, Chvátil, Kramer-Kiesling, Stegmaier, Vaccarino.
5 player lenses: Engine-Builder, Interactionist, AP-Prone, Thematic, Fresh-Face.

Each persona's `preferred_axes` (in frontmatter) lists 6-8 ranked axes they'd draft from the Pool. Overlaps between persona preferences are designed — Knizia and K-K both want Elegance; they can't both get it.

With 8 designers × 3 drafts = 24 drafts vs 24-axis Pool, every axis gets advocated every game (the never-drafted pattern from games 1-4 is closed by Vaccarino's A4/A5/B3 coverage).

## Directory map (v2.0)

- `docs/specs/` — spec docs (v1.0 archival, v2.0 current, protocol amendments)
- `docs/specs/reviews/` — panel reviews of TIGRIS-internal artifacts
- `docs/handoff/` — **portfolio-level session handoffs** (`YYYY-MM-DD-<slug>.md`). Latest file is the session resume point.
- `games/NNNN-<slug>/` — one directory per game (Parliament is 0001). Each has its own per-game `handoff.md`.
- `personas/`
  - `axis-pool.md` — the 24-axis Pool + Rubric Ledger (forward-only) + Retirement-Reversal log
  - `playtest-rubric.md` — Parliament-shape mechanics (stakes/argument/amendment)
  - `playtest-innovations.md` — append-only, `trigger_pattern` field
  - `forbidden-words.md` — vocabulary discipline
  - `designers/` + `player-lenses/` — persona files with preferred_axes
- `ideas/` — ideate-hopper system (primitives / mashups / gaps / frustrations / hopper; see `ideas/README.md`)
- `.claude/skills/` — Claude Code skills: `tigris-concept`, `tigris-design`, `tigris-panel`, `tigris-amendment`, `tigris-ideate`, `tigris-handoff`. Must live here (not at repo root) for CC discovery.
- `reference/` — cached rules, designer research
- `scripts/` — seeded RNG

## Session discipline

- **Session start**: if user says "lets continue" / "resume" / equivalent, follow the **Session resume** section above (read latest `docs/handoff/` first). Otherwise, read `TRACKER.md` or the relevant artifact for the explicitly-named task.
- **Session end**: run `/tigris-handoff <slug>` to snapshot portfolio state to `docs/handoff/YYYY-MM-DD-<slug>.md`. This is the resume point for the next session.
- **Per-game end**: every substantive game pipeline gets its own `games/NNNN-<slug>/handoff.md` inline with the review.
- Innovation log is append-only. Amendments to axes are deterministic (argument record evidence).
- **YAGNI aggressively.** Don't build a skill until you've done its job twice manually and felt friction.

## Anchor rule (v2.0)

Before any Phase 2 skill is built, **Parliament** (`games/0001-parliament/`) must have a complete Parliament-procedure review with all pipeline stages executed. Parliament is both:
- The first game the factory produces (originals path).
- The first review the factory conducts on itself (reviews path — Parliament reviews Parliament).

T&E (`games/0002-tigris-and-euphrates/`) becomes anchor #2 once Parliament passes.

## Success criteria for a review (per game)

From `personas/playtest-rubric.md`:
- ≥ 5 earned stakes across the panel
- ≥ 2 earned on Band B or C axes
- ≥ 1 clean collision resolution
- ≥ 1 axis promoted (adopted) OR retired

Fewer than 5 earned = personas aren't arguing. No Band B/C earnings = personas read as interchangeable. No collisions = argument isn't live. No promotion/retirement = rubric isn't evolving. Any of the four failing is a v2.1 blocker.

## Sibling-repo glossary

Every term inherited from marathon / puzzlehunt / chronicle / rmm / artisan is defined at first mention in the v2.0 spec §3. If writing a new doc that introduces a sibling-inherited term, add its definition to the glossary in the v2.0 spec.
