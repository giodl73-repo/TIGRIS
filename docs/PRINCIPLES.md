# TIGRIS Principles

## TIGRIS-P-01: Well-formed disagreement beats consensus

**Statement:** TIGRIS produces inspectable disagreement among designer personas,
not smoothed consensus.

**Rationale:** The factory's design value comes from incompatible stakes,
attacks, defenses, collisions, and amendments. Consensus language hides whether
an axis, mechanism, or player-count claim actually survived pressure.

**Decision rule:** A review claim must name the axis, persona, player count or
moment anchor, and outcome instead of using broad evaluative vocabulary.

**Consequence:** A game can be valuable because it creates a readable argument,
even when individual stakes are refuted.

**Evidence:** `CLAUDE.md`, `README.md`, `personas/forbidden-words.md`, and
`docs/vtrace/REQUIREMENTS.md` REQ-TIG-002 and REQ-TIG-003.

**Status:** ACTIVE

## TIGRIS-P-02: Rubric evolution is forward-only

**Statement:** TIGRIS axes are adopted, retired, or held from recorded play and
review evidence; prior scores are not silently rewritten.

**Rationale:** The corpus only remains comparable if every axis change preserves
the version and evidence that produced it.

**Decision rule:** Change an axis only through the rubric ledger, amendment
records, and handoff state; never edit history to make older reviews match the
current rubric.

**Consequence:** Superseded or retired axes remain part of the evidence trail.

**Evidence:** `personas/axis-pool.md`, `README.md`, and
`docs/vtrace/REQUIREMENTS.md` REQ-TIG-004.

**Status:** ACTIVE

## TIGRIS-P-03: Product-owned fixtures are not shared APIs

**Statement:** TIGRIS can expose MUDDLE, RALLY, COURT, and RACKET fixtures
without transferring TIGRIS rules, personas, or rubric semantics into shared
engines.

**Rationale:** Shared engine compatibility proves that the table slice can be
described externally; it does not make TIGRIS a reusable rules library.

**Decision rule:** Treat cross-repo outputs as product-owned fixtures unless a
separate versioned contract, downstream manifest, and consumer-owned
compatibility test exist.

**Consequence:** Shared infrastructure can validate boundaries without absorbing
TIGRIS game design semantics.

**Evidence:** `README.md` portfolio reuse posture, `repo-map.toml`, and
`docs/vtrace/REQUIREMENTS.md` REQ-TIG-007.

**Status:** ACTIVE
