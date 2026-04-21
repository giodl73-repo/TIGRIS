---
name: v2.22 Dormancy-Watch — Parliament Review
slug: v2.22-dormancy-watch-review
stage: panel
version: 1.0.0
rubric_version: v2.21
bet_version: parliament
artifact_under_review: docs/specs/2026-04-20-v2.22-dormancy-watch.md
personas_seated: [knizia, rosenberg, feld, lacerda, chvatil, kramer-kiesling, stegmaier, vaccarino]
review_type: meta-review (TIGRIS-internal protocol amendment)
anchor_persona: knizia
anchor_axis: C1
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# v2.22 Dormancy-Watch — Parliament Review

Spec-review of `docs/specs/2026-04-20-v2.22-dormancy-watch.md`. 8 designers, no lenses (meta-review). Anchor: Knizia on C1 Tension Budget — the dormancy-watch amendment is fundamentally a tension-clock question: how long should a narrow axis persist before the factory actively tests it?

## Drafts

### Knizia (anchor)
1. **C1 Tension Budget.** 7/10. "The dormancy-watch mechanism creates a clean tension clock for narrow axes: 8-game earn-window → 3-game targeted-earn window → de-adoption review. Each threshold is a decision point. Well-calibrated. Earn."
2. **A1 Elegance.** 6/10. "3-tier state machine (adopted → dormancy-watch → de-adoption-review) is minimal but adds complexity. The 8-game qualifier is an arbitrary number. Hold."
3. **C2 Minimum-Score Shape.** 1/10. "N/A to protocol. Retire-explicit."

### Rosenberg
1. **B1 System Gearing.** 5/10. "Retire-explicit accumulation → 2.0R threshold → dormancy-watch → targeted earn → de-adoption review. 4-gear chain from stake to retirement. Works, but gear 2→3 (2.0R → dormancy-watch) is not deterministic — requires the 8-game qualifier. Hold."
2. **D4 Late-Game Lock-in.** 6/10. "Once an axis enters de-adoption review, the result is permanent (retire or return to adopted). Appropriate lock-in for an axis that has exhausted its dormancy window. Earn."
3. **C3 Scarcity Bite.** 1/10. "N/A. Retire-explicit."

### Feld
1. **B2 Catastrophe Pressure.** 5/10. "The catastrophe case: if C2 fails all 3 targeted earn games, a de-adoption review fires. That review could retire a foundational axis. B2 earns if the spec adequately protects against premature axis death. The 3-game window is thin — B6 was dormant for 15 games and recovered. Hold."
2. **A2 Decision Density.** 6/10. "Dormancy-watch creates a meaningful new factory decision: 'is this game a good targeted-earn candidate for C2?' This adds strategic dimension to game selection. Earn."
3. **C6 Point-Salad.** 1/10. "N/A. Retire-explicit."

### Lacerda
1. **B4 Information-Transparency-Cost.** 7/10. "The spec makes the axis state fully transparent: dormancy-watch axes are flagged in axis-pool.md, their targeted-earn windows are documented, and de-adoption review procedure is defined. Excellent B4. Earn."
2. **A7 Emergence-Replayability.** 1/10. "N/A. Retire-explicit."
3. **B3 Conversion Chain Depth.** 2/10. "Protocol chain is 4-step (retire-explicit → 2.0R → dormancy-watch → targeted-earn → de-adoption). Chain length is appropriate but B3 earns on resource pipelines, not protocol chains. Retire-explicit."

### Chvátil
1. **B5 Architectural Novelty.** 4/10. "Dormancy-watch is a novel state machine not seen in other adversarial-review protocols. However, the spec correctly notes that B5's novelty claim requires the mechanism to be load-bearing in the game design — a protocol state isn't a game mechanism. Hold."
2. **A3 Interaction.** 1/10. "N/A. Retire-explicit."
3. **A4 Variance Calibration.** 5/10. "The 8-game dormancy qualifier and 3-game targeted-earn window create calibrated variance in when axes face review. The numbers aren't arbitrary — 8 games is long enough to span different game types. Earn-leaning."

### Kramer-Kiesling
1. **C7 Action-Menu Clarity.** 7/10. "§Immediate application table is precise: B5 and C4 are flagged as monitoring-only (earned within 8 games), C2 enters dormancy-watch. The protocol's decision tree is clear at each branch point. Earn."
2. **D1 Family-to-Expert Scaling.** 4/10. "Dormancy-watch is accessible to a new TIGRIS contributor (§Dormancy-watch status is 3 bullet points). De-adoption review is more complex. Moderate scaling gradient. Hold."
3. **D2 Spatial-Interaction.** 1/10. "N/A. Retire-explicit."

### Stegmaier
1. **A6 Teachability.** 7/10. "§Problem is the clearest section in the spec — it defines retire-explicit vs. formal refutation with a worked example (C5 was genuinely wrong; B5/C2/C4 are scope-limited). The distinction is teachable. Earn."
2. **D3 Count-Robustness.** 1/10. "N/A. Retire-explicit."
3. **C8 First-Turn Compression.** 1/10. "N/A. Retire-explicit."

### Vaccarino
1. **A4 Variance Calibration.** 4/10. "Already staked by Chvátil. Retire-explicit (duplicate stake)." → RETIRE-EXPLICIT (duplicate; Vaccarino should have checked.)
2. **C4 Engine-Garden.** 2/10. "N/A. Retire-explicit."
3. **A5 Downtime-Pacing.** 1/10. "N/A. Retire-explicit."

*Note: Vaccarino's A4 was already drafted by Chvátil. Under v2.1 rules, no axis may be drafted twice in the same game. Vaccarino's A4 is a procedural violation — rule: retire-explicit on the duplicated draft. Vaccarino's stake is voided; no amendment contribution from A4 (Vaccarino). Chvátil's A4 stands.*

## GATE (spec-review; no anchor-adjacency check required)

1. Anchor viable? C1 (Knizia) on tension-clock interpretation of dormancy-watch. Pass.
2. Coverage ≥ 3 bands? A (A1, A2, A3, A4, A6, A7), B (B1, B2, B3, B4, B5), C (C1, C2, C3, C6, C7, C8), D (D1, D2, D3, D4). Pass.
3. ≥1 collision candidate? B2 (Feld) vs C7 (K-K) — catastrophe-protection concern vs. action-menu clarity. Pass.

## Argument

### §Problem — is the retire-explicit/formal-refutation distinction real?

**Knizia (C1 anchor):** "§Problem correctly identifies the tension: the retirement trigger treats retire-explicit (0.5 weight) and formal refutation (1.0 weight) as the same type of signal, just different magnitudes. But they're qualitatively different: retire-explicit says 'this game wasn't the right test'; formal refutation says 'this game proved the axis wrong.' The C1 tension clock for the spec is: how long before you accept that an axis will never find the right game? The 8-game window is the answer. +1 defend on C1."

**Stegmaier (A6):** "§Problem's worked example (C5 genuinely wrong vs. B5/C2/C4 scope-limited) is the teachable core. A contributor reading this can immediately distinguish their situation. +1 defend on A6."

**Lacerda (B4):** "§Problem documents the exact counts (B5: 2.0R, C2: 2.0R, C4: 2.0R) and their sources. Fully transparent. +1 defend on B4."

**Chvátil (B5):** "The distinction between scope-limitation and argument-failure is real in the corpus. B5 retires-explicit in games that disclaim novelty — that's not an argument that novelty is an invalid concept. Hold on B5 (moderate novelty claim for the protocol mechanism itself)."

### §Dormancy-watch state — 8-game qualifier and 3-game window

**Feld (B2):** "The 3-game targeted-earn window is thin. B6 was dormant for 15 games before recovering at GARNER. If the factory hadn't happened to design a majority-bonus game, B6 would have failed 3 targeted games and been de-adopted. Is 3 games enough?"

**Knizia (C1) defends against B2:** "The targeted-earn window is specifically for games where the axis IS plausibly applicable — the factory has to choose appropriate games. B6's dormancy wasn't targeted; it was incidental. Under v2.22, B6 would never have entered dormancy-watch because it earned in GARNER before the 8-game qualifier + 2.0R combination triggered. The targeted-earn window and B6's dormancy are different scenarios. +1 defend on C1 (tension clock is correctly calibrated)."

**K-K (C7):** "The 3-game window gives the factory 3 explicit decision points: 'is game #N an appropriate C2 test?' If yes and C2 earns: clear. If yes and C2 fails: count. This is clearer than the current implicit situation where axes drift without tracking. +1 defend on C7."

### COLLISION — B2 (Feld) vs C7 (K-K)

**Feld (B2):** "The catastrophe scenario: C2 Minimum-Score Shape has been dormant for many games. The 3 targeted games for C2 are Codenames (#25), HOP-005 Nest (#26), HOP-001 Furrow (#27) per the spec. Codenames is a word game — C2 is almost certainly a retire-explicit there (no numerical scoring floor). Nest is domestic tile-laying — C2 might earn if the design includes a minimum-floor. Furrow is trick-taking — C2 could earn via harvest-floor mechanics. But all three could legitimately fail, triggering de-adoption of a foundational Knizia axis. The catastrophe risk is real."

**K-K (C7):** "The de-adoption review procedure (§De-adoption review) requires a full Parliamentary procedure — not a rubber stamp. Knizia writes a defense brief; the opposing case is argued; Parliament votes with a Retire ≥ 5-of-8 threshold. That's high bar for retirement. If C2 genuinely can't earn in 3 specifically designed targeted games, the de-adoption review may correctly retire it. The action menu is clear at every step."

**Vote on CR or OP:**
- Knizia → OP: "B2 asks 'is the catastrophe risk adequately guarded?' C7 asks 'is the procedure clear?' Different questions about the same mechanism."
- Rosenberg → OP: "B2 is about failure modes; C7 is about decision-clarity. Both fire simultaneously here — neither subsumes the other."
- Lacerda → OP: "B4 perspective: the spec is transparent about both the risk (B2) and the clarity (C7). They're orthogonal dimensions of evaluation."
- Feld → CR: "C7 clarity is a precondition for B2 safety — if the procedure is clear, the catastrophe is adequately guarded. C7 wins."
- Chvátil → OP: "B2 addresses 'what happens if everything goes wrong'; C7 addresses 'is the normal path well-defined'. Different registers."
- Stegmaier → OP: "A6 teachability perspective: I teach B2 (risks) separately from C7 (procedure). Different teaching moments."
- Vaccarino → OP.

**Result: 6-1 for OP** (Feld only dissent). B2 and C7 both earn. OP confirms both axes fire on different evaluation dimensions of the same spec.

### §Immediate application — B5/C4 monitoring vs. C2 dormancy-watch

**Knizia (C1):** "The split between B5/C4 (monitoring; earned within 8 games) and C2 (dormancy-watch; has not earned recently) is the spec's most precise judgment. B5 earned at Pandemic Legacy #22 (2 games ago); C4 earned at CANTON #24 (0 games ago). Neither triggers the 8-game dormancy qualifier. C2's last canonical earn is likely ZEN PATH #10 or CoB #12 — 12+ games ago. The split is well-evidenced. +1 defend on C1 (2nd defend)."

**Feld (A2):** "The immediate application creates a concrete factory decision: 'Should Nest #26 include a minimum-score floor for C2?' This is a good decision to surface — better than drifting without awareness. +1 defend on A2."

**Rosenberg (D4):** "C2's de-adoption review (if triggered) is permanent — it would retire or return C2 to full adopted status with no ambiguity. Appropriate lock-in. +1 defend on D4."

## FORCED-ENGAGEMENT

Stakes at 0: A1, B1, B5, A4 (Chvátil).

**Knizia (A1):** HOLD-EXPLICIT. "The 3-tier state machine adds complexity; whether it achieves rule-count/depth elegance depends on whether it's ever exercised. Hold."
**Rosenberg (B1):** LAST-CALL. "The gear chain from retire-explicit to de-adoption review. Final round."
**Chvátil (B5):** HOLD-EXPLICIT. "Dormancy-watch is a novel protocol mechanism, but the novelty doesn't make the protocol earn B5 — it's a meta-level claim. Hold."
**Chvátil (A4):** LAST-CALL. "8-game/3-game window calibration = managed variance."

## Final moment

**Rosenberg (B1 LAST-CALL):** "The 4-gear chain (retire-explicit → 2.0R → dormancy-watch → targeted-earn → de-adoption) is documented and deterministic. Each step is a rule-trigger, not a judgment call. +1 defend on B1."

**Chvátil (A4 LAST-CALL):** "8 games is long enough for diverse game-type coverage; 3 games is short enough to resolve quickly. The numbers are defensible. +1 defend on A4."

## Required amendment before adoption

### A-spec-2.22-01 — Clarify the 8-game dormancy qualifier measurement

The spec states "≥ 8 games since its last earn" but does not specify: 8 games total across the whole corpus, or 8 games where the axis was drafted? A narrow axis drafted only twice in 8 games could easily pass 8 games without a earn through pure non-drafting.

Amendment: clarify that "8 games" means 8 games **where the axis was drafted** (not 8 games total). An axis that is never drafted cannot have a dormancy-watch triggered — only axes that are being actively argued in the panel but failing to earn count toward the window.

This prevents axes from entering dormancy-watch through neglect (never drafted) vs. genuine argument failure (drafted but not earning).

## Earned stakes (7)

| Persona | Axis | Status |
|---|---|---|
| **Knizia (anchor)** | **C1 Tension Budget** | Earned canonical (tension clock for narrow axes) |
| Lacerda | B4 Info-Transparency-Cost | Earned (fully transparent state documentation) |
| Stegmaier | A6 Teachability | Earned (§Problem distinction is teachable) |
| K-K | C7 Action-Menu Clarity | Earned (OP; procedure is clear at each branch) |
| Feld | B2 Catastrophe Pressure | Earned (OP; de-adoption risk adequately guarded) |
| Rosenberg | D4 Late-Game Lock-in | Earned (de-adoption review result is permanent) |
| Feld | A2 Decision Density | Earned (dormancy-watch creates targeted factory decisions) |

Also earning LAST-CALL: B1 (Rosenberg), A4 (Chvátil) = 9 total earned.

## Retire-explicits (10): C2, C3, A3, A7, B3, D2, D3, C8, C6, A5, Vaccarino's A4 (voided).

## Silent-retire: 0.

## Success criteria

| # | Criterion | Result |
|---|---|---|
| 1 | ≥ 5 earned | PASS (9) |
| 2 | ≥ 2 on Band B/C | PASS (B4, B2, B1, C1, C7) |
| 3 | ≥ 1 collision | PASS (B2↔C7 OP 6-1) |
| 4 | Spec-amendment | **PASS — A-spec-2.22-01 (8-game qualifier clarification)** |

## Verdict

**Spec approved for v2.22 adoption after A-spec-2.22-01 applied.** The dormancy-watch mechanism correctly resolves the I-pandemic-04 protocol gap by distinguishing scope-limited axes from genuinely-refuted axes. The immediate application is precise: only C2 enters dormancy-watch; B5 and C4 are monitoring-only because they earned recently. The de-adoption review procedure preserves the adversarial-review ethos for any axis that exhausts its targeted-earn window.
