# TIGRIS Verification

## Scope

Repo: TIGRIS

VTRACE stage: Verification

Baseline date: 2026-06-01

## Verification Ladder

| Level | Verification ID | Parent Requirements / Interfaces | Command Or Inspection | Purpose | Expected Evidence |
|---|---|---|---|---|---|
| L0 | VER-TIG-001 | REQ-TIG-001, REQ-TIG-008, IF-TIG-001 | Inspect corpus, TIGER BEAT docs, methodology, and research links | Confirm corpus/research claims have evidence or draft/blocked labels. | Claim packet or explicit blocker. |
| L0 | VER-TIG-002 | REQ-TIG-002, REQ-TIG-003, IF-TIG-002 | Inspect game panels, personas, `TRACKER.md`, and handoffs | Confirm Parliament outcomes preserve anchors and handoff state. | Review packet with outcome fields. |
| L1 | VER-TIG-003 | REQ-TIG-006, IF-TIG-004 | `cd tools\tigris-sim; cargo test --quiet` | Prove simulator baseline tests pass. | Passing test output or explicit blocker. |
| L1 | VER-TIG-004 | REQ-TIG-006, IF-TIG-004 | `cd tools\tigris-sim; cargo run --quiet --bin tigris-sim -- --seed parliament-smoke` | Verify seeded Parliament simulation context. | Seeded simulation output. |
| L2 | VER-TIG-005 | REQ-TIG-006, IF-TIG-004 | `cd tools\tigris-sim; cargo run --quiet --bin tigris-sim -- --seed parliament-smoke --compare-variants --runs 20 --players 4` | Verify variant comparison evidence. | Seed/run/player/variant packet. |
| L2 | VER-TIG-006 | REQ-TIG-005, IF-TIG-003 | Inspect original design gap/counter-pressure packets | Verify original concepts trace to evidence. | Gap design packet. |
| L2 | VER-TIG-007 | REQ-TIG-007, IF-TIG-005 | Inspect or run MUDDLE/COURT/RACKET fixture proof | Verify shared-engine boundaries. | Fixture proof without TIGRIS rule transfer. |

## Verification Rules

- Corpus/research claims without source or methodology evidence remain blocked
  or draft.
- Simulation output without seed, run count, player count, game target, and
  variant context is exploratory.
- Axis changes are verified as forward-only records.

## Role Review Summary

Role lenses applied from `.roles/`: game-space cartography, mechanism/tension
editing, axis governance, corpus claim auditing, and table-experience
observation.

No critical or major actionable findings remain. Exact claim checklist and
shared-engine schema rows are deferred to validation, trace, and work packages.
