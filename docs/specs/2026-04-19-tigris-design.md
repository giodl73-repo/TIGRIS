---
name: TIGRIS Factory — Design Blueprint (v1.0 — SUPERSEDED)
slug: tigris-design
date: 2026-04-19
author: giodl@microsoft.com (+ Claude)
tradition: Euro / designer
status: superseded by docs/specs/2026-04-19-tigris-v2.0-design.md (Parliament bet)
superseded_on: 2026-04-19
superseded_reason: v1.0 panel review surfaced a 5-voice orthogonality cluster (personas chorused rather than argued). v2.0 replaces the shared 8-axis rubric with a 24-axis Pool and disjoint drafting.
rubric_version: v1.0
bet_version: chorus
supersedes: none
inherits_from:
  - C:\src\marathon (7-stage pipeline, innovation log, seed tracker, player lenses)
  - C:\src\puzzlehunt (blind persona testing, staged gates, content-driven mechanism)
  - C:\src\chronicle (Anchor Rule, rubric amendments, multi-lens friction)
  - C:\src\artisan (tiered simulation stack, matrix format, scale-as-axis)
  - C:\src\rmm (signal-artifact discipline, handoff continuity)
---

> **This v1.0 spec has been superseded** by `docs/specs/2026-04-19-tigris-v2.0-design.md` (the Parliament restructure). See `docs/specs/reviews/2026-04-19-tigris-design/SUMMARY.md` for the panel review that triggered the restructure, and `personas/playtest-innovations.md` for the full amendment history. v1.0 is kept in the repo as history — do not use it as a current reference.



# TIGRIS — A Board Game Factory

> Named for Tigris & Euphrates (Knizia, 1997). Built on the bones of marathon, puzzlehunt, chronicle, rmm, and artisan. Markdown-first. Euro-tradition. Ships designs **and** reviews of existing games through the same pipeline.

---

## 1. Purpose

TIGRIS is a Claude-Code-driven factory that:

1. **Designs new Euro-tradition board games** end-to-end (concept → design → playtest → panel → publish).
2. **Reviews existing published games** through the same pipeline, so the rubric is calibrated against masterworks.
3. **Evolves its own design guidelines** via an append-only innovation log with cluster-triggered rubric amendments (forward-only versioning, per marathon / chronicle).

Every game — new or existing — passes through the same seven stages and produces the same artifact shape. Reviews and originals differ only at stages 1–2 (CONCEPT/DESIGN): originals write them; reviews import published rules.

---

## 2. Tradition & Voice

**Anchor tradition:** Euro / designer. Reiner Knizia is the spiritual patron. The rubric favours elegance, decision density, interaction, and mechanical coherence over narrative immersion, dice-drama, or production gloss.

**Forbidden framings** (audited by /tigris-clean when built):
- "Fun", "well-designed", "great game", "works well", "solid" — all too vague.
- "Euro vs Ameritrash" as evaluative framing — we evaluate *this* game against *its own ambition*.
- "Unbalanced" without naming the specific dominant strategy and player-count where it emerges.

**Required language:** every claim names (a) which rubric axis, (b) which persona/lens flagged it, (c) which player count or game-state triggered it.

---

## 3. Architecture Overview

```
Approach A (Marathon-clone) + two adaptations:
  + CERES Tier-A matrix format inside tierA-sim.md
  + chronicle Anchor Rule: ship Tigris & Euphrates review end-to-end before building skill #2
```

**Pipeline (per game):**

```
CONCEPT → DESIGN → TIER-A → [GATE] → TIER-B → PANEL → INNOVATE → HANDOFF
         (skip for reviews)           ↑
                                      └─ failed gate → revise DESIGN, re-run TIER-A
```

- **CONCEPT + DESIGN**: for originals only. Reviews start at TIER-A with published rules pasted into `design.md`.
- **TIER-A**: persona × player-count × axis matrix; fast rubric scoring (~30 min of model time).
- **GATE**: hard threshold. Failed gates block TIER-B; design must revise and re-score. Thresholds defined in §7.
- **TIER-B**: one seeded narrated playthrough per non-trivial player count. Usually 2p, 3p, 4p.
- **PANEL**: each designer persona writes a full review, weighted aggregate in SUMMARY.md.
- **INNOVATE**: append per-game innovation candidates to `personas/playtest-innovations.md`. Cluster triggers amendment proposal.
- **HANDOFF**: end-of-cycle state capture; next session reads this first.

**Tier C (tournament)** is a separate, commissioned skill — triggered only when a specific question demands N-seat agent play. Not part of the default cycle.

---

## 4. Directory Layout

```
C:\src\tigris\
├── CLAUDE.md                       # house rules, forbidden words, frontmatter contract
├── README.md                       # project overview + pipeline map
├── TRACKER.md                      # per-game status log, rubric version history
├── MODULES.md                      # (optional) multi-author callsign assignments
│
├── docs/
│   └── specs/                      # design docs (this file lives here)
│
├── personas/
│   ├── designers/
│   │   ├── knizia.md               # Reiner Knizia — elegance, tension budgets
│   │   ├── rosenberg.md            # Uwe Rosenberg — worker placement, compact mastery
│   │   ├── feld.md                 # Stefan Feld — point salad, every-turn-matters
│   │   ├── lacerda.md              # Vital Lacerda — interconnected systems
│   │   ├── chvatil.md              # Vlaada Chvátil — structural innovation
│   │   ├── kramer-kiesling.md      # elegance + theme, family-to-expert scaling
│   │   └── stegmaier.md            # Jamey Stegmaier — accessibility, count-robustness
│   ├── player-lenses/
│   │   ├── engine-builder.md       # compounding optimizer; disruption-averse
│   │   ├── interactionist.md       # seeks confrontation; bored by solitaire
│   │   ├── ap-prone.md             # decision paralysis flag; measures clarity
│   │   ├── thematic.md             # immersion-seeker; disengages on pure abstract
│   │   └── fresh-face.md           # first-time player; teachability lens
│   ├── playtest-rubric.md          # v1.0 — 8 axes, definitions, anchors, current amendments
│   ├── playtest-innovations.md     # append-only innovation log
│   └── forbidden-words.md          # vocabulary discipline (see §2)
│
├── games/                          # each game is a numbered directory
│   └── NNNN-<slug>/
│       ├── concept.md              # (originals only) premise, count, length, artifact
│       ├── design.md               # rules, components, turn structure, edge cases
│       ├── tierA-sim.md            # matrix: persona × count × axis → verdict+metric
│       ├── playtests/
│       │   ├── PT01-<persona>-<count>p.md
│       │   ├── PT02-...
│       │   └── TOURNAMENT.md       # Tier C, when commissioned
│       ├── panel/
│       │   ├── knizia.md
│       │   ├── rosenberg.md
│       │   ├── ...
│       │   ├── lenses/
│       │   │   ├── engine-builder.md
│       │   │   └── ...
│       │   └── SUMMARY.md          # weighted consensus, punchlist, verdict
│       ├── innovations.md          # this game's innovation candidates
│       └── handoff.md              # end-of-cycle state, next-session brief
│
├── skills/                         # Claude Code skills (built as needed per YAGNI)
│   ├── tigris-tierA/
│   ├── tigris-panel/
│   ├── tigris-innovate/
│   ├── tigris-handoff/
│   ├── tigris-concept/             # Phase 2
│   ├── tigris-design/              # Phase 2
│   ├── tigris-tierB/               # Phase 2
│   ├── tigris-tierC/               # Phase 3 (deferred)
│   ├── tigris-resume/              # Phase 3 (deferred)
│   └── tigris-status/              # Phase 3 (deferred)
│
├── reference/
│   ├── rules/                      # cached public-domain / quoted rules excerpts
│   ├── designers/                  # research notes per designer
│   └── community/                  # BGG discussion archives, essays, interviews
│
└── scripts/
    └── seed-rng.sh                 # deterministic RNG for Tier B/C playthroughs
```

Numbered game directories (0001, 0002, …) mirror marathon's `adventures/` convention for stable sort + seed-tracker joins.

---

## 5. Personas

### 5.1 Designer roster (review/panel lens)

Seven voices drawn from the Euro canon. Each persona file contains:

```yaml
---
name: <full designer name>
slug: <lowercased-hyphenated>
role: designer-persona
tradition: euro
signature_works: [list]
rubric_weights:           # sum = 8.0 across the 8 axes
  elegance: 1.5
  decision_density: 1.2
  ...
---

# <Name>

## Design philosophy
300-500 words in their voice / written about their approach.

## Signature moves
5-8 bullet techniques this designer reaches for.

## Red flags (what they'd push back on)
- ...

## Greenlights (what they'd praise)
- ...

## Diagnostic questions (10-12)
- "Does the rule count justify the strategic depth?"
- "Where does the Knizia-triangle appear — or is VP conversion linear?"
- ...
```

**Starter roster (build in this order):**
1. Knizia — anchor voice, strongest fit with T&E review.
2. Rosenberg — worker-placement / action-economy authority.
3. Feld — decision-density / point-salad authority.
4. Chvátil — structural innovation / "what is a game even".
5. Lacerda — deep-puzzle / interconnected-systems authority.
6. Kramer-Kiesling — family-to-expert elegance.
7. Stegmaier — accessibility + player-count robustness.

Reviews with fewer than 5 designer voices don't produce a SUMMARY.md.

### 5.2 Player lens roster (playtest scoring)

Five playstyles, modeled on marathon's lens system:

| Lens | Optimizes for | Frustrated by | Weight signature |
|---|---|---|---|
| **Engine-Builder** | long-term compounding, efficient conversions | disruption, take-that, short games | elegance↑ decision_density↑ interaction↓ |
| **Interactionist** | direct confrontation, zero-sum moments | multiplayer solitaire | interaction↑↑ variance↑ |
| **AP-Prone** | clear choices, visible information | decision paralysis, hidden scoring | elegance↑ teachability↑ decision_density → paradoxically ↓ |
| **Thematic** | story / narrative coherence | pure abstraction | thematic_integration↑↑ emergence↑ |
| **Fresh-Face** | teachability, first-session success | rules surface, edge cases | teachability↑↑ elegance↑ |

Each lens's weight signature reshapes the 8-axis score for that lens's review.

### 5.3 Persona evolution

New personas are added when the innovation log captures 3+ instances of a perspective the existing roster can't express (chronicle-style amendment mechanism).

---

## 6. Rubric

**Eight axes.** Each scored 0–10 per cell. Axes are weighted per persona/lens.

| # | Axis | Definition | Anchor 0 | Anchor 5 | Anchor 10 |
|---|---|---|---|---|---|
| 1 | **Elegance** | Rule-count-to-depth ratio | Rulebook bloats; each rule adds <1 strategic concept | Rules match depth | Rule count understates depth; emergent complexity |
| 2 | **Decision Density** | Meaningful choices per turn | Many turns auto-resolve | ~1 real decision per turn | Every turn presents a multi-axis choice |
| 3 | **Interaction** | Player-to-player impact | Multiplayer solitaire | Indirect competition for shared resources | Direct confrontation drives the arc |
| 4 | **Thematic Integration** | Mechanics ↔ theme coherence | Theme is paint | Mechanics and theme rhyme | Mechanics **are** the theme (e.g., T&E's balance of the four aspects) |
| 5 | **Variance Calibration** | Luck scaled to game length | Random-enough to nullify skill over full arc | Luck early, skill late | Luck is a resource players manage |
| 6 | **Downtime / Pacing** | Wait between turns | >5 min between meaningful player activity | Moderate AP-tolerant pacing | Turns overlap or always present a decision |
| 7 | **Teachability** | Rules → first-turn friction | Rulebook required mid-game | Can play after one read | Can teach in under 10 min |
| 8 | **Emergence / Replayability** | Strategy space diversity | Solved game; dominant opener | Two or three paths | Genuinely different arcs across plays; no solved state |

**Rubric version:** v1.0 at spec approval. Amendments logged in `personas/playtest-rubric.md` changelog. Forward-only: prior scores locked at their scoring version.

**Cell verdict shorthand** (CERES-style, used in TIER-A matrix). A *cell* is one persona (or lens) evaluating one player-count; its cell score is the **weighted aggregate** of that persona's 8 axis scores using their rubric_weights vector (0–10 scale preserved):
- `win` — weighted aggregate ≥ 7.0
- `marginal` — 4.0–6.9
- `fail` — < 4.0

Underneath every matrix cell, `tierA-sim.md` also records the 8 individual axis scores that fed the aggregate (compact view at top, per-cell expansion below).

---

## 7. The Seven-Stage Pipeline

### 7.1 CONCEPT (originals only)

Output: `concept.md`. ~500 words. Fields:
- Premise (one paragraph, player-facing hook)
- Target player count range
- Target play length
- Anchor mechanic (the one system the whole game is built around)
- **Artifact** — the physical object whose constraints drive design (board? deck? tile bag?) (marathon "artifact-as-story" port)
- Inspiration / lineage (what it descends from, honestly)

### 7.2 DESIGN

Output: `design.md`. Complete rules as they'd appear in a rulebook. For reviews: paste / cite published rules. Use consistent section order:
1. Overview & Object of the Game
2. Components
3. Setup
4. Turn structure
5. Actions
6. Scoring / end condition
7. Edge cases & clarifications

### 7.3 TIER-A — Rubric Simulation Matrix

Output: `tierA-sim.md`. The heart of the fast-path. A CERES-style matrix:

```
            2p           3p           4p           5p+
knizia:     win/9.1      win/8.8      marginal/6.2  fail/3.1
rosenberg:  marginal/6.0 win/7.6      win/7.9       marginal/5.4
feld:       ...
engine-b:   ...
ap-prone:   ...
```

Each cell shows `<verdict>/<weighted-aggregate>` where the aggregate is the persona's 8 axis scores combined via their `rubric_weights` vector (thresholds in §6). Below the compact matrix, `tierA-sim.md` expands each cell into its 8 per-axis scores plus a one-sentence note per axis explaining the call. Matrix cells are computed by invoking each persona's diagnostic questions against the DESIGN document — no turn-by-turn play.

**Expected model time:** 30–60 min for 7 designers × 5 lenses × 4 counts = 48 cells.

### 7.4 GATE

Hard threshold before TIER-B begins:
- **Minimum:** ≥ 50% of cells are `marginal` or better (by aggregate score, §6).
- **Anchor-cell:** the persona-count cell the designer declared (e.g., "Feld at 4p" for a point-salad game) must be `win`. Misses force a redesign.
- **No critical fail-cluster:** at the target player count, no single axis scores ≤ 3 across ≥ 3 personas. (I.e., if three designers independently mark Decision Density = 2 at 4p, the design has a structural flaw that must be fixed, even if the aggregate cell verdicts are acceptable.)

Failed gates produce `tierA-sim.md` with a REVISE verdict at the bottom. Design must iterate; no TIER-B until GATE passes.

### 7.5 TIER-B — Narrated Playthrough

Output: `playtests/PT<NN>-<persona>-<count>p.md`. One narrated playthrough per anchor count. The playthrough agent:
- Seats one persona at each chair (e.g., Knizia-AI + Engine-Builder + Interactionist + AP-Prone at 4p)
- Uses seeded RNG (`scripts/seed-rng.sh`)
- Narrates every decision in-character per the persona's playstyle heuristic
- Logs each turn: decision space, choice, reasoning, board state delta
- Ends with a per-persona verdict on their own experience (did the game deliver what their lens values?)

Total length: 3000–5000 words per playthrough. Expect 3 playthroughs per game (2p, 3p, 4p).

### 7.6 PANEL

Output: `panel/<persona>.md` (one per designer) + `panel/lenses/<lens>.md` (one per lens) + `panel/SUMMARY.md`.

Each panel file: 800–1500 words. Structure:
- Opening verdict (one paragraph, in-voice)
- Three greenlights (what the game does well per this lens)
- Three red flags (what the game does poorly per this lens)
- Amendment candidates (what rubric axis this game exposes as under-specified)
- Final score per the 8 axes (weighted by this persona)

SUMMARY.md weights each designer's scores by their canonical weight (all 1.0 by default; overridden by the anchor-designer declaration).

### 7.7 INNOVATE

Output: append to `games/NNNN-<slug>/innovations.md` AND `personas/playtest-innovations.md`.

Each innovation: `I-<game>-<NN>` entry with:
- Dimension (which rubric axis)
- Trigger (what game state / persona observation produced the note)
- Scope (this-game / pattern-across-games / universal)
- Status (candidate / proposed-amendment / adopted)

**Cluster trigger:** 2+ innovations in the same dimension across ≥ 2 games → propose rubric amendment. 3+ around a player-behavior theme → propose new lens (e.g., "meta-gamer", "pure-luck-tolerator"). User approves amendments; rubric version bumps.

### 7.8 HANDOFF

Output: `games/NNNN-<slug>/handoff.md`. Marathon-pattern. Fields:
- Locked (campaign-permanent facts about this game)
- Pending (open questions, deferred decisions)
- Next session brief (what to work on next)

Also appends a row to top-level `TRACKER.md`.

---

## 8. Tiered Simulation Stack (CERES port)

| Tier | Mode | Cost | Trigger | Output |
|---|---|---|---|---|
| **A** | Rubric-scored matrix | 30–60 min model time | Every game, every pipeline run | `tierA-sim.md` |
| **B** | Narrated persona playthrough | 2–4 hours model time | After GATE passes | `playtests/PT<NN>-*.md` |
| **C** | Multi-seat agent tournament | 6–12+ hours, possibly days | Commissioned by specific question (e.g., "does the runaway-leader effect activate above 4p?") | `playtests/TOURNAMENT.md` |

Tier A runs on **every** game. Tier B runs only after Tier A passes gate. Tier C is opt-in per game.

---

## 9. Artifact Contracts

Every generated file starts with YAML frontmatter:

```yaml
---
name: <human-readable title>
slug: <hyphenated>
game: <NNNN-slug>                     # which game this belongs to
stage: concept|design|tierA|tierB|panel|innovate|handoff
version: <semver, e.g., 1.0.0>
rubric_version: v1.0                   # which rubric this was scored against
author: <persona-slug or human>
created: YYYY-MM-DD
updated: YYYY-MM-DD
sources: [list of citations]
---
```

This lets downstream skills traverse, filter, and re-score programmatically.

---

## 10. Anchor Plan — Tigris & Euphrates Review

**The Anchor Rule (chronicle):** run Tigris & Euphrates end-to-end through the full pipeline before building any Phase 2 skill.

Anchor game: `games/0001-tigris-and-euphrates/`. It's a *review*, so CONCEPT is a single paragraph noting "imported" and DESIGN is the published Knizia rules (cited, not reproduced in full — copyright posture: summary + excerpt under fair use; `reference/rules/tigris-and-euphrates.md` holds the detailed summary).

Pipeline for the anchor:
1. DESIGN (import published rules summary) — ~1 session
2. TIER-A (7 designers × 5 lenses × 3 counts = 36 cells) — ~1 session
3. GATE — expect `win` trivially; anchor's purpose is to calibrate the rubric, not block it
4. TIER-B (3 playthroughs: 2p, 3p, 4p) — ~3 sessions
5. PANEL (7 designer + 5 lens reviews + SUMMARY) — ~1 session
6. INNOVATE (whatever the anchor surfaces) — same session as PANEL
7. HANDOFF — same session

**Success criterion for the anchor:** the rubric and personas produce a review that reads as *true to T&E* — Knizia-AI recognizes his own triangle; Feld-AI flags the game as "points from everywhere done right"; an Interactionist lens picks up the catastrophes as a core driver. If the anchor's review reads as generic or misses T&E's well-known features, the rubric or personas need revision before Phase 2.

---

## 11. Skills Roadmap (YAGNI-disciplined)

**Phase 1 — ship the anchor.** Build only these:
1. `/tigris-tierA` — produces `tierA-sim.md` matrix
2. `/tigris-panel` — produces the panel/ directory and SUMMARY
3. `/tigris-innovate` — appends to innovations logs + cluster detection
4. `/tigris-handoff` — end-of-cycle capture

Tier B for the anchor can be authored as a *manual* narrated playthrough this round (one long `Write` per playtest file). A proper `/tigris-tierB` skill becomes Phase 2 once we know what we actually need.

**Phase 2 — open the design path** (triggered when user commissions first original game):
5. `/tigris-concept`
6. `/tigris-design`
7. `/tigris-tierB` (narrated playthrough, seeded RNG)

**Phase 3 — factory-scale ergonomics** (deferred until 3+ games exist):
8. `/tigris-tierC` (tournament)
9. `/tigris-resume` (crash-safe stage pickup)
10. `/tigris-status` (pipeline dashboard)
11. `/tigris-clean` (forbidden-words audit)
12. `/tigris-publish` (print-and-play PDF, only if user demands)

Each skill deferred until used twice manually first (YAGNI).

---

## 12. Bootstrapping Checklist

Order of operations to stand TIGRIS up from empty directory:

1. `git init`, connect to `https://github.com/giodl73-repo/TIGRIS.git`, first commit = this spec
2. Write `CLAUDE.md` (house rules, frontmatter contract, forbidden words, pipeline map)
3. Write `README.md` (what TIGRIS is, how to contribute, pipeline diagram)
4. Write `TRACKER.md` (empty table, ready for row 1)
5. Write `personas/playtest-rubric.md` v1.0 (the 8-axis rubric from §6)
6. Write `personas/playtest-innovations.md` (empty append-only log)
7. Write `personas/forbidden-words.md`
8. Write the 7 designer personas + 5 player lenses
9. Author `/tigris-tierA`, `/tigris-panel`, `/tigris-innovate`, `/tigris-handoff` skills (Phase 1)
10. Scaffold `games/0001-tigris-and-euphrates/`
11. Run the anchor end-to-end
12. Retrospective: which rubric axes moved? which personas overlapped? amend before Phase 2

---

## 13. Open Questions (to resolve during or after anchor)

- **Copyright posture for reviews**: summary + excerpt + citation under fair use vs. commission licensed rules excerpts. Assume fair use for in-house calibration; revisit if we publish externally.
- **T&E rule source**: which edition — Hans im Glück 1997, FFG 2015 reprint, or the 2019 25th anniversary? Pick one canonical source in `reference/rules/tigris-and-euphrates.md`.
- **Multi-author callsigns**: port puzzlehunt's NATO phonetic roster? Only needed if TIGRIS gets multiple contributors. Defer.
- **Print-and-play rendering**: defer per §11 Phase 3. Revisit after original game #3.
- **Rubric axis count**: 8 is the starting guess. Anchor will tell us if we need a 9th (e.g., "Catch-up mechanism" as a separate axis from Variance) or can fold one down.

---

## 14. Non-Goals

- No digital game implementation (rules-executable sim) in any phase. Tier B narrative is the fidelity ceiling.
- No image/asset generation. Components described textually.
- No multi-player real human playtest coordination — TIGRIS simulates; humans audit.
- No BGG scraping or live community feedback ingestion. Reference material is curated into `reference/` manually.
- No cross-tradition coverage (wargames, party games, ameritrash) in Phase 1. Euro-only until the Euro pipeline is proven.

---

## 15. Success Criteria

TIGRIS v1.0 is "working" when:

1. The Tigris & Euphrates anchor review exists end-to-end: all 7 stages, artifacts match the contracts in §4 and §9.
2. The anchor's PANEL/SUMMARY.md reads as *true to T&E* — a knowledgeable reader recognizes the game from the review alone.
3. The innovation log has ≥ 3 entries, ≥ 1 of which clusters into a proposed rubric amendment.
4. A second game (review or original) can be started using only the existing skills and personas — no fresh scaffolding needed.
5. Rubric v1.0 has at least one amendment proposed (v1.1 candidate) off the anchor, proving the evolution mechanism works.
