---
name: SOLSTICE — Design
slug: solstice-design
game: 0050-solstice
stage: design
version: 1.0.0
rubric_version: v2.23.21
bet_version: parliament
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# SOLSTICE — Design

## Overview

SOLSTICE is a 2–4p seasonal card game played over 4 Seasons. Each player receives 15 cards (a personal deck of 5 card types × 3 copies each). Five Arenas score at the end of each Season based on which card types were played to them. A player's score = their minimum-Arena score + Solstice Bonus (extra VP in their strongest Arena).

## Setup

- 5 Arenas in the center: Forest, Mountain, River, City, Sea.
- Each player has a personal hand of 15 cards: 3 Suns, 3 Moons, 3 Stars, 3 Storms, 3 Seeds.
- Each card type has an affinity: Suns score in Forest + City; Moons score in River + Sea; Stars score in Mountain + Sea; Storms score in Mountain + City; Seeds score in Forest + River.
- Each Arena's Season bonus = sum of played cards × that card's affinity multiplier.

## Core turn (each Season)

1. **Allocation phase (simultaneous):** Each player secretly distributes 3 cards from their remaining hand (face-down) to any Arenas they choose. Up to 3 Arenas can receive 1 card each, or 1 Arena can receive all 3.
2. **Reveal and score:** All cards revealed simultaneously. Each Arena calculates its Season score: sum of matching-affinity cards played by all players, + 1 VP base per card in the Arena.
3. **Distribute Season VPs:** Each Arena's score is split proportionally among players who contributed to it.

## Scoring formula

After Season 4: each player's TOTAL score = minimum(Arena1_score, Arena2_score, Arena3_score, Arena4_score, Arena5_score) + bonus_in_strongest_arena.

Solstice Bonus: the player's single highest-scoring Arena grants +2 VP per card they contributed to that Arena.

## Special cards (advanced variant)

Each card type has one special (★) card in the 15-card personal deck:
- **★ Sun:** score as 2 Suns anywhere.
- **★ Moon:** protect your River/Sea contributions from being undercut.
- **★ Star:** move 1 card from another Arena to Mountain/Sea.
- **★ Storm:** cause 1 VP loss to opponent's strongest Arena this Season.
- **★ Seed:** after scoring, plant +1 VP in lowest-scoring Arena permanently.

## Tension mechanics

With 15 cards over 4 Seasons (3 per Season × 4 = 12 played + 3 held), players must commit their full budget thoughtfully. After Season 2, hand size is 9 remaining cards — the allocation decisions lock in at Season 2 (enough cards for 3 Seasons, but committing to Arena balance vs. Solstice focus becomes critical).

## Elegance note

The FULL scoring rule: minimum across arenas + bonus-in-best. That is the entire game. The card-type affinity table is the complete rules annex. SOLSTICE is designed to be teachable in under 6 minutes, playable in 30, and strategically interesting for 50 plays.
