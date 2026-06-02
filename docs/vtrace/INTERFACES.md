# TIGRIS Interfaces

## Scope

Repo: TIGRIS

VTRACE stage: Interfaces

Baseline date: 2026-06-01

## Interface Matrix

| Interface ID | Parent Architecture | Interface | Producer | Consumer | Contract | Evidence |
|---|---|---|---|---|---|---|
| IF-TIG-001 | ARCH-TIG-001 | Corpus claim packet | Corpus, TIGER BEAT docs, research files | Corpus auditor, publication reviewer | Packet cites corpus rows, axis matrix, methodology docs, and review evidence or remains draft/blocked. | Inspect data/docs/research links. |
| IF-TIG-002 | ARCH-TIG-002 | Parliament review packet | Review operator and personas | Axis steward, handoff reviewer | Packet records stakes, axis/persona/player or moment anchor, collisions, earned/refuted/retired outcomes, amendments, and handoff. | Inspect game panels/tracker/handoffs. |
| IF-TIG-003 | ARCH-TIG-003 | Gap design packet | Gap map, persona disagreement, mechanism editor | Original game designer | Packet ties concept to design gap, counter-pressure target, or documented disagreement before promotion. | Inspect game/gap/design artifacts. |
| IF-TIG-004 | ARCH-TIG-004 | Simulation telemetry packet | `tools/tigris-sim` | Simulation reviewer, RALLY fixture | Packet names seed, run count, player count, game target, variant context, and exploratory/promoted label. | Run selected simulator commands. |
| IF-TIG-005 | ARCH-TIG-005 | Shared-engine tabletop fixture | TIGRIS fixture exporter | MUDDLE, COURT, RACKET, RALLY | Fixture exposes TIGRIS-owned tabletop state/rules evidence while shared engines own adaptation/rendering/telemetry behavior. | Fixture diagnostics/smokes. |

## Boundary Rules

- Corpus and publication claims without source/methodology evidence remain draft or blocked.
- Axis adoption and retirement are forward-only.
- Simulation output without seed/run/player/game/variant context is exploratory.
- Shared engines must not own TIGRIS rules.

## Role Review Summary

Role lenses applied from `.roles/`: game-space cartography, mechanism/tension
editing, axis governance, corpus claim auditing, and table-experience
observation.

No critical or major actionable findings remain. Exact claim checklist,
simulator command levels, and shared-engine schema rows are deferred to later
stages.
