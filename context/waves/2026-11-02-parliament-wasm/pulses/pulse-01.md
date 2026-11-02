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

## Qualified native foundation

Both default and no-default all-target test matrices pass all original 18 tests.
Formatting and strict default/native plus portable WASM Clippy pass. The single
loop repair uses enumerate over mutable players in the same order and retains
all RNG calls/conditions. Fourteen captured controls match byte for byte: eight
seeded Parliament/UPSTAGE batch/variant reports, three native CLI save/transcript
scripts and three native resume outputs. The raw before/after records and logs
are retained in TRACKER's private QA cache; they are not caller data or release
receipts. Original table library, all native launcher source and Cargo.lock are
byte-unchanged. The package MIT identifier reflects the existing software grant;
the split content/software notice and prior grants remain unchanged.

No browser transport or Pages release is claimed. The shared MUDDLE snapshot
serializer is currently private to its existing player crate; expose/reuse that
upstream boundary before adding TIGRIS transport, and preserve all native
dependency identities or explicitly qualify any necessary adoption change.
