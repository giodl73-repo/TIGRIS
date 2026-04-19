---
name: Tigris & Euphrates — Tier-C Amendments
slug: te-tierC-amendments
game: 0002-tigris-and-euphrates
stage: tierC
version: 1.0.0
rubric_version: v2.2
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# Tier-C Amendment — T&E (game #2)

## Reviewer-stake ledger update (game #2)

| Axis | Game #2 state | Marks (defend / refute / collision-win) | Cumulative (both games) |
|---|---|---|---|
| C2 Minimum-Score Shape | **earned** (LAST-CALL win) | 1d / 0r / 0cw (+ LAST-CALL bonus) | Parliament earned + T&E earned → **ADOPTED** |
| B4 Information-Transparency-Cost | **earned** (LAST-CALL win) | 1d / 0r / 0cw (+ LAST-CALL bonus) | Parliament earned + T&E earned → **ADOPTED** |
| B1 System Gearing | **earned** | 2d / 0r / 1cw | T&E only earned → queued-for-adoption (1 more earning triggers) |
| B2 Catastrophe Pressure | **earned** | 2d / 0r / 1cw | T&E only earned → queued-for-adoption |
| D2 Spatial-Interaction Presence | **earned** | 2d / 0r / 1cw | T&E only earned → queued-for-adoption |
| C5 Anti-Catch-up Pressure | weak-defended | 1d / 0r / 0cw | Parliament refuted + T&E weak-defended → conflict; stays queued-for-retirement with contested note |
| C4 Engine-Garden Dependency | contested | 1d / 0r / 1 collision loss | Parliament refuted + T&E contested → stays queued-for-retirement (no second full refute) |
| C3 Scarcity Bite | refuted | 0d / 1r / 0cw | Parliament ignored + T&E refuted → 1 full refute cumulative |
| A3 Interaction | contested | 1d / 0r / 1 collision loss | Parliament ignored + T&E contested |
| D4 Late-Game Lock-in Point | weak-defended | 1d / 0r / 0cw | Parliament ignored + T&E weak-defended |
| C6 Point-Salad Incommensurability | retire-explicit | 0d / 0.5r (v2.1) / 0cw | Parliament ignored + T&E retire-explicit → 0.5 refute cumulative |
| B5 Architectural Novelty | retire-explicit | 0d / 0.5r (v2.1) / 0cw | Parliament earned + T&E retire-explicit → contested cumulative (earned and refuted in different games) |
| C8 First-Turn Compression | retire-explicit | 0d / 0.5r (v2.1) / 0cw | Parliament ignored + T&E retire-explicit → 0.5 refute cumulative |
| D3 Count-Robustness | weak-defended | 1d / 0r / 0cw | Parliament weak + T&E weak → no change |
| C1 Tension Budget | weak-defended (hold-explicit + implicit) | 1d / 0r / 0cw | Parliament ignored + T&E weak-defended |
| A1 Elegance | hold-explicit | 0 game-action marks | Parliament contested + T&E hold-explicit |
| A2 Decision Density | hold-explicit | 0 game-action marks | Parliament ignored + T&E hold-explicit |
| A7 Emergence-Replayability | hold-explicit | 0 game-action marks | Parliament contested + T&E hold-explicit |
| C7 Action-Menu Clarity | hold-explicit | 0 game-action marks | Parliament weak-defended + T&E hold-explicit |
| D1 Family-to-Expert Scaling | hold-explicit | 0 game-action marks | Parliament ignored + T&E hold-explicit |
| A6 Teachability | hold-explicit | 0 game-action marks | Parliament contested + T&E hold-explicit |

**Silent-retire count: 0.** v2.1 FORCED-ENGAGEMENT protocol achieved its design target.

## Promotion / retirement triggered

### ADOPTED (this session)

1. **C2 Minimum-Score Shape** — Knizia's anchor axis. Earned in Parliament (3 defends) + earned in T&E (1 defend via LAST-CALL). Meets threshold: ≥ 2 earned states across ≥ 2 games. **First adopted axis in v2.x history.**
2. **B4 Information-Transparency-Cost** — Lacerda-primary. Earned in Parliament (1d + 1cw = 2 marks) + earned in T&E (1 defend via LAST-CALL). Threshold met. **Second adopted axis.**

### RETIRED (this session)

**Zero.** No axis hit ≥ 2 full refutes across ≥ 2 games.

- C3 Scarcity Bite: 1 refute cumulative (T&E only). Queued-for-retirement activating; needs a third game with refutation to trigger.
- C4 Engine-Garden: 1 refute (Parliament) + 1 contested (T&E). Doesn't cumulate.
- C5 Anti-Catch-up: 1 refute (Parliament) + 1 weak-defended (T&E). Contested cross-game.
- C6, C8: each 0.5 refute cumulative from retire-explicit. Half-weight; 2 more retire-explicits would cumulate to 1 full refute.

**The adoption/retirement asymmetry in v2.1:** adoptions are firing, retirements are not (yet). This is a design feature — retirements are structurally harder (axes defend themselves in second games, like C4 and C5 did partially in T&E). The rubric should err on the side of keeping axes rather than retiring them prematurely.

### Queued-for-adoption (needs one more earning to trigger)

- **B1 System Gearing** (Lacerda primary)
- **B2 Catastrophe Pressure** (Chvátil primary)
- **D2 Spatial-Interaction Presence** (K-K primary)

Game #3 will likely trigger these three if the anchor game has any Euro-rich system.

### Queued-for-retirement (needs one more refutation)

- C3 Scarcity Bite (1 refute cumulative)
- C4 Engine-Garden Dependency (queued from Parliament, not refuted in T&E)
- C5 Anti-Catch-up Pressure (queued from Parliament, contested in T&E)

## Rubric version bump: v2.1 → v2.2

v2.2 marks the first ADOPTION event in TIGRIS history. The rubric's permanent axis set now contains 2 adopted axes:
- C2 Minimum-Score Shape
- B4 Information-Transparency-Cost

Adopted axes continue to be draftable but no longer require 2 draft-slots (they're canonical). Their scores from this point forward contribute to calibration data, not to the adoption/retirement machine.

The forward-only versioning rule is preserved: Parliament scores remain at v2.0.1; T&E scores lock at v2.2. No retro-scoring.

## Innovations surfaced in this game

### I-te-01 — New adjacency candidate: Catastrophe Pressure ↔ Anti-Catch-up Pressure

- **Dimension:** adjacency chart
- **Trigger pattern:** `new_adjacency_surfaced`
- **Source:** Turn 5 of T&E 4p session
- **Supporting voices:** Chvátil, Feld (both stakes fired on the same moment)
- **Scope:** universal
- **Status:** candidate
- **Note:** B2 and C5 fired on the same turn-5 catastrophe moment. Not currently on the adjacency chart (Parliament design.md §9). Proposal: add B2 ↔ C5 adjacency for future games. Low-risk amendment.

### I-te-02 — Stable preferences produce identical drafts

- **Dimension:** draft protocol
- **Trigger pattern:** `identical_drafts_across_games`
- **Source:** Parliament draft + T&E draft — identical axis selections by all 7 personas
- **Supporting voices:** (implicit — no one explicitly flagged this)
- **Scope:** pattern-across-games
- **Status:** candidate (observational, not yet proposing a change)
- **Note:** Two consecutive games with the same seated personas produced identical 21-axis drafts because nobody's preferences were sniped. This is consistent with the system's design (preferences drive drafting) but raises a question: if personas always draft the same axes regardless of the game being reviewed, are they reviewing the game or just their own preferences? Candidate for future amendment: shuffle draft order per game OR adjust preferences based on game context (e.g., "for party games, Interactionist drafts higher on Variance").

### I-parliament-03 cluster confirmation (anchor adjacency)

- **Status update:** candidate → **proposed-amendment** (2026-04-19) — second instance logged.
- **Note:** T&E showed the same `anchor_axis_without_collision_partner` pattern. Knizia's C2 again lacked B3 Conversion Chain Depth as a drafted collision partner. The anchor still earned via LAST-CALL, but the axis relies on LAST-CALL rather than collision-evidence. Now has 2 instances (Parliament + T&E) → clusters. Rate limit (1 adoption per 2-game cycle) blocked this cycle; scheduled for next cycle.

### I-parliament-02 cluster status (Scoring Multiplier Dependency)

- **Status:** still candidate (1 instance only; T&E didn't produce scoring-multiplier evidence since T&E's scoring is min-score not multiplicative).
- **Note:** Parliament's scoring multiplier is the canonical instance; T&E doesn't have a similar mechanism. Awaiting a third game that uses multiplier scoring (Scythe's end-game multipliers, Wingspan's bonus cards) to cluster.

### I-te-03 — FORCED-ENGAGEMENT fix validated

- **Dimension:** protocol validation
- **Trigger pattern:** `protocol_amendment_validated`
- **Source:** T&E vs Parliament comparison
- **Supporting voices:** all seven (no one used silent-retire)
- **Scope:** universal
- **Status:** adopted (log entry)
- **Note:** Parliament game #1 had 48% ignore rate. T&E game #2 had 0% silent-retire rate. FORCED-ENGAGEMENT micro-phase from v2.1 works as designed. Ratifies I-parliament-01's adoption.

## Success criteria outcome

| # | Criterion | Result | Note |
|---|---|---|---|
| 1 | ≥ 5 earned stakes | **PASS (5)** | Fixes Parliament's miss; v2.1 fix validated |
| 2 | ≥ 2 earned on Band B or C | **PASS (4)** | C2, B1, B2, B4 |
| 3 | ≥ 1 clean collision | **PASS (2)** | A3/D2, B1/C4 |
| 4 | ≥ 1 axis promoted OR retired | **PASS (2 adoptions)** | C2, B4 adopted |

**All 4 criteria PASS.** Clean anchor review. The bet continues to validate.

## Verdict on v2.1

- v2.1's FORCED-ENGAGEMENT fix eliminated the 48% ignore rate (0% silent-retire in T&E).
- Retirement protocol is appropriately conservative — no axis retired despite Parliament's first-refuted evidence on C4/C5, because T&E gave them a second look (contested/weak-defended, not refuted).
- Adoption protocol fires cleanly on ≥ 2 earnings across ≥ 2 games — 2 axes adopted (C2, B4), 3 more queued.
- The anchor-adjacency problem (I-parliament-03) is now clustering. Next cycle should adopt its fix.

The rubric evolves by play, not by decree. First adoptions have occurred.

Proceeding to SUMMARY + HANDOFF.
