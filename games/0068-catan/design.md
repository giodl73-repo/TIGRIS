---
name: Catan — Design (Review Import)
slug: catan-design
game: 0068-catan
stage: design
version: 1.0.0
rubric_version: v2.23.40
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: vaccarino
anchor_axis: A4
review_type: imported-review
bgg_id: 13
bgg_url: https://boardgamegeek.com/boardgame/13
canonical_edition: The Settlers of Catan (1995, first edition)
game_designer: Klaus Teuber
game_first_published: 1995 (Catan Studios / Kosmos)
---

# Catan — Design (Review Import)

## Overview

Catan (The Settlers of Catan) is a 3–4 player resource collection and road/settlement building game. Players collect resources via dice rolls, trade with each other, and build roads, settlements, and cities to score VP. First player to 10 VP wins.

BGG weight 2.33. BGG rank: Family Games all-time great; Strategy Games top 100.

---

## Rules summary (for TIGRIS review)

### Core loop

Each round:
1. **Dice roll:** active player rolls 2d6; all players with settlements/cities on hexes matching the roll collect resources
2. **Robber:** rolling 7 → place Robber on any hex (blocks collection); steal 1 resource from adjacent player
3. **Trade:** active player may trade with other players or 4:1 with bank
4. **Build:** active player may build roads (1W+1B), settlements (1W+1B+1G+1O), cities (2G+3O), or development cards

### A4 design — Variance calibration

Catan's primary variance mechanism:
- **Dice variance:** settlements on high-probability numbers (6,8) collect more than on low (2,12)
- **Port management:** 2:1 ports reduce variance by enabling efficient conversion
- **Settlement placement:** initial placement is the core variance-calibration decision — placing on 6,8,9 with a 2:1 Ore port is canonical A4 play
- **Development cards:** cavalry cards move Robber away from your hexes (variance mitigation tool)

The Robber on 7 is the central variance event: rolling 7 depletes resources (hand limit: discard half if >7 cards) AND blocks a hex AND steals a card. A4 fires: every random element has a mitigation tool.

### A3 design — Trading interaction

Catan requires trade. The game's economy is designed so no player can develop efficiently without trading; different starting positions produce different resource surpluses. Trading with opponents is mandatory strategic interaction — not optional diplomacy.

### C3 design — Resource scarcity

The Robber creates resource scarcity: blocking a hex stops all production at that site. Players with settlements concentrated on one hex type are vulnerable to Robber deployment. Resource scarcity is player-directed and can be targeted.

### D2 design — Hex board blocking

The Longest Road and blocking via road-placement creates spatial interaction. Initial settlement placement blocks opponent settlement positions (settlements must be at least 2 road segments apart). Road-racing toward opponent intended positions is a direct spatial mechanic.

---

## Expected analysis angles

- **A4:** Dice + Robber + ports = variance calibration design
- **A3:** Mandatory trading; diplomacy-driven economy
- **C3:** Robber as player-directed scarcity (opponent can steal and block)
- **D3:** 3-4p; designed for exactly these counts
- **D2:** Settlement placement blocking; road-racing
- **A6:** Accessible teach (15 min); gateway game reputation
- **C1:** Perpetual resource tension (always need what you don't have)
- **A1:** Moderate elegance; rule count appropriate for weight

## Collision candidate

**A4↔C3 OP 6-2** — Variance calibration (A4: dice managed via ports/settlements; Robber = manageable variance) vs. scarcity pressure (C3: Robber depletes; resource monopolies form; player-directed scarcity). 41st temporal-register OP. New pair; first A4↔C3 instance. A4 asks "how is randomness managed?"; C3 asks "how much does resource shortage bite?".
