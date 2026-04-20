---
name: Ideate-Hopper v1.0 — Smoke Test Notes
slug: ideate-hopper-smoke-test-notes
version: 1.0.0
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# Smoke Test Notes — Ideate-Hopper v1.0

Observations from first-run simulation of the `/tigris-ideate` skill.

## Verified in-session

- **Pool minimum-viable sizes enforced**: primitives (15) ≥ 10, mashups (5) ≥ 5, gaps populated across 5 sections, frustrations empty (treated as dropped source per spec).
- **SKILL.md procedure well-specified**: Following Steps 1-5 manually produced 3 coherent candidates without ambiguity in the instructions.
- **Write discipline holds**: only `ideas/hopper.md` was modified by the run; no pool files touched.
- **HOP-NNN counter updates monotonically**: HOP-000 → HOP-003 after 3 candidates, consistent with spec invariant.
- **Quality gates (A-plan-1.1-03) apply cleanly**:
  - Anchor-persona check against roster → all 3 pass (Chvátil, Stegmaier, Lacerda).
  - Anchor-axis check against Pool → all 3 pass (B5, A6, A7 all adopted).
  - One-liner ≥ 2 sentences + topical to sources → all 3 pass.
  - Tension hypothesis names panel argument → all 3 pass.

## Deferred to live-skill invocation

The following plan Task 10 steps require the `/tigris-ideate` skill to be live in the session (discoverable by Claude Code's skill registry, not just on disk). Because the skill was written in this same session, invocation via the Skill tool requires a session restart per plan amendment A-plan-1.1-01.

- **Task 10 Step 1** — `/tigris-ideate --sources frustration` with empty frustrations pool. Documented behavior: source skipped; then error because no other active source. Needs live run to confirm error message matches spec.
- **Task 10 Step 2** — Below-minimum primitives test (rename `.bak`, run `/tigris-ideate --sources primitives`). Needs live run.
- **Task 10 Step 3** — Promote a HOP entry and run `/tigris-concept HOP-NNN`. Requires user to (a) review HOP-001/002/003, (b) promote one, (c) invoke the updated `/tigris-concept` skill. The skill edit is in place (commit `61fb248`); exercising it end-to-end needs a session where both skills are live.
- **Task 10 Step 4** — Single-commit verification for concept+hopper. Depends on Step 3.

## Recommended post-restart sequence

1. Restart the Claude Code session so `/tigris-ideate` and the updated `/tigris-concept` are discoverable.
2. Run `/tigris-ideate` once to confirm live behavior matches the simulated run (hopper gains HOP-004/005/006 or similar).
3. Review HOP-001 / HOP-002 / HOP-003; promote one you find interesting by editing `ideas/hopper.md` to set its status to `promoted`.
4. Run `/tigris-concept HOP-NNN` (replacing NNNN with your chosen ID) to generate `games/0018-<slug>/concept.md`.
5. Verify the hopper entry flipped to `consumed` with backlink, and both files committed together.
6. Optionally exercise failure modes (`--sources frustration` on empty pool; temporarily shrink primitives.md below 10 entries and retry).

## Known-good state

- Commits `9209709..57e8bd8` on `main` cover Tasks 1-9 of the plan.
- Pool files + skill files stable and spec-compliant (per `review.md` + quality review).
- 3 candidates in hopper awaiting user review.

## What a failure of the deferred tests would look like

If post-restart invocation reveals defects:

- **Skill not discovered** → check `skills/tigris-ideate/SKILL.md` path + frontmatter; verify no typo in `name: tigris-ideate`.
- **Error on empty pool** → check SKILL.md §Preconditions wording; may need a regression fix.
- **`/tigris-concept HOP-NNN` doesn't find entry** → check updated `skills/tigris-concept/SKILL.md` Step 0 logic; commit `61fb248`.
- **Single-commit discipline violated** → Step 6 of updated `/tigris-concept` needs tightening.

Any defect surfaced during deferred tests should be logged as a follow-up amendment to the plan or spec (not silently patched).
