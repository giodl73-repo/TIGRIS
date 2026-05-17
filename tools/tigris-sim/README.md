# tigris-sim

Seeded board-game simulation pilot for TIGRIS.

The first adapter targets `games/0001-parliament/` and models the argument
loop at a coarse systems level: axis draft, stakes, adjacency collisions,
defense/refutation marks, adoption pressure, and final scores. TIGRIS owns the
Parliament-specific policy; RALLY supplies deterministic run, actor trace, and
metric primitives.

## Commands

```powershell
cargo test --quiet
cargo run --quiet -- --seed parliament-smoke
cargo run --quiet -- --seed parliament-smoke --runs 20 --players 4
```

## Current validation signal

- Collision count and collision rate.
- Axis adoption/refutation pressure.
- Per-chair action and blocked-turn traces.
- Batch adoption rate, no-collision rate, no-adoption rate, and win spread.
- RALLY validation status and findings.
