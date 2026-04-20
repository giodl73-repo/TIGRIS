---
name: TIGRIS Ideas Directory
slug: ideas-readme
version: 1.1.0
rubric_version: v2.15
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# ideas/ — TIGRIS Ideate-Hopper

Source-of-ideas system for TIGRIS original designs. Per spec `docs/specs/2026-04-19-tigris-ideate-hopper-design.md` v1.1.

## Files

| File | Kind | Maintained by | Min size for skill |
|---|---|---|---:|
| `primitives.md` | hand-curated | human | 10 entries |
| `mashups.md` | hand-curated | human | 5 entries |
| `gaps.md` | hand-curated | human (periodic refresh) | 1 section |
| `frustrations.md` | hand-curated | human | 0 (may be empty) |
| `hopper.md` | generated + promoted | `/tigris-ideate` + human | n/a |

At least one pool must meet its minimum for `/tigris-ideate` to run.

## Usage

```
/tigris-ideate                          # 3 candidates, all sources
/tigris-ideate --count 5                # 5 candidates
/tigris-ideate --sources mashup,gap     # only these sources
/tigris-ideate --format full            # concept.md-shaped stub
/tigris-ideate --theme nature           # constrain to a tag
```

Candidates land in `hopper.md` with status `fresh`. Promote by editing status to `promoted`. Consume with `/tigris-concept HOP-NNN` — the concept skill seeds `games/NNNN-<slug>/concept.md` and marks the entry consumed.

## Lifecycle

`fresh → promoted | retired` (manual)
`promoted → consumed` (automatic via `/tigris-concept HOP-NNN`)
`promoted → retired` (manual)
`consumed` (terminal)

## Write discipline

`/tigris-ideate` writes ONLY to `hopper.md`. Pools are stable knowledge base. `/tigris-concept HOP-NNN` writes to `games/NNNN-<slug>/` and marks the consumed entry in `hopper.md` only.

## See also

- Spec: `docs/specs/2026-04-19-tigris-ideate-hopper-design.md`
- Parliament review: `docs/specs/reviews/2026-04-19-ideate-hopper/review.md`
