---
name: Codenames — Rules Summary (review-import)
slug: codenames-design
game: 0025-codenames
stage: design
version: 1.0.0
rubric_version: v2.22
bet_version: parliament
author: TIGRIS (review summary of Chvátil's rules)
created: 2026-04-20
updated: 2026-04-20
canonical_source: Codenames rulebook (Czech Games Edition, 2015)
bgg_id: 178900
review_type: imported-review
---

# Codenames — Rules Summary (Review Import)

Condensed summary for review purposes. Canonical rules in the 2015 Czech Games Edition rulebook. Fair-use: critical commentary.

## 1. Overview

2–8 players in two teams (Red and Blue). A 5×5 grid of 25 word cards is laid on the table. A secret key card (known only to the Spymasters) marks 8–9 words as Red agents, 8–9 as Blue agents, 1 as the Assassin, and the remainder as Bystanders. Teams alternate turns. On each turn:

- The **Spymaster** gives a one-word clue + a number (e.g., "ocean 3" = "I'm hinting at 3 words related to 'ocean'").
- The **Operatives** on that team discuss and touch 1 word per guess, up to (number + 1) guesses. A correct guess (your team's agent) is covered with your color token. An opponent-agent touch ends your turn and gives them a covered word. Touching the Assassin ends the game immediately — you lose.

**Win condition:** First team to cover all their agent words wins.

## 2. Components

- 200 double-sided Word cards (400 unique words total; 25 used per game in 5×5 grid)
- 40 Agent cards (Red and Blue)
- 1 Double Agent card (first team's color gets 9 words instead of 8)
- 7 Innocent Bystander cards
- 1 Assassin card
- 40 Key cards (secret maps of which words are which team's)
- 1 Card stand (holds the Key card upright so only Spymasters see it)

## 3. Setup

1. Lay 25 word cards randomly in a 5×5 grid.
2. One player from each team becomes the Spymaster. Spymasters sit together, facing their team.
3. A random Key card is placed in the card stand (only Spymasters see it).
4. The Key card shows: Red agent positions, Blue agent positions, Assassin position, and Bystander positions. The team with 9 agents (instead of 8) goes first.

## 4. Turn structure

**Spymaster's turn:** Give exactly one word clue + one integer. The clue must:
- Be one word (no compound words unless they're standard).
- Not be a word currently on the grid.
- Not be a variant or partial of a grid word.
- Connect semantically to the target words without being too obvious.

**Operatives' turn:** Discuss and touch grid words in sequence:
- Touch a word → Spymaster reveals whether it's your agent (cover it), opponent's agent (cover with their color; your turn ends), Bystander (cover gray; your turn ends), or Assassin (you lose immediately).
- May continue guessing up to (clue number + 1) times, as long as you keep hitting your agents.
- May stop guessing at any time (pass is allowed after ≥ 1 correct guess).

**Turn ends** when: Operatives choose to stop, touch a non-agent word, or reach the maximum guess count.

## 5. End condition

- **Win**: Cover all your team's agent words first.
- **Lose via Assassin**: Touch the Assassin on your turn.
- **Lose by covering all opponent agents**: If you accidentally cover the last opponent agent, they win.

## 6. Key mechanics summary

| Mechanic | Role |
|---|---|
| **One-word clue + number** | Core information-compression mechanism |
| **Shared word grid** | Both teams see all 25 words; only Spymasters know the key |
| **Assassin constraint** | Creates a hard boundary on clue construction |
| **Semantic adjacency** | Spymaster must find words that span multiple targets without touching enemy/Assassin |
| **Team discussion** | Operatives interpret clue collaboratively; social negotiation of meaning |
| **Cascading risk** | Each guess burns safety margin; stopping early sacrifices speed |

## 7. Edge cases

- **Invalid clue**: "numbers, sounds, abbreviations, and symbols" are banned. The one-word rule is strict.
- **Proper nouns**: allowed if the word is printed on a card (e.g., "Mercury" is a grid word, not a proper-noun clue violation).
- **Language**: clues must be in the same language as the grid. Multi-language editions exist.
- **2-player variant**: one Spymaster vs. one Operative; takes roughly 2× as long to interpret.

## Citations

- **Canonical rulebook**: Codenames (Czech Games Edition, 2015); designer Vlaada Chvátil.
- **BGG**: boardgamegeek.com/boardgame/178900
- **Fair-use posture**: Rules summary used as grounding for critical commentary in TIGRIS Parliament review.
