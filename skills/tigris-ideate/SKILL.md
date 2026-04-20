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
- Companion skill: `skills/tigris-concept/SKILL.md` (accepts `HOP-NNN`)
