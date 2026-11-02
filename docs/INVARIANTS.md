# TIGRIS Invariants

## TIGRIS-I-01: Review claims carry anchors

**Claim:** Parliament and review claims cite axis, persona, and player count or
moment anchor.

**Why it matters:** Unanchored praise or criticism cannot be tested against the
factory's play record or corpus.

**Test:** `docs/vtrace/VERIFICATION.md` VER-TIG-002 and inspection of game
panels, personas, `TRACKER.md`, and handoffs.

**Status:** VERIFIED

## TIGRIS-I-02: Axis ledger changes are forward-only

**Claim:** Axis adoption, retirement, monitoring, and live status changes are
recorded in the rubric ledger without silent retroactive score changes.

**Why it matters:** TIGER BEAT and the review corpus depend on versioned axis
history.

**Test:** `personas/axis-pool.md` rubric ledger inspection and
`docs/vtrace/REQUIREMENTS.md` REQ-TIG-004.

**Status:** VERIFIED

## TIGRIS-I-03: Corpus claims remain evidence-bound

**Claim:** TIGER BEAT, gap, corpus, and research claims are tied to corpus rows,
axis matrices, methodology docs, or explicit draft/blocked status.

**Why it matters:** Design-space claims can look mathematical even when source,
methodology, or corpus evidence is incomplete.

**Test:** `docs/vtrace/VERIFICATION.md` VER-TIG-001 over `data/axis-matrix.csv`,
`docs/tiger-beat.md`, `docs/tiger-beat-gaps.md`, methodology docs, and research
links.

**Status:** VERIFIED

## TIGRIS-I-04: Simulation claims carry run context

**Claim:** Simulation claims cite seed, run count, player count, game target,
and variant context.

**Why it matters:** Seeded simulation output is reproducible only when its run
context travels with the claim.

**Test:** `docs/vtrace/VERIFICATION.md` VER-TIG-003 through VER-TIG-005 and
`tools/tigris-sim` command outputs.

**Status:** PARTIAL
