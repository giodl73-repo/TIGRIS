---
name: Scythe — Design (Rules summary, imported)
slug: scythe-design
game: 0005-scythe
stage: design
version: 1.0.0
rubric_version: v2.4
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
canonical_source: Scythe rulebook, Stonemaier Games 2016
---

# Scythe — Rules Summary (Review Import)

Condensed summary for review purposes. Canonical rules in the Stonemaier 2016 rulebook.

## 1. Overview

Alternate 1920s Eastern Europe. Each player leads an asymmetric faction controlling a unique character, 4 mechs, workers, and a home territory. Over ~30 turns, players expand territory, upgrade their action mat, build structures, engage in occasional combat, and complete encounter cards. First player to achieve 6 stars (out of ~12 star conditions) triggers endgame. Final scoring multiplies across 6+ categories.

**Object:** achieve 6 stars to trigger endgame; score highest via end-game multiplier.

## 2. Components

- Hex board (~40 territories)
- 5 asymmetric faction boards (Saxony, Rusviet, Polania, Crimea, Nordic)
- 5 asymmetric player mats (random pairing with faction at setup)
- Character miniatures, mech miniatures, workers, structures, resources
- Combat cards, encounter cards, objective cards
- Coin tokens, popularity tracker, power tracker
- Factory card (central deck)

## 3. Setup

1. Each player gets a random faction board + random player mat (25 combinations).
2. Place character + 2 workers on home territory.
3. Deal 2 objective cards, 2 combat cards, 1 starting resources.
4. Determine first player randomly.

## 4. Turn structure

One action per turn. Each action comes from one of 4 rows of your player mat.

Each row has a **top action** (cheap) and a **bottom action** (payment + reward). You pick one row per turn; optionally take top, bottom, or both (bottom requires paying the cost, usually gained from top).

### Mat rows (typical — varies by mat):

1. **Move / Gain** (top) → **Upgrade** (bottom: improve action mat)
2. **Bolster** (top: gain power or card) → **Deploy** (bottom: build mech)
3. **Trade** (top: gain resources or popularity) → **Build** (bottom: build structure)
4. **Produce** (top: generate resources via workers) → **Enlist** (bottom: gain recruit bonuses)

**Repeating the same row twice in a row is forbidden** — rotation forces action diversity.

## 5. Actions

### Top actions (cheap)

- **Move** — move characters/mechs/workers to adjacent territories (lake/mountain restrictions).
- **Bolster** — pay $1: gain 2 power OR a combat card.
- **Trade** — pay $1: gain 2 resources (any) OR +1 popularity.
- **Produce** — produce resources on worker territories (cost scales with worker count).

### Bottom actions (expensive, one-time per mat)

- **Upgrade** — pay resources, improve future top/bottom action.
- **Deploy** — pay metal, place a mech (enables new movement).
- **Build** — pay wood, place a structure (permanent territory benefit).
- **Enlist** — pay food, gain recruit (triggers recurring bonus).

## 6. Combat

When two opposing forces meet on a territory, combat resolves. Both players bid power + combat cards secretly. Higher total wins; loser retreats (workers die). Winner takes territory.

Combat is rare (~2-3 per game) and often avoidable. Most Scythe play is peaceful expansion.

## 7. Stars and endgame

Achieve stars by completing milestones:
- Deploy all 4 mechs (+1 star)
- Build all 4 structures (+1 star)
- Enlist all 4 recruits (+1 star)
- Max out popularity (+1 star)
- Max out power (+1 star)
- Win 2 combats (+2 stars total for 2 wins)
- Complete both objectives (+2 stars)
- Produce 8 workers (+1 star)
- ...

First player to 6 stars triggers endgame (one more round then scoring).

## 8. Final scoring (the multiplier)

Score depends on popularity tier (3 bands):

- **Low popularity (0-6):** stars × 3 + territories × 2 + $/2 + resources × 1
- **Mid popularity (7-12):** stars × 4 + territories × 3 + $/2 + resources × 1
- **High popularity (13-18):** stars × 5 + territories × 4 + $/2 + resources × 1

**This is the 6+-factor end-game multiplier that's the signature Stegmaier scoring move.** Your stars, territories, and resources are multiplied differently depending on your popularity band — encouraging benevolent play (popularity matters) but still rewarding aggressive play.

**For TIGRIS rubric: this is the instance that I-parliament-02 (Scoring Multiplier Dependency axis candidate) is testing.**

## 9. Special subsystems

- **Encounter cards** — randomly triggered when characters land on encounter hexes. Choose: Popularity + Resources / Combat boost / Power gain. Provides narrative flavor.
- **Structures** — once built, permanent territory benefits (e.g., Monument: +1 popularity per territory with a structure).
- **Recruits** — permanent passive bonuses.
- **Factory card** — central deck; characters can visit the Factory hex to gain powerful special cards.

## 10. Designer notes (Stegmaier's commentary)

Stegmaier designed Scythe around three principles:
1. **Player-count robustness** — 1p (Automa), 2p, 3p, 4p, 5p all tuned. 1-5 officially supported; 7p via expansion.
2. **Asymmetric starting conditions** — 5 factions × 5 mats = 25 combinations per session.
3. **Avoid combat as primary driver** — most play is peaceful expansion; combat is optional and rare.

The end-game multiplier is designed to reward balanced play across multiple strategic dimensions simultaneously.

## 11. Citations

- *Scythe* rulebook, Stonemaier Games 2016.
- Stegmaier's Kickstarter update series (2014-2016).
- Stegmaier, *A Crowdfunder's Strategy Guide* (for context on the game's design process).
- BGG entry for Scythe (game ID 169786).

Summary posture: critical commentary under fair use. No verbatim rule text beyond minimum.

## 12. Weight & target audience

- Weight ~3.4/5 on BGG.
- Teach time: 30-45 min.
- Play time: 90-120 min base.
- Audience: mid-to-heavy Euro gamers; strong crossover appeal.

## 13. What this game is NOT

- Not a wargame — combat is secondary.
- Not a quick game — 90+ min standard.
- Not asymmetric only — multiple mat/faction combos make it asymmetric for any given play, but balance is maintained across combinations.
- Not a legacy game — each session is self-contained.
