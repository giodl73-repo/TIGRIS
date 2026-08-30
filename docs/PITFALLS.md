# TIGRIS Pitfalls

## TIGRIS-PF-01: Consensus vocabulary erases the argument

**Pattern:** Review outputs collapse into words such as fun, solid, or consensus
instead of recording which persona attacked or defended which axis at which
moment.

**Domain:** Parliament reviews, summaries, game panels, public README claims,
and research conclusions.

**Why it is hard to catch:** Consensus language reads polished and efficient,
especially when a review has many voices.

**Structural solution:** Enforce forbidden vocabulary and require axis, persona,
player-count or moment anchors, and outcome labels.

**Status:** MITIGATED

**Evidence:** `personas/forbidden-words.md`, `README.md`, and
`docs/vtrace/REQUIREMENTS.md` REQ-TIG-002 and REQ-TIG-003.

## TIGRIS-PF-02: Rubric amendments become committee taste

**Pattern:** An axis is adopted, retired, or reworded because it sounds useful
rather than because repeated play or review evidence triggered the change.

**Domain:** `personas/axis-pool.md`, amendment records, handoffs, and research
papers.

**Why it is hard to catch:** The amended rubric may read better even if the
evidence trail got weaker.

**Structural solution:** Keep the rubric ledger forward-only and require
evidence-triggered adoption or retirement rules.

**Status:** MITIGATED

**Evidence:** `personas/axis-pool.md`, `README.md`, and
`docs/vtrace/REQUIREMENTS.md` REQ-TIG-004.

## TIGRIS-PF-03: Corpus math outruns source custody

**Pattern:** PCA, gap, or profile language is stated more strongly than the
underlying corpus rows, matrix construction, or methodology evidence supports.

**Domain:** TIGER BEAT docs, gap docs, publication drafts, decks, and public
claims.

**Why it is hard to catch:** Tables, dimensions, and scores create a strong
impression of precision.

**Structural solution:** Tie corpus claims to axis matrix rows, methodology
docs, review evidence, and draft/blocked labels when evidence is missing.

**Status:** MITIGATED

**Evidence:** `docs/tiger-beat.md`, `docs/tiger-beat-gaps.md`,
`data/axis-matrix.csv`, and `docs/vtrace/REQUIREMENTS.md` REQ-TIG-001 and
REQ-TIG-008.

## TIGRIS-PF-04: Fixture compatibility becomes rule transfer

**Pattern:** A successful MUDDLE, RALLY, COURT, or RACKET fixture is treated as
permission for another repo to depend on TIGRIS rules, personas, rubric axes, or
skill workflows.

**Domain:** Shared-engine adapters, repo-map dependency records, and portfolio
reuse claims.

**Why it is hard to catch:** Compatibility fixtures are real cross-repo evidence
and can be mistaken for stable exported APIs.

**Structural solution:** Keep fixtures product-owned and require a separate
versioned data/library contract plus consumer-owned compatibility tests before
any inbound reuse claim.

**Status:** MITIGATED

**Evidence:** `README.md` portfolio reuse posture, `repo-map.toml`, and
`docs/vtrace/REQUIREMENTS.md` REQ-TIG-007.
