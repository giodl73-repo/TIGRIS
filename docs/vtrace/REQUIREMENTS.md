# TIGRIS Requirements

## Scope

Repo: TIGRIS

VTRACE stage: Requirements

Baseline date: 2026-06-01

## Requirement Matrix

| Requirement ID | Requirement | Source | Rationale | Priority | Owner Surface | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-TIG-001 | TIGRIS shall tie corpus and TIGER BEAT claims to corpus rows, axis matrix, methodology docs, and review evidence. | NEED-TIG-001 / CON-TIG-002 | Research/design claims require traceable evidence. | must | data, docs, publications | inspection | proposed |
| REQ-TIG-002 | TIGRIS shall require Parliament/review claims to cite axis, persona, player count or moment anchor. | NEED-TIG-003 / CON-TIG-001 | Unsupported broad claims are forbidden by factory discipline. | must | game reviews, panels | inspection | proposed |
| REQ-TIG-003 | TIGRIS shall preserve earned, refuted, retired, collision, amendment, and handoff outcomes for reviews. | CON-TIG-001 | Rubric evolution depends on explicit outcome records. | must | tracker, handoff docs | inspection | proposed |
| REQ-TIG-004 | TIGRIS shall evolve axes forward-only from evidence-triggered adoption/retirement rules. | NEED-TIG-003 / CON-TIG-001 | Silent retirements and retroactive changes break corpus integrity. | must | axis pool, tracker | inspection | proposed |
| REQ-TIG-005 | TIGRIS shall trace original game concepts to design gaps, counter-pressure targets, or documented persona disagreement. | NEED-TIG-002 / CON-TIG-003 | Originals should be products of the factory evidence loop. | must | games, gap docs | inspection | proposed |
| REQ-TIG-006 | TIGRIS shall tie simulation claims to seed, run count, player count, game target, and variant context. | NEED-TIG-004 / CON-TIG-004 | Simulation output must be reproducible and scoped. | must | `tools/tigris-sim` | command, inspection | proposed |
| REQ-TIG-007 | TIGRIS shall expose MUDDLE, COURT, and RACKET fixtures without moving TIGRIS rules into shared engines. | NEED-TIG-004 / CON-TIG-005 | Product rule ownership must remain local. | must | shared-engine fixtures | command, review | proposed |
| REQ-TIG-008 | TIGRIS shall mark corpus/research claims draft or blocked when source, corpus, or methodology evidence is missing. | CON-TIG-002 | Missing evidence must not become published certainty. | must | docs, research | review | proposed |

## Deferred Definitions

| Deferred ID | Item | Disposition |
|---|---|---|
| DEF-TIG-001 | Exact claim-inspection checklist for corpus and publication rows. | Defer to specification baseline. |
| DEF-TIG-002 | Simulator L0/L1/L2 command ladder. | Defer to verification. |
| DEF-TIG-003 | Shared-engine fixture interface rows. | Defer to interfaces. |

## Role Review Summary

Role lenses applied from `.roles/`: game-space cartography, mechanism/tension
editing, axis governance, corpus claim auditing, and table-experience
observation.

Findings:

| Role | Finding | Disposition |
|---|---|---|
| Corpus Claim Auditor | Corpus/research claims need hard evidence requirements. | Addressed by REQ-TIG-001 and REQ-TIG-008. |
| Axis Governance Steward | Axis outcome and forward-only rules must be explicit. | Addressed by REQ-TIG-003 and REQ-TIG-004. |
| Mechanism Tension Editor | Original designs need gap/counter-pressure traceability. | Addressed by REQ-TIG-005. |
| Table Experience Observer | Simulation claims require seed/run/player context. | Addressed by REQ-TIG-006. |

Fixed-point decision:

No critical or major actionable findings remain for the requirements stage.
Exact specs, interface schemas, verification command levels, and work packages
are deferred to later VTRACE stages.
