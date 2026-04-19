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

## Active clusters (v2.0 era)

Parliament (game #1) completed 2026-04-19. Four new innovations logged.

### I-parliament-01 — Ignored-stake protocol too permissive

- **Dimension:** Argument protocol
- **Trigger pattern:** `ignored_stake_over_threshold`
- **Source:** games/0001-parliament/
- **Supporting voices:** Stegmaier (success-criterion miss), Knizia (self-observed), Feld (self-observed)
- **Scope:** universal
- **Status:** **ADOPTED** (2026-04-19, rubric v2.0 → v2.1) — rate-limit budget consumed for this 2-game cycle
- **Note:** 10 of 21 drafted stakes (48%) never engaged in argument. Panel produced rich collisions on a subset of axes while the rest sat idle. Adopted fix: FORCED-ENGAGEMENT micro-phase inserted between penultimate and final moment-anchor. Each still-ignored stake must resolve to HOLD-EXPLICIT (1-token cost), RETIRE-EXPLICIT (+0.5 refute, token refund), or LAST-CALL-STAKE (double-weight final-round stake). Stakes still at 0-marks become `silent-retire` (-3 pts, +1 toward retirement, persona-retirement flag at 3+ per game). See `personas/playtest-rubric.md` v2.1 changelog.

### I-parliament-02 — Scoring Multiplier Dependency (new axis candidate)

- **Dimension:** new axis candidate (Band B)
- **Trigger pattern:** `axis_not_in_pool`
- **Source:** games/0001-parliament/amendments.md (Rosenberg end-of-game remark)
- **Supporting voices:** Rosenberg (originator), Knizia (anchor axis implicitly asserts this)
- **Scope:** pattern-across-games
- **Status:** candidate
- **Note:** Parliament doesn't have engine-gardens but has a structurally similar scoring multiplier. Define as: "Do endgame points depend multiplicatively on choices across multiple subsystems, forcing balance?" Gives Rosenberg a primary-advocate axis in games without literal engine-gardens.

### I-parliament-03 — Anchor axis without collision partner is fragile

- **Dimension:** GATE protocol
- **Trigger pattern:** `anchor_axis_without_collision_partner`
- **Source:** Parliament TIER-A draft record
- **Supporting voices:** Knizia (anchor persona who went without partner)
- **Scope:** universal
- **Status:** candidate
- **Note:** Knizia passed on B3 Conversion Chain Depth (his anchor C2's natural adjacency). He earned C2 without collision help. This worked but isn't guaranteed. Proposal: GATE check that anchor axis has ≥ 1 adjacent axis drafted by another persona.

### I-parliament-04 — Architectural Novelty validation (bet log entry)

- **Dimension:** bet validation
- **Trigger pattern:** `novel_architecture_defended`
- **Source:** Chvátil-reviewer earned 3 marks on B5 in Parliament
- **Supporting voices:** Chvátil (stake-holder), Knizia + Lacerda (voted for Novelty in collision)
- **Scope:** bet
- **Status:** adopted (as a log entry, not a protocol change) — v2.0 bet validates itself on first run
- **Note:** The v1.0 panel flagged "zero novel architectural bet." The v2.0 bet (Parliament) claimed architectural novelty. In v2.0's first use, Architectural Novelty as an axis earned the second-strongest reviewer-stake record (3 marks). The claim produced its own evidence.

## Adopted amendments (v2.x era)

- **I-parliament-01 → v2.1** — FORCED-ENGAGEMENT micro-phase. Adopted 2026-04-19 before game #2. Rate-limit consumed. Validated by T&E (0% silent-retire vs Parliament's 48% ignore rate).

## Adopted axes (v2.x era)

- **C2 Minimum-Score Shape → v2.2** — Knizia's anchor. Earned in Parliament (3 defends) + earned in T&E (LAST-CALL). First adoption event (2026-04-19).
- **B4 Information-Transparency-Cost → v2.2** — Lacerda-primary. Earned in Parliament (1d+1cw) + earned in T&E (LAST-CALL). Second adoption event (2026-04-19).

## Game #2 (T&E) innovations

### I-te-01 — New adjacency: Catastrophe Pressure ↔ Anti-Catch-up Pressure

- **Dimension:** adjacency chart
- **Trigger pattern:** `new_adjacency_surfaced`
- **Source:** T&E Turn 5
- **Supporting voices:** Chvátil, Feld (both stakes fired simultaneously)
- **Status:** candidate (low-risk, adoptable as v2.2.1 patch outside rate-limit)

### I-te-02 — Identical drafts across games

- **Dimension:** draft protocol
- **Trigger pattern:** `identical_drafts_across_games`
- **Source:** Parliament + T&E draft comparison — bit-identical 21-axis picks
- **Supporting voices:** observational
- **Status:** candidate (awaiting 3rd game to see if pattern breaks naturally)

### I-te-03 — FORCED-ENGAGEMENT fix validated

- **Dimension:** protocol validation
- **Trigger pattern:** `protocol_amendment_validated`
- **Source:** T&E vs Parliament comparison (0% vs 48%)
- **Status:** adopted (log entry)

## I-parliament-03 cluster update

- **Status:** candidate → **proposed-amendment** (2026-04-19)
- **Note:** Second instance logged in T&E (Knizia's C2 again without B3 collision partner). Now clusters. Scheduled for adoption after game #3 completes rate-limit cycle.

## Amendment-adopted summary

| Cycle | Adopted | Triggered by |
|---|---|---|
| 1 (Parliament → T&E) | I-parliament-01 | 48% ignore rate flagged in Parliament |
| 2 (T&E → game #3) | TBD — likely I-parliament-03 | 2 instances of anchor-adjacency pattern |

## Bet version history

| Bet | Adopted | Retired / superseded | Summary |
|---|---|---|---|
| **parliament** (v2.0) | 2026-04-19 | — | Adversarial stake-and-argument. Replaces chorus-of-weighted-reviewers. Source: `docs/specs/2026-04-19-tigris-v2.0-design.md`. |
| chorus (v1.0) | superseded 2026-04-19 | 2026-04-19 | Shared 8-axis rubric with per-persona weight vectors. Retired because personas correlated rather than argued. v1.0 scores locked and not re-scored.
