---
name: Covenstat — Design (Rules)
slug: covenstat-design
game: 0019-covenstat
stage: design
version: 1.0.0
rubric_version: v2.16
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
anchor_persona: lacerda
anchor_axis: A7
---

# Covenstat — Rules

## 1. Overview & Object of the Game

Covenstat is a 2–5 player horror roll-and-write. Each player is an investigator of the same occult phenomenon but holds a different **Objective Sheet** that turns shared dice into private interpretations. 8 fate dice are rolled each round. Players draft dice in turn order and mark their sheets. The game ends when any player's sheet end-trigger fires OR round 12 completes. Highest individual score wins.

The core tension: shared input (dice), private objective. No player can copy another's strategy because the rubric is printed on each sheet, and the sheets differ. The Coven tracks summoning progress. The Skeptic tracks inconsistencies that disprove the occult. The Priest tracks blessings vs transgressions. Same dice; different games.

## 2. Components

- **12 Objective Sheets** (tri-fold A5 pads, 50 sheets each): Coven, Detective, Occultist, Medium, Alienist, Priest, Skeptic, Heir, Witness, Cartographer, Archivist, Relative.
- **10 Fate Dice** (4 standard d6 + 4 fate-colored d6 + 2 symbol d6).
  - Standard d6: numeric 1–6.
  - Fate d6: symbols (Sun / Moon / Blood / Key / Rune / Void).
  - Symbol d6: tarot glyphs (Fool / Tower / Star / Devil / Hanged / Wheel).
- **Pencils** (1 per player; soft graphite).
- **Round counter** (tracks 1–12).
- **First-player marker** (rotates each round).
- **Scoring quick-reference cards** (12, one per sheet — identical to sheet back-panel for readability).

## 3. Setup

1. Place the Round counter at 1.
2. Shuffle the 12 Objective Sheet pads face-down. Deal one to each player (2–5 sheets dealt).
3. Each player reads their sheet's front panel (flavor + end-trigger), inside panel (3 tracking columns with mark-boxes), and back panel (scoring rubric). Silently. Roughly 2 minutes.
4. Place remaining sheets face-up in the center as a **Public Catalog** — players can reference opponents' rubrics if they want, but need not.
5. Roll dice to determine first-player. First-player marker placed.
6. Round 1 begins.

## 4. Turn structure

Each round has 4 sub-phases:

### 4.1 Roll phase
All dice are rolled together in the center. Current player count determines dice pool:
- 2p: 6 dice (2 standard + 2 fate + 2 symbol)
- 3p: 7 dice (3 standard + 2 fate + 2 symbol)
- 4p: 8 dice (4 standard + 2 fate + 2 symbol)
- 5p: 10 dice (4 standard + 4 fate + 2 symbol)

### 4.2 Draft phase
Starting with first-player and proceeding clockwise, each player claims dice from the pool. Each player claims **2 dice per round at 2-3p, 2 dice per round at 4-5p** (the remainder go un-marked into the Discard). Claimed dice are taken in front of the player.

### 4.3 Mark phase
Simultaneously, each player marks their sheet per the sheet's printed rules. A sheet tells its owner which column each die feeds and how the die-face affects that column. Example: the **Coven** sheet reads "Standard d6: mark the number in Column A (Incantation). Fate d6: mark the symbol in Column B (Ritual). Symbol d6: mark the glyph in Column C (Vision)." Other sheets have different mappings.

### 4.4 End-trigger check phase
Each sheet has a printed end-trigger (e.g., Coven: "fill all 7 Incantation boxes"; Skeptic: "mark 4 Contradictions"). If any player's trigger fires this round, game ends **after end-of-round scoring**. Otherwise, advance Round counter, rotate first-player marker clockwise, proceed to next round.

Game also ends if Round 12 completes without any trigger.

## 5. Actions

Formally, there is exactly **one action**: Claim-and-Mark. Each player claims their allotted dice from the round's pool and marks them per sheet rules. The complexity lives in:

- **Die selection** (which dice to claim — 2-axis decision: which die values best feed your columns × which dice *deny* your opponents).
- **Column assignment** (some sheets allow player choice of which column a die feeds).
- **Bonus triggers** (each sheet has 2-4 printed bonuses — e.g., Detective: "3 matching values in Column A → +5 VP").
- **Pattern tracking** (sheets like Cartographer score for adjacency patterns across columns).

Single-action menu is **deliberately narrow** per A6 Teachability constraint, consistent with the Vigil-lineage of action-menu clarity. Sheets absorb the complexity.

## 6. Scoring / end condition

**Individual scoring** — no cross-player aggregation or comparison. Each player computes their own score from their sheet's rubric at game-end.

End-triggers (round 12 max):
- Any player's sheet-trigger fires → round completes, then game ends.
- Round 12 completes with no trigger → game ends.

Scoring: each sheet produces a 0–60 total from:
- **Column totals** (raw mark counts × printed multipliers)
- **Pattern bonuses** (matched values, adjacencies, symbol-groups per sheet rubric)
- **Trigger bonus** (+10 VP for the player whose trigger fired)
- **Collapse penalties** (−15 VP per Column that overflowed its capacity)

Highest score wins. Ties broken by: most trigger-bonuses > most columns at capacity > dice-count remaining in front of player.

## 7. Edge cases & clarifications

- **Dice conflicts in draft**: claims are one-at-a-time in clockwise order; no simultaneous claiming. If you want a die your opponent wants, seat order matters (hence first-player rotation).
- **Unused dice**: dice left in the pool after all players have drafted go to Discard. No one claims them.
- **Sheet swaps not allowed**: once dealt, sheets are locked. No trading.
- **Column overflow**: a column is "at capacity" when all its boxes are marked. Any additional mark that would go into it becomes an **overflow mark** in a sidebar; 3 overflow marks in a single column = Collapse (−15 VP penalty, sheet-specific effect).
- **Multiple triggers same round**: if two players hit end-triggers simultaneously, both get the +10 trigger bonus.
- **Player count scaling**: dice-per-player rate is calibrated so each player averages 24 marks across 12 rounds at all counts.
- **Tie on first-player roll**: reroll tied dice only.

## 8. Special subsystems

### The 12 Objective Sheets (summaries)

Each is a 3-column sheet with a unique rubric. Full sheet content is in separate files (deferred to v1.1); summaries for panel review:

| Sheet | Columns (abbrev.) | End-trigger | Collapse means |
|---|---|---|---|
| **Coven** | Incantation / Ritual / Vision | Fill all Incantation | Ritual fails → −15 + Vision locked |
| **Detective** | Suspects / Evidence / Motives | Close the Case (3 suspects + 3 evidence + 1 motive match) | Wrong suspect convicted → −15 |
| **Occultist** | Texts / Rites / Sigils | Complete 2 Sigils | Text misread → −15 + Sigil forfeit |
| **Medium** | Séances / Spirits / Messages | Contact 5 Spirits | Possession → −15 + control-shift |
| **Alienist** | Delusions / Diagnoses / Treatments | Classify 4 Diagnoses | Patient escapes → −15 |
| **Priest** | Blessings / Transgressions / Penances | 10 Blessings minus Transgressions | Faithless → −15 |
| **Skeptic** | Inconsistencies / Explanations / Proofs | 4 Proofs of non-occult origin | Converted → −15 |
| **Heir** | Deeds / Claims / Secrets | Uncover 3 Secrets | Disinherited → −15 |
| **Witness** | Sightings / Testimonies / Recants | 6 Testimonies | Credibility broken → −15 |
| **Cartographer** | Rooms / Routes / Hidden | Map adjacency: 4-room loop | Cartographer gets lost → −15 |
| **Archivist** | Documents / Dates / References | 3 Reference-chains of length 3 | Archive fire → −15 |
| **Relative** | Memories / Wounds / Confessions | 2 Confessions | Denial → −15 |

Each sheet takes ~30 seconds to understand after first demo. First-play with 12-sheet catalog is the teach-risk per §10.

### First-Player rotation

First-player marker moves clockwise each round. Rationale: first-player advantage is real (best pick of dice), so rotation averages the advantage across the table.

### Column-capacity calibration

Each column has 5-8 boxes, calibrated so a single column filling alone happens around round 8-10 at competent play. Sheets with longer columns typically have faster-firing triggers in other columns.

## 9. Variants

- **2-player variant**: each player holds **2 sheets** instead of 1. Dice pool reduced to 6. Allows strategic sheet-allocation (which sheet to favor each round).
- **5-player variant**: standard; dice pool 10.
- **Speed Variant**: game ends at round 8 regardless. Higher-scoring rubric rewards aggressive marking.
- **Horror Variant**: 4 additional Chaos d6 added to pool; Chaos faces trigger sheet-specific negative marks if claimed. High-risk dice.

## 10. Designer notes

**The load-bearing bet is A7 Emergence-Replayability.** Pool at 24/25 adopted includes A7 at contested-watch (2E/1R after AFoS Halifax Hammer refute). Vigil demonstrated that counter-pressure designs can move contested-watch axes. Covenstat mirrors the pattern for A7.

**Why 12 sheets, not 6 or 20?** 12 is the sweet spot: 12C5 = 792 five-player role-draws; 12P5 = 95,040 seat-order-dependent combinations. Multiplied by dice-shuffle permutations, this exceeds 10^8 distinct opening-states. Below 10 sheets, role-draws become predictable across repeated plays. Above 15, teach-time cliff.

**Why shared dice + private rubric, not personal dice?** Personal dice would make Covenstat pure multiplayer-solitaire (the Wingspan A3 refute pattern). Shared dice force players to watch opponents' claims. A Skeptic watching a Coven draft a Sun fate die learns: "Coven is building Ritual; I can deny the next Moon to slow them." This is the A3 Interaction hook.

**Why no Monster / haunting track?** Vigil already covers horror-via-monster-pressure. Covenstat's horror is interpretive — 5 people looking at the same dice and seeing 5 different nightmares. Different design bet from Vigil; avoids redundancy in the 3-original corpus.

**A7 ↔ A3 design tension**: emergence benefits from private objectives (variance), but interaction benefits from shared state (dice-competition). Covenstat threads the needle: objectives private, dice shared, sheets publicly visible. This may or may not land — the panel will test it.

## 11. Collision Adjacency Chart

New adjacency candidates:
- **A7 Emergence ↔ A4 Variance Calibration** — Covenstat's emergence IS variance-as-resource. In roll-and-write designs with sheet-catalog variation, these two axes gear together (unlike adversarial games where they are orthogonal).
- **A7 ↔ C6 Point-Salad Incommensurability** — 12 sheets × 3 columns × 2-4 bonuses = high point-salad density. Emergence-in-rubric-variety produces point-salad-in-scoring.
- **A3 ↔ B4 Information-Transparency-Cost** — Covenstat's public-sheet / private-valuation layer is an A3-enforcing B4. When sheets are visible but meaning is hidden, opponents must pay attention, which enforces interaction.

Submit to `personas/adjacency-chart.md` for I-te-01-style cluster review after panel.

## 12. Open at DESIGN-stage, to resolve post-review

- **Sheet count (10 / 12 / 15)?** 12 chosen; panel likely tests whether teach-time exceeds 10 min at 12 sheets visible in catalog.
- **Dice pool sizing**: 6 / 7 / 8 / 10 across player counts. Calibration target: 24 marks per player across 12 rounds. Verify at 3p and 5p.
- **Draft-order simultaneity**: strict clockwise vs simultaneous-with-conflict-resolution. Start strict; test downtime at 5p.
- **Trigger-bonus calibration**: +10 VP for end-trigger firing. Is this enough to motivate racing? Too much?
- **Collapse severity**: −15 VP per collapsed column. Can stack if multiple columns collapse — cap it?
- **Variant sheet balance**: some sheets inherently faster (Skeptic: 4 Proofs) than others (Witness: 6 Testimonies). Need playtest-data balance table.
- **Public Catalog value**: do players actually reference opponent rubrics, or ignore them? A3-load depends on this.
- **First-play setup time**: 12 sheets × 2 minutes each of flavor-read = 24 min bookkeeping, far over 10-min teach target. Mitigation: sheets pre-dealt, only draw your own + quick-ref cards for opponents. Test.
- **Does A7 actually earn?** The whole point. If 3 sessions of Covenstat produce identical strategy arcs, A7 refutes again and takes 2nd refute-game → adopted-contested formal status. High-stakes.
