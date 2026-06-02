# TIGRIS Specification Baseline

## Scope

Repo: TIGRIS

VTRACE stage: Specification Baseline

Baseline date: 2026-06-01

## Baseline Inventory

| Surface | Paths | Baseline Status | Notes |
|---|---|---|---|
| Mission/CONOPS/requirements | `docs/vtrace/` | current | VTRACE planning chain established through requirements. |
| Corpus and TIGER BEAT | `data/`, `docs/tiger-beat*.md`, `research/` | current | Corpus, dimensions, gaps, and publications form the evidence base. |
| Parliament/review | `games/`, `personas/`, `TRACKER.md`, `docs/handoff/` | current | Review outcomes drive axis governance and handoff. |
| Simulation | `tools/tigris-sim` | current | Seeded Parliament/UPSTAGE simulation and shared-engine fixture proof. |
| Shared-engine fixtures | MUDDLE/COURT/RACKET references | current | Product-owned tabletop slice supports shared clients. |

## Specification Items

| Spec ID | Parent REQ IDs | Type | Baseline | Specification Statement | Verification | Validation | Owner Surface | Risk |
|---|---|---|---|---|---|---|---|---|
| SPEC-TIG-001 | REQ-TIG-001, REQ-TIG-008 | evidence | current | Corpus/research claims should cite corpus rows, axis matrix, methodology docs, and review evidence or remain draft/blocked. | inspection | corpus review | data/docs/research | high |
| SPEC-TIG-002 | REQ-TIG-002, REQ-TIG-003 | process | current | Parliament claims should preserve axis, persona, player/moment anchor, earned/refuted/retired/collision/amendment/handoff outcomes. | inspection | parliament review | games/personas/tracker | high |
| SPEC-TIG-003 | REQ-TIG-004 | governance | current | Axis adoption and retirement are forward-only and evidence-triggered. | inspection | governance review | axis pool/tracker | high |
| SPEC-TIG-004 | REQ-TIG-005 | design | current | Original game concepts should trace to design gaps, counter-pressure targets, or persona disagreement. | inspection | design review | games/gap docs | medium |
| SPEC-TIG-005 | REQ-TIG-006 | software/evidence | current | Simulation claims should name seed, run count, player count, game target, and variant context. | command, inspection | simulation review | `tools/tigris-sim` | medium |
| SPEC-TIG-006 | REQ-TIG-007 | interface | current | Shared-engine fixtures expose TIGRIS-owned rules without moving rules into MUDDLE, COURT, RACKET, or RALLY. | command, review | adapter review | fixture APIs | high |

## Unknowns And Deferred Detail

| Unknown ID | Unknown | Risk | Disposition |
|---|---|---|---|
| SPEC-TIG-UNK-001 | Claim inspection checklist is not VTRACE-indexed. | Corpus/research audit may be uneven. | Defer to verification/validation. |
| SPEC-TIG-UNK-002 | Simulator L0/L1/L2 command ladder is not selected. | Verification may overrun or under-cover. | Defer to verification. |
| SPEC-TIG-UNK-003 | Shared-engine fixture schemas are not captured as interface rows. | Adapter boundaries may remain implicit. | Defer to interfaces. |

## Role Review Summary

Role lenses applied from `.roles/`: game-space cartography, mechanism/tension
editing, axis governance, corpus claim auditing, and table-experience
observation.

Findings:

| Role | Finding | Disposition |
|---|---|---|
| Corpus Claim Auditor | Corpus/research evidence needs high-risk specification status. | Addressed by SPEC-TIG-001. |
| Axis Governance Steward | Review outcomes and forward-only rules should be separate specs. | Addressed by SPEC-TIG-002 and SPEC-TIG-003. |
| Mechanism Tension Editor | Gap/counter-pressure traceability belongs in the baseline. | Addressed by SPEC-TIG-004. |
| Table Experience Observer | Simulation command context must be specified before verification. | Addressed by SPEC-TIG-005. |

Fixed-point decision:

No critical or major actionable findings remain for the specification baseline.
Gate is `pass_with_risk` because exact claim checklist, simulator command
levels, and interface rows are deferred.
