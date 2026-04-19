---
name: TIGRIS Playtest Rubric
slug: playtest-rubric
version: 1.0.0
rubric_version: v1.0
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# TIGRIS Playtest Rubric — v1.0

Eight axes. Each scored 0–10. Axes are weighted per persona / lens via their `rubric_weights` vector (sum = 8.0).

Cell verdicts apply to the **weighted aggregate** of a persona's 8 axis scores at a single player-count:
- `win` — aggregate ≥ 7.0
- `marginal` — aggregate 4.0–6.9
- `fail` — aggregate < 4.0

GATE (from spec §7.4): proceed to Tier-B when ≥ 50% of cells are marginal-or-better, the anchor-persona cell is `win`, and no axis scores ≤ 3 across ≥ 3 personas at the target player count.

## The eight axes

### 1. Elegance

**Definition:** Rule-count-to-depth ratio. How much strategic space does each rule create?

- 0: Rulebook bloats; each rule adds less than one distinct strategic concept.
- 2: Rules outnumber the concepts they support.
- 5: Rules match depth (one rule, one concept).
- 7: Rules compress into more depth than the rulebook's page count suggests.
- 10: Rule count understates depth; emergent complexity the rules didn't spell out (e.g., T&E's catastrophe triggers).

**Diagnostic test:** Can you summarize what the game is *about* strategically in fewer words than it takes to teach the rules?

### 2. Decision Density

**Definition:** Meaningful choices per turn.

- 0: Many turns auto-resolve or have a dominant action.
- 3: One interesting decision every 2–3 turns.
- 5: About one real decision per turn.
- 7: Every turn presents a multi-axis choice (which action, which target, which cost).
- 10: Every turn *and* every sub-decision is live; no wasted pip, no dead space.

**Diagnostic test:** What percentage of turns could a reasonable AI play with <1 second of thought? If >20%, density is low.

### 3. Interaction

**Definition:** Player-to-player impact.

- 0: Multiplayer solitaire. Each player optimizes their own board with no cross-influence.
- 3: Shared resources; indirect competition only.
- 5: Indirect competition for contested spots plus occasional direct actions.
- 7: Direct confrontation is common and drives the game arc.
- 10: The game *is* the interaction. Remove opponents and the game has no content (e.g., auction games, T&E conflicts).

**Diagnostic test:** How does the game change when a player plays well vs. poorly? If it only affects their own score, interaction is low.

### 4. Thematic Integration

**Definition:** Mechanics ↔ theme coherence.

- 0: Theme is paint over a generic mechanic; re-skinnable without loss.
- 3: Theme rhymes loosely with mechanics.
- 5: Mechanics and theme rhyme plausibly.
- 7: Theme predicts mechanics (a newcomer could guess half the rules from the theme).
- 10: Mechanics **are** the theme (e.g., Agricola's feeding pressure *is* medieval subsistence; T&E's aspect-balance *is* civilization).

**Diagnostic test:** Strip the theme and replace with abstract labels. Does the game still make sense as a system? If yes, low. If no, high.

### 5. Variance Calibration

**Definition:** Luck scaled to game length.

- 0: Randomness dominates skilled play over a full arc.
- 3: Luck drives outcomes more often than skill.
- 5: Luck early, skill late. Standard Euro calibration.
- 7: Luck is a resource players actively manage.
- 10: Every random element has a mitigation tool; variance creates strategy space rather than noise.

**Diagnostic test:** If two equally-skilled players play 10 times, how lopsided can the series be? 10–0 = over-luck. 6–4 = calibrated.

### 6. Downtime / Pacing

**Definition:** Wait between meaningful player activity.

- 0: >5 minutes of pure observation between meaningful decisions for an average player.
- 3: Noticeable downtime, especially at higher counts.
- 5: Moderate pacing, AP-tolerant.
- 7: Turns are short or overlapping; little dead time.
- 10: Turns interleave. There is no downtime — players always have a live decision (e.g., simultaneous drafting, real-time layers).

**Diagnostic test:** Average wait time from end-of-your-turn to beginning-of-your-next-turn at 4p. Factor out AP players.

### 7. Teachability

**Definition:** Rules → first-turn friction.

- 0: Rulebook must be referenced mid-game on the first play; cannot be taught verbally.
- 3: Teach takes >30 min or requires a reference sheet for every player.
- 5: Can play after one thorough read; occasional rule check.
- 7: Can teach verbally in 10–15 minutes.
- 10: Can teach in under 10 minutes; first-turn decisions are comprehensible without errata.

**Diagnostic test:** Time to first meaningful turn with a new player. If longer than 15 minutes, teachability ≤ 5.

### 8. Emergence / Replayability

**Definition:** Strategy space diversity across plays.

- 0: Solved game. One dominant opener; most games play out the same way.
- 3: Two or three paths, all variants of the same core strategy.
- 5: Several distinct strategies; opening varies by table.
- 7: Genuinely different arcs across plays; no solved state after 10 plays.
- 10: 50+ plays still reveal new strategic veins; meta evolves over the life of a play group.

**Diagnostic test:** After 10 plays, can a veteran predict the winning strategy before turn 3? If yes, emergence is low.

## Scoring protocol

A persona scoring a game on these axes must:
1. Read the design.md and any referenced rules.
2. For each axis, give a score 0–10, plus a one-sentence justification grounded in a specific rule or mechanic.
3. Use only the diagnostic language from `personas/forbidden-words.md`.
4. Compute the weighted aggregate using their `rubric_weights` vector.
5. Produce a cell verdict per player-count evaluated.

## Amendment protocol (forward-only)

When the innovation log clusters 2+ entries in the same dimension across ≥ 2 games:
1. The cluster proposes an amendment in `personas/playtest-innovations.md`.
2. User approves or rejects.
3. If approved, the amendment is added here with a new version number (v1.1, v1.2, …).
4. Prior scores stay locked at their original `rubric_version`. They are **not** retro-adjusted.
5. Subsequent games score against the new version.

## Changelog

- **v1.0** — 2026-04-19 — Initial rubric per spec `docs/specs/2026-04-19-tigris-design.md`.
