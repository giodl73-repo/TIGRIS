---
name: CRUCIBLE — Design (Rules)
slug: crucible-design
game: 0052-crucible
stage: design
version: 1.0.0
rubric_version: v2.23.23
bet_version: parliament
author: TIGRIS
created: 2026-04-21
updated: 2026-04-21
anchor_persona: feld
anchor_axis: B2
---

# CRUCIBLE — Rules

## 1. Overview & Object of the Game

CRUCIBLE is a 3–4 player alchemical production game played in a shared laboratory. Players gather the four base elements (Earth, Water, Fire, Air) and commit them to a shared Reaction Chamber — a rotating dial with 8 reaction slots. After each round, the chamber advances and reactions resolve: correct element combinations produce valuable compounds; incorrect or absent combinations cause explosions that permanently damage the reactor.

Damage is cumulative and collective. Every explosion adds tokens to the shared Damage track. At 5 Damage, all reactions become more expensive. At 8 Damage, the laboratory collapses and the game ends immediately — everyone scores what they have. Players who play recklessly accelerate the game's end for everyone; players who play too cautiously let rivals claim the most valuable compounds.

Players pursue one of three incommensurable scoring paths: **Gold** (safe, steady), **Elixirs** (volatile, high-value), or **Artifacts** (sequential chain, exponential). Committing to Elixirs means accepting explosions — which hurts everyone. Committing to Artifacts requires three consecutive successful reactions — risky when the chamber is damaged. The three paths cannot be simultaneously maximized.

**Winning condition:** highest VP at game end (either laboratory collapse at 8 Damage, or after 8 complete rounds). Ties broken by fewest held Damage penalties; further ties by Artifact count.

---

## 2. Components

- **1 Reaction Chamber dial** (rotating; 8 Reaction Slots; Damage track 0–8 printed on inner ring; arrow marker for current slot)
- **8 Reaction Slot cards** (one per slot; each specifies: required elements, risk level 0–3, compound output type)
- **Element tiles** — 40 per player (10 each of Earth/Water/Fire/Air; 4 colors)
- **1 shared Element Supply pool** (10 tiles of each type = 40 total in pool)
- **Compound tokens:**
  - Gold ×24 (2 VP each; yellow)
  - Elixir ×16 (3 VP each; purple)
  - Artifact Component ×12 (gray; 3 required to form 1 Artifact)
  - Artifact ×4 (8 VP each; once formed from 3 Components; gold-bordered)
- **Damage tokens** ×10 (black; placed on Damage track)
- **Explosion penalty tokens** ×12 (red; personal −1 VP each at game end)
- **VP score board** + **4 VP markers**
- **4 personal Lab boards** (element storage + compound collection area)
- **Round track** (8 rounds; collapses end the game early)

---

## 3. Setup

1. Place the Reaction Chamber in the center. Set Damage track to 0. Place the 8 Reaction Slot cards in the 8 slots (arranged by increasing risk level around the dial — Risk 0 slots are adjacent; Risk 3 slots face each other).
2. Each player takes a Lab board and 10 tiles of each element type (40 tiles total).
3. Fill the shared Element Supply pool: place 10 of each type face-up.
4. Set round track to Round 1.
5. First player determined randomly.

---

## 4. Turn Structure

Each round proceeds as follows:

### Phase 1 — Actions (per player, clockwise)
Starting with first player, each player takes **2 Actions**. Continue around the table until all players have taken their 2 actions.

### Phase 2 — Rondel Advancement
Advance the Reaction Chamber 1 step clockwise. **All 8 slots resolve simultaneously:**
- **Success**: slot has correct matching elements → place 1 Compound token of the slot's type on the triggering player's Lab board; remove elements from slot.
- **Explosion**: slot has mismatched or no elements → add Risk Level Damage tokens to the Damage track; all elements in that slot are returned to the pool; each player who placed in that slot takes 1 Explosion penalty token.

### Phase 3 — Damage check
After all slots resolve: if Damage track reaches **8**, the laboratory collapses — skip remaining rounds and proceed to Final Scoring. If Damage track reaches **5**, announce "Reactor Stressed" — all future Place actions cost 1 extra element (see §5).

### Phase 4 — Round advance
Move round track forward. If Round 8 just completed, proceed to Final Scoring.

---

## 5. Actions

Players choose 2 Actions per turn (may repeat the same action twice):

| Action | Cost | Effect |
|---|---|---|
| **Gather** | Free | Take up to 3 Element tiles from the shared Supply pool into your personal storage. |
| **Place** | 1+ element tiles (from personal storage) | Place elements onto one Reaction Slot that you have not yet placed on this round. You must place exactly the elements required by the slot (per the Reaction Slot card). If Damage ≥ 5: each Place costs 1 additional matching element (pays "stress tax"). |
| **Repair** | 3 matching Element tiles → discard | Remove 1 Damage token from the Damage track. |
| **Forge Artifact** | 3 Artifact Components (from your Lab board) → discard | Combine 3 Artifact Components into 1 Artifact token (8 VP). |
| **Score** | 1 Compound token (from your Lab board) → discard | Gain VP equal to compound value (Gold: 2 VP; Elixir: 3 VP; Artifact: 8 VP) immediately. |
| **Pass** | Free | End both of your actions for this round early; take 1 element from the pool as compensation. |

**Reaction Slot cards** specify:
- **Required elements**: which types and how many (e.g., "2 Fire + 1 Air")
- **Risk level** (0–3): Damage tokens added on explosion
- **Output type**: Gold, Elixir, or Artifact Component

*Risk levels by slot type:*
- Gold slots: Risk 0–1 (safe; low reward)
- Elixir slots: Risk 2–3 (volatile; higher reward)
- Artifact Component slots: Risk 1–2 (moderate; sequential dependency)

---

## 6. Scoring & End Condition

**End triggers** (whichever occurs first):
1. Damage track reaches **8** → laboratory collapse; game ends immediately.
2. **Round 8** completes.

**Scoring:**
- Score compounds via the **Score** action *during play* (you choose when to cash out).
- Compounds still on your Lab board at game end score at face value: Gold 2 VP, Elixir 3 VP, Artifact 8 VP.
- Artifact Components on your Lab board at game end score **0 VP** (unforged = wasted).
- **Explosion penalty tokens**: −1 VP each.
- **Repair contribution bonus**: player who spent the most total elements on Repair actions across the game earns +3 VP (announced at game end; ties: both earn +2 VP).

**Minimum score**: no designed floor — Explosion penalty tokens can theoretically drive a player to 0 or negative VP. This is intentional (B2 catastrophe consequence).

---

## 7. Edge Cases & Clarifications

- **Slot contention**: two players may not place in the same Reaction Slot in the same round. First-come-first-served: the player who declared their Place action first (earlier in turn order) claims the slot; later players must choose a different slot.
- **Empty slot explosion**: if no player placed in a slot this round, the slot still resolves in Phase 2. An empty slot with Risk > 0 causes an explosion of that risk level (the chamber is unstable). Empty slots with Risk 0 resolve as success with no output (no one earned the compound, no damage).
- **Damage at 5 ("Reactor Stressed")**: the stress tax applies immediately after announcing, starting from the next Place action in the current round. Players who have already placed this round are not retroactively charged.
- **Forge Artifact timing**: Forge Artifact is an action taken during Phase 1. You cannot forge mid-Rondel advancement.
- **Score action timing**: Score may be taken any time during Phase 1 (your action turn). You cannot Score during Phase 2 Rondel advancement.
- **Repair and explosions**: Repair removes 1 Damage token per action. If an explosion occurs *after* a Repair in the same round, the removed token does not protect against the new explosion's Damage.

---

## 8. Special Subsystem — Reaction Slot Cards

Each of the 8 Reaction Slot cards is a distinct recipe:

| Slot | Required elements | Risk | Output |
|---|---|---|---|
| 1 | 2 Earth | 0 | Gold |
| 2 | 2 Water | 0 | Gold |
| 3 | 1 Fire + 1 Air | 1 | Gold |
| 4 | 3 Fire | 2 | Elixir |
| 5 | 2 Air + 1 Water | 2 | Elixir |
| 6 | 3 Air + 1 Fire | 3 | Elixir |
| 7 | 2 Earth + 1 Water | 1 | Artifact Component |
| 8 | 2 Fire + 2 Earth | 2 | Artifact Component |

The chamber's 8-slot arrangement is fixed; the rondel *rotates*, meaning which slot is "active" (adjacent to which player position) changes each round. This creates temporal pressure: a high-risk Elixir slot may be adjacent to you in Round 3 but three steps away in Round 6.

---

## 9. Variants

**3-player adjustment**: remove one Risk-3 Elixir slot from play (replace with a Wild slot: produces any compound type the placed player chooses). Reduces explosion rate for smaller groups.

**Aggressive mode**: start Damage track at 2 (pre-damaged laboratory). Game begins under stress; collapse is imminent from the first round.

---

## 10. Designer Notes — B2 and B2↔C6 Documentation

**B2 Catastrophe Pressure (anchor)**: the Damage track is the game's catastrophe engine. Each explosion adds Damage; at 5 it stresses the reactor (cost increase); at 8 it collapses. The escalation is collective: every player's Risk-3 Elixir slot explosion affects everyone's future reaction costs. B2's canonical earning condition: catastrophe that escalates and is genuinely threatening, not just a minor penalty. The 8-Damage collapse provides both urgency and genuine catastrophe.

**B2↔C6 OP (designed collision)**: B2 asks "how bad is playing dangerously?" (the Damage track consequence); C6 asks "which scoring path should I commit to?" (Gold vs. Elixir vs. Artifact incommensurability). B2 fires on the *consequence* of the choice; C6 fires on the *choice itself*. Different analytical registers: B2 = what happens when you fail to manage risk; C6 = what you're optimizing for. The same rondel mechanism generates both, but they're asking different questions.

---

## 11. Collision Adjacency Chart

| Axes | Adjacency type | Reasoning |
|---|---|---|
| **B2 ↔ C6** | OP candidate (primary) | Catastrophe pressure (consequence of risky play) vs. point-salad incommensurability (which scoring path). B2 = what happens when risk materializes; C6 = which risk level to target. Different analytical registers. |
| **B2 ↔ C1** | co-firing (existing) | Catastrophe pressure (B2 = explosions escalate) drives the tension budget (C1 = Damage track as countdown clock). |
| **C6 ↔ A2** | co-firing (existing) | Incommensurable scoring paths (C6) multiply the per-action decision space (A2): which slot to target depends on which path you're pursuing. |
| **B1 ↔ C6** | co-firing (candidate) | System gearing (B1 = element → reaction → compound → VP) feeds the incommensurable paths (C6 = which compound type to pursue). |

---

## 12. Open at DESIGN-stage

1. **Risk-3 explosion rate**: at 3 Damage tokens per explosion, a player who triggers 3 Risk-3 explosions has singlehandedly collapsed the reactor. Is this too aggressive? Target: 8-round game on average; collapse should threaten from Round 5 onward.
2. **Element supply depletion**: 40 tiles in the shared pool for 3–4 players. Does the supply run out before Round 8? Each Gather action takes 3; each Place action uses 2–4. Estimation: ~15 actions × 3 elements = 45 element draws vs. 40 supply. Depletion is possible. Add rule: if supply empties, players draw from a reserve bag (unlimited supply, but triggers +1 Damage as the lab sources unstable raw elements).
3. **Artifact Component timing**: if a player places in the Artifact Component slot and it succeeds, do they get the Component immediately (during Phase 2) or at start of next round? Currently: immediately (Phase 2). Alternative: next round (creates storage/timing tension). Needs playtesting.
4. **Score action vs. end-of-game scoring**: should compounds score automatically at game end (no Score action needed) or only if a Score action was taken? Current design: compounds on your board at game end score at face value. Score action is used for mid-game VP conversion flexibility. If this is confusing, cut the Score action and score everything at game end.
5. **Repair as a social mechanic**: the player who Repairs most earns +3 VP. Does this create a "Repair race" that undermines B2's catastrophe pressure? Consider removing the bonus if it over-incentivizes Repair.
