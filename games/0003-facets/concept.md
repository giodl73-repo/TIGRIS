---
name: FACETS — Concept
slug: facets-concept
game: 0003-facets
stage: concept
version: 1.0.0
rubric_version: v2.2
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
anchor_persona: feld
anchor_axis: point-salad-incommensurability
---

# FACETS — Concept

## Premise

FACETS is a 2–4 player, 15–20 minute abstract strategy game about mining six incommensurable kinds of beauty. Each jewel type rewards the player under a **different scoring function**: rubies sum linearly, emeralds cap at five, sapphires reward pairs, topazes reward triples, diamonds are wild, pearls punish excess.

The entire design pressure is on the player making peace with six maths at once. A player who chases only rubies wins ten points. A player who chases only sapphires with perfect pair-ing wins fifteen. Neither wins the game. Winning requires recognizing which types the bag is giving you, which types you've already committed to, and when to stop.

FACETS teaches in eight minutes and plays in twenty. It's designed to be the antidote to Parliament's weight — where Parliament leans into complexity to prove the factory can produce a serious Euro, FACETS leans into restraint to prove the factory can produce an abstract that passes the family-tier teach test. It's also designed as a **direct test of Feld's C6 Point-Salad Incommensurability axis**, which Feld himself retired-explicitly against T&E. If FACETS earns C6, the axis rehabilitates.

## Players and length

- **Players:** 2–4 (designed for 3–4; 2p works as a variant).
- **Length:** 15–20 min (first play 25 min; experienced play 15 min).
- **Weight:** ~1.5/5.

## Anchor mechanic

**Shared jewel market with type-differentiated scoring functions.** Each turn, take one jewel from a 5-slot face-up market (or pass). The market refills from an opaque bag. The game ends when the bag empties and the market falls below 5. Final VP is the sum of six scoring functions applied to your collection — one per type.

Scoring at a glance:
- **Ruby** (straight): +1 each
- **Emerald** (bounded): min(count, 5) × 2 — capped
- **Sapphire** (pairs): floor(count/2) × 3 — pairs only
- **Topaz** (triples): floor(count/3) × 5 — triples only
- **Diamond** (wild): assign at endgame to any other type; scored under that type's function
- **Pearl** (singleton): +3 if exactly 1; −2 per Pearl beyond the first

No two jewels score under the same math. Specialization in any one type caps at ~15 points. Winning requires balance across multiple types.

## Artifact-as-story

The **jewel bag** is the centerpiece. 60 physical tokens (10 of each type) inside an opaque drawstring bag. Players reach in blind to draw market replacements. The bag's composition is known (10 of each type); its sequence is not. The tactile physicality of the bag drives the game — players develop bag-intuition over plays.

In legacy terms, FACETS's bag is *ephemeral-within-game* (reset each session). Contrast Parliament's persistent Axis Pool deck and T&E's emergent map.

## Inspiration / lineage

- **Reiner Knizia — *Ingenious* (2004)** — six hex-tile colors with minimum-color scoring. FACETS borrows the "multiple incommensurable types" move but **changes the scoring operator per type** (Ingenious uses minimum across types; FACETS uses six distinct functions).
- **Reiner Knizia — *Poison* / *Lost Cities scoring*** — Pearl's punish-duplicates mechanic is a direct port.
- **Stefan Feld — *Castles of Burgundy*** — the "many paths to VP, none substitutable" design ethos.
- **Marc André — *Splendor* (2014)** — shared market mechanic with take-one-or-reserve.
- **Sébastien Pauchon — *Jaipur* (2009)** — market-draft with set-scoring asymmetries.

Honest: the core novelty claim is **different scoring function per type within one game** — every type of jewel tells the player "the math for me is different, and you must rebalance." I don't know a tabletop game that commits to this as the entire design. Splendor has one scoring function (VP). Ingenious has one function (min). FACETS breaks this. Whether that's enough for an earned Architectural Novelty stake is for Chvátil's panel to decide.

## Target review in the TIGRIS pipeline

Predicted earnings (stakes likely to earn in FACETS's panel):
- **C6 Point-Salad Incommensurability** (Feld anchor) — the game's structural bet. If this doesn't earn, the design failed.
- **A6 Teachability** — target is 8-minute teach, well under Kramer-Kiesling's family-tier threshold.
- **C8 First-Turn Compression** — Turn 1 every player takes or passes; genuinely meaningful.
- **A2 Decision Density** — every turn is a 6-type evaluation.
- **A4 Variance Calibration** — bag-draw variance; finally gets drafted (undrafted in Parliament and T&E).

Predicted refutations (stakes likely to fail in FACETS's panel):
- **B1 System Gearing** — no subsystem interdependence; flat.
- **D2 Spatial-Interaction Presence** — no board.
- **B5 Architectural Novelty** — borrowed heavily from Ingenious/Splendor/Jaipur; novelty claim rests on the per-type scoring-function move.
- **A3 Interaction** — weak; only interaction is market-exclusion (taking from shared pool).

**If FACETS earns C6, Feld's queued-for-retirement on C5 Anti-Catch-up gets a second look next game.** The axis ledger's retirement protocol is designed to allow reversal.

## Non-goals

- No components beyond 60 jewels + bag + market strip. Intentionally minimal.
- No theme beyond "jewels to sort." No fictional world.
- No solo mode.
- No variable setup (starting hands, asymmetric powers). Everyone starts equal.
- No catch-up mechanism. Leading players stay in control.
- No art assets this version — jewel types described by color/shape; playable with glass beads, poker chips, or generic tokens.

## Open questions (to resolve during DESIGN)

1. **Scoring value calibration.** Sapphire ×3 and Topaz ×5 may over- or under-pay vs Ruby ×1 and Emerald (min 5) ×2. First play should record max-score-by-type-specialization and adjust.
2. **Diamond assignment rule.** Can Diamonds be split across types (e.g., 2 to Ruby, 3 to Topaz) or must all Diamonds be assigned the same type? Recommend split-allowed for maximum strategic depth; confirm in DESIGN §6.
3. **Pass action behavior.** Proposed: passing adds 2 jewels from bag to market (accelerates endgame). Alternative: passing gives +1 tiebreaker token. First play will reveal which generates more meaningful choice.
4. **Market refill after take.** Refill immediately (market always at 5 until late-game) vs refill at turn end (strategic timing). Recommend immediate; simpler.
5. **End condition tuning.** "Bag empty AND market < 5" may end game abruptly. Alternative: "Bag empty, then one more full round." Test which produces cleaner endgames.

All five resolved in `design.md` pre-TIER-A.

## Compliance checks (per `/tigris-concept` skill invariants)

- ✅ Anchor persona declared: Feld (file exists at `personas/designers/feld.md`).
- ✅ Anchor axis declared: C6 Point-Salad Incommensurability (in axis-pool.md, currently `live`).
- ⚠️ Anchor-axis is queued from a retire-explicit in T&E (0.5 refute). A successful earning in FACETS vindicates the axis; failure compounds the retirement case. Risky anchor — but deliberate.
- ✅ Artifact-as-story identified: jewel bag + physical tokens.
- ✅ Honest inspiration list; direct borrowings called out.
- ⚠️ Axis-adjacency partner for C6 in the Parliament chart: A2 Decision Density (C6 ↔ A2). Feld's preferred_axes include A2 at rank #3 — if drafted, collision opportunity exists. If not drafted by Feld or another persona, this instance of `anchor_axis_without_collision_partner` strengthens I-parliament-03's cluster (already proposed-amendment, 2 instances; would become 3 if FACETS also lacks partner).

Ready for DESIGN.
