---
name: CODEX
slug: 0151-codex
game: 0151-codex
stage: design
version: 1.0.0
rubric_version: v2.24
bet_version: parliament
author: TIGRIS
created: 2026-04-22
updated: 2026-04-22
type: original
anchor_persona: feld
anchor_axis: B1
players: 3-4
weight_estimate: 2.8
duration_estimate: 90
---

# CODEX — Design Document v1.0

## Elevator Pitch

Rival Schools of Magic compete to build the most powerful arcane engine on a shared hex
map. Before the game, each School drafts the **laws** of how their magic works — not
resources or spells, but the fundamental rules of their reality. Then they build, activate,
and fight for the territories that make their engine sing, before the Veil collapses and
the Unraveling settles who built best.

**Design gap targeted:** T+G+R (High Tension + Deep Architecture + Range) — the
"holy grail" gap in the 150-game TIGRIS corpus where no existing game scores high on all
three simultaneously.

---

## Core Architecture

**Two-layer design.** Phase 1 (Convocation) you are a *legislator* — writing your School's
constitution from a shared pool of Doctrine cards. Phase 2 (Construction) you are an
*executor* — building and running the engine those Doctrines define. The two layers are
cognitively distinct, creating a meta-game (the draft) that drives the main game.

**Engine depth from rule interaction, not component count.** Doctrines are rules that
modify how components behave. A game with 5 Doctrines and 6 components can achieve
Lacerda-level interaction density without Lacerda-level setup overhead.

**Tension from the Veil, not from score tracks.** The Veil is a shared doom clock that
advances every round and accelerates when engines grow powerful. Building deeper engines
hastens the endgame. Players cannot ignore it — the Veil creates a pressure envelope that
means every turn matters.

---

## Components

| Component | Count | Notes |
|---|---|---|
| Hex map | 48 hexes | 5 terrain types; modular tile variant available |
| School boards | 6 | Flame, Tide, Iron, Earth, Wind, Ash (4 play per game) |
| Founding Doctrine decks | 6 × 5 = 30 cards | 5 per School; draw 1 randomly at setup |
| Shared Doctrine pool | 25 cards | Snake drafted 4 each during Convocation |
| Veil Event deck | 20 cards | Flip 1 per Veil step advancement |
| Component tokens | 6 per School | Forges · Sanctums · Channels · Spires |
| Mana tokens | — | Ember · Flow · Weight · Void |
| Veil track | 1 board | 10-step shared doom clock |
| Iron Tokens | 20 | Used with Iron School mechanics |

---

## The Four Schools (Base Game)

### Flame — School of the Spreading Fire
**Color:** Red/orange. **Home terrain:** Ember Peaks + Forge Grounds.
**Core identity:** Speed and map control. Acts first every round. Punishes adjacency.
**Founding Doctrine deck (draw 1):**
- *Wildfire:* Once per game, your build action places 2 components instead of 1.
- *Consuming Fire:* Components adjacent to opponents produce +2 Ember when activated.
- *Ignition Point:* First Conflict action each round generates 2 Ember immediately.
- *Backdraft:* When targeted by a Conflict action, gain 1 Ember and 1 VP.
- *The Furnace:* Weight converts to Ember at 1:1 (default: 2:1).

### Tide — School of the Turning Water
**Color:** Blue. **Home terrain:** Flow Channels + Sanctum Hills.
**Core identity:** Flexibility and conversion. Can reposition components; highest conversion efficiency.
**Founding Doctrine deck (draw 1):**
- *The Current:* Activating one Channel chain-activates all adjacent Channels for free.
- *Rising Tide:* Flow converts to any mana type at 1:1 (default: 2:1).
- *The Undertow:* When an opponent builds adjacent to you, move their component 1 hex.
- *Drift:* Your components ignore adjacency requirements — build anywhere on the map.
- *Tidal Chart:* At game start, look at the top 6 draft cards and rearrange them in any order.

### Iron — School of the Patient Forge
**Color:** Dark grey/silver. **Home terrain:** Forge Grounds + Sanctum Hills.
**Core identity:** Accumulation and snowball. Every built component gets stronger over time.
**Founding Doctrine deck (draw 1):**
- *Tempered:* Each component gains 1 Iron Token permanently when built (+1 production).
- *Iron Discipline:* Each round you take no Conflict action: gain 2 Weight AND 1 VP.
- *The Alloy:* Activating a Forge adjacent to a Sanctum also activates the Sanctum for free.
- *Patience Protocol:* Each round you take no build action: gain 3 Weight.
- *Smelting:* Void converts to Weight at 1:1 (default: 3:1).

### Earth — School of the Rooted Ground
**Color:** Brown/green. **Home terrain:** Sanctum Hills + Forge Grounds.
**Core identity:** Resilience and territory. Immune to Edicts. Rewards wide territorial presence.
**Founding Doctrine deck (draw 1):**
- *Deep Roots:* Doctrines may be suspended by Edicts, but score 50% VP (rounded down) even when suspended. +1 VP per component on matching terrain (max 5 VP).
- *The Overgrowth:* After building, immediately claim 1 adjacent empty hex as territory.
- *Root Memory:* When any of your components is removed, gain 3 VP immediately.
- *The Watershed:* Sanctums also produce Void when activated (dual-output).
- *Fertile Ground:* Empty hexes adjacent to your components produce 1 Flow passively each round.

**Design constraint:** Deep Roots + Legislative Strike is a banned combination (see Key Rules). If Earth draws Deep Roots and drafts Legislative Strike during Convocation, Legislative Strike is immediately returned to the cut pile and Earth drafts a replacement card.

---

## The Two Expansion Schools

### Wind — School of the Howling Wind
**Color:** Light grey/white. **Home terrain:** Void Rifts + Ember Peaks.
**Core identity:** Disruption and displacement. Moves opponents rather than itself.
**Founding Doctrine deck (draw 1):**
- *Slipstream:* Once per round, push 1 adjacent opponent component 1 hex.
- *Scattering Law:* When an opponent activates adjacent to you, reduce their output by 2.
- *The Weathervane:* When any player uses a Conflict action, you may take 1 free action.
- *Interference:* Once per round, cancel 1 mana from any opponent's production (before they collect).
- *Turbulence:* When the Veil advances, each opponent must move 1 of their components 1 hex.

### Ash — School of the Dying Ember
**Color:** Black with ember-red highlights. **Home terrain:** Void Rifts + Forge Grounds.
**Core identity:** Decay and recycling. Profits from destruction. Uses cut/discarded Doctrines.
**Founding Doctrine deck (draw 1):**
- *Smoldering Core:* When any component (yours or opponent's) is destroyed, gain 2 Void.
- *Cinders of War:* When an opponent's Conflict action targets you, gain 3 Void instead of taking damage.
- *Ashfall Doctrine:* Once per game, take any 1 cut Doctrine from the draft discard into your active set.
- *From the Ashes:* When one of your components is destroyed, build a replacement anywhere adjacent for free.
- *The Salvage:* At end of each round, convert 1 destroyed component token (from any player) into 2 Void.

---

## The 25-Card Shared Doctrine Pool

Organized by type. All available every game unless Draft Pool Rotation variant is used.

### Production (5)
| Card | Text |
|---|---|
| **Iron Vein** | Each Forge adjacent to a Spire produces +2 Weight when activated. |
| **Tidal Mill** | Each Channel adjacent to another Channel produces +1 Flow. |
| **Sanctum Chain** | Each Sanctum adjacent to another Sanctum produces +1 of its mana type. |
| **Iron Bloom** | Immediately after building a component, activate it for free once. |
| **Living Root** | Channels on Earth territory produce +1 Flow AND +1 Void when activated. |

### Conversion (5)
| Card | Text |
|---|---|
| **Transmutation** | Flow converts to any mana type at 1:1 (default: 2:1). |
| **The Great Work** | While you hold all 4 mana types simultaneously, score 1 VP per round. |
| **Void Gateway** | Spend 4 Void to build a component without using a build action. |
| **Void Alchemy** | Spend 3 Void to copy any opponent's Doctrine effect for one round. |
| **Stone Memory** | Weight you spend on building converts to Void at 2:1 instead of being lost. |

### Spatial (5)
| Card | Text |
|---|---|
| **The Spider's Web** | Score 1 VP per component in your largest connected group. |
| **Expansion Law** | Your first build action each round ignores adjacency restrictions. |
| **Contested Ground** | Your components adjacent to opponent components produce +2 of their type. |
| **Deep Territory** | Score 2 VP for each different territory type you have at least 1 component on. |
| **Encirclement** | If your components fully surround an opponent's component, gain 4 VP immediately. |

### Temporal (5)
| Card | Text |
|---|---|
| **The Final Hour** | In round 5, all your components activate twice. |
| **Momentum** | Each round you produce more total mana than the previous round, gain 2 VP. |
| **Dawn Law** | In round 1, place 2 components on your first build action. |
| **Patient Iron** | Each round you take no Conflict action, gain 2 Weight. |
| **Temporal Loop** | Once per game, repeat all actions from your previous round at half mana cost. |

### Conflict (5)
| Card | Text |
|---|---|
| **Counter-Doctrine** | When an opponent's Conflict doctrine targets you, it also applies to them. |
| **Legislative Strike** | Your Edict in the Unraveling targets 2 Doctrines instead of 1. |
| **Scorched Earth** | When any opponent builds adjacent to you, they pay 1 extra mana. |
| **Raiding Party** | Once per round, steal 1 mana from an opponent with an adjacent component. |
| **Tribute Law** | Each opponent who builds adjacent to you must give you 1 mana of your choice. |

---

## Map

### Five Territory Types
| Territory | Component produces | Build cost |
|---|---|---|
| Forge Grounds | Weight (2) | 2 Weight |
| Sanctum Hills | Flow (2) | 2 Flow |
| Flow Channels | Void (1) | 2 Void |
| Ember Peaks | Ember (2) | 2 Ember |
| Void Rifts | Void + Ember (1 each) | 2 Void |

### Map Features (20 placed on specific hexes)
Selected highlights:
- **The Crown** (center): Controller scores 5 VP + names 1 opponent blocked from Conflict doctrine scoring.
- **The Fault Line** (corridors): Both adjacent components produce nothing while contested.
- **The Library** (center): Once: take any cut Doctrine into your active set.
- **The Veil's Eye** (center): Veil track advances 1 slower while you control this hex.
- **Ancient Monument** (mid-ring): Worth 1 VP per continuous round of control.
- **Founder's Mark** (mid-ring): 3 VP immediately on first claim; 2 VP at game end.
- Full list: see `docs/codex-map-territories.md`.

---

## Base Production Rates

| Component | Produces | Per activation |
|---|---|---|
| Forge | Weight | 2 |
| Sanctum | Flow | 2 |
| Channel | Void | 1 |
| Spire | Ember | 2 |

**Starting mana:** 3 Weight + 2 Flow.

**Mana conversion (default rates):**
- Adjacent types (Ember↔Flow, Flow↔Weight, Weight↔Void): 2:1
- Diagonal types (Ember↔Weight, Flow↔Void): 3:1
- Opposite types (Ember↔Void): 4:1

**Storage cap:** 12 mana total per player between rounds.

---

## The Three Phases

### PHASE 1 — CONVOCATION (~10 min)

1. Each player draws 1 Founding Doctrine from their School's 5-card deck at random. Reveal.
2. Optional: remove 5 random cards from the 25-card pool (Draft Pool Rotation variant).
3. Snake draft: 1-2-3-4-4-3-2-1-1-2-3-4-4-3-2-1. Each player picks 4 Doctrine cards.
4. Each player now has 5 Doctrines (1 Founding + 4 drafted). All face-up. Public information.

### PHASE 2 — CONSTRUCTION (~50 min, 5 rounds)

Each round, players take 3 actions each (turn order: determined by Founding Doctrine or clockwise for first round).

**Available actions:**

| Action | Cost | Effect |
|---|---|---|
| Build | 2 mana (matching terrain) | Place 1 component on adjacent empty hex |
| Activate | 1 mana | Produce from 1 component + trigger matching Doctrines |
| Convert | 1 action | Exchange mana at standard or Doctrine-modified rates |
| Conflict | 2 mana | Trigger one Conflict Doctrine against adjacent opponent |
| Advance | Free | Voluntarily advance Veil track 1 step |

**After all players act each round: Veil track advances 1 step.**

**Veil acceleration triggers (additional +1 per event):**
- Any player builds their 5th component
- Any player activates 3+ Forges in one round
- A Conflict action destroys a component

**Veil Events:** Each time the Veil advances, flip the top Veil Event card. Effect applies until start of next round.

**Doctrine activations are automatic.** When you Activate a component, every Doctrine whose trigger conditions are met fires. No choice.

### PHASE 3 — THE UNRAVELING (~20 min)

Triggered when Veil hits 0.

1. **Final Activation:** Each player activates all their components once, in turn order.
2. **Edicts:** Reverse turn order. Each player names one opponent Doctrine card — it is suspended for scoring. Legislative Strike names 2. Earth's Deep Roots founding doctrine = immune.
3. **Scoring:** All VP-generating Doctrines, map features, Spatial bonuses, 1VP per 2 mana stored.

**Tie-break:** Most active (unsuspended) Doctrines.

---

## Tension Mechanisms (T=8 target)

1. **The Veil clock** — always counting down. No way to stop it, only slow it.
2. **Depth accelerates doom** — building your 5th component, activating 3+ Forges, destroying via Conflict all advance the Veil. Better engines = faster death.
3. **The Edict** — in the Unraveling, opponents can gut your best Doctrine. Everything you built toward can be suspended in the final moment.
4. **Fault Lines** — contested hexes shut down both players' engines. Standoffs are costly.
5. **The Crown** — one hex, center of the map, worth more than anything else. Everyone sees it. Everyone wants it. Reaching it costs two corridors.

---

## Engine Depth Mechanisms (G=8 target)

1. **5 Doctrines = 5 rules** — interactions between Production × Spatial × Temporal create multi-step chains that reward planning.
2. **Iron Tokens** — Iron's Tempered founding doctrine creates permanent accumulation. Every built component is an investment that compounds.
3. **Chain activations** — The Current (Tide) and The Alloy (Iron) create cascade activations where one action triggers three or four.
4. **Component adjacency** — Iron Vein, Tidal Mill, Sanctum Chain all reward spatial positioning. The map is the engine.

---

## Range Mechanisms (R=8 target)

1. **Founding Doctrine randomness** — 5^4 = 625 starting configurations before a card is drafted.
2. **Draft pool** — with optional rotation (remove 5 of 25), 53,130 unique pool configurations.
3. **6 Schools, choose 4** — C(6,4) = 15 School combinations change map dynamics entirely.
4. **Veil Events** — 20-card deck, different subset and order each game.
5. **Modular tiles** — 6 variable tiles shuffled each game change the map's feature landscape.
6. **Total unique opening configurations: ~500 million.**

---

## Success Criteria (TIGRIS rubric)

For this review: minimum 5 earned stakes, minimum 2 on Band B or C axes, minimum 1
collision resolution, minimum 1 axis adopted or retired.

CODEX is assessed as a Euro-tradition game with strong B1 (Pipeline Gearing), B2
(Catastrophe Pressure), and C1 (Tension Budget) presence. The anchor persona is Feld
with anchor axis B1 — the pipeline/engine gearing question is central to whether CODEX
achieves its T+G+R design goal.
