---
name: TIGRIS native portability foundation
slug: parliament-wasm-pulse-01
stage: design
version: 1.0.0
rubric_version: v2.24.85
bet_version: parliament
author: TIGRIS
created: 2026-11-02
updated: 2026-11-02
sources: [tools/tigris-sim/Cargo.toml, tools/tigris-sim/src/lib.rs, tools/tigris-sim/src/main.rs]
---

# Pulse 01: preserve native behavior and separate desktop hosts

Before implementation: all 18 original tests and formatting pass at 2b2e389.
Strict lint reports one needless_range_loop in the UPSTAGE CLI simulation at
src/main.rs:689. Native CLI/scripted/save and seeded simulation controls must be
captured before that narrow repair or feature/dependency changes.

Planned default-enabled native-hosts feature gates desktop-only dependencies
and the three existing MUDDLE launchers. Seeded simulation and core table policy
stay available independently. Keep Cargo.lock identities unchanged and validate
the default all-target and no-default library/test matrices, strict lint and
wasm32 library compilation. No native behavior or product readiness claim follows
from portability alone; browser/source/default/live gates remain open.
