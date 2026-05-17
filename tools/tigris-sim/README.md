# tigris-sim

Seeded board-game simulation pilot for TIGRIS.

The first adapter targets `games/0001-parliament/` and models the argument
loop at a coarse systems level: axis draft, stakes, adjacency collisions,
defense/refutation marks, adoption pressure, and final scores. The second
adapter targets `parlor/games/0001-upstage/` and models party-game robustness:
trigger fires, committed upstages, DOUBLE moments, false upstages, pile-on
chaos, and score spread. TIGRIS owns game-specific policy; RALLY supplies
deterministic run, actor trace, metric, validation, and comparison primitives.

## Commands

```powershell
cargo test --quiet
cargo run --quiet -- --seed parliament-smoke
cargo run --quiet -- --seed parliament-smoke --runs 20 --players 4
cargo run --quiet -- --seed parliament-smoke --compare-variants --runs 20 --players 4
cargo run --quiet -- --game upstage --seed upstage-smoke --players 8
cargo run --quiet -- --game upstage --seed upstage-smoke --players 8 --runs 24
cargo run --quiet -- --game upstage --seed upstage-smoke --players 8 --compare-variants --runs 24
cargo run --quiet --bin tigris-muddle
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
- UPSTAGE trigger pressure, DOUBLE rate, false-upstage/commitment risk,
  eight-player chaos, score spread, and RALLY comparison-report status.
- MUDDLE solo-play table loop with deterministic Parliament AI opponent,
  command transcript, save/resume, recent log, inventory, and score panels.
- RALLY tabletop primitives for AI-opponent turn order, score tracking, and
  pressure/token bookkeeping.

## MUDDLE AI opponent host

`tigris-muddle` mounts `parliament_ai_muddle_host()` through the shared MUDDLE
runner. The first AI slice is deterministic on purpose: the opponent drafts
axes, builds pressure, counter-drafts when pressure is armed, and can be
challenged by the human chair. Shared table bookkeeping uses RALLY's
`TurnOrder`, `ScoreTrack`, and `TokenPool`; TIGRIS keeps Parliament policy local.

```powershell
@("go board", "draft axis", "place tiger", "end turn", "challenge ai", "quit") |
  cargo run --quiet --bin tigris-muddle -- --save target\tigris-ai.muddle --transcript target\tigris-ai.txt
@("status", "quit") |
  cargo run --quiet --bin tigris-muddle -- --load target\tigris-ai.muddle
```

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

## UPSTAGE variants

| Variant | Purpose |
|---|---|
| `baseline` | Current UPSTAGE trigger and commitment model. |
| `warmup-scene` | Tests whether a rehearsal scene improves commitment. |
| `clearer-triggers` | Tests more legible physical trigger cards. |
| `double-spotlight` | Tests whether explicit reward for DOUBLE moments raises memorable co-play. |
| `eight-player-chain-limit` | Tests a high-player-count guardrail against pile-on chaos. |

Early signal from `--game upstage --compare-variants --runs 24 --players 8`:
`double-spotlight` raises DOUBLE moments while improving most comparison metrics,
and `eight-player-chain-limit` removes pile-on chaos but trades away some raw
upstage pressure.
