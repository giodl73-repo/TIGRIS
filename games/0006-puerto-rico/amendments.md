---
name: Puerto Rico — Tier-C Amendments
slug: pr-tierC-amendments
game: 0006-puerto-rico
stage: tierC
version: 1.0.0
rubric_version: v2.6
bet_version: parliament
author: TIGRIS (via /tigris-amendment skill)
created: 2026-04-19
updated: 2026-04-19
---

# Tier-C Amendment — Puerto Rico (game #6)

## Stake classification

**Earned (10)**: C1 (Knizia anchor), C3 (Rosenberg), B3 (Vaccarino), C4 (Rosenberg; adopted re-earn), D4 (Rosenberg LAST-CALL; adopted), C7 (K-K LAST-CALL), A2 (Feld LAST-CALL; adopted), A4 (Vaccarino defending LOW), B4 (Lacerda defending LOW, adopted), A3 (Chvátil)

**Refuted (full)**: C5 Anti-Catch-up (Feld self-refute)

**Retire-explicit (4)**: B2 (Chvátil), C5 (Feld — second retire in one game), C2 (Knizia adopted), A5 (Knizia fallback)

**Hold-explicit (6)**: A1, D1, A7, B5, C6, C8

**Silent-retire**: 0 (5th consecutive)

## Key events

### First retirement event in TIGRIS history: C5 Anti-Catch-up Pressure

Cumulative refute weight across games:
- Parliament: 1 full refute (Feld self-refute)
- T&E: weak-defended (neutral)
- FACETS: collision loss (single-moment, neutral)
- Scythe: retire-explicit = 0.5 refute
- Puerto Rico: retire-explicit (0.5) + full self-refute (1) = 1.5

**Total: 3.0 refute weight across 3 games with refute states** (P, Scythe, PR). Threshold: ≥2 refute states across ≥2 games with sufficient weight. **TRIGGER: C5 RETIRED.**

C5 Anti-Catch-up Pressure is the first axis to retire from the TIGRIS Pool. Retirement is forward-only; prior games' stakes on C5 remain at their scoring version.

### Reversal: C3 Scarcity Bite

C3 was queued-for-retirement (1 refute from T&E LAST-CALL fail). Puerto Rico earned it with 3 marks → **2 games of earning evidence across ≥2 games** (Rosenberg's stake + combined with prior weak-defends now count). Actually re-checking: Puerto Rico is the FIRST earning for C3. Not yet 2 earnings. **Queued-for-adoption (pending second game).**

### First C1 Tension Budget earning

5 games of dormancy end. C1 earns 3 defends in Puerto Rico. Queued-for-adoption.

### First B3 Conversion Chain Depth earning

Vaccarino's #4 preference finally earns (first game drafted = Scythe; first game earned = Puerto Rico). Queued-for-adoption.

## Rubric Ledger updates

| Axis | Before PR | After PR |
|---|---|---|
| C1 Tension Budget | 0/0/0 | **1/0/0** (queued-for-adoption, first earning) |
| C3 Scarcity Bite | 0/1/0 queued-for-retirement | **1/1/0** (queued-for-adoption, mixed history) |
| C5 Anti-Catch-up | 0/1/2 queued-for-retirement | **0/3/0** → **RETIRED** |
| B3 Conversion Chain | 0/0/0 | **1/0/0** (queued-for-adoption) |
| A4 Variance | 0/0/0 weak | 0/0/0 (1 defend in PR via low-stake defense; weak-defended) |
| B4 Info-Transparency (adopted) | 2/0/0 adopted | 2/0/0 (re-holds as adopted; 1 weak-defend PR as diagnostic-low) |
| C4 Engine-Garden | 2/1.5/1 adopted | 2/1.5/1 (re-earns; adopted) |
| D4 Late-Game Lock-in | 2/0/0 adopted | 2/0/0 (re-earns via LAST-CALL) |
| A2 Decision Density | 2/0/0 adopted | 2/0/0 (re-earns via LAST-CALL) |
| C7 Action-Menu Clarity | 0/0/0 | **1/0/0** (LAST-CALL earning; queued-for-adoption) |

## Adoption / retirement events

### ADOPTED this session: 0
No axes hit the 2-game-earned threshold this session. C1, C3, B3, C7 all have first-earnings; each needs a second game.

### RETIRED this session: 1
**C5 Anti-Catch-up Pressure.** First retirement event in TIGRIS history.

### Protocol amendment adopted: I-parliament-02 (Scoring Multiplier Dependency)

**3-cluster instances confirmed** (Parliament + Scythe + Puerto Rico). Rate limit consumed for cycle 4.

New axis added to Pool:
- **B6 Scoring Multiplier Dependency** — "Do endgame points depend multiplicatively on choices made across multiple subsystems, forcing balance across them?"
- **Anchor 0**: single-track VP (raw sum, no multipliers)
- **Anchor 5**: one or two multipliers present (e.g., Scythe's popularity band)
- **Anchor 10**: multiple stacking multipliers define the scoring space (Puerto Rico: Residence, Guild Hall, Fortress, etc.)
- **Primary advocate**: Rosenberg (engine-payout); secondary Vaccarino
- **Bands**: B (Euro-specific)

Added to Rosenberg's preferred_axes at rank #7; added to Vaccarino's preferred_axes at rank #9.

Pool now has **25 axes**. 8 personas × 3 drafts = 24; one axis per game goes undrafted (selection pressure returns).

## Rubric version bump

**v2.5 → v2.6.**
- Retirement event (C5)
- Protocol amendment (B6 SMD axis added)
- No new axis adoptions (4 queued-for-adoption await game #7)

## Innovations

### I-pr-01 — C5 first retirement (milestone)

- **Dimension:** ledger dynamics
- **Trigger pattern:** `first_retirement_event`
- **Source:** Puerto Rico cumulative refute
- **Status:** adopted (log entry)
- **Note:** Proves the retirement protocol works. Conservative-but-not-frozen: axes that don't earn across many games and accumulate refute evidence can retire. C5 took 6 games to build sufficient cumulative refute.

### I-pr-02 — Fallback draft validates protocol flexibility

- **Dimension:** draft protocol
- **Trigger pattern:** `persona_draft_fallback`
- **Source:** Knizia drafted A5 outside preferences (all preferences exhausted)
- **Status:** observational
- **Note:** First time a persona drafted outside their preferred_axes list. Knizia drafted A5 in round 3 fallback. He staked 4/4 honestly and retire-explicit at FORCED-ENGAGEMENT. Confirms that the draft protocol can handle exhausted-preferences cases gracefully. No protocol change needed; the fallback mechanism (pick any remaining axis) works.

### I-pr-03 — Adopted axes defending LOW stakes (diagnostic validity)

- **Dimension:** ledger dynamics
- **Trigger pattern:** `adopted_axis_low_stake_diagnostic`
- **Source:** Lacerda B4 at 2/2 (adopted axis, low stake); Vaccarino A4 at 2/2
- **Status:** observational
- **Note:** Adopted axes staked LOW (2/2) in specific games demonstrate the axes measure something real. If axes only ever scored HIGH or retired, they'd be too blunt. Low stakes defended via "the axis is real; it just doesn't fit this game" preserve diagnostic integrity. This is the bet working: axes aren't propaganda, they're measurements.

### I-pr-04 — I-parliament-02 ADOPTED as B6 Scoring Multiplier Dependency

- **Dimension:** rubric axis (new axis added to Pool)
- **Trigger pattern:** `scoring_multiplier_present` (3 cluster instances)
- **Status:** ADOPTED (v2.6 protocol amendment)
- **Note:** Parliament's earned × adopted multiplier + Scythe's popularity band × stars + Puerto Rico's endgame building multipliers = 3 distinct mechanical instances. Pool now 25 axes. Rosenberg primary advocate.

## Success criteria

| # | Criterion | PR |
|---|---|---|
| 1 | ≥ 5 earned | **PASS (10)** |
| 2 | ≥ 2 earned on Band B/C | **PASS (5 — C1, C3, C4, B3, B4)** |
| 3 | ≥ 1 clean collision | **PASS (1 — B1↔C4)** |
| 4 | ≥ 1 axis promoted/retired | **PASS (1 retirement + 1 new B6 axis adopted)** |

**All 4 PASS.** Puerto Rico is the strongest review by-a-different-measure than Dominion or Scythe: most earned stakes (10), first retirement event, first new-axis adoption.

## State after 6 games

- **11 adopted axes** (no new adoptions this session; 4 queued-for-adoption: C1, C3, B3, C7)
- **1 retired axis** (C5) — **first retirement ever**
- **1 new axis in Pool** (B6 SMD)
- **Pool now 25 - 1 retired = 24 active axes** (C5 still in file but marked retired; B6 added)
- **5 protocol amendments** (v2.1, v2.3, v2.3.1, v2.4, v2.6)
- **0% silent-retire** across 5 games under v2.1

## Next cycle (5)

Game #7 will:
- Test B6 Scoring Multiplier Dependency for first draft
- Trigger second earnings for C1, C3, B3, C7 (all queued-for-adoption)
- Possibly first retirement-since-C5 event? C3's queued-for-retirement status cleared by Puerto Rico earning; no other axis close to retirement threshold.

Proceeding to HANDOFF.
