---
name: tigris-panel
description: Run the TIGRIS panel review on a game (full designer roster + player lenses) or on a spec/process artifact (designer roster only). Produces one review file per persona + a weighted SUMMARY.md. Use when a game has passed Tier-A GATE and is ready for panel, or when reviewing a TIGRIS-internal artifact (spec, rubric change, skill).
---

# tigris-panel

Execute a panel review on an artifact. Two modes:

- **game-review** — for a game under `games/NNNN-<slug>/`. Invokes the full roster: 7 designer personas + 5 player lenses. Outputs to `games/NNNN-<slug>/panel/`.
- **spec-review** — for a TIGRIS-internal artifact (spec doc, rubric amendment, skill proposal). Invokes designer personas only (player lenses review *games*, not processes). Outputs to `docs/specs/reviews/<artifact-slug>/`.

## When to invoke

- A game has produced a PASS verdict in `tierA-sim.md` and moved past GATE → `game-review`.
- An original TIGRIS design doc (spec, amendment, new skill) is written and wants panel scrutiny before adoption → `spec-review`.
- Never invoke on a draft that hasn't been committed — the review must attach to a specific version.

## Preconditions

- The artifact exists and has YAML frontmatter with `slug`, `stage`, `version`, `rubric_version`.
- The seven designer personas exist at `personas/designers/*.md`.
- For game-review, the five player lenses exist at `personas/player-lenses/*.md`.
- The rubric matching `rubric_version` is the current v1.0 (or whatever is in `personas/playtest-rubric.md`).
- The forbidden-words guide exists at `personas/forbidden-words.md`.

## Procedure

### Step 1 — Load artifact + context

1. Read the full artifact.
2. Read `personas/playtest-rubric.md` (the 8-axis rubric + verdict thresholds).
3. Read `personas/forbidden-words.md` (required vocabulary discipline).
4. Determine mode:
   - If artifact path starts with `games/`, mode = `game-review`.
   - If artifact path starts with `docs/specs/`, `personas/`, or `skills/`, mode = `spec-review`.
5. Create output directory:
   - `game-review` → `games/<slug>/panel/` and `games/<slug>/panel/lenses/`
   - `spec-review` → `docs/specs/reviews/<artifact-slug>/`

### Step 2 — Dispatch reviewers

Dispatch one agent per persona, in parallel when the model/runtime allows. Each agent receives:

- Full artifact text
- That persona's `.md` file from `personas/designers/` or `personas/player-lenses/`
- The rubric
- The forbidden-words guide
- Target output path (one file per reviewer)

Each agent produces **one review file** named `<persona-slug>.md`:

```yaml
---
name: <Persona Name> review of <artifact name>
slug: <persona-slug>
stage: panel
version: 1.0.0
rubric_version: v1.0
author: <persona-slug>
artifact_under_review: <path to artifact>
created: <today>
updated: <today>
---
```

**Body structure** (800–1500 words):

1. **Opening verdict** (1 paragraph, in-voice) — what this persona thinks about the artifact, headline only.
2. **Three greenlights** — what the artifact does well, axis-grounded, in-voice. Forbidden vocabulary disallowed.
3. **Three red flags** — what the artifact does poorly, axis-grounded, in-voice.
4. **Amendment candidates** — what does this artifact expose as under-specified in the rubric, personas, or process? (Cluster bait for `/tigris-innovate`.)
5. **Scoring** — per the 8-axis rubric, each axis 0–10 with a one-sentence justification each. Include the persona's weighted aggregate at the bottom.

For `game-review` only: reviews are evaluated at each relevant player count; each persona reports a cell verdict per count.

For `spec-review`: single aggregate only — no player-count axis. Scoring focuses on whether the artifact is a good *process* design, not a good game.

### Step 3 — Synthesize SUMMARY.md

After all reviewers finish, write `SUMMARY.md` in the same panel directory:

```yaml
---
name: Panel Summary — <artifact name>
slug: SUMMARY
stage: panel
version: 1.0.0
rubric_version: v1.0
author: TIGRIS synthesis
artifact_under_review: <path>
reviewers: [list of persona-slugs]
created: <today>
updated: <today>
---
```

**Body:**

1. **Panel verdict** — one paragraph. Consensus position across reviewers, including prominent dissents. Name the dissenting persona and why.
2. **Per-axis consensus table** — for each of the 8 rubric axes, show each reviewer's score and compute a simple average (unweighted for SUMMARY; weighted aggregates remain in individual reviews). Flag axes where max-min spread > 4 as "contested."
3. **Greenlights consensus** — greenlights that appeared in ≥ 2 reviews.
4. **Red flags consensus** — red flags that appeared in ≥ 2 reviews. These are the must-fix items.
5. **Amendment candidates** — innovation log entries proposed. For each, note dimension, scope, and supporting reviewers. This section feeds directly into `/tigris-innovate`.
6. **Punchlist** — ordered list of items to address before the artifact is considered "panel-approved." Cite the reviewer behind each item.
7. **Next step** — what stage comes after this panel (INNOVATE → HANDOFF for games; AMEND → RATIFY for specs).

### Step 4 — Update TRACKER.md

Append a row to `TRACKER.md` with the review outcome:

| Artifact | Status | Stage | Rubric ver | Last update | Notes |
|---|---|---|---|---|---|
| <artifact> | panel-reviewed | PANEL | v1.0 | <today> | Consensus verdict + count of amendments proposed |

### Step 5 — Hand off

Report to the user:
- Where the panel output lives (absolute paths)
- The verdict headline
- The top 3 punchlist items
- The number of amendment candidates generated (for `/tigris-innovate` to cluster-check)

Do NOT invoke `/tigris-innovate` as part of this skill — innovation is its own stage, with its own deliberate pacing.

## Invariants

- Each review is written in the persona's voice. If a review reads like generic criticism, the reviewer agent wasn't properly briefed.
- Every claim names axis, persona, and (for games) player count. Forbidden-words compliance is enforced at review-write time, not post-hoc.
- Scores are **justified**, not asserted. A score without a one-sentence justification is rejected.
- SUMMARY.md never invents consensus that isn't there. A 4-axis spread of 3–9 is flagged as contested, not averaged quietly.
- Reviews are immutable once written. A revision creates a new version (`<persona>-v2.md`) — the old review stays as history.
- The tigris-panel skill does not approve or reject artifacts. It produces evidence. Approval/rejection is a separate user decision informed by the panel output.

## Mode differences quick reference

|  | game-review | spec-review |
|---|---|---|
| Designer personas | yes (7) | yes (7) |
| Player lenses | yes (5) | no |
| Per-player-count scoring | yes | no |
| Output path | `games/<slug>/panel/` | `docs/specs/reviews/<slug>/` |
| Feeds into | `/tigris-innovate` + `/tigris-handoff` | `/tigris-innovate` + rubric amendment process |

## Example invocation (informal)

> "Run tigris-panel on `docs/specs/2026-04-19-tigris-design.md`."

Expected output structure:
```
docs/specs/reviews/2026-04-19-tigris-design/
├── knizia.md
├── rosenberg.md
├── feld.md
├── lacerda.md
├── chvatil.md
├── kramer-kiesling.md
├── stegmaier.md
└── SUMMARY.md
```
