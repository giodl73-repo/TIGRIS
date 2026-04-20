---
name: tigris-handoff
description: Write a portfolio-level TIGRIS handoff document capturing overall program state — decisions, investments, open items, next-session priorities. Snapshots axis-pool + TRACKER + innovations + hopper into a single resume point. Use at session end, before clearing context, or at milestone checkpoints (rubric-version bumps, contested-watch transitions, protocol amendments). Distinct from per-game handoff.md files (those wrap single-game pipelines).
---

# tigris-handoff — portfolio-level handoff

## 60-second quickstart

```
You:    "/tigris-handoff post-rewild"
Claude: [reads personas/axis-pool.md, personas/playtest-innovations.md,
         TRACKER.md, ideas/hopper.md, ideas/gaps.md]
        [snapshots current state into a single handoff doc]
        [writes docs/handoff/<today>-<slug>.md]
        [reports: "Handoff written. Clean resume point at commit <sha>."]
```

## When to invoke

- **Session end** — before clearing context, write a handoff so the next session resumes cleanly.
- **Milestone checkpoint** — rubric version bump (major), contested-watch transitions, protocol amendments.
- **Stakeholder handoff** — writing a resume point for someone else to continue the factory work.
- **Before risky / exploratory work** — create a baseline before non-linear changes.

Do NOT use for per-game handoff (that's inline in each game's pipeline as `games/NNNN-<slug>/handoff.md`).

## Preconditions

- `personas/axis-pool.md` exists with current ledger + Retirement-Reversal log.
- `TRACKER.md` exists with Games + Session log sections.
- `personas/playtest-innovations.md` exists.
- `ideas/hopper.md` exists.
- At least 1 game has been reviewed (otherwise there's no state to snapshot).

## Procedure

### Step 1 — Read state files

- `personas/axis-pool.md` — Pool ledger + changelog + Retirement-Reversal log
- `personas/playtest-innovations.md` — recent innovations log
- `TRACKER.md` — session log + rubric versions
- `ideas/hopper.md` — open candidates
- `ideas/gaps.md` — corpus/coverage gaps
- `games/<latest>/handoff.md` — most recent game handoff (for continuity)
- `CLAUDE.md` — current house rules

### Step 2 — Determine handoff slug

User supplies slug argument (e.g., "post-rewild"). If omitted, generate from: `post-<latest-game-slug>` OR `milestone-v<current-rubric>`.

Filename: `docs/handoff/YYYY-MM-DD-<slug>.md`. Date is today's.

### Step 3 — Write handoff document

Use this template. Each section required.

```yaml
---
name: TIGRIS Handoff — <slug>
slug: handoff-<slug>
stage: handoff
version: 1.0.0
rubric_version: <current>
bet_version: parliament
author: TIGRIS
created: <today>
updated: <today>
type: portfolio-handoff
covers_through_game: <latest game #>
covers_through_rubric: <current rubric>
---

# TIGRIS Handoff — <slug>

## Headline state

One-paragraph snapshot. Rubric version, game count, Pool health (adopted / contested / retired), silent-retire streak, protocol amendments count, contested-watch status, most recent game.

## Decisions & investments (this period)

Bullet list of load-bearing decisions made since last handoff (or since project start if first handoff). Include:
- **Rubric amendments** — protocol changes, axis adoption/retirement, retirement-reversal events.
- **New designs** — originals produced (with anchors); reviews run (with anchors).
- **New skills / infrastructure** — skills added, directory structures created.
- **Pool dynamics** — contested-watch transitions, new adjacencies, counter-pressure events.

For each: name, date, primary evidence (game or commit), current status.

## Pool state snapshot (table)

Full table: each of 25 axes × adopted/retired/contested status × earned/refuted/contested/ignored counts × note.

## Hopper state

Current fresh / promoted / consumed / retired HOP-NNN entries. Recommended next consumption candidates.

## Corpus coverage (BGG domains + mechanisms)

What's covered; what's underweight; what's absent.

## Open items (requires next-session attention)

- **Pending amendments** — candidates for next rate-limit cycle.
- **Queued retirement-reversal triggers** — axes awaiting counter-pressure earnings.
- **Gaps.md refresh due?** — is gaps.md stale vs current state?
- **Unstarted infrastructure** — skills proposed but not built; dead-letter paths.

## Next-session priorities

Ranked list. 3-7 items. For each: description + estimated scope + current blockers.

## Resume-point instructions

1. Read this handoff first.
2. Skim CLAUDE.md for house rules.
3. Read most recent `games/<latest>/handoff.md` for per-game context.
4. Check current rubric version + axis-pool.md state.
5. Review ideas/hopper.md for candidate originals.
6. Proceed per Next-session priorities.

## Full state (for archival / audit)

- Rubric versions and their changelog entries (append last 5-10 from axis-pool.md Changelog).
- Recent session log (last 5-10 rows from TRACKER.md).
- Innovation log summary (last 10-15 innovations by cycle).
```

### Step 4 — Commit

Commit the handoff doc as a standalone commit. Message format:
```
TIGRIS handoff — <slug>

Portfolio-level snapshot at rubric <v2.19> / game #<21>.
Resume point for post-<slug> sessions.
```

### Step 5 — Report

Report to user:
- Path to handoff doc.
- Commit SHA.
- Brief summary of what's in it (1-2 sentences).
- Suggest clearing context (new session; use this handoff as resume point).

## Invariants

- **Portfolio-scope, not per-game scope.** Per-game handoff stays in games/NNNN-<slug>/handoff.md.
- **Snapshot, not narrative.** Handoff doc describes *state*, not *story*. Narrative lives in individual game/review artifacts.
- **Immutable once written.** If state changes, write a new dated handoff — don't edit the old one.
- **Self-contained.** Reader should be able to pick up TIGRIS from the handoff + CLAUDE.md alone.
- **Honest about open items.** Don't gloss over pending / blocked work.

## Mode differences

None. Single mode: portfolio-level snapshot.

## Example invocation

> "/tigris-handoff post-rewild"

Expected output: `docs/handoff/2026-04-20-post-rewild.md` + committed.

## What this skill does NOT do

- Invoke any other pipeline stage (CONCEPT / DESIGN / TIER-A-B-C).
- Modify pool ledgers or innovation log (read-only access).
- Close or retire hopper entries.
- Make protocol amendments (that's via spec review + amendment).
- Generate narrative — the handoff is a state dump, not a story.

## See also

- Per-game handoff templates: any `games/NNNN-<slug>/handoff.md`.
- Session log source: `TRACKER.md`.
- Pool ledger source: `personas/axis-pool.md`.
- Hopper candidates: `ideas/hopper.md`.
