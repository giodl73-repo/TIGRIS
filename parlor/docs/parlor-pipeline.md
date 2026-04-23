---
name: PARLOR — Party Game Factory Spec
slug: parlor-pipeline
version: 1.0.0
created: 2026-04-23
updated: 2026-04-23
status: canonical
sibling_to: tigris (euro), marathon (D&D), artisan (CERES), chronicle (LUCIA)
---

# PARLOR — Party Game Factory

A Claude-Code-driven factory for social and party games. Designs original party games
through a lightweight Parliament procedure adapted from TIGRIS. Sibling factory to
TIGRIS (euro-tradition), marathon (D&D), artisan (CERES).

**The bet:** Party games earn on E-band axes (A25–A32) that euro games structurally
suppress. PARLOR is the factory that takes those axes seriously. Its first two games
are RITE (TIGRIS #153, the serious euro-adjacent original) and UPSTAGE (PARLOR #1,
the party game proper).

---

## What makes a PARLOR game

- **Weight:** ≤ 2.0 (BGG-estimate). If it needs a 30-minute teach, it's TIGRIS.
- **Duration:** 15–45 minutes.
- **Player count:** 3–8. Party games must scale.
- **Primary axes:** E-band (A25 Emotional Arc, A26 Social Contract, A27 Physical
  Affordance, A28 Cognitive Load, A29 Forgiveness, A30 Strategy Ergodicity, A31
  Failure Texture, A32 Information Architecture) + A7 Emergence.
- **Secondary axes:** A1 Elegance (always relevant), A6 Teachability (high bar —
  must teach in ≤ 5 min), D3 Count-Robustness (must work at 3p AND 8p).
- **Not scored:** B1–B6 (system gearing, conversion chains, etc.) — these are
  structural euro concerns that don't apply to party games. A persona drafting B1
  for a PARLOR game is making a category error.

---

## The PARLOR pipeline

```
CONCEPT → DESIGN → TIER-A [STAKES] → TIER-B [ARGUMENT] → TIER-C [AMENDMENT] → HANDOFF
```

Same structure as TIGRIS. Lighter execution: 4 personas, 2 axes each = 8 stakes.

### Persona roster (4 — party-game relevant subset of TIGRIS 8)

| Persona | Primary PARLOR axes | Why |
|---|---|---|
| **Chvátil** | A26 Social Contract, A32 Info Architecture | Social game corpus: Codenames, The Mind, Skull — he lives here |
| **Stegmaier** | A25 Emotional Arc, A6 Teachability | Accessible design, first-session experience, emotional shape |
| **Kramer-Kiesling** | A27 Physical Affordance, D3 Count-Robustness | Physical games, family scaling, component as experience |
| **Vaccarino** | A30 Strategy Ergodicity, A7 Emergence | Variety, no dominant strategy, replayability |

### Draft structure

4 personas × 2 axes = 8 stakes. Snake order; anchor persona last in Round 1.

### Success criteria (PARLOR)

| Criterion | TIGRIS threshold | PARLOR threshold |
|---|---|---|
| Earned stakes | ≥ 5 | ≥ 3 |
| Earned on target band | ≥ 2 Band B/C | ≥ 1 Band E (A25–A32) |
| Clean collision | ≥ 1 | ≥ 1 |
| Axis adoption/retirement | ≥ 1 | ≥ 1 (first-earn counts) |

### Key difference from TIGRIS

TIGRIS panels argue whether a euro game achieves mechanical depth.
PARLOR panels argue whether a party game earns the social/physical contract
it claims — and whether it creates the emotional conditions for play to feel
alive, not just correct.

The forbidden state in PARLOR is not "silent retire" — it is **dead air**.
A party game that produces dead air in the argument phase has failed.

---

## Directory structure

```
parlor/
├── docs/
│   ├── parlor-pipeline.md       ← this file
│   └── handoff/                 ← session handoffs (same convention as TIGRIS)
├── games/
│   └── NNNN-<slug>/             ← one dir per game
│       ├── concept.md
│       ├── design.md
│       ├── tierA-stakes.md
│       ├── playtests/
│       │   └── PT01-argument.md
│       ├── amendments.md
│       ├── panel/
│       │   └── SUMMARY.md
│       └── handoff.md
├── personas/                    ← symlink or copy from TIGRIS personas/
│   ├── axis-pool.md             (shared with TIGRIS — same Pool, same ledger)
│   └── designers/               (same 8 designers, subset of 4 used)
└── TRACKER.md                   ← PARLOR-specific game log
```

**Shared axis pool:** PARLOR uses the same 24+8 axis Pool as TIGRIS. Earnings in
PARLOR count toward the same adoption/retirement ledger. A party game earning A27
Physical Affordance contributes to A27's corpus record alongside euro earns.

---

## TRACKER.md (PARLOR)

| Version | Date | Game | Earned | Collisions | Notes |
|---|---|---|---|---|---|
| (to be populated as games complete) | | | | | |

---

## Session discipline

Same as TIGRIS: session ends with `/parlor-handoff` (to be built) or manual snapshot
to `parlor/docs/handoff/YYYY-MM-DD-<slug>.md`.

---

## Naming convention

PARLOR game titles are one word, theatrical or social in register.
Current games: UPSTAGE (#1).
Candidate pipeline: UPSTAGE → RITE (shared with TIGRIS #153) → TBD.

Note: RITE (TIGRIS #153) is a joint production — it runs through the TIGRIS
Parliament (full 8-persona, 24-stake review) but earns on PARLOR-relevant axes
(A25, A26, A27, A32). Its ledger entries count in both pipelines' records.
