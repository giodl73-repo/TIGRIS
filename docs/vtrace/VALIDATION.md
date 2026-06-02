# TIGRIS Validation

## Scope

Repo: TIGRIS

VTRACE stage: Validation

Baseline date: 2026-06-01

## Validation Scenarios

| Validation ID | Parent Verification | Scenario | Acceptance Standard | Evidence |
|---|---|---|---|---|
| VAL-TIG-001 | VER-TIG-001 | Corpus or publication claim is promoted. | Claim cites corpus rows, axis matrix, methodology docs, and review evidence; otherwise it remains draft or blocked. | Corpus claim packet. |
| VAL-TIG-002 | VER-TIG-002 | Parliament review outcome is accepted. | Stakes, axis/persona/player or moment anchors, collisions, earned/refuted/retired outcomes, amendments, and handoff are preserved. | Parliament review packet. |
| VAL-TIG-003 | VER-TIG-006 | Original game concept is promoted. | Concept traces to gap, counter-pressure target, or documented persona disagreement. | Gap design packet. |
| VAL-TIG-004 | VER-TIG-003, VER-TIG-004, VER-TIG-005 | Simulation claim supports a design decision. | Claim names seed, run count, player count, game target, variant context, and exploratory/promoted label. | Simulator telemetry packet. |
| VAL-TIG-005 | VER-TIG-007 | Shared engine consumes tabletop fixture. | TIGRIS owns tabletop rules; MUDDLE/COURT/RACKET/RALLY own adaptation/rendering/telemetry behavior. | Fixture diagnostics/smokes. |

## Claim Rules

- Unsupported corpus/research claims remain draft or blocked.
- Axis evolution is forward-only.
- Simulation output without full context is exploratory.
- Shared-engine proof does not transfer TIGRIS rules.

## Role Review Summary

Role lenses applied from `.roles/`: game-space cartography, mechanism/tension
editing, axis governance, corpus claim auditing, and table-experience
observation.

No critical or major actionable findings remain. Exact claim checklist and
fixture tasks move to trace and work packages.
