---
name: ZEN PATH — Concept (v1.1 post-panel)
slug: zen-path-concept
game: 0010-zen-path
stage: concept
version: 1.1.0
rubric_version: v2.9
bet_version: parliament
author: TIGRIS (user + Claude collaboration)
created: 2026-04-19
updated: 2026-04-19
anchor_persona: knizia
anchor_axis: minimum-score-shape
review_type: original-design
origin: "Second co-designed TIGRIS original. Chutes & Ladders × Samurai. Concept v1.0 panel-reviewed by 8 designers; v1.1 applies full punchlist P1-P5."
revision_notes: "v1.1 applies 5 revisions: P1 branching forks (architectural teeth), P2 aspect-partitioned Steps (scoring-dominance fix), P3 2p full mode + solo sketch (count-robustness), P4 reduced Steps + Moon Phase (feeding-equivalent), P5 private hand + small market (partial info-asymmetry)."
---

# ZEN PATH — Concept (v1.1)

## Premise

ZEN PATH is a 3-4 player, 45-60 minute tile-placement race game where a samurai's journey splits at strategic forks, progress is earned by balancing three aspects (**Honor**, **Strength**, **Strategy**), and the winner is whose weakest aspect is strongest — multiplied by how far they traveled.

**The cross**: Chutes & Ladders (track race with jumps) meets Samurai (Knizia 1998 — minimum-score-shape). v1.1 restructures the track into **branching forks** (per K-K), partitions movement tiles **by aspect** (per Knizia), adds a **private hand** (per Lacerda), introduces a **Moon Phase feeding obligation** (per Rosenberg), and fully specs **2p and solo modes** (per Stegmaier).

The scoring equation unchanged:
```
final_score = track_position × MIN(Honor, Strength, Strategy)
```

But now EVERY movement action commits to an aspect (partitioned Steps), EVERY turn you choose a private tile AND a public one (hand-and-market), and EVERY 5 rounds the **Moon Phase** forces an aspect-sacrifice or tile-purge.

The bet: this structural revision makes the 3 aspects genuinely incommensurable (C6 earning), the path genuinely spatial (D2 earning), and the emergence real (A7 earning) — all axes that v1.0 failed.

## Players and length

- **Players:** 2-4 (2p full mode, 3-4p design target). Solo sketched.
- **Length:** 45-60 min (up from v1.0's 35-45 — branching + Moon Phase add time).
- **Weight:** ~2.8/5 (up from 2.5).

## Anchor mechanic (v1.1)

**Branching aspect-track race with min-score endgame + private-hand drafting + aspect-partitioned movement + Moon Phase feeding.**

Four intertwined systems (up from 3):

1. **Branching path** — not a single line. 40 total squares distributed across a branching shape: Origin → linear to Fork-1 at pos 10 → split into two paths (Mountain / River, each 10 squares) → rejoin at pos 20 → Fork-2 (Temple / Market, each 10 squares) → final to Peak at pos 40. Different paths have different pre-printed ladders/chutes.
2. **Aspect tracks (0-10 each)** — Honor / Strength / Strategy. Players commit to building these through AspectStep movements.
3. **Hand + small market (hybrid draft)** — each player keeps a private 5-tile hand. A public 3-slot market shows 3 tiles from the bag. Players can draft from market (public info) OR draw from bag (hidden).
4. **Moon Phase obligation** — every 5 rounds, all players simultaneously resolve a Moon event: −1 to lowest aspect OR retreat 2 squares OR discard 2 tiles from hand. Forces periodic sacrifice.

## Core mechanic details (v1.1)

### Turn structure

Each turn = **2 actions**. Actions:

- **DRAFT** (1 action): take a tile from the 3-slot market into your private hand. Market refills from bag.
- **DRAW** (1 action): draw a tile from the bag into your hand (blind; public doesn't see). 
- **PLAY** (1 action): play a tile from your hand. Apply its effect.

Hand size cap: **5 tiles**. If full, can't DRAFT or DRAW until you PLAY.

Turn flow: pick any 2 actions (can be same type). Play passes clockwise.

### Tile types (~50 tiles total — v1.1 revised)

**v1.1 changes**: Step count reduced from 18 to 12 (3 × 4); Steps are now aspect-partitioned.

| Type | Count | Effect |
|---|---:|---|
| **Honor-Step** | 4 | Move 1-3 squares; +1 Honor. Movement IS aspect-commitment. |
| **Strength-Step** | 4 | Move 1-3; +1 Strength. |
| **Strategy-Step** | 4 | Move 1-3; +1 Strategy. |
| **Ascend** | 12 | +1 to one chosen aspect (no movement). 4 of each aspect. |
| **Focus** | 6 | +2 to one chosen aspect (rarer, stronger). |
| **Ladder** | 5 | Place on any unoccupied path square; creates a new ladder with aspect-condition of your choice. |
| **Chute** | 5 | Place on any unoccupied path square; creates a new chute with aspect-condition. |
| **Duel** | 4 | Force aspect-comparison with an adjacent-path samurai; loser retreats 2 squares. Resolution: which aspect to compare is printed on the card (1 of 3). |
| **Moon** | 3 | Optional tile — triggers a Moon Phase early (can be tactical). |

**Total**: 51 tiles. Reduced from v1.0's 55.

**Key v1.1 change**: Step tiles are aspect-committed. You can't race without building aspects. Addresses Knizia's race-plus-one-aspect dominance concern.

### Branching forks (v1.1, P1)

**Fork 1** (position 10): path splits into Mountain Path OR River Path. Both 10 squares long; rejoin at position 20. Each has 2 pre-printed ladders/chutes with different aspect-conditions:
- Mountain Path: Honor/Strength-favored (rugged terrain rewards warrior aspects)
- River Path: Strategy-favored (flowing navigation rewards tactical thinking)

**Fork 2** (position 20): splits into Temple Path OR Market Path, both 10 squares to Peak.
- Temple Path: Honor-favored
- Market Path: Strategy/Strength-favored

At a fork, you choose which side to enter. Once chosen, you can't backtrack until rejoin. Different aspect investments reward different fork choices.

**Earns D2 Spatial-Interaction** (per K-K); **earns A7 Emergence-Replayability** (multiple path topologies).

### Moon Phase (v1.1, P4)

After every 5 rounds of play, a **Moon Phase** triggers. All players simultaneously pick ONE:

- **REFLECT** — lose 1 point from your lowest aspect. (Forces aspect-balance.)
- **RETREAT** — move back 2 squares on the path. (Path cost.)
- **PURGE** — discard 2 tiles from your hand face-down. (Hand cost.)

Moon tiles (3 in the deck) can trigger a Moon Phase early via a player's action — tactical timing.

**Feeding-equivalent** per Rosenberg. Periodic obligation that forces strategic sacrifice.

### Scoring (unchanged — Knizia anchor)

```
final_score = track_position × MIN(Honor, Strength, Strategy)
```

Position: 1-40 (based on location at game end; samurai's current square).
MIN: lowest of your 3 aspect-track values.

Tiebreakers: (1) second-lowest aspect, (2) highest position, (3) shared victory.

### End condition (unchanged)

- Any player reaches Peak (position 40), OR
- Tile bag empties and market has ≤ 2 tiles.

Complete current round; then score.

## v1.1 additions

### 2-player full mode (P3)

- Path: reduced to 30 squares (Origin → Fork 1 at 8 → rejoin 15 → Fork 2 at 23 → Peak at 30).
- Tiles: 40 (scale down proportionally).
- Hand size: 4 tiles (instead of 5).
- Moon Phase: every 4 rounds (faster pace).
- Expected length: 25-35 min.

### Solo mode sketch (deferred full rules to v1.1 post-playtest)

- Single player vs "Master" automa.
- Automa moves on the main path via fixed scripted action deck.
- Automa builds balanced aspects (fixed 3-3-3 to 4-4-4 over game length).
- Win condition: beat Master's final score.
- Expected length: 30-40 min.
- Notes: specifics to be finalized in design.md after first play tests of 2-4p base game.

## Artifact-as-story

**The branching path board** is the centerpiece. Not a single line; a Y-shaped split at two decision points. Each fork visually distinct (Mountain vs River vs Temple vs Market with thematic art). Three aspect tracks beside the board. Moon cycle indicator (tracks rounds between Moon Phases). Private hand screens for each player.

## Inspiration / lineage (v1.1 updated)

- **Reiner Knizia — *Samurai* (1998; BGG 3)** — minimum-score shape; 3 aspects.
- **Reiner Knizia — *Tigris & Euphrates* (1997; BGG 42)** — min-score across multiple tracks.
- **Snakes & Ladders / Chutes & Ladders** — track race with jumps (BGG 2921).
- **Reiner Knizia — *Taj Mahal* (2000; BGG 475)** — auction + min-score.
- **Tzolk'in (Chvátil 2012; BGG 126163)** — for path-rotating/cycling precedent (v1.0 panel reference).
- **Thebes (Prinz 2007; BGG 27708)** — time-track race with balance choice (v1.0 panel reference).
- **Tikal (Kramer/Kiesling 1999; BGG 54)** — branching path + action-point (v1.1 branching inspiration).
- **Dominion (Vaccarino 2008; BGG 36218)** — private-hand card game (v1.1 hand precedent).

Honest: the architecture is now a **compound novel bet** rather than a simple combination. Branching + aspect-partitioned Steps + private hand + Moon Phase × min-score endgame is a new stack. May earn B5 where v1.0 refuted.

## Target review (predicted)

**Expected earnings (v1.1):**
- C2 Min-Score Shape (anchor) — still canonical
- C1 Tension Budget — hand cap + Moon obligation
- C7 Action-Menu Clarity — 3 actions per turn; still clean
- A1 Elegance — math still tight; added systems don't bloat rules
- B3 Conversion Chain Depth — tile → aspect → position → MIN-score × position
- D4 Late-Game Lock-in — fork choice commits mid-game
- A3 Interaction — Duel + Chute placement + shared fork contest
- **D2 Spatial-Interaction** — branching forks NOW earn (v1.0 didn't)
- **A7 Emergence** — branching + Moon timing variance creates game arcs
- A2 Decision Density — 2 actions × 3 choices + aspect-committed movement
- **A6 Teachability** — still contingent; iconography required

**Expected refute (v1.1):**
- B2 Catastrophe Pressure — Moon is periodic obligation, not catastrophe
- C6 Point-Salad — still single-curve scoring (C6 remains refute-direction)
- **B5 Architectural Novelty** — compound bet may earn 5-6; still not Chvátil-canonical

## Non-goals (v1.1)

- No dice (design commitment preserved).
- No multi-player asymmetric powers (base game; potential expansion).
- Not a story-narrative game; mechanical puzzle first.

## Open questions (to resolve during DESIGN)

1. Path exact topology (tile positions; ladders/chutes per fork).
2. Hand cap tuning (5 at 3-4p; 4 at 2p).
3. Market size tuning (3 slots proposed).
4. Moon Phase timing (every 5 rounds; may need 4 or 6).
5. Aspect cap (10 proposed).
6. Solo Automa action deck (~20 cards sketched).
7. Duel tile aspect-selection (printed on card, not chosen at play — simpler).
8. Tile ratio balance (12 Steps : 12 Ascends : 6 Focus : 10 Ladder/Chute : 4 Duel : 3 Moon = 51 total).
9. Starting hand size (3 tiles proposed).
10. 2p path scaling (30 squares; may need 24 or 36).

## Compliance checks (v1.1)

- ✅ Anchor: Knizia on C2 (unchanged).
- ✅ Anchor adjacency (v2.3): C2 ↔ B3. Vaccarino drafts B3. Cross-player. Passes.
- ✅ Artifact: branching path board.
- ✅ Honest lineage (v1.1 adds Tikal branching, Dominion hand).
- ✅ All 5 punchlist revisions applied.

## Status (v1.1)

**Concept v1.1 complete.** All 5 user-approved revisions applied:
- P1 (architecture teeth): **branching forks** — 2 decision points splitting the path
- P2 (scoring dominance): **aspect-partitioned Step tiles** — movement IS aspect-commitment
- P3 (count-robustness): **2p full mode + solo sketch** specified
- P4 (tile economy): **Steps reduced to 12 + Moon Phase** feeding-equivalent
- P5 (info-transparency): **private hand + small market** hybrid

**Expected panel score improvement:**
- Chvátil B5: 3 → 6-7 (compound novel bet; not Tzolk'in-radical but earns teeth)
- Lacerda B1: 3 → 7+ (4 subsystems now; branching + Moon multiply with existing)
- Knizia C2: 7.5 → 8+ (aspect-partitioned Steps fix scoring dominance)
- Feld C6: 3.5 → 5 (still single-curve; acknowledged)
- K-K D2: 4.5 → 7+ (branching spatial matters)
- Stegmaier D3: 4 → 7 (2p+solo specced)
- Rosenberg C3/C4: 4/3 → 6/5 (Moon Phase feeding + aspect-Steps as soft-engine)

**Next step:** 3-persona validation pass (Knizia + Chvátil + Lacerda — the sharpest critics from v1.0). If all three greenlight, proceed to design.md.
