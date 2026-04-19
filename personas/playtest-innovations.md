---
name: TIGRIS Innovation Log
slug: playtest-innovations
version: 2.0.0
rubric_version: v2.0
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# Innovation Log

Append-only. Records observations surfaced during Stakes, Argument, and Amendment phases. Amendments flow from this log into `personas/playtest-rubric.md` and `personas/axis-pool.md`.

## Schema (v2.0)

Each entry:

```
### I-<source-slug>-<NN> — <short title>

- **Dimension:** <axis slug from pool OR "bet change" OR "new axis candidate: <slug>">
- **Trigger pattern:** <short machine-readable predicate, e.g., "axis_collinearity", "shallow_cell_scoring", "anchor_cell_ambiguous">
- **Source:** <game slug | spec-review path | session ID>
- **Supporting voices:** <list of personas>
- **Scope:** this-game | pattern-across-games | pipeline-architectural | universal | bet
- **Status:** candidate | proposed-amendment | adopted | retired | superseded
- **Note:** <2-4 sentences; what was seen, why it matters, what it proposes>
```

### Trigger pattern conventions

Keep predicates short, snake_case, and specific. Cluster detection matches on `trigger_pattern` first, `dimension` second. Examples:

- `axis_collinearity` — two axes covary in scoring across plays
- `shallow_cell_scoring` — Tier-A cell received insufficient reasoning time
- `anchor_cell_ambiguous` — the anchor-persona / anchor-axis was under-specified at CONCEPT
- `dead_stage_for_reviews` — pipeline stage produced nothing material
- `persona_chorus` — personas produced interchangeable output despite different weights
- `theme_reskinnable` — axis definitions would apply unchanged to a non-Euro domain
- `first_hour_contributor_fails` — spec requires off-document reading to act

Two or more entries sharing a `trigger_pattern` across ≥ 2 sources → propose amendment. This is stricter than v1.0's "same dimension across ≥ 2 games" which clustered on axis-name string-match.

## Rate limit

At most **one rubric amendment adopted per 2-game cycle** (not per version bump). This prevents the ledger from moving faster than the game corpus can validate. Exception: architectural bet changes (v1.0 → v2.0) are not amendments — they reset the rate-limit counter.

## Bet changes

A **bet change** is larger than an amendment. It restructures the architectural posture, not a single axis or protocol rule. Bet changes lock all prior scores at their `bet_version` and start a fresh ledger. Bet changes are logged with `dimension: "bet change"` and `scope: bet`.

---

## Entries

### I-spec-v1-01 — Architectural bet: Parliament (adversarial review system)

- **Dimension:** bet change
- **Trigger pattern:** `persona_chorus`
- **Source:** docs/specs/reviews/2026-04-19-tigris-design/ (v1.0 panel)
- **Supporting voices:** Chvátil (primary), Knizia, Rosenberg, Feld, Kramer-Kiesling (aligned — five of seven designers flagged orthogonality / chorus from distinct angles)
- **Scope:** bet
- **Status:** adopted (2026-04-19)
- **Note:** v1.0 personas shared a single 8-axis rubric and differed only by weight vector. Five reviewers independently flagged this as producing "chorus, not parliament" (Chvátil) / correlated signal (Knizia) / hidden dominant-engine problem (Rosenberg) / covariant axes (Feld) / axis overlap (K-K). v2.0 replaces the shared rubric with a 24-axis Pool from which personas draft 3 each per game; two personas cannot score the same axis simultaneously. Panels produce argument records (stakes earned / refuted / contested / ignored / collided), not weighted aggregates. Amendments to the Pool are *won* in argument, not *proposed* by cluster detection. See `docs/specs/2026-04-19-tigris-v2.0-design.md`.

### I-spec-v1-02 — Axis-set asymmetry per persona (A8)

- **Dimension:** bet change (subsumed by I-spec-v1-01)
- **Trigger pattern:** `persona_chorus`
- **Source:** v1.0 panel — Chvátil amendment A1
- **Supporting voices:** Chvátil (originator)
- **Scope:** bet
- **Status:** adopted — implemented as the draft mechanism in v2.0.

### I-spec-v1-03 — Sub-axis taxonomy + amendment rate limit (A3)

- **Dimension:** rubric meta
- **Trigger pattern:** `axis_name_only_clustering`
- **Source:** v1.0 panel
- **Supporting voices:** Rosenberg, Lacerda, Knizia (3)
- **Scope:** universal
- **Status:** adopted (2026-04-19) — `trigger_pattern` field added to this log schema; rate limit declared above.

### I-spec-v1-04 — Tier-A calibration + time budget (A4)

- **Dimension:** stakes protocol
- **Trigger pattern:** `shallow_cell_scoring`
- **Source:** v1.0 panel
- **Supporting voices:** Knizia, Rosenberg, Lacerda (3)
- **Scope:** universal
- **Status:** adopted (2026-04-19) — v2.0 STAKES phase has explicit 60–90 min budget; collisions during ARGUMENT provide inter-rater signal without requiring redundant scoring.

### I-spec-v1-05 — Euro-specific axes (A5)

- **Dimension:** new axis candidates (System Gearing, Catastrophe Pressure, Conversion Chain Depth, Information Transparency Cost, Architectural Novelty)
- **Trigger pattern:** `theme_reskinnable`
- **Source:** v1.0 panel
- **Supporting voices:** Lacerda, Chvátil (2)
- **Scope:** universal
- **Status:** adopted (2026-04-19) — all five axes added to Band B of the Pool in `personas/axis-pool.md`. Thematic Integration from v1.0 dropped (re-skinnable failure mode).

### I-spec-v1-06 — Anchor-cell lock protocol (A6)

- **Dimension:** GATE protocol
- **Trigger pattern:** `anchor_cell_ambiguous`
- **Source:** v1.0 panel
- **Supporting voices:** Knizia, Feld (2)
- **Scope:** universal
- **Status:** adopted (2026-04-19) — anchor-persona + anchor-axis declared at CONCEPT and locked; one-time re-declaration permitted at GATE only if another persona's drafted axis is obviously stronger for this design. For spec-reviews, anchor may be null.

### I-spec-v1-07 — Sibling-repo glossary + SKILL quickstart (A7)

- **Dimension:** teachability (process)
- **Trigger pattern:** `first_hour_contributor_fails`
- **Source:** v1.0 panel
- **Supporting voices:** Kramer-Kiesling, Stegmaier (2)
- **Scope:** universal
- **Status:** adopted (2026-04-19) — v2.0 spec §3 contains sibling-repo glossary. `skills/tigris-panel/SKILL.md` now opens with a 60-second quickstart. Standard extends to future SKILL.md files.

### I-spec-v1-08 — Dead-stage retrospective (A9)

- **Dimension:** pipeline architectural
- **Trigger pattern:** `dead_stage_for_reviews`
- **Source:** v1.0 panel
- **Supporting voices:** Feld (1, but kept — specific and testable)
- **Scope:** universal
- **Status:** adopted (2026-04-19) — scheduled as post-Parliament retrospective. Current open question: CONCEPT for review-mode, which v2.0 §8.1 partially addresses by making anchor-stake declaration happen at CONCEPT for reviews too.

## Retired amendments (subsumed or superseded)

### I-spec-v1-09 — Orthogonality audit of persona weight vectors (A1)

- **Status:** retired — replaced by v2.0 draft mechanism. Weight vectors are gone; axis sets are disjoint by construction. Orthogonality audit is no longer needed because orthogonality is enforced at draft time.

### I-spec-v1-10 — Scoring selection rule, SD penalty on hedging (A2)

- **Status:** retired — replaced by v2.0 stake-state mechanism. There is no weighted aggregate to penalize hedging; stakes are binary (earned/refuted). The "hedging" failure mode is now an `ignored` stake — the persona did not argue for their stake, so it produces no evidence.

## Active clusters

None yet under v2.0. First Parliament game (games/0001-parliament/) expected to surface first v2.x-era clusters.

## Bet version history

| Bet | Adopted | Retired / superseded | Summary |
|---|---|---|---|
| **parliament** (v2.0) | 2026-04-19 | — | Adversarial stake-and-argument. Replaces chorus-of-weighted-reviewers. Source: `docs/specs/2026-04-19-tigris-v2.0-design.md`. |
| chorus (v1.0) | superseded 2026-04-19 | 2026-04-19 | Shared 8-axis rubric with per-persona weight vectors. Retired because personas correlated rather than argued. v1.0 scores locked and not re-scored.
