---
name: TIGRIS Playtest Rubric
slug: playtest-rubric
version: 2.4.0
rubric_version: v2.4
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
supersedes: v2.3 (added Vaccarino persona; v2.3.1 adjacency chart patch; panel now 8 designers)
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
- **Collide** — two personas with semantically-adjacent stakes at this moment resolve simultaneously. Two collision outcomes (v2.20):
  - **Contested-Resolution (CR)** — default. Whichever stake better characterizes the moment earns a collision credit (+1 defend); the other takes a single-moment refutation (+1 refute).
  - **Orthogonal-Preservation (OP)** — named outcome. If both stakes address genuinely different analytical registers of the same moment (neither subsumes the other), either persona may propose OP with a one-sentence rationale naming the registers. The panel adjudicates whether the rationale names orthogonal registers; the proposing persona cannot self-certify (simple majority vote resolves adequacy disputes). If ≥ 5-of-8 present vote for OP → both stakes earn a defend credit (+1 each; no refutation). If ≥ 7-of-8 → Decisive OP, counts as 2 collision resolutions toward success criterion §3 (but cannot satisfy §3 as the sole collision — see below). If < majority for OP → collision resolves as CR normally. In the argument file, record as `COLLISION (OP): Persona-A Axis-X ↔ Persona-B Axis-Y / Vote: N-M / Outcome: both earn defend.`

Attacks and defenses are written into `playtests/PT<NN>-argument.md` with specific turn-numbers and rule-citations. Vagueness is banned per `personas/forbidden-words.md`.

### FORCED-ENGAGEMENT micro-phase (v2.1)

**Inserted between the penultimate and final moment-anchor of every Argument phase.** Addresses the ignored-stake problem surfaced by Parliament game #1 (I-parliament-01: 48% ignore rate). No stake may reach end-of-game with zero engagement; each persona must resolve still-silent stakes before the final anchor.

For each drafted stake that has 0 marks (0 defends + 0 refutes + 0 collisions) at the end of the penultimate anchor, the owning persona must declare one of:

- **HOLD-EXPLICIT** — stake still applies despite no in-game evidence. Persona cites what they expected to see and why it didn't materialize. Costs 1 stake token (prestige reservation). At endgame, hold-explicit stakes count for 0 points but do **not** trigger silence penalty.
- **RETIRE-EXPLICIT** — stake was wrong or poorly-timed for this subject/game. Persona names the miscalibration. Refunds 1 stake token. Axis accumulates 0.5 refute-marks (half weight toward the retirement threshold).
- **LAST-CALL-STAKE** — persona re-stakes on the final round at **double token weight** (minimum 2 tokens). The axis gets one more round to earn. If it fails to earn at final-round, the double tokens are lost and 1 refute-mark is added.

Any stake still at 0-marks after FORCED-ENGAGEMENT (i.e., the persona declined to make any declaration) becomes **silent-retire** at endgame:

- **Silent-retire penalty:** −3 points per silent-retire axis (replacing v2.0's 0 penalty for ignored).
- **3+ silent-retires in a single game** → persona is flagged as a retirement candidate. At 3+ flags across games, the persona retires from future drafts (superseded by a new persona per §5.3 of spec).

**Expected effect:** ignore rate drops from ~48% (Parliament game #1) to below 20% in future games. Personas who cannot engage their own drafts have less place on the panel.

### Stake state updates (v2.1)

The four v2.0 states are joined by two new v2.1 states:

| State | Definition | Amendment contribution |
|---|---|---|
| earned | Defended ≥ 2; no unrefuted refute | +1 toward adoption (≥ 2 earned across ≥ 2 games) |
| contested | Both defends and refutes across moments | neutral |
| refuted | Refuted ≥ 2; no successful defense | +1 toward retirement |
| **hold-explicit** (v2.1) | FORCED-ENGAGEMENT hold; no play evidence | neutral; persona paid prestige token |
| **retire-explicit** (v2.1) | FORCED-ENGAGEMENT retire; persona concedes | +0.5 toward retirement |
| **silent-retire** (v2.1) | Ignored through FORCED-ENGAGEMENT | +1 toward retirement; −3 points; persona-retirement flag |
| ignored | 0 marks (pre-FORCED-ENGAGEMENT only — v2.1 eliminates this state for post-engagement stakes) | counts toward silent-retire if not addressed |

`ignored` as a final state is no longer possible in v2.1 — every stake has a classification by end-of-game.

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
4. **Anchor adjacency partner present (v2.3 — I-parliament-03).** The anchor axis has at least one semantically-adjacent axis drafted by a DIFFERENT persona than the anchor. Without a cross-player partner, the anchor's collision potential is zero, and its earning relies entirely on direct defenses. If the anchor persona happens to draft both the anchor axis AND its adjacency partner (same-player situation), GATE fails — unless the anchor has no other drafted adjacency option at all (explicit exemption). Applies forward-only; prior games locked at their version.

Failed GATE forces re-draft (if axis scarcity caused the problem) or design revision (if the design is too narrow to admit collision).

## Success criteria for a panel review (per game)

A Parliament review succeeds when:

- ≥ 5 stakes earn `earned` status across the full panel.
- ≥ 2 of those earned stakes are on Band B (Euro-specific) or Band C (persona-signature) axes — i.e., the personas are distinguishable.
- ≥ 1 collision resolves cleanly (not into mutual ignored-state). Both CR and OP outcomes satisfy this criterion. A Decisive OP (≥ 7-of-8) counts as 2 collision resolutions, but at least one CR or standard-OP collision must still occur independently — Decisive OP alone does not satisfy criterion §3.
- The amendment pass promotes ≥ 1 axis to `adopted` OR retires ≥ 1 axis.

If fewer than 5 stakes earn, the rubric is too consensus-seeking (personas aren't arguing). If no Band-B/C stakes earn, the personas are being read as interchangeable. If no collisions resolve, the argument isn't happening. If no promotion/retirement, the rubric isn't evolving. Each failure is a v2.1 blocker.

## Forward-only versioning

Preserved from v1.0 unchanged.

- v2.0 → v2.1 bump occurs the first time any axis is `adopted` or `retired`.
- v2.x scores lock to their scoring version. No retro-adjustment.
- A new architectural bet (v3.0) would similarly lock v2.x scores — a *bet change* is a larger event than an *amendment*.

## Changelog

- **v2.4.0** — 2026-04-19 — Adopted I-facets-03 + I-dominion-04 (Vaccarino persona). Panel now 8 designers. A4/A5/B3 now advocated. Pre-Scythe (cycle 3 rate-limit consumed).
- **v2.3.1** — 2026-04-19 — Adjacency chart patch (I-te-01 + I-facets-01 + I-facets-02). Canonical chart at `personas/adjacency-chart.md` with 17 adjacencies.
- **v2.3.0** — 2026-04-19 — Adopted I-parliament-03 (anchor-adjacency partner GATE check). Bumped to v2.3 consolidating 5 axis adoptions from Dominion session (B5, A6, A2, B1, D4) + protocol amendment. Dominion provided positive counter-example showing GATE passes naturally when personas coordinate. Rate-limit consumed for cycle 2.
- **v2.1.0** — 2026-04-19 — Adopted I-parliament-01 (FORCED-ENGAGEMENT micro-phase). New stake states: hold-explicit, retire-explicit, silent-retire. `ignored` no longer a terminal state. Source: Parliament game #1 surfaced 48% ignore rate. Rate-limited adoption per innovation log protocol (1 per 2-game cycle).
- **v2.0.0** — 2026-04-19 — Replaces v1.0 rubric with Parliament-shape (stakes, argument, amendment). 24-axis Pool in `axis-pool.md`. v1.0 scores remain at v1.0; no retro-scoring.
- **v1.0** — superseded. See `docs/specs/2026-04-19-tigris-design.md` and `docs/specs/reviews/2026-04-19-tigris-design/` for the v1.0 panel that produced this restructure.
