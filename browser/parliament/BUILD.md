---
name: Build Parliament browser pilot
slug: parliament-browser-build
stage: design
version: 1.0.0
rubric_version: v2.24.85
bet_version: parliament
author: TIGRIS
created: 2026-11-02
updated: 2026-11-02
sources: [browser/parliament/Cargo.toml, browser/parliament/Cargo.lock]
---

# Build and replay

Use Python 3.11+, Rust 1.95.0, wasm-bindgen-cli 0.2.127, Node 22+ and the
committed npm lock. Cargo downloads pinned Git/registry dependencies; they are
not vendored into this source archive. First-party path dependencies stay inside
the extracted software tree. No TRACKER checkout is required.

```powershell
rustup toolchain install 1.95.0 --profile minimal
rustup target add wasm32-unknown-unknown --toolchain 1.95.0
cargo +1.95.0 install wasm-bindgen-cli --version 0.2.127 --locked
cargo +1.95.0 test --manifest-path tools/tigris-sim/Cargo.toml --all-targets --locked
cargo +1.95.0 test --manifest-path tools/tigris-sim/Cargo.toml --no-default-features --all-targets --locked
cargo +1.95.0 test --manifest-path browser/parliament/Cargo.toml --all-targets --locked
cargo +1.95.0 clippy --manifest-path browser/parliament/Cargo.toml --all-targets --locked -- -D warnings
cargo +1.95.0 clippy --manifest-path browser/parliament/Cargo.toml --lib --target wasm32-unknown-unknown --locked -- -D warnings
python browser/parliament/build-prototype.py --bindgen <path-to-wasm-bindgen>
cd browser/parliament
npm ci
npx playwright install chromium
npm run test:pages
```

Serve `target/parliament-prototype` to expose `/TIGRIS/`; the test runner starts
its own local server. Do not open the HTML with a file URL. Release packaging
also provides software source, complete grants/notices and an exact build/file
record. Source/license/download routes must work when the engine fails.

The native control binary takes one JSON request on stdin. Its transport is
identical to the executing WASM export. Request examples:

```json
{"commands":["choose persona","go board","draft axis","stake claim","end turn"]}
```

Use `save_text` for your own native MUDDLE save, and `commands` for subsequent
attempts. See MODEL.md for admission, checkpoint semantics and claim boundaries.
The independent original CLI controls run the unchanged `tigris-muddle` binary,
including native save and transcript exports, rather than comparing two calls
to the browser adapter. The seeded simulation and desktop hosts remain native.
