# TIGRIS Work Packages

## Scope

Repo: TIGRIS

VTRACE stage: Work Packages

Baseline date: 2026-06-01

## Package Backlog

| WP ID | Source | Title | Outcome | Verification / Validation | Status |
|---|---|---|---|---|---|
| WP-TIG-001 | GAP-TIG-001, REV-TIG-001 | Lock corpus/research claim checklist | `TRACKER.md` and TIGER BEAT/research docs preserve corpus rows, axis versions, methodology links, BGG/reference anchors, and draft/blocked handling as the current claim checklist. | VER-TIG-001, VAL-TIG-001 | complete |
| WP-TIG-002 | GAP-TIG-002, REV-TIG-002 | Execute simulator evidence packet | `cargo test --quiet`, seeded Parliament smoke, and 20-run Parliament variant comparison passed with seed, player count, run count, variant, and claim labels. | VER-TIG-003, VER-TIG-004, VER-TIG-005, VAL-TIG-004 | complete |
| WP-TIG-003 | GAP-TIG-003, REV-TIG-003 | Prove shared-engine fixture boundary | `cargo run --quiet --bin tigris-muddle -- --save tigris.muddle` passed; fixture code also exposes COURT/RACKET/RALLY boundary claims while TIGRIS owns table rules. | VER-TIG-007, VAL-TIG-005 | complete |
| WP-TIG-004 | TR-TIG-002, TR-TIG-003 | Lock Parliament outcome and gap-design packets | `TRACKER.md` preserves Parliament outcomes, zero-silent-retire handoffs, original-game gap/counter-pressure anchors, and axis/persona/moment evidence as the current packet contract. | VER-TIG-002, VER-TIG-006, VAL-TIG-002, VAL-TIG-003 | complete |

## Execution Rules

- Unsupported corpus/research claims remain draft or blocked.
- Axis evolution is forward-only.
- Simulation output without full context remains exploratory.
