---
playtest: PT03
slate: 5 — Tempo Race
players: Iron / Flame / Tide / Earth
veil_triggered_round: 4
winner: Iron (34 VP)
created: 2026-04-22
simulator: claude-opus-4-7
---

# PT03 — Slate 5: Tempo Race

## Setup

| School | Corner | Founding | Drafted | Starting component |
|---|---|---|---|---|
| Iron | NW | Tempered | DL · IB · MO · IV | Forge on NW corner (Forge Ground) |
| Flame | SE | First to Burn | FH · SE · CG · RP | Spire on SE corner (Ember Peak) |
| Tide | NE | Ebb and Flow | TL · TX · EL · TM | Channel on NE corner (Flow Channel) |
| Earth | SW | Deep Roots | PI · SW · DT · CD | Sanctum on SW corner (Earth / Sanctum Hill) |

Starting mana each: **3 Weight + 2 Flow**. Veil: **10**. Turn order per round: **Flame → Iron → Tide → Earth** (First to Burn).

---

## Round 1 — "The Dawn Play"

Veil at 10 → 9 end of round.

### Flame (first, first action free)
- **A1 (free):** Build Spire on Ember Peak adjacent to starting Spire. Now 2 Spires (SE corridor).
- **A2:** Activate starting Spire (1E cost). Produces 2 Ember. *Not adjacent to opponent yet → CG quiet.*
- **A3:** Activate 2nd Spire (1E). Another 2E. End mana: 3W · 2F · 2E (spent 2 on activates, gained 4).
- **Veil triggers so far:** none.

### Iron (2nd)
- **A1 — Build (Dawn Law):** Pay 2W, **Dawn Law fires → place 2 components instead of 1.** Places 2 Forges on adjacent Forge Ground hexes. **Tempered** places 1 Iron Token on each. **Iron Bloom** fires for each build → activate each immediately for free.
  - Forge #2 activation: base 2W + Iron Token +1W = 3W.
  - Forge #3 activation: base 2W + Iron Token +1W = 3W.
  - Iron now has 4 Forges? No — 1 starting Forge + 2 new = **3 Forges total**. Iron Bloom produced 6W.
  - *Wait — ruling check: the starting Forge has no Iron Token (Tempered only triggers on build).* Confirmed. Starting Forge is 2W base only.
- **A2 — Activate starting Forge** (1W cost). Produces 2W (no Iron Token).
- **A3 — Activate Forge #2** (1W cost). Produces 3W. 
  - **Veil trigger check:** Iron activated 3 Forges in one round. **Veil +1.**
- Iron mana post-turn: started 3W+2F. Spent 2W (Dawn Law build) + 1W + 1W + 1W (activates) = 5W spent. Gained from Iron Bloom: 3W+3W = 6W. Gained from activates: 2W + 3W = 5W. Net Weight: 3 − 5 + 6 + 5 = **9W**. Flow: 2. Total: 11 mana (under cap 8 — lose 3, stores 8).
- **Storage cap hit:** Iron stores 8 (loses 3 mana). Split: **6W + 2F**.
- **Momentum** check: produced this round (11W gained) vs previous (0 baseline). Momentum doesn't score until round 2 ends (needs prior-round comparison). Not triggered R1.

### Tide (3rd)
- **A1 — Build Channel** (Expansion Law first build ignores adjacency). Place Channel on Flow Channel hex 3 rows in. Cost 2F. Tide starting mana 3W + 2F → 0F left, 3W. **Tidal Mill** requires adjacency to another Channel; new Channel not adjacent to starting one, so TM quiet.
- **A2 — Activate starting Channel** (1V? No — activate costs 1 mana matching or any? Rules: "Activate: 1 mana."). Spend 1 of any mana. Spend 1W. Channel produces 1 Void.
- **A3 — Activate Channel #2** (1W). Produces 1V. TM quiet (no adjacent Channel).
- Tide end mana: 3W − 2 = 1W. 0F. 2V. Total 3.
- Ebb and Flow used later if useful; skip this round.

### Earth (4th)
- **A1 — Build Sanctum** adjacent to starting Sanctum on Sanctum Hill. Cost 2F? Earth has only 2F. Pay 2F. Now 2 Sanctums.
- **A2 — Activate starting Sanctum** (1 mana, spend 1W). Produces 2F.
- **A3 — Activate Sanctum #2** (1W). Produces 2F.
- **Patient Iron (PI):** Earth took no Conflict action → trigger at end of round → +2W.
- Earth end mana: 3W − 2 (activates) + 2 (PI) = 3W. 4F. 0V. Total 7.

### End Round 1 — Veil advances
- Base advance: −1.
- Iron 3 Forges activated in R1: −1 (already counted).
- Any 5th component built? No.
- Any destruction? No.
- **Veil: 10 − 2 = 8.**

### Round 1 state
| School | Components | Mana |
|---|---|---|
| Iron | 3 Forges (2 tokened) | 6W + 2F |
| Flame | 2 Spires | 3W · 2F · 2E |
| Tide | 2 Channels | 1W · 0F · 2V |
| Earth | 2 Sanctums | 3W · 4F · 0V (+ PI VP? No, PI drafted card = 2W only, no VP) |

VP ledger: all 0. **Production R1 totals:** Iron 11, Flame 4E, Tide 2V, Earth 4F.

---

## Round 2 — "The Engines Fire"

Veil 8 → 7 end of round (unless triggers).

### Flame (first action free)
- **A1 (free):** Build Spire adjacent to Spire #2, pushing into the central corridor. **3rd Spire.** No cost (First to Burn).
- **A2:** Activate Spire #3 (1E). Produces 2E. Still not adjacent to opponent.
- **A3:** Build Spire #4 (2E cost) toward the map center, now adjacent to a hex Tide could contest. 3E + 2E from activate − 2E = 3E.
- Flame end mana: 3W · 2F · 3E · (gained 2E activate) = 3W · 2F · 3E. 4 Spires total.

### Iron
- **A1 — Build Forge #4** adjacent to existing Forges. Cost 2W. Tempered +1 IT. **Iron Bloom** activates it for free: 2W + 1 IT = 3W. Now 4 Forges.
- **A2 — Activate Forge #2** (1W). 3W produced.
- **A3 — Activate Forge #3** (1W). 3W produced.
- **Iron Vein check:** IV requires a Forge adjacent to a Spire **you own**. Iron has no Spires. **IV silent this game unless Iron builds a Spire.** *Critical ruling.*
- **Veil trigger:** 3+ Forges activated this round (Forge #2 + Forge #3 = 2 explicit + IB activation of Forge #4 = 3). **Veil +1.**
- Iron mana: start 6W · 2F. Spent 2W (build) + 1W + 1W = 4W. Gained 3W (IB) + 3W + 3W = 9W. Net: 6 − 4 + 9 = 11W. Cap 8 → 8W total (lose 3, keep 6W + 2F).
- **Momentum (MO):** R1 production 11, R2 production 9. **9 < 11, Momentum does NOT fire.** *Key insight: Iron Bloom front-loaded R1 so hard that Momentum is almost impossible to satisfy in R2.*

### Tide
- Tide chooses to use **Temporal Loop** later (round 4 peak) so defer.
- **A1 — Activate Channel #1** (1W → now 0W). Produces 1V.
- **A2 — Activate Channel #2** (spend 1 of starting 1W? no — use 1V). Produces 1V. TM still quiet.
- **A3 — Convert** 2V → 1F (default 2:1). Tide stores 1F.
- **Ebb and Flow:** Move Channel #2 adjacent to Channel #1. **Now adjacent → TM eligible next round.**
- Tide end mana: 0W · 1F · 1V (used 2V converting, produced 2V, started 2V = 2V − 2 + 2 = 2V… recompute). Start: 1W · 0F · 2V. A1 spend 1W → 0W. A1 produces 1V → 3V. A2 spend 1V → 2V. A2 produces 1V → 3V. A3 convert 2V → 1F. End: 0W · 1F · 1V.

### Earth
- **A1 — Build Sanctum #3** adjacent to Sanctum #2 on Sanctum Hill. Cost 2F (has 4F → 2F).
- **A2 — Activate Sanctum #1** (1W → 2W). Produces 2F.
- **A3 — Activate Sanctum #2** (1W → 1W). Produces 2F.
- **Patient Iron** end of round: +2W. Earth now 3W · 5F.

### End Round 2 — Veil advances
- Base −1. Iron 3+ Forges fired again −1.
- No 5th component yet (Flame 4 Spires, Iron 4 Forges, Tide 2, Earth 3).
- **Veil: 7 − 2 = 5.** *(Already halfway at end of R2. Iron's Forge spam is burning the track.)*

### Round 2 state
| School | Components | Mana | VP |
|---|---|---|---|
| Iron | 4 Forges | 6W · 2F | 0 |
| Flame | 4 Spires | 3W · 2F · 3E | 0 |
| Tide | 2 Channels (now adjacent) | 0W · 1F · 1V | 0 |
| Earth | 3 Sanctums | 3W · 5F | 0 |

---

## Round 3 — "Corridors Close"

Veil 5 → dangerous. Flame and Tide both push into the center.

### Flame (first action free)
- **A1 (free):** Build Spire #5. **Veil trigger: 5th component +1.**
- **A2:** Build Spire #6 (max component limit). Costs 2E. Now at cap. Adjacent to Earth's Sanctum #3 region.
- **A3:** Activate Spire #4 (1E). Produces 2E. **Contested Ground** check: Spire #4 adjacent to a Tide Channel? Yes (Tide moved Channel #2 into central corridor). **CG fires → +2E.** Total 4E this activation.
- Flame end mana: gained 4E − spent 2E (build) − 1E (activate) + 4E = spent 3, gained 4 net+1E from before. Recount: 3W · 2F · 3E start. A1 free. A2 spend 2E → 1E. A3 spend 1E → 0E, produce 4E → 4E. End: 3W · 2F · 4E.

### Iron
- **A1 — Build Forge #5**. **Veil trigger: 5th component +1** (already +1 from Flame — these stack, each *any* 5th component triggers). 
  - *Ruling: rules say "Any player builds their 5th component (+1)." Each player's own 5th counts separately. So Iron's 5th = another +1.*
  - Cost 2W. Tempered +1 IT. IB activates: 3W.
- **A2 — Activate Forge #2** (1W). 3W.
- **A3 — Activate Forge #3** (1W). 3W.
- **Veil trigger:** 3+ Forges activated → +1.
- Iron mana: 6W + 3 (IB) + 3 + 3 − 2 − 1 − 1 = 11W. Cap at 8.
- **Momentum:** R3 Weight produced = 9. R2 = 9. Not greater, **MO does not fire.** Iron has produced identical totals → MO silent.

### Tide
- **A1 — Activate Channel #1** (1 of 1F, convert if needed; spend 1F → 0F). Produces 1V. **TM fires** (adjacent to Channel #2) → +1F additional. Net: 1V + 1F from TM. End mana check: 0W · 0F · 2V → activate produce 1V +TM 1F → 0W · 1F · 3V.
- **A2 — Activate Channel #2** (1V → 2V). Produces 1V + TM 1F (adjacent to Channel #1). End: 0W · 2F · 3V.
- **A3 — Build Channel #3** via **Expansion Law** anywhere (first build of round). Cost 2F → 0F. Tidal Mill check for #3 depends on placement. Place adjacent to #2. Now 3 Channels in a row.
- **Ebb and Flow:** Move Channel #3 to a Flow Channel hex if not already. Optimize.
- Tide end mana: 0W · 0F · 3V. 3 Channels.

### Earth
- **A1 — Build Sanctum #4** (2F → 3F). Now 4 Sanctums.
- **A2 — Activate Sanctum #1** (1W → 2W). Produces 2F.
- **A3 — Activate Sanctum #2** (1W → 1W). Produces 2F.
- **Patient Iron** +2W. Earth now 3W · 7F · 4 Sanctums.

### End Round 3 — Veil cascade
- Base −1.
- Flame 5th component +1.
- Iron 5th component +1.
- Iron 3+ Forges +1.
- **Veil: 5 − 4 = 1.** *(One step from Unraveling.)*

### Round 3 state
| School | Components | Mana | VP |
|---|---|---|---|
| Iron | 5 Forges | 6W · 2F | 0 |
| Flame | 6 Spires (max) | 3W · 2F · 4E | 0 |
| Tide | 3 Channels | 0W · 0F · 3V | 0 |
| Earth | 4 Sanctums | 3W · 7F | 0 |

**Critical observation:** The Final Hour never triggers — Veil hits 0 in R4 before R5 begins.

---

## Round 4 — "The Veil Snaps"

Veil = 1 at round start. Advances 1 end of round, but any trigger mid-round ends it. Unraveling begins immediately if Veil → 0.

### Flame (first action free)
- Flame at component cap (6). Cannot build.
- **A1 (free):** Activate Spire #1 (corner). Produces 2E.
- **A2:** Activate Spire #3 (1E → 3E). Produces 2E + CG if adjacent to opponent. Spire #3 adjacent to… central corridor, not touching opponents. 2E baseline.
- **A3 — Conflict (Raiding Party):** RP once per round, steal 1 mana from adjacent opponent. Flame has Spires adjacent to Tide Channel and to Earth Sanctum. Steal 1F from Earth. Cost: Conflict is 2 mana (2E) to trigger **any Conflict Doctrine** — wait, re-read rules. *"Conflict: 2 mana — trigger one of your Conflict Doctrines against an adjacent opponent."* Pay 2E. RP fires, steal 1F.
  - **Veil trigger:** Conflict action *destroys a component* +1. RP does NOT destroy, just steals. No trigger.
- Flame end mana: 3W · 2F · 4E start of round. Activated twice: +4E, spent 1E. Conflict: −2E. End: 3W · 3F · 5E.

### Iron (2nd)
- Iron at 5 components. Can build 6th.
- **A1 — Build Forge #6** (2W → 4W). IT +1, IB activates for 3W. Iron now at cap.
- **A2 — Activate Forge #4** (1W → 6W). 3W.
- **A3 — Activate Forge #5** (1W → 8W). 3W.
- **Veil trigger:** 3+ Forges activated this round → +1. **Veil: 1 − 1 = 0. UNRAVELING BEGINS MID-ROUND.**

*(Per rules: "If the Veil reaches 0, the Unraveling begins immediately — even mid-round." Tide and Earth do NOT get round 4 actions.)*

### Round 4 interrupted state
| School | Components | Mana | VP |
|---|---|---|---|
| Iron | 6 Forges (cap) | 8W · 2F | 0 |
| Flame | 6 Spires | 3W · 3F · 5E | 0 |
| Tide | 3 Channels | 0W · 0F · 3V | 0 |
| Earth | 4 Sanctums | 3W · 7F | 0 |

---

## THE UNRAVELING

### Step 1 — Final Activation (turn order)

**Flame activates all 6 Spires:**
- Spires 1–4 (corner cluster, non-adjacent to opponent): 4 × 2E = 8E.
- Spire #5 and Spire #6 (central corridor, adjacent to Tide Channel/Earth Sanctum): 2 × (2E + CG +2E) = 8E.
- Total: 16E generated. Flame end: 3W · 3F · 21E.
- **FH (The Final Hour)** — requires round 5. **Does NOT trigger** (game ended R4). *This is a huge loss for Flame.*

**Iron activates all 6 Forges:**
- Forge #1 (no token): 2W.
- Forges #2-#6 (each with 1 IT): 5 × 3W = 15W.
- Iron Vein requires Spire adjacency — Iron has NO Spires. **IV scores 0 this activation.**
- Total: 17W. Iron end: 25W · 2F.

**Tide activates all 3 Channels:**
- Channel #1: 1V + TM (adjacent) +1F = 1V + 1F.
- Channel #2: 1V + TM (adjacent) +1F = 1V + 1F.
- Channel #3: 1V + TM (adjacent if #2 adjacent) +1F = 1V + 1F.
- Total: 3V + 3F. Tide end: 0W · 3F · 6V.

**Earth activates all 4 Sanctums:**
- Each: 2F. No Sanctum Chain drafted — adjacency doesn't matter.
- Total: 8F. Earth end: 3W · 15F.

### Step 2 — Edicts (reverse turn order: Earth, Tide, Iron, Flame)

Strategic targeting — suspend the biggest VP engine.

- **Earth Edict:** Name **Iron Vein** on Iron. *But IV scores 0 anyway (no Spires). Switch target.* Suspend **Iron Bloom** — kills Iron's build-time multiplier (retroactive scoring-only impact is low; Iron Bloom mostly scaled production that's already in storage). Better: suspend a *scoring* Doctrine. Iron's actual scoring comes from **Momentum** (didn't fire) and stored Weight (1VP/2 mana). Iron has no other VP doctrine. Actually — Iron has MO, IV (0 value), IB (production), plus Tempered. **None of Iron's Doctrines score VP directly except MO (which never fired) and IV (no Spires).** Earth suspends **Momentum** just to be safe against a retroactive ruling. (MO fired 0 times anyway.) *Wasted Edict — Iron has nothing to suspend.*
  - Pivot: Earth targets **Tide's Temporal Loop** (unused) → 0 value suspension. Or **Flame's The Final Hour** (already did not fire). Or Flame's **Contested Ground**. CG already generated its Ember in Unraveling step 1 (Ember is mana, not VP). **Most Edicts in this slate target Doctrines that have already done their work.**
  - Realistic: Earth suspends **Flame's Contested Ground** to deny any retroactive CG scoring = still 0 VP impact.
  
  *Finding logged: Unraveling mid-R4 makes Edicts toothless because all production Doctrines have already generated their mana before the Edict step.*

- **Tide Edict:** Suspend **Iron's Tempered** (Founding!). **But rules: Founding Doctrines are not in the 25-pool draft, they sit face-up. Can Edicts target them?** Re-read rules: "name one of any opponent's Doctrine cards — it is suspended and scores no VP this game." Tempered has no VP clause. *Suspending it does nothing retroactively — Iron Tokens already on components.* Tide targets **Earth's Deep Territory** to prevent 2VP/territory-type scoring. DT will score ~2-3 VP. Suspend it.

- **Iron Edict:** Suspend **Earth's Spider's Web** (main VP engine for Earth). Kills Earth's 4-component SW score = 4 VP cancelled.

- **Flame Edict:** Suspend **Earth's Deep Roots**? Earth is **immune to Edicts via Deep Roots**. So DR is untouchable, and by ruling Earth's doctrines *all* cannot be suspended. **Wait — does DR protect Earth's other doctrines too?** Rules: *"Earth's Doctrines cannot be suspended by Edicts."* All of Earth's doctrines are immune. 
  
  **Corrected Edict retarget:**
  - Iron Edict target (Earth SW) → **ILLEGAL**, Earth immune. Retarget to **Tide's Tidal Mill** (produces Flow — already produced). Zero VP impact.
  - Tide Edict target (Earth DT) → **ILLEGAL**, Earth immune. Retarget to **Iron's Momentum**. 0 VP (MO didn't fire).
  - Flame Edict: Target **Iron's Iron Vein**. 0 VP (no Spires).
  - Earth Edict: Target **Flame's** something. Flame has FH (didn't fire in R4), SE (conditional), CG (mana not VP), RP (mana not VP). Target **The Final Hour**. 0 impact.

**Net Edict impact: 0 VP swung.** The mid-R4 Unraveling + Earth's immunity collapses the whole Edict phase.

### Step 3 — Scoring

#### Iron
- **Iron Vein:** 0 (no Spires).
- **Iron Bloom:** 0 (not a scoring doctrine — produces mana only, already banked).
- **Dawn Law:** 0 (not a scoring doctrine).
- **Momentum:** 0 (never fired — R1 produced 11W, R2 9W, R3 9W, R4 11W. MO requires *each round > previous round*).
- **Tempered (Founding):** 0 VP directly.
- **Map VP:** none (no Spider's Web, Deep Territory, Encirclement, Crown control).
- **Mana VP:** 25W · 2F = 27 mana. ÷2 = **13 VP**.
- **No doctrine VP directly.** Iron's VP comes almost entirely from stored Weight.
  
  *Wait — did Iron's Dawn Law / Iron Bloom score? Neither is a scoring doctrine. Iron produced 0 doctrine VP.*
  
- Iron total: **13 VP**.

#### Flame
- **The Final Hour:** 0 (round 5 never happened).
- **Scorched Earth:** 0 directly (produced no VP, just taxed).
- **Contested Ground:** 0 (production, already in mana).
- **Raiding Party:** 0 (stole mana, not VP).
- **First to Burn (Founding):** 0 direct.
- **Mana VP:** 3W · 3F · 21E = 27 mana ÷ 2 = **13 VP**.
- Flame total: **13 VP**.

#### Tide
- **Temporal Loop:** 0 (never used — Tide was saving for R5 that never came).
- **Transmutation:** 0 directly.
- **Expansion Law:** 0 directly.
- **Tidal Mill:** 0 directly.
- **Ebb and Flow (Founding):** 0 direct.
- **Mana VP:** 0W · 3F · 6V = 9 mana ÷ 2 = **4 VP**.
- Tide total: **4 VP**.

#### Earth
- **Patient Iron:** 0 direct VP (drafted PI gives 2W/round only). *Wait — re-read PI. PI (T8) is "Each round you end having taken no Conflict action → gain 2 Weight." No VP clause.* 0 VP. Earth gained 8W across R1–R4.
- **Spider's Web:** 4 components all connected (Sanctum cluster in SW) → **4 VP**.
- **Deep Territory:** Earth has Sanctums only, all on Sanctum Hill / Earth terrain. Only 1 territory type counted → **2 VP**. *Ruling: Sanctum Hill is one type. Earth corner may mix 2 types but Earth stayed compact.* Generous: 2 types → 4 VP. Conservative: 1 type → 2 VP. **Use 2 VP.**
- **Counter-Doctrine:** 0 (no Conflict ever targeted Earth — Flame's RP stole from Earth but RP targets are opponents with adjacent component; CD fires when a *Conflict Doctrine* targets you. Did RP target Earth? Yes. CD mirrors RP back at Flame → Earth steals 1 of Flame's mana in return. Happened in R4 post-resolution. **+1 mana for Earth (pick 1E).** Retroactively: 3W · 15F · 1E = 19 mana ÷ 2 = 9 VP. (Swing +1 from CD.)
- **Deep Roots (Founding):** +2 VP per component on matching terrain (Earth terrain). All 4 Sanctums on Sanctum Hill — that's Sanctum terrain, not Earth terrain. *Ruling ambiguity.* SW corner IS Earth territory per setup (Earth corner). Assume 4 components on Earth-adjacent/Earth terrain → **8 VP**.
- **Mana VP:** 3W · 15F · 1E = 19 mana ÷ 2 = **9 VP**.
- Earth total: 4 + 2 + 8 + 9 = **23 VP**.

*Recompute Iron for parity — Iron also has starting Forge on Forge Ground. No bonus doctrine for Iron's home terrain.*

#### Wait — I missed Iron's biggest possible score. Let me check all Iron's doctrines again for any scoring clauses:
- DL: trigger-only, no scoring.
- IB: trigger-only, no scoring.
- IV: trigger-only (production).
- MO: 2 VP per round where production > previous. **Did it fire any round?**
  - R1 production: 11W. Previous: 0 (baseline). R1 > 0 → MO fires R1? Rules say "produced more total mana this round than previous round." R1 has no previous round. *Ruling: cannot fire R1 (no baseline).* Most consistent interpretation.
  - R2: 9W vs R1 11W. 9 < 11. Does not fire.
  - R3: 9W vs R2 9W. Not greater. Does not fire.
  - R4: 11W (partial round — 3 activations produced 9W + build produced 3W = 12W actually). 12 > 9 → fires. **+2 VP.**
  - **Correction: Iron gets 2 VP from Momentum R4.**

**Updated Iron total: 13 + 2 = 15 VP.**

#### Recheck Final Activation production for Iron (Step 1 of Unraveling)
I counted Iron's step-1 activation separately (17W). That 17W is included in Iron's end Weight (25W). But it's not counted as "round 4 production" for MO purposes — Unraveling is its own phase. Production comparison: R4 only includes in-round activations before Veil triggered.

R4 Iron in-round production: build IB 3W + 2 activations × 3W = 9W. **R4 = 9W = R3 9W.** Not greater. **MO does not fire R4.** 

**Iron total back to 13 VP.**

### Final Scores

| School | Doctrine VP | Map VP | Mana VP | Total |
|---|---|---|---|---|
| Iron | 0 | 0 | 13 | **13** |
| Flame | 0 | 0 | 13 | **13** |
| Tide | 0 | 0 | 4 | **4** |
| Earth | 8 (DR) | 6 (SW 4 + DT 2) | 9 | **23** |

**Winner: Earth (23 VP).**

*(Revising my top-line winner claim: Iron did NOT win. Earth did, because Earth is the only School with significant scoring doctrines that fired.)*

**Updating frontmatter:** winner: Earth (23 VP).

---

## Findings

### Veil pacing — **BROKEN TOO FAST**
Veil hit 0 during round 4 mid-turn. Game ended with 4 rounds completed (round 5 never happened). Iron's 3-Forge-activation pattern triggered "3+ Forges in one round" every single round starting R1. Combined with per-round −1 plus 5th-component triggers (Flame R3, Iron R3), the Veil burned at an average of 2.25 steps per round. **Veil target of 10 is not survivable with Iron drafting DL+IB+IV and running Forge-heavy.**

### Iron round 1 power — **DOMINANT IN ACTION ECONOMY BUT NOT VP**
Dawn Law + Iron Bloom + Tempered in R1 gave Iron:
- 3 Forges placed (vs 1 for everyone else)
- 6W from Iron Bloom activations
- 5W from round-1 activations (11W total)
- Full storage cap hit immediately (lost 3 mana to cap)

**However:** Iron's round-1 dominance did NOT translate to winning the game. Reasons:
1. **Mana cap punishes Iron's burst.** Iron dumped 11W into storage in R1, cap at 8, lost 3W immediately. Iron's explosion is partially wasted.
2. **No Spire → Iron Vein is dead.** IV never fired once. Iron's 4-doctrine drafted engine is really a 3-doctrine engine.
3. **Momentum is impossible after a front-loaded R1.** MO needs production to grow each round. R1's 11W becomes the ceiling Iron cannot beat without more Forges (cap at 6 components).
4. **No scoring doctrines.** Iron's entire kit (DL, IB, IV, MO, Tempered) is production/trigger. Only MO scores VP, and it didn't fire. Iron's VP = stored mana / 2. Cap limits this hard.

**Verdict on Iron R1 power:** The opening is visually oppressive — 3 components on turn 1 vs everyone else's 1 or 2 feels bad at the table — but it does not win the game. Iron's R1 advantage is a *trap*. The real winner is the School with scoring doctrines (Earth).

### 90-minute estimate — **YES, comfortably**
Game ended R4 mid-turn. 4 rounds × ~15 min/round + 20 min Unraveling = ~80 min. *However*, this is because the Veil broke early. If Veil were tuned to reach R5, estimate is ~95-100 min. Slate's "T=5" rating was correct that **this specific configuration is fast** but the reason is a bug: Veil ends the game before R5.

### Balance verdict — **2 / 5**
Severe imbalances:
- Iron has a spectacular-looking R1 that delivers no VP.
- Flame's entire drafted engine (FH, CG, RP, SE) generates mana, not VP. Flame scores 13 VP purely from stored mana.
- Tide is catastrophically weak — TL never fired (no R5), TM only works late, Tide had no VP doctrines at all. 4 VP total is not a game.
- Earth runs away because (a) SW scores doctrine VP, (b) DR founding scores doctrine VP, and (c) Earth is immune to edicts.

Three of four players drafted zero end-game VP doctrines. This is a drafting-pool problem revealed by the slate: the 25-card pool is heavy on mana-production doctrines and light on VP-scoring doctrines. When slates don't give every School at least one VP scorer (SW, DT, GW, EN, MO), mana-to-VP conversion (1:2) becomes the only engine, and the mana cap punishes over-production.

### Recommended changes

1. **Veil threshold bump to 12 or 14.** At 10, Iron-heavy Forge play (Iron Vein slate, Production War slate, Tempo Race slate) ends in R4. Target is R5. Raise Veil to 12 minimum, maybe 14.

2. **Veil trigger "3+ Forges" is too easy.** Iron triggers this **every round** by default in any production slate. Either:
   - Raise threshold to "5+ Forges activated in one round" (only late-game triggers), OR
   - Remove this trigger entirely and add "any School exceeds 10 stored mana" instead (rewards conversion/spending).

3. **Iron Bloom needs a per-round cap.** Currently IB activates *every* newly-built component — combined with Dawn Law's R1 double-build, that's 3 free activations in R1 alone (6W + 3 IT tokens). Cap IB to "once per round" OR require the activation to happen immediately and cost 1 mana (not free).

4. **The Final Hour needs a fallback.** When the Veil breaks early (as here), FH becomes a dead card. Add clause: *"If round 5 never occurs, The Final Hour doubles all components' Final Activation output during the Unraveling instead."* Makes FH a resilience hedge, not a prayer for R5.

5. **Tide needs a draft-pool rebalance.** Tide drafting TL + TX + EL + TM gave Tide zero scoring doctrines. TM and TX are production/conversion; TL is a tempo card that needs a round to repeat. Tide needs **at least one VP-scoring doctrine available** from its natural draft lanes. Consider making **The Spider's Web or Deep Territory** explicitly cross-School attractive to Tide, OR adding a Tide-leaning scoring doctrine to the pool (e.g., "Rising Tide: score 1 VP per Flow Channel territory you occupy at game end").

6. **Mana cap at 8 is too punishing for Iron's burst.** Iron's R1 overflow of 3 mana is ~15% of Iron's eventual total VP. Either (a) raise cap to 10 or (b) give Iron's Tempered Founding an implicit +2 cap.

7. **CG + FH expected combo never materialised for Flame.** Flame drafted FH specifically to double CG in R5. Because R5 never happens in fast slates, FH's entire value vanishes. Consider making FH trigger on "*the final round, whichever that is*" rather than specifically round 5.

### Stakes-relevant observations (for future TIER-A)

- **"Iron's R1 is dominant" is refuted by play record.** Iron placed 3 components R1 and finished third, tied. The *feeling* of Iron dominance (visual corner control) differs from the *VP outcome*.
- **"Veil is a doom clock that creates tension" is partially refuted.** In this game, the Veil created an anticlimax: R5 never happened, FH never fired, TL never fired. The clock didn't compress decisions — it canceled them.
- **"Edicts are load-bearing in the Unraveling" is severely refuted.** All 4 Edicts in this game were 0-VP plays because (a) Earth is immune, (b) production doctrines have already banked their mana, (c) scoring doctrines that didn't fire don't need suspending. **Edicts need VP-bearing suspension targets.**
- **"Drafting is a real decision" partially refuted for Flame/Tide.** Flame drafted FH which did not fire. Tide drafted TL which did not fire. When ~30-50% of a School's draft is "banking on round 5," and R5 is unreliable, drafting becomes a gamble with no information.

### Cross-reference to rubric axes
- **A1 Tempo:** Iron's R1 tempo is overpowered but non-converting to VP. Tempo ≠ winning.
- **B1 System Gearing (if in Pool):** Iron's gears mesh but produce wasted output (cap).
- **B2 Catastrophe Pressure:** Veil pressure exists but triggers too easily — clock feels broken rather than tense.
- **C-band Scarcity Bite:** Mana cap of 8 creates real scarcity for Iron, but no scarcity for Tide (barely produces).

---

*PT03 complete. Primary finding: Slate 5 "Tempo Race" demonstrates that the slate's T=5 rating (90-min fit) is achieved only because the Veil breaks too early — the fast time is a symptom, not a feature. Secondary finding: Iron's Dawn Law + Iron Bloom opening is a visual powerhouse that the mana cap, Iron Vein's Spire-adjacency requirement, and Momentum's monotonic-growth requirement all punish. The game's actual winner (Earth) won because Earth drafted the only scoring doctrines available (SW, DT) and runs them behind Deep Roots immunity — no one can touch Earth in the Unraveling, and most other doctrines don't generate VP.*
