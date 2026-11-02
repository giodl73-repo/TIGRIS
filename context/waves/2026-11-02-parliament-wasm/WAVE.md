---
name: TIGRIS Parliament AI WASM table
slug: parliament-wasm
stage: design
version: 1.0.0
rubric_version: v2.24.85
bet_version: parliament
author: TIGRIS
created: 2026-11-02
updated: 2026-11-02
sources: [tools/tigris-sim/README.md, tools/tigris-sim/src/lib.rs, docs/handoff/2026-04-23-post-rite-upstage.md]
---

# Existing Parliament AI table in the browser

Reader task: play the existing deterministic Parliament AI table pilot through
its native room/command session, inspect the original hints, phase/next-action
beats, resources and transcript, and export/resume native saves/checkpoints.
Allow typed commands and caller-owned native save text. This is the established
pilot, not a new implementation of Parliament or a game-review/axis-amendment
engine. No automated persona review, corpus certification or rubric mutation.

## Source custody and native baseline

Canonical entry 2b2e389 is one PITFALL factory-evidence commit ahead of public
main 6c91cc5. It is preserved remotely as pitfall/factory-evidence-20261102.
Implementation is isolated in an owned child worktree; the canonical checkout
and the other owner sessions remain untouched until a qualified portfolio pulse.

All 18 original Rust tests pass (10 library/table tests and eight seeded
simulation tests). Formatting passes. Strict Clippy finds one existing indexed
player loop in UPSTAGE simulation; preserve native outputs/RNG order while
correcting it without allowances. Keep all original tests and native binaries.

The manifest currently makes native CLI/window/Macroquad dependencies
unconditional. Make them optional behind a default-enabled native-hosts feature
and require that feature for their native binaries. The library remains the same
TIGRIS policy with the same Git-pinned MUDDLE/Court/RALLY dependencies. Do not use
TRACKER-relative dependencies or silently upgrade production dependency versions.

Native MUDDLE save resume replays commands, checks final room, then imports the
host checkpoint. Saves do not authenticate host identity, transcript/state
consistency, provenance or review authority. The original TIGRIS importer loops
over supplied round; browser admission must bound numeric checkpoint fields
before native import, not after an unbounded loop. Retain native parser ownership;
an explicit pre-import resource-limit scan is not a new save-format parser.

## Agent role review (not human approval)

| Lens | Decision |
|---|---|
| Table Experience Observer | Preserve readable native table/phase/next-action and command hints; show every accepted turn and native error. Provide keyboard/mobile play and export/resume. This is pilot interaction evidence, not a human playtest finding. |
| Corpus Claim Auditor | Pilot score/closed/adopted counters are local game state, not scored corpus evidence, observed market/player research or proof of the factory's generality. No imported published-game rules/content are bundled. |
| Axis Governance Steward | Native score-amendment/close commands may change local pilot state only. Pool ledger, rubric version, game reviews, hopper and innovations stay governed by existing evidence procedures. Caller saves can be edited; acceptance is not legitimacy. |
| Shared-contract boundary | MUDDLE owns sessions, save format, client snapshot/controls and transport serialization; TIGRIS owns the existing host/policy. Reuse or expose the established upstream snapshot transport before adopting it, rather than create a product-owned wire-schema fork. |

## Delivery and qualification

1. Preserve canonical PITFALL history; capture native CLI/simulation controls,
   close one scoped lint issue, and qualify default/native and portable feature
   matrices with all original tests. Keep dependency lock identities stable.
2. Provide a bounded adapter over original host/session/save APIs and shared
   MUDDLE snapshot transport. Reject whole oversized inputs/results, with real
   worker/read/startup deadlines and compiled memory maximum. Independently
   compare native CLI transcript/save controls and executing browser WASM.
3. Ship interactive local play, own save import, typed/hinted commands, clear
   native refusals, restart and separate original/current/request/report/save/
   transcript exports. Fence stale file reads, workers and jobs atomically.
4. Package exact complete software/native tests and required first-party source
   with explicit included/omitted paths and full-repo link; no corpus/reference
   content mirror by inference. Retain MIT software, CC BY-NC content and upstream
   grants in their original scope. Rebuild delivered source outside Git, then
   qualify hosted/default/live behavior and every artifact hash before snapshot.
5. Write an immutable portfolio handoff using tigris-handoff: retain the game
   factory's current state and ranked creative priorities; distinguish the WASM
   engineering resume point. Do not rewrite existing handoffs or creative ledgers.
