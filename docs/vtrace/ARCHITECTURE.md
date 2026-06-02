# TIGRIS Architecture

## Scope

Repo: TIGRIS

VTRACE stage: Architecture

Baseline date: 2026-06-01

## Architecture Elements

| Architecture ID | Parent Specs | Element | Responsibility | Boundary | Verification Target |
|---|---|---|---|---|---|
| ARCH-TIG-001 | SPEC-TIG-001 | Corpus/research evidence layer | Holds corpus rows, axis matrix, TIGER BEAT docs, and publications. | Claims without evidence remain draft or blocked. | Inspect corpus/docs/research links. |
| ARCH-TIG-002 | SPEC-TIG-002, SPEC-TIG-003 | Parliament governance layer | Records stakes, collisions, earned/refuted/retired outcomes, amendments, and handoffs. | Axis evolution is forward-only. | Inspect game panels/tracker/handoffs. |
| ARCH-TIG-003 | SPEC-TIG-004 | Gap design layer | Converts gaps/counter-pressure targets/persona disagreement into originals. | Ideation is not promoted design until reviewed. | Inspect game/gap/design artifacts. |
| ARCH-TIG-004 | SPEC-TIG-005 | Simulation layer | Runs seeded Parliament/UPSTAGE proof with player/run/variant context. | Exploratory output is not a claim. | Run selected `tools/tigris-sim` commands. |
| ARCH-TIG-005 | SPEC-TIG-006 | Shared-engine fixture boundary | Exposes product-owned tabletop fixtures to MUDDLE/COURT/RACKET. | Shared engines do not own TIGRIS rules. | Fixture diagnostics/smokes. |

## Data And Control Flow

```text
corpus/review evidence -> axis/gap map -> design or review target
  -> Parliament outcome -> amendment/handoff/tracker
  -> optional simulation proof -> shared-engine fixture proof
```

## Architecture Risks

| Risk ID | Risk | Mitigation |
|---|---|---|
| RISK-TIG-001 | Corpus/research claims lose evidence links. | Corpus evidence layer and blocked/draft path. |
| RISK-TIG-002 | Axis governance mutates history. | Forward-only Parliament governance boundary. |
| RISK-TIG-003 | Shared engines absorb tabletop rules. | Product-owned fixture boundary. |

## Role Review Summary

No critical or major actionable findings remain. Exact package IDs, interface
schemas, and verification commands are deferred to later stages.
