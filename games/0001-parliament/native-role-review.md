# Parliament Native Role Review

## Reviewed roles

| Role | Hard question | Finding | Decision |
|---|---|---|---|
| Game Space Cartographer | What coordinate is this design trying to occupy? | The native slice occupies "factory-as-game" space, but the previous scene looked like infrastructure. | Show three table coordinates: designer seat, argument board, amendment ledger. |
| Mechanism Tension Editor | Where does this mechanism force a player to care? | Scores and pressure existed, but the board did not show why pressure mattered. | Add stake bowl, collision lane, AI pressure, tiger marker, and dissent tags. |
| Table Experience Observer | Did players experience the intended pressure, arc, interaction, and texture? | The arc was command-complete but not self-explanatory. | Add player-read prompts and non-mutating inspections for comprehension. |
| Axis Governance Steward | Did the play record earn this axis change? | Closeout text named adoption, but the visual layer needed ledger discipline. | Tie ledger, adoption sticker, and close badge to scored/closed state only. |
| Physical Surface Director | Does the native app look like a table players can sit at? | The artifacts improved the table, but the surface still risked reading as floating UI cards. | Add tabletop plane, side rails, table edges, and placement shadows before adding more policy variants. |

## Approved slice

1. Beautify the TIGRIS Parliament Macroquad scene with product-owned table
   artifacts and stateful TIGRIS palette frames.
2. Keep the existing solve path stable.
3. Add optional inspect beats only as explanation; they must not alter score,
   pressure, room, or amendment state.
4. Add phase, subject-card, and next-action reads so the richer table is also
   playable without source-code context.
5. Raise TRACKER's TIGRIS visual-smoke thresholds to match the richer table.
6. Preserve tabletop physicality: the scene must keep explicit surface, side,
   and placement nodes in addition to card/token artifacts.

## Follow-up validation slice

The follow-up implementation added three table-experience lenses against the
native slice:

1. `first-time-designer` checks that look/inspect/readable beats explain the
   table before optimization.
2. `competitive-optimizer` checks that phase, pressure, AI, and challenge reads
   support efficient play.
3. `confused-observer` checks that out-of-order commands remain guided and
   recover into a closed Parliament.

These lenses are covered in the TIGRIS harness and the portfolio
`test-muddle-personas.ps1` script so future native beautification must preserve
comprehension, not only visual density.
