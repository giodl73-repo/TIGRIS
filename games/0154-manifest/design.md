---
name: MANIFEST — Design (Rules)
slug: manifest-design
game: 0154-manifest
stage: design
version: 1.0.0
rubric_version: v2.24.85
bet_version: parliament
author: TIGRIS
created: 2026-04-23
updated: 2026-04-23
anchor_persona: rosenberg
anchor_axis: B3
gap_target: Gap3-G+R
---

# MANIFEST — Rules

## 1. Overview & Object of the Game

MANIFEST is a river-trading Euro game for 2–5 players. Each player commands a merchant
ship on a river shared by all. Every ship is identical in capability; what differs is
your **manifest** — the cargo commitments you drafted at the season's opening. Those
obligations have named parties, deadlines, and consequences. They are yours to fulfill,
or to carry as a growing weight.

The river has one port. The port has rules — which cargo berths first, which goods pay
customs, when the deep-water channel opens, who gets the repair yard. Rules are set each
round by whoever holds the harbormaster's seal. The seal belongs to whoever earned the
influence to claim it. Influence is earned by fulfilling obligations. Authority is not
purchased; it is won by doing the work.

Every round, each player faces the same binary: **DOCK** (stay in port, compete for
authority) or **SAIL** (go to the river, earn cargo income). You cannot do both. Over
five rounds — one trading season — players collect raw goods upriver, process them at
port into finished goods, and deliver them to downstream markets. Goods pass through
up to three transformations: Timber → Planks → Furniture; Grain → Flour → Bread;
Wool → Cloth → Garments. The deeper you run the chain, the more you score.

**The winner** is the player who best balances fulfilling their own obligations,
navigating the constraints others impose, and occasionally holding the seal long enough
to set rules they can keep.

---

## 2. Components

**Player materials (×5, per color):**
- 1 ship mat (holds cargo, tracks constraint tokens)
- 1 ship pawn
- 1 DOCK/SAIL decision disc (double-sided: anchor / wave)
- 5 cargo tokens (player-colored; used when commandeering rivals' ships)
- 6 constraint tokens
- 10 influence tokens

**River board:**
- 1 linear river board with three zones: UPRIVER / PORT / DOWNSTREAM
- UPRIVER: 4 collection nodes (Timber, Grain, Wool, Fish), each holding 3 raw tokens
  per round
- PORT: 7 building tile slots, 1 harbormaster seal space, 1 bid space per player,
  customs office marker, berth priority track
- DOWNSTREAM: 3 market stalls (Near Market, Town Market, Great Market)

**Manifest deck (30 cards):**
Each card: named party + good type + destination + deadline round + one-line flavor.
12–16 distinct named parties across the deck. Four good types represented equally.
Examples: *"Widow Aldren's grain — owed before the first frost. Deliver to Durnford
Mill by round 3."* / *"The Guild's timber — paid three months past. Deliver to Harbor
Works by round 4."*

**Rule card deck (15 cards):**
Five types, three copies each:
- **Berth Priority** (×5 variants): names which good type berths ahead of others
- **Tidal Window** (×3 variants): deep-water berth open / shallow channel only /
  double tide
- **Customs Rate** (×4 variants): names a good type that pays 1-coin inspection fee
- **Repair Allocation** (×3 variants): one free repair / repair at 3+ tokens / no repairs

Every rule card carries the line: *"This rule applies to all ships, including the
harbormaster's."*

**Voluntary contract deck (20 cards):**
Additional manifest obligations players may take during play. Easier deadlines than the
draft deck but lower bonus VP on completion.

**Building tiles (7):**
Step-1 buildings (2 berths each): Sawmill (Timber→Planks), Millworks (Grain→Flour),
Fulling Mill (Wool→Cloth), Smokehouse (Fish→Smoked Fish).
Step-2 buildings (1 berth each): Carpentry (Planks→Furniture), Bakehouse
(Flour→Bread), Tailor (Cloth→Garments).

**Tokens:**
Raw goods: Timber (×10), Grain (×10), Wool (×10), Fish (×10).
Step-1 processed: Planks (×8), Flour (×8), Cloth (×8), Smoked Fish (×8).
Step-2/step-3 finished: Furniture (×6), Bread (×6), Garments (×6), Provisions (×6).

*Note: Fish → Smoked Fish is step-1; Smoked Fish → Provisions is step-2. Fish has a
two-step chain (not three). Fish reaches the Town Market but not the Great Market.*

**Shared tokens:**
1 harbormaster seal token. 1 starting-player token. 1 round marker (5 spaces).

---

## 3. Setup

1. Place the river board in the center of the table.
2. Place all 7 building tiles on their PORT slots.
3. Seed each UPRIVER collection node with 3 raw good tokens of the matching type.
4. Set up the three DOWNSTREAM market stalls: Near Market (always open), Town Market
   (rounds 2–5, place marker face-down until round 2), Great Market (rounds 4–5,
   face-down until round 4).
5. Shuffle the manifest deck. Deal **3 manifest cards** to each player, face-down.
   Players look at their cards privately. Starting with the first player and proceeding
   in snake order, each player either **keeps** one card from their hand or **passes**
   one card to the center discard. Continue until each player has chosen their 3 manifest
   cards. *This is the manifest draft: you are choosing which obligations to own.*
   Place all chosen manifest cards face-up in front of you. They are public for the
   rest of the game.
6. Place all remaining manifest cards back in the box. Shuffle the voluntary contract
   deck and place it face-down near the board.
7. Shuffle the rule card deck and place it face-down near the harbormaster seal space.
8. Each player places their ship pawn at PORT (center position).
9. Each player takes their starting influence tokens: 3 influence each.
   *Note: no player starts with constraint tokens.*
10. Randomly determine the starting player. Give them the starting-player token.
11. Place the round marker on space 1.

---

## 4. Turn Structure

Each of the five rounds follows this sequence:

**Phase 1 — Declaration**
All players simultaneously hold their DOCK/SAIL disc in a closed fist and reveal
together on a count of three. Players who show DOCK are in port this round. Players
who show SAIL are on the river.

**Phase 2 — Port Phase** *(DOCK players only)*
*(If no player docked: skip to Phase 3. The harbormaster seal is unclaimed this round;
no rule card is played.)*

2a. **Harbormaster bid:** All docked players simultaneously place a number of influence
tokens face-down in their bid space. Reveal simultaneously. Highest bid wins the seal.
Ties: tied players re-bid exactly 1 influence token; repeat if still tied; clockwise
from starting player breaks a final tie. The winner pays their bid to the supply.
Losers keep their bid tokens.

2b. **Rule card:** The harbormaster draws 2 rule cards from the deck, chooses 1 to play
face-up as standing law, and discards the other face-down. The chosen rule applies for
the remainder of the round to all ships, including the harbormaster's.

2c. **Direct order** *(optional)*: The harbormaster may spend **3 influence** to issue
one direct order: they take one of their own cargo tokens and place it on a target
rival's ship mat. They name aloud which obligation this represents (e.g., "I am
commandeering your hold for the Guild's timber"). The named manifest card is placed
face-up beside the token so all players can read it. *The harbormaster's cargo token
and named obligation now sit visibly on the rival's ship.*

2d. **Port work** *(non-harbormaster docked players)*: Each non-harbormaster docked
player takes one port action (choose one):
- **Process:** move 1 batch of goods through 1 step at any available building
  (pay goods of input type; receive goods of output type)
- **Repair:** remove all constraint tokens from your ship
- **Contract:** pay 1 influence to draw 1 voluntary contract card; add it to your
  manifest (face-up)

**Phase 3 — River Phase** *(SAIL players only)*
*(If no player sailed: skip to Phase 4. River nodes do not reseed.)*

SAIL players act in **berth priority order** as specified by the current rule card.
If no berth priority rule is in effect this round, act in clockwise order from
starting player.

Each SAIL player takes their turn in order:

3a. **Move:** advance or retreat your ship pawn 1 or 2 positions on the river track
(Upriver ↔ Port ↔ Downstream). You may not move more than 2 positions per round.

3b. **Action** *(choose one based on current position)*:

**At UPRIVER:**
- *Collect:* take 2 raw good tokens from the node at your current position (the 4
  nodes are fixed: Timber node, Grain node, Wool node, Fish node; you must be at
  the node matching the good). If only 1 token remains at the node, take 1.
- *Collect + contract:* take 1 raw good token AND draw 1 voluntary contract card
  for free.

**At PORT:**
- *Process:* pay goods of the required input type to any building with an open
  berth; receive the output type. Step-1 buildings have 2 berths; step-2 buildings
  have 1 berth. A berth occupied by a DOCK player's goods earlier this round counts
  as used.
- *Deliver (Near Market):* deliver any step-1 or higher processed good to the Near
  Market. Receive **2 VP + 1 influence** per good delivered.

**At DOWNSTREAM:**
- *Deliver (Town Market):* deliver any step-2 or higher finished good. Receive
  **5 VP + 1 influence** per good delivered. *(Available rounds 2–5.)*
- *Deliver (Great Market):* deliver any step-3 fully finished good (Furniture, Bread,
  or Garments only — not Provisions). Receive **9 VP + 2 influence** per good
  delivered. *(Available rounds 4–5 only.)*

*Customs:* if the active rule card imposes a customs fee on your cargo type, pay 1
coin (influence token) to the supply before delivering. If you cannot pay, you may
not deliver that good this phase.

**Phase 4 — Obligation Check**
In player order, each player checks their manifest:

4a. For each manifest card whose deadline round has passed (deadline round < current
round marker): if the obligation is **not yet fulfilled**, take 1 constraint token
per overdue card.

4b. For each commandeered cargo token sitting on your ship mat: if it has been there
for more than 1 round (it was placed last round and still undelivered), take 1
constraint token.

**Phase 5 — Influence Awards**
Award influence for obligations fulfilled this round:
- Fulfilled your own manifest obligation: **+1 influence**
- Fulfilled your own obligation exactly on its deadline round: **+2 influence**
  (replaces the above)
- Fulfilled a chain-complete delivery (step-3 good, on-deadline): **+3 influence**
  (replaces the above)
- Delivered commandeered cargo (harbormaster's token removed from your mat): **+1
  influence**

**Bite-back bonus:** If the harbormaster fulfilled at least one of their own manifest
obligations this round while their own rule card was in effect, they earn **+1
influence**. The table checks this together. It is announced.

**Phase 6 — Constraint Status Check**
- **2 constraint tokens:** ship cargo capacity reduced to 2 hold spaces.
- **3 constraint tokens:** player *must* DOCK next round to repair (they may bid for
  harbormaster as normal; repair counts as their port work action).
- **4+ constraint tokens:** player is dock-bound until all tokens are cleared via repair.

**Phase 7 — Reseed & Advance**
Reseed each UPRIVER node to 3 raw good tokens (replace any tokens taken this round).
Advance the round marker. Pass the starting-player token clockwise.

---

## 5. Actions

Six discrete action types across the Dock/Sail split. Action-menu is divided by
declaration; players never choose from all six simultaneously.

**DOCK actions (choose one):**
| Action | Cost | Effect |
|---|---|---|
| **Bid for seal** | Influence tokens (bid) | Win harbormaster; pay bid to supply |
| **Process** | Input goods | Convert via building (1 step per action) |
| **Repair** | 1 DOCK turn | Remove all constraint tokens from ship |
| **Contract** | 1 influence | Draw 1 voluntary contract card |

**SAIL actions (choose one after moving):**
| Action | Position | Cost | Effect |
|---|---|---|---|
| **Collect** | Upriver | — | Take 2 raw goods from node |
| **Process** | Port | Input goods | Convert via building |
| **Deliver** | Port / Downstream | Goods + customs if applicable | VP + influence per market tier |

**Harbormaster exclusive (within DOCK):**
- Play 1 rule card (mandatory once seal is won)
- Issue 1 direct order (optional; costs 3 influence)

*C7 note: the six action types are split across two modes. A player never faces more
than four choices at once (the dock side or the sail side).*

---

## 6. Scoring & End Condition

The game ends immediately after Phase 7 of round 5.

**Scoring order:**

1. **Delivered goods VP:** accumulated throughout the game at market delivery (already
   scored on the score track as delivered).

2. **On-deadline fulfillment bonus:** +2 VP per manifest obligation delivered exactly
   on or before its deadline round. *(Standard delivery already awarded +1 or +2
   influence in Phase 5; this is an end-game VP bonus on top.)*
   
   *Wait — consolidating to avoid double accounting: influence awarded in Phase 5 is
   separate from end-game VP. End-game bonus: +2 VP per manifest obligation that was
   fulfilled by its deadline. -2 VP per obligation that was not fulfilled at all.*

3. **Unfulfilled obligations penalty:** -2 VP per manifest card (including voluntary
   contracts) not fulfilled by game end.

4. **Constraint token penalty:** -1 VP per constraint token remaining on your ship.

5. **Influence conversion:** every 3 influence tokens remaining = +1 VP (round down).

6. **Harbormaster last-round bonus:** the player holding the harbormaster seal at the
   end of round 5 earns +2 VP.

**Tiebreaker:** most influence tokens remaining, then most fulfilled obligations,
then earliest clockwise from starting player.

**Minimum-score shape:** a player who docked every round, lost every bid, and took
on 4 constraint tokens (never sailed) scores approximately 0–4 VP. A player who
sailed every round and fulfilled all manifest obligations at step-1 scores approximately
22–30 VP. The full spread (step-3 deliveries + on-time bonuses + good bidding) reaches
50–65 VP. The floor is real but not punishing beyond recovery.

---

## 7. Edge Cases & Clarifications

**Both players dock (2p game):** valid. No sailing happens. The harbormaster sets a
rule; the loser takes a port action. Upriver nodes do not reseed this round but do
reseed normally next round. No VP are scored this round from goods delivery.

**All players dock:** valid. No SAIL players means no deliveries, no collection, no
processing via sailing. Docked players can still process via the dock port-work action.
Game continues normally.

**All players sail:** valid. No harbormaster this round. No rule card is played. No
direct orders. The harbormaster seal sits unclaimed; the player with the seal from a
prior round loses it (seal resets to unclaimed).

**Commandeered cargo and game end:** if the harbormaster issued a direct order and the
commandeered cargo is still on the rival's ship at game end, the harbormaster scores
0 VP for that cargo (the gamble failed). The carrier does not score VP for undelivered
cargo. The carrier does not take a constraint token for commandeered cargo that was
still pending at game end — only mid-game non-delivery triggers constraint tokens.

**Constraint tokens beyond 4:** a player may accumulate more than 4 constraint tokens
if obligations and commandeered penalties fire simultaneously. The 4-token dock-bound
rule still applies: they must dock until all tokens are cleared. Tokens above 4 each
cost -1 VP at end game.

**Buildings at capacity:** if all berths of a required building are occupied (by other
players processing this round), you cannot use that building this phase. You may take a
different action. This scarcity is intentional.

**Voluntary contracts at end of deck:** if the voluntary contract deck is exhausted,
no new contracts are available. No reshuffling.

**Influence bid of 0:** a player may bid 0 influence tokens for the harbormaster seal
(intentionally losing; keeps all influence). A 0-bid player does not win the seal if
any other player bids more, including another 0-bid player. Among multiple 0-bid
players, the clockwise tiebreaker applies but the harbormaster seal goes to the
first clockwise player willing to accept it — they must declare acceptance.

**Rule card self-constraint:** if the harbormaster sets a customs rule that applies to
a good type they plan to deliver that same round — but they are DOCKED and thus not
sailing — the rule has no effect on them this round. The bite-back bonus does not
apply if the harbormaster cannot be affected by their own rule due to their DOCK status.
*Design note: this creates a loophole where a harbormaster can set punishing rules
without personal risk by simply docking. Track in playtests — may need a correction.*

---

## 8. Special Subsystems

### The Manifest Draft

The manifest draft at setup is the primary source of strategic asymmetry. Players see
their 3 cards before snake-drafting (each player takes turns keeping one card or passing
it to discard, in snake order). This means players choose their constraints, not receive
them randomly. Choosing a hard obligation deliberately (because you can navigate it) is
rewarded; taking an easy obligation may leave harder ones for rivals.

The draft is not hidden — once you've kept a card, it's face-up. Other players see your
manifest before the game begins. This is intentional: manifest transparency is load-
bearing for the harbormaster's targeting decisions.

### The Commandeering Stack

A player may have multiple commandeered tokens on their ship mat simultaneously (from
multiple rounds of direct orders, or a single round if somehow two orders applied —
which is not possible in standard rules but may arise in variants). Each commandeered
token is a separate obligation. They are tracked separately from manifest cards.

When delivering commandeered cargo, the carrier must announce which commandeered token
is being fulfilled; they remove that specific token and return it to the issuing player's
supply. The carrier earns +1 influence per commandeered token delivered.

### Building Berth Tracking

A simple shared marker (a round disc) is placed on each berth when a player occupies
it for processing. The disc is removed at the end of Phase 3. DOCK players who process
during Phase 2d also occupy berths — their disc is placed during Phase 2d and removed
at end of Phase 3. This means a building can fill up between phases.

---

## 9. Variants

### 2-Player Variant: Mandatory Harbormaster

With 2 players, the Dock/Sail prisoner's dilemma is sharpest. Use this variant:
- In each of rounds 1, 3, and 5: the starting player of that round **must** DOCK
  (they become harbormaster automatically with no bid required; they pay 0 influence).
  They cannot SAIL that round.
- In rounds 2 and 4: free choice as normal.
- The forced harbormaster may still issue a direct order (costs 3 influence as normal).
- This guarantees the authority mechanic fires in every odd round without requiring both
  players to coordinate.

### Market Event Variant (add after 3+ plays)

Shuffle the 20 market event cards. At the start of each round's Declaration phase, flip
1 event card face-up. It modifies this round only. Types: good-type price bonus, weather
delay (penalizes upriver movement), harbor festival (bonus influence for docked players),
storm warning (1 constraint token to all sailing players).
*This variant adds variance; omit for first 2–3 plays.*

### Extended Season (6 rounds)

For 4–5 players who want more time to run full chains: play 6 rounds. Great Market opens
in round 5 (instead of 4). Starting player cycles twice. This variant recommended only
after the 5-round game is well understood; it extends play by 20–30 minutes.

---

## 10. Designer Notes

**On symmetric ships:** ships are identical because the design's strategic asymmetry
must come from the constraint landscape, not engine asymmetry. Asymmetric ship powers
would shift the game toward Gap 1 (E+R) territory and dilute the G+R thesis. The
manifest draft is where you differentiate yourself — before the first round begins.

**On the bite-back rule:** the +1 influence bonus for navigating your own rule is small
deliberately. It should not dominate the economy. Its purpose is to make the mechanic
*legible* — the game tells you that self-constraining is recognized and rewarded, so
players try it. The stories it generates (the harbormaster who set a grain customs rule
and then delivered their own grain anyway, earning the bonus) are worth more than
the influence token.

**On the Fish two-step chain:** Fish has a shorter chain than the other goods by design.
It provides a fast-track path to income for players who need quick influence to maintain
harbormaster bids. It also creates a strategic question: do you commit to deep chains
(higher VP ceiling) or fast chains (lower ceiling but faster influence)? The gap should
be large enough that Fish alone cannot win, but close enough that a well-timed Fish run
is legitimate.

**On the 2p prisoner's dilemma:** the mandatory harbormaster variant solves the dilemma
by removing one player's choice. The resulting game is more like Brass than Agricola at
2p — you take turns being constrained by the other's authority. This is acceptable but
should be tested.

---

## 11. Collision Adjacency Chart

Candidate axis-adjacency pairs implied by MANIFEST's mechanics. Feed to amendment
process after Parliament.

| Pair | Basis | Type | Strength |
|---|---|---|---|
| **B3↔C3** | The conversion chain creates the scarcity: the same step-2 buildings (1 berth each) are contested because completing the chain requires them. Chain depth generates scarcity directly. | Structural | Strong — expected to fire at TIER-A |
| **B3↔B1** | Chain completion earns influence; influence earns authority; authority shapes chain access. The full gearing loop runs through B3. Removing conversion chain depth removes the reason authority matters. | Causal | Strong |
| **B1↔A4** | The feedback loop (sail → influence → authority → rules → sail) is the primary source of variance: different rule configurations in different games create radically different chain access patterns. Variance is emergent from gearing. | Emergent | Moderate — may refute if loop doesn't fire consistently |
| **C3↔D2** | Berth scarcity at the Port zone creates the spatial competition: you must be at Port to process, and Port has limited capacity. Scarcity is spatially located. | Positional | Moderate |

---

## 12. Open Questions at DESIGN-Stage

The panel is expected to pressure-test the following. They are unresolved by design,
not by oversight.

1. **Rule card self-constraint loophole** (§7): a docked harbormaster sets a punishing
   rule with no personal risk. Does this make the bite-back mechanic hollow? Fix
   candidates: (a) harbormaster must SAIL at least once every two rounds or loses seal;
   (b) bite-back bonus also fires if harbormaster's future manifest obligations are
   constrained by their current rule (deferred self-constraint).

2. **2p calibration** (§9): the mandatory harbormaster variant has not been playtested.
   Does it solve the prisoner's dilemma or just shift it? At 2p, is the core
   Dock/Sail tension still felt, or is the game too deterministic?

3. **Fish fast-track ceiling**: is the VP gap between Fish (Town Market max: 5 VP)
   and full-chain goods (Great Market: 9 VP) large enough? Does a Fish-specialist
   strategy remain competitive? The gap should punish pure Fish reliance without
   making Fish worthless.

4. **Harbormaster bid economy**: influence is earned in small increments (1–3 per
   fulfillment). The bid for the seal and the direct-order cost (3 influence) are
   competing sinks. Playtest needed to verify the economy doesn't deadlock (everyone
   too influence-poor to bid meaningfully after round 2).

5. **Named card count and deck composition**: 30 manifest cards with 12–16 named
   parties — are the names memorable enough to generate table stories, or do they
   need more specificity (occupations, locations, personality cues on the card)?

6. **Commandeering split incentive**: carrier earns +1 influence, harbormaster earns
   goods VP. Is the carrier incentive strong enough that commandeered cargo is usually
   delivered, making the mechanic reliable? If carriers routinely neglect (taking the
   constraint token as acceptable collateral), the harbormaster mechanic becomes
   too risky to use. Target: commandeered cargo delivered ≥70% of the time.

7. **Great Market timing**: Great Market opens round 4. With a 5-round game and a
   3-step chain, a player who starts collecting in round 1 can reach step-3 by round
   3 (1 collect, 1 process ×2). Is this too easy to plan? Consider opening Great
   Market in round 5 only, making it a capstone rather than a 2-round window.
