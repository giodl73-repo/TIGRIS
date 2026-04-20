# TIGRIS Ideate-Hopper Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the TIGRIS Ideate-Hopper v1.1 — a 5-pool + 1-skill idea-generation system that produces candidate original-design ideas and feeds them into the `/tigris-concept` pipeline.

**Architecture:** Directory `ideas/` holds 5 hand-curated markdown pool files (primitives, mashups, gaps, frustrations) + a generated queue (hopper.md). Skill `/tigris-ideate` reads the pools and writes only to `hopper.md`. Skill `/tigris-concept` is extended to accept `HOP-NNN` as an argument, which seeds a game concept from a promoted hopper entry and marks the entry consumed.

**Tech Stack:** Markdown files + Claude Code skills (SKILL.md format). No code, no tests in the pytest sense — validation is schema compliance and live-run behavior.

**Spec source:** `docs/specs/2026-04-19-tigris-ideate-hopper-design.md` v1.1.

**Parliament reviews:**
- Spec: `docs/specs/reviews/2026-04-19-ideate-hopper/review.md` (3 amendments applied in spec v1.1)
- Plan: `docs/specs/reviews/2026-04-19-ideate-hopper/plan-review.md` (4 amendments applied in plan v1.1)

**Plan version:** 1.1 (2026-04-20).

## Skill discovery assumption (per A-plan-1.1-01)

Claude Code auto-discovers skills placed in `.claude/skills/<skill-name>/SKILL.md` (project-local) or `~/.claude/skills/<skill-name>/SKILL.md` (user-level). Skills at a non-`.claude/` path (e.g., repo-root `skills/`) are **not** discovered. No registration step is required for correctly-placed skills. After Task 7 commits `.claude/skills/tigris-ideate/SKILL.md`, invoke with `/tigris-ideate` and Claude Code will recognize the skill on the next session. **If the skill isn't discovered in the same session where it was written, restart the session before running Task 9.**

---

## File Structure

**Create:**
- `ideas/README.md` — usage overview
- `ideas/primitives.md` — 15 bootstrap entries (meets A-spec-1.1-02 minimum)
- `ideas/mashups.md` — 5 bootstrap entries (meets minimum)
- `ideas/gaps.md` — populated from current TIGRIS state (axis-pool.md + review record + BGG Top 100 scan)
- `ideas/frustrations.md` — empty schema header
- `ideas/hopper.md` — empty schema header
- `.claude/skills/tigris-ideate/SKILL.md` — the `/tigris-ideate` skill definition

**Modify:**
- `.claude/skills/tigris-concept/SKILL.md` — add `HOP-NNN` argument handling (Step 0 pre-scan and post-creation hopper mark-consumed)

**Rationale:** 5-pool separation matches 5 source types per spec §Architecture. Each file has one clear responsibility. Pools are read-only from the skill's perspective; `hopper.md` is the only write target (§Write discipline).

---

## Task 1: Scaffold `ideas/` directory and README

**Files:**
- Create: `ideas/README.md`

- [ ] **Step 1: Create `ideas/` directory**

Run: `mkdir -p ideas`
Expected: directory exists; `ls ideas` returns empty.

- [ ] **Step 2: Write `ideas/README.md`**

```markdown
---
name: TIGRIS Ideas Directory
slug: ideas-readme
version: 1.1.0
rubric_version: v2.15
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# ideas/ — TIGRIS Ideate-Hopper

Source-of-ideas system for TIGRIS original designs. Per spec `docs/specs/2026-04-19-tigris-ideate-hopper-design.md` v1.1.

## Files

| File | Kind | Maintained by | Min size for skill |
|---|---|---|---:|
| `primitives.md` | hand-curated | human | 10 entries |
| `mashups.md` | hand-curated | human | 5 entries |
| `gaps.md` | hand-curated | human (periodic refresh) | 1 section |
| `frustrations.md` | hand-curated | human | 0 (may be empty) |
| `hopper.md` | generated + promoted | `/tigris-ideate` + human | n/a |

At least one pool must meet its minimum for `/tigris-ideate` to run.

## Usage

```
/tigris-ideate                          # 3 candidates, all sources
/tigris-ideate --count 5                # 5 candidates
/tigris-ideate --sources mashup,gap     # only these sources
/tigris-ideate --format full            # concept.md-shaped stub
/tigris-ideate --theme nature           # constrain to a tag
```

Candidates land in `hopper.md` with status `fresh`. Promote by editing status to `promoted`. Consume with `/tigris-concept HOP-NNN` — the concept skill seeds `games/NNNN-<slug>/concept.md` and marks the entry `consumed`.

## Lifecycle

`fresh → promoted | retired` (manual)
`promoted → consumed` (automatic via `/tigris-concept HOP-NNN`)
`promoted → retired` (manual)
`consumed` (terminal)

## Write discipline

`/tigris-ideate` writes ONLY to `hopper.md`. Pools are stable knowledge base. `/tigris-concept HOP-NNN` writes to `games/NNNN-<slug>/` and marks the consumed entry in `hopper.md` only.

## See also

- Spec: `docs/specs/2026-04-19-tigris-ideate-hopper-design.md`
- Parliament review: `docs/specs/reviews/2026-04-19-ideate-hopper/review.md`
```

- [ ] **Step 3: Commit**

```bash
git add ideas/README.md
git commit -m "ideate-hopper: scaffold ideas/ with README"
```

---

## Task 2: Seed `ideas/primitives.md` with 15 bootstrap entries

**Files:**
- Create: `ideas/primitives.md`

Per spec §Pool schemas — primitives.md. 15 entries = 5 themes + 7 mechanisms + 3 table-dynamics. Meets A-spec-1.1-02 minimum (≥ 10).

- [ ] **Step 1: Write `ideas/primitives.md`**

```markdown
---
name: TIGRIS Primitives Catalog
slug: ideas-primitives
version: 1.0.0
rubric_version: v2.15
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# Primitives Catalog

Atomic design primitives for `/tigris-ideate`. Hand-curated. Skill reads; never writes.

## Themes

### P-001 — Nature / biology
- **Kind:** theme
- **Tag:** nature
- **One-liner:** Ecosystems, species, habitats, conservation — biological subject matter as theme.
- **Examples:** Wingspan, Ark Nova, Earth, Wyrmspan
- **Notes:** TIGRIS has 1 (Wingspan); corpus underweight.

### P-002 — Cosmic / space
- **Kind:** theme
- **Tag:** space
- **One-liner:** Galactic scale, planets, stars, void; from hard-SF to mythic cosmology.
- **Examples:** Terraforming Mars, Eclipse, Twilight Imperium, Race for the Galaxy
- **Notes:** Overrepresented in BGG Top 100.

### P-003 — Farming / pastoral
- **Kind:** theme
- **Tag:** farming
- **One-liner:** Land, animals, crops, seasons; Rosenberg-territory.
- **Examples:** Agricola, Caverna, Viticulture, Fields of Arle
- **Notes:** Well-represented in TIGRIS (Agricola) and BGG.

### P-004 — Horror / cosmic dread
- **Kind:** theme
- **Tag:** horror
- **One-liner:** Unknown threats, sanity cost, survival-under-pressure.
- **Examples:** Arkham Horror, Mansions of Madness, Nemesis, Eldritch Horror
- **Notes:** TIGRIS has 0; coverage gap.

### P-005 — Domestic / mundane
- **Kind:** theme
- **Tag:** domestic
- **One-liner:** Household, kitchen, suburb, daily-life; anti-epic scale.
- **Examples:** Herd Mentality, Cat Lady, Tokaido (pastoral-adjacent)
- **Notes:** Rare in Euro corpus.

## Mechanisms

### P-006 — Deck-building
- **Kind:** mechanism
- **Tag:** deck-building
- **One-liner:** Purchase cards into a personal deck; cycle via reshuffle.
- **Examples:** Dominion, A Few Acres of Snow, Trains, Clank!
- **Notes:** Vaccarino-signature.

### P-007 — Worker placement
- **Kind:** mechanism
- **Tag:** worker-placement
- **One-liner:** Commit limited pawns to board spaces; spaces exclusive or degrading.
- **Examples:** Agricola, Caylus, Lords of Waterdeep, Viticulture
- **Notes:** Rosenberg + K-K cluster.

### P-008 — Tile laying
- **Kind:** mechanism
- **Tag:** tile-laying
- **One-liner:** Place polyomino-or-square tiles; spatial optimization.
- **Examples:** Carcassonne, Azul, Patchwork, Castles of Burgundy
- **Notes:** Feld-adjacent.

### P-009 — Rondel
- **Kind:** mechanism
- **Tag:** rondel
- **One-liner:** Circular action-selection track; stepping forward costs something.
- **Examples:** Concordia, Great Western Trail, Teotihuacan, Trajan
- **Notes:** Gerdts-signature.

### P-010 — Roll-and-write
- **Kind:** mechanism
- **Tag:** roll-and-write
- **One-liner:** Public dice → private scoresheet; parallel play.
- **Examples:** Welcome To…, Ganz Schön Clever, Qwixx, Railroad Ink
- **Notes:** TIGRIS has 0.

### P-011 — Area control / influence
- **Kind:** mechanism
- **Tag:** area-control
- **One-liner:** Compete for majority or dominance over map regions.
- **Examples:** El Grande, Blood Rage, Twilight Struggle, Scythe
- **Notes:** K-K-adjacent.

### P-012 — Trick-taking
- **Kind:** mechanism
- **Tag:** trick-taking
- **One-liner:** Lead card; follow suit; highest trump wins.
- **Examples:** The Crew, Sheepshead, Skull King, Haggis
- **Notes:** TIGRIS has 0.

## Table-dynamics

### P-013 — Simultaneous action selection
- **Kind:** table-dynamic
- **Tag:** simultaneous-action
- **One-liner:** All players reveal choices at once; eliminates AP.
- **Examples:** 7 Wonders, Race for the Galaxy, Blood Rage
- **Notes:** Anti-downtime.

### P-014 — Lock-in / point-of-no-return
- **Kind:** table-dynamic
- **Tag:** lock-in
- **One-liner:** Decisions become irreversible at a defined moment.
- **Examples:** Brass (canals → rails), Twilight Struggle (DEFCON)
- **Notes:** D4 anchor.

### P-015 — Traitor / hidden role
- **Kind:** table-dynamic
- **Tag:** hidden-role
- **One-liner:** One or more players are secretly opposed.
- **Examples:** Battlestar Galactica, Shadows over Camelot, Dead of Winter
- **Notes:** TIGRIS has 0.
```

- [ ] **Step 2: Verify file has ≥ 10 entries**

Run: `grep -c "^### P-" ideas/primitives.md`
Expected: `15`

- [ ] **Step 3: Commit**

```bash
git add ideas/primitives.md
git commit -m "ideate-hopper: seed primitives.md with 15 bootstrap entries"
```

---

## Task 3: Seed `ideas/mashups.md` with 5 bootstrap entries

**Files:**
- Create: `ideas/mashups.md`

Meets A-spec-1.1-02 minimum (≥ 5).

- [ ] **Step 1: Write `ideas/mashups.md`**

```markdown
---
name: TIGRIS Mashups Catalog
slug: ideas-mashups
version: 1.0.0
rubric_version: v2.15
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# Mashups Catalog

Pre-seeded theme × mechanism (or dynamic × theme) pairs. Hand-curated. Distinct from the Primitives source, which creates fresh pairings on the fly.

## Schema

```
### M-<NNN> — <pair slug>
- **A:** <primitive tag or freeform>
- **B:** <primitive tag or freeform>
- **Tension hypothesis:** <what argument makes this work or fail?>
- **Source:** curator | auto-generated | external
```

## Entries

### M-001 — Deck-building × conservation
- **A:** deck-building (P-006)
- **B:** nature/conservation (P-001)
- **Tension hypothesis:** Can each card represent a species you're *not* supposed to use? Removing cards from your deck could mean "rewilding" — unwinding the engine is the engine. Subverts the build-bigger instinct.
- **Source:** curator

### M-002 — Roll-and-write × horror
- **A:** roll-and-write (P-010)
- **B:** horror (P-004)
- **Tension hypothesis:** Public shared dice encode a collective fate; private sheets are personal survival. When the monster rolls, everyone flinches. Horror via statistics, not miniatures.
- **Source:** curator

### M-003 — Rondel × cosmic
- **A:** rondel (P-009)
- **B:** cosmic (P-002)
- **Tension hypothesis:** Orbital mechanics as a rondel — each step forward costs fuel; planets align periodically. The rondel *is* the solar system.
- **Source:** curator

### M-004 — Lock-in × domestic
- **A:** lock-in (P-014)
- **B:** domestic (P-005)
- **Tension hypothesis:** Mundane choices (buy the house, pick the school) become Brass-level irreversible. Life-decisions as a designer game; scale down the epic, keep the weight.
- **Source:** curator

### M-005 — Trick-taking × farming
- **A:** trick-taking (P-012)
- **B:** farming (P-003)
- **Tension hypothesis:** Harvest rounds as trick-taking; crops as suits; weather as trump. The follow-suit obligation becomes seasonal planting. Inverts Rosenberg's action-selection into Skull-King pacing.
- **Source:** curator
```

- [ ] **Step 2: Verify file has ≥ 5 entries**

Run: `grep -c "^### M-" ideas/mashups.md`
Expected: `5`

- [ ] **Step 3: Commit**

```bash
git add ideas/mashups.md
git commit -m "ideate-hopper: seed mashups.md with 5 bootstrap entries"
```

---

## Task 4: Derive and write `ideas/gaps.md` from TIGRIS state

**Files:**
- Create: `ideas/gaps.md`

Per spec §Initial bootstrap step 4: derive gaps during bootstrap from axis-pool, review record, BGG scan.

- [ ] **Step 1: Confirm current TIGRIS state**

Read: `personas/axis-pool.md` (for adopted/contested axes)
Read: `TRACKER.md` Games section (for anchor history across 0001..0017)

- [ ] **Step 2: Write `ideas/gaps.md`**

```markdown
---
name: TIGRIS Gaps — live state
slug: ideas-gaps
version: 1.0.0
rubric_version: v2.15
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# Gaps — Live TIGRIS State

Live snapshot derived from `personas/axis-pool.md`, `TRACKER.md`, and BGG Top 100 scan. Manually refreshed.

**Last refresh:** 2026-04-20 (after Wingspan game #17).

## Axes underexplored

Adopted axes with single-persona advocacy (no cross-persona canonical):

- **A4 Variance Calibration** — Vaccarino sole advocate. Earned CoB + Agricola. Would benefit from a canonical case in a non-Vaccarino-adjacent game.
- **D4 Late-Game Lock-in Point** — Rosenberg + Lacerda earnings; few others have engaged canonically.
- **B6 Scoring Multiplier Dependency** — Rosenberg-primary; only 2 canonical cases (UNFOLD + TtA).

## Personas without recent anchor

Looking at games 0013..0017 (last 5):

- **Knizia** — last own-game anchor at Parliament (#1) and T&E (#2). Non-own anchor TS (#15 on C1). Ready for another own-or-non-own anchor.
- **Feld** — last anchor CoB (#12). Open to new anchor.
- **Lacerda** — last anchor Lisboa (#11). Open.
- **Chvátil** — last anchor TtA (#8). Overdue.
- **Rosenberg** — last anchor Agricola (#14). Recent.
- **K-K** — last anchor Tikal (#13). Recent.
- **Stegmaier** — last anchor Scythe (#5). Overdue (many non-anchor defends since).
- **Vaccarino** — anchor at Famiglia (#9), AFoS (#16), Wingspan (#17). Saturated recently.

**Most overdue for own-anchor: Stegmaier, Chvátil.**

## BGG Top 100 domains missing from TIGRIS corpus

Per BGG Top 100 scan (commit 74571ab-adjacent analysis):

- **Horror** — Nemesis, Eldritch Horror, Arkham LCG, Mansions of Madness, Kingdom Death. Count in corpus: 0.
- **Dexterity / action** — Crokinole. Count: 0.
- **Party / word** — Codenames, Decrypto. Count: 0.
- **Legacy** — Pandemic Legacy S1/S2, Clank! Legacy. Count: 0.
- **Hidden role** — Battlestar Galactica, Secret Hitler. Count: 0.
- **Abstract strategy** — Azul, Patchwork, Go, Hive. Count: 0.

## Mechanisms never in TIGRIS reviews

- **Roll-and-write** (Welcome To…, Ganz Schön Clever, Railroad Ink).
- **Trick-taking** (The Crew, Haggis).
- **Flicking / dexterity** (Crokinole, Flick 'em Up).
- **Pick-up-and-deliver** (Pandemic-adjacent, Great Western Trail has this).
- **Bag-building** (Quacks of Quedlinburg, Orléans).
- **Legacy / campaign** (Pandemic Legacy, Gloomhaven campaign structure).

## Adopted-contested-watch

Per v2.14 A-v2.14-01 amendment, axes at risk of adopted-contested status:

- **A3 Interaction** — 2E/1R (Wingspan multiplayer-solitaire self-refute by Chvátil). Next refute game → adopted-contested formal.
- **A7 Emergence-Replayability** — 2E/1R (AFoS Halifax Hammer refute by Lacerda). Next refute game → adopted-contested formal. Wingspan counter-defense earned but refute unchanged.

**Design opportunity**: a game that structurally *guarantees* interaction or emergence could be a counter-pressure review target.
```

- [ ] **Step 3: Verify gaps.md has all 5 sections**

Run: `grep -c "^## " ideas/gaps.md`
Expected: `5` (Axes underexplored / Personas without recent anchor / BGG domains missing / Mechanisms never / Adopted-contested-watch)

- [ ] **Step 4: Commit**

```bash
git add ideas/gaps.md
git commit -m "ideate-hopper: derive gaps.md from current TIGRIS state (post-Wingspan)"
```

---

## Task 5: Initialize `ideas/frustrations.md` with empty schema

**Files:**
- Create: `ideas/frustrations.md`

Per A-spec-1.1-02: may be empty. Skill treats empty as dropped source.

- [ ] **Step 1: Write `ideas/frustrations.md`**

```markdown
---
name: TIGRIS Frustrations Catalog
slug: ideas-frustrations
version: 1.0.0
rubric_version: v2.15
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# Frustrations Catalog

External-source catalog: designer interviews, BGG threads, Kickstarter post-mortems, "I wish there was a game that…" observations. Hand-curated.

**Status:** empty — to be populated. `/tigris-ideate` treats empty as dropped source (per spec §Minimum-viable pool sizes).

**Per Parliament A-spec-1.1-05 observational:** if this pool sits unchanged for 10+ games, revisit — consider dropping the source OR adding auto-scraping.

## Schema

```
### F-<NNN> — <problem slug>
- **Problem:** <what's missing, broken, or unexplored>
- **Source:** <citation or URL>
- **Date seen:** YYYY-MM-DD
- **Notes:** <why interesting>
```

## Entries

<!-- None yet. Add entries below as they're encountered. -->
```

- [ ] **Step 2: Commit**

```bash
git add ideas/frustrations.md
git commit -m "ideate-hopper: initialize frustrations.md with empty schema"
```

---

## Task 6: Initialize `ideas/hopper.md` with empty schema

**Files:**
- Create: `ideas/hopper.md`

- [ ] **Step 1: Write `ideas/hopper.md`**

```markdown
---
name: TIGRIS Hopper — generated candidates
slug: ideas-hopper
version: 1.0.0
rubric_version: v2.15
bet_version: parliament
author: TIGRIS (generated by /tigris-ideate)
created: 2026-04-20
updated: 2026-04-20
---

# Hopper — Generated Candidates Queue

`/tigris-ideate` appends here. User promotes/retires manually. `/tigris-concept HOP-NNN` consumes promoted entries.

## Counter

Last assigned ID: HOP-000 (no entries yet).

## Schema

```
### HOP-<NNN> — <working title>
- **One-liner:** <2-3 sentence pitch>
- **Anchor guess:** <persona>/<axis>
- **Sources:** <which source entries this candidate combined, with IDs>
- **Tension hypothesis:** <what argument would the panel have?>
- **Status:** fresh | promoted | consumed | retired
- **Created:** YYYY-MM-DD
- **Updated:** YYYY-MM-DD
- **Notes:** <user notes; backlink to games/NNNN-<slug>/ if consumed>
```

## Entries

<!-- /tigris-ideate appends below this line. -->
```

- [ ] **Step 2: Commit**

```bash
git add ideas/hopper.md
git commit -m "ideate-hopper: initialize hopper.md with empty schema + counter"
```

---

## Task 7: Write `.claude/skills/tigris-ideate/SKILL.md`

**Files:**
- Create: `.claude/skills/tigris-ideate/SKILL.md`

- [ ] **Step 1: Create skill directory**

Run: `mkdir -p .claude/skills/tigris-ideate`

- [ ] **Step 2: Write `.claude/skills/tigris-ideate/SKILL.md`**

```markdown
---
name: tigris-ideate
description: Generate candidate TIGRIS original-design ideas by sampling from the 5-pool ideas/ directory. Appends fresh HOP-NNN entries to ideas/hopper.md. Never modifies pools. Use when seeking seeds for a new original design; pair with /tigris-concept HOP-NNN to promote an idea to a game.
---

# tigris-ideate — generate original-design candidates

## 60-second quickstart

```
You:    "/tigris-ideate"
Claude: [reads ideas/ pools + personas/axis-pool.md + innovations log]
        [samples 3 sources; generates 3 candidates]
        [appends to ideas/hopper.md with fresh status and new HOP-NNN IDs]
        [reports: "Appended HOP-001, HOP-002, HOP-003 to ideas/hopper.md.
         HOP-001 — Rewilding (deck-un-building × nature). Anchor: Vaccarino / C4."]
```

## When to invoke

- User wants new candidate ideas for an original TIGRIS game.
- Before `/tigris-concept` — a hopper entry may seed the concept (`/tigris-concept HOP-NNN`).
- Do NOT use for game reviews; those skip ideation and start at DESIGN with published rules imported.

## Preconditions

- `ideas/primitives.md` with ≥ 10 entries, OR
- `ideas/mashups.md` with ≥ 5 entries, OR
- `ideas/gaps.md` with ≥ 1 populated section

If NO pool meets its minimum, error with: "Ideate-hopper pools below minimum. See ideas/README.md for bootstrap steps." Do not invent candidates from nothing.

## Invocation flags

| Flag | Default | Meaning |
|---|---|---|
| `--count N` | 3 | Number of candidates to generate |
| `--sources <list>` | `mashup,gap,mining,frustration,primitives` | Comma-separated source subset |
| `--format brief\|full` | `brief` | `brief` = 3-sentence pitch; `full` = concept.md-shaped stub |
| `--theme <tag>` | unset | Constrain to primitives with this tag (e.g., `--theme nature`) |

## Procedure

### Step 1 — Read pool files + ledger state

- `ideas/primitives.md`
- `ideas/mashups.md`
- `ideas/gaps.md`
- `ideas/frustrations.md`
- `personas/axis-pool.md` (for axis status, contested-watch)
- `personas/playtest-innovations.md` (for mining observational/candidate entries)
- `ideas/hopper.md` (for dedup — check last 20 entries)

### Step 2 — Check minimum-viable pool sizes

- primitives.md: ≥ 10 entries
- mashups.md: ≥ 5 entries
- gaps.md: ≥ 1 populated section
- frustrations.md: 0 (may be empty; treated as dropped source)

If any source below its minimum, drop it from the active source list for this run. If all below minimum, error per Preconditions.

### Step 3 — Generate each candidate

For each of N candidates:

1. Select 1-2 sources from active `--sources` list. Selection is LLM-judgment (read pools and pick based on session context; not pseudorandom; repeat runs should produce different candidates).
2. Sample entries from selected source(s):
   - **Mashup**: pick a pre-seeded pair from `mashups.md`.
   - **Gap**: pick a gap-entry from `gaps.md` (any section).
   - **Mining**: pick an observational/candidate entry from `personas/playtest-innovations.md` and convert to a design prompt.
   - **Frustration**: pick a `F-NNN` entry from `frustrations.md`.
   - **Primitives**: sample 2-3 `P-NNN` entries across kinds; invent a new pair on the fly.
3. Generate: working title + pitch (brief or full per `--format`) + anchor-persona guess + anchor-axis guess + tension hypothesis.
4. Dedup: compare new candidate against hopper.md last 20 entries. If match on title OR same anchor+sources, regenerate. Max 3 regenerations — on 4th attempt, emit with `near-duplicate-warning` note in the hopper entry.

### Step 4 — Append to hopper.md

For each candidate, increment the hopper counter in the frontmatter and write a new entry:

```markdown
### HOP-<NNN> — <working title>
- **One-liner:** <pitch>
- **Anchor guess:** <persona>/<axis>
- **Sources:** <source-type (source-entry-IDs)>; <source-type (source-entry-IDs)>
- **Tension hypothesis:** <panel-argument prediction>
- **Status:** fresh
- **Created:** YYYY-MM-DD
- **Updated:** YYYY-MM-DD
- **Notes:** <empty, or "near-duplicate-warning: <reason>">
```

Update the hopper counter (`Last assigned ID: HOP-<NNN>`).

### Step 5 — Report

Report to user:
- Count of candidates appended
- List of HOP-NNN IDs with one-line titles
- Any warnings (pools skipped for being below minimum; near-duplicate warnings)
- Next step suggestion: "Promote a candidate by editing its status to `promoted`, then run `/tigris-concept HOP-NNN`."

## Invariants

- Writes ONLY to `ideas/hopper.md`. Never modifies pools or other ledger files.
- Does not invent candidates when pools are empty. Errors out with bootstrap instruction.
- Each candidate must name a plausible anchor-persona + anchor-axis from existing TIGRIS roster. Do not invent new personas or axes.
- Hopper counter is monotonic. Last assigned ID in frontmatter always reflects current max.

## Failure modes

- **Empty pool**: source skipped; run continues with remaining sources.
- **All pools empty or below minimum**: skill errors with bootstrap instruction. Does not generate candidates.
- **Near-duplicate candidate**: regeneration loop; up to 3 attempts; 4th attempt emits with warning.
- **Stale `gaps.md`**: not detectable by skill. User responsibility to refresh. Known limitation.

## What this skill does NOT do

- Run the full concept generation (that's `/tigris-concept`).
- Modify pool files (read-only access to primitives/mashups/gaps/frustrations).
- Rate or score candidates (no quality signal — user judgment is the filter).
- Auto-refresh gaps.md (deferred to v1.2 `--refresh-gaps`).
- Scrape external sources (deferred).

## See also

- Spec: `docs/specs/2026-04-19-tigris-ideate-hopper-design.md`
- Parliament review: `docs/specs/reviews/2026-04-19-ideate-hopper/review.md`
- Companion skill: `.claude/skills/tigris-concept/SKILL.md` (accepts `HOP-NNN`)
```

- [ ] **Step 3: Commit**

```bash
git add .claude/skills/tigris-ideate/SKILL.md
git commit -m "ideate-hopper: write /tigris-ideate skill v1.1"
```

---

## Task 8: Modify `.claude/skills/tigris-concept/SKILL.md` to accept `HOP-NNN`

**Files:**
- Modify: `.claude/skills/tigris-concept/SKILL.md` (add Step 0 before Step 1; add post-creation hopper-mark)

- [ ] **Step 0: Read the current skill file (per A-plan-1.1-02)**

Run: Read `.claude/skills/tigris-concept/SKILL.md` end-to-end.

Expected: current content matches the blocks referenced in Steps 2 and 3 below. If the file has drifted from the plan's expected blocks (e.g., headings renamed, step numbers shifted), adjust the `old_string` arguments in Steps 2-3 accordingly before invoking Edit. Edit tool requires the file to be Read first — skipping this step will fail.

- [ ] **Step 1: Plan the insertion points**

Insert a new "Step 0 — Optional hopper seed" before existing Step 1, and a new "Step 6 — Mark hopper entry consumed (if seeded from HOP-NNN)" after existing Step 5.

- [ ] **Step 2: Edit `.claude/skills/tigris-concept/SKILL.md` — add Step 0**

Find this block (starts around line 34):

```markdown
## Procedure

### Step 1 — Determine game number
```

Replace with:

```markdown
## Procedure

### Step 0 — Optional hopper seed

If the skill is invoked with a `HOP-NNN` argument (e.g., `/tigris-concept HOP-007`):

1. Read `ideas/hopper.md`. Locate the entry with matching `HOP-NNN`.
2. If entry not found, error: "HOP-NNN not found in ideas/hopper.md."
3. If entry status is not `promoted`, error: "HOP-NNN status is `<status>`, must be `promoted` before consuming."
4. Extract from entry: `one-liner` → premise seed; `anchor guess` → anchor-persona + anchor-axis defaults; `tension hypothesis` → target-review-section seed.
5. Continue to Step 1 with these values pre-filled; user can override.

If no `HOP-NNN` argument, proceed to Step 1 with no seed.

### Step 1 — Determine game number
```

- [ ] **Step 3: Edit `.claude/skills/tigris-concept/SKILL.md` — add Step 6**

Find this block (near line 116):

```markdown
### Step 5 — Report

Report to user:
- Path to concept.md
- Anchor declaration (persona + axis)
- Any warnings (queued-for-retirement axis, missing adjacency partner)
- Next step: run `/tigris-design` to author rules

## Invariants
```

Replace with:

```markdown
### Step 5 — Report

Report to user:
- Path to concept.md
- Anchor declaration (persona + axis)
- Any warnings (queued-for-retirement axis, missing adjacency partner)
- Next step: run `/tigris-design` to author rules

### Step 6 — Mark hopper entry consumed (if seeded from HOP-NNN)

If Step 0 loaded a HOP-NNN entry:

1. Edit the hopper entry in `ideas/hopper.md`:
   - Change `Status: promoted` → `Status: consumed`
   - Update `Updated: <today's date>`
   - Append to `Notes`: `consumed by games/NNNN-<slug>/`

2. Commit hopper update together with the new concept.md in a single commit.

## Invariants
```

- [ ] **Step 4: Verify the edits**

Run: `grep -n "Step 0 — Optional hopper seed" .claude/skills/tigris-concept/SKILL.md`
Expected: line number around 37.

Run: `grep -n "Step 6 — Mark hopper entry consumed" .claude/skills/tigris-concept/SKILL.md`
Expected: line number around 125.

- [ ] **Step 5: Commit**

```bash
git add .claude/skills/tigris-concept/SKILL.md
git commit -m "tigris-concept: accept HOP-NNN argument for hopper-seeded concepts"
```

---

## Task 9: Live smoke-test — run `/tigris-ideate` for first time

**Files:**
- Modify: `ideas/hopper.md` (via skill execution)

**Note on variance (per A-plan-1.1-04):** `/tigris-ideate` output is LLM-judgment-based (spec v1.1 §`/tigris-ideate` Step 3). Different runs produce different candidates. "Expected output" in this task refers to *shape* — 3 entries, `fresh` status, unique HOP-NNN IDs, schema-compliant fields — not specific titles or content. If first run produces obviously-bad candidates (non-existent personas, axes not in Pool, non-sequitur pitches), re-run before committing.

- [ ] **Step 1: Run `/tigris-ideate` (default invocation)**

Invoke: `/tigris-ideate`
Expected (shape): 3 candidates appended to `ideas/hopper.md` with status `fresh`, unique HOP-NNN IDs (HOP-001 through HOP-003), and each naming an anchor-persona + anchor-axis.

- [ ] **Step 1b: Evaluate output quality (per A-plan-1.1-03)**

Check each generated HOP-NNN entry against these gates:

1. **Anchor-persona** is one of: knizia, rosenberg, feld, lacerda, chvatil, kramer-kiesling, stegmaier, vaccarino. Reject if not.
2. **Anchor-axis** exists in `personas/axis-pool.md` (A1-A7, B1-B6, C1-C4, C6-C8, D1-D4; NOT C5 which is retired). Reject if not.
3. **One-liner** is ≥ 2 sentences AND mentions concepts from the declared sources. Reject if generic or contradicts sources.
4. **Tension hypothesis** names a plausible panel argument (references adoption/refute dynamics, or specific persona preferences). Reject if vague.

If any candidate fails these checks:
- If nothing has been committed yet: manually delete the failing entry from `ideas/hopper.md` and re-run `/tigris-ideate --count 1` to replace.
- If the entry was already committed: leave it and add a retiring edit (change status to `retired` with note "failed quality gate on first run").

Do not proceed to Step 2 until all 3 entries pass the gates OR have been retired.

- [ ] **Step 2: Verify hopper.md has 3 fresh entries**

Run: `grep -c "^- \*\*Status:\*\* fresh" ideas/hopper.md`
Expected: `3`

Run: `grep -c "^### HOP-" ideas/hopper.md`
Expected: `3`

- [ ] **Step 3: Verify hopper counter updated**

Run: `grep "Last assigned ID:" ideas/hopper.md`
Expected: `Last assigned ID: HOP-003`

- [ ] **Step 4: Verify no pool files modified**

Run: `git status ideas/primitives.md ideas/mashups.md ideas/gaps.md ideas/frustrations.md`
Expected: no changes.

- [ ] **Step 5: Commit the hopper update**

```bash
git add ideas/hopper.md
git commit -m "ideate-hopper: first live run — 3 candidates generated"
```

---

## Task 10: Verify failure modes + minimum-viable contract

- [ ] **Step 1: Test empty-frustrations behavior**

`frustrations.md` is currently empty. Run: `/tigris-ideate --sources frustration`
Expected: skill errors with bootstrap instruction OR skips the source and errors because no other source selected. Either is correct per spec §Failure modes.

If skill skips and errors: record behavior in a brief note; fine as-is.
If skill proceeds anyway: **bug**. Return to Task 7 and fix Step 2 logic.

- [ ] **Step 2: Test below-minimum contract**

Temporarily rename `ideas/primitives.md` to `ideas/primitives.md.bak`. Run: `/tigris-ideate --sources primitives`
Expected: error "primitives below minimum, 0 < 10" OR equivalent.

Restore: `mv ideas/primitives.md.bak ideas/primitives.md`.

- [ ] **Step 3: Promote one hopper entry manually, test `/tigris-concept HOP-NNN`**

Edit `ideas/hopper.md` HOP-001 `Status: fresh` → `Status: promoted`.

Invoke: `/tigris-concept HOP-001`
Expected:
- New game directory `games/NNNN-<slug>/` created.
- `games/NNNN-<slug>/concept.md` seeded with HOP-001 values.
- `ideas/hopper.md` HOP-001 status updated to `consumed` with backlink.

- [ ] **Step 4: Verify single commit covers both files**

Run: `git log -1 --stat`
Expected: one commit touching both `ideas/hopper.md` AND `games/NNNN-<slug>/concept.md`.

- [ ] **Step 5: Commit any pending test observations**

If smoke tests revealed gaps, create a follow-up issue note at `docs/specs/reviews/2026-04-19-ideate-hopper/smoke-test-notes.md` and commit:

```bash
git add docs/specs/reviews/2026-04-19-ideate-hopper/smoke-test-notes.md
git commit -m "ideate-hopper: smoke-test observations from first live run"
```

Otherwise skip this step.

---

## Done criteria (recap of spec §Success criteria for v1.0 release)

- [ ] `/tigris-ideate` produces 3 coherent candidates on first run (Task 9).
- [ ] Each candidate names a plausible anchor persona + axis (Task 9 visual check).
- [ ] User can flag a `fresh` entry as `promoted` with a single text edit (Task 10 Step 3).
- [ ] `/tigris-concept HOP-NNN` converts a promoted entry into a game concept cleanly (Task 10 Step 3-4).
- [ ] Pool files stay stable; skill reads but does not write to them (Task 9 Step 4).

---

## Out of scope (v1.2+)

- `--seed <hash>` for reproducible sampling.
- `--refresh-gaps` to auto-derive gaps.md from ledger state.
- Auto-scraping of frustrations from external sources.
- Rating / scoring system on candidates.
- Auto-archive of retired hopper entries.
- Hopper-to-mashup feedback loop (consumed games feeding mashups.md).

## Plan changelog

- **v1.1** (2026-04-20) — Parliament plan-review applied. 4 amendments:
  - A-plan-1.1-01: skill discovery assumption declared (header + session-restart note).
  - A-plan-1.1-02: Task 8 Step 0 added (Read-before-Edit).
  - A-plan-1.1-03: Task 9 Step 1b added (output-quality gate).
  - A-plan-1.1-04: Task 9 header LLM-variance note added.
- **v1.0** (2026-04-20) — initial plan written from spec v1.1.
