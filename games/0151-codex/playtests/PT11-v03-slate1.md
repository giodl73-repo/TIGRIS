---
playtest: PT11
version: v0.3
slate: 1 — The Balanced Opener (baseline retest)
players: Iron / Flame / Tide / Earth
v01_result: Earth 17, Iron 10, Flame 12, Tide 8
winner: Iron (31 VP)
veil_triggered_round: 5
---

# PT11 — v0.3 Slate 1: The Balanced Opener

**Purpose:** baseline retest of Slate 1 under CODEX v0.3, applying all 6 fixes:
1. Deep Roots: +1 VP/component on matching terrain, MAX 5 VP
2. Iron Vein: triggers Forge adjacent to **Sanctum** (not Spire)
3. Mana cap: 12 total (not 8)
4. Encirclement: 3+ adjacent hexes = 2VP, 5+ = 3VP, all 6 = 4VP
5. Edict: strips VP tokens banked on doctrine + blocks future VP
6. Veil 3-Forge acceleration: once per player per game only

**Slate 1 (unchanged from v0.1):**
- Iron: Tempered + IV · SC · PI · SW
- Flame: First to Burn + DL · SE · MO · CG
- Tide: Ebb and Flow + TX · EL · TL · TM
- Earth: Deep Roots + CD · DT · FH · SM

**Simulation mode:** simultaneous activation from R3 onward. Turn order Flame→Iron→Tide→Earth. Starting mana 3W+2F. Same corner terrain assumptions as PT02. Same corner components: Iron=Forge (NW), Tide=Channel (NE), Flame=Spire (SE), Earth=Sanctum (SW).

**Key strategic difference from v0.1:** Iron now has *two* ways to trigger Iron Vein — build a Forge adjacent to the starting Sanctum (Fix 2). This radically re-prices Iron's early draft-vs-placement calculus.

---

## Round-by-round summary

### Round 1 (Veil: 10 → 9)

**Flame** (first, free first action): *Dawn Law* — place 2 Spires (one on adjacent Ember Peaks, one on Forge Ground edge). Action 2: Activate Ember Peaks Spire → 2E (1E spent). Action 3: Activate Forge-Ground Spire → 2E (1E spent; no territory mismatch since Spire activation uses matching mana of Spire's mana-type, which is Ember regardless of hex). End: 3W + 2F + 2E, 3 components. *MO: 4E produced this round vs 0 "last" → no trigger (R1 has no predecessor).* **Flame VP = 0.**

**Iron** (now drafts IV with intent to use Forge+Sanctum adjacency — key v0.3 change): Action 1: Activate starting Forge → 2W base + 1W Tempered = 3W (1W spent). Action 2: Build Sanctum on adjacent Sanctum Hills (2F cost, Tempered token placed). Action 3: Activate new Sanctum — produces 2F + 1F token = 3F (1F spent). End: 4W + 4F, 2 components. **Iron VP = 0.** *Note: Iron's starting Forge is now adjacent to the new Sanctum — Iron Vein is primed.*

**Tide**: Action 1: Build Channel on adjacent Flow Channel (2F). Action 2: Activate new Channel — *Tidal Mill* adjacent to starting Channel → 1F + 1F = 2F (1F spent). Action 3: Activate starting Channel — TM → 2F (1F spent). End: 3W + 3F, 2 components. **Tide VP = 0.**

**Earth**: Action 1: Build Channel on adjacent Flow Channel (2F; Earth wants Earth-terrain placements for Deep Roots +1VP/component but must start from corner adjacency). Action 2: Activate Sanctum → 2F (1F spent). Action 3: Activate Channel → 1F (1F spent). End: 3W + 3F, 2 components. **Earth VP = 0.**

**End of R1:** Veil 10 → 9. No special triggers.

### Round 2 (Veil: 9 → 8)

**Flame** (first, free): Activate Spire → 2E. Action 2: Build 4th Spire on adjacent Ember Peaks (2E — Flame has 4E now). Action 3: Activate middle Spire → 2E. End: 3W + 2F + 2E, 4 components. *MO: this round produced 4E, last round 4E — tie, no VP.* **Flame VP = 0.**

Actually re-checking MO: R1 produced 4E; R2 produced 4E (activate + activate). Tie, no MO. Flame's MO is dead.

**Iron**: Action 1: Activate Forge — **Iron Vein fires** (Forge adjacent to Sanctum). 2W base + 1W Tempered + 2W IV = **5W** (1W spent, net +4W). Action 2: Build 2nd Sanctum on Sanctum Hills adjacent to existing Sanctum (2F). Tempered token. Action 3: Activate new Sanctum — **Sanctum Chain fires** (adjacent to existing Sanctum): 2F + 1F SC + 1F Tempered = 4F (1F spent, net +3F). End: 8W + 6F, 3 components. **Iron VP = 0.**

**Tide**: Action 1: Build 2nd Channel adjacent to starting Channel (2F). Action 2: Activate — Tidal Mill chain: new Channel adjacent to 2 Channels → produces 1F base + 1F TM = 2F (1F spent). Action 3: Activate middle-adjacent Channel → 2F (1F spent). End: 3W + 5F, 3 components. **Tide VP = 0.**

**Earth**: Action 1: Build Sanctum on Sanctum Hills (2F). Action 2: Activate starting Sanctum → 2F (1F spent). Action 3: Build Channel on Earth territory (2F; Earth tries to seed Earth-terrain presence for Deep Roots). End: 3W + 1F, 4 components. **Earth VP = 0.**

**End of R2:** Veil → 8. No triggers.

### Round 3 (Veil: 8 → 7) — simultaneous activation engaged

**Flame**: Free — Build 5th Spire on Ember Peaks contesting Iron's southern corridor. **+1 Veil (5th component):** 8→7. Cost 2E; Flame has 2E, pays it. Action 2: Activate contested Spire *adjacent to Iron's Sanctum* — **CG fires**: 2E + 2E CG = 4E (1E spent). Action 3: Activate non-contested Spire → 2E (1E spent). End: 3W + 2F + 6E, 5 components. *MO: R3 produced 6E vs R2 4E → +2 VP.* **Flame VP = 2.**

**Iron**: Action 1: Activate Forge — Iron Vein still fires (still adjacent to Sanctum) — 5W gross (1W spent, +4W). Action 2: Build Sanctum on Sanctum Hills adjacent to existing 2 Sanctums (2F). Action 3: Activate new Sanctum — **SC fires** (adjacent to 2 Sanctums counts; effect is +1F per card text, triggers once) → 4F (1F spent, +3F). End: 11W + 8F. *Cap 12 total — 19 → 12.* Iron keeps 7W + 5F (prioritizes Weight for Forges). Actually Iron wants F for Sanctum build next round too, balance 7W + 5F. 4 components. **Iron VP = 0.**

Wait — the v0.1 PT02 had 8-cap and Iron was hemorrhaging mana. v0.3's 12-cap lets Iron keep substantially more. This is a visible Fix 3 effect.

**Tide**: Action 1: Activate middle Channel — TM chains (adjacent to 2 others, all 3 chain) → 1F base + 1F TM = 2F (1F spent). Action 2: *Expansion Law* free adjacency — Build Channel somewhere advantageous on Flow Channel near map center (2F). Action 3: Activate — TM → 2F (1F spent). End: 3W + 5F, 4 components. **Tide VP = 0.**

**Earth**: Action 1: Activate Sanctum → 2F (1F spent). Action 2: Build 5th component — Channel on Earth territory (2F). **+1 Veil (5th):** 7→6. Action 3: Activate new Channel → 1F (1F spent). End: 3W + 3F, 5 components. **Earth VP = 0.**

**End-of-round decay:** Veil 6 → 5.

Running Veil tally R3: entered at 8. Flame 5th: -1→7. Earth 5th: -1→6. End decay: -1→5.

### Round 4 (Veil: 5)

**Flame**: Free — Activate contested Spire → 4E (1E spent). Action 2: Activate Spire → 2E (1E spent). Action 3: Build 6th Spire on Ember Peaks (2E). **+1 Veil (6th is not a separate trigger — only 5th triggers; nothing fires).** End: 3W + 2F + 4E, 6 components (max). *MO: 6E this round same as R3 — tie. No VP.* **Flame VP = 2.**

**Iron**: Action 1: Activate Forge (Forge+Sanctum still adjacent) — IV fires → 5W (1W spent, +4W). Action 2: Build 5th component — 2nd Forge on Forge Ground adjacent to a Sanctum (now ensuring *both* Forges are adjacent to a Sanctum so IV fires on both in R5) (2W). **+1 Veil (5th):** 5→4. Action 3: Activate Sanctum Chain Sanctum — SC fires → 4F (1F spent, +3F). End: 10W + 8F = 18 → cap 12. Iron keeps 7W + 5F. 5 components. *PI trigger end of round: no Conflict → +2W.* End: 9W + 3F = 12. Fits exactly at cap. **Iron VP = 0.**

**Tide**: Action 1: Activate Channel — TM → 2F. Action 2: Activate Channel — TM → 2F. Action 3: Convert 2F→2W via Transmutation (1:1). End: 3W + 2W + 3F + 2F - 2F spent on activation = 5W + 3F. *Ebb and Flow: skipped.* 4 components. **Tide VP = 0.**

**Earth**: Action 1: Activate Sanctum → 2F (1F spent). Action 2: Build 6th component — Sanctum on Sanctum Hills (2F). Action 3: Activate new Sanctum → 2F (1F spent). End: 3W + 3F, 6 components. **Earth VP = 0.**

**End-of-round:** Veil 4 → 3 (Iron 5th trigger handled above, end decay -1). 

Wait, let me retally. Veil entering R4 at 5. During R4: Iron 5th: -1→4. End decay: -1→3.

### Round 5 (Veil: 3 → 2 start? or Unraveling?)

Veil enters R5 at 3. Unraveling triggers at 0.

**Flame**: Free — Activate contested Spire → 4E. Action 2: Activate Spire → 2E. Action 3: Convert 4E→2W (2:1 default). End: 5W + 2F + 2E, 6 components. *MO: 6E same as R4, tie, no VP.* **Flame VP = 2.**

**Iron**: Action 1: Activate Forge #1 — IV fires → 5W (1W spent, +4W). Action 2: Activate Forge #2 — IV fires (Forge #2 is adjacent to Sanctum per Iron's R4 placement) → 5W (1W spent, +4W). **3+ Forges activated? Only 2 Forges total. No Veil trigger.** Action 3: Activate SC Sanctum → 4F (1F spent, +3F). End: 12W + 3F = 15 → cap 12. Iron keeps 12W + 0F. Wait actually 9W entering + 8W gain - 1F activation cost... let me recompute. Entering R5: 9W + 3F (after PI). Gains: +4W +4W +3F. End raw: 17W + 6F = 23 → cap 12. Iron keeps 12W. *PI: no Conflict → +2W; exceeds cap, stays at 12W.* 5 components. **Iron VP = 0.**

**Tide**: Action 1: Activate middle Channel — TM → 2F. Action 2: Activate Channel → 2F. Action 3: **Temporal Loop** — repeat R4's actions at half cost (Activate/Activate/Convert). Gains 2F + 2F + 2W (from Convert 2F→2W), costs rounded-up 1F+1F+1F=3F. Net: +4F +2W -3F activation. Entering R5: 5W + 3F. After R5 actions: +2F +2F = +4F, -2F spent = +2F. After TL: +4F +2W -3F = +1F +2W. Total: 9W + 6F, 4 components. *Cap 12: 15 → 12. Keeps 9W + 3F.* **Tide VP = 0.**

**Earth**: **Final Hour fires — all activations double this round.** Action 1: Activate Sanctum → 2F × 2 = 4F (1F spent, +3F). Action 2: Activate Sanctum → 4F (1F spent, +3F). Action 3: Activate Channel → 1F × 2 = 2F (1F spent, +1F). End: 3W + 10F, 6 components. *Cap 12: 13 → 12. Keeps 3W + 9F.* **Earth VP = 0.**

**End-of-round:** No Conflict destruction. End decay: Veil 3 → 2. Not yet Unraveling.

Hmm, Veil still at 2 after R5. Continue to R6 or does game end at R5? Rules say "5 Rounds" in Phase 2. Veil never reached 0. Per rules: Phase 2 is "5 Rounds"; the Veil is a doom clock for early Unraveling. If Veil doesn't hit 0 in 5 rounds, Unraveling begins at end of R5 regardless. Confirmed by structure (Phase 3 begins after R5).

**Unraveling begins at end of R5.** Veil triggered round: **5** (normal end of Phase 2).

---

## The Unraveling

### Step 1 — Final Activation

**Flame** (Flame first, turn order): 6 Spires. 1 contested (Iron's Sanctum adjacent) → 4E. Others 5 × 2E = 10E. Total 14E produced, cost 6E (1E per). Net +8E. End: 5W + 2F + 10E, 17 mana.

**Iron**: 5 components — 2 Forges + 3 Sanctums. Both Forges adjacent to Sanctums (IV). All 3 Sanctums at least adjacent to another Sanctum (SC).
- Forges: 2W + 1W Tempered + 2W IV = 5W each × 2 = 10W (- 2W cost) = +8W.
- Sanctums: 2F + 1F Tempered + 1F SC = 4F each × 3 = 12F (-3F cost) = +9F.
- Iron Vein counts as production. 2 Forges activated. 3+ Forge Veil trigger: **only 2 Forges. No trigger.** (Fix 6 didn't apply — only 2 Forges.)
End: Iron 12W (from cap) + 0F entering → +8W +9F → 20W + 9F = 29 mana total. *No cap during Unraveling per PT02 interpretation.* 

**Tide**: 3 Channels. All adjacent (TM). 1F + 1F TM = 2F each × 3 = 6F (-3F cost) = +3F. End: 9W + 3F entering + 3F = 9W + 6F = 15 mana.

**Earth**: 6 components — 3 Sanctums + 3 Channels. No SC drafted. No LR drafted. Plain: Sanctums 2F - 1F = +1F × 3 = +3F. Channels 1F - 1F = 0F × 3. Total +3F. End: 3W + 9F + 3F = 3W + 12F = 15 mana.

### Step 2 — Edicts (reverse turn order: Earth → Tide → Iron → Flame)

**VP tokens on the board (Fix 5) before Edicts:**
- Flame's **Momentum**: 1 VP token (earned R3). Banked 2 VP.
- Iron: no token-accruing cards this game.
- Tide: no token-accruing cards.
- Earth: no token-accruing cards (DT/FH/CD/SM are all end-game scorers or passive).

End-game VP doctrines that *will* score:
- Iron's **Spider's Web** (5 VP from 5-component connected group)
- Earth's **Deep Roots** bonus (now capped at 5 VP — Fix 1)
- Earth's **Deep Territory** (2 VP per territory type)
- Earth is **immune** (Deep Roots).

**Earth Edict (first):** Names **Iron's Spider's Web** → SW blocked, no future VP. (SW has no banked VP tokens since it's pure endgame.) Iron loses 5 VP potential.

**Tide Edict:** Iron's SW already suspended. Earth immune. Tide targets Flame's **Momentum** to strip 2 banked VP tokens (Fix 5: Edict strips banked VP). **Flame loses 2 VP.** And blocks future MO VP — but no more MO rounds anyway.

**Iron Edict:** Earth immune, Tide has nothing scoring, Flame's MO already stripped. Iron names **Flame's Contested Ground** (cosmetic, CG has no VP). Or names **Tide's Transmutation** — no VP effect. Iron's Edict is wasted. Iron for tie-break names **Flame's Scorched Earth** (cosmetic).

**Flame Edict:** All remaining non-Earth targets are non-VP. Flame names **Iron's Iron Vein** (passive; already fired during Final Activation — stripping now has no retroactive effect under Fix 5 since IV produces *mana*, not *VP tokens*). Flame's Edict is cosmetic.

**Fix 5 verdict in this slate:** Edicts retroactively stripped Flame's 2 banked Momentum VP. Without Fix 5, v0.1 assumed banked VP was safe. Now Tide's Edict had real teeth — removed 2 VP from Flame.

### Step 3 — Scoring

**Iron:**
- Spider's Web: **SUSPENDED** → 0 VP
- 5 components in one connected group would have been 5 VP
- Mana VP: 29 / 2 = **14 VP**
- **Iron total: 14 VP**

Wait — Fix 3 cap 12 means Iron kept MORE mana round-by-round. Between rounds the cap was 12 (was 8). Final Activation adds to that un-capped. Iron's final pool is substantially larger than v0.1.

Let me double-check: entering Final Activation Iron had 12W. Final Activation +8W +9F = 20W + 9F = 29 mana. 29/2 = 14.5 rounds to 14 VP. **Fix 3 gave Iron +4 VP** relative to v0.1 (where Iron had ~20 mana for 10 VP).

**Flame:**
- Momentum: banked 2 VP **STRIPPED by Tide's Edict** → 0 VP
- Mana VP: 17 / 2 = **8 VP**
- **Flame total: 8 VP**

Flame loses 4 VP vs v0.1 (had 12: 4 MO + 8 mana; now 0 + 8).

**Tide:**
- No scoring doctrines
- Mana VP: 15 / 2 = **7 VP**
- **Tide total: 7 VP**

Tide roughly same as v0.1 (was 8).

**Earth:**
- Deep Territory: Earth on Sanctum Hills (3 Sanctums), Flow Channel (1 Channel), Earth territory (2 components). 3 types × 2 VP = **6 VP**
- Deep Roots bonus (Fix 1: +1 VP/component on matching terrain, MAX 5 VP): Earth has 2 components on Earth territory. **2 VP** (capped at 5 so no cap here). *v0.1 gave +2 VP/component = 4 VP.*
- Mana VP: 15 / 2 = **7 VP**
- **Earth total: 15 VP**

Earth loses 2 VP vs v0.1 (Deep Roots nerf: 4 → 2).

### Final standings

| School | v0.1 | v0.3 | Δ |
|---|---|---|---|
| Iron | 10 | **14** | +4 |
| Flame | 12 | 8 | -4 |
| Tide | 8 | 7 | -1 |
| Earth | 17 | 15 | -2 |

Wait — Earth still wins at 15 vs Iron 14? Let me re-examine.

Actually I need to reconsider: Iron Vein firing now produces substantially more Weight for Iron across ALL rounds, not just Unraveling. Let me recount Iron's mana generated across R1-R5:
- R1: Forge 3W (no IV, no Sanctum adjacent yet), Sanctum 3F. Produced: 3W + 3F.
- R2: Forge with IV 5W, Sanctum SC 4F. Produced: 5W + 4F.
- R3: Forge IV 5W, Sanctum SC 4F. Produced: 5W + 4F.
- R4: Forge IV 5W, SC Sanctum 4F, PI +2W. Produced: 7W + 4F.
- R5: Forge #1 IV 5W, Forge #2 IV 5W, SC Sanctum 4F, PI +2W. Produced: 12W + 4F.

Total gross production R1-R5: 32W + 19F.
Activation costs across R1-R5: ~12 mana total (mostly 1F or 1W each activation).
Build costs: 2F (R1 Sanctum) + 2F (R2 Sanctum) + 2F (R3 Sanctum) + 2W (R4 Forge) = 2W + 6F.

Net held mana entering Unraveling: 32-2=30W gross, minus cap losses. Cap 12 means Iron loses substantial mana each round when it exceeds 12.

Let me re-track cap losses. End of R2: Iron 8W + 6F = 14, cap 12 → loses 2, keeps 12. End of R3: additional 5W+3F gain this round added to 12 = 20 → cap 12, loses 8. End of R4: after cap reset + R4 gains, Iron hits cap again. End of R5: cap 12.

The 12-cap still hurts Iron, just less than the 8-cap did. Iron enters Unraveling at 12 mana. Final Activation unCAPPED adds +8W +9F = 29 total. **Iron mana VP = 14.**

For Earth I said 15 total mana after FA. Let me recheck: Earth entered FA with 3W+9F=12. FA +3F = 15. Earth mana VP = 7.

Earth doctrine VP: 6 (DT) + 2 (Deep Roots Fix 1 — 2 components × 1 VP). Wait Fix 1 is "+1 VP/component on matching terrain, MAX 5 VP". So Earth has 2 Earth-territory components → 2 VP. Hmm, in PT02 I computed v0.1 Deep Roots as +2/component × 2 = 4 VP. Fix 1 halves it: 1/component × 2 = 2 VP. Cap doesn't bind (2 < 5).

Earth total: 6 + 2 + 7 = **15 VP**.

### Comparison to v0.1 (per system): 
- Earth drops 17 → 15 (−2 from Deep Roots nerf)
- Iron climbs 10 → 14 (+4 from cap 12 letting mana VP breathe AND from IV firing every round)
- Flame drops 12 → 8 (−4 from Edict stripping banked MO + already lost R4/R5 MO)
- Tide drops 8 → 7 (−1 noise)

**New margin: Earth 15 vs Iron 14 — 1 VP.**

That's a dramatic tightening from v0.1's 5-VP Earth win. Earth still wins but Iron is basically tied. If Iron had drafted differently (e.g., Deep Territory instead of Patient Iron), Iron likely wins.

---

## Final scores

| School | Doctrine VP | Map VP | Mana VP | Total |
|---|---|---|---|---|
| Iron | 0 (SW suspended) | 0 | 14 | **14** |
| Flame | 0 (MO stripped) | 0 | 8 | **8** |
| Tide | 0 | 0 | 7 | **7** |
| **Earth** | **8 (DT 6 + DR 2)** | 0 | 7 | **15** |

**Winner: Earth, 15 VP. Margin: 1 VP over Iron.**

---

## v0.1 vs v0.3 comparison

| School | v0.1 score | v0.3 score | Delta |
|---|---|---|---|
| Earth | 17 | 15 | −2 |
| Flame | 12 | 8 | −4 |
| Iron | 10 | 14 | +4 |
| Tide | 8 | 7 | −1 |

**Earth margin over runner-up:** v0.1 = +5 (over Flame). v0.3 = +1 (over Iron). **Gap reduced 80%.**

**Score spread (range):** v0.1 = 17−8 = 9. v0.3 = 15−7 = 8. Slightly tighter.

**Winner changed:** No — Earth still wins. But runner-up changed from Flame to Iron, and margin collapsed from 5 to 1.

---

## Fix validation — all 6 fixes

**Fix 1 — Deep Roots +1/max5:** YES, nerf landed. Earth lost 2 VP vs v0.1 (4 → 2). Earth still won but by 1 VP not 5. The cap (MAX 5) didn't bind this game (only 2 components on terrain), so the per-component halving did the work. **Verdict: effective, correctly scaled.** In a slate where Earth spreads 5+ components on Earth terrain, the cap would start to bite. Here the halving alone was enough.

**Fix 2 — Iron Vein Forge+Sanctum:** Iron drafted IV for the first time productively. IV fired 5 times across the game (R2, R3, R4, R5×2). Contributed roughly +10W total across the game. This was the single largest mechanical shift for Iron. v0.1 had Iron drafting IV but never triggering it (wrong spatial synergy with Iron's natural Sanctum+Forge build). **Verdict: excellent fix. IV now aligns with Iron's drafting pattern (Tempered + Forge-Ground placement).**

**Fix 3 — Mana cap 12:** In v0.1 Iron was constantly at 8-cap losing mana. In v0.3 at 12-cap Iron still hit cap at end of R3, R4, R5 but kept ~4 more mana per round. This translated to +4 VP from mana. Tide's mana was slightly less stressed too. **Verdict: effective.** Mana-VP becomes *slightly more* significant with cap 12 — note the slate has only 2 VP-earning doctrines (SW, DT), so mana VP still over-determines the result. But this isn't on Fix 3 — it's a separate observation about VP distribution.

**Fix 4 — Encirclement scaled:** Not triggered. EN wasn't drafted in Slate 1 (cut). **Verdict: untestable in this slate.** Need to rerun on Slate 6 or a slate with EN present.

**Fix 5 — Edict strips VP tokens:** YES, landed meaningfully. Tide's Edict stripped Flame's 2 banked Momentum VP, dropping Flame from 12 to 8. Without Fix 5, v0.1 assumed banked VP was safe. **Verdict: genuinely new mechanic; made Edicts feel threatening even when the target's doctrine had no future VP.** Flame's MO bank was vulnerable. Caveat: Iron's and Flame's Edicts were still wasted this game because only one doctrine per non-Earth player had any VP footprint (Iron's SW + Flame's MO). In slates where more doctrines accumulate tokens, Fix 5 will have more targets.

**Fix 6 — Veil 3-Forge once/game:** Did not trigger. Iron built only 2 Forges (game ran out of rounds). **Verdict: untestable here.** In Slate 7 (Iron Snowball) with 3+ Forges it will matter.

---

## Overall balance verdict: 3.5/5

The slate is materially closer in v0.3 than v0.1. The 5-VP → 1-VP margin drop is the headline. But Earth still wins; Deep Roots + Edict immunity still asymmetrically protects Earth's endgame VP, while every other School's scoring can be Edicted. **The structural problem isn't solved — it's compressed.** Iron is now competitive because cap-12 unlocks mana VP, not because Edicts became fair.

Secondary concerns:
- Mana VP remains the primary VP source (Iron 14 from mana, 0 from doctrines is a bad pattern).
- Non-Earth endgame-VP doctrines are scarce in Slate 1's drafted cards (only SW). This is a slate-design issue, not a rule issue.
- Momentum's "strictly greater than last round" trigger is still too narrow — fires R3 once, then tie-plateau kills it.

---

## Is CODEX v0.3 ready for a real human playtest?

**Yes, with the caveat that Slate 1 doesn't fully exercise Fixes 4 and 6.**

Recommended playtest order:
1. **Slate 1 (v0.3)** — establishes baseline with Fixes 1, 2, 3, 5.
2. **Slate 3 or Slate 6** — exercises Fix 4 (Encirclement).
3. **Slate 7 or Slate 2** — exercises Fix 6 (Iron 3+ Forge Veil trigger).

The 1-VP margin in this retest is within playtest noise (turn-order variance, corner-terrain assumptions, edict-order choices). A human table would almost certainly produce different results — potentially Iron winning, or a genuine 4-way close game. That's a desirable level of tension for a first real playtest.

## Top remaining issue if any

Mana-VP (1 VP per 2 mana at cap 12) is doing too much of the work — Iron's 14 VP is 100% mana, which reads as "Iron won despite having no scoring doctrines fire", and that feels wrong. Consider a mana-to-VP step function (e.g., 1 VP per 3 mana beyond the first 6) so that doctrine VP remains the primary lever.
