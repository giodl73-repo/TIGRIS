---
title: CODEX — Visual Identity (Direction 4: Gilded Void)
created: 2026-04-22
status: canonical
---

# CODEX Visual Identity

**Direction: Gilded Void**
Warm dark backgrounds + gilt accents + Cinzel serif names + Space Grotesk rules text.
The collector's edition Euro aesthetic — premium, legible, arcane without being baroque.

---

## Reference file
`docs/mockup-direction4.html` — open in browser for full mockup.

---

## Colour System

| Token | Hex | Use |
|---|---|---|
| `--bg` | #13100C | Page / table background |
| `--surface` | #1E1810 | Card base, board base |
| `--surface2` | #2A2218 | Elevated surfaces, insets |
| `--gold` | #C9941A | Primary accent, borders, Veil track |
| `--gold-soft` | #DBA830 | Hover, highlights |
| `--text-hi` | #F0E8D8 | Card names, primary text |
| `--text-mid` | #C0B090 | Rule text |
| `--text-lo` | #786040 | Labels, tags, meta |

### School Colours
| School | Hex | Character |
|---|---|---|
| Flame | #FF5020 | Warm red-orange |
| Tide | #1A9AFF | Bright blue |
| Iron | #8FA0B8 | Steel blue-grey |
| Earth | #50C060 | Warm green |
| Wind | #B0A0FF | Soft lavender |
| Ash | #FF7040 | Ember orange |

### Mana Colours
| Mana | Hex |
|---|---|
| Ember | #FF8C42 |
| Flow | #42BEFF |
| Weight | #A8B4CC |
| Void | #9060FF |

---

## Typography

| Role | Font | Weight | Size | Notes |
|---|---|---|---|---|
| Card names | Cinzel | 700 | 15px | Letter-spacing 0.03em |
| Section headers | Cinzel | 600 | 8px | Letter-spacing 5px, uppercase |
| Rule text | Space Grotesk | 400 | 11px | Line-height 1.55 |
| Labels / tags | Space Grotesk | 600–700 | 7px | Letter-spacing 3px, uppercase |
| Veil numbers | Space Mono | 700 | 16px | Monospace — deliberate contrast |
| School names on boards | Cinzel | 700 | 14px | |

---

## Card Anatomy

```
┌─────────────────────────────┐
│ [3px School-colour bar]      │  ← gradient, left to right
├──────────────────────────────┤
│ TYPE LABEL          [sigil]  │  ← Space Grotesk 7px caps
│ Card Name  (Cinzel 15px 700) │
├──────────────────────────────┤
│ WHEN  (Space Grotesk 7px)    │  ← gold-tinted label
│ Trigger text (11px)          │
│ ─────────────────────────── │  ← 1px border rgba(gold,0.18)
│ THEN  (Space Grotesk 7px)    │
│ Effect text (11px)           │
├──────────────────────────────┤
│ ○ ○ ○ ○ ○  (VP pips)   tag  │  ← gold pip circles
└─────────────────────────────┘
```

VP pip circles: 9px, border `rgba(gold, 0.35)`. Filled = `#C9941A`.

---

## School Sigils

Each School has a geometric SVG sigil:
- **Flame** — triangle with inner triangle + circle base
- **Tide** — interlocking ellipses (horizontal + vertical) + center circle
- **Iron** — hexagon with inner hexagon + rotated square center
- **Earth** — circle with inner ring + root lines below + center fill
- **Wind** — flowing S-curves crossing at center
- **Ash** — diamond with inner diamond + open center circle

Sigils render in School colour at full size. A faint gilt ring (#C9941A, 0.3 opacity) surrounds them at all sizes. On dark card backgrounds use School colour at 60% opacity for watermark use.

---

## Hex Map Terrain

Warm dark gradients — all terrain is dark but tonally distinct:

| Terrain | Base gradient |
|---|---|
| Forge Grounds | #2C2416 → #3A2E1C (warm brown) |
| Sanctum Hills | #162030 → #1E3040 (cool blue-dark) |
| Flow Channels | #0C1E34 → #143050 (deep blue) |
| Ember Peaks | #2E1208 → #401A0C (red-dark) |
| Void Rifts | #14101E → #1E1428 (purple-dark) |

Special hexes (The Crown, The Fault Line, The Library etc.) get a gold drop-shadow filter: `drop-shadow(0 0 6px rgba(201,148,26,0.5))`.

---

## Component Tokens

Four component types per School (6 each):
- **Forge** — heavy hexagonal form, School colour fill, gilt border
- **Sanctum** — domed octagonal form, School colour fill, gilt border
- **Channel** — elongated oval, School colour fill, gilt border
- **Spire** — tall triangle, School colour fill, gilt border

All components have a subtle gilt inner-ring emboss. Iron School components have Iron Token slots (small gilt circles on the token face).

---

## Veil Track

10 steps, left to right, warm progression:
- Steps 8–10: `--surface` background, `--text-lo` numeral
- Steps 4–7: warm amber tint, `#FFA050` numeral
- Steps 1–3: red-dark tint, `#FF6050` numeral
- Current step: `--gold-glow` background, `--gold-soft` numeral, 16px box-shadow gold glow

---

## Next design priorities
1. Full 25-card doctrine pool in D4 style
2. School board full layout (Founding Doctrine slot, component track, mana storage)
3. Box art concept (The Crown hex at center, 4 School sigils at corners)
4. Physical component specification (token sizes, materials)
