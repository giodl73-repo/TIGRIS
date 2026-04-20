---
name: Knizia concept review v2 — ZEN PATH (validation pass)
slug: knizia-concept-review-v2-zen-path
game: 0010-zen-path
stage: concept-review
reviewer: knizia
anchor: true
version: 2.0.0
rubric_version: v2.9
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# Knizia — ZEN PATH concept review v2 (validation)

## Did it land?

Yes. The high-leverage revision I named in v1.0 is on the page: Step tiles are now partitioned into Honor-Step / Strength-Step / Strategy-Step (4 each). Movement no longer launders fungibly into a side-ledger. **The tile that moves you IS the aspect.** That is the Samurai lesson executed, not cited.

The branching forks earn their keep too. Fork 1 (Mountain vs River) and Fork 2 (Temple vs Market) each bias different aspects, which means the *path choice* is an aspect-choice — a second commitment vector on top of tile-type. Two orthogonal commitments where v1.0 had zero is the structural fix, not a decoration.

## Arithmetic attack (the test)

**v1.0 racer:** position 35 × MIN 5 = 175. Balanced 20 × 5 = 100. Racer wins.

**v1.1 racer:** to reach position 35, the racer plays ~12 Step tiles (avg 2.9/square with partitioned Steps averaging 2 squares). Partitioned Steps force distribution: to get 12 Steps out of 12 in bag (4 per aspect), the racer's Honor/Strength/Strategy are each +4 from Steps alone. MIN from Step-play = 4. Ascends add selectively; call it +1-2 to the weakest. MIN ≈ 5. Score: 35 × 5 = 175 — but *only if they drafted across all three Step subtypes*. If the market denies one subtype (12 tiles / 3 = 4 of each, trivially contested), MIN drops to 3. Score: 35 × 3 = 105.

**Balanced player, position 25 × MIN 6:** 150. Competitive. More importantly: the **contested subtype** (the one three racers all need) becomes the Modern-Art-style moral instrument. Scarcity on one Step-color punishes the greedy.

Moon Phase REFLECT (−1 to lowest) is additional pressure on the min. The shape now bites from two directions.

Arithmetic passes. Marginally — but passes.

## C2 Minimum-Score Shape: **8.5** (up from 7.5)

Aspect-partitioned Steps close the fungibility gap. Not a 10 (Samurai still wins — castes had exclusive terrain, here the Ascend/Focus tiles remain an unpartitioned 18-tile reservoir that can repair neglect cheaply). But 8+ earned, threshold-healthy.

## Remaining concerns

1. **Ascend/Focus (18 tiles) are still fungible.** A racer can still top up a lagging aspect via the unpartitioned reservoir. Recommend at DESIGN: partition Ascend too, or cap Focus at one-per-aspect-per-game.
2. **Moon Phase REFLECT is soft.** −1 to lowest aspect per 5 rounds is a tap, not a blow. Compare T&E catastrophe. Consider −2, or scaling with leader position.
3. **Duel tile's printed aspect** removes a choice. Not fatal; makes duels a lottery rather than a commitment. Watch in playtest.

## Greenlight

**Yes.** Proceed to design.md. The two structural fixes (partitioned Steps, branching forks) convert the min-score shape from decoration into the load-bearing constraint. The math now punishes the greedy sprint. Tune Ascend fungibility at DESIGN; don't let it undo the work done here.
