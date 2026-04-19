---
name: TIGRIS Playtest Rubric
slug: playtest-rubric
version: 2.0.0
rubric_version: v2.0
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
supersedes: v1.0 (8 flat axes, weighted aggregate per persona)
---

# TIGRIS Playtest Rubric — v2.0 (Parliament shape)

**v2.0 is not an amendment of v1.0 — it is a new rubric under a new architectural bet.** v1.0 scores remain locked at v1.0 and are *not* re-scored here. The Parliament bet (see `docs/specs/2026-04-19-tigris-v2.0-design.md`) replaces flat-axis consensus scoring with an adversarial stake-and-argument procedure.

For axis definitions see `personas/axis-pool.md` — the pool of 24 axes from which personas draft.

## The rubric is a game

A review is produced by playing a three-phase game between the designer-personas (and optionally player-lenses) seated at the table. The phases are cognitively distinct:

1. **STAKES (TIER-A)** — each persona *drafts* axes from the Pool and plants stake scores against the design.
2. **ARGUMENT (TIER-B)** — stakes are attacked, defended, or collide during a narrated playthrough.
3. **AMENDMENT (TIER-C)** — the mechanical pass that promotes earned stakes and retires refuted ones, updating the Pool.

There is no weighted aggregate. There is no consensus score. The output is the record of the argument.

## Stake states

Each stake has one of four final states after Argument:

| State | Definition | Effect at Amendment |
|---|---|---|
| **Earned** | Defended at ≥ 2 moments during Argument; no unrefuted refutation. | Contributes toward axis adoption (≥ 2 earned across ≥ 2 games → `adopted`). |
| **Contested** | Defended *and* refuted in different moments. | Recorded in ledger; does not count toward adoption or retirement. |
| **Refuted** | Refuted at ≥ 2 moments; no successful defense. | Contributes toward axis retirement (≥ 2 refuted across ≥ 2 games → `retired`). |
| **Ignored** | No attacks, no defenses, no collisions. | The silent failure mode — a stake nobody cared about. Counts against the persona (3+ ignored stakes in a game → consider retiring this persona from future drafts). |

Collisions (two personas' stakes on semantically-adjacent axes resolved simultaneously) produce *both* players a defense credit on their stake plus a shared "collision marker" that is the strongest signal of live argument in the record.

## Draft protocol

See `personas/axis-pool.md` for full draft rules. Summary:

- 7 designer personas + optional 5 player lenses at the table.
- 3 rounds × 7+ personas = ≥ 21 drafts per game.
- Snake order; anchor-persona (designer-declared) drafts last round-1, first round-2.
- Each persona drafts **3 axes per game**.
- A retired axis costs 2 draft-slots (a persona claiming a retired axis is staking prestige on the argument having changed).
- No axis drafted twice in the same game.

## Stake scoring protocol

Once drafted, a persona assigns their axis a score 0–10 at each target player count, with a one-sentence justification naming a specific rule or mechanic. **The score is the stake.** No weighted aggregate. No per-cell verdict at this stage — the verdict comes from Argument.

Anchor descriptors (0/5/10) for each axis are defined in `personas/axis-pool.md`.

## Argument protocol

At pre-declared moments during narrated play (turn 1, turn 5, mid-game, late-game, end — plus any persona-flagged moment):

- **Hold** — persona affirms their stake still applies; no opposed action.
- **Attack** — persona argues another persona's stake is weak at this moment. Requires citing what their own lens sees that the opposing stake misses. Attacked persona must defend (cite rule/mechanic) or concede.
- **Defend** — persona fortifies their own stake by citing evidence from current play state.
- **Collide** — two personas with semantically-adjacent stakes at this moment resolve simultaneously: whichever persona's stake better matches the moment earns a collision credit; the other takes a single-moment refutation.

Attacks and defenses are written into `playtests/PT<NN>-argument.md` with specific turn-numbers and rule-citations. Vagueness is banned per `personas/forbidden-words.md`.

## Amendment protocol (deterministic)

At TIER-C, for each stake in the game's record:

1. Classify final state (earned / contested / refuted / ignored).
2. Update the Rubric Ledger in `personas/axis-pool.md` with per-game state counts.
3. Trigger adoption / retirement:
   - **Adopt** if ≥ 2 earned states across ≥ 2 different games → mark `adopted`, bump rubric to v2.1+.
   - **Retire** if ≥ 2 refuted states across ≥ 2 different games → mark `retired`.
4. Log all new innovations (see §7 of v2.0 spec + `personas/playtest-innovations.md`).

No user approval required; the argument record is the evidence. User override is possible but explicit.

## GATE thresholds (before TIER-B)

GATE runs between TIER-A (Stakes) and TIER-B (Argument). Pass requires:

1. **Anchor stake viable.** The designer-declared anchor-persona's anchor-axis has not been pre-staked better by another persona on the same target count.
2. **Draft coverage.** The drafted axes collectively cover ≥ 3 of the 4 Bands (A/B/C/D).
3. **≥ 1 collision candidate.** At least two personas' drafted axes are semantically adjacent enough that argument can produce a collision. A fully orthogonal draft has nothing to argue about.

Failed GATE forces re-draft (if axis scarcity caused the problem) or design revision (if the design is too narrow to admit collision).

## Success criteria for a panel review (per game)

A Parliament review succeeds when:

- ≥ 5 stakes earn `earned` status across the full panel.
- ≥ 2 of those earned stakes are on Band B (Euro-specific) or Band C (persona-signature) axes — i.e., the personas are distinguishable.
- ≥ 1 collision resolves cleanly (not into mutual ignored-state).
- The amendment pass promotes ≥ 1 axis to `adopted` OR retires ≥ 1 axis.

If fewer than 5 stakes earn, the rubric is too consensus-seeking (personas aren't arguing). If no Band-B/C stakes earn, the personas are being read as interchangeable. If no collisions resolve, the argument isn't happening. If no promotion/retirement, the rubric isn't evolving. Each failure is a v2.1 blocker.

## Forward-only versioning

Preserved from v1.0 unchanged.

- v2.0 → v2.1 bump occurs the first time any axis is `adopted` or `retired`.
- v2.x scores lock to their scoring version. No retro-adjustment.
- A new architectural bet (v3.0) would similarly lock v2.x scores — a *bet change* is a larger event than an *amendment*.

## Changelog

- **v2.0.0** — 2026-04-19 — Replaces v1.0 rubric with Parliament-shape (stakes, argument, amendment). 24-axis Pool in `axis-pool.md`. v1.0 scores remain at v1.0; no retro-scoring.
- **v1.0** — superseded. See `docs/specs/2026-04-19-tigris-design.md` and `docs/specs/reviews/2026-04-19-tigris-design/` for the v1.0 panel that produced this restructure.
