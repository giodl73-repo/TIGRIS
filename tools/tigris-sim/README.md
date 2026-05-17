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
cargo run --quiet -- --seed parliament-smoke --compare-variants --runs 20 --players 4
```

## Current validation signal

- Collision count and collision rate.
- Axis adoption/refutation pressure.
- Per-chair action and blocked-turn traces.
- Batch adoption rate, no-collision rate, no-adoption rate, and win spread.
- Rule-variant comparison for adoption-pressure tuning.
- RALLY comparison-report status for each variant (`improved`, `mixed`, or
  `regressed`) against baseline adoption rate, collision count, and
  no-adoption rate.
- RALLY validation status and findings.

## Rule variants

| Variant | Purpose |
|---|---|
| `baseline` | Current Parliament pressure model. |
| `expanded-adjacency` | Adds more axis pairs so collisions fire more often. |
| `lower-adoption` | Tests whether the adoption threshold is the blocker. |
| `collision-boost` | Raises collision reward and challenge pressure. |
| `tournament-pressure` | Combines expanded adjacency with stronger collision/challenge pressure. |

Early signal from `--compare-variants --runs 20`: `tournament-pressure` improves
adoption pressure without making adoption automatic, while `lower-adoption`
appears too permissive.
