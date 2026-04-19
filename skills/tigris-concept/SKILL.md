---
name: tigris-concept
description: Scaffold games/NNNN-<slug>/concept.md for a new original TIGRIS game. Collects premise, player count, length, anchor mechanic, artifact-as-story, and anchor-persona+anchor-axis declarations into a v2.1+ concept document ready for DESIGN. Use when starting a new original (not reviewing an existing game — use tigris-design review-mode for that).
---

# tigris-concept — scaffold a new original

## 60-second quickstart

```
You:    "Run tigris-concept for a new game called FACETS — 2-4p jewel-draft with
         different scoring functions per jewel type."
Claude: [determines next game number by scanning games/ dir]
        [creates games/0003-facets/ directory if not exists]
        [writes concept.md with v2.1 frontmatter + 12 sections]
        [prompts user (if any field ambiguous) or fills with sensible defaults]
        [reports: "games/0003-facets/concept.md written. Anchor declared: Feld on
         C6 Point-Salad Incommensurability. Next: /tigris-design for rules."]
```

## When to invoke

- User wants to start a new original TIGRIS game.
- The CONCEPT stage is the first pipeline step (before DESIGN).
- Do NOT use for reviews of existing games — those skip CONCEPT and start at DESIGN with published rules imported (see `/tigris-design` review-mode).

## Preconditions

- `personas/axis-pool.md` exists with current axes.
- `personas/designers/*.md` exist with `preferred_axes` fields.
- User has at least a rough premise — don't spend time inventing premises the user hasn't asked for.

## Procedure

### Step 1 — Determine game number

Scan `games/` directory. Pick next `NNNN` (e.g., 0003 if 0001 and 0002 exist). Create `games/NNNN-<slug>/` if not exists.

### Step 2 — Gather required inputs

From user input or reasonable inference:

- **Title** — human-readable name (e.g., "FACETS")
- **Slug** — lowercase hyphenated (e.g., "facets")
- **Player count range** — e.g., 2–4
- **Target length** — minutes
- **Weight** — 1–5 (BGG-style)
- **Anchor persona** — which designer-lens the game is centered on
- **Anchor axis** — which axis from the Pool the game claims to exemplify (this is the axis the GATE will check)
- **Artifact-as-story** — the physical centerpiece whose constraints drive design (deck, board, tile bag, token economy)
- **Inspiration / lineage** — honest list of what the design descends from

If user hasn't specified the anchor persona/axis: infer from the game's mechanical emphasis. If FACETS scores via 6 incommensurable functions, the natural anchor is Feld on C6 Point-Salad Incommensurability. Make the inference, report it, and let the user correct.

### Step 3 — Write concept.md

Use this template exactly. Fill fields from Step 2.

```yaml
---
name: <Title> — Concept
slug: <slug>-concept
game: NNNN-<slug>
stage: concept
version: 1.0.0
rubric_version: <current, e.g., v2.2>
bet_version: parliament
author: TIGRIS
created: <YYYY-MM-DD>
updated: <YYYY-MM-DD>
anchor_persona: <slug>
anchor_axis: <axis-slug>
---

# <Title> — Concept

## Premise
<~500-word description. Player-facing hook. What is the experience?>

## Players and length
- **Players:** <range>
- **Length:** <min>
- **Weight:** <1-5>

## Anchor mechanic
<The one system the whole game is built around. 1 paragraph.>

## Artifact-as-story
<What is the physical centerpiece? Why is it load-bearing for design?>

## Inspiration / lineage
- **<Designer (year)>** — <what it borrows>
- **<Designer (year)>** — <what it borrows>
- ...
<Be honest — call out direct borrowings. TIGRIS's architectural-novelty axis rewards load-bearing novelty, not claims of it.>

## Target review in the TIGRIS pipeline
<Which axes from the Pool do you expect to earn? Which to refute? Predictions here are testable at amendment.>

## Non-goals
- <what this game is NOT>
- <what scope it won't reach>

## Open questions (to resolve during DESIGN)
- <design questions that design.md will pin down>
- <calibration numbers to set in first play>
```

### Step 4 — Fill anchor-persona + anchor-axis fields

Check `personas/designers/<anchor_persona>.md` exists. Check axis is in `personas/axis-pool.md` Pool (or is a new axis candidate). If axis is queued-for-retirement, warn user — may be a risky anchor choice.

If anchor-axis lacks a drafted collision partner in the most likely TIER-A draft (simulate preferences), flag it — this is the `anchor_axis_without_collision_partner` pattern from I-parliament-03 (proposed-amendment as of v2.2).

### Step 5 — Report

Report to user:
- Path to concept.md
- Anchor declaration (persona + axis)
- Any warnings (queued-for-retirement axis, missing adjacency partner)
- Next step: run `/tigris-design` to author rules

## Invariants

- The anchor-persona + anchor-axis declared here are **locked** once TIER-A begins (per I-parliament-06 / A6 / the v2.0 spec). Re-declaration only permitted at GATE.
- CONCEPT is for originals only. Review-mode games (games/NNNN-<slug>/ for imported games) use `/tigris-design` directly with a short "imported review" note replacing CONCEPT's body.
- The artifact-as-story field is not decoration. If the game has no physical centerpiece with load-bearing constraints, note it explicitly — this flags the game as abstract/pure-mechanic.
- Inspiration honesty: list direct borrowings, don't pretend novelty. Architectural Novelty is scored against what survives a re-skin test, not against what the designer claims.

## What this skill does NOT do

- Write the rules — that's `/tigris-design`.
- Run TIER-A stakes — that's manual for now; use the Parliament and T&E examples as templates.
- Validate design against rubric — stakes and argument do that; CONCEPT is pre-evaluation.
- Create original artwork — markdown-only per spec §13 non-goals.
