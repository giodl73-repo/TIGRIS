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

- **I-parliament-01 → v2.1** — FORCED-ENGAGEMENT micro-phase. Adopted 2026-04-19 before game #2. Validated across 10 games (0% silent-retire).
- **I-parliament-03 → v2.3** — Anchor-axis adjacency-partner GATE check. Adopted after Dominion.
- **I-te-01 + I-facets-01 + I-facets-02 → v2.3.1** — Adjacency chart updates. Consolidated into `personas/adjacency-chart.md`.
- **I-facets-03 + I-dominion-04 → v2.4** — Vaccarino persona added as 8th designer. Resolves never-drafted axes.
- **I-parliament-02 → v2.6** — B6 Scoring Multiplier Dependency new axis. Full validation loop complete at v2.8.

## Triple-review innovations (games #11-13, parallel via TeamCreate)

### Lisboa (game #11) innovations

- **I-lisboa-01**: Lacerda-on-Lacerda first anchor (5th designer-on-own). Pattern completes for canonical-axes-already-adopted designer.
- **I-lisboa-02**: Canonical-confirmation game pattern — Lacerda's territory pre-adopted; 0 new adoptions but clean re-validation.
- **I-lisboa-03**: A3 Interaction cross-mechanic universality — earned across Famiglia/TtA/Lisboa (3 different mechanical families).
- **I-lisboa-04**: Retire-explicit cadence at 7 (heavy-expert profile signature).

### Castles of Burgundy (game #12) innovations

- **I-cob-01**: Feld-on-Feld first published anchor (6th designer-on-own; Feld had FACETS original + CoB published).
- **I-cob-02**: A4 Variance Calibration first-ever earning — dormant 11 games, canonical case via Feld's dice-as-resource.
- **I-cob-03**: Multiplayer-solitaire critique operationalized — A3 retire-explicit via Chvátil self-refute.
- **I-cob-04**: Axis-definition reference game earning — C6's own reference case (axis literally cites Castles of Burgundy).

### Tikal (game #13) innovations

- **I-tikal-01**: First dual-canonical anchor pattern — two adopted axes both name Tikal in their anchor-10 descriptor (C7 and D2).
- **I-tikal-02**: Queued-for-adoption pipeline fully cleared (D3 + A5 both triggered).
- **I-tikal-03**: Anchor-descriptor voting priority (both K-K collisions won by anchor-named axis).
- **I-tikal-04**: TIGRIS record 17 earned stakes. Heavyweight-classic games earn broadly.

## Parallel-review pattern (v2.11)

- **First parallel-pipeline session** in TIGRIS history. 3 agents × 1 game each via TeamCreate orchestration.
- Each agent wrote only to its game directory; user integrated shared state (axis-pool, innovations, TRACKER) after all three completed.
- Hard constraints preserved ledger integrity (no conflicts).
- **Pattern success**: parallel review is viable for independent games without shared-axis-adoption conflict. Useful for processing backlog quickly when games don't interact structurally.

## Adopted axes (v2.x era)

- **C2 Minimum-Score Shape → v2.2** — Knizia's anchor. Earned Parliament + T&E. First adoption.
- **B4 Information-Transparency-Cost → v2.2** — Lacerda-primary. Earned Parliament + T&E. Second adoption.
- **B5 Architectural Novelty → v2.3** — Chvátil-primary. Earned Parliament + Dominion. Third adoption.
- **A6 Teachability → v2.3** — Stegmaier-primary. Earned FACETS + Dominion. Fourth adoption.
- **A2 Decision Density → v2.3** — Feld-primary. Earned FACETS + Dominion. Fifth adoption.
- **B1 System Gearing → v2.3** — Lacerda-primary. Earned T&E + Dominion. Sixth adoption.
- **D4 Late-Game Lock-in → v2.3** — Rosenberg/Lacerda. Earned FACETS + Dominion. Seventh adoption.
- **A7 Emergence-Replayability → v2.5** — Lacerda-primary. Earned Dominion + Scythe. Eighth adoption.
- **D1 Family-to-Expert Scaling → v2.5** — K-K-primary. Earned Dominion + Scythe. Ninth adoption.
- **D2 Spatial-Interaction Presence → v2.5** — K-K-primary. Earned T&E + Scythe. Tenth adoption.
- **C4 Engine-Garden Dependency → v2.5** — Rosenberg-primary. Earned Dominion + Scythe. Eleventh adoption (retirement reversal).
- **C1 Tension Budget → v2.7** — Knizia-primary. Earned PR + UNFOLD. Twelfth adoption (dormant→recovered).
- **C3 Scarcity Bite → v2.7** — Rosenberg-primary. Earned PR + UNFOLD. Thirteenth adoption (queued-for-retirement cleared).
- **C6 Point-Salad Incommensurability → v2.7** — Feld-primary. Earned FACETS + UNFOLD. Fourteenth adoption (contested history resolved).
- **C7 Action-Menu Clarity → v2.7** — K-K-primary. Earned PR + UNFOLD. Fifteenth adoption.
- **B2 Catastrophe Pressure → v2.8** — Chvátil-primary. Earned T&E + TtA. Sixteenth adoption (retirement salvage).
- **B3 Conversion Chain Depth → v2.8** — Vaccarino-primary. Earned PR + TtA (LAST-CALL). Seventeenth adoption (Vaccarino signature).
- **B6 Scoring Multiplier Dependency → v2.8** — Rosenberg-primary. Earned UNFOLD + TtA. Eighteenth adoption (new-axis full validation).
- **A3 Interaction → v2.9** — Chvátil-primary. Earned TtA + Famiglia. Nineteenth adoption.
- **A1 Elegance → v2.10** — Knizia-primary. Earned Famiglia + ZEN PATH. Twentieth adoption.
- **D3 Count-Robustness → v2.11** — Stegmaier-primary. Earned Scythe + ZEN PATH + Tikal + CoB. 21st adoption (closes queue; **Band D fully adopts**).
- **A5 Downtime-Pacing → v2.11** — Vaccarino-secondary. Earned Famiglia + Tikal + CoB. 22nd adoption.

Total adopted axes: **22 of 25 Pool** (88%). After 13 games. Plus 1 retired (C5). Remaining live: A4 (queued), C8.

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
| 1 (Parliament → T&E) | I-parliament-01 (v2.1 FORCED-ENGAGEMENT) | 48% ignore rate flagged in Parliament |
| 2 (FACETS → game #4) | TBD — likely I-parliament-03 | 3 instances of anchor-adjacency pattern (now ready-for-adoption) |

## Game #3 (FACETS) innovations

### I-facets-01 — New adjacency: C6 Point-Salad ↔ A6 Teachability

- **Dimension:** adjacency chart
- **Trigger pattern:** `new_adjacency_surfaced`
- **Source:** FACETS Turn 8 collision
- **Supporting voices:** Feld, Stegmaier
- **Scope:** universal
- **Status:** candidate (low-risk; adoptable as adjacency-chart-patch outside rate-limit)

### I-facets-02 — New adjacency: A2 Decision Density ↔ A6 Teachability

- **Dimension:** adjacency chart
- **Trigger pattern:** `new_adjacency_surfaced`
- **Source:** FACETS Turn 4 collision
- **Supporting voices:** Feld, Stegmaier
- **Status:** candidate

### I-facets-03 — Systematically undrafted axes (A4, A5, B3)

- **Dimension:** pool curation
- **Trigger pattern:** `axes_never_drafted`
- **Source:** 3-game draft history
- **Supporting voices:** observational
- **Status:** candidate — proposed options: retire axes, reassign advocacy, or add a new "Referee/Tournament Player" persona with A4/A5/B3 preferences.

### I-facets-04 — Self-refutation validates factory self-correction

- **Dimension:** bet validation
- **Trigger pattern:** `self_refutation_by_evidence`
- **Source:** Chvátil A3 Interaction self-refutation at FACETS Turn 8
- **Supporting voices:** Chvátil; context I-parliament-04 + I-te-03
- **Status:** adopted (log entry) — second self-refutation across 3 games; bet validation strengthens.

## Cluster status updates (2026-04-19)

### I-te-02 — Identical drafts across games

- **Status update:** proposed-amendment (3 instances) → **closed observational** (Dominion broke the pattern — Lacerda fell through preferences to A7). Pattern is not universal; no amendment needed.

### I-parliament-03 — Anchor-axis without collision partner

- **Status:** **ADOPTED in v2.3** (2026-04-19, Dominion session).
- **GATE amendment:** "Anchor adjacency partner present — the anchor axis has at least one semantically-adjacent axis drafted by a DIFFERENT persona."
- **Applied:** forward-only. Dominion retrospectively PASSES; P, T&E, FACETS would have required re-draft but are locked at their versions.

### I-facets-03 — Systematically undrafted axes (A4, A5, B3)

- **Status update:** candidate (3 instances) → **proposed-amendment** (4 instances after Dominion).
- **Scheduled for cycle 3 adoption.** Likely fix: add a "Vaccarino" or "Referee" persona whose preferences include A4/A5/B3; OR retire these axes from Pool. See I-dominion-04 for the Vaccarino candidate.

## Dominion (game #4) innovations

### I-dominion-01 — First cross-player anchor collision

- **Dimension:** bet validation
- **Trigger pattern:** `anchor_collision_occurred`
- **Source:** Dominion Turn 10 (B5↔A7 collision, Chvátil vs Lacerda)
- **Status:** adopted (log entry)
- **Note:** First time across 4 games that anchor axis and its adjacency partner were drafted by different personas, enabling cross-player anchor collision. Outcome: 3-2 for A7. Validates I-parliament-03's adoption — cross-player partners produce informative collisions.

### I-dominion-02 — Retirement reversal handled cleanly

- **Dimension:** protocol validation
- **Trigger pattern:** `retirement_reversal_via_earning`
- **Source:** C4 Engine-Garden trajectory (P refuted → T&E contested → FACETS retire-explicit → Dominion earned)
- **Status:** adopted (log entry)
- **Note:** Ledger protocol handled rehabilitation correctly. No special logic needed; normal thresholds + new earning moved C4 from queued-for-retirement back to live.

### I-dominion-03 — Cluster-adoption pattern

- **Dimension:** ledger dynamics
- **Trigger pattern:** `multi_axis_adoption_event`
- **Source:** Dominion (5 adoptions)
- **Status:** candidate (observational)
- **Note:** Dominion produced 5 adoptions in one session. Pattern: games that hit many rubric dimensions produce adoption bursts. Game-selection matters — reviewing diverse games is more ledger-productive than reviewing similar ones.

### I-dominion-04 — Vaccarino persona

- **Dimension:** persona roster
- **Trigger pattern:** `persona_gap_identified`
- **Source:** Dominion review — Chvátil proxied but lens-mismatch identifiable
- **Status:** **ADOPTED in v2.4** — Vaccarino persona added at `personas/designers/vaccarino.md`. Signatures: Dominion, 13 expansions, Kingdom Builder, card-level elegance, randomized setup, shuffle-variance-as-resource. Preferred_axes cover A4, A5, B3, B5, C4, C7, A1, A2 — authentic deck-building lens.

## Amendment-adopted summary

| Cycle | Adopted | Triggered by |
|---|---|---|
| 1 (P → T&E) | I-parliament-01 (v2.1 FORCED-ENGAGEMENT) | 48% ignore rate in P |
| 2 (FACETS → Dominion) | I-parliament-03 (v2.3 anchor-adjacency GATE) | 3 cluster instances + positive counter-example |
| 3 (pre-Scythe) | I-facets-03 + I-dominion-04 (v2.4 Vaccarino persona) | 4 instances never-drafted + Dominion lens-gap |
| 4 (PR) | I-parliament-02 (v2.6 B6 Scoring Multiplier Dependency new axis) | 3 cluster instances (P + Scythe + PR) |

## UNFOLD (game #7) innovations

### I-unfold-01 — Factory successfully reviews its own original design

- **Trigger:** `factory_reviews_own_output`
- **Source:** UNFOLD was designed BY TIGRIS (via concept→review→revise→design workflow) and reviewed BY the same pipeline. 4 adoptions, 11+ earned, 0 design-blocking issues.
- **Status:** adopted (log entry) — strongest bet-validation signal to date.

### I-unfold-02 — Collision tiebreak by earlier-drafted axis

- **Trigger:** `collision_tiebreak_earlier_draft`
- **Source:** UNFOLD B5 ↔ A7 tied 3-3; resolved per rule. First instance.
- **Status:** observational.

### I-unfold-03 — Re-earning adopted axes as secondary metric

- **Trigger:** `adopted_axis_multiple_reearns`
- **Source:** 6 adopted axes re-earned in UNFOLD (B5, B1, A7, A2, D4, D2).
- **Status:** observational — suggests tracking "times-earned-after-adoption" for future ledger metrics.

### I-unfold-04 — 4-adoption events in one session (pattern)

- **Trigger:** `multi_adoption_event`
- **Source:** UNFOLD (matches Dominion and Scythe).
- **Status:** observational — cultural-structure games (4-culture UNFOLD) produce Band-C adoption bursts.

## Bet version history

| Bet | Adopted | Retired / superseded | Summary |
|---|---|---|---|
| **parliament** (v2.0) | 2026-04-19 | — | Adversarial stake-and-argument. Replaces chorus-of-weighted-reviewers. Source: `docs/specs/2026-04-19-tigris-v2.0-design.md`. |
| chorus (v1.0) | superseded 2026-04-19 | 2026-04-19 | Shared 8-axis rubric with per-persona weight vectors. Retired because personas correlated rather than argued. v1.0 scores locked and not re-scored.
