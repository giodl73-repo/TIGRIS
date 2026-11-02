---
name: Parliament browser admission and task
slug: parliament-browser-model
stage: design
version: 1.0.0
rubric_version: v2.24.85
bet_version: parliament
author: TIGRIS
created: 2026-11-02
updated: 2026-11-02
sources: [tools/tigris-sim/src/lib.rs]
---

# Browser task and boundaries

This mounts the existing deterministic Parliament AI pilot. TIGRIS owns its
host rules; MUDDLE owns session parsing, successful turns, save replay/import,
client snapshot and its shared Rust JSON transport. JavaScript presents the
native hints, panels, complete history and visual intent; it does not simulate
policy or fetch references. Native CLI and desktop hosts remain available.

Browser admission rejects whole operations: request 512 KiB, save 128 KiB,
checkpoint 8 KiB, each checkpoint field/command/room 1024 UTF-8 bytes, 128 saved
commands plus new attempts, report 2 MiB. Every numeric checkpoint occurrence
is checked before native resume: unsigned counters at most 4096, signed score
and tiger-marker magnitude at most 10000. Malformed/unknown fields remain native
errors. These are browser resource limits, not universal native save limits.
Result saves must satisfy the same admission. History is never truncated.

The WASM build sets a 64 MiB memory maximum. File reads, engine startup and
computation have 5/12/10 second deadlines. Worker generations, job IDs and
candidate revisions fence stale results. Accepted state/exports stay intact on
refusal or failure. New table replaces accepted state only on success. Original
opened bytes, current editor text, LF text and accepted native exports have
separate custody. Retry retains accepted state. New table resets attempt history.

Native resume replays commands, checks the final room, then imports a caller
checkpoint. It does not authenticate the host, match checkpoint to transcript,
prove provenance, conduct a game review or amend the rubric. Scores, adopted
axes and closed flags are local pilot state. Games, corpus, personas, Pool,
innovation log and hopper are not changed by this workbench.
