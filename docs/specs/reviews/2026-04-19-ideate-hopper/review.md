---
name: Ideate-Hopper Spec — Parliament Review
slug: ideate-hopper-review
stage: panel
version: 1.0.0
rubric_version: v2.15
bet_version: parliament
artifact_under_review: docs/specs/2026-04-19-tigris-ideate-hopper-design.md
personas_seated: [knizia, rosenberg, feld, lacerda, chvatil, kramer-kiesling, stegmaier, vaccarino]
review_type: meta-review (TIGRIS-internal spec)
anchor_persona: lacerda
anchor_axis: B1
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# Ideate-Hopper Spec — Parliament Review

Meta-review of `docs/specs/2026-04-19-tigris-ideate-hopper-design.md` under Parliament procedure. 8 designers, no player lenses (processes don't get played). Axis-Pool adapted to spec context: "game behavior" reads as "skill behavior"; "playability" reads as "usability"; axes not applicable to processes retire-explicit cleanly.

## Anchor

**Lacerda** anchors **B1 System Gearing**. Anchor claim: "The 5 pools must gear into the skill and back to `/tigris-concept`. If any pool is dead weight or the skill writes to pools it also reads, gearing breaks. Canonical engineering concern for a skill spec."

Adjacency partner: **B4 Information-Transparency-Cost** (Lacerda-self) + **A6 Teachability** (Stegmaier cross-player). GATE passes.

## Drafts

### Knizia
1. **A1 Elegance.** 6/10. "5 pools for 5 sources is symmetrical but verbose. Elegance would collapse mashups into primitives. Hold-explicit leaning refute — spec is clear but not minimal."
2. **C2 Minimum-Score Shape.** 3/10. "Max-features goal, not min-structure. Retire-explicit — process spec doesn't have score shape."
3. **C1 Tension Budget.** 2/10. "No DEFCON-equivalent; spec is cooperative with its user. Retire-explicit."

### Rosenberg
1. **B3 Conversion Chain Depth.** 7/10. "Clean chain: primitives/mashups/gaps/frustrations → skill → hopper → `/tigris-concept` → games/. 4-stage conversion from raw sources to committed game. Canonical for a pipeline spec."
2. **C3 Scarcity Bite.** 2/10. "Pools are abundant; nothing bites. Retire-explicit."
3. **D4 Late-Game Lock-in.** 3/10. "Consumed is terminal; hopper archive mentioned but YAGNI'd. Weak lock-in. Hold-explicit."

### Feld
1. **A2 Decision Density.** 5/10. "User decisions per run: --count, --sources, --format, --theme, then promote/retire per entry. Moderate density for a skill. Earn-leaning."
2. **B2 Catastrophe Pressure.** 4/10. "**Failure modes not enumerated.** What if skill generates 3 near-identical candidates? What if all pools are stale? Spec silent on degenerate outputs. **Soft refute** on incompleteness."
3. **C6 Point-Salad Incommensurability.** 2/10. "No scoring dimensions on candidates. Retire-explicit (feature-gap or YAGNI, depends on reader)."

### Lacerda
1. **B1 System Gearing (anchor).** **8/10.** "5 pools gear into skill via read-only discipline; skill gears to `/tigris-concept` via HOP-NNN handoff; hopper gears back to games/ via consumed backlink. Gearing is explicit and one-directional. Canonical anchor."
2. **B4 Information-Transparency-Cost.** 7/10. "Spec declares write-discipline (skill writes ONLY to hopper). Information flow is traceable. Earned."
3. **A7 Emergence-Replayability.** 6/10. "Unbounded idea space via primitives × mashups × gaps × frustrations × mining combinations. Earn-leaning."

### Chvátil
1. **B5 Architectural Novelty.** **4/10.** "**Soft refute.** Hybrid pool-plus-skill is not novel — it's a standard idea-catalog-with-generator pattern (cf. any `/brainstorm` tool). TIGRIS's novelty is in the *content* (5 sources specific to TIGRIS), not the *structure*. Hold-explicit leaning refute."
2. **A3 Interaction.** 3/10. "Spec is cooperative; user and skill don't oppose each other. Retire-explicit."
3. **B6 Scoring Multiplier Dependency.** 4/10. "Primitives × mashups × gaps = combinatorial. Multiplier exists but is structural, not scoring. Hold-explicit."

### Kramer-Kiesling
1. **C7 Action-Menu Clarity.** 7/10. "4 flags (--count, --sources, --format, --theme), each icon-coded. Default behavior is 1-argument (`/tigris-ideate`). Clear. Earned."
2. **D1 Family-to-Expert Scaling.** **6/10.** "Novice: `/tigris-ideate`. Expert: flags. But no 'family mode' for curating pools — curation is expert-only. Earn-leaning hold."
3. **D2 Spatial-Interaction.** 1/10. "No spatial dimension. Retire-explicit."

### Stegmaier
1. **A6 Teachability.** **5/10.** "Spec readable in ~10 min. But bootstrap burden (~80 pool entries to write before skill is useful) is a teachability cliff. **Soft refute.** Spec requires off-document labor before skill works. Canonical A6 issue."
2. **D3 Count-Robustness.** 4/10. "Single-user design. No multi-user or team mode. Retire-explicit for solo-only scope."
3. **C8 First-Turn Compression.** 7/10. "Bootstrap step is compressed: the '9 numbered steps' in §Initial Bootstrap make first use well-mapped. Paradoxically compression is a positive here — opposite of Halifax Hammer. Earn."

### Vaccarino
1. **C4 Engine-Garden Dependency.** 7/10. "Hopper itself is an engine-garden of ideas; each run of ideate deposits more cards into the tableau. Canonical C4 on a meta level."
2. **A4 Variance Calibration.** **3/10.** "**Soft refute.** Spec doesn't specify variance source — is sampling truly uniform-random, or LLM-judgment-weighted? 'Uniform random' stated but how is randomness actually injected? Hidden mechanism. Hold-explicit leaning refute."
3. **A5 Downtime-Pacing.** 2/10. "No pacing concern in an async skill. Retire-explicit."

## Argument

### Round 1 — the gearing claim

- **Lacerda (B1) defends**: "Write-discipline section is explicit: skill reads pools + ledgers; skill writes only to hopper.md. No cycle. Pools are stable knowledge base. This is the gearing bet — read contracts one-way, write contracts narrow. +1 defend on B1." → B1 at 1 defend.
- **Rosenberg (B3) defends**: "Primitives → mashups → hopper → `/tigris-concept` → games/ is a 5-step conversion. Each stage transforms the raw material. +1 defend on B3." → B3 at 1 defend.

### Round 2 — the under-specified variance

- **Vaccarino (A4) attacks**: "Section §Behavior says 'uniform random across the chosen list' but doesn't specify the randomness source. Is this LLM-judgment? Seed-file? Hash of inputs? If unspecified, repeat runs are non-reproducible AND not uniform. **Refute.** -1 on A4."
- **Feld (B2) piles on**: "Combined with unenumerated failure modes — what's the behavior when pools are empty? Spec silent. Refute."
- **Knizia defends A4 partial**: "Uniform random in an LLM context is reasonable-default. 'Repeatable' is not a spec-level concern for a brainstorming tool."
- **Vote**: Knizia → partial (spec unclear but intent reasonable); Feld → full refute (unspecified behavior); Lacerda → partial refute (could be clarified in v1.1); Chvátil → full refute (spec should declare randomness source); Stegmaier → full refute (bootstrap-user will hit this); K-K → partial refute; Rosenberg → defense (intent is clear); Vaccarino (self) → full refute.
- **Result: 4-3-1 leaning refute. A4 partial refute on spec ambiguity. Amendment proposed (see below).**

### Round 3 — the teachability cliff

- **Stegmaier (A6) attacks**: "Section §Initial Bootstrap requires writing ~40-50 primitives + 20-30 mashups + 10-15 frustrations + auto-derive gaps = ~85-95 entries before the skill is useful. This is an off-document labor burden. Classic A6 teach-cliff. Refute."
- **Knizia (A1) agrees**: "5 separated pool files amplify this — if we had 3 pools, bootstrap would be smaller. Elegance refute aligns with teach refute."
- **Lacerda defends partial**: "Each pool could ship with a minimum-viable seed — e.g. 10 primitives + 5 mashups — and skill still works. Bootstrap-to-minimum != bootstrap-to-full."
- **Feld defends**: "Spec does say 'seeded' for each file — not that skill requires full seeding."
- **Vote**: Stegmaier (self) → refute; Knizia → refute; Chvátil → refute (echoes elegance); Lacerda → partial; Feld → defense; Rosenberg → defense; K-K → partial; Vaccarino → neutral.
- **Result: 3-2-3. A6 soft-refute.** Amendment: declare minimum-viable pool sizes for skill to function.

### Round 4 — the novelty collapse

- **Chvátil (B5) attacks**: "Pool-plus-generator is a standard brainstorming-tool pattern. TIGRIS's value is in the *content* — the 5 sources are TIGRIS-specific, but the *structure* is generic. B5 only earns if structural novelty is claimed. Spec does not claim it. Refute."
- **Vaccarino defends weakly**: "The integration with `/tigris-concept` HOP-NNN lifecycle is TIGRIS-specific and novel-adjacent."
- **Vote**: Chvátil (self) → refute; Lacerda → partial refute (agree, structure isn't novel); everyone else → hold (not load-bearing for v1.0 release).
- **Result: B5 hold-explicit with soft-refute undertone.** Not blocking.

### Round 5 — the frustrations question

- **Lacerda observation**: "Spec notes `frustrations.md` is user-curated. Hand-curated logs are the most-abandoned artifact type in any system. If user doesn't feed it, skill degrades to 4 sources."
- **Chvátil**: "Should consider auto-mining frustrations from Reddit/BGG/designer-diaries — out-of-scope for v1.0 but flag as v1.1 watch."
- **Vote**: Observational, no stake earns or refutes.

### Collision 1 — A1 Elegance vs B1 System Gearing

- **Knizia (A1)**: "5 pools for 5 sources is verbose; gearing could be cleaner with 3 pools."
- **Lacerda (B1)**: "5 matches the source-type count; collapsing loses legibility. Gearing is clearer with 5 than with 3 overloaded pools."
- **Vote**: Feld → B1 (legibility wins over compression); Rosenberg → B1; Stegmaier → A1 (bootstrap burden compounds); Chvátil → A1 (same concern); Vaccarino → B1; K-K → B1.
- **Result: 4-2 for Lacerda B1. Orthogonal-preservation possible — both valid, B1 wins on this spec.**

## Earned stakes (6)

| Persona | Axis | Status |
|---|---|---|
| **Lacerda (anchor)** | **B1 System Gearing** | Earned canonical (write-discipline explicit; one-way read contracts) |
| Rosenberg | B3 Conversion Chain | Earned (5-stage pipeline) |
| Lacerda | B4 Information-Transparency-Cost | Earned (write-discipline declared) |
| K-K | C7 Action-Menu Clarity | Earned (4 flags, default is minimal) |
| Stegmaier | C8 First-Turn Compression | Earned (bootstrap steps compressed — positive sense) |
| Vaccarino | C4 Engine-Garden Dependency | Earned (hopper as engine-garden of ideas) |

## Refuted stakes (3 soft-refutes on spec quality)

| Persona | Axis | Status |
|---|---|---|
| **Vaccarino** | **A4 Variance Calibration** | **Soft refute** — randomness source unspecified; spec silent on reproducibility |
| **Stegmaier** | **A6 Teachability** | **Soft refute** — ~85-95-entry bootstrap burden is an off-document labor cliff |
| **Feld** | **B2 Catastrophe Pressure** | **Soft refute** — failure modes (empty pools, near-duplicate candidates, stale gaps) not enumerated |

## Retire-explicits (10)

C2 C1 (Knizia), C3 (Rosenberg), C6 (Feld), A3 (Chvátil), D2 (K-K), D3 (Stegmaier), A5 (Vaccarino) + partial D4 (Rosenberg), B5 hold (Chvátil), D1 partial (K-K). All adopted; +0.5r each.

## Silent-retire: 0

## Spec amendments required for v1.1

Per Parliament verdict, the following must be added before implementation begins:

### A-spec-1.1-01 — Randomness source declared

Add to §`/tigris-ideate` skill — Behavior:
- Declare that sampling uses LLM judgment with seed-file fallback for reproducibility.
- OR declare explicit non-reproducibility as design choice.
- Either way, spec must state what "uniform random" means operationally.

### A-spec-1.1-02 — Minimum-viable pool sizes

Add to §Initial bootstrap:
- Minimum pool sizes for skill to function:
  - primitives.md: ≥ 10 entries
  - mashups.md: ≥ 5 entries
  - gaps.md: ≥ 1 section populated
  - frustrations.md: may be empty; skill treats as dropped source if so
- Skill should gracefully handle any pool being below min or empty (skip that source).

### A-spec-1.1-03 — Failure modes enumerated

Add new §`Failure modes`:
- Empty pool → source skipped.
- All pools empty → skill errors with instruction to seed.
- Near-duplicate candidates → regeneration loop; if 3 attempts fail, skill emits with warning.
- Stale gaps → spec can't detect; user responsibility. Document as known limitation.

### A-spec-1.1-04 (OPTIONAL) — Hopper archive auto-trigger

Current spec says archive is "manual housekeeping when > 200 entries." This is dead protocol text. Either:
- Remove the 200-threshold line (YAGNI), or
- Make it a skill responsibility (auto-archive on every run if > threshold).

Recommend: remove the line. Archive is out-of-scope for v1.0.

### A-spec-1.1-05 (OBSERVATIONAL) — Frustrations pool decay watch

Flag in §Open questions: hand-curated logs tend to abandoned. If frustrations.md sits unchanged for 10+ games, consider:
- Remove frustrations as a source (collapse to 4 sources).
- Add auto-scraping of external forums (out-of-scope v1.0).

## Success criteria

| # | Criterion | Result |
|---|---|---|
| 1 | ≥ 5 earned | PASS (6) |
| 2 | ≥ 2 on Band B/C | PASS (B1, B3, B4, C4, C7, C8) |
| 3 | ≥ 1 clean collision | PASS (A1↔B1, 4-2 for B1) |
| 4 | Spec-amendment or Pool-amendment | **PASS — 3 required spec amendments + 1 optional + 1 observational** |

## Verdict

**Spec is approved to proceed to v1.1 implementation after A-spec-1.1-01 / 02 / 03 are applied.**

Core architecture (5-pool + 1-skill + lifecycle) survives the review. Three substantive gaps:
1. Randomness source unspecified (Vaccarino refute — A4).
2. Bootstrap burden not gated by minimum sizes (Stegmaier refute — A6).
3. Failure modes not enumerated (Feld refute — B2).

All three are fixable by inline additions to existing sections. No structural re-design needed.

**Recommended action**: apply amendments A-spec-1.1-01/02/03 to the spec, mark v1.1, then proceed to writing-plans for implementation.

Anchor B1 (Lacerda / System Gearing) carries the review — the spec is architecturally sound where it declares write discipline, pool-skill separation, and one-way read contracts. The refutes are at the edges, not at the core.
