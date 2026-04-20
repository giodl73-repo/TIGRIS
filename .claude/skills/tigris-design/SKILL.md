---
name: tigris-design
description: Scaffold games/NNNN-<slug>/design.md — the full rulebook-shaped document for a new original (original-mode) or imported rules summary for a review (review-mode). Produces a v2.1+ design doc ready for TIGRIS TIER-A STAKES.
---

# tigris-design — author rules

## 60-second quickstart

```
You:    "Run tigris-design for games/0003-facets in original-mode"
Claude: [reads games/0003-facets/concept.md]
        [reads personas/axis-pool.md for scoring-function inspiration]
        [writes games/0003-facets/design.md with 12 sections]
        [checks rules cover: overview, components, setup, turn, actions,
         scoring, edge cases, open questions]
        [reports: "design.md written. Ready for TIER-A STAKES. 6 scoring
         functions described; 1 open question flagged for first-play."]
```

For reviews:
```
You:    "Run tigris-design for games/0002-tigris-and-euphrates in review-mode
         with source: FFG 2015 rulebook"
Claude: [creates design.md with citation header + 15-section rules summary
         + citations block + fair-use posture note]
```

## When to invoke

- **Original-mode:** CONCEPT is complete; author full rules.
- **Review-mode:** new game is a review of a published design; import and summarize rules for the panel to evaluate.

## Preconditions

- For original-mode: `games/NNNN-<slug>/concept.md` exists with anchor declarations.
- For review-mode: the game being reviewed is publicly released with available rules sources.
- `personas/axis-pool.md` exists (used to inform design-vocabulary choices).

## Procedure — Original-mode

### Step 1 — Read CONCEPT

Read `games/NNNN-<slug>/concept.md`. Extract:
- Title, slug, player count, length, weight
- Anchor mechanic (informs §4 Turn and §5 Actions)
- Artifact-as-story (informs §2 Components)
- Target axes (informs what rules should make mechanically legible)

### Step 2 — Write design.md

Use this template. Each section required unless marked optional.

```yaml
---
name: <Title> — Design (Rules)
slug: <slug>-design
game: NNNN-<slug>
stage: design
version: 1.0.0
rubric_version: <current>
bet_version: parliament
author: TIGRIS
created: <YYYY-MM-DD>
updated: <YYYY-MM-DD>
anchor_persona: <inherited from concept>
anchor_axis: <inherited from concept>
---

# <Title> — Rules

## 1. Overview & Object of the Game
<The experience in 2-4 paragraphs. Name the winning condition and the core tension.>

## 2. Components
<Tactile list. Counts per type. Physical properties that matter for play.>

## 3. Setup
<Step-by-step. Includes persistent state for legacy-lite games.>

## 4. Turn structure
<The turn's spine. Use numbered sub-phases. This is the §4 every rubric reviewer will read first.>

## 5. Actions
<Action menu. Action-Menu Clarity axis (C7) is scored against this section. Target ≤6 actions.>

## 6. Scoring / end condition
<Exact scoring math. End-trigger details. Minimum-Score Shape axis (C2) is scored against this section.>

## 7. Edge cases & clarifications
<Non-obvious rule interactions. Ties. Can X happen with Y?>

## 8. [Optional] Special subsystems
<Monuments, catastrophes, treasures, legacy mechanics — anything that doesn't fit §5 Actions.>

## 9. [Optional] Variants
<Count-specific variants, tournament variants, solo-mode if any.>

## 10. [Optional] Designer notes
<Why specific choices were made. Helps reviewers understand the ambition.>

## 11. [Original-mode] Collision Adjacency Chart
<If this game's mechanics create novel axis-adjacency pairs, document them here. Parliament's §9 is the template. Feed candidates to I-te-01 for possible pool-wide adjacency updates.>

## 12. Open at DESIGN-stage, to resolve post-review
<Calibration numbers. Mechanisms the panel is expected to challenge. Honesty here protects reviews from irrelevant nitpicks.>
```

### Step 3 — Sanity-check coverage

Before reporting:
- Does §4 name 2-6 action types? (Action-Menu Clarity target.)
- Does §6 specify end-trigger AND scoring? (Both required for Min-Score Shape earning.)
- Does §5 or §8 cover every piece in §2? (Component-rule coverage.)
- Are all CONCEPT anchor declarations reflected in the rules? (Anchor coherence.)

If any sanity check fails, flag in report. User may want to extend before TIER-A.

### Step 4 — Report

Report: path to design.md, section count, any flagged gaps, next step (TIER-A STAKES).

## Procedure — Review-mode

### Step 1 — Gather source material

User specifies the canonical rules source (edition, publisher, year). Acceptable sources:
- Published rulebook (cite edition)
- BGG pages, designer interviews, community FAQ
- Academic / journalistic commentary (e.g., Parlett's *Oxford History of Board Games*)

**Fair-use posture:** rules summary is critical commentary; no verbatim rulebook text beyond minimum needed to ground the review. Citations required.

### Step 2 — Write design.md (review-mode)

```yaml
---
name: <Title> — Design (Rules summary, imported)
slug: <slug>-design
game: NNNN-<slug>
stage: design
version: 1.0.0
rubric_version: <current>
bet_version: parliament
author: TIGRIS (review summary of <Designer>'s rules)
created: <YYYY-MM-DD>
updated: <YYYY-MM-DD>
canonical_source: <Exact rulebook edition>
---

# <Title> — Rules Summary (Review Import)

Condensed summary for review purposes. Canonical rules in <source>. This document grounds the TIGRIS panel's stakes.

## 1. Overview
<1-2 paragraphs; the game's pitch.>

## 2. Components
<Brief list.>

## 3-N. Rules sections
<Match the published rulebook's section order when practical. Summarize each
section in 1-3 paragraphs. Cite specific mechanics the panel will stake on.>

## Key mechanics summary
<Table: mechanic → role. One row per load-bearing system. Reviewers stake against this table.>

## Citations
- <Canonical rulebook edition>
- <BoardGameGeek page ID>
- <Secondary sources>
- <Fair-use posture statement>
```

### Step 3 — Fair-use discipline check

- No verbatim reproduction of rulebook text beyond what's necessary for review grounding.
- Citations present.
- Fair-use posture statement present.
- If the review purpose exceeds "critical commentary" (e.g., a play-ready rulebook), flag for user — that's outside fair-use.

### Step 4 — Report

Report: path to design.md, citation completeness, any copyright-posture concerns, next step (TIER-A STAKES).

## Invariants

- design.md is **immutable** once TIER-A begins. Revisions create a new version (v1.1, etc.) and may trigger re-running earlier stages.
- Review-mode always includes citations and fair-use posture.
- Original-mode design.md must include a §11 adjacency chart if the game's mechanics imply new axis adjacencies — feeds I-te-01 amendment cycle.
- Section §12 (open questions) is required for original-mode — it protects the review from addressing nits the designer already knows about.

## What this skill does NOT do

- Write CONCEPT — that's `/tigris-concept`.
- Run the TIER-A draft — that's still manual for now; see Parliament and T&E `tierA-stakes.md` for templates.
- Generate component art or print-and-play assets — per spec §13 non-goals.
- Validate rules-consistency at the level of a real playtest — the panel procedure does that.
