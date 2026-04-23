---
playtest: PT13
version: v0.4
slate: 7 — Iron Snowball
key_fix_tested: Edict phase balance with v0.4 token rules
winner: Earth (14 VP) — Iron 2nd (13 VP, MO stripped again)
veil_triggered_round: 5 (end-of-round; never hit 0 during play)
---

# PT13 — v0.4 Slate 7: Iron Snowball

**Simulation mode:** simultaneous activation from R3 onward. Turn order
Flame-first (First to Burn): Flame → Iron → Tide → Earth. Starting mana:
all 3W + 2F. Starting components: one in each corner.

**Slate 7 configuration (v0.4 — identical draft to v0.3 PT08):**
- Iron — Tempered + IV · IB · SC · MO
- Flame — First to Burn + SE · DL · RP · CG
- Tide — Ebb and Flow + TX · EL · TL · DT
- Earth — Deep Roots + CD · FH · SW · PI

**Starting corners & terrain (same as PT08):**
- Iron NW — Forge Ground adj. Sanctum Hills + Flow Channel (starting Forge)
- Tide NE — Flow Channel adj. Sanctum Hills + Void Rift (starting Channel)
- Flame SE — Ember Peaks adj. Forge Ground + Flow Channel (starting Spire)
- Earth SW — Earth territory adj. Sanctum Hills + Flow Channel (starting Sanctum)

**v0.4 rules active (delta vs v0.3):**
1. Deep Roots: doctrines cannot be suspended (stay active). **BUT banked VP tokens CAN be stripped.** (v0.3 had full doctrine immunity too; the wording is clarified.)
2. Iron Vein: Forge adjacent to SANCTUM (carried from v0.3)
3. Mana cap 12 (carried)
4. Encirclement scaled (carried; not drafted)
5. **Edict strips VP tokens — including from active-immune Deep Roots cards.** This is the key mechanical change: Iron can now name Earth's doctrines (still active) and strip any VP tokens already banked.
6. Veil 3-Forge once per player per game (carried)
7. Tidal Mill banks 1 VP token per trigger (N/A — not drafted)
8. Scorched Earth banks 1 VP token per trigger (Flame drafted — tracked)

**What this slate stress-tests for v0.4:**
v0.3 PT08 demonstrated that Iron loses its signature slate because Earth's
Edict strips all 6 of Iron's MO tokens while all of Earth's Deep-Roots-protected
doctrines remain untouchable. v0.4 adds token-strip-through-immunity: Iron can
name Earth's cards and strip their banked tokens.

**But what counts as a "banked token"?** Critical rule clarification (per the
team lead's brief): end-game doctrines — SW (Spider's Web), DT (Deep Territory),
Deep Roots terrain bonus — score at game end, NOT during play. They never
accumulate tokens during play. Edict against them = no tokens to strip AND
they still score at game end (since Deep Roots keeps them active).

This leaves a structural asymmetry open:
- **In-play banking doctrines** (Iron's MO, Flame's SE if it banks, Flame's CG
  if it banks) — strippable by Edict.
- **End-game scoring doctrines** (Earth's SW/DT, Earth's Deep Roots terrain
  bonus) — effectively Edict-proof under v0.4.

Slate 7's matchup sits exactly on this asymmetry. Iron's 4 VP doctrines all
bank (MO is the only real scorer). Earth's SW + DR terrain bonus both score
at the end. **Token-strip Edict changes nothing between Iron and Earth.**

Flame's SE (Scorched Earth, v0.4 banks 1 VP per trigger) is now strippable —
this is new. But Flame is 3rd in the race, not directly competing with Iron
or Earth.

---

## Round-by-round summary

Round-by-round play tracks PT08 exactly, since no drafted doctrines interact
with the v0.4 rule changes differently until the Unraveling's Edict phase.
Iron's Tempered, IV, IB, SC all behave identically to PT08. Earth's PI, CD,
FH, Deep Roots all behave identically. Tide is still stalled without TM.
Flame's SE now banks a token on trigger (new).

Compressed narration (full details in PT08; deltas called out):

### Round 1 (Veil: 10 → 9)
- Flame: DL double-Spire, ends R1 with 1W + 2F + 2E, 3 Spires.
  - **SE check:** SE requires an enemy component being destroyed — not triggered R1.
- Iron: Activate Forge (3W), Build+IB Sanctum (+3F), Activate Sanctum (3F). End: 5W + 5F = 10 mana.
- Tide: 2 Channels, ends 1W + 1F = 2 mana.
- Earth: Activate Sanctum (2F), Build Channel (2F), Convert 2W→1F. PI +2W. End: 3W + 2F = 5 mana.

### Round 2 (Veil: 9 → 8)
- Flame: Build 4th Spire (free), activate 2 Spires → 4E net. End: 1W + 2F + 4E.
  - SE still not triggered (no conflict destructions yet).
- Iron: **IV fires every Forge activation.** Activate F1 (5W), Build+IB F2 (5W), Activate Sanctum (3F). Raw 15W + 3F. Cap 12, keeps 12W + 0F.
- Tide: Still mana-starved. End: 1W + 1F.
- Earth: 2 Sanctum activations + build 2nd Sanctum. PI +2W. End: 5W + 2F.

### Round 3 (Veil: 8 → 7 → 6 → 5 via Flame 5th)
- Flame: Build 5th Spire (+1 Veil), activate contested Spire → CG fires → 4E. **Raiding Party** steals 1W from Iron.
  - Still no SE trigger. Flame has contested position but no destruction yet.
- Iron: Convert 4W→2F, then Build Sanctum #2 → IB+SC fires → 4F. Activate F1 (5W). Raw 11W + 4F. Keeps 11W + 1F.
  - **MO gross-production check**: R3 = 5W + 4F = 9. R2 = 13. R3 < R2 → no MO.
- Tide: stalled, zero-net activations. End: 1W + 1F.
- Earth: 2 Sanctum activates + Build Channel. PI +2W. End: 7W + 2F.

### Round 4 (Veil: 5 → 4 → 3 via Earth 5th)
- Flame: Activate contested (4E), RP steal 1W, activate start Spire. End: 3W + 1F + 8E (capped).
  - *Flame decides to Conflict to trigger SE.* Conflict action against Iron's Sanctum #2 — Iron counter-Doctrine: none drafted. Flame destroys Iron's Sanctum #2? Rules: Conflict destroys a component if attacker spends 2 mana and target has no counter. Costs Flame 2E, Veil +1 (destruction). But this costs Flame 2 mana AND triggers SE for Flame's reward.
  - Actually reconsider: Flame's R4 already used its 3 actions above. Flame cannot Conflict in R4 without sacrificing an action. Move SE trigger attempt to R5.
- Iron: IV fires again. Activate F1 (5W), 2× Sanctum activates with SC (4F each). Raw 14W + 7F. Keeps 8W + 4F.
  - **MO check**: R4 = 5W + 4F + 4F = 13. R3 = 9. R4 > R3 → **MO fires: +2 VP token.**
- Tide: stalled. End: 1W + 1F.
- Earth: 2 Sanctum activates + Build Sanctum #3 (+1 Veil, 5th comp). PI +2W. End: 9W + 2F.

### Round 5 (Veil: 3 start; Unraveling after R5)
- Flame: Activate contested (4E), RP steal 1W, activate start Spire. End mana ~12 (capped).
  - **Flame's SE play:** Flame converts 1 action to Conflict against Iron's Sanctum. 2E cost, destroys Sanctum, +1 Veil, **SE banks 1 VP token**. But this replaces Flame's A3 activation. Net: Flame trades 2E + 2E activation (4E forgone) for 1 SE VP token (=1 VP worth ~2 mana VP). Not worth it unless SE triggers multiply. Flame's SE only fires once on one destruction.
  - Decision: Flame skips SE trigger — it's a mana-losing play for only 1 banked token that Earth can still ignore (Earth has nothing to Edict Flame with that's higher-value than stripping Iron's MO).
  - Actually recalibrate: SE banks 1 VP token per trigger (v0.4 rule 8). If Flame destroys 1 component, SE = 1 VP token (= 1 VP). Flame's cost was ~4 mana = 2 mana VP forgone. **Net +(-1) — bad.** Flame skips. Same as PT08.
- Iron: Activate F1 (5W) + F2 (5W) + Sanctum (4F, SC). Raw 15W + 4F. Keeps 12W + 0F.
  - **MO check**: R5 = 5W + 5W + 4F = 14. R4 = 13. R5 > R4 → **MO fires: +2 VP token.** Iron MO banked: 4 tokens.
  - But recount R2 (5W + 5W + 3F = 13): R2 > R1 (9) → **MO fires R2: +2 VP.**
  - Total MO triggers: R2, R4, R5 = 3 triggers = **6 VP banked on MO**.
- Tide: TL fired — replays R4 at half cost. R4 was zero-net. TL delivers nothing.
- Earth: 3 Sanctum activates with **FH double** — 4F each = 12F gross. Cost 3F = +9F. PI +2W. Raw 9W + 11F. Keeps 2W + 10F (cap 12).

**End R5**: Veil drops to 2. Unraveling begins.

---

## VP token tracking (v0.4 emphasis)

**Doctrines that banked VP tokens DURING play** (strippable):

| School | Doctrine | Tokens banked | Trigger rounds |
|---|---|---|---|
| Iron | MO | 6 | R2, R4, R5 (3 triggers × 2 VP) |
| Flame | SE | 0 | Never triggered (Flame avoided Conflict destructions) |
| Flame | CG | 0 | CG generates mana (2E), not VP tokens. No banking. |
| Flame | RP | 0 | RP steals mana, not VP tokens. |
| Flame | DL | 0 | DL is a placement trigger, no VP banking. |
| Tide | TX, EL, TL, DT | 0 | TX = conversion; EL = placement; TL = action replay; DT scores at end game. |
| Earth | PI | 0 | PI produces mana (not VP tokens). |
| Earth | CD | 0 | CD is reactive mana; no VP. |
| Earth | FH | 0 | FH doubles activations; no VP banking. |
| Earth | SW | 0 | **End-game scoring only. No banked tokens during play.** |
| Earth | Deep Roots terrain | 0 | **End-game scoring only. No banked tokens.** |

**Only Iron's MO has banked tokens.** Everyone else's VP either (a) comes from
map features and end-game doctrines (no banking), or (b) is passive production
with no VP. Flame's SE was in the draft but never triggered.

**Implication for Edict phase:** The only strippable target on the entire board
is Iron's MO. This is functionally identical to v0.3 PT08.

---

## Edict phase analysis

v0.4 changed two things for Edicts:
1. **Can now strip VP tokens from Deep Roots cards** (card stays active, tokens removed). PT13 outcome: Earth's Deep-Roots-protected cards (SW, DT, DR terrain) have zero banked tokens, so the change is inert.
2. **Can strip active-immune doctrine tokens**. Same outcome — nothing to strip.

### Edicts in reverse turn order: Earth → Tide → Iron → Flame

**Earth Edict** (first):
- Best target: Iron's MO (6 banked VP tokens). **Strip 6 VP.** Same as v0.3 PT08.
- Card is suspended (not Deep Roots protected — Iron has no immunity) and tokens are stripped.

**Tide Edict**:
- Iron's MO already stripped. Iron's IV, IB, SC have 0 tokens and no endgame score.
- Earth's SW has 0 banked tokens. Naming it suspends it? **No** — Deep Roots keeps it active. Stripping banked tokens? None to strip.
- **v0.4 new option for Tide:** name Earth's SW. Result: 0 tokens stripped, SW stays active via Deep Roots, SW still scores at game end. **Wasted.**
- Tide's least-bad play: name Flame's SE to block its hypothetical future triggers. But SE already never triggered; no forward VP to block.
- Tide names Iron's IV (preserve-the-no-change-from-v0.3 choice). Wasted.

**Iron Edict**:
- Iron's best target: **Earth's SW** (5 VP forecast at game end). v0.4 permits naming SW. Effect: 0 tokens stripped (end-game scoring, no banking). SW stays active (Deep Roots). **SW still scores 5 VP at game end.**
- Iron's next target: Earth's Deep Roots terrain bonus (2 VP forecast). Same problem: no tokens, still scores.
- **Key finding: Iron's Edict against Earth in v0.4 achieves nothing.**
- Fallback: Iron names Tide's DT (2 VP forecast). DT isn't Deep Roots protected (Tide isn't Earth). Name DT → suspend it, block 2 VP. **Worth 2 VP.** Same as PT08.
- Iron picks Tide's DT. **-2 VP to Tide.**

**Flame Edict**:
- Same problem as Tide. Earth's SW/DR/DT all have 0 strippable tokens.
- Flame names something; wasted. (Names Iron's IB — 0 tokens, 0 endgame — pure formality.)

### Edict summary
| Edictor | Target | Tokens stripped | Forward VP blocked | VP impact |
|---|---|---|---|---|
| Earth | Iron's MO | 6 | ~0 (R5 was final) | -6 Iron |
| Tide | Iron's IV | 0 | 0 | 0 |
| Iron | Tide's DT | 0 | 2 | -2 Tide |
| Flame | Iron's IB | 0 | 0 | 0 |

**Iron tried to Edict Earth's SW for the first time under v0.4 rules. It didn't work — SW has no banked tokens to strip, and Deep Roots keeps it active for end-game scoring.**

---

## Final scores

Unraveling Step 1 (Final Activation) identical to PT08 mana flow:
- Iron: 12W cap + 2 Forge activations (+8W) + 2 Sanctum activations (+6F) = **20W + 6F = 26 mana**.
- Flame: 12 cap + activations = **19 mana** (2W + 17E).
- Tide: **2 mana** (1W + 1F).
- Earth: 12 cap + FH-active Final Activation (3 Sanctums +3F, 2 Channels 0 net) = 2W + 13F = **15 mana**.

| School | Doctrine VP | Stripped tokens | Map VP | Mana VP | Total |
|---|---|---|---|---|---|
| Iron | 0 (MO 6 tokens stripped) | -6 | 0 | 13 | **13** |
| Flame | 0 | 0 | 0 | 9 | **9** |
| Tide | 0 (DT 2 VP blocked) | 0 (end-game, not token strip) | 0 | 1 | **1** |
| Earth | 7 (SW 5 + DR terrain 2) | 0 | 0 | 7 | **14** |

**Winner: Earth, 14 VP. Iron 2nd, 13 VP. 1 VP margin — same result as v0.3 PT08.**

---

## Fix validation

### Iron vs Earth Edict symmetry: can Iron now meaningfully target Earth's VP?

**No.** v0.4 permits Iron to *name* Earth's SW/DR/DT under Edict, but the
mechanic (strip banked tokens) does nothing against end-game-scoring doctrines.
Earth's 7 VP of endgame Doctrine scoring is untouched. Iron's Edict against
Earth remains functionally equivalent to v0.3 — it still achieves zero.

The rule change is **mechanically valid but strategically inert** for this
matchup. Earth still wins by 1 VP. Same margin as PT08.

### End-game doctrine immunity: is SW/DT/Deep Roots terrain still effectively immune?

**Yes — structurally immune under v0.4's token-strip rule.** The clarification
that end-game doctrines have no banked tokens during play means they are
**effectively Edict-proof** even when named.

This is not a bug — it's a direct consequence of v0.4's design. The rule
"Edict strips banked tokens" assumes banking happens during play. End-game
doctrines don't bank during play; they calculate at scoring. So there's
nothing to strip, and if the doctrine stays active (Deep Roots), it still
scores.

### Does Iron win Slate 7?

**No. Iron still loses by 1 VP.** v0.4 did not change the outcome.

Iron banked 6 VP on MO, lost all 6 to Earth's Edict, scored 13 mana VP total.
Earth scored 5 SW + 2 DR terrain + 7 mana = 14 VP.

### New asymmetry identified

**Yes. The end-game/in-play asymmetry is now the dominant balance axis.**

Pre-v0.4: asymmetry was "Earth is Edict-immune, others aren't."
v0.4: asymmetry is "end-game doctrines are Edict-immune, in-play banking
doctrines aren't."

Earth's kit (SW, DT, Deep Roots terrain) skews heavily to end-game scoring.
Iron's kit (MO, production multipliers) skews heavily to in-play banking.
**The asymmetry didn't disappear — it got relabeled.**

Flame's SE and Tidal Mill (v0.4 rule 7) both newly bank tokens during play,
making them strippable. This compresses the strategic space further:
*any doctrine that banks during play is risky to invest in; any that scores
at game end is Edict-safe.* Rational drafters will over-pick end-game scoring.

### Balance verdict: 2/5

- **v0.4 Edict token-strip through Deep Roots (the intended fix): 1/5 — cosmetic.**
  The change lets Iron name Earth's cards but achieves no VP transfer because
  Earth's cards don't bank. The "fix" fires zero times in Slate 7.
- **v0.4 Deep Roots clarification (immune doctrine, strippable tokens): 3/5 —
  coherent rule, but inert in this matchup.** Would matter if Earth drafted
  a banking doctrine like MO or a new v0.4 in-play-banking card.
- **v0.4 SE banks 1 VP/trigger: 3/5 — rules-coherent, but SE never triggered
  this slate.** Flame avoided Conflict destructions because the math didn't pay.
  Need a slate where Flame has conflict incentives to test this.
- **v0.4 TM banks 1 VP/trigger: N/A — not drafted.**
- **Iron Snowball outcome: unchanged from v0.3.** Slate 7 still resolves
  Earth 14, Iron 13. The slate name is a lie: Iron does not snowball to
  victory even in its signature configuration.

### Recommended follow-up fix

**The token-strip Edict rule is the wrong dial.** v0.4 tried to give Iron
a lever against Earth's immunity, but Earth's VP comes from end-game scoring
(no tokens to strip). The correct fix targets the end-game/in-play asymmetry
directly. Three options, ranked:

1. **Edicts also block future VP from end-game doctrines** (not just in-play
   banking). This is the simplest fix: name SW, SW stops scoring at game end
   (even though Deep Roots keeps it "active" for other mechanical effects).
   This restores Edict leverage against Earth's actual win condition.

2. **Cap Edict token-stripping at 3 VP per target** so MO-stripping doesn't
   one-shot Iron's entire VP path. Forces Earth to split Edicts across
   multiple targets; Iron keeps some MO value.

3. **Make MO score end-game (1 VP per MO-eligible round survived)** so MO
   banking moves to end-game and becomes as Edict-resistant as Earth's kit.
   Aligns production Schools with the "end-game-scoring is safe" rule.

Recommend (1) for PT14: test Edict-blocks-end-game-scoring on Slate 7.
Hypothesis: Iron's Edict against SW removes Earth's 5 VP, flipping Earth
to 9, Iron stays at 13. **Iron wins by 4.** This validates Slate 7's name.

If (1) is too swingy, (2) is the scalpel.

---

**Slate verdict (v0.4)**: The v0.4 Edict rule changes are mechanically
coherent but **strategically inert** for Slate 7. The Iron-vs-Earth asymmetry
is preserved under a new label (in-play vs end-game banking). Iron still
loses its signature slate by 1 VP. The token-strip mechanic needed to be
paired with an end-game-scoring block to actually level the Edict phase.

**The team lead's hypothesis — that v0.4's token strip fixes Iron's Edict
problem — is not supported by this playtest.** The fix fires in name only;
no VP transfers. A structural fix (option 1 above) is needed for v0.5.
