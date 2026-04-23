---
title: CODEX — Modular Map Tile System
created: 2026-04-22
game: CODEX
status: design complete
---

# CODEX Modular Map Tile System

The modular map replaces the fixed 48-hex board with 12 interlocking hex tiles (4 hexes each
= 48 hexes total). Each game the 6 Variable tiles shuffle, creating a different map each time.

---

## System Rules

- **12 tiles total**, each containing exactly **4 hexes** (48 hexes total, matching base game)
- **6 School tiles** (fixed starting positions, never shuffled): 1 per School, placed in corners/edges
- **6 Variable tiles** (shuffled each game, placed in non-corner positions)
- For **4-player** games: use all 6 Variable tiles
- For **3-player** games: remove 1 Variable tile randomly before placement; leave that position empty

**Tile orientation:** Variable tiles may be rotated freely (any of 6 hex orientations is valid).
School tiles are placed in a fixed orientation so home terrain points inward.

---

## Map Assembly Rules

**Corner placement** is determined by which Schools are playing:

| Schools in game | Corner assignment |
|---|---|
| Iron | Northwest corner |
| Tide | Northeast corner |
| Flame | Southeast corner |
| Earth | Southwest corner |
| Wind | North edge (between NW and NE corners) |
| Ash | South edge (between SE and SW corners) |

For 4-School games (standard), the 4 active School tiles go to their corners. The 6 Variable
tiles fill the 6 remaining positions: 2 center-row positions and 4 edge-corridor positions.

**Shuffle → place → rotate:** Shuffle the 6 Variable tiles face-down. Draw and place one at a
time, rotating each freely before locking. No tile is placed twice.

---

## SCHOOL TILES (6 tiles, never shuffled)

*Each School tile: 3 hexes of home terrain + 1 neutral connector hex*
*No special features on School tiles — they are stable ground.*

---

### Tile S1 — IRON TILE
**Position:** Northwest corner

| Hex | Terrain | Notes |
|---|---|---|
| S1-A | Forge Grounds | Home hex — Iron's natural production |
| S1-B | Forge Grounds | — |
| S1-C | Forge Grounds | — |
| S1-D | Sanctum Hills | Neutral connector (faces inward toward Variable tiles) |

**Strategic character:** A dense production corner with no flow, making Iron dependent on the
Variable tile it neighbors for conversion options.

---

### Tile S2 — TIDE TILE
**Position:** Northeast corner

| Hex | Terrain | Notes |
|---|---|---|
| S2-A | Flow Channel | Home hex — Tide's natural terrain |
| S2-B | Flow Channel | — |
| S2-C | Flow Channel | — |
| S2-D | Sanctum Hills | Neutral connector (faces inward) |

**Strategic character:** Flow-heavy corner that incentivizes Tide to draft heavily from
Production Doctrines for Channels. The Sanctum connector is a natural Tier-2 expansion target.

---

### Tile S3 — FLAME TILE
**Position:** Southeast corner

| Hex | Terrain | Notes |
|---|---|---|
| S3-A | Ember Peaks | Home hex — Flame's natural production |
| S3-B | Ember Peaks | — |
| S3-C | Ember Peaks | — |
| S3-D | Forge Grounds | Neutral connector (faces inward) |

**Strategic character:** Pure aggression corner. Three Ember Peaks and a Forge connector make
Flame's early economy entirely Ember-dependent — and fast. Forge connector gives Flame early
access to Weight terrain if it expands inward.

---

### Tile S4 — EARTH TILE
**Position:** Southwest corner

| Hex | Terrain | Notes |
|---|---|---|
| S4-A | Sanctum Hills | Home hex — Earth's natural resilience |
| S4-B | Sanctum Hills | — |
| S4-C | Sanctum Hills | — |
| S4-D | Flow Channel | Neutral connector (faces inward) |

**Strategic character:** Earth's corner rewards patient building — Sanctum Hills generate Flow,
and Earth's Founding Doctrine bonuses apply here. Flow connector invites Doctrine-based
conversion strategies rather than raw production.

---

### Tile S5 — WIND TILE
**Position:** North edge (expansion only)

| Hex | Terrain | Notes |
|---|---|---|
| S5-A | Void Rifts | Home hex — Wind's volatile footing |
| S5-B | Void Rifts | — |
| S5-C | Ember Peaks | Secondary home terrain |
| S5-D | Flow Channel | Neutral connector (faces inward) |

**Strategic character:** Wind's tile is unstable even at home — 2 Void Rifts provide Void
production but complicate early builds. The Ember Peaks and Flow connector push Wind toward
the center early to establish disruption positions.

---

### Tile S6 — ASH TILE
**Position:** South edge (expansion only)

| Hex | Terrain | Notes |
|---|---|---|
| S6-A | Void Rifts | Home hex — Ash's reclamation ground |
| S6-B | Void Rifts | — |
| S6-C | Forge Grounds | Secondary home terrain |
| S6-D | Sanctum Hills | Neutral connector (faces inward) |

**Strategic character:** Ash's Void Rifts provide consistent Void income. The Forge Grounds
hex is Ash's anchor for early Weight production, enabling Build actions before Ash acquires
enough Void from destruction. Sanctum connector faces inward toward Variable tiles.

---

## VARIABLE TILES (6 tiles, shuffled each game)

*Each Variable tile: 4 hexes of mixed terrain + exactly 1 special feature*
*Special feature is placed on 1 of the tile's hexes (specified below).*

---

### Tile V1 — THE CONTESTED CROSSING
**Terrain:** Forge Grounds · Flow Channel · Flow Channel · Sanctum Hills
**Special feature:** *The Fault Line* — the hex border between Forge Grounds and the first
Flow Channel is a Fault Line. Components on either side of this border cannot produce while
any opponent has a component on the other side.

**Strategic character:** A production choke point. Whoever controls both sides of the Fault
Line border can shut down an opponent's output without a Conflict action — but they pay in
Veil advancement if that standoff holds.

---

### Tile V2 — THE ANCIENT FORGE
**Terrain:** Forge Grounds · Forge Grounds · Ember Peaks · Void Rifts
**Special feature:** *The Founder's Mark* (on the Forge Grounds hex closest to center) —
the player who first builds here gains a permanent marker: +1 Weight production on this hex
for the rest of the game, regardless of component type placed here.

**Strategic character:** A Weight-and-Ember power tile. The Founder's Mark rewards the
player who claims the center Forge early — making this tile a reliable early Conflict magnet.
Ash benefits from the adjacent Void Rift; Iron fights hardest for the Forge Grounds.

---

### Tile V3 — THE VOID SPIRAL
**Terrain:** Void Rifts · Void Rifts · Void Rifts · Ember Peaks
**Special feature:** *The Veil's Eye* (on the central Void Rift hex) — the player who
controls The Veil's Eye slows the Veil track by 1 step at the end of each round.

**Strategic character:** The most powerful single hex in the game, but placed in a sea of
Void Rifts that costs significant mana to access. Wind and Ash both covet The Veil's Eye.
In games without Wind or Ash, watch for Iron or Earth making a late play to seize it.

---

### Tile V4 — THE CONFLUENCE POINT
**Terrain:** Flow Channel · Sanctum Hills · Sanctum Hills · Flow Channel
**Special feature:** *The Deep Spring* (on one Flow Channel hex) — this hex's occupying
component produces +2 Flow when activated, regardless of component type.

**Strategic character:** A Flow-dense tile that enables Tide's engine and powers Conversion
Doctrines. The Deep Spring makes this tile worth fighting over for any School running
Transmutation or Void Alchemy. Two Sanctum Hills adjacent to two Flow Channels creates
excellent terrain for Sanctum Chain and Tidal Mill Doctrines.

---

### Tile V5 — THE EMBER CROWN
**Terrain:** Ember Peaks · Ember Peaks · Forge Grounds · Sanctum Hills
**Special feature:** *The Crown* (on the higher Ember Peaks hex) — scores 5 VP at game
end for whoever controls it. Most valuable scoring hex on the map.

**Strategic character:** The VP magnet of the map. The Crown is worth fighting over regardless
of School — but Flame's natural Ember production makes establishing on The Crown early trivial
for them. Expect The Crown to generate more Conflict actions per game than any other single hex.

---

### Tile V6 — THE DEEP ROOTS
**Terrain:** Sanctum Hills · Flow Channel · Forge Grounds · Void Rifts
**Special feature:** *The Ancient Monument* (on the Sanctum Hills hex) — scores 3 VP at
game end for whoever controls it. Unlike The Crown, this Monument also grants: "Your Doctrines
cannot be suspended by Edicts while you control this hex."

**Strategic character:** Earth's dream hex — the Monument's Edict immunity stacks with Earth's
Founding Doctrine immunity, making Earth nearly invulnerable to Edicts if it controls this tile.
For all other Schools, the Monument is a defensive tool that partially mirrors Earth's Founding
Doctrine and is worth contesting even without Earth in the game.

---

## FULL TILE REFERENCE SUMMARY

| # | Name | Terrain Mix | Special Feature | Hex |
|---|---|---|---|---|
| S1 | Iron Tile | Forge/Forge/Forge/Sanctum | None | — |
| S2 | Tide Tile | Flow/Flow/Flow/Sanctum | None | — |
| S3 | Flame Tile | Ember/Ember/Ember/Forge | None | — |
| S4 | Earth Tile | Sanctum/Sanctum/Sanctum/Flow | None | — |
| S5 | Wind Tile | Void/Void/Ember/Flow | None | — |
| S6 | Ash Tile | Void/Void/Forge/Sanctum | None | — |
| V1 | The Contested Crossing | Forge/Flow/Flow/Sanctum | The Fault Line | Forge-Flow border |
| V2 | The Ancient Forge | Forge/Forge/Ember/Void | The Founder's Mark | Center Forge hex |
| V3 | The Void Spiral | Void/Void/Void/Ember | The Veil's Eye | Central Void hex |
| V4 | The Confluence Point | Flow/Sanctum/Sanctum/Flow | The Deep Spring | Flow hex |
| V5 | The Ember Crown | Ember/Ember/Forge/Sanctum | The Crown | Upper Ember hex |
| V6 | The Deep Roots | Sanctum/Flow/Forge/Void | The Ancient Monument | Sanctum hex |

---

## DESIGN NOTES

**Why 4 hexes per tile (not 3 or 7)?**
Four hexes keeps tiles compact enough to rotate without breaking adjacency logic. At 4 hexes
each, the 12 tiles produce 48 hexes — identical to the base game's fixed map. Players who
know the base game can make the switch without relearning scale.

**Why School tiles have no special features:**
School tiles should feel stable. The chaos is in the Variable tiles. If your home corner
had a Fault Line or a Crown, setup variance would overshadow all draft decisions. School
tiles give each player a known, predictable anchor — the Variable tiles provide the surprise.

**Why 6 Variable tiles instead of more?**
With 6 Variable tiles, the number of distinct maps is 6! × (6 orientations each) = 720 ×
6^6 = approximately 3.4 million configurations. That is more than enough for infinite
replayability. Adding more tiles would increase setup time without strategic payoff.

**3-player balance note:**
Removing 1 Variable tile randomly compresses the map appropriately for 3 players. The
missing tile leaves a gap — do not attempt to fill it. The gap is intentional: it prevents
one player from being geographically central when three Schools occupy three corners.
