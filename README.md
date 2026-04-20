# TIGRIS

**A board game factory.** Named for Tigris & Euphrates (Knizia, 1997). Sibling to marathon, puzzlehunt, chronicle, rmm, artisan.

Claude Code-driven. Markdown-first. Euro tradition. Produces original game designs **and** reviews of existing games through a single adversarial pipeline. Evolves its own design guidelines as games are played.

## The architectural bet (v2.0 — Parliament)

TIGRIS doesn't produce consensus. It produces an **argument record**. Designer personas draft exclusive axes from a shared 24-axis Pool, plant stakes against the design, then attack/defend/collide during a narrated playthrough. Stakes that survive become evidence; stakes that get refuted become evidence of the other kind. The rubric evolves by play, not by decree.

The first game TIGRIS produces is **Parliament** — a 3–4 player, 45–60 minute Euro whose mechanics *are* the factory's pipeline. Playing Parliament teaches TIGRIS; reviewing Parliament with TIGRIS tests whether the factory can recognize the rubric it's made of.

## What's here

- **`docs/specs/`**
  - `2026-04-19-tigris-v2.0-design.md` — source of truth (Parliament architecture)
  - `2026-04-19-tigris-design.md` — v1.0 (superseded; kept for history)
  - `reviews/2026-04-19-tigris-design/` — the v1.0 panel review that triggered the v2.0 bet
- **`personas/`** — 7 designers + 5 player lenses + axis-pool + rubric + innovation log
- **`games/`** — one numbered dir per game; `0001-parliament/` is anchor #1
- **`.claude/skills/`** — Claude Code skills executing the pipeline (`tigris-panel`, `tigris-concept`, `tigris-design`, `tigris-amendment`, `tigris-ideate`). Must live here for CC discovery.
- **`CLAUDE.md`** — house rules, frontmatter contract, forbidden vocabulary

## The pipeline (v2.0 Parliament-shape)

```
CONCEPT → DESIGN → TIER-A [STAKES] → GATE → TIER-B [ARGUMENT] → TIER-C [AMENDMENT] → HANDOFF
```

Three cognitively distinct phases (not three compute budgets for the same thing):

- **STAKES** — personas *draft* axes from the Pool (snake order, exclusive). Each plants stake scores against the design.
- **ARGUMENT** — narrated playthrough. Stakes are attacked, defended, or *collide* on semantically-adjacent axes.
- **AMENDMENT** — deterministic: earned stakes promote axes to `adopted`; refuted stakes retire them. Rubric evolves. Forward-only.

## Status

- 2026-04-19 S01 — scaffolding; v1.0 spec panel review produced v2.0 restructure; Parliament scaffolded; ready for the anchor's TIER-A.
- See `TRACKER.md` for current pipeline state.

## v1.0 → v2.0 changelog

The v1.0 spec's own panel review (7 designer voices) flagged a 5-voice cluster on persona-rubric orthogonality: personas chorused instead of arguing. v2.0 replaces the shared 8-axis rubric with a 24-axis Pool and disjoint drafting, producing an adversarial review shape. See `personas/playtest-innovations.md` for the full amendment record.
