---
name: Parliament browser implementation and qualification
slug: parliament-wasm-pulse-03
stage: design
version: 1.0.0
rubric_version: v2.24.85
bet_version: parliament
author: TIGRIS
created: 2026-11-02
updated: 2026-11-02
sources: [browser/parliament/MODEL.md, browser/parliament/BUILD.md]
---

# Bounded native table in a worker

The standalone unpublished companion uses the original Parliament host,
MUDDLE session/save/transcript APIs and released snapshot_json transport with
player-entry disabled. All shared native/companion package identities match;
MUDDLE packages share one exact released revision. No JavaScript game rules or
copied shared snapshot schema are added. The original table and native launchers
remain unchanged, with the previously recorded UPSTAGE lint-only loop repair.

Seven adapter tests, strict native/WASM Clippy and release native/WASM builds
pass. Fifty-two responses (each native case repeated) match executing WASM;
12 independent original native CLI save/transcript pairs match exactly. All
16 local task/recovery checks pass, including real 5/12/10 second deadlines,
stale candidate/file/worker fencing, retry, mobile, keyboard and separate byte
custody. Actual memory.grow beyond the compiled 64 MiB maximum is rejected.
Source/legal/package delivery, outside-Git replay and hosted/default/live gates
are next; this is not yet a browser release or canonical snapshot.

QA history is retained in ignored .cache logs and trace archives. The first
full nine-case run failed at initial rendering: the shared native map is
optional text, not a list. Correcting the renderer yielded all nine passes.
The first expanded recovery run passed 15/16: its test-only direct page WASM
call omitted init (the product worker had initialized correctly). Initializing
that independent test module yielded all 16 passes. An earlier Rust boundary
fixture attempted end turn from the table room; using the actual board room
exercises the unchanged overflow-result refusal. No assertion/limit/deadline
was weakened or bypassed.

Original scores/adopted axes/closed flags remain pilot state, not review or
rubric authority. Explicit software source scope excludes creative/reference
corpus mirroring and retains original split grants and third-party notices.
