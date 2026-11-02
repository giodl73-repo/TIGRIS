---
name: TIGRIS adopts shared MUDDLE transport
slug: parliament-wasm-pulse-02
stage: design
version: 1.0.0
rubric_version: v2.24.85
bet_version: parliament
author: TIGRIS
created: 2026-11-02
updated: 2026-11-02
sources: [tools/tigris-sim/Cargo.toml, tools/tigris-sim/Cargo.lock]
---

# Pulse 02: qualify the released shared transport dependency

MUDDLE PR2 is main/source/hosted/live qualified at
08aea37b9849a2e4ec69ea3786efbc09ba57f8d1. Its public snapshot_json Rust API
retains the original client mapping and supports default-disabled embedding.
TIGRIS still pins cf13a86b. That older-to-newer native dependency adoption must
be verified before a browser adapter is declared compatible.

Pin all direct MUDDLE packages to the one released Git revision; browser
transport/core/CLI dependencies must use that same Git source identity. Keep
unrelated COURT, RACKET, RALLY and registry pins unless a documented resolution
constraint requires a change. Preserve the existing TIGRIS table policy and
native launchers, all 18 tests under both feature matrices, and all 14 captured
original simulation/CLI/save/resume controls. Run strict native/WASM lint and
inspect the exact lock delta. New helper-specific adapter dependencies will
receive a committed standalone lock, with shared identities checked explicitly.

No product browser release or corpus/rubric evidence is claimed by adopting
transport. The original checkpoint importer still needs browser admission
before its round loop; original native save parsing and semantics stay owned by
MUDDLE/TIGRIS rather than a JavaScript or transport schema fork.
