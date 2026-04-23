---
name: TIGRIS Handoff — Post-RITE + UPSTAGE
slug: handoff-post-rite-upstage
stage: handoff
version: 1.0.0
rubric_version: v2.24.85
bet_version: parliament
author: TIGRIS
created: 2026-04-23
updated: 2026-04-23
type: portfolio-handoff
covers_through_game: 153 (TIGRIS) + PARLOR #1 (UPSTAGE)
covers_through_rubric: v2.24.85
---

# TIGRIS Handoff — Post-RITE + UPSTAGE

## Headline State

Rubric v2.24.85. TIGRIS corpus: 153 games (69 full-pipeline + 81 fast-track + CODEX
#151 + ESTUARY #152 + RITE #153). PARLOR pipeline launched: 1 game complete (UPSTAGE).
Pool: 32 axes — 29 adopted, 1 retired (C5), 2 live (A29, A30; 0E each), 1 live
first-earned (A31; 1E). A32 adopted this session (2E: ESTUARY + RITE). Zero
contested-watch. 144th consecutive 0-silent-retire. Biggest session event: Gap 1
E+R entered the TIGRIS corpus for the first time via RITE #153, and the PARLOR party
game factory launched with UPSTAGE.

---

## Decisions & Investments (This Session)

### CODEX PT16 — Slate 7 5p confirm
- **What:** Iron mana-dominance fix confirmed at v0.5. Wind school balanced (9 VP,
  2nd-3rd). Ash theoretically acceptable as specialist school.
- **Result:** PASS. No v0.6 triggered. Flag D-01 (Wind LS + disruption founding =
  elevated Edict leverage at 5p+) logged as non-critical.
- **Commit:** 64dbe0f

### Production work (Tasks 2-5)
- PCA numbers: already correct in panel files — no-op.
- Wind/Ash founding cards: already complete in founding-decks.html — no-op.
- Veil track spec: added to component-spec.html (SVG diagram + physical spec row).
- HOP-030 to HOP-033 generated: Pasture (Rosenberg/C3+B3), Debt (Feld/A29+C6),
  Ruin (Stegmaier/A31+D3), Prism (Vaccarino/A30+A4).
- **Commits:** 09529e0 (Veil + HOPs)

### ESTUARY #152 — TIGRIS original #30 (Lacerda/B1)
- **Anchor:** Lacerda / B1 System Gearing. HOP-027 consumed.
- **Result:** 14 earned. A32 FIRST EARN (hidden-valuations/obligation type, score 7).
  B6 +1R formal. Two new OP pairs: C1↔B4 (126th TP), B5↔B1 (127th TP).
- **Closed:** Lacerda own-anchor gap (98 games since VIADUCT #53).
- **Commit:** b3e7484

### RITE #153 — TIGRIS original #31 (Chvátil/A26) — Gap 1 E+R
- **Anchor:** Chvátil / A26 Social Contract Dependency.
- **Result:** 13 earned (6 E-band). A32 ADOPTED (2nd earn: ESTUARY + RITE). A31
  FIRST EARN ("I was Elder for one turn"; score 8). Three OPs: A25↔A26 new pair
  (129th TP), A27↔A32 new pair (130th TP), A26↔A27 confirmed 2nd-then-3rd instance.
- **Gap 1 entered:** RITE lands at E=8.5, R=8 — first TIGRIS corpus entry in Gap 1.
- **Closed:** Chvátil own-anchor gap (89 games since FRACTURE #62).
- **Rubric bump:** v2.24.84 → v2.24.85 (A32 adoption).
- **Commit:** a8bdf8c

### PARLOR pipeline — v1.0 launch
- **What:** New party game factory. 4 personas (Chvátil, Stegmaier, K-K, Vaccarino),
  8 stakes per game, E-band axis focus, shared Pool with TIGRIS.
- **Spec:** parlor/docs/parlor-pipeline.md.
- **First game:** UPSTAGE (PARLOR #1, Chvátil/A26, 4-8p, 20-30 min, weight 1.0).
  Scene-stealing party game. The DOUBLE mechanic (dual upstage) is the design's
  peak moment. PARLOR Parliament: 6 earned, A26↔A27 OP 131st TP (3rd instance —
  confirmed highly reliable pair).
- **Commit:** ca97a56

### HOP-034 Euro-Charades → RITE + UPSTAGE split
- Ideation session surveyed 10 social/party games (True American, Twister, Werewolf,
  Wavelength, The Mind, Skulls, Hanabi, Jenga, Charades, Coup) + Johnny Whoops.
- Four design layers extracted: card economy, performance, reaction cards, spatial zones.
- Three configs A/B/C generated 15 game concepts; top 6 pitched.
- RITE (Config B room game) and UPSTAGE (Config C full-stack, reframed as party game)
  emerged as the two builds. HOP-034 consumed by games/0153-rite/.

---

## Pool State Snapshot

| Axis | E | R | Status | Note |
|---|---:|---:|---|---|
| A1 Elegance | 72 | 0 | adopted | |
| A2 Decision Density | 85 | 0 | adopted | |
| A3 Interaction | 90 | 0.5 | adopted | |
| A4 Variance Calibration | 47 | 0.5 | adopted | |
| A5 Downtime/Pacing | 39 | 1.0 | adopted | |
| A6 Teachability | 99 | 0.5 | adopted | |
| A7 Emergence/Replayability | 51 | 0 | adopted | |
| B1 System Gearing | 55 | 1.5 | adopted | |
| B2 Catastrophe Pressure | 35 | 7.5 | adopted | |
| B3 Conversion Chain Depth | 23 | 7.5 | adopted | |
| B4 Info Transparency Cost | 32 | 12.0 | adopted | |
| B5 Architectural Novelty | 21 | 14.5 | adopted | |
| B6 Scoring Multiplier Dep. | 19 | 3.5 | adopted | B6 +1R formal (ESTUARY) |
| C1 Tension Budget | 88 | 0 | adopted | |
| C2 Minimum-Score Shape | 11 | 26.0 | adopted | monitoring — highest R |
| C3 Scarcity Bite | 33 | 6.0 | adopted | |
| C4 Engine-Garden Dep. | 22 | 6.5 | adopted | |
| C5 Anti-Catch-up Pressure | 0 | 3 | **RETIRED** | |
| C6 Point-Salad Incommensurab. | 32 | 10.5 | adopted | |
| C7 Action-Menu Clarity | 82 | 0 | adopted | |
| C8 First-Turn Compression | 12 | 12.5 | adopted | |
| D1 Family-to-Expert | 59 | 1.0 | adopted | |
| D2 Spatial-Interaction | 34 | 8.5 | adopted | |
| D3 Count-Robustness | 104 | 0.5 | adopted | |
| D4 Late-Game Lock-in | 47 | 2.0 | adopted | |
| A25 Emotional Arc | 14 | 0 | adopted | |
| A26 Social Contract Dep. | 22 | 0 | adopted | PARLOR signature axis |
| A27 Physical Affordance | 12 | 0 | adopted | PARLOR signature axis |
| A28 Cognitive Load Profile | 15 | 0 | adopted | |
| A29 Forgiveness Curve | 0 | 0 | **live** | 2 contested data points: ESTUARY=4, RITE=3-4. Pattern: social-physical games lean punishing. |
| A30 Strategy Ergodicity | 0 | 1.0 | **live** | 2 retire-explicit (ESTUARY + UPSTAGE). Dominant strategy concern. |
| A31 Failure Texture | 1 | 0 | **live — FIRST EARN** | RITE #153 (score 8). "I was Elder for one turn." Needs 2nd earn. |
| A32 Info Architecture Type | 2 | 0 | **ADOPTED v2.24.85** | ESTUARY (hidden-obligation type) + RITE (hidden-state + involuntary leakage). Two subtypes confirmed. |

**Pool health:** 29 adopted + 1 retired + 1 live first-earned + 2 live unearned = 33 total axes. Zero contested-watch.

---

## Hopper State

| ID | Title | Anchor | Status | Priority |
|---|---|---|---|---|
| HOP-028 | Ward | kramer-kiesling/D1+D2 | fresh | MEDIUM — K-K 90 games; community-planning spatial |
| HOP-029 | Cipher | chvatil/B5+A7 | fresh | LOW — Chvátil just completed RITE; gap now 0 games |
| HOP-030 | Pasture | rosenberg/C3+B3 | fresh | HIGH — Rosenberg 86 games; B3 chain design |
| HOP-031 | Debt | feld/A29+C6 | fresh | HIGH — Feld 89 games; A29 forgiving-end target |
| HOP-032 | Ruin | stegmaier/A31+D3 | fresh | **TOP** — Stegmaier 94 games (longest gap); A31 2nd earn target |
| HOP-033 | Prism | vaccarino/A30+A4 | fresh | HIGH — Vaccarino 84 games; A30 first earn target |

HOP-029 Cipher demoted: Chvátil just anchored RITE — gap now 0. Cipher becomes low
priority. Re-evaluate in 10+ games.

**Recommended next consumption:** HOP-032 Ruin (Stegmaier/A31+D3) — closes Stegmaier's
94-game gap AND targets A31's 2nd earn (adoption trigger). Double value.

---

## PARLOR State

| # | Title | Stage | Key result |
|---|---|---|---|
| 1 | UPSTAGE | complete | 6 earned; DOUBLE mechanic; D3 contested (4p/8p diverges) |

**PARLOR open items:**
- D3 calibration for UPSTAGE (4p vs 8p trigger density)
- Chorus role energy at 8p (extra UPSTAGE card variant)
- Natural close timing (5 sec test vs 3 sec)
- Build parlor-specific skills (parlor-concept, parlor-panel) if PARLOR reaches 3+ games
- PARLOR #2 candidate: a second party game from the Gap 1/E+R space, or a Gap 4 (T+E) attempt

---

## Confirmed Adjacency Pairs (Needs Chart Update)

**A26↔A27 is now at 3 corpus instances** (ESTUARY 126th TP, RITE 128th TP, UPSTAGE 131st TP).
This is confirmed highly reliable for social-physical game designs. **Add to adjacency
chart before next session** — currently only a "candidate" annotation in amendments files.

Other new pairs (1st instance each, candidate only):
- C1↔B4 (ESTUARY, 126th TP)
- B5↔B1 (ESTUARY, 127th TP)
- A25↔A26 (RITE, 129th TP)
- A27↔A32 (RITE, 130th TP)

---

## Open Items

1. **Add A26↔A27 to adjacency-chart.md** — 3 instances confirmed. Quick edit.
2. **gaps.md refresh** — very stale (last refreshed post-#50; now 103 games later).
   Key new coverage: Gap 1 E+R (RITE), Gap 5 T+G+R (CODEX). Gaps 2, 3, 4 still empty.
3. **A31 second earn** — needs one more game. HOP-032 Ruin (Stegmaier) targets it.
4. **A29 data — forgiving design needed** — 2 contested punishing data points. Debt
   (HOP-031/Feld) deliberately targets A29 at the forgiving end. Important for
   calibrating the axis's full range.
5. **A30 first earn** — Prism (HOP-033/Vaccarino) targets it. No earn yet.
6. **CODEX Flag D-01** — Wind LS + disruption-founding adjacency. Monitor in tabletop.
7. **PARLOR skills** — no skills built yet for parlor-concept, parlor-design,
   parlor-panel. YAGNI for now (1 game); build if PARLOR reaches 3 games.
8. **CODEX Wind + Ash card completeness** — founding-decks.html already has all 30
   cards (both Wind and Ash complete). No action needed.

---

## Next-Session Priorities

**Priority 1 (QUICK): Add A26↔A27 to adjacency-chart.md**
3 instances. 10-minute edit. Unblock this before doing anything else.

**Priority 2 (HIGH): HOP-032 Ruin (Stegmaier/A31+D3)**
Stegmaier's 94-game own-anchor gap is the longest in the corpus. Ruin targets A31
(failure texture via tide-tables anti-snowball mechanic) and D3 (simultaneous-action
structure at 2-5p). Running /tigris-concept HOP-032 → /tigris-design → /tigris-panel
closes the gap AND triggers A31 adoption (if earned).

**Priority 3 (HIGH): HOP-031 Debt (Feld/A29+C6)**
A29's 2 contested data points both cluster at the punishing end (scores 3-4). Debt
deliberately targets the forgiving end — Feld rondel where Debt tokens are the
comeback engine, not a punishment. This completes A29's range calibration. Feld
89 games since own-anchor (CAULDRON #64).

**Priority 4 (MEDIUM): HOP-030 Pasture (Rosenberg/C3+B3)**
Rosenberg 86 games since DELUGE #67. Pasture (transhumance Euro, Milk→Butter→Cheese
conversion chain) would earn B3 and potentially confirm C3↔B1 as a 2nd pair
instance. Good design, straightforward pipeline.

**Priority 5 (MEDIUM): gaps.md refresh**
Run gaps.md refresh against current corpus state (153 TIGRIS games + 1 PARLOR game).
Gap 1 E+R is now entered (RITE). Gap 5 T+G+R was entered by CODEX. Gaps 2, 3, 4
remain empty. Update ideas/gaps.md to reflect accurate current state.

**Priority 6 (MEDIUM): PARLOR #2 ideation**
UPSTAGE targets Gap 1 from the party-game angle. The next PARLOR game could target:
- Gap 4 (T+E: High Tension + Experiential) — negotiation game where social contract
  IS the tension mechanism (Chinatown × Skull archetype from the 15-game analysis)
- Or a second Gap 1 E+R design with different architecture than UPSTAGE/RITE

**Priority 7 (LOW): HOP-028 Ward and HOP-033 Prism**
K-K (90 games) and Vaccarino (84 games) both have fresh HOPs. Lower priority than
Stegmaier/Feld/Rosenberg. Run after priorities 2-4.

---

## Resume-Point Instructions

1. Read this handoff first.
2. Skim CLAUDE.md for house rules.
3. Read games/0153-rite/handoff.md for most recent TIGRIS per-game context.
4. Read parlor/games/0001-upstage/handoff.md for PARLOR context.
5. Check personas/axis-pool.md Changelog top entry (should be v2.24.85).
6. Quick-edit: add A26↔A27 to adjacency-chart.md (Priority 1 above).
7. Proceed with HOP-032 Ruin as first substantive task.

---

## Full State (Archival)

### Recent rubric changelog (last 5 entries)
- v2.24.85 — 2026-04-23 — RITE #153. A32 ADOPTED. A31 first earn. Gap 1 entered.
- v2.24.84 — 2026-04-23 — ESTUARY #152. A32 first earn. Lacerda anchor closed.
- v2.24.83 — 2026-04-22 — CODEX Parliament v2 (delta review). 21 earned. A1 hold→earned.
- v2.24.82 — 2026-04-22 — CODEX Parliament v1. 15 earned. 4 contested.
- v2.24.81 — 2026-04-21 — High Society #150. CORPUS COMPLETE (150 games).

### Recent TRACKER rows (last 5)
- v2.24.85 | RITE #153 | Chvátil/A26 | 13 earned | 3 OPs | A32 adopted; A31 first earn; Gap 1
- v2.24.84 | ESTUARY #152 | Lacerda/B1 | 14 earned | 2 OPs | A32 first earn; B6 +1R
- v2.24.83 | CODEX v2 delta | Knizia/A1 | 21 earned | 3 | A1 hold→earned; 4 contested resolved
- v2.24.82 | CODEX v1 | Feld/B1 | 15 earned | 4 collisions | 4 contested
- v2.24.81 | High Society #150 | Knizia/C1 | 7 earned | C1↔A1 | CORPUS COMPLETE

### PARLOR state
- PARLOR #1 UPSTAGE complete. 6 earned. A26↔A27 OP 131st TP (3rd instance).
- Pipeline spec: parlor/docs/parlor-pipeline.md v1.0.
- No PARLOR-specific skills built yet.

### Key session stats
- Games completed: PT16 (CODEX confirm) + ESTUARY #152 + RITE #153 + UPSTAGE PARLOR #1
- New axes adopted: A32 (Information Architecture Type)
- First earns: A31 (Failure Texture — needs 2nd)
- Own-anchor gaps closed: Lacerda (ESTUARY), Chvátil (RITE)
- New factories launched: PARLOR
- 0-silent-retire streak: 144 consecutive
- Total OP collisions: 131 (128th-131st this session)
- New confirmed adjacency pairs: A26↔A27 (3rd instance — add to chart)
- New candidate pairs: C1↔B4, B5↔B1, A25↔A26, A27↔A32
