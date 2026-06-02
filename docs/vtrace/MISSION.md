# TIGRIS Mission

## Scope

Repo: TIGRIS

VTRACE stage: Mission

Baseline date: 2026-06-01

TIGRIS is a board-game factory. Its mission is to review published games,
discover game-design dimensions from corpus evidence, design original games into
gaps, and validate simulations and playable adoption fixtures through explicit
Parliament-style disagreement rather than smoothed consensus.

## Mission Need

| Need ID | Need | Primary User | Success Signal |
|---|---|---|---|
| NEED-TIG-001 | Preserve the TIGER BEAT corpus and rubric as evidence-backed game-design infrastructure. | Game-space cartographer, corpus auditor | Corpus rows, axis pool, gap maps, and publications remain traceable to reviews. |
| NEED-TIG-002 | Turn design gaps and persona disagreement into original board-game concepts. | Game designer, mechanism editor | Originals such as Parliament, CODEX, RITE, and UPSTAGE trace to gap or counter-pressure evidence. |
| NEED-TIG-003 | Keep Parliament review as a game-like, staged review system with explicit stakes, collisions, amendments, and handoff. | Parliament operator, axis steward | Session and tracker rows preserve earned/refuted/retired axes and zero silent-retirement claims. |
| NEED-TIG-004 | Validate simulation and shared-engine fixtures without moving TIGRIS rules into MUDDLE, COURT, RACKET, or RALLY. | Adapter maintainer, runtime reviewer | `tools/tigris-sim` provides seeded smokes and product-owned adoption fixtures. |

## Mission Success Criteria

| Criterion ID | Criterion | Evidence Surface | Deferred Detail |
|---|---|---|---|
| MSC-TIG-001 | A future agent can identify corpus status, latest game, active gaps, and handoff point. | `README.md`, `TRACKER.md`, `docs/handoff/` | Trace rows deferred to `TRACE.md`. |
| MSC-TIG-002 | Parliament/review claims cite axes, personas, player counts, and moments. | `personas/`, game panel summaries, `TRACKER.md` | Requirements deferred to `REQUIREMENTS.md`. |
| MSC-TIG-003 | Simulation claims are seed-backed and product-owned. | `tools/tigris-sim` commands | Command levels deferred to `VERIFICATION.md`. |
| MSC-TIG-004 | Shared runtime fixtures preserve TIGRIS rule ownership. | MUDDLE, COURT, RACKET fixture references | Interface ownership deferred to `INTERFACES.md`. |

## Constraints

- TIGRIS must not make unsupported fun/quality claims without axis/persona/moment
  anchors.
- Rubric evolution must remain forward-only and evidence-triggered.
- Shared engines must not own TIGRIS product rules.
- Research/publication claims need corpus and methodology traceability.

## Initial Validation Expectations

```powershell
cd tools\tigris-sim
cargo test --quiet
cargo run --quiet -- --seed parliament-smoke
cargo run --quiet -- --seed parliament-smoke --compare-variants --runs 20 --players 4
cargo run --quiet -- --game upstage --seed upstage-smoke --players 8 --compare-variants --runs 24
```

## Role Review Summary

Role lenses applied from `.roles/`: game-space cartography, mechanism/tension
editing, axis governance, corpus claim auditing, and table-experience
observation.

Findings:

| Role | Finding | Disposition |
|---|---|---|
| Corpus Claim Auditor | Corpus and research claims must remain source-backed and traceable. | Addressed in NEED-TIG-001 and constraints. |
| Axis Governance Steward | Axis adoption/retirement must stay evidence-triggered and forward-only. | Addressed in NEED-TIG-003. |
| Mechanism Tension Editor | Original designs must trace to design gaps or counter-pressure evidence. | Addressed in NEED-TIG-002. |
| Table Experience Observer | Play and simulation claims must stay anchored to table moments. | Deferred to requirements and validation. |

Fixed-point decision:

No critical or major actionable findings remain for the mission stage. Exact
requirements, command levels, interface rows, and work packages are deferred to
later VTRACE stages.
