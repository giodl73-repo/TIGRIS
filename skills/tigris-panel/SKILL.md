---
name: tigris-panel
description: Run a TIGRIS Parliament review — the three-phase adversarial procedure (Stakes / Argument / Amendment) that produces a review-as-game-record for a board game or a TIGRIS-internal artifact. Invoked after a game's DESIGN is complete or a spec is ready for scrutiny. Outputs: tierA-stakes.md, playtests/PT*-argument.md, panel/SUMMARY.md, amendments.md.
---

# tigris-panel — Parliament procedure

## 60-second quickstart

```
You:    "Run tigris-panel on games/0001-parliament/"
Claude: [reads design.md, axis-pool.md, personas/, forbidden-words.md]
        [dispatches 7 designer personas to DRAFT 3 axes each in snake order]
        [writes games/0001-parliament/tierA-stakes.md]
        [reports: "21 stakes drafted, GATE passed — 3 collision candidates identified"]
        [runs ARGUMENT phase: narrated playthrough with attacks/defends/collisions]
        [writes games/0001-parliament/playtests/PT01-argument.md]
        [runs AMENDMENT phase deterministically]
        [writes games/0001-parliament/amendments.md + updates axis-pool.md]
        [writes panel/SUMMARY.md]
        [reports: "review complete — 5 earned, 2 refuted, 1 collision, 1 axis adopted"]
```

First invocation of the session, Claude should also announce "using tigris-panel to run the Parliament review on <artifact>" before reading anything.

## What this skill does

Executes the three-phase Parliament procedure defined in `docs/specs/2026-04-19-tigris-v2.0-design.md` §8.3–8.6. Produces an argument-record, not a consensus document.

Two modes:

- **game-review** — artifact is under `games/NNNN-<slug>/`; full roster (7 designers + optional 5 lenses); output under `games/<slug>/`.
- **spec-review** — artifact is under `docs/specs/` or `personas/` or `skills/`; designers only (lenses don't parliament on processes); output under `docs/specs/reviews/<artifact-slug>/` with the same Parliament shape.

## Preconditions

- Artifact exists and has YAML frontmatter with `slug`, `stage`, `version`, `rubric_version: v2.0`.
- All 7 designer personas exist at `personas/designers/*.md` with `preferred_axes` field populated.
- `personas/axis-pool.md` exists and has 24 live-or-adopted axes.
- `personas/forbidden-words.md` exists (vocabulary discipline enforced at write-time).
- For originals: CONCEPT has declared `anchor_persona` and `anchor_axis`. For reviews: CONCEPT may declare these or skip (anchor-cell may be left null in spec-review mode).

## Procedure

### Step 1 — Load context

1. Read the artifact.
2. Read `personas/axis-pool.md` (current live axes, adopted list, retired list).
3. Read `personas/playtest-rubric.md` (the Parliament-shape rubric v2.0).
4. Read `personas/forbidden-words.md`.
5. Read `docs/specs/2026-04-19-tigris-v2.0-design.md` §8 for procedure details.
6. Determine mode (game-review vs spec-review) from artifact path.
7. Create output directory.

### Step 2 — TIER-A: STAKES (draft phase)

1. Determine seating: 7 designer personas (spec-review) or 7 designers + optional 5 lenses (game-review).
2. Establish draft order:
   - Round 1: user-specified order, usually "anchor persona last" (forces them onto shared ground).
   - Round 2: reverse of round 1.
   - Round 3: same as round 1.
3. For each draft slot, dispatch a mini-task:
   - Persona reads their preferred_axes list.
   - Persona reads the Pool's current state (live / adopted / retired).
   - Persona picks one axis not already drafted this game.
   - A persona may pick a retired axis at cost of 2 draft-slots (skip their next draft).
   - Record draft in `tierA-stakes.md`.
4. After drafts complete, each persona stakes their 3 axes:
   - For each target player count of the design, assign score 0–10 with a rule-grounded one-sentence justification.
   - Record in persona's section of `tierA-stakes.md`.
5. Publish `tierA-stakes.md`.

### Step 3 — GATE check

Verify all three GATE conditions from `personas/playtest-rubric.md`:

1. Anchor stake viable? (If declared; otherwise mark N/A for spec-reviews.)
2. Draft coverage ≥ 3 bands of the 4 (A/B/C/D)?
3. ≥ 1 collision candidate identifiable?

If pass: proceed to Step 4. If fail: return to user with specific failure mode and a re-draft proposal.

### Step 4 — TIER-B: ARGUMENT (narrated playthrough)

1. For games: seeded-RNG narrated playthrough covering 2p / 3p / 4p serially. For specs: walk the spec section-by-section with personas commenting.
2. At each pre-declared moment (start, turn 1 / §1, turn 5 / §5, mid, late, end):
   - Each persona reviews their stake(s) applicable at this moment.
   - Each persona may declare: **hold**, **attack <other>**, **defend**, or **collide-with <other>**.
   - Attacks require cited evidence; defenses require cited evidence; collisions require adjacency argument.
3. Record every action in `playtests/PT<NN>-argument.md` with moment-anchors (turn numbers or section numbers).
4. At game end, classify each stake: earned / contested / refuted / ignored.

### Step 5 — TIER-C: AMENDMENT (deterministic pass)

1. Read `personas/axis-pool.md` Rubric Ledger.
2. For each stake in this game, update ledger row:
   - +1 to earned / contested / refuted / ignored count on that axis.
3. Check adoption triggers (≥ 2 earned across ≥ 2 games):
   - Mark axis `adopted`.
   - Bump rubric version in `personas/playtest-rubric.md` (v2.0 → v2.1).
4. Check retirement triggers (≥ 2 refuted across ≥ 2 games):
   - Mark axis `retired`.
5. Write `games/<slug>/amendments.md` with the full change record.
6. Log new innovations to `personas/playtest-innovations.md` (include `trigger_pattern` field).

### Step 6 — Synthesize SUMMARY.md

Write `panel/SUMMARY.md` with:

```yaml
---
name: Parliament Summary — <artifact>
slug: SUMMARY
stage: panel
version: 1.0.0
rubric_version: v2.0
bet_version: parliament
artifact_under_review: <path>
personas_seated: [list]
total_stakes: <N>
earned: <N> / refuted: <N> / contested: <N> / ignored: <N>
collisions: <N>
axes_adopted: [list]
axes_retired: [list]
created: <today>
---
```

**Body structure (lean — this is an argument record, not an essay):**

1. **Headline** — one sentence naming the argument's shape.
2. **Earned stakes** — table: persona, axis, count, score, defending moment.
3. **Refuted stakes** — table: persona, axis, count, score, refuting moment.
4. **Collisions** — each collision with both personas, both axes, resolving moment.
5. **Ignored stakes** — flagged as silent failures with persona names (persona-retirement candidates if repeated).
6. **Amendment outcome** — axes adopted, axes retired, rubric version change.
7. **Success criteria check** — did the review meet `playtest-rubric.md`'s 4 success criteria?

### Step 7 — Update TRACKER.md

Append a row:

| Artifact | Status | Stage | Rubric ver | Update | Earned / Refuted / Collisions | Notes |
|---|---|---|---|---|---|---|
| <path> | parliament-complete | TIER-C | v2.x | <today> | N/N/N | <headline> |

### Step 8 — Hand off

Report to user:
- Paths to all output artifacts.
- Success criteria check result.
- Total earned / refuted / ignored / collision counts.
- Axes adopted or retired.
- Next step: HANDOFF (if game) or spec amendment integration (if spec-review).

## Invariants

- Stakes are never averaged. Aggregate scores are banned.
- Every attack, defense, and collision cites a specific moment (turn number, section number) and a specific rule or mechanic.
- Ignored stakes are reported, not hidden. They are as meaningful as earned stakes.
- Axis adoption and retirement are deterministic from the ledger — no user approval for the default path.
- Reviews are immutable once written. A revision creates a new version directory.
- Forbidden-words compliance is enforced at stake-write time, not post-hoc.

## Mode differences

|  | game-review | spec-review |
|---|---|---|
| Designer personas | 7 | 7 |
| Player lenses | 5 (optional) | 0 |
| Per-count scoring | 2p/3p/4p | N/A (single aggregate at moment-anchors) |
| Anchor stake declared | required (from CONCEPT) | optional |
| Output path | `games/<slug>/` | `docs/specs/reviews/<slug>/` |
| Feeds into | HANDOFF → next game | rubric amendment / spec v+1 |

## Example invocation

> "Run tigris-panel on games/0001-parliament/"

Expected output:
```
games/0001-parliament/
├── tierA-stakes.md          # 21 stakes, 3 per persona
├── playtests/
│   └── PT01-argument.md     # argument record with moment-anchors
├── amendments.md            # what this game adopted/retired
├── panel/
│   └── SUMMARY.md           # lean argument record
└── handoff.md               # (written by a later HANDOFF step)
```

Plus updates to:
- `personas/axis-pool.md` — ledger updated
- `personas/playtest-rubric.md` — version bumped if adoption
- `personas/playtest-innovations.md` — new innovations appended
- `TRACKER.md` — new row

## What this skill does NOT do

- Invoke HANDOFF — that's a separate step.
- Re-score prior games on amendment — forward-only versioning means prior scores are locked.
- Approve or reject the artifact — the output is evidence; user decides.
- Generate physical components or renderings — Parliament's design doc is markdown-only for v2.0.
