---
name: Ideate-Hopper Plan — Parliament Review
slug: ideate-hopper-plan-review
stage: panel
version: 1.0.0
rubric_version: v2.15
bet_version: parliament
artifact_under_review: docs/specs/plans/2026-04-20-tigris-ideate-hopper-plan.md
personas_seated: [knizia, rosenberg, feld, lacerda, chvatil, kramer-kiesling, stegmaier, vaccarino]
review_type: meta-review (TIGRIS-internal plan)
anchor_persona: stegmaier
anchor_axis: A6
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# Ideate-Hopper Plan — Parliament Review

Meta-review of `docs/specs/plans/2026-04-20-tigris-ideate-hopper-plan.md` under Parliament procedure. 8 designers, no player lenses. Axes adapted to plan context: "game behavior" reads as "plan-execution behavior"; "canonical case" reads as "reference-engineer-readability."

## Anchor

**Stegmaier** anchors **A6 Teachability.** Anchor claim: "A plan's job is to be executed by an engineer with zero context. Every step is a teach moment. Canonical concern for a plan review."

Adjacency partner: **C7 Action-Menu Clarity** (K-K cross-player) — closely related to step clarity. GATE passes.

## Drafts (24)

### Knizia
1. **A1 Elegance.** 5/10. "10 tasks. Could Tasks 2-6 (all 'seed a pool file') collapse into one composite task? Separation is clear but verbose."
2. **C2 Minimum-Score Shape.** 1/10. "N/A to plan. Retire-explicit."
3. **C1 Tension Budget.** 2/10. "N/A. Retire-explicit."

### Rosenberg
1. **B3 Conversion Chain Depth.** 6/10. "Plan has a clear pipeline: scaffold → seed pools → write skill → modify companion skill → smoke test → failure tests. 6-stage chain. Earn."
2. **C3 Scarcity Bite.** 2/10. "N/A. Retire-explicit."
3. **D4 Late-Game Lock-in.** 4/10. "Task 10 locks in the done-criteria mapping; earlier tasks can be reverted. Weak lock-in. Hold."

### Feld
1. **A2 Decision Density.** 6/10. "Each step has a concrete decision or action. Density healthy across 10 tasks."
2. **B2 Catastrophe Pressure.** **4/10.** "**Soft refute.** Plan doesn't enumerate what happens when Task 9 (live smoke test) fails — if skill doesn't generate viable candidates, what's the rollback? Plan silent on bad-output handling. Refute."
3. **C6 Point-Salad.** 1/10. "N/A. Retire-explicit."

### Lacerda
1. **B1 System Gearing.** **6/10.** "**Soft refute on one gap.** Task 7 writes `/tigris-ideate` skill but plan doesn't address skill *registration* — how does Claude Code discover the new skill? TIGRIS convention is auto-discovery from `skills/` dir, but plan should state this explicitly. Gearing depends on discovery."
2. **B4 Information-Transparency-Cost.** 7/10. "Exact file paths everywhere; expected outputs declared; no hidden state. Earned."
3. **A7 Emergence-Replayability.** 2/10. "N/A to a linear plan. Retire-explicit."

### Chvátil
1. **B5 Architectural Novelty.** 2/10. "Standard implementation plan. No novelty claim. Retire-explicit."
2. **A3 Interaction.** 1/10. "N/A. Retire-explicit."
3. **B6 Scoring Multiplier.** 1/10. "N/A. Retire-explicit."

### Kramer-Kiesling
1. **C7 Action-Menu Clarity.** **7/10.** "Each step has: Files section, numbered substeps, code blocks where code is needed, Run commands where commands are needed, exact expected outputs. Canonical clarity. Earned."
2. **D1 Family-to-Expert Scaling.** **5/10.** "**Soft refute.** Plan assumes Edit-tool-competent engineer. A first-time TIGRIS contributor couldn't execute Task 8 Step 2 (find-and-replace on a skill file) without seeing the *current* file first. No 'Read this file first' step in Task 8. Refute."
3. **D2 Spatial-Interaction.** 1/10. "N/A. Retire-explicit."

### Stegmaier
1. **A6 Teachability (anchor).** **7/10.** "Plan header includes architecture summary + tech stack + spec link. Each task has Files section + bite-sized steps with expected outputs. Readable by fresh engineer — but the D1 gap (Task 8 read-first) bites teachability too. Earn with caveats."
2. **D3 Count-Robustness.** 1/10. "N/A. Retire-explicit."
3. **C8 First-Turn Compression.** 6/10. "First task (scaffold + README) is compressed + self-contained — good on-ramp. Earn."

### Vaccarino
1. **A4 Variance Calibration.** **4/10.** "**Soft refute.** Task 9 Step 1: 'Run /tigris-ideate'. But the skill is LLM-judgment-based (per spec v1.1 A-spec-1.1-01). Plan's expected output is '3 candidates appended.' What if the LLM generates 3 *bad* candidates? No quality gate in plan. 'Coherent' in done-criteria is subjective."
2. **C4 Engine-Garden.** 3/10. "N/A to plan. Retire-explicit."
3. **A5 Downtime-Pacing.** 2/10. "N/A. Retire-explicit."

## Argument

### Round 1 — the skill-registration gap

- **Lacerda (B1) attacks**: "Task 7 creates `.claude/skills/tigris-ideate/SKILL.md`. But plan never says 'now Claude Code discovers the new skill.' Is there a registration step? A restart? An import? Plan assumes auto-discovery but doesn't declare it. Refute."
- **Rosenberg defends weakly**: "TIGRIS convention — existing skills in `skills/` are auto-discovered. Implicit."
- **Stegmaier (anchor) concedes**: "Implicit isn't good enough for a plan targeted at 'engineer with zero context.' Plan should state the discovery assumption."
- **Vote**: Lacerda → refute; Stegmaier → refute; K-K → refute; Feld → refute; Rosenberg → defense; Chvátil → neutral; Vaccarino → refute; Knizia → neutral.
- **Result: 5-1-2 refute. B1 soft-refute. Amendment required.**

### Round 2 — the read-before-edit gap (Task 8)

- **K-K (D1) attacks**: "Task 8 Step 2 says 'Find this block / Replace with.' But the Edit tool REQUIRES the engineer to have Read the file first (that's a tool contract, not optional). Plan doesn't include a 'Step 1: Read .claude/skills/tigris-concept/SKILL.md to refresh exact strings.' A fresh engineer will try the Edit and fail with 'file not read' error. Refute."
- **Stegmaier (A6 anchor) agrees**: "Teachability refute compounds here — plan teaches an engineer to fail on first try."
- **Vote**: K-K → refute; Stegmaier → refute; Lacerda → refute; Feld → neutral; others → partial.
- **Result: 3-0-5 leaning refute. D1 soft-refute. Amendment required.**

### Round 3 — the failure-handling gap (Task 9)

- **Feld (B2) attacks**: "Task 9 says 'Expected: 3 candidates appended with plausible anchor.' What does the engineer do if expected output isn't met? No rollback, no diagnostic, no re-run instruction. Plan is optimistic about the happy path."
- **Vaccarino (A4) agrees**: "And since output is LLM-judgment-based, variability is the norm, not the exception. Plan needs quality gate or re-invoke instruction."
- **Vote**: Feld → refute; Vaccarino → refute; Stegmaier → refute; others → partial.
- **Result: 3-0-5 leaning refute. B2 + A4 soft-refute. Amendment required.**

### Round 4 — the task-count elegance question

- **Knizia (A1) attacks mildly**: "Tasks 2-6 are all 'create a file in ideas/ with a schema header.' Could collapse to one task 'Seed the 5 pool files' with 5 substeps. 6-task plan vs 10-task plan."
- **Lacerda defends**: "Per-file separation matches commit-per-task discipline. 5 small commits is TIGRIS YAGNI-friendly. Compression here would obscure what changed when."
- **Stegmaier (anchor) defends**: "Commit-per-pool helps a bisect if something breaks. Keep 10 tasks."
- **Vote**: Knizia → partial refute (verbose); Lacerda → defend; Stegmaier → defend; Feld → defend; K-K → defend; others → neutral.
- **Result: 4-1-3 for current structure. A1 hold.** No amendment required.

### Collision 1 — A6 Teachability vs D1 Family-to-Expert

- **K-K (D1)**: "Plan expert-biased; fresh engineer fails Task 8 and doesn't know about skill discovery."
- **Stegmaier (A6)**: "Plan readable and mostly self-contained; the gaps are addressable."
- **Vote**: Both agree the issues are real — orthogonal-preservation. Both earn with open amendments.
- **Result**: both survive. No winner-take-all.

## Earned stakes (4)

| Persona | Axis | Status |
|---|---|---|
| **Stegmaier (anchor)** | **A6 Teachability** | Earned (with caveats from D1 refute) |
| K-K | C7 Action-Menu Clarity | Earned |
| Rosenberg | B3 Conversion Chain | Earned (6-stage task pipeline) |
| Lacerda | B4 Information-Transparency-Cost | Earned |

## Refuted stakes (4 soft-refutes on plan quality)

| Persona | Axis | Status |
|---|---|---|
| **Lacerda** | **B1 System Gearing** | **Soft refute** — skill discovery assumption not declared |
| **K-K** | **D1 Family-to-Expert** | **Soft refute** — Task 8 missing read-before-edit step |
| **Feld** | **B2 Catastrophe Pressure** | **Soft refute** — failure-handling / rollback not enumerated |
| **Vaccarino** | **A4 Variance Calibration** | **Soft refute** — LLM output variance unaddressed; no quality gate |

## Retire-explicits (10)

All N/A-to-plan axes. Adopted preservations +0.5r each.

## Silent-retire: 0

## Plan amendments required for v1.1

### A-plan-1.1-01 — Declare skill discovery assumption

Add to plan header or Task 7 intro:
> Claude Code auto-discovers skills placed in `.claude/skills/<skill-name>/SKILL.md` (project-local) or `~/.claude/skills/<skill-name>/SKILL.md` (user-level). Skills at a non-`.claude/` path are **not** discovered. After commit, invoke with `/tigris-ideate` and Claude Code will recognize the skill (live-change detection typically applies within the current session).

### A-plan-1.1-02 — Add Read-before-Edit step in Task 8

Insert new Step 0 at start of Task 8:
```
- [ ] **Step 0: Read the current skill file**

Run: Read `.claude/skills/tigris-concept/SKILL.md`
Expected: current content matches the blocks referenced in Steps 2 and 3. If the file has drifted from the plan's expected blocks, adjust the `old_string` arguments accordingly.
```

### A-plan-1.1-03 — Enumerate failure handling for Task 9

Add new Step after Task 9 Step 1:
```
- [ ] **Step 1b: Evaluate output quality**

Check each generated HOP-NNN entry:
- Anchor-persona is in `personas/designers/*.md` roster?
- Anchor-axis is in `personas/axis-pool.md`?
- One-liner is ≥ 2 sentences and topical to sources named?

If any candidate fails these checks, re-run `/tigris-ideate --count 3` and replace (manually edit hopper.md to retire the failing entry, or delete if never committed). Do not proceed to Task 10 with bad candidates.
```

### A-plan-1.1-04 — Add LLM-variance note to Task 9 header

Add to Task 9 top:
> **Note on variance:** `/tigris-ideate` output is LLM-judgment-based (spec v1.1 §`/tigris-ideate` Step 2). Different runs produce different candidates. "Expected output" in this task refers to *shape* (3 entries, fresh status, unique IDs, schema-compliant), not specific content. If first run produces obviously-bad candidates, re-run before committing.

## Success criteria

| # | Criterion | Result |
|---|---|---|
| 1 | ≥ 5 earned | **SOFT PASS (4)** — just below threshold due to high retire-explicit rate (plan reviews inherently limit applicable axes) |
| 2 | ≥ 2 on Band B/C | PASS (B3, B4, C7) |
| 3 | ≥ 1 clean collision | PASS (A6↔D1 orthogonal-preservation) |
| 4 | Plan-amendment | **PASS — 4 required amendments** |

**Criterion 1 soft-pass** because plan-reviews inherently have many N/A axes (C1, C6, A3, A7, C4, A5, B5, B6, D3, D2, D4, C2, C3 all retire-explicit for processes-not-games). 4 earned against ~14 applicable axes ≈ 29% earn rate, which is healthy for a plan.

## Verdict

**Plan is approved to proceed to execution after A-plan-1.1-01 / 02 / 03 / 04 are applied.**

Core plan structure (10 tasks, bite-sized steps, commit-per-task) survives. Four gaps at edges:
1. Skill-discovery assumption undeclared (Lacerda B1).
2. Task 8 missing Read-before-Edit step (K-K D1).
3. Task 9 missing quality-gate on LLM output (Feld B2).
4. Task 9 missing LLM-variance note (Vaccarino A4).

All four are inline additions. No re-structuring needed.

**Recommended action**: apply A-plan-1.1-01/02/03/04 to the plan, bump to v1.1, then proceed to execution (subagent-driven or inline).

Anchor A6 (Stegmaier / Teachability) carries the review — plan teaches the happy path well, but teaches failure-paths inadequately. Amendments close the gap.
