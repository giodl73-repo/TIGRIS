---
name: A Few Acres of Snow — Tier-C Amendments
slug: few-acres-of-snow-tierC-amendments
game: 0016-few-acres-of-snow
stage: tierC
version: 1.0.0
rubric_version: v2.14
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# Tier-C Amendment — A Few Acres of Snow (game #16)

## Headline

**C8 First-Turn Compression adopts as 24th axis — Pool at 24/25 = 96%.** Stegmaier-primary. TS + AFoS earnings close the dormancy. **First adopted-axis refute in TIGRIS history** (A7 Emergence under Halifax Hammer evidence). 12 earned / 1 refuted / 2 collisions / 0 silent-retire (13th consecutive). **Rubric v2.13 → v2.14.**

## Adoption: C8 First-Turn Compression

- TS first earning (v2.13) + AFoS second earning (v2.14) = 2 games → **ADOPTED** (24th axis).
- Stegmaier-primary. C8 dormant 14 games; then 2-game close-out (TS + AFoS). Matches A4 dormancy-recovery pattern (CoB→Agricola).
- **Pool now at 24/25 = 96%.**

Remaining live: 1 retired (C5). **Only non-adopted axes: the permanently-retired C5.** Pool is effectively **full-adopted-or-retired.**

## First adopted-axis formal refute: A7 Emergence

- **Halifax Hammer** evidence: British has a dominant strategy with >85% win rate at expert level. Strategy space collapses.
- Lacerda refuted A7 in both AFoS playthroughs (game 1 + game 2 of the session).
- Ledger accounting: refutes within the same game-session count as 1 game-of-refute (per v2.x convention). A7 goes 2E/0R/1C/0I → **2E/1R/1C/0I**.
- A7 **remains adopted** (still has 2 earnings; refute threshold for retirement requires ≥ 2 refutes across ≥ 2 distinct games).
- But A7 is now **adopted-contested** — first adopted axis with non-zero refute weight.
- **Protocol question**: should adopted axes accumulate refute weight toward "adopted-contested" status? Current ledger tracks it implicitly. Proposal: formalize "adopted-contested" as a subtype.

### Amendment A-v2.14-01: Adopted-contested subtype

- **Dimension:** rubric meta
- **Status:** **ADOPTED** (v2.14, this session; consumes no rate-limit because it formalizes an existing pattern)
- **Change:** Adopted axes accumulate refute weight. If ≥ 2 refutes across ≥ 2 distinct games, axis status becomes `adopted-contested`. A further refute pushes to `queued-for-de-adoption`. ≥ 3 refutes across ≥ 3 games → `de-adopted` (first-ever retirement of a previously-adopted axis).
- **Precedent:** A7 Emergence at 1 game of refute (2 Halifax Hammer refutes in 1 AFoS session). Needs 1 more refute in a different game to reach `adopted-contested`.
- **Why:** Factory should be able to unadopt axes if repeated structural evidence accumulates. Prevents ledger from being one-way-adopted-only.

## Stake classification

**Earned (12)**: C4 (Vaccarino anchor), C8 (Stegmaier ADOPTION), B1, B4 (Lacerda), D4, B3 (Rosenberg), A3, B5 (Chvátil), A2 (Feld), D2 (K-K), A4, A5-LAST-CALL (Vaccarino).

**Refuted (1 axis, 2 instances)**: A7 Emergence-Replayability (Lacerda, Halifax Hammer structural refute both games).

**Retire-explicit (6)**: C2 (Knizia), C6 (Feld), B6 (Chvátil), D1 (K-K), A6 D3 (Stegmaier). +0.5r each. Preserved.

**Hold-explicit (6 across personas).**

**Silent-retire: 0 (13th consecutive).**

## Rubric version: v2.13 → v2.14

- **Minor bump for 1 adoption** (C8).
- **Minor bump for 1 protocol amendment** (A-v2.14-01: adopted-contested subtype) — this is a ledger formalization, uses the current 2-game-cycle slot.

## Innovations

### I-afos-01 — C8 First-Turn Compression adopts (closes 14-game dormancy)

- **Dimension:** pool dynamics
- **Trigger pattern:** `dormant_axis_canonical_close_out`
- **Source:** games/0015-twilight-struggle/ + games/0016-few-acres-of-snow/
- **Supporting voices:** Stegmaier (primary both games); panel unanimous on Halifax Hammer canonicality
- **Scope:** pool
- **Status:** **ADOPTED as 24th axis (v2.14)**
- **Note:** C8 waited 14 games before canonical case (TS), then 1 game later closed adoption (AFoS). **Two-game dormancy-recovery** matches A4 pattern. **Pool is now 96% adopted (24/25).** The only non-adopted axis is the permanently-retired C5.

### I-afos-02 — First adopted-axis formal refute

- **Dimension:** rubric meta
- **Trigger pattern:** `adopted_axis_refuted`
- **Source:** AFoS Turn 5 (Lacerda) + Game 2 Turn 5 (Lacerda)
- **Supporting voices:** Lacerda (primary); Feld + K-K (collision vote supporting refute strength)
- **Scope:** universal
- **Status:** **ADOPTED log entry + protocol amendment A-v2.14-01**
- **Note:** A7 Emergence-Replayability takes its first formal refute weight across 16 games. Evidence: Halifax Hammer solves the game. The Pool now has a mechanism for adopted-axis de-adoption (v2.14 amendment). A7 remains adopted but at adopted-contested threshold if next-game refute occurs.

### I-afos-03 — Orthogonal-preservation collision resolution

- **Dimension:** argument protocol
- **Trigger pattern:** `collision_orthogonal_preservation`
- **Source:** AFoS Collision 1 (A7 refute vs C4 anchor defend)
- **Supporting voices:** Knizia + Chvátil + Stegmaier (majority-vote) articulated that axes can collide without one dominating; both survive
- **Scope:** universal
- **Status:** observational
- **Note:** Previous collisions resolved with winner/loser. AFoS produced a collision where the resolution preserved BOTH axes as making orthogonal claims about the same game. Resolution rule clarifies: a collision can settle "A wins on its axis; B separately holds/refutes on its axis." Adds expressive depth to the argument protocol.

### I-afos-04 — Tiebreak via snake-order earlier-draft

- **Dimension:** argument protocol
- **Trigger pattern:** `collision_tiebreak_earlier_draft`
- **Source:** AFoS Collision 2 (C8 vs C4 tied 3-3; C4 wins via earlier-draft rule)
- **Supporting voices:** observational
- **Status:** observational
- **Note:** Second instance of the earlier-draft tiebreak rule firing (first: UNFOLD B5↔A7). Rule remains valid and consistent. No amendment needed.

### I-afos-05 — Second non-own anchor (non-Euro Wallace)

- **Dimension:** anchor protocol
- **Trigger pattern:** `non_own_anchor_consecutive`
- **Source:** TS (Gupta/Matthews) → AFoS (Wallace)
- **Supporting voices:** observational
- **Status:** observational
- **Note:** Second consecutive non-own anchor. Both earned their anchor axis. Non-own anchor is now **proven viable across 2 games**. Protocol change unnecessary. Factory can review any designer's game without panel authorship.

## State after 16 games

- **24 adopted axes / 25 Pool (96%)** — up from 23 (92%).
- 1 retired (C5).
- 0 unadopted live axes. **Pool effectively closed.**
- 3 salvages (C4, C3, B2).
- 1 adopted-contested (A7 with 1 refute game; Halifax Hammer structural).
- **8/8 designer-on-own-anchor tally** preserved.
- **13 consecutive games 0% silent-retire.**
- 6 protocol amendments (v2.1, v2.3, v2.4, v2.6, v2.14).
- **160+ total earned stakes.**
- Rubric v2.14.

## Success criteria

| # | Criterion | AFoS |
|---|---|---|
| 1 | ≥ 5 earned | PASS (12) |
| 2 | ≥ 2 on Band B/C | PASS |
| 3 | ≥ 1 clean collision | PASS (2; one orthogonal-preservation, one tiebreak) |
| 4 | ≥ 1 adoption/retirement | **PASS — C8 24th adoption; Pool → 96%** |

4/4 PASS.

## Verdict

A Few Acres of Snow delivered the **fullest-ledger-impact game in TIGRIS history**:
- 24th axis adoption (C8 closes dormancy).
- First adopted-axis formal refute (A7 under Halifax Hammer).
- New protocol amendment (adopted-contested subtype, v2.14).
- Pool effectively closed at 96%.

The factory now has a two-way adoption mechanism (axes can move out of adopted status via accumulated refute weight).

Proceeding to HANDOFF.
