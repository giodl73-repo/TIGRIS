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

Resolution finding before the adapter: the native lock currently uses
serde/serde_core/serde_derive1.0.229; the released shared transport requires
serde1.0.228 exactly. Align those three registry packages to1.0.228 so the
native baseline and companion share the same Serde identities. This is a named
compatibility constraint, not a general registry refresh. Keep all other native
registry/Git pins unchanged and rerun both original test matrices and byte
controls after alignment. Serde derive 1.0.228 uses the existing syn2.0.119;
the now-unused syn3.0.3 is removed, with no other package upgrades.

Qualification: all 18 unchanged native tests pass under default and portable
feature matrices. Strict default all-target and portable wasm32 library Clippy
pass. All 14 captured original native simulation, CLI, save and resume records
remain byte-identical (SHA256
bd12a8fa9737ff06399e44d1faf20e48f9dcaff8a879408d6d53c3f74003abc0).
The seven MUDDLE packages share the exact released revision; muddle-core now
uses the already-locked RUNE core/derive. COURT/RACKET/RALLY pins are unchanged.
