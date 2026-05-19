# Parliament Native Experience Plan

## Status

- Lifecycle state: MUDDLE native beautification pass
- Current version: v0.visual-table
- Last evidence source: Macroquad Parliament play review and TIGRIS role review
- Next gate: novice/table-experience persona transcript over the richer table

## Experience thesis

The TIGRIS Parliament native slice should read like a live argument table, not a
generic command runner. A player should see three zones at a glance: persona
seating, axis-board disagreement, and score-ledger amendment. The visual scene
should make TIGRIS' factory promise visible: argument becomes evidence, evidence
becomes a forward-only rubric change.

## Visual pillars

| Pillar | Player read | Implementation direction |
|---|---|---|
| Designer seat | Who is making the argument? | Persona mat, Knizia seat state, preferred-axis reminder |
| Axis vocabulary | What is being defended? | Axis Pool deck, human Tension Budget card, stake token bowl |
| Visible dissent | Where does pressure bite? | Collision lane, AI chair, pressure dial, dissent tags |
| Evidence ledger | Why did the session matter? | Raw score label, rubric ledger, adoption sticker, close gavel |
| Factory continuity | What changes next time? | Closed Parliament badge and next-session amendment prompt |

## Role review

| Role | Finding | Decision |
|---|---|---|
| Game Space Cartographer | The native slice showed the loop but not the TIGRIS coordinate/factory identity. | Use table zones and artifacts to show Parliament's target space: design disagreement as a playable system. |
| Mechanism Tension Editor | Pressure was numeric but not tactile. | Add visible stake, collision, AI pressure, tiger, and dissent markers so the bite lives on the board. |
| Table Experience Observer | Players need to know what they are supposed to feel in each room. | Add room-local player-read prompts and optional inspect beats that explain the current table moment. |
| Axis Governance Steward | The closeout must not imply arbitrary rubric change. | Keep the ledger/adoption sticker tied to scoring and closure state. |

## Implementation plan

1. Keep MUDDLE product-neutral; use existing generic frame states.
2. Add TIGRIS-owned visual nodes for persona mat, axis deck, stake bowl,
   collision lane, AI chair, pressure dial, tiger marker, dissent tags, rubric
   ledger, adoption sticker, and close badge.
3. Add optional `inspect ...` commands for table, board/collision, pressure, AI,
   and ledger explanation without mutating state.
4. Raise the visual-smoke density gate so TIGRIS cannot regress to a sparse
   three-token scene.

## Non-goals

- No new Parliament rules in this pass.
- No new shared MUDDLE rendering behavior unless multiple products need it.
- No art asset pipeline yet; sprite paths remain semantic placeholders for the
  Macroquad renderer.
