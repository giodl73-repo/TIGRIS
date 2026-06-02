# TIGRIS CONOPS

## Scope

Repo: TIGRIS

VTRACE stage: CONOPS

Baseline date: 2026-06-01

This CONOPS describes how TIGRIS users preserve the game-design corpus, run
Parliament-style reviews, evolve axes forward-only, design into evidence-backed
gaps, and validate simulation/shared-engine fixtures without transferring
product rules.

## Operational Scenarios

| Scenario ID | Actor | Trigger | Nominal Flow | Degraded / Failure Flow | Evidence Output |
|---|---|---|---|---|---|
| CON-TIG-001 | Parliament operator | A game review or original design review starts. | Draft persona stakes, run playthrough/argument, record earned/refuted/retired axes, amend/handoff. | If stakes lack axis/persona/moment anchors, block claims and revise review artifacts. | Panel summary, amendment, tracker row, handoff. |
| CON-TIG-002 | Corpus claim auditor | Corpus, TIGER BEAT, or publication claim is updated. | Link claim to corpus rows, axis matrix, methodology docs, and review evidence. | If source/corpus/methodology evidence is missing, mark claim draft or blocked. | Corpus row, axis matrix, publication/reference evidence. |
| CON-TIG-003 | Mechanism/tension designer | A design gap or counter-pressure target is selected. | Use gap map and persona disagreement to create original design, then review through Parliament. | If concept lacks gap/counter-pressure trace, keep as ideation rather than promoted design. | Game concept, design file, panel review, gap evidence. |
| CON-TIG-004 | Simulation maintainer | Parliament or UPSTAGE simulation proof is requested. | Run seeded `tools/tigris-sim` smokes and compare variants with RALLY-backed telemetry. | If seed/run/player counts are missing, treat output as exploratory only. | Simulation output, telemetry, variant comparison. |
| CON-TIG-005 | Shared-engine adapter maintainer | MUDDLE, COURT, or RACKET needs tabletop fixture proof. | Expose Parliament/UPSTAGE product-owned fixture or surface while TIGRIS owns rules. | If shared engine needs TIGRIS rules, reject boundary and add interface requirements. | MUDDLE save/transcript, COURT snapshot, RACKET diagnostic/runtime proof. |

## Operating Modes

| Mode | Purpose | Entry Condition | Exit Condition |
|---|---|---|---|
| Parliament review | Turn play evidence into axis decisions. | Game or original design is selected. | Handoff and tracker row record review outcome. |
| Corpus governance | Keep claims source-backed. | Corpus or publication claim changes. | Evidence links satisfy review or claim is blocked. |
| Gap design | Produce original games from discovered design space. | Gap/counter-pressure target exists. | Design is reviewed or held as ideation. |
| Simulation fixture | Validate product-owned mechanics through seeded runs. | Simulation target and seed exist. | Variant output supports claim or remains exploratory. |
| Shared-engine fixture | Exercise MUDDLE/COURT/RACKET without rule transfer. | Fixture is stable. | Adapter proof passes and product ownership is preserved. |

## Role Review Summary

Role lenses applied from `.roles/`: game-space cartography, mechanism/tension
editing, axis governance, corpus claim auditing, and table-experience
observation.

Findings:

| Role | Finding | Disposition |
|---|---|---|
| Axis Governance Steward | CONOPS must preserve earned/refuted/retired axis paths. | Addressed in CON-TIG-001. |
| Corpus Claim Auditor | Research and TIGER BEAT claims need blocked/draft paths. | Addressed in CON-TIG-002. |
| Mechanism Tension Editor | Originals must trace to gaps or counter-pressure targets. | Addressed in CON-TIG-003. |
| Table Experience Observer | Simulations require seed/run/player context. | Addressed in CON-TIG-004. |

Fixed-point decision:

No critical or major actionable findings remain for the CONOPS stage. Exact
requirement IDs, validation command levels, interface rows, and work packages
are deferred to later VTRACE stages.
