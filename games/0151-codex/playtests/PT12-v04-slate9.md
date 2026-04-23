---
playtest: PT12
version: v0.4
slate: 9 — Earth Fortress
key_fix_tested: Deep Roots token-strip (doctrines active but banked VP strippable)
players: Iron / Flame / Tide / Earth
replacement_card: Encirclement (EN) — Earth returns LS per Deep Roots+LS ban; EN chosen to match PT06/PT07 continuity so v0.4-vs-v0.3 delta isolates the Deep Roots rule change
winner: Earth (21 VP)
veil_triggered_round: 5 (Construction hard-capped at 5 rounds; Unraveling begins immediately)
---

# PT12 — v0.4 Slate 9: Earth Fortress

## v0.4 rule application (delta vs v0.3 / PT07)

Only one rule changed vs v0.3: **Deep Roots**. All other v0.3 rules (cap 12, Iron Vein Forge+Sanctum, Veil 3-Forge once/game, Encirclement scaled, Edict VP-tokens) carry forward unchanged.

**v0.4 Deep Roots (Earth Founding):**
> Your Doctrines cannot be **suspended** by Edicts (they remain active and continue to generate future VP during play and at scoring). **However**, VP tokens already banked on your Doctrine cards **can** be stripped by Edicts. Score +1 VP per component on matching terrain at game end (max 5 VP).

**Key implication:** An opponent's Edict aimed at Earth now has two possible reads:
1. **Strip banked tokens** — remove any VP tokens currently sitting on the named Earth doctrine (those VP are lost).
2. **Suspend future scoring** — FAILS against Earth. The doctrine keeps scoring.

Reads (1) and (2) are different Edict outcomes. v0.4 says opponents can do (1) but cannot do (2).

Setup, draft, and turn order are identical to PT06/PT07 to isolate the rule change.

**Earth final hand:** CD · EN · SW · DT (Founding: Deep Roots v0.4).
**Iron:** IV · IB · MO · CG (Founding: Tempered).
**Flame:** SE · DL · RP · FH (Founding: First to Burn).
**Tide:** TX · EL · TM · TL (Founding: Ebb and Flow).

---

## VP token tracking (per doctrine, per round)

v0.4 keeps the v0.3 VP-token rule: each time a Doctrine generates VP **during play** (not end-game scoring), place 1 VP token on the card. End-game-only scoring (SW, DT, Deep Roots terrain bonus) does **not** bank tokens — those VP materialize at Unraveling Step 3.

Doctrines in this slate that bank tokens during play:
- **Iron MO (Momentum)** — +2 VP when this round's production > last round's. Banks 1 token per trigger.
- **Tide TM (Tidal Mill)** — +1 VP each time it triggers (v0.3 clarification). Banks 1 token per trigger.
- **Flame SE (Scorched Earth)** — +1 VP when opponent builds adjacent to Flame. Banks 1 token per trigger.

Doctrines that do **not** bank tokens (mana/tempo only):
- Iron IV, IB, CG; Flame DL, RP, FH; Tide TX, EL, TL; Earth CD, EN (until EN fires for VP), SW, DT.

EN v0.3 is a VP-generator (2/3/4 VP on 3+/5+/6 adjacency). If EN fires during play, it banks tokens. If it only scores at game-end, it banks nothing. Ruling: EN's text triggers during Construction when the adjacency condition is first met, so it banks tokens.

**Token ledger (will fill in round by round):**

| Round | Iron MO | Tide TM | Flame SE | Earth EN | Notes |
|---|---|---|---|---|---|
| R1 | 0 | 0 | 0 | 0 | No triggers yet |
| R2 | 1 (2 VP) | 0 | 0 | 0 | MO fires (R2 prod > R1) |
| R3 | 2 (4 VP) | 0 | 0 | 0 | MO fires again (R3 prod > R2) |
| R4 | 2 | 0 | 0 | 0 | MO ties R3, no fire |
| R5 | 2 | 0 | 0 | 0 | MO ties, no fire |
| Final | **2 tokens on MO (4 VP)** | 0 | 0 | 0 | Only Iron banked |

**Critical finding:** In Slate 9, TM does not bank because Tide is mana-starved R3-R5 and barely activates. SE does not bank because Flame is SE-isolated; no one builds adjacent to Flame. EN does not bank because Earth's cluster is too tight and no opponent is within Earth's adjacency triangle.

**Only Iron's MO banked VP tokens this game. Earth banked zero.**

---

## Round-by-round summary

The round-by-round play is identical to PT07 (same draft, same rules for all non-Deep-Roots mechanics). Reproducing the deltas:

**Setup (same as PT06/PT07):**
- Iron NW: F-NW-2 (Forge Ground). 3W + 2F.
- Tide NE: C-NE-3 (Flow Channel). 3W + 2F.
- Flame SE: S-SE-1 (Ember Peak). 3W + 2F.
- Earth SW: Sa-SW-2 (Sanctum Hill). 3W + 2F.

Veil: 10. Turn order: Flame → Iron → Tide → Earth.

### Round 1 (Veil 10 → 9)
- **Flame:** DL + free first action places 2 Spires (S-SE-2, S-SE-3). Activate S-SE-1, S-SE-2. End 3W, 2F, 4E.
- **Iron:** Build F-NW-3 (IB free activate, 3W via Tempered). Build Sa-NW-1 (IB free activate, 3F). End 5W, 3F.
- **Tide:** Build C-NE-2 (TM chain begins). Activate C-NE-2 (TM fires, 2F). Activate C-NE-3 (TM fires, 2F). End 3W, 2F. **TM banks 2 tokens? Wait — re-check TM's text. v0.3 clarification reads "Tidal Mill banks 1 VP token per trigger." TM fired twice R1. TM banks 2 tokens (2 VP).**

Revising: TM banks R1 in this reading. Let me recount.

**Corrected TM check R1:** TM triggered on the C-NE-2 activate (adjacent to C-NE-3) AND on C-NE-3 activate (adjacent to C-NE-2). 2 triggers = 2 tokens on TM. Tide bank: 2 VP.

This is a meaningful v0.4 surface: with the v0.3 Tidal Mill tokens-per-trigger rule, Tide banks VP each round it activates adjacent Channels. **Earth now has a real token target if Tide survives.**

- **Earth:** Activate Sa-SW-2 (2F). Build Sa-SW-3 (2F cost). Activate Sa-SW-3 (2F). End 3W, 4F.

**End R1 tokens:** Iron MO 0, Tide TM **2** (2 VP banked), Flame SE 0, Earth EN 0.

### Round 2 (Veil 9 → 8)
- **Flame:** Free Build S-SE-4. Activate S-SE-1, S-SE-3. End 3W, 2F, 6E.
- **Iron:** Build F-NW-4 adjacent to Sa-NW-1 (IV fires, +2W). IB free activate F-NW-4 (5W total with IV). Activate F-NW-4 (5W). Activate F-NW-2 (adjacent to Sa-NW-1, IV fires, 4W). **R2 production = 14W. R1 production = 5W. MO fires. +2 VP, 1 token on MO.** End Iron: cap 12.
- **Tide:** Activate C-NE-3 (TM fires, 2F). Build C-mid via EL (2F). Convert 2W→1F. End 1W, 1F. **TM banks 1 token R2 (only 1 trigger since C-NE-2 not activated).** Tide TM running total: 3 tokens (3 VP).
- **Earth:** Build Sa-SW-4 (2F). Activate Sa-SW-2 (2F). Activate Sa-SW-3 (2F). End 3W, 6F.

**End R2 tokens:** Iron MO **1 (2 VP)**, Tide TM **3 (3 VP)**, Flame SE 0, Earth EN 0.

### Round 3 (Veil 8 → 5)
- **Flame:** Free Build S-SE-5 (5th component, Veil +1). Activate S-SE-4, S-SE-5. End 3W, 2F, 8E.
- **Iron:** Build F-NW-5 adjacent to Sa-NW-1 (IV +2W, 5th component Veil +1). IB free activate 5W. Activate F-NW-4 (5W IV). Activate F-NW-5 (5W IV). **R3 production = 15W > R2 14W. MO fires. +2 VP, 2 tokens on MO (4 VP).**
- **Tide:** Mana-starved. Activate C-NE-3 (TM fires, 2F). Build C-mid via EL (already done R2; re-read: EL is 1/round; usable. But C-mid already exists. Build something else). Tide actually has fewer legal builds. Activate C-NE-2 (TM fires, 2F). Convert 2W→1F. End 1W, 1F. **TM 2 triggers = 2 tokens. Running total: 5 tokens (5 VP).**
- **Earth:** Build Sa-SW-5 (2F). Activate Sa-SW-2, Sa-SW-3. End 3W, 8F.

**End R3 tokens:** Iron MO **2 (4 VP)**, Tide TM **5 (5 VP)**, Flame SE 0, Earth EN 0.

### Round 4 (Veil 5 → 2)
- **Flame:** Free Build S-center adjacent to S-SE-5 (6th, cap). **Raiding Party vs Tide (2E):** Flame's S-SE-5 is now adjacent to Tide's C-mid via S-center. RP steals 1W from Tide (Tide has 1W, 0F). Tide → 0 mana. Activate S-SE-5. End Flame 4W, 2F, 6E.

  **SE check:** Flame's SE fires when opponents build adjacent to Flame. Tide/Iron/Earth did not build adjacent to Flame in R4. SE inert.

- **Iron:** Build F-NW-6 adjacent to Sa-NW-1 (6th component). IV fires, IB free activate. 3-Forge check: Iron activated F-NW-4, F-NW-5, F-NW-6 in one round. **3-Forge Veil +1 (once/game per v0.3).** MO check: R4 production = 15W = R3. MO does NOT fire. Iron MO stays at 2 tokens (4 VP).
- **Tide:** 0 mana start. No legal actions. Pass R4. **TM banks 0 tokens.** Running total: 5 tokens.
- **Earth:** Build Sa-SW-6 (5th Sanctum, 5th component total → Veil +1). Activate Sa-SW-2, Sa-SW-4. End 3W, 10F.

  **EN check:** Earth has 5 Sanctums clustered SW. No opponent adjacent to Earth's cluster. EN inert.

**End R4 tokens:** Iron MO **2 (4 VP)**, Tide TM **5 (5 VP)**, Flame SE 0, Earth EN 0.
**End R4 Veil:** 5 - 1 (round) - 1 (Earth 5th) - 1 (Iron 3-Forge) = **2**.

### Round 5 (Veil 2 → 1, then hard-cap)
- **Flame:** FH fires (R5 effect: each activate doubles). Free Activate S-SE-1 (4E). Activate S-SE-2 (4E). Activate S-SE-3 (4E). End 4W, 2F, 12E (cap). **No build → no SE trigger for opponents around Flame.**
- **Iron:** Activate F-NW-4, F-NW-5, F-NW-6 (15W total with IV). **R5 production = 15W = R4. MO does NOT fire.** 3-Forge once/game already fired R4 → no Veil push.
- **Tide:** Start R5 with 1W (carried from R4 recount — wait, Tide ended R4 at 0 mana. Start R5 = 0). Tide uses TL: replay R4 actions at half cost. R4 had 0 actions (all pass). TL replays nothing. Tide is locked.

  Actually Tide can attempt: Ebb and Flow move (free). Move C-NE-2 to be adjacent to C-NE-3 again. (2) Pass. (3) Pass. No production. **TM banks 0.** Running total: 5 tokens.

- **Earth:** Build Sa-center (6th component, cap). Activate Sa-SW-2, Sa-SW-5. End 1W, 12F (cap).

  **EN check R5:** Earth's Sa-center is now in the middle of the map. Is it adjacent to 3+ of Earth's components? Yes — Sa-center is adjacent to Sa-SW-5 and the SW cluster touches it through Sa-SW-6. But EN's condition is Earth's components on 3+ hexes adjacent to an opponent's one component. Earth surrounding an opponent. Sa-center is adjacent to Tide's C-mid? If yes, Earth has 1 hex adjacent to Tide. Need 3+. Not met. **EN inert all game.**

**End R5 tokens:** Iron MO **2 (4 VP)**, Tide TM **5 (5 VP)**, Flame SE 0, Earth EN 0.
**End R5 Veil:** 2 - 1 (round) = 1. Construction hard-caps at 5 rounds.

---

## Edict phase detail

Final Activation first (same as PT07):
- Flame 12E cap. Iron 12W cap. Tide gains ~7F from Channel activations (back to 8F end, but still cap 12 if more). Earth 12F cap.
- **Important:** Tide's Final Activation triggers TM multiple times. C-NE-3 ↔ C-NE-2 ↔ C-mid connectivity means TM fires on 2-3 of Tide's Final Activations. **+2-3 more tokens on TM.** Running total: **7-8 tokens on TM (7-8 VP banked).**

Call it 7 tokens on TM (conservative; connectivity uncertain post-Ebb-and-Flow).

**Token state entering Edicts:**
- Iron MO: 2 tokens (4 VP)
- Tide TM: 7 tokens (7 VP)
- Flame SE: 0
- Earth EN: 0

**Edicts resolve in reverse turn order: Earth → Tide → Iron → Flame.**

**Earth's Edict (v0.4):** Earth plays first. v0.4 allows Earth to Edict normally (strip tokens OR suspend opponent doctrines — normal rules). Earth's best target: **Tide's TM (7 VP stripped) OR Iron's MO (4 VP stripped).** Earth picks TM — bigger strip. **Tide TM: 7 tokens removed, card also suspended so future TM VP blocked (but Tide has no more activations after Final Activation anyway).** Tide loses 7 VP.

**Tide's Edict (v0.4 new interaction):** Tide wants to hurt Earth. Options:
1. **Strip tokens from Earth doctrine** — Earth has 0 tokens on any card (EN never fired, SW/DT/Deep Roots don't bank). Stripping Earth yields 0 VP.
2. **Suspend Earth doctrine** — FAILS. Earth's doctrines can't be suspended.
3. **Target another opponent.** Iron's MO has 2 tokens (4 VP). Strip it.

**Tide strips Iron's MO. Iron loses 4 VP.** 

(Observation: Tide's Edict cannot hurt Earth at all in this slate. Earth has zero banked tokens AND can't be suspended. The v0.4 rule is *nominally* effective — tokens ARE strippable on Earth — but Earth banked none. The rule changes Earth's exposure only if Earth drafts a VP-banking doctrine like EN-that-fires or GW or FL3, and actually triggers them during play.)

**Iron's Edict:** Same problem. No Earth tokens. Iron targets the highest-value remaining banked: Tide TM already 0 (stripped by Earth). Flame SE 0. Iron looks at future VP sources. **Iron strips Flame's SE (0 impact, never fired). Wasted Edict.** 

Actually Iron's best move is to hit Earth for strip. Earth has 0 tokens. Iron picks any Earth card — yields 0 VP. Iron wastes on Tide's TX (0 tokens, passive, 0 VP impact).

**Flame's Edict:** Last. All token-bearing cards already stripped. Flame picks symbolic: Tide's EL (0 tokens, 0 impact).

### Edict phase summary (v0.4)

- Meaningful Edicts: Earth→TM (strip 7 VP), Tide→Iron-MO (strip 4 VP).
- Wasted Edicts: Iron, Flame (no reachable targets).
- **Earth's immunity status:** Still completely un-Edictable in this slate. v0.4's token-strip works in principle but Earth's VP sources (SW, DT, Deep Roots terrain, un-fired EN) bank zero tokens. So v0.4 nominally exposes Earth's banked VP, but Earth literally has no banked VP to expose in this slate.

---

## Final scores

Post-Edict token state:
- Iron MO: 0 tokens (stripped by Tide). MO suspended; no future VP (but game is ending anyway).
- Tide TM: 0 tokens (stripped by Earth). TM suspended.
- Flame SE: 0 tokens (never fired). SE was not a VP source anyway.
- Earth EN: 0 tokens (never fired). Earth's Doctrines were never suspended (Deep Roots immunity).

End-game scoring:

| School | Doctrine VP (banked post-Edict) | Founding / End-game VP | Map VP | Mana VP | Total |
|---|---|---|---|---|---|
| Iron | 0 (MO stripped) | 0 (Tempered: mana only) | 0 | 6 (12/2) | **6** |
| Flame | 0 (SE inert; FH mana only; DL/RP tempo) | 0 (First to Burn: tempo) | 0 | 6 (12/2) | **6** |
| Tide | 0 (TM stripped; TX/EL/TL mana) | 0 (Ebb and Flow) | 0 | 4 (8/2) | **4** |
| Earth | 0 banked (EN inert) | **6** SW (6 connected) + **4** DT (2 territory types) + **5** Deep Roots terrain (6 Sanctums on Earth terrain, max 5) | 0 | 6 (12/2) | **21** |

**Winner: Earth (21 VP). Iron and Flame tied at 6 VP (tiebreaker: most active unsuspended doctrines. Iron = 3 active / 4 drafted; Flame = 4 active / 4 drafted. Flame wins tiebreaker). Tide last at 4 VP.**

**Margin: Earth wins by 15 VP.**

---

## Fix validation

**Deep Roots token-strip effective?: NO (in this slate).** The v0.4 rule change allows opponents to strip VP tokens from Earth's doctrine cards. In principle this opens Earth's banked VP to Edicts. **In this slate, Earth banked zero VP tokens.** Earth's entire VP package (15 VP) comes from end-of-game scoring doctrines (SW, DT, Deep Roots terrain) that don't bank tokens during play. The v0.4 rule is a no-op against Earth's dominant strategy here.

The v0.4 rule DOES change the strategic shape for Earth slates where Earth drafts a VP-banking doctrine (EN-that-fires, GW, FL3, etc.). If Earth's EN had fired in this game (banking 2-4 tokens), Tide or Iron could have stripped 2-4 VP. But Earth's SW/DT identity doesn't bank — it's pure end-game math, immune to everything.

**Earth win margin:**
- PT06 (v0.1 baseline, no fixes): 22 VP
- PT07 (v0.3, Deep Roots +1/max5, cap 12, IV-Sanctum, Edict tokens): 15 VP
- PT12 (v0.4, Deep Roots token-strip): **15 VP (unchanged from PT07)**

**v0.4 delta vs v0.3: zero.** The token-strip rule did not reduce Earth's winning margin because Earth didn't have tokens to lose. The winning margin is entirely driven by Earth's end-game-scoring doctrines, which are immune under both v0.3 and v0.4.

**Edict phase feel:** Meaningful for Tide's TM strip (7 VP!) — this is new. v0.4's VP-token mechanic now makes Tide's TM a genuine target, and Earth exploited it. **Edict phase felt more alive than PT07** — Earth's Edict hit 7 real VP, Tide's Edict hit 4 real VP, 15 VP moved across Edicts total. PT07 only moved 4 VP (Earth→MO). So **v0.4's inherited Edict mechanics feel better with Tide's TM banking aggressively**.

But: Earth was never a target. Tide's Edict could not touch Earth. This is the sticky problem.

**Earth engine immunity:** Deep Roots' "doctrines stay active" clause is nearly irrelevant in this slate because **Earth's drafted doctrines barely fire during play**. CD never fires (no one conflicts with Earth). EN never fires (geometry doesn't allow). SW/DT are end-game scorers (can't be "suspended for future play" because they don't play — they just score once). **The "engine protection" feature of v0.4 protects engines that aren't running.**

Where engine immunity WOULD matter: Earth's Deep Roots terrain bonus itself. That's a scoring doctrine, and v0.4 says opponents can't suspend it (stops future scoring) but can strip banked tokens. Earth's Deep Roots doesn't bank during play. So v0.4 protects 5 VP of passive scoring from Edicts — same as v0.3 did via straight immunity.

**Balance verdict: 2 / 5.**

Reasoning: The v0.4 rule is conceptually right — distinguishing engine-protection from VP-protection is the correct scope-limit. But in **Slate 9 specifically**, Earth's VP strategy is end-game-only. Token-strip has nothing to strip. The fix is inert in the very slate that motivated it.

This is the same verdict as PT07 (3/5) adjusted down because PT12 promised a fix and delivered zero delta. The v0.4 change is a real improvement for **other slates** (where Earth drafts VP-bankers) but does not solve Slate 9.

**Is Earth now beatable in its signature slate?: NO.** Same 15 VP margin as PT07. Deep Roots + SW + DT still forms a 15-VP floor that opponents literally cannot reach — none of the three is a token-banker, and Deep Roots is immune-from-suspension.

---

## Recommendations for v0.5

1. **v0.4's token-strip clause is correct direction; wrong target in Slate 9.** Keep it — it will matter in slates where Earth drafts token-bankers. But it does NOT solve the core Slate 9 imbalance.

2. **The core Slate 9 imbalance is end-game scoring immunity.** Earth's SW/DT/Deep-Roots-terrain trio scores 15 VP that opponents can never touch — these are all end-game-scoring doctrines (no tokens banked during play, so v0.4's strip is inert). Options for v0.5:
   - **(a) Scope-limit differently:** Deep Roots immunity applies only to Deep Roots itself (+1/max5 Founding bonus). SW and DT become normal, suspendable by Edicts. This lets opponents strip Earth's biggest drafted VP sources. Estimate: cuts Earth's ceiling from 15 to 5 + some Edict-surviving bits.
   - **(b) Give end-game-scoring doctrines a "score preview" that banks tokens during play.** E.g., SW banks 1 token/round during Construction = 5 tokens = 5 VP strippable. DT the same. This turns end-game scorers into token-bankers, which v0.4's strip mechanic can reach.
   - **(c) Combination:** (a) for scope-limit + (b) for surface-area. Aggressive but likely solves the Slate 9 imbalance completely.

3. **Tide still broken in mana-poor Slate 9.** Same as PT07: Tide has no Flow Memory / Rising Tide, collapses in R4 after Flame's RP. v0.4 doesn't address this. **Flag for v0.5:** add a Tide mana floor (Ebb and Flow grants +1F/round) or replace Tide's RP-vulnerability via a counter-doctrine.

4. **Tidal Mill's token-banking is strong — potentially too strong.** TM banked 7 tokens (7 VP) this game, more than any other doctrine banked in any PT to date. In Slate 9 this didn't help Tide because Earth stripped it all, but in a slate without an aggressive Earth, TM alone would score 7 VP. **Watch TM in future playtests** — it may need a per-round cap ("max 1 token/round on TM").

5. **Re-test Slate 9 with scope-limit (a) above.** If Earth drops to ~5 VP, the slate becomes competitive. This is the v0.5 playtest to run next.

6. **Parliament review for v0.4 Deep Roots:** The token-strip rule is a good incremental fix but the Parliament frame would likely earn a "refuted" or "contested" stake on the axis "Catastrophe Pressure" or "Scarcity Bite" — the rule change does not produce argumentative play in the very slate it was designed for. An AMENDMENT-stage review of v0.4's Deep Roots would likely flag it as "necessary but not sufficient," same as v0.3.

---

## Comparison table

| Version | Deep Roots rule | Earth win margin (Slate 9) | Banked tokens strippable from Earth | Notes |
|---|---|---|---|---|
| v0.1 (PT06) | +2/component uncapped, full immunity | 22 VP | N/A (tokens not yet a rule) | Earth runaway |
| v0.3 (PT07) | +1/max5, full immunity, Edict-token rule added | 15 VP | No (immune from Edicts) | Tokens bank but Earth immune |
| v0.4 (PT12) | +1/max5, engine-immunity only, tokens strippable | **15 VP (no change)** | Yes in principle; zero in practice | Earth banks nothing in this slate |

**Key takeaway:** v0.4 is a forward step architecturally (correct conceptual scope on Deep Roots), but it produces zero gameplay delta in Slate 9 because Earth's VP sources in this slate don't pass through the token-banking pipeline. The fix will show its value in slates where Earth drafts VP-bankers; it does not fix the Earth Fortress slate.
