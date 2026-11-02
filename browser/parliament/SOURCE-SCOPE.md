---
name: Parliament corresponding software scope
slug: parliament-browser-source-scope
stage: design
version: 1.0.0
rubric_version: v2.24.85
bet_version: parliament
author: TIGRIS
created: 2026-11-02
updated: 2026-11-02
sources: [LICENSE, tools/tigris-sim/Cargo.toml]
---

# Corresponding software, not a corpus mirror

The source ZIP includes the complete first-party `tools/tigris-sim/` crate
(original table host, seeded simulation, three native hosts, all 18 original
tests, manifest, lock and ordinary software docs), the complete unpublished
`browser/parliament/` companion (adapter, UI, test/build/package scripts, locks,
synthetic QA inputs), original LICENSE and .gitignore. Generated caches,
node_modules, build artifacts and ignored local files are excluded.

`source-files.json` lists each included Git blob and SHA256. `omitted-files.json`
lists other tracked paths. Games, personas, hopper, reference material, creative
handoffs, research, imported published-game rules and review corpus are omitted
from this executable source package. This does not withdraw any prior grant.
The complete repository at the recorded commit remains separately available
through the full Git archive link in build.json and the licenses page.

The existing LICENSE retains MIT software/ordinary software documentation and
CC BY-NC original non-software content scopes, with third-party grants separate.
Do not infer blanket MIT terms for the full repository or its linked content.
Dependencies are pinned in Cargo.lock and downloaded by Cargo; they are not
vendored. Their original notices and Rust runtime notices accompany the compiled
browser artifact. Native optional graphics dependencies retain native scope.

The native table embeds its established pilot strings and intent as software;
they are retained in source. Source availability does not authenticate caller
saves, conduct review, amend the rubric or certify factory/corpus evidence.
