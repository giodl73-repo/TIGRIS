---
name: Hanabi — Design
slug: hanabi-design
game: 0058-hanabi
stage: design
version: 1.0.0
rubric_version: v2.23.29
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: lacerda
anchor_axis: B4
bgg_id: 98778
review_type: imported-review
---

# Hanabi — Design (Review Mode: Rules Summary)

## Published rules summary

**Players:** 2–5. **Length:** 25 min. **Weight:** BGG 1.7.

### Setup

50 cards (5 colors × 10 cards: 3×1, 2×2, 2×3, 2×4, 1×5). Players hold cards facing OUT (they see others' hands; not their own). 8 clue tokens; 3 fuse tokens.

### Turn structure

On your turn: do ONE of:
1. **Give clue:** Spend 1 clue token. Point to one or more cards in one player's hand and say either their color OR their number. (Not both; one attribute per clue.)
2. **Play a card:** Take one card from your hand and place it to the fireworks table. If it continues a sequence (matching color, next number), it succeeds. If not, lose 1 fuse token; discard the card.
3. **Discard a card:** Take one card from your hand, discard it face-up (return 1 clue token). Draw a replacement.

### End conditions

- **Win:** Complete all 5 fireworks (25 cards played perfectly).
- **Lose:** 3rd fuse token lost (misplay), OR deck runs out (finish the round; score partial).

### Information structure (B4 mechanism)

Your OWN hand is hidden from you. You can see all other players' hands completely. A clue gives you partial information: "your position-1 and position-3 cards are red" tells you color but not number; "your position-2 card is a 4" tells you number but not color. Most information about your own hand is inferred from clue combinations + deductive reasoning about what others would or wouldn't clue you.

**Cost of information:** Each clue costs a token; discarding recovers a token; playing costs nothing but risks a fuse. The information economy is the game — players balance clue-spending with fuse-risk.
