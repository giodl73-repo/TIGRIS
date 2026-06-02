# TIGRIS Communications Strategy

## Purpose

This artifact maps accepted TIGRIS VTRACE intent to user-facing docs surfaces.
The docs package explains corpus claims, Parliament review, original-game gap
design, simulator evidence, and shared-engine fixture boundaries without
claiming unsupported fun or quality outcomes.

## Surface Plan

| Surface ID | Source IDs | Audience | User Question | Generated Docs | Cadence | Owner | Status |
|---|---|---|---|---|---|---|---|
| COMMS-TIG-README-001 | NEED-TIG-001 / MSC-TIG-001 / WP-TIG-001 | game-space cartographer / future agent | Where do I start, and what corpus or game state is current? | `docs/README.md` docs map plus README routing | every docs wave | TIGRIS maintainer | planned |
| COMMS-TIG-CORPUS-001 | NEED-TIG-001 / REQ-TIG-001 / WP-TIG-001 | corpus auditor / research reviewer | What evidence supports a TIGER BEAT or research claim? | `docs/concepts/corpus-claim-checklist.md` | when corpus methodology changes | TIGRIS corpus owner | planned |
| COMMS-TIG-PARLIAMENT-001 | NEED-TIG-003 / REQ-TIG-004 / WP-TIG-004 | Parliament operator / axis steward | How does Parliament preserve disagreement and outcomes? | `docs/tutorials/parliament-review-path.md` | when Parliament packet rules change | TIGRIS Parliament owner | planned |
| COMMS-TIG-GAP-001 | NEED-TIG-002 / VAL-TIG-002 / WP-TIG-004 | game designer / mechanism editor | How do gaps or counter-pressures become original games? | `docs/concepts/gap-design-packets.md` | when gap-design packet rules change | TIGRIS design owner | planned |
| COMMS-TIG-SIM-001 | NEED-TIG-004 / IF-TIG-003 / WP-TIG-002 | simulator analyst / reviewer | How do I run and interpret seeded simulator evidence? | `docs/how-to/run-tigris-sim.md` | when simulator command shape changes | TIGRIS sim owner | planned |
| COMMS-TIG-FIXTURE-001 | NEED-TIG-004 / IF-TIG-004 / WP-TIG-003 | MUDDLE, COURT, RACKET, and RALLY maintainers | What fixture behavior is product-owned by TIGRIS? | `docs/examples/shared-engine-fixtures.md` | when shared-engine fixture evidence changes | TIGRIS fixture owner | planned |
| COMMS-TIG-DOCS-001 | REV-TIG-003 / WP-TIG-001 / WP-TIG-002 / WP-TIG-003 / WP-TIG-004 | docs owner / future agent | Who owns corpus, Parliament, simulator, and fixture docs updates? | `docs/CORPUS.md` | every docs wave | TIGRIS docs owner | planned |

## Review Checklist

| Item | Required | Decision | Evidence / Rationale |
|---|---|---|---|
| Docs claims trace to controlled source IDs. | yes | accepted | Rows cite mission, requirements, interfaces, validation, work packages, and review rows. |
| Concepts/tutorials/examples do not overclaim unvalidated behavior. | yes | accepted | Corpus, simulation, and shared-engine rows preserve evidence and ownership limits. |
| Public interfaces have expected usage or expected output docs. | if applicable | accepted | Simulator and shared-engine fixture rows map to how-to or example docs. |
| `docs/CORPUS.md` names ownership and update obligations. | if multiple surfaces exist | planned | COMMS-TIG-DOCS-001 records the corpus surface. |
