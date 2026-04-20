---
reviewer: Vital Lacerda
game: 0010-zen-path
stage: concept-review-v2
axes_drafted: [B1, B4, A7]
rubric_version: v2.9
review_version: 2.0
created: 2026-04-19
---

# Lacerda on ZEN PATH v1.1 — validation pass

I said the v1.0 design failed the Lisboa test: remove the player-placed ladders/chutes and a game remained. The revision claims four subsystems now multiply. Let me trace the gearing, subsystem by subsystem, before I re-score.

## Lisboa test, second pass

**Pull the branching forks (P1).** Path collapses back to a line. Aspect-partitioned Steps still commit you to an aspect; Moon still bites; hand still matters. But the fork is the *spatial locus* where aspect investment becomes path geometry — Mountain rewards Honor/Strength, River rewards Strategy. Without forks, aspect choice stops mapping to board position. The min-score geometry is intact, but the *placement meaning* evaporates. Forks are now load-bearing. Good.

**Pull aspect-partitioned Steps (P2).** Movement detaches from aspect commitment. Suddenly you can race without building and build without racing, exactly the v1.0 failure mode. Ascend/Focus remain, but they no longer compete with Steps for the same action. The fork-reward logic loses its teeth — you arrive at Mountain Path having spent no Honor/Strength to get there. Partitioned Steps are load-bearing.

**Pull the private hand (P5).** Public market still functions; draft-only game is playable. Here I hesitate — but the Moon Phase's PURGE branch (discard 2 face-down) presupposes hand privacy as a pressure. And DRAW (blind) exists only because a private hand exists to hide it. Pull the hand, Moon loses one leg, DRAW becomes pointless. Load-bearing, thinly.

**Pull the Moon Phase (P4).** This is the cleanest gear. Moon forces periodic aspect sacrifice OR path retreat OR hand purge — it ties all three other subsystems into a single recurring obligation. Remove it, and the game becomes monotonic accumulation again. The 10th-play optimum I warned about reopens. Load-bearing.

Four removals, four losses. The test passes. Not Lisboa-tight — Moon's three branches are alternatives, not a single gear that torques all systems at once — but this is genuine multiplication, not addition stacked higher.

## B4, second pass

Private hand plus blind DRAW plus face-down PURGE creates an actual information economy. You pay an action to DRAW blind (hidden upside) versus DRAFT visible (public commitment). Opponents can read your DRAFTs but not your DRAWs, so hand composition becomes a genuine read. It is not Taj Mahal — the asymmetry is thin and the market still leaks most information — but the axis is no longer abandoned. It is engaged, modestly.

## Gearing between Moon and path position

Moon's RETREAT branch costs 2 squares; REFLECT costs a min-score point; PURGE costs hand tempo. The samurai racing for Peak pays RETREAT harder than the samurai building aspects; the samurai with a thin hand pays PURGE harder. Different board states face different Moon costs. That is gearing between tile economy and path position — the exact dependency v1.0 lacked.

## New scores

- **B1 System Gearing: 3 → 6.** Four subsystems, each load-bearing under removal. The multiplication is real but not dense — Moon's alternatives are OR-branches, not AND-gears.
- **B4 Info-Transparency Cost: 2 → 5.** Hand + blind DRAW + face-down PURGE constitute a modest information economy. Not a Lacerda-canonical asymmetric engine, but the axis is engaged.
- **A7 Emergence: 4 → 6.** Fork topology × Moon timing × aspect-partitioned commits gives the 10-play arc more surface. Dominant-curve solution now depends on fork-choice, which depends on opponent aspect reads.

## Remaining concerns

Moon Phase is three alternative costs, not one compounding event — closer to Rosenberg feeding than to Lisboa's ship-clock. The private hand is 5 tiles of soft-hidden info in a game with a visible market; true asymmetry remains thin. And the scoring equation `position × MIN(H,S,St)` is still single-curve — the fork mechanic rewards it, but does not branch it.

## Greenlight

Conditional greenlight. The Lisboa test passes. Proceed to design.md. The remaining work is in playtest: verify that Moon's three branches actually get chosen differentially by board state, and that the private hand survives contact with veteran reads. If either collapses to a dominant response, B1 slips back to 4.

Knizia was right on the equation. I was right on the machine. The revision heard both.

---
Word count: ~490.
