---
name: TIGRIS Collision Adjacency Chart (canonical)
slug: adjacency-chart
version: 1.0.0
rubric_version: v2.3.1
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
consolidates:
  - Parliament design.md §9 (9 pairs)
  - I-te-01 (B2↔C5)
  - I-facets-01 (C6↔A6)
  - I-facets-02 (A2↔A6)
---

# Collision Adjacency Chart — canonical

**This is the canonical source for collision adjacencies in TIGRIS reviews.** Parliament's `design.md §9` remains a subset of this chart (the in-game version for playing Parliament). This file applies to all TIGRIS review panels.

Amendments to this chart are low-risk patches applied outside the rate-limit (per cycle-protocol): adjacency discoveries are observational, not bet-altering. Updated via `personas/playtest-innovations.md` entries with `trigger_pattern: new_adjacency_surfaced`.

## Active adjacencies (as of v2.3.1)

Pairs of axes whose stakes can **collide** when drafted by different personas in the same review. When both stakes fire on the same moment-anchor, a collision resolves by non-colliding-reviewer vote.

| # | Left | Right | Theme | Source |
|---|---|---|---|---|
| 1 | C1 Tension Budget | C5 Anti-Catch-up Pressure | Pressure management across arc | Parliament §9 |
| 2 | C1 Tension Budget | C3 Scarcity Bite | Resource-under-want | Parliament §9 |
| 3 | A1 Elegance | C7 Action-Menu Clarity | Tight rule sets | Parliament §9 |
| 4 | A2 Decision Density | C6 Point-Salad Incommensurability | Per-turn richness | Parliament §9 |
| 5 | A3 Interaction | D2 Spatial-Interaction Presence | Direct board-coupling | Parliament §9 |
| 6 | A7 Emergence/Replayability | B5 Architectural Novelty | Strategy-space diversity | Parliament §9 |
| 7 | B1 System Gearing | C4 Engine-Garden Dependency | Subsystem interdependence | Parliament §9 |
| 8 | B4 Info-Transparency-Cost | A6 Teachability | Inverse — high cost = low teach | Parliament §9 |
| 9 | B3 Conversion Chain Depth | C2 Min-Score Shape | Scoring non-linearity | Parliament §9 |
| 10 | A4 Variance Calibration | B2 Catastrophe Pressure | Reversal events management | Parliament §9 |
| 11 | D4 Late-Game Lock-in Point | C5 Anti-Catch-up Pressure | End-game structure | Parliament §9 |
| 12 | D3 Count-Robustness | D1 Family-to-Expert Scaling | Access across audiences | Parliament §9 |
| 13 | C8 First-Turn Compression | A6 Teachability | Onboarding mechanics | Parliament §9 |
| 14 | A5 Downtime/Pacing | C7 Action-Menu Clarity | Flow management | Parliament §9 |
| **15** | **B2 Catastrophe Pressure** | **C5 Anti-Catch-up Pressure** | Punish-the-leader mechanism | **I-te-01 (T&E Turn 5, adopted v2.3.1)** |
| **16** | **C6 Point-Salad Incommensurability** | **A6 Teachability** | Incommensurable scoring requires teachable rules | **I-facets-01 (FACETS Turn 8, adopted v2.3.1)** |
| **17** | **A2 Decision Density** | **A6 Teachability** | Many choices + clear menu | **I-facets-02 (FACETS Turn 4, adopted v2.3.1)** |

**17 active adjacencies.** Adjacency chart is open-ended — new pairs discovered in future panels are logged and adopted on user approval (low-risk).

## Rules

- A collision fires when both sides of an adjacency pair are staked at the same moment-anchor by DIFFERENT personas.
- Same-player adjacencies (one persona drafts both sides) do NOT collide — no internal debate.
- Non-colliding reviewers vote on "which axis better fits the moment." Majority wins. Ties broken by earliest-drafted axis.
- Winner earns a collision credit (+1 defend-equivalent). Loser takes single-moment refutation.

## Proposing new adjacencies

When two stakes fire on the same moment in a review and no chart entry exists, the reviewer can flag a `new_adjacency_surfaced` innovation candidate. User approves for adoption. Chart updates forward-only (prior games don't retro-adjust).

## Changelog

- **v2.3.1** — 2026-04-19 — Adopted 3 new adjacencies (I-te-01, I-facets-01, I-facets-02). Canonical chart file created. Previously adjacencies lived in Parliament's design.md §9; now this file is canonical and Parliament §9 is a subset applied to Parliament-the-game-itself.
- **v2.0** — Parliament design.md §9 — 14 initial adjacencies.
