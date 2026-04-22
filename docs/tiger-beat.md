---
title: TIGER BEAT — Nine-Dimension Game Design Framework
created: 2026-04-22
source: PCA on 150-game TIGRIS corpus (axis-matrix.csv, commit d855f7c)
status: canonical
---

# TIGER BEAT

Nine empirically-derived dimensions of game design space, discovered via PCA on 150 games
× 28 axes. TIGER covers 72% of variance (primary structure). BEAT covers the remaining 28%
(secondary texture).

---

## TIGER — Primary Framework (72% of variance)

| Dim | Name | PC | Variance | Key axes | What it measures |
|---|---|---|---|---|---|
| **T** | Tension | PC7 | 5% | C1, B2, B5, C2 | Stakes, pressure, fear of losing |
| **I** | Interaction | PC3 | 12% | A3, B4, B5 | Direct conflict, hidden info, player targeting |
| **G** | Game-spectrum | PC1 | 34% | B1/B3/C4 ↔ A1/A6/C7 | Bipolar: Architecture (high) ↔ Elegance (low) |
| **E** | Experiential | PC2 | 15% | A26, A27, A28 | Social contract, physical affordance, cognitive load |
| **R** | Range | PC5 | 6% | A4, C7, D3 | Variability, replayability, strategic breadth |

**G is bipolar:** a single axis where high G = deep engine/architecture (Lisboa, On Mars),
low G = elegant/minimal (Crokinole, Love Letter). G and E (elegance) are opposite ends of
the same spectrum (r = -0.738 between A1 and B1).

---

## BEAT — Secondary Framework (28% of variance)

| Dim | Name | PC | Variance | Key axes | What it measures |
|---|---|---|---|---|---|
| **B** | Breadth | PC8 | 4% | D2, D3, C7 | Solo mode, count-robustness, multi-mode flexibility |
| **EA** | Emotional Arc | PC6 | 5.5% | A25 | Narrative journey vs pure mechanical replayability |
| **A** | Accessibility | PC4 | 7% | D1, D4 | Onboarding ease, first-turn footprint, language independence |
| **TX** | Texture | PC9-12 | ~12% | C3, B3, C6 | Scarcity bite, pressure shape, floor mechanics |

---

## Key corpus findings (real numbers, 150 games)

**Dimension distributions:**
- T: mean=4.9, sd=1.0 — relatively compressed; most games have moderate tension
- I: mean=5.5, sd=1.5 — widest spread in TIGER; big range from solo to fighting games
- G: mean=3.6, sd=1.7 — corpus skews elegant (fast-track games are lighter)
- E: mean=4.2, sd=1.1 — experiential axes compressed; new axes, limited range yet
- R: mean=6.0, sd=1.0 — corpus skews replayable (selection bias toward good games)

**Corpus leaders:**
- T: Space Alert (7.5), Tigris & Euphrates (7.2), High Society (7.2), Bridge (7.2)
- I: Letters from Whitechapel (8.7), Bridge (8.7), The Mind (8.3), Space Alert (8.3)
- G (high arch): On Mars (7.8), Weather Machine (7.3), Vinhos Deluxe (7.3), The Gallerist (7.3)
- E: Secret Hitler (5.7), Hanabi (5.7), Codenames (5.7)
- R: Welcome To (8.0), Sagrada (8.0), Qwixx (8.0), Crokinole (8.0)

**Biggest design gaps (EMPTY — 0 games in corpus):**
- E+R high: Experiential AND high range — no game is both social/physical AND endlessly replayable
- G+E high: Deep architecture AND high experiential — no complex Euro with strong social/physical character
- G+R high: Deep architecture AND high range — no engine-heavy game with massive strategic variety
- T+E high: High tension AND high experiential — no pressure game with strong social contract

**Most isolated games (unique fingerprints):**
1. Space Alert — T=7.5, I=8.3, G=1.2, E=3.7, R=3.7 (gap=3.5 to nearest neighbour)
2. Pandemic Legacy — T=6.5, I=6.7, G=3.7, E=9.0, R=3.7 (gap=3.3, EA=9.0 singleton)
3. Through the Ages — T=6.8, I=5.3, G=5.3, E=3.3, R=5.3 (gap=2.8)
4. Quacks of Quedlinburg — T=6.2, I=3.0, G=2.5, E=3.0, R=7.7 (gap=2.2)

---

## Source files

| File | Purpose |
|---|---|
| `data/axis-matrix.csv` | Single authoritative matrix: #1-69 honest rescores + #70-150 from record.md |
| `data/axis-residuals.csv` | Weight-normalized residuals |
| `data/pca-results.txt` | Raw PCA output (PC loadings, correlations) |
| `scripts/build-rescored-matrix.py` | Builds axis-matrix.csv from source files |
| `scripts/pca-analysis.py` | Runs PCA on axis-matrix.csv |
| `scripts/tiger-beat-gap.py` | TIGER BEAT gap analysis and design opportunity finder |

**Do not use:** `data/archive/axis-matrix-v2.csv`, `data/archive/axis-residuals-v2.csv`
— these are archived broken files, superseded by the above.
