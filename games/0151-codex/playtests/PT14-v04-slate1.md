---
playtest: PT14
version: v0.4
slate: 1 — The Balanced Opener (3rd iteration)
players: Iron / Flame / Tide / Earth
v01_result: Earth 17, Flame 12, Iron 10, Tide 8
v03_result: Earth 15, Iron 14, Flame 8, Tide 7
winner: Earth (15 VP) — canonical with optimal Edict routing; Tide alt scenario reaches 22 VP
veil_triggered_round: 5
---

# PT14 — v0.4 Slate 1: The Balanced Opener (Tide & Flame fun test)

**Purpose:** retest Slate 1 under CODEX v0.4. Two v0.4 rules changes carry this
test:

1. **Tidal Mill banks 1 VP token EACH TIME it triggers** (previously: passive
   mana production only).
2. **Scorched Earth banks 1 VP token EACH TIME it triggers** (previously:
   +1 E when opponent builds adjacent, no VP).

All other v0.3 fixes (Deep Roots max 5, Iron Vein Forge+Sanctum, cap 12,
Encirclement scaled, Edict strips VP tokens, Veil 3-Forge once/game) carry
forward unchanged.

**Slate 1 (unchanged from PT02/PT11):**
- Iron: Tempered + IV · SC · PI · SW
- Flame: First to Burn + DL · SE · MO · CG
- Tide: Ebb and Flow + TX · EL · TL · TM
- Earth: Deep Roots + CD · DT · FH · SM

**Simulation notes:** identical board/corner assumptions as PT11. Turn order
Flame→Iron→Tide→Earth. Simultaneous activation from R3 onward. Same component
set: Iron=Forge (NW), Tide=Channel (NE), Flame=Spire (SE), Earth=Sanctum (SW).

---

## Token tracking (the v0.4 story)

### Tidal Mill triggers (each = 1 VP token banked on TM)

Tide's build path tracks PT11: starting Channel → R1 build 2nd Channel (adj)
→ R2 build 3rd Channel (adj to two) → R3 Expansion Law free-adjacency build of
4th Channel → no further builds R4/R5, + Temporal Loop replay in R5 + Final
Activation.

TM fires any time Tide **activates** a Channel with at least one adjacent
Channel.

| Round | Activations triggering TM | Triggers |
|---|---|---|
| R1 | Activate new Channel (adj to start), Activate start Channel (adj to new) | **2** |
| R2 | Activate middle Channel (adj to 2), Activate one more Channel | **2** |
| R3 | Activate middle Channel, Activate EL-built Channel (adj to existing) | **2** |
| R4 | Two Channel activations (all adjacent) | **2** |
| R5 main | Activate middle Channel, Activate another Channel | **2** |
| R5 TL replay | Temporal Loop repeats "Activate/Activate/Convert" — 2 Channel activations at half cost | **2** |
| Final Activation | 3 Channels, all chained via adjacency | **3** |

**TM total triggers: 15 → 15 VP tokens banked on Tidal Mill.**

v0.4 makes this Tide's primary VP engine: every chain activation now banks.
Tide is no longer a doctrine that produces free mana silently — it's the
engine.

### Scorched Earth triggers (each = 1 VP token banked on SE)

SE fires when **an opponent builds a component adjacent to a Flame component.**

Mapping the slate's build patterns against Flame's SE-corner position (SE
board corner, Spires on Ember Peaks and adjacent Forge Ground):

| Round | Opponent builds adjacent to Flame? | Triggers |
|---|---|---|
| R1 | Iron: Sanctum on Sanctum Hills (NW, far). Tide: Channel on Flow (NE, far). Earth: Channel on Flow (SW, far). | **0** |
| R2 | Iron: Forge on Forge Ground (NW cluster). Tide: Channel adj its own. Earth: Sanctum on Sanctum Hills + Channel on Earth territory. | **0** |
| R3 | Iron: Sanctum (NW). Tide: EL-placed Channel toward **map center** — does it land adjacent to Flame's Ember Peaks cluster? If Tide places EL-build near Flame's corridor to disrupt: **1 trigger.** Otherwise 0. Assume Tide places EL-Channel adjacent to Flame to contest: **1.** Earth: Channel on Earth territory (SW, far). | **1** |
| R4 | Iron: Forge on Forge Ground near SE edge (Iron's 2nd Forge placement to prime IV on both Forges per PT11). This Forge is on Forge Ground which in the NW cluster, not adjacent to Flame. **0.** Earth: Sanctum on Sanctum Hills (far). | **0** |
| R5 | No new builds. | **0** |

**SE total triggers: 1 → 1 VP token banked on Scorched Earth.**

This is below the prompt's predicted 3-6 tokens. **Reason:** in Slate 1 the
four Schools sit in their own corners. Iron and Earth barely approach Flame's
SE corner. Only Tide's EL (Expansion Law) free-adjacency build plausibly lands
near Flame — and only if Tide uses it aggressively to harass. Flame's Dawn
Law early-game footprint also locks in Flame's Ember Peaks zone before
opponents have mana to contest it.

**SE is still structurally passive in this slate.** v0.4's token-banking fix
helps, but only marginally: Flame's SE corner is geographically isolated.
*The v0.4 fix would land harder in slates where Flame sits in a contested
center or where another School drafts expansion tools like Expansion Law AND
directs them toward Flame.*

### Momentum triggers (unchanged from v0.3 — still strict-greater-than)

Per PT11: MO fires exactly once — R3 (6E > 4E). R4 is 6E = 6E tie (no fire).
R5 ties. **MO total: 1 trigger = 2 VP banked.**

---

## Round-by-round summary (Tide + Flame focus)

### Round 1

**Flame:** First to Burn free + Dawn Law places 2 Spires. Activates for 4E
produced. Mana end: 3W+2F+2E. **SE: 0. MO: no prior round.**

**Tide:** Builds 2nd Channel on Flow Channel adj start. Activates new Channel
— TM fires (1 VP token to TM). Activates start Channel — TM fires again
(2nd token). Mana +3F spent 2F; end 3W+3F. **TM VP tokens: 2.**

### Round 2

**Flame:** Activate Spire (free) +2E. Builds 4th Spire (2E). Activates middle
Spire (2E). No opponent adjacent build → **SE: 0.** MO: 4E vs 2E last → +2
VP banked on MO (strict greater). Wait — R1 produced 4E not 2E? Let me
recheck R1: Dawn Law built 2 Spires, activated ONE for 2E, no further
activation. R1 produced 2E. R2 produces 2(free activate)+2(middle activate)
= 4E. **MO: 4>2 → +2 VP on MO.**

Correction to PT11's count: PT11 said R1 = 4E because Flame had two
activations, but Dawn Law's *build* of 2 Spires doesn't produce. Only
activations produce. If Flame only activated once in R1 (as PT11 trace
implies — R1 actions were Dawn Law free + Activate + Activate), R1 produced
4E (two activations). Then R2's 4E ties, no MO fire.

Re-reading PT11 R1: *"Action 2: Activate Ember Peaks Spire → 2E. Action 3:
Activate Forge-Ground Spire → 2E."* — two activations, **4E produced R1**.

**R2 MO: 4E vs 4E → tie, no VP.** Reverting to PT11's analysis.

**Tide:** Builds 3rd Channel adjacent to 2 existing (Flow Channel middle of
cluster). Activates — TM fires (adj to 2, but TM banks per trigger not per
adjacency count = 1 token). Activates middle Channel — TM fires again (1
token). **TM VP tokens tally: 2+2 = 4.**

### Round 3 (simultaneous activation active)

**Flame:** Builds 5th Spire contesting Iron's corridor (**+1 Veil 8→7**).
Activates contested Spire → CG fires +2E → 4E. Activates non-contested Spire
→ 2E. R3 total: 6E. **MO: 6>4 → +2 VP banked on MO.**

**Tide:** Activates middle Channel — TM (1 token). Uses Expansion Law to
build 4th Channel at **contested center adjacent to Flame's Ember Peaks
zone.** Activates new Channel — TM (1 token). **TM VP tokens tally: 4+2 = 6.**

*Tide's EL placement adjacent to Flame's Spires = SE fires once.* **SE VP
tokens: 1.**

### Round 4

**Flame:** Activate contested Spire (free) → 4E. Activate Spire → 2E. Build
6th Spire on Ember Peaks (2E cost). R4 produced 6E = R3 6E → MO tie, no
fire.

**Tide:** Activates Channel — TM (1 token). Activates another Channel — TM
(1 token). Converts 2F→2W. **TM VP tokens tally: 6+2 = 8.**

### Round 5

**Flame:** Activate contested → 4E. Activate Spire → 2E. Convert 4E→2W.
MO: 6E tie, no fire.

**Tide main actions:** Activate middle Channel — TM (1 token). Activate
Channel — TM (1 token). **Temporal Loop replay:** repeats "Activate/
Activate/Convert" at half cost. Two more Channel activations → TM fires
twice more (2 tokens). **TM VP tokens tally: 8+2+2 = 12.**

### Unraveling — Final Activation

**Tide:** 3 Channels, each activation fires TM (all adjacent). 3 TM
triggers. **TM VP tokens final tally: 12+3 = 15.**

**Flame:** 6 Spires. No new opponent builds → SE final tally stays at 1.

---

## Final scores

### Pre-Edict VP token state

- **Flame MO:** 2 VP tokens banked (R3 only fire)
- **Flame SE:** 1 VP token banked (R3 only fire — Tide's EL-Channel)
- **Tide TM:** 15 VP tokens banked (every Channel activation chained)
- **Iron SW:** 0 tokens (pure endgame scorer — 5 VP pending)
- **Earth DT/DR:** 0 tokens (both pure endgame scorers — ~8 VP pending)

### Edict phase (Earth → Tide → Iron → Flame)

**Earth's Edict** (first): Earth sees Tide's 15 banked TM tokens — massive
pile. **Strips Tide's Tidal Mill: 15 VP tokens gone.** This is the new v0.4
Edict game: huge piles of token VP are Edict magnets.

**Tide's Edict:** Earth immune. Iron's SW is endgame (0 tokens banked). Flame's
MO has 2 banked, SE has 1 banked. Tide targets **Iron's Spider's Web** to
block its 5 VP endgame score. **Iron SW: 0 VP.**

**Iron's Edict:** Earth immune, Tide already stripped. Iron targets **Flame's
Momentum** (2 banked) OR **Flame's Scorched Earth** (1 banked). Both small.
Iron picks MO (bigger pile). **Flame MO: 2 VP stripped.**

**Flame's Edict:** Earth immune. Tide's TM already stripped. Iron's SW already
stripped. Flame targets **Earth's Deep Territory — but Earth is immune
(Deep Roots Founding).** Flame's Edict is WASTED. Flame names a cosmetic
card (Iron's Iron Vein, passive, no tokens).

**Post-Edict token state:**
- Tide TM: **stripped → 0 VP**
- Flame MO: **stripped → 0 VP**
- Flame SE: **1 VP (survived — Iron didn't pick SE because MO was bigger)**
- Iron SW: **suspended → 0 VP**
- Earth DT: **protected (Deep Roots immunity) → 6 VP**
- Earth Deep Roots: **protected → 2 VP**

### Mana VP

(Use PT11's numbers since v0.4 doesn't change mana production or cap rules.)

- **Flame:** 17 mana → 8 VP
- **Iron:** 29 mana → 14 VP (same cap-12 boost as PT11)
- **Tide:** 15 mana → 7 VP
- **Earth:** 15 mana → 7 VP

### Scoring table

| School | Doctrine VP | Token VP (banked) | Tokens stripped | Map VP | Mana VP | Total |
|---|---|---|---|---|---|---|
| Iron | 0 (SW suspended) | 0 | 0 | 0 | 14 | **14** |
| Flame | 0 | 1 (SE survived) | 2 (MO stripped) | 0 | 8 | **9** |
| Tide | 0 | 0 (all TM stripped) | 15 (TM stripped) | 0 | 7 | **7** |
| Earth | 8 (DT 6 + DR 2, immune) | 0 | 0 | 0 | 7 | **15** |

Wait — this still shows Earth winning. Let me re-examine the Edict logic.

**Re-examination:** Earth's Edict is the pivot. If Earth correctly reads the
board, Earth strips the biggest banked VP pile, which is Tide's 15 TM tokens.
That leaves Tide with only mana VP. Tide finishes at 7.

But this feels wrong for a v0.4 "Tide fun test." The design goal was for
Tide to *feel* like it was scoring, not to be a VP piñata for Earth's Edict.
Let me try the alternate narrative: what if Tide plays *around* the Edict by
keeping TM tokens lower and banking into other scoring?

**Alternate scenario — Tide hedges:** Tide can't easily avoid TM triggers
(they fire automatically on Channel activation adjacency per rules). Tide's
only mitigation is to **build non-Channel components** to reduce
chain-activation count. But Tide's whole draft (TX/EL/TL/TM) is Channel-
centric. Tide cannot hedge without wasting its draft.

**Verdict: Tide becomes a giant Edict target in v0.4.** The VP-banking rule
makes Tide's engine *visible* as a VP pile, and Edicts strip visible piles
preferentially.

### Revised honest scoring

| School | v0.1 | v0.3 | v0.4 | Trend |
|---|---|---|---|---|
| Iron | 10 | 14 | 14 | flat |
| Flame | 12 | 8 | 9 | +1 (SE survived) |
| Tide | 8 | 7 | **7** | flat (gained 15, lost 15) |
| Earth | 17 | 15 | 15 | flat |

**Earth wins 15 VP. Tide is last at 7 VP.** v0.4 did not change the finishing
order.

**But**: this is the outcome if *optimal Edict play* happens. What's the
experiential/fun change? Tide's play pattern DID change — Tide now watches
tokens pile up through the game (15 VP visible on the card!), and then
watches them all stripped by an Edict. That's arguably more *emotionally
engaging* than v0.3's "Tide quietly produces some mana" — but it's a
rollercoaster of excitement then loss, not a satisfying scoring arc.

**Alternative Edict routing:** What if Earth doesn't Edict Tide? Earth's
rational play is: maximize own score, protect own engine, strip biggest
threat. Tide at 15 banked + 7 mana = 22 potential vs Earth's 15 is a clear
threat. Earth MUST strip Tide.

*Unless Earth chooses to strip Iron's SW (5 endgame VP) to prevent Iron
overtaking.* But 5 < 15, so Earth's rational Edict goes at Tide.

**Net effect of v0.4 for Tide:** TM's mana production (unchanged) was already
baked into Tide's mana pool. v0.4 adds *visible VP pile* that gets stripped.
Tide's final VP is the SAME as v0.3. The fun delta is negative — Tide feels
robbed rather than rewarded.

---

## Final scores (honest)

**Tide scored 15 banked VP tokens via 15 TM triggers. Earth's Edict stripped
all 15. Tide's final: 7 VP (mana only).**

Wait — I need to reconsider. The scoring prompt says "winner: Tide (26 VP)"
in the template. That would only happen if Earth's Edict goes elsewhere.
Let me examine: does v0.4 make Tide's TM pile ACTUALLY too valuable to
Edict-strip, forcing Earth to make a different choice?

**Scenario: Earth plays suboptimally OR weighs doctrine-diversity as
tiebreaker.** In that case Earth strips Iron's Spider's Web (Iron as runner-
up threat). Tide keeps 15 TM banked → Tide final 7 mana + 15 TM = **22 VP**
(close to 26 template).

**Realistic outcome range for Tide:**
- If Earth Edicts Tide: Tide = 7 VP (last place, robbed)
- If Earth Edicts Iron: Tide = 22 VP (first place, rewarded for engine)

Both scenarios are possible; the v0.4 design question is whether Tide
"feels fun" regardless. Answer: **conditionally fun.** Tide's arc is high
variance — either crowned by the engine or crushed by the Edict.

### Canonical PT14 result (Earth plays rationally)

| School | Doctrine VP | Token VP | Tokens stripped | Map VP | Mana VP | Total |
|---|---|---|---|---|---|---|
| Iron | 0 (SW suspended) | 0 | 0 | 0 | 14 | **14** |
| Flame | 0 | 1 (SE) | 2 (MO) | 0 | 8 | **9** |
| Tide | 0 | 0 | 15 (TM) | 0 | 7 | **7** |
| Earth | 8 | 0 | 0 | 0 | 7 | **15** |

**Earth wins 15 VP. Margin over Iron: 1 VP. Margin over Tide: 8 VP.**

But this slate's intent was Tide-fun validation. Let me document the
alternate scenario for completeness:

### Alternate PT14 result (Earth prioritizes Iron over Tide)

| School | Doctrine VP | Token VP | Tokens stripped | Map VP | Mana VP | Total |
|---|---|---|---|---|---|---|
| Iron | 0 (SW suspended by Earth) | 0 | 0 | 0 | 14 | **14** |
| Flame | 0 | 1 (SE survived) | 2 (MO stripped by Iron) | 0 | 8 | **9** |
| Tide | 0 | 15 (TM survived) | 0 | 0 | 7 | **22** |
| Earth | 8 | 0 | 0 | 0 | 7 | **15** |

**In the alternate: Tide wins 22 VP. Earth 15. Iron 14. Flame 9.**

---

## Version comparison

| School | v0.1 | v0.3 | v0.4 canonical | v0.4 alt (Earth Edicts Iron) | Trend |
|---|---|---|---|---|---|
| Iron | 10 | 14 | 14 | 14 | stable |
| Flame | 12 | 8 | 9 | 9 | +1 over v0.3 |
| Tide | 8 | 7 | 7 | **22** | bimodal |
| Earth | 17 | 15 | 15 | 15 | stable |

**The headline finding: v0.4's TM token rule makes Tide a high-variance
School.** Either Tide's engine is crowned (22 VP, winner) or stripped (7 VP,
last). There is no middle outcome for Tide in Slate 1 — Tide either has 15
banked tokens or 0.

---

## Fun assessment

- **Tide fun score: 3/5** — MAJOR improvement in activation-to-activation
  engagement (each trigger banks a visible token, so Tide has a rising
  counter through the game) but the terminal Edict risk creates a
  "rollercoaster crash" pattern. Tide is now emotionally loaded, not
  boring. Whether the Edict lands on Tide determines whether the emotional
  payoff is positive or negative. v0.3 Tide score was ~2/5 (passive), v0.1
  was ~1/5 (invisible). **+1 to +2 improvement but not a clean win.**

- **Flame fun score: 2/5** — SE banked only 1 VP token because Slate 1's
  geography isolates Flame. The fun fix didn't engage here. MO still fires
  once and plateaus. Flame's mid-game remains passive. The v0.4 rule is
  correct in direction but Slate 1 doesn't stress it. **+0 to v0.3 (still
  passive); would likely score higher in slates where Tide/Iron push into
  Flame's zone via Expansion Law or similar.**

- **Iron fun score: 3.5/5** — IV chains through the midgame (engaging), PI
  fires quietly, SW gets Edicted away (painful but expected). Iron's fun is
  dominated by cap-12 mana VP being the terminal payoff, not doctrines.
  Same as v0.3.

- **Earth fun score: 3/5** — Deep Roots immunity makes endgame VP feel
  solid but passive. The Edict phase is where Earth is ACTIVE (choosing who
  to strip); v0.4's big TM pile makes Earth's Edict decision meatier — that
  IS fun for Earth. Earth's fun improves by +0.5 to +1 over v0.3 because
  Edicts now have obvious high-value targets.

- **Most exciting moment:** Tide watching the 8th, 12th, 15th TM token
  accumulate on Tidal Mill through Round 5 and the Temporal Loop replay —
  *"the engine is scoring!"* — immediately followed by Earth's Edict
  stripping all 15. The collapse-of-a-pile moment is genuinely dramatic.

- **Most frustrating moment:** Flame building 6th Spire in R4 with no MO
  trigger (R3 and R4 both produce 6E, tie kills MO) AND no SE triggers to
  compensate. Flame's mid-game sits at the same VP count (6 from earlier
  v0.1, 2 from v0.3 R3 MO, 3 in v0.4) while watching every other School
  accumulate tokens visibly. Flame remains the "worst seat" in Slate 1.

---

## Balance verdict: 3/5

v0.4 solved the "Tide is invisible" problem but replaced it with "Tide is
high-variance" — both of Tide's outcomes (22 or 7) are EXTREME. A balanced
4-player game shouldn't have its lowest-floor and highest-ceiling School be
the same player.

**Root cause:** TM token banking is un-throttled. TM triggers 15 times in
Slate 1 because Tide's whole draft pushes Channel chain activation. The
fix should scale: **TM banks 1 VP token per round instead of per trigger**
(caps at ~5 tokens), OR **TM banks 1 token, but requires spending 1 F to
activate** (mana-gates the VP).

SE is the opposite problem — fires too rarely in corner-sitting slates.
SE fix should probably be: **SE banks 1 VP token whenever Flame activates a
component adjacent to opponent OR opponent builds adjacent.** This doubles
the trigger surface.

Earth's Edict strategy is now over-determined: always strip the biggest
banked pile. This removes Edict-choice texture.

**Score spread (v0.4 canonical):** 15 − 7 = 8. Same as v0.3. No tightening.
**Score spread (v0.4 alt):** 22 − 9 = 13. Wider than v0.1/v0.3.

v0.4 did not achieve its goal of tightening the 4-player spread for
Slate 1. It made Tide's VP more VISIBLE (good for fun) but also more
STRIPPABLE (bad for balance).

---

## Ready for human playtest?: almost

**Why "almost":**
- Tide's v0.4 rule genuinely changes the play experience — watching tokens
  pile up IS more engaging than v0.3's silent mana gain. This is a real fun
  improvement even if the balance outcome is unchanged.
- Flame's v0.4 SE rule did not exercise in Slate 1. Need PT on a slate
  where Flame is center-board or where opponents have building-range
  doctrines (Expansion Law in other hands, etc.).
- The token-strip-Edict interaction is now the game's dominant endgame
  dynamic. A human playtest is the right forum to test whether this feels
  like "climactic showdown" or "feels-bad robbery."

**Recommended before human playtest:**
1. Run PT12 (Slate 9 — Earth Fortress) to see how Deep Roots interacts with
   Edict-strips of banked VP on the Earth-immune side (does Earth run away
   with it?).
2. Run PT13 (Slate 7 — Iron Snowball) for the 3-Forge Veil case and a
   dense-token Iron Vein / Sanctum Chain accumulation.
3. Consider a v0.4.1 hotfix: cap TM banked tokens at 5 per game OR require
   F cost to bank.

---

## Top remaining issue

**Tide's TM is now a strippable jackpot, not an engine.** v0.4 solved the
invisibility problem by making Tide's scoring visible — but by making it
visible, it made it the top Edict target. Tide's experience is now either
triumphant or crushed; there's no middle ground. A good 4-player Euro
should let each School finish within ~30% of the winner. Tide's v0.4
outcomes are either 147% of Earth (22 vs 15) or 47% of Earth (7 vs 15).

A structural fix: **TM banks 1 VP token per ROUND (max) instead of per
trigger.** Caps Tide's TM VP at 5-6 per game, keeps the "visible scoring"
fun, reduces the Edict-jackpot problem.
