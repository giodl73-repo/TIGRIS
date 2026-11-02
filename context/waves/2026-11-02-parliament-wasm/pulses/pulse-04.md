---
name: Native control oversized UTF-8 admission
slug: parliament-wasm-pulse-04
stage: design
version: 1.0.0
rubric_version: v2.24.85
bet_version: parliament
author: TIGRIS
created: 2026-11-02
updated: 2026-11-02
sources: [browser/parliament/src/main.rs, browser/parliament/tests/admission.rs]
---

# Keep native and WASM refusal transport equal

After PR4/default qualification, a final boundary probe sent a valid 600016-byte
JSON request containing multibyte UTF-8 to the native control CLI. Its bounded
read_to_string prefix split a character, returning InvalidData exit1 before
Rust request admission. Browser evaluate already returned browser_refusal.

Read at most request-limit plus one raw bytes, check byte admission before
decoding, and route oversized requests through the same Rust refusal. Valid
admitted UTF-8 remains unchanged; malformed UTF-8 remains a native I/O error.
The public Rust limit constant avoids a second CLI-specific numeric limit.
No input/result is truncated and native policy, browser UI, all prior tests,
limits and deadlines remain unchanged. Add a real subprocess regression and
the multibyte request to native/executing-WASM controls (now53); retain all12
original CLI pairs. This follow-up must qualify before TRACKER's release pulse.
