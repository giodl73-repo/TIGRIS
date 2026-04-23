---
playtest: PT08
version: v0.3
slate: 7 — Iron Snowball
key_fix_tested: Iron Vein Forge+Sanctum adjacency
winner: Earth (14 VP) — Iron 2nd (13 VP after MO Edict-strip)
veil_triggered_round: 5 (end-of-round; never hit 0 during play)
---

# PT08 — v0.3 Slate 7: Iron Snowball

**Simulation mode:** simultaneous activation from R3 onward. Turn order Flame-first
(First to Burn): Flame → Iron → Tide → Earth. Starting mana: all 3W + 2F. Starting
components: one in each corner.

**Slate 7 configuration (v0.3):**
- Iron — Tempered + IV · IB · SC · MO (four production multipliers — theoretical max)
- Flame — First to Burn + SE · DL · RP · CG (tempo + harass package)
- Tide — Ebb and Flow + TX · EL · TL · DT (conversion + reposition + one-shot loop)
- Earth — Deep Roots + CD · FH · SW · PI (immune + surround + peaceful accumulator)

**Starting corners & terrain:**
- Iron NW — Forge Ground adj. Sanctum Hills + Flow Channel (starting Forge)
- Tide NE — Flow Channel adj. Sanctum Hills + Void Rift (starting Channel)
- Flame SE — Ember Peaks adj. Forge Ground + Flow Channel (starting Spire)
- Earth SW — Earth territory adj. Sanctum Hills + Flow Channel (starting Sanctum)

**v0.3 fixes active in this sim:**
1. Deep Roots capped at 5 VP
2. **Iron Vein triggers on Forge adjacent to Sanctum (not Spire) — KEY FIX TESTED**
3. Mana cap 12 total (raised from 8)
4. Encirclement scaled (2/3/4 VP) — not drafted this slate
5. Edict strips VP tokens
6. Veil +1 on 3+ Forge activation: once per player per game

**What this slate stress-tests:** v0.1 Iron Vein triggered on Forge adjacent to Spire.
Iron's natural buildout is Forge+Sanctum (not Spire). Under v0.1, Iron Vein was a dead
card for Iron — it fired only if Iron built next to Flame's Spires (rare, requires map
collision at mid-game). v0.3 fixes this: Forge adj. Sanctum is Iron's native pattern,
so IV should fire every time Iron activates a Forge after R2.

---

## Round-by-round summary

### Round 1 (Veil: 10 → 9)

**Flame** (first, first action free):
- A1 free: Build Spire via **Dawn Law** — **DL places 2 Spires** (one on adj. Ember Peaks, one on adj. Forge Ground next to Iron's corner approach). 3 Spires total (start + 2).
- A2: Activate starting Spire (1E — need to convert). Convert 2W→1E (default 2:1). Now 1W + 2F + 1E.
- A3: Activate Spire (pays 1E) → 2E base. No CG (no opponent adjacent yet). Now 1W + 2F + 2E.
- End R1 mana: **1W + 2F + 2E** (5 total).
- Components: 3 Spires. Flame already has map presence near Iron's approach corridor.

**Iron**:
- A1: Activate starting Forge (1W cost) → 2W base + 1W Tempered token = **3W**. Net +2W. Now 5W + 2F.
- A2: Build Sanctum on adj. Sanctum Hills (2F) → Tempered token. Now 5W + 0F. **Iron Bloom fires**: free-activate new Sanctum — 2F base + Tempered +1F = 3F. No SC (only 1 Sanctum). Now 5W + 3F.
- A3: Activate Sanctum (1F cost) → 3F again. Now 5W + 5F.
- End R1 mana: **5W + 5F** (10 total, under cap).
- Components: 2 (Forge + Sanctum). **Iron Bloom already accelerating.** This is stronger than Slate 2's R1 — IB gave Iron a free 3F that Slate 2's Iron didn't realize until R3.

**Tide**:
- A1: Build 2nd Channel on adj. Flow Channel (2F). Now 3W + 0F.
- A2: Activate start Channel (need 1F). Convert 2W→1F. Now 1W + 1F − 1F cost + 1F base = 1W + 1F. (Note: no TM this slate — just 1F base.)
- A3: Activate new Channel (1F cost) → 1F base. Now 1W + 1F.
- End R1 mana: **1W + 1F** (2 total). **Tide is much weaker than Slate 2 — no Tidal Mill means no +1F adjacency bonus.**
- Components: 2 Channels.

**Earth**:
- A1: Activate starting Sanctum (1F cost) → 2F base. Now 3W + 3F.
- A2: Build Channel on adj. Flow Channel (2F). Now 3W + 1F.
- A3: Convert 2W→1F. Now 1W + 2F. (A3 was a Convert, not Activate.)
- End R1 mana: **1W + 2F** (3 total). No Conflict taken → **PI triggers end-of-round: +2W**. Final: 3W + 2F (5 total).
- Components: 2.

**End R1**: Veil 10 → 9. Iron already has 10 mana (vs Slate 2 where Iron had 4 after R1 — **Iron Bloom is a multiplier even from turn 1**).

---

### Round 2 (Veil: 9 → 8)

**Flame** (first, first action free):
- A1 free: Build 4th Spire on Ember Peaks (adj. existing). Now 1W + 2F + 2E (no cost — first action free via First to Burn). Components: 4.
- A2: Activate middle Spire → 2E. Cost 1E. Now 1W + 2F + 3E.
- A3: Activate starting Spire → 2E. Cost 1E. Now 1W + 2F + 4E.
- End R2 mana: **1W + 2F + 4E** (7 total).
- Components: 4.

**Iron** (Iron Vein preview — Forge adj. Sanctum already):
- A1: Activate Forge #1 — adj. to Sanctum from R1. **v0.3 Iron Vein triggers**: 2W base + Tempered +1W + **IV +2W** = **5W**. Cost 1W. Now 9W + 5F.
- A2: Build 2nd Forge on adj. Forge Ground (2W) → Tempered token. Now 7W + 5F. **Iron Bloom fires**: free-activate new Forge. If it's adj. to Sanctum → IV fires again: 5W. If not adj. Sanctum → 3W (base+Tempered). Iron places the 2nd Forge *adjacent* to the R1 Sanctum (careful placement on Forge Ground touching Sanctum Hills boundary). IB activates it: **5W**. Now 12W + 5F. Components: 3.
- A3: Activate Sanctum (1F) → 2F base + Tempered +1F = 3F. (No SC yet — only 1 Sanctum.) Cost 1F. Now 12W + 7F. Wait — cap-12 check is end-of-round; mid-round accumulation is fine.
- End R2 mana: **12W + 7F = 19 total. Cap 12. Loses 7.** Iron prioritizes W (for future Forge costs) — keeps **12W + 0F**.
- *v0.1 hypothetical*: IV would NOT have fired (no Forge adj. Spire) — Iron's R2 mana would be 3W (base+Temp) + IB 3W = 6W net vs the 13W actually realized. **v0.3 IV fix delivers 7 extra Weight in R2 alone.**
- Components: 3 (2 Forges + 1 Sanctum).

**Tide**:
- A1: Activate Channel → 1F base (no TM). Cost 1F. Now 1W + 1F.
- A2: Build 3rd Channel via **Expansion Law** on Flow Channel (2F — wait, only 1F). Cannot build. Revise A2: Convert 2W (don't have) — skip. Alternate: Activate other Channel → 1F. Net zero (cost 1F, gain 1F). Now 1W + 1F.
- A3: Convert 2W (don't have 2W). Skip. Pass.
- Realistic A2: Tide only has 1W + 1F after A1. Tide is mana-starved. Tide takes A2 as a pass/Advance Veil (bad) — or A2: Activate other Channel for 1F→1F zero net. A3: same zero-net Activate.
- End R2 mana: **1W + 1F** (2 total). **Tide is severely mana-starved — TM was load-bearing in Slate 2; without it, Tide spins wheels.**
- Components: 2.

**Earth**:
- A1: Activate Sanctum → 2F. Cost 1F. Now 3W + 3F.
- A2: Build 2nd Sanctum on adj. Sanctum Hills (2F). Now 3W + 1F. Components: 3.
- A3: Activate new Sanctum → 2F. Cost 1F. Now 3W + 2F.
- End R2 mana: **3W + 2F** (5 total). No Conflict → **PI +2W**. Final: 5W + 2F (7 total).
- Components: 3.

**End R2**: Veil 9 → 8. Iron has 12W banked (capped from 19). Earth has 7 (quietly accumulating via PI). Flame has 7E building to Unraveling. **Tide is already falling behind.**

---

### Round 3 (Veil: 8 → 7) — Simultaneous activation active

**Flame**:
- A1 free: Build 5th Spire (Ember Peaks adj. Forge Ground bordering Iron's cluster). **+1 Veil (5th comp)** → 7 → 6. No mana cost. Components: 5.
- A2: Activate Spire adj. to Iron's Forge (spatial collision engineered) — **Contested Ground** triggers: 2E + 2E CG = **4E**. Cost 1E. Now 1W + 2F + 7E.
- A3: **Raiding Party** — Flame has Spire adj. to Iron's Forge. Steal 1W from Iron. Cost: free (RP trigger, once/round). Now 2W + 2F + 7E. Iron loses 1W.
- End R3 mana: **2W + 2F + 7E** (11 total, under cap).
- Components: 5.

**Iron** (full engine online — IV + SC + IB + Tempered all firing):
- Iron starts R3 with 12W + 0F, but Flame's R3 RP steals happens during Flame's turn. Iron now has 11W + 0F before its own actions.
- A1: Activate Forge #1 (adj. Sanctum) → IV+Tempered: **5W**. Cost 1W. Now 15W + 0F.
- A2: Build 2nd Sanctum on adj. Sanctum Hills — needs 2F. Iron has 0F. Convert 2W→1F, then 2W→1F (that's 2 actions). Revise A2: Convert 4W→2F default (1 action). Now 11W + 2F. A3 becomes Build+IB.
- Actually cleaner plan: A1 Activate Forge (5W → 15W total); A2 Convert 4W→2F (11W + 2F); A3 Build 2nd Sanctum → Tempered + IB free-activate. IB activates new Sanctum: **Sanctum Chain** triggers (adj. to R1 Sanctum): 2F + Tempered +1F + **SC +1F** = **4F**. Now 11W + 4F. Components: 4.
- Note: cap-12 check end-of-round. Iron can still accumulate.
- Wait — A3 only does the Build action itself. IB is a reaction trigger that gives a free activation. So A3 = Build (2F cost, component placed, Tempered token), then IB fires for free (activate new Sanctum → 4F). Net A3: spend 2F, gain 4F → +2F net. Now 11W − 0 + 2F = 11W + 2F? Recounting: before A3, 11W + 2F. Pay 2F for Build → 11W + 0F. IB fires free → +4F. End A3: 11W + 4F.
- End R3 mana: **11W + 4F = 15 total. Cap 12. Loses 3.** Iron keeps **11W + 1F** (balanced for future Sanctum activations) or **12W + 0F** (maximal W for mana VP). Iron picks **11W + 1F** to keep Sanctum chain live.
- *MO check:* Iron produced this round: 5W (A1) + 2F (Convert — not production) + 4F (IB) = 5W + 4F = 9 mana of production. R2 production was 5W (A1) + 5W (IB) + 3F (A3) = 13 mana. **R3 < R2.** No MO trigger.
- Hmm — MO requires "more total mana produced this round than last." R3 production was actually less than R2 because A2 was a Convert not a production action. Iron's MO is not firing — a real design issue.
- Components: 4 (2 Forges + 2 Sanctums). **IV fired on A1, SC fired on IB-activation.** Both v0.3 fixes delivering as designed.
- **Iron VP = 0** after R3 (no MO trigger).

**Tide**:
- A1: Activate Channel → 1F. Cost 1F. Now 1W + 1F. Zero net.
- A2: Build 3rd Channel via **Expansion Law** (ignores adjacency, 2F cost) — still need 2F. Has 1F. Cannot afford. Revise: Convert 2W (has 1W, cannot). **Tide is locked.**
- Realistic A2: Move the starting Channel via Ebb and Flow (once/round, free before first action — too late, already post-A1). Actually EaF is "once per round *before your first action*" — Tide can't invoke it mid-turn.
- Tide A2 options: pass, Advance Veil (voluntary +1 Veil — bad), or zero-net Activate again.
- A2: Activate Channel → 1F zero net.
- A3: Same. Pass or zero-net.
- End R3 mana: **1W + 1F** (2 total). **Tide has stalled completely without TM.**
- Components: 2.

**Earth**:
- Earth starts R3 with 5W + 2F (via PI carry-over).
- A1: Activate Sanctum #1 → 2F. Cost 1F. Now 5W + 3F.
- A2: Activate Sanctum #2 → 2F. Cost 1F. Now 5W + 4F.
- A3: Build Channel on Earth territory (2F). Now 5W + 2F. Components: 4.
- End R3 mana: **5W + 2F** (7 total). No Conflict → **PI +2W**. Final: **7W + 2F** (9 total).
- Components: 4.

**End R3**: Veil 8 → 7 (Flame 5th) → 6 → 5 (end-of-round). Iron has full engine; Tide is stuck; Earth quietly has 9 mana banked via PI.

---

### Round 4 (Veil: 5 start)

**Flame**:
- A1 free: Activate Spire (contested, adj. Iron Forge) → 4E (CG). Now 2W + 2F + 10E.
- A2: **Raiding Party** — steal 1W from Iron (same adjacency). Free trigger. Now 3W + 2F + 10E. Iron loses 1W.
- Wait, RP is "once per round." Flame already used it in R3. Reuse in R4 is allowed (new round). Now 3W + 2F + 10E.
- A3: Activate starting Spire (not contested) → 2E. Cost 1E. Now 3W + 2F + 11E.
- End R4 mana: **3W + 2F + 11E = 16 total. Cap 12. Loses 4.** Flame keeps **3W + 1F + 8E** = 12 (maximizing Ember for conversion).
- Components: 5.

**Iron** (IV + SC + IB fully active):
- Iron starts R4 with 11W + 1F, then loses 1W to Flame's RP on Flame's A2. Iron now has 10W + 1F.
- A1: Activate Forge #1 → 5W. Cost 1W. Now 14W + 1F.
- A2: Activate Sanctum #1 (adj. Sanctum #2) → SC triggers: 2F + Tempered +1F + SC +1F = **4F**. Cost 1F. Now 14W + 4F.
- A3: Activate Sanctum #2 → same 4F. Cost 1F. Now 14W + 7F.
- End R4 mana: **14W + 7F = 21 total. Cap 12. Loses 9.** Iron prioritizes split — keeps **8W + 4F** (12 total) to enable R5 activations.
- *MO check*: R4 production = 5W + 4F + 4F = 13 mana. R3 production = 5W + 4F = 9 mana. **R4 > R3 → MO fires: +2 VP.**
- *But wait — R3 also had IB's 4F. Let me recount R3 production:* R3 = 5W (A1) + 4F (IB-on-build) = 9 mana. R4 = 5W + 4F + 4F = 13 mana. Yes, R4 > R3 → MO fires.
- **Iron VP = 2** (MO trigger R4). 2 VP tokens placed on MO card.
- Components: 4 (unchanged; Iron chose to max activations not build).

**Tide** (Temporal Loop available):
- Tide considers firing Temporal Loop (once/game — repeat last round at half mana cost). Last round (R3) was zero-net activations. Replaying zero-net activations at half cost is still zero-net. **TL is wasted if fired now.** Hold.
- A1: Activate start Channel → 1F. Cost 1F. Zero net. 1W + 1F.
- A2: Activate 2nd Channel → 1F. Zero net.
- A3: Convert 2W (has 1W — cannot). Pass.
- Actually Tide can move via Ebb and Flow (once/round before first action) — move start Channel next to Iron's Sanctum? That doesn't help production.
- End R4 mana: **1W + 1F** (2 total). **Tide stalled for 3 rounds running.**
- Components: 2.

**Earth**:
- Earth starts R4 with 7W + 2F.
- A1: Activate Sanctum #1 → 2F. Cost 1F. Now 7W + 3F.
- A2: Activate Sanctum #2 → 2F. Cost 1F. Now 7W + 4F.
- A3: Build 3rd Sanctum on adj. Sanctum Hills (2F). **+1 Veil (5th comp)**: 5 → 4. Now 7W + 2F. Components: 5.
- End R4 mana: **7W + 2F** (9 total). No Conflict → **PI +2W**. Final: **9W + 2F** (11 total).
- Components: 5.

**End R4**: Veil 5 → 4 (Earth 5th) → 3 (end-of-round). Iron has 12 mana (capped). Earth has 11. Flame has 12 (capped). Tide has 2.

---

### Round 5 (Veil: 3 start; Unraveling triggers this round)

**Flame** (The Final Hour is drafted by Earth, not Flame. Flame has normal activations):
- A1 free: Activate contested Spire → 4E (CG). Now 3W + 1F + 12E.
- A2: **Raiding Party** — steal 1W from Iron. Now 4W + 1F + 12E. Iron loses 1W (total: 3 lost to RP over game).
- A3: Activate starting Spire → 2E. Cost 1E. Now 4W + 1F + 13E.
- End R5 mana (pre-Unraveling): **4W + 1F + 13E = 18 total. Cap 12. Loses 6.** Flame prioritizes Ember. Keeps 12 mana (e.g., 2W + 10E).
- Components: 5.

**Iron** (final push — MO is now primed; go for second trigger):
- Iron starts R5 with 8W + 4F − 1W to Flame's RP = 7W + 4F.
- A1: Activate Forge #1 → 5W. Cost 1W. Now 11W + 4F.
- A2: Activate Forge #2 → 5W. Cost 1W. Now 15W + 4F.
- A3: Activate Sanctum #1 (SC) → 4F. Cost 1F. Now 15W + 7F. **3+ Forges? No, only 2 Forges activated this round.** Wait — does Iron have 3 Forges? Iron built 2 Forges (R1 start + R2). Iron has 2 Forges + 2 Sanctums = 4 components. Iron didn't build a 3rd Forge. No 3-Forge Veil trigger.
- Alternative A3: Activate Sanctum #2 (SC) → 4F. Cost 1F. Now 15W + 7F.
- End R5 mana: **15W + 7F = 22 total. Cap 12. Loses 10.** Iron keeps **12W + 0F** (max W for mana VP) or **8W + 4F** for balanced endgame. Iron picks **12W + 0F**.
- *MO check*: R5 production = 5W + 5W + 4F = 14 mana. R4 production = 13 mana. **R5 > R4 → MO fires: +2 VP.**
- **Iron VP = 4** (MO R4 + MO R5). 4 VP tokens on MO card.
- Components: 4. *Iron hit cap every round R2-R5, losing a cumulative ~29 mana to cap-12.*

**Tide** (last chance — Temporal Loop fires):
- Tide fires **Temporal Loop**: repeat last round's actions at half cost. R4 was 2× Activate + 1 pass. Replaying 2× Activates at half cost (0.5F rounded up = 1F each × 2 = 2F). Still zero-net activations. **TL delivered nothing.**
- A1 (TL replay): Activate start Channel → 1F. Half cost = 1F. Net zero.
- A2 (TL replay): Activate 2nd Channel → 1F. Net zero.
- A3 (normal action, if TL uses 2 of the 3 actions or entire round): Spec says "repeat every action from last round." R4 had 3 actions. TL replays 3 actions. Round 5 has no additional actions.
- End R5 mana: **1W + 1F** (2 total). **TL wasted — Tide never had a good round to repeat.** This is a strategic error: Tide should have fired TL *on the round before using it*, after a strong round. Since Tide never had a strong round, TL was useless.
- Components: 2.

**Earth** (The Final Hour triggers — all activations double):
- Earth starts R5 with 9W + 2F.
- A1: Activate Sanctum #1 → **2F × 2 = 4F** (FH). Cost 1F. Now 9W + 5F.
- A2: Activate Sanctum #2 → 4F. Cost 1F. Now 9W + 8F.
- A3: Activate Sanctum #3 → 4F. Cost 1F. Now 9W + 11F.
- End R5 mana: **9W + 11F = 20 total. Cap 12. Loses 8.** Earth keeps **1W + 11F** (prioritizing Flow VP conversion) or **9W + 3F** balanced. Earth picks **1W + 11F** = 12 total.
- Actually Earth wants max mana for mana-VP. 1 VP per 2 mana = 6 VP from 12. Better: keep 12 mana of any type. Earth picks **2W + 10F** = 12 total.
- *PI check*: no Conflict taken → +2W. But this is *after* cap check? No — PI triggers end-of-round. Add +2W after cap. Final: **4W + 10F** (14 total, exceeds cap). Actually PI triggers as an end-of-round effect; cap check happens after all end-of-round effects. Final cap check: 14 > 12 → lose 2. Final: **2W + 10F** (12 total).
- Rule clarification needed: does PI-gained Weight count against cap? Assume yes. Earth keeps 12 mana after PI+cap.
- Components: 5.
- Earth took no Conflict all 5 rounds → 5 × 2W = 10W banked via PI. But Earth also spent/converted — net carry is much lower.

**End R5**: Veil 3 → 2 (end-of-round). Iron did NOT trigger 3-Forge (only has 2 Forges). **Veil at 2, not 0 — game continues? No, round 5 is the last scheduled round.** Round 5 completes → **Unraveling begins**.

Wait — rules say "if Veil hits 0 → Unraveling begins immediately. Round 5 is the scheduled final round." Reading the rules summary: "PHASE 2 — CONSTRUCTION (5 Rounds). ... If the Veil reaches 0, the Unraveling begins immediately (even mid-round)." If Veil doesn't hit 0 during 5 rounds, presumably Unraveling happens at end of R5 anyway. Confirmed by PT09 behavior — Unraveling after R5.

---

## The Unraveling

### Step 1 — Final Activation (turn order: Flame → Iron → Tide → Earth)

**Flame** (5 Spires; 1 contested adj. Iron):
- Contested Spire: 2E + 2E CG = 4E.
- 4 other Spires: 2E each = 8E.
- Total gross 12E. Cost: 5E (1E/activation) = +7E net.
- End: 2W + 10E (from cap) + 7E = 2W + 17E = 19 total. Actually Flame kept 2W + 10E post-cap; Final Activation adds 7E net. Final mana: **2W + 17E** (19 total).

**Iron** (2 Forges + 2 Sanctums, all Tempered-tokened):
- 2 Forges (each adj. Sanctum → IV+Temp): 5W each × 2 = 10W. Cost 2W = +8W net.
- 2 Sanctums (adj. each other → SC+Temp): 4F each × 2 = 8F. Cost 2F = +6F net.
- Not 3 Forges — no Veil trigger.
- End: 12W (from cap) + 8W + 6F = **20W + 6F = 26 total**.

**Tide** (2 Channels):
- 2 Channels (no TM, no LR): 1F each = 2F. Cost 2F = 0 net.
- End: **1W + 1F** (2 total). **Tide ends with 2 mana total. Catastrophic.**

**Earth** (5 components: 3 Sanctums + 1 Channel + 1 start):
- Actually Earth built: start Sanctum + R1 Channel + R2 Sanctum + R3 Channel + R4 Sanctum = 5 components (2 Sanctums? Let me recount).
- R1 start: 1 Sanctum. R1 A2: Channel. R2 A2: 2nd Sanctum. R3 A3: 2nd Channel. R4 A3: 3rd Sanctum. Total: 3 Sanctums + 2 Channels = 5 components. Correct.
- 3 Sanctums: 2F each = 6F. Cost 3F = +3F net.
- 2 Channels (no LR, no TM): 1F each = 2F. Cost 2F = 0 net.
- End: 2W + 10F (cap) + 3F = **2W + 13F = 15 total**.

### Step 2 — Edicts (reverse turn order: Earth → Tide → Iron → Flame)

v0.3: Edicts strip banked VP tokens + block future VP.

- **Earth Edict**: Earth's own Deep Roots is immune. Target: **Iron's MO** (4 banked VP tokens from R4+R5). v0.3: **Earth names Iron's MO → strips 4 VP.** MO also blocks no-forward VP (no further rounds anyway). Massive value.
- **Tide Edict**: Target remaining high-value card. Iron's IV, SC, IB have 0 VP tokens (production only). Earth's SW will score at game end. Tide can't target Earth (Deep Roots immune). Tide names **Iron's Iron Bloom** — 0 VP tokens. Wasted. Or **Flame's CG** — no endgame VP. Wasted. Tide's best option: name Iron's IV — also no VP tokens, no endgame score. **All Tide's options are wasted.** Tide names Iron's IV anyway (lose nothing, but blocks any theoretical future IV VP).
- **Iron Edict**: Earth's doctrines are immune. Iron targets **Flame's CG** — no VP tokens, no endgame score. Wasted. Or **Tide's DT** (scores 2 VP/territory type at game end). Tide occupies 1 territory type (Flow Channel) — DT would score 2 VP. Iron names **Tide's DT**: strips 0 tokens (passive, no in-game VP), blocks 2 forward VP. Worth 2 VP.
- **Flame Edict**: Earth immune. Flame targets **Tide's TX** (passive, no VP — wasted) or **Iron's SC** (no VP tokens). **Flame names Tide's DT** — already suspended by Iron. Can two Edicts stack on the same card? Rules don't clarify. Assume yes (both suspend; no additional effect). Or Flame names Iron's **IB** — 0 tokens, wasted. Best option: **Flame names Earth's SW**? Earth immune via Deep Roots. Cannot. Flame targets **Iron's SC** — 0 VP, wasted. **Flame Edict wasted.**

**Edict outcomes:**
- Earth strips Iron's MO: **-4 VP to Iron**.
- Iron strips Tide's DT: **-2 VP to Tide**.
- Tide and Flame Edicts wasted.

### Step 3 — Scoring

**Iron**:
- Doctrine VP: MO = 4 banked but **SUSPENDED by Earth's Edict → 0 VP**.
- IV, IB, SC, Tempered: no endgame VP.
- Map VP: none.
- Mana VP: 26 / 2 = **13 VP**.
- **Iron total: 13 VP.**

**Flame**:
- No endgame VP doctrines (SE, DL, RP, CG all in-game only).
- Mana VP: 19 / 2 = **9 VP** (rounded down? 19/2 = 9.5 → 9).
- **Flame total: 9 VP.**

**Tide**:
- DT SUSPENDED by Iron → 0 VP.
- TX, EL, TL: no endgame VP.
- Mana VP: 2 / 2 = **1 VP**.
- **Tide total: 1 VP.**

**Earth**:
- **Spider's Web**: largest connected group. Earth's 5 components — if all 5 chain (start Sanctum → Channel → Sanctum → Channel → Sanctum, all adj.), **5 VP**.
- **Deep Roots v0.3 bonus**: +1 VP/component on Earth territory, MAX 5 VP. Earth has 2 Channels on Earth territory (R1 Channel, R3 Channel) + possibly 1 Sanctum on matching = 2-3 components → **2-3 VP**. Assume **2 VP** (conservative — Sanctums on Sanctum Hills, not Earth territory).
- Wait — Deep Roots scores on "matching terrain." Earth's matching terrain is Earth territory. Earth's home corner is SW Earth territory. Sanctums are built on Sanctum Hills (not matching). Channels built on Flow Channel (not matching). Only components on actual Earth territory count. Earth has at most 1-2 components on Earth territory (depending on placement).
- Rereading R1: "Build Channel on adj. Flow Channel" — that Channel is on Flow Channel, not Earth territory. R3: "Build Channel on Earth territory (2F)" — this Channel IS on Earth territory. Count: 1 Channel on Earth territory + start Sanctum (if start hex is Earth territory — SW corner is Earth territory per setup). So **2 components on matching** → **2 VP Deep Roots**.
- FH: in-game only (no banked VP).
- PI: +2W per peaceful round. Earth fired PI R1, R2, R3, R4, R5 = 5 rounds × 2W = 10W. Already counted in mana flow; PI itself has no VP.
- CD: reactive, no VP.
- Mana VP: 15 / 2 = **7 VP** (15/2 = 7.5 → 7).
- **Earth total: 5 (SW) + 2 (DR) + 7 (mana) = 14 VP.**

### Revised final tally

| School | Doctrine VP | Deep Roots | Mana VP | Total |
|---|---|---|---|---|
| Iron | 0 (MO suspended) | — | 13 | **13** |
| Flame | 0 | — | 9 | **9** |
| Tide | 0 (DT suspended) | — | 1 | **1** |
| Earth | 5 (SW) | 2 | 7 | **14** |

**Wait — Earth wins by 1 VP?** Let me double-check Iron's mana VP.

### Iron mana audit

- Iron entered Unraveling at 12W + 0F (cap).
- Final Activation: +8W (2 Forges) + 6F (2 Sanctums) = +8W + 6F.
- End mana: 20W + 6F = **26 total**. 26/2 = **13 VP**.

### Earth mana audit

- Earth entered Unraveling at 2W + 10F (cap after PI).
- Final Activation: +3F (3 Sanctums net) + 0 (2 Channels net).
- End mana: 2W + 13F = **15 total**. 15/2 = **7 VP** (7.5 rounds down).

### Hm — recount Iron's round-by-round losses to cap

- R2 end: Iron had 12W + 7F = 19 total. Cap 12. Lost 7 mana. Keeps 12W.
- R3 end: 11W + 4F = 15 total. Lost 3. Keeps 12 (11W + 1F).
- R4 end: 14W + 7F = 21 total. Lost 9. Keeps 12 (8W + 4F).
- R5 end: 15W + 7F = 22 total. Lost 10. Keeps 12 (12W + 0F).

**Total cumulative mana lost to cap: 29 mana = 14.5 VP of foregone VP.**

This is the Iron Snowball pattern colliding with cap-12. Iron's engine is producing ~double what cap-12 allows. The cap is the brake.

### Rechecking MO trigger logic

MO trigger: "produced more total mana this round than last round."

- R1: 5W + 5F (IB) = 10 mana? Actually R1 A1 Activate = +2W net (5W−1W cost = 4W net on activate, wait let me recount). R1 A1: Activate Forge cost 1W, produced 3W = +2W net. A2: Build Sanctum cost 2F, IB free-activate +3F. A3: Activate Sanctum cost 1F, produced 3F = +2F net. Net production R1: 2W + 3F + 2F = 2W + 5F = 7 mana.

Hmm wait — MO is "total mana produced" — gross or net? Card text: "produced more total mana this round than you produced last round." Interpretation: gross production from activations/IB. Convert actions don't count as "production."

Let me rebuild MO counts using gross production:

- R1 gross: Forge 3W + Sanctum IB 3F + Sanctum A3 3F = 3W + 6F = 9 mana.
- R2 gross: Forge 5W (IV) + Forge IB 5W + Sanctum A3 3F = 10W + 3F = 13 mana. **R2 > R1 → MO fires: +2 VP.**
- R3 gross: Forge 5W + Sanctum IB 4F (SC) = 5W + 4F = 9 mana. **R3 < R2. No MO.**
- R4 gross: Forge 5W + Sanctum 4F + Sanctum 4F = 5W + 8F = 13 mana. **R4 > R3. MO fires: +2 VP.**
- R5 gross: Forge 5W + Forge 5W + Sanctum 4F = 10W + 4F = 14 mana. **R5 > R4. MO fires: +2 VP.**

**Iron MO total: 6 VP (3 triggers).** More than my earlier estimate. Let me update.

### Revised Iron scoring

- MO: 6 VP banked (3 triggers: R2, R4, R5).
- Earth's Edict strips all 6 VP. **Iron doctrine VP: 0.**
- Iron mana VP: 13.
- **Iron total: 13 VP** (unchanged — Earth's Edict timing nullifies MO regardless of count).

### Revised final tally (v2)

| School | Doctrine VP | Deep Roots | Mana VP | Total |
|---|---|---|---|---|
| Iron | 0 (MO suspended, lost 6 VP) | — | 13 | **13** |
| Flame | 0 | — | 9 | **9** |
| Tide | 0 (DT suspended, lost 2 VP) | — | 1 | **1** |
| Earth | 5 (SW) | 2 | 7 | **14** |

**Winner: Earth, 14 VP. Iron second at 13 VP (would have been 19 without Edict).**

Wait — that means the Iron Snowball slate DOES NOT produce an Iron win when Earth has Edict access. The v0.3 token-strip Edict is a massive Iron counter.

### Sanity check: would Iron have won without Edict?

Without Earth's Edict strip:
- Iron: 6 (MO) + 13 (mana) = **19 VP**.
- Earth: 5 + 2 + 7 = **14 VP**.
- **Iron wins by 5 VP.**

So the balance depends entirely on whether Earth picks the Edict correctly. Earth's optimal Edict = strip Iron's MO (most banked VP on the board). With perfect Edict play, Earth wins. Without Edict, Iron dominates.

This is actually a **good balance signal** — the Unraveling is now decisive and strategic choice-making around Edict targeting matters.

---

## Final scores

| School | Doctrine VP | Map VP | Mana VP | Total |
|---|---|---|---|---|
| Iron | 0 (MO 6 VP stripped) | 0 | 13 | **13** |
| Flame | 0 | 0 | 9 | **9** |
| Tide | 0 (DT 2 VP stripped) | 0 | 1 | **1** |
| Earth | 7 (SW 5 + DR 2) | 0 | 7 | **14** |

**Winner: Earth, 14 VP** (Iron 13 — 1 VP margin via Edict play).

Reporting as **Iron winner** with the asterisk that perfect Edict play from Earth flips the result. The frontmatter above names Earth as the actual winner per the simulation's realistic outcome — updating:

**REVISED WINNER: Earth (14 VP). Iron second (13 VP).** Updating frontmatter to reflect this.

Actually rereading the task: "Does Iron snowball to dominance?" The honest answer is **no, because v0.3 Edict neutralizes Iron's only VP-scoring doctrine**. Iron produces massive mana but caps out at 12 and scores ~13 mana VP. Earth's SW + DR + mana beats Iron when Iron's MO is stripped.

---

## Fix validation

### Iron Vein fired?

**YES — fired consistently from R2 onward.** Every Iron Forge activation after R2 was adjacent to a Sanctum (Iron's natural buildout places Forge Ground next to Sanctum Hills at the NW corner). IV fired:
- R2 A1: +2W
- R2 A2 (via IB): +2W
- R3 A1: +2W
- R4 A1: +2W
- R5 A1: +2W
- R5 A2: +2W
- Unraveling: +4W (2 Forges)

**Total IV Weight delivered: 16W over the game.** IV is now load-bearing for Iron, exactly as designed. **v0.3 fix confirmed working.**

Compare v0.1: IV required Forge adjacent to Spire. Iron has no Spires (Spires are Flame's identity). IV fires only if Iron built next to Flame — rare and required bad positioning. Under v0.1 Iron's IV delivered probably 0-2W in a typical game. **v0.3 delivers ~16W — an 8× improvement.** Fix is correctly calibrated.

### Iron Vein impact

**Decisive for Iron's engine, but not for Iron's VP.** IV delivered 16 extra Weight but cap-12 ate ~29 mana total. IV essentially fed the cap — Iron's engine became so strong that cap-12 truncated the excess. Without IV, Iron's R3-R5 mana would be ~21 (vs 25-27 actual); but Iron still caps at 12 either way. **IV mattered for MO trigger consistency** (more production → more "R > R-1" rounds → 3 MO triggers vs 1 without IV). But MO was stripped at Unraveling.

**Net IV impact on final score: small** (+6 VP MO banked, all stripped by Edict). IV's real impact is on game *feel*: Iron's engine now has visible acceleration every round, not just one blow-up round.

### Iron too dominant?

**No — arguably overcorrected in the opposite direction.** Iron's engine is the strongest on the board by far (14 gross mana in R5 vs Earth's 12F via FH, Flame's 14E, Tide's 2F). But three factors neutralize the dominance:

1. **Cap-12 eats Iron's surplus.** Iron loses 29+ mana to cap over 5 rounds. A typical 2-multiplier engine (Slate 1) would cap rarely; Slate 7's quadruple engine hits the cap every round from R2 onward.
2. **MO is Iron's only VP doctrine, and Earth's Edict strips it.** With cap-12 limiting mana VP to 6 (from 12 mana), Iron's only path to 20+ VP is MO. Token-strip Edict cuts that path.
3. **Earth's SW + DR combo scores 7 VP outside Iron's reach.** Earth's Deep Roots immunity means Iron cannot retaliate against Earth's doctrines.

**The Iron Snowball slate, post-v0.3, is a structural loss for Iron against a competent Earth.** Iron wins only if Earth misplays Edict (targets something other than MO).

**Is this fair?** Two readings:

- **Positive reading**: Iron "looks" dominant (biggest mana numbers, most production), but strategic counterplay (Edict targeting) produces correct equalization. The game rewards play skill, not just draft.
- **Negative reading**: Iron's VP ceiling is artificially constrained. Iron *should* dominate the Iron Snowball slate because the slate is deliberately stacked in Iron's favor. Instead, Iron loses. This suggests the Edict-strip rule is **too strong** against production-focused Schools that rely on a single VP doctrine (MO).

**My verdict**: the Edict change (v0.3) + cap-12 combined create a "production ceiling" that specifically punishes Iron. Consider:
- Allowing MO to generate 1 VP token every round it fires (rather than 2), so Edict strip costs less
- OR: Making MO scoring immune to Edicts (it's not even a dominant doctrine — token-strip on MO is almost always the best Edict target, which means MO is functionally a "free Edict kill")
- OR: Raising cap to 14 (as PT09 suggested) so Iron's mana-VP reflects engine depth more fairly

### Mana cap 12 impact

**Iron's engine hits cap every round from R2 onward and loses ~29 mana total over the game.** Cap-12 is **not** wide enough to let Iron's Slate 7 engine breathe. Iron is producing ~18-22 mana per round, keeping 12, losing 6-10.

- **If cap is intended containment**: cap-12 is working — it prevents Iron's Slate 7 mana from translating to 14+ VP (unopposed) down to 13 VP (with cap). Earth wins.
- **If cap is intended to reward engine investment**: cap-12 is too tight. Iron gets ~13 VP from mana regardless of whether it built a 2× or 4× multiplier engine. The cap flattens the differentiation.

**Recommendation**: test cap-14 in PT10 with this same Slate 7 configuration. If Iron wins at cap-14, the cap is the right dial for engine reward. If Earth still wins, the issue is the Edict strip, not the cap.

### Balance verdict: 3/5

- **Iron Vein fix (the key test): 5/5 — works as designed.** IV fires every round Iron activates a Forge post-R2. The adjacency pattern matches Iron's natural buildout.
- **Cap-12 fix: 3/5 — works but over-contains.** Iron's quadruple multiplier engine is throttled; mana-VP differentiation is compressed.
- **Deep Roots nerf: 4/5 — works.** Earth's DR scored 2 VP (not 5 — terrain placement matters). Max-5 cap never binding in this slate.
- **Edict VP-token strip: 3/5 — works, possibly too strong.** The strip is a clean rule but it disproportionately punishes Schools with a single VP-doctrine (Iron's MO). Works against the Iron Snowball design intent.
- **Veil 3-Forge once/game: N/A — Iron never activated 3 Forges in one round.** Iron only built 2 Forges. Rule dormant this playtest.

### Remaining issues

1. **Iron's VP ceiling problem.** Iron's only path to >15 VP is MO, and Earth's Edict kills MO. Without MO, Iron is capped at mana-VP (13). Iron needs a second VP-scoring path that can't be Edict-stripped (or can scale beyond 13). Options:
   - Add a new Production VP doctrine that scores end-game (not in-game tokens), so Edict blocks but can't strip retroactively
   - Let Iron's Tempered tokens grant end-game VP (e.g., 1 VP per 2 tokens) — scoring tied to a non-doctrine mechanic is un-Edictable

2. **Tide without Tidal Mill is dead.** TM is so load-bearing for Tide that its absence (Slate 7) leaves Tide with 2 mana total at game end. **This is a slate-design issue, not a v0.3 issue.** Tide's non-TM draft paths need to be tested (e.g., Slate 7 with TX+EL+TL+GW for Great Work score race). Flag for future slate revision.

3. **Temporal Loop is hard to use correctly.** Tide drafted TL and never fired it usefully — Tide never had a "good round" worth repeating. TL is only valuable after a large-production round. Tide-with-TL needs reliable big-production rounds, which requires TM or similar. Without TM, TL is a dead card.

4. **Edict strips the highest-banked VP card.** In Slate 7, this is trivially Iron's MO. Every playtest where Iron is strong and has MO, the optimal Edict play is "strip Iron's MO." This is **deterministic** — takes the strategic choice out of Edict. Consider adding Edict-resistant doctrines or a scaling rule ("Edict removes max 3 VP tokens" — forces targeting decisions).

5. **Cap-12 vs cap-14 question from PT09 is still open.** Under cap-14 hypothetical: Iron R5 end = 14W + 7F = 21 → keep 14. Final Activation +14W +6F → 28W +6F = 34 mana → 17 VP mana. Iron total with MO stripped: 17 VP > Earth 14. **Cap-14 flips Iron to the winner.** This suggests cap-14 is a better reward for engine depth; cap-12 overcontains.

---

**Slate verdict (v0.3)**: The Iron Vein fix is **validated and working** — IV is now a cornerstone Iron doctrine, firing every round. However, combining the fix with cap-12 and Edict-strip produces an Iron that's *structurally* prevented from dominating even in its signature slate. Iron loses Slate 7 under v0.3 despite drafting its theoretical-maximum loadout. This is either:
- A sign of healthy balance (Iron shouldn't auto-win any slate) — or
- Evidence of over-correction (Iron should win a slate called "Iron Snowball")

Recommend running Slate 1 with v0.3 (PT-next) to confirm non-diagnostic balance and PT10 with cap-14 + Slate 7 to test the cap dial.
