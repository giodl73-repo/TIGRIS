---
name: TIGRIS Portfolio Handoff — Post-CODEX Design Session
slug: post-codex-design
stage: handoff
version: 1.0.0
rubric_version: v2.24.83
bet_version: parliament
author: TIGRIS
created: 2026-04-22
updated: 2026-04-22
type: portfolio-handoff
covers_through_game: 151
covers_through_rubric: v2.24.83
---

# TIGRIS Portfolio Handoff — Post-CODEX Design Session

**Session snapshot: 2026-04-22. Rubric v2.24.83. 151 games complete (150 reviews + 1 CODEX original in playtesting).**

---

## Headline

This session completed the most scope-dense work since the v2.0 restructure: the 150-game corpus was finalized and analyzed via real PCA, producing the TIGER BEAT nine-dimension framework (docs/tiger-beat.md); the resulting design-gap analysis identified five empty TIGER-space regions and birthed CODEX (games/0151-codex/) — a Schools of Magic engine-builder targeting the T+G+R gap; CODEX ran 15 playtests (v0.1–v0.5), two full Parliament reviews (v1: 15 earned, v2: 21 earned, all four v1-contested axes resolved), and a complete print-ready design system (Direction 4 Gilded Void, seven HTML asset files). The session ends with CODEX design system complete, CODEX Parliament v2 passed, and three fresh hopper entries awaiting next-session originals.

---

## Session Investments

### 1. TIGER BEAT Framework (docs/tiger-beat.md)
- **What:** PCA on the completed 150-game × 28-axis corpus, producing nine empirically-derived design dimensions.
- **TIGER** (72% variance): T=Tension (PC7, 5%), I=Interaction (PC3, 12%), G=Game-spectrum/bipolar Architecture↔Elegance (PC1, 34%), E=Experiential (PC2, 15%), R=Range (PC5, 6%).
- **BEAT** (28% variance): B=Breadth (PC8, 4%), EA=Emotional Arc (PC6, 5.5%), A=Accessibility (PC4, 7%), TX=Texture (PC9-12, ~12%).
- **Key finding:** G is bipolar (A1↔B1 r=-0.738). PC1 alone is 34%; 6 components for 80% variance.
- **Source files:** data/axis-matrix.csv, data/pca-results.txt, scripts/build-rescored-matrix.py, scripts/pca-analysis.py, scripts/tiger-beat-gap.py.

### 2. Data Cleanup (axis-matrix.csv)
- Rebuilt axis-matrix.csv from 150 record.md files (games #1–69 honest rescores + #70–150 fast-track).
- Fixed broken regex in build-rescored-matrix.py that blocked fast-track score extraction.
- Archived stale files: data/archive/axis-matrix-v2.csv, data/archive/axis-residuals-v2.csv.

### 3. Design Gap Analysis (docs/tiger-beat-gaps.md)
- 5 empty TIGER-space regions identified from real corpus distributions.
- Gaps: E+R high, G+E high, G+R high (CODEX's target), T+E high, plus T+G+R multi-gap.
- Each gap entry includes meetup-crowd inspiration notes.

### 4. CODEX Designed, Playtested, Parliament-Reviewed (games/0151-codex/)
- **Concept:** T+G+R gap game. Schools of Magic engine-builder on shared hex map. Anchor: Knizia/A1.
- **Playtests:** 15 total across v0.1–v0.5. Key iterative fixes: Iron Vein trigger repair, cap-12 on doctrine accumulation, token-banking, 50% Deep Roots rule, simultaneous activation.
- **Parliament v1 (games/0151-codex/panel/SUMMARY.md):** 15 earned stakes. 4 contested axes (A1 hold-explicit, A5, A6, C7).
- **Parliament v2 (games/0151-codex/panel/SUMMARY-v2.md):** 21 earned stakes across all four bands (A:8, B:5, C:6, D:2). All four v1-contested axes resolved to earned. First intra-axis collision in TIGRIS corpus (M6: A1 Knizia ↔ A1 K-K, CR 5-2 for Knizia). B6 new earn (token-banking × Edict-stripping × FH/TL multiplier stack). 4 v1-contested axes cleared — strongest contested-resolution event in corpus.
- **Success criteria:** PASS on all four (≥5 earned: 21; ≥2 Band B/C: 11; ≥1 collision: 3 with 4 credits; ≥1 adoption/retirement: C2+1R formal, A1 hold→earned, 4 v1-contested resolved).
- **Notable precedents:** Knizia's first hold-explicit→earned transition; delta-review-as-a-mode formalized (halve prestige cost for re-staking previously-earned axes; holds cannot expire silently across reviews).

### 5. CODEX Design System — Direction 4 Gilded Void (games/0151-codex/design/)
All seven assets delivered as print-ready HTML:
- `cards/doctrine-pool.html` — 25 shared Doctrine cards
- `cards/founding-decks.html` — 30 Founding cards, 6 Schools
- `cards/veil-events.html` — 20 Veil Event cards
- `boards/school-boards.html` — 6 School boards
- `boards/modular-tiles.html` — 12 modular map tiles
- `components/component-spec.html` — complete component specification
- `box/box-art.html` — box art

---

## Pool State (post-CODEX v2, rubric v2.24.83)

Full state in personas/axis-pool.md Changelog top entry (v2.24.81 = last full-batch entry; CODEX v2 adds incremental earns).

**Current rubric version:** v2.24.83 (CODEX v2 delta). TRACKER.md top row: v2.24.81 (High Society #150, corpus complete).

### Headline numbers
- **Pool size:** 32 axes (28 original + A29–A32 new v2.25 axes, all live).
- **Adopted:** A1–A7, B1–B6, C1–C4, C6–C8, D1–D4, A25–A28 (A28 adopted v2.24.11). C5 retired.
- **Live (not yet adopted):** A29, A30, A31, A32.
- **Most-earned:** D3 (104E, first to 100E), A2 (84E), C1 (85E career high), A3 (88E), C7 (80E).
- **Highest retire-explicit:** C2 (27R career high after CODEX v2 +1R formal). Diagnostic-refute watch ongoing.
- **B4 monitoring:** 31E / 11.5R; B4 recovered post-batch (31E now vs 11E at post-matrix). No longer in dormancy-watch.
- **C8 monitoring:** 12E / 12.5R; recovered post-batch. No longer in dormancy-watch.

### New v2.25 axes (A29–A32)
- A29 Forgiveness Curve (Feld primary) — 0E/0R, live
- A30 Strategy Ergodicity (Vaccarino primary) — 0E/0R, live
- A31 Failure Texture (Stegmaier primary) — 0E/0R, live
- A32 Information Architecture Type (Lacerda/Chvátil primary) — 0E/0R, live

---

## Hopper State

| ID | Title | Status | Anchor | Notes |
|---|---|---|---|---|
| HOP-027 | Estuary | **fresh** | lacerda/B1+B4 | Information-asymmetric port-trading; B4 now recovered but still under-earned for Lacerda anchor |
| HOP-028 | Ward | **fresh** | kramer-kiesling/D1+D2 | Community-planning spatial; K-K 2nd own-anchor candidate |
| HOP-029 | Cipher | **fresh** | chvatil/B5+A7 | Structural information-asymmetry deduction; Chvátil 2nd own-anchor candidate |

3 fresh entries. CODEX (HOP consumed → games/0151-codex/) was not a hopper entry — it emerged directly from TIGER BEAT gap analysis.

---

## Persona Anchor Status

| Persona | Last own-anchor | Games since | Priority |
|---|---|---|---|
| Knizia | CODEX #151 (anchor persona) | 0 | Just completed |
| Rosenberg | DELUGE #67 | 84 | HIGH — longest gap; needs new HOP |
| Feld | CAULDRON #64 | 87 | HIGH — very long gap; needs new HOP |
| Lacerda | VIADUCT #53 | 98 | HIGH — longest gap in corpus; HOP-027 Estuary ready |
| Chvátil | FRACTURE #62 | 89 | HIGH — long gap; HOP-029 Cipher ready |
| K-K | MERIDIAN #63 | 88 | HIGH — long gap; HOP-028 Ward ready |
| Stegmaier | CHORUS #59 | 92 | HIGH — long gap; needs new HOP |
| Vaccarino | MATRIX #69 | 82 | HIGH — long gap; needs new HOP |

Note: games #70–150 were fast-track reviews (no originals); all 7 non-Knizia personas have been without an own-anchor original for 80+ games. The 150-game fast-track batch skipped originals entirely. Catching up will require sustained originals pipeline.

---

## Open Items

1. **CODEX v0.5 final confirmatory playtest** — Parliament v2 passed, but two design-side flags remain open: (a) Slate 7 test to confirm Iron mana-dominance is fixed by v0.5 changes; (b) verify cap-12 doesn't over-constrain Wind/Ash schools in 5p. Parliament does not adjudicate; design team owns.
2. **craftworks-research papers — real PCA numbers update** — Papers referenced simulated PC1=50.0% and PC1=81.3%. Real numbers are: PC1=34.1% (G-spectrum), 6 components for 80% variance. Papers in games/0151-codex/panel/ need revision pass with real numbers before any external audience.
3. **CODEX 6-school expansion** — Wind and Ash schools exist in concept but are missing founding-doctrine art in `cards/founding-decks.html`. 4 of 6 schools complete; Wind (Tempest Warden theme) and Ash (Mortuary Engineer theme) need their 5 founding cards each.
4. **CODEX Veil track physical spec** — component-spec.html covers tokens/cards/tiles but Veil track (the arc-pacing spine) lacks a physical production spec (dimensions, printing notes, layering).
5. **v2.25 axes (A29–A32) need first earns** — four new live axes with 0E. The corpus's next 5-10 games should include targeted drafts to establish baselines.
6. **Personas without own-anchor for 80+ games** — Rosenberg, Feld, Lacerda, Chvátil, K-K, Stegmaier, Vaccarino all need originals. Three fresh HOPs (027/028/029) cover Lacerda, K-K, Chvátil. Rosenberg, Feld, Stegmaier, Vaccarino need new HOPs generated.

---

## OP-Collision Corpus

CODEX v2 added 3 collisions (125th TP total):
- M2 B1↔C4 Decisive 7-1 (124th TP; 2nd B1↔C4)
- M3 A5↔A6 Standard 6-2 (125th TP; first A5↔A6 pair)
- M6 A1↔A1 CR 5-2 for Knizia (first intra-axis collision in corpus; no TP number — collision-review sub-type)

**Intra-axis collision precedent established.** Protocol: absorbed into standard CR; prestige token redemption on win.

---

## 0-Silent-Retire Streak

**142 consecutive 0-silent-retire** (through High Society #150; CODEX #151 adds 0 silent retires). Streak continues.

---

## Next-Session Priorities (ranked)

**Priority 1 (HIGH): CODEX v0.5 final playtest — Slate 7 Iron/Wind/Ash confirm**
Run Slate 7 playtest targeting: (a) Iron mana-dominance fix confirmed at 5p; (b) Wind and Ash school balance vs. Iron under cap-12. If Slate 7 clean, CODEX design phase complete. If Iron still dominates, v0.6 triggered.

**Priority 2 (HIGH): HOP-027 Estuary (Lacerda/B1+B4)**
Lacerda is the longest-gap persona (98 games since VIADUCT #53). Estuary's information-asymmetric port-trading targets B1+B4. B4 is now recovered (31E) but Lacerda's own-anchor gap is the primary driver. Run /tigris-concept HOP-027 → /tigris-design → /tigris-panel.

**Priority 3 (MEDIUM-HIGH): craftworks-research papers — real PCA numbers**
Update all references from simulated PC1=81.3%/50.0% to real PC1=34.1%; update "6 components for 80% variance" stat; verify corpus-leader tables match real data/pca-results.txt output. This is a targeted find-replace pass across papers in games/0151-codex/panel/.

**Priority 4 (MEDIUM): CODEX design — Wind + Ash founding cards**
Complete the 6th-school founding decks. 5 founding cards per school: Wind (Tempest Warden) and Ash (Mortuary Engineer). Use cards/founding-decks.html as template. Add to component-spec.html.

**Priority 5 (MEDIUM): CODEX Veil track physical spec**
Add Veil track section to components/component-spec.html: dimensions (recommend 280mm × 60mm track strip), token sizes, print notes, layering guide.

**Priority 6 (MEDIUM): Generate new HOPs for Rosenberg, Feld, Stegmaier, Vaccarino**
Run /tigris-ideate targeting each persona's gap. Suggested axis targets: Rosenberg/C3+B3 (feeding + chain), Feld/A29+C6 (forgiveness + incommensurable scoring), Stegmaier/A31+D3 (failure texture + count-robustness), Vaccarino/A30+A4 (strategy ergodicity + variance calibration). This clears the 4-persona own-anchor backlog.

---

## Key Milestones Reached This Session

- **Corpus complete at 150 games** (v2.24.81; High Society #150).
- **TIGER BEAT framework established** — first empirically-derived design theory from TIGRIS corpus.
- **CODEX Parliament v2 passed** — 21 earned stakes; 4 v1-contested resolved; first intra-axis collision.
- **First delta-review-as-a-mode** — CODEX is the first TIGRIS game with two complete Parliament reviews. Protocol formalized in SUMMARY-v2.md.
- **First hold-explicit→earned transition** — Knizia/A1 at CODEX v0.5 via CR 5-2.
- **A6 reaches 100E** (second axis to do so, after D3).
- **B5 and B6 hit 20E milestone.**
- **A7 hits 50E milestone.**
