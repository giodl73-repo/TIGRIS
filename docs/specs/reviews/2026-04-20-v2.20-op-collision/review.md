---
name: v2.20 OP-Collision Formalization — Parliament Review
slug: v2.20-op-collision-review
stage: panel
version: 1.0.0
rubric_version: v2.19
bet_version: parliament
artifact_under_review: docs/specs/2026-04-20-v2.20-op-collision-formalization.md
personas_seated: [knizia, rosenberg, feld, lacerda, chvatil, kramer-kiesling, stegmaier, vaccarino]
review_type: meta-review (TIGRIS-internal protocol amendment)
anchor_persona: lacerda
anchor_axis: B4
author: TIGRIS
created: 2026-04-20
updated: 2026-04-20
---

# v2.20 OP-Collision Formalization — Parliament Review

Spec-review of `docs/specs/2026-04-20-v2.20-op-collision-formalization.md`. 8 designers, no lenses (meta-review). Anchor: Lacerda on B4 Information-Transparency-Cost — the core question is whether "motivated rationale" is operationally clear enough to distinguish OP from CR in live argument.

## Drafts

Draft order Round 1: Knizia → Rosenberg → Feld → Chvátil → K-K → Stegmaier → Vaccarino → Lacerda (anchor last).

### Knizia
1. **A1 Elegance.** 7/10. "OP adds one new outcome type and one vote threshold. 2 additions for 5 instances of evidence. Earn-leaning — compact machinery for a real pattern."
2. **C1 Tension Budget.** 4/10. "OP redistributes tension: CR's loser supplies refutation-pressure; OP has no loser. Lower aggregate tension per OP outcome. Hold-leaning — could earn or retire depending on how common OP becomes."
3. **A5 Downtime/Pacing.** 2/10. "OP vote adds a procedural step between the collision trigger and resolution. Timing cost acknowledged but marginal. Retire-explicit."

### Rosenberg
1. **C4 Engine-Garden Dependency.** 7/10. "OP protects C4 from inappropriate CR losses when C4 fires alongside interaction axes on the same mechanism. Historically: A3↔C4 at Wingspan was a majority OP. +1 earn."
2. **B1 System Gearing.** 7/10. "OP defend marks count identically to CR earns in the adoption ledger. Correct gearing — the ledger measures axis relevance, not argument route. Earn."
3. **D4 Late-Game Lock-in.** 3/10. "OP's interaction with FORCED-ENGAGEMENT (what if a 0-mark stake proposes OP during FE micro-phase?) is unaddressed. Hold — spec gap surfaced."

### Feld
1. **B2 Catastrophe Pressure.** 6/10. "§Non-goals addresses OP-as-avoidance. However, the 'motivated-rationale' adequacy check is undelegated — spec doesn't say who adjudicates whether a rationale is sufficient. Latent abuse vector. Earn-contested."
2. **C6 Point-Salad Incommensurability.** 4/10. "OP-earn vs. CR-earn: are these the same? Spec says yes (§Ledger treatment). But a persona who earned via OP may have avoided the argument test CR imposes. Hold — could go either way."
3. **C8 First-Turn Compression.** 5/10. "If OP is known to be available, personas may draft adjacent axes strategically in Round 1 to engineer OP outcomes. §Non-goals says 'motivated rationale blocks this' — but the incentive exists at draft time, before argument. Earn-leaning."

### Chvátil
1. **A3 Interaction.** 5/10. "OP creates a cooperation escape: at a collision, both personas can jointly preserve their stakes rather than one winning. This softens the adversarial character of Parliament. §Non-goals addresses abuse but not the structural softening. Contest."
2. **B5 Architectural Novelty.** 7/10. "No published adversarial-review system uses OP as a named collision outcome. TIGRIS invents it from 5 instances of its own evidence. Earn."
3. **C3 Scarcity Bite.** 2/10. "Adjacent-axis drafting slots are implicitly scarce. OP slightly changes the scarcity economy — pairing two preferred axes for OP becomes a draft strategy. Retire-explicit — too speculative."

### Kramer-Kiesling
1. **C7 Action-Menu Clarity.** 7/10. "OP proposal procedure: persona proposes → motivated sentence → panel votes once → CR if < majority for OP. Clear 4-step decision tree. Earn."
2. **D1 Family-to-Expert Scaling.** 5/10. "OP requires recognizing 'different analytical registers' — expert panels will apply this naturally; novice panels may not know where the register boundary is. Asymmetry concern. Hold."
3. **D2 Spatial-Interaction Presence.** 1/10. "N/A. Retire-explicit."

### Stegmaier
1. **A6 Teachability.** 7/10. "Spec's 3 worked examples (B5↔A7, A3↔C4, A6↔C6) cover different register-pair types: structural vs. experiential; mechanism vs. social; temporal complexity stages. 3 examples is the right teaching surface. Earn."
2. **D3 Count-Robustness.** 6/10. "Spec includes 'majority of designers present' fallback for reduced panels. Covers 5-7 seated personas cleanly. Earn."
3. **C2 Minimum-Score Shape.** 1/10. "N/A. Retire-explicit."

### Vaccarino
1. **A4 Variance Calibration.** 7/10. "5-of-8 threshold for standard OP; 7-of-8 for Decisive OP. Two-tier threshold calibrates well: majority catches genuine orthogonality; near-unanimous flags canonical cases. Earn."
2. **A2 Decision Density.** 6/10. "OP introduces one new meaningful decision in every collision: propose OP or accept CR. This decision requires the proposer to assess register-independence — non-trivial. Earn."
3. **B6 Scoring Multiplier Dependency.** 5/10. "Decisive OP double-counts toward success criterion §3. This is a multiplier. Well-contained (only at 7-0; rare), but could in theory substitute for a genuine CR collision. Hold-leaning concern."

### Lacerda (anchor)
1. **B4 Information-Transparency-Cost (anchor).** **8/10.** "The motivated-rationale requirement is the spec's core transparency gate. '§Non-goals: bare "both are valid" is not sufficient and must be ruled CR by default' — but the spec doesn't specify who rules. The panel? A chairperson? The proposing persona can't self-certify adequacy. This is a transparency gap: the adjudication step is invisible. Earn-contested pending clarification."
2. **A7 Emergence-Replayability.** 6/10. "OP enables more varied argument records across games: some games will have CR-dominant argument, others OP-dominant, creating distinct session shapes. Good for emergence. Earn."
3. **B3 Conversion Chain Depth.** 4/10. "OP 3-step pipeline: draft adjacent axes → collide → vote OP → both earn. Clean, but the chain is short. Hold."

## GATE check

1. **Anchor stake viable?** Yes — Lacerda B4 on motivated-rationale transparency is the load-bearing question.
2. **Draft coverage ≥ 3 bands?** A(A1,A3,A4,A5,A6,A7,A2), B(B1,B2,B3,B4,B5,B6), C(C1,C2,C3,C4,C6,C7,C8), D(D1,D2,D3,D4). **All 4 bands covered. Pass.**
3. **≥ 1 collision candidate?** A3 (adversarial-escape concern) vs. A6 (teachability of OP distinction) — adjacent in the spec's own worked examples. **Pass.**

GATE pass. Proceeding to ARGUMENT.

## Argument

### §Problem — corpus sufficiency (Moments 1-2: §Problem table, §5-instance count)

**Lacerda (B4 anchor):** "The §Problem table lists 'majority' for 4 of 5 instances without specifying vote counts. The §OP vote threshold section later defines majority as ≥5-of-8, but a reader of the table can't verify retroactively. The §Problem should at least add a footnote: 'majority = ≥5 of 8 unless otherwise noted.' Partial earn on B4 — core transparency concern is real."

**Knizia (A1):** "5-instance table is compact and informative. The cross-reference to §vote threshold is implicit but findable. The spec prioritizes concision over redundancy — acceptable. +1 defend on A1."

**Chvátil (A3):** "The 5 instances span games #16–#21. All 5 involve one 'mechanism' axis and one 'quality or dependency' axis — A7, C4, A6, C6, B5, A3. The pattern is real, but §Problem doesn't name the register types that make them orthogonal. Readers are left to infer the pattern from examples. Hold on A3 — earn depends on §Definitions section."

**Rosenberg (C4):** "The Wingspan A3↔C4 instance in the table correctly identifies multiplayer-solitaire (A3) and solo-engine-architecture (C4) as the respective charges. These are genuinely different questions about the same mechanism. The table validates C4's position. +1 defend on C4."

### §Definitions and §Vote threshold (Moments 3-4)

**Lacerda (B4):** "§OP vote threshold: 'A bare "both are valid" is not sufficient and must be ruled CR by default.' Who rules? The proposing persona can't self-certify. The panel can vote (but that's circular — the OP vote IS the ruling). The spec omits a moderator role or panel-chair authority. This is the core B4 gap. Sustained B4 concern — earns once gap is acknowledged."

**Feld (B2) attacks Lacerda B4:** "The moderator gap isn't just B4's problem — it's a catastrophe surface. An under-specified adjudication mechanism means the 'default to CR' rule is unenforceable in practice. If the proposing persona insists their rationale is adequate and the panel can't call it insufficient without another vote, the protection collapses. +1 refute-direction on B2."

**Knizia (A1) defends against B2 attack:** "Panel consensus is the de-facto moderator in the Parliament procedure generally. No other collision resolution specifies a chairperson either — the panel votes and the majority outcome stands. The OP case is no different. 'Default to CR' is enforced by the same mechanism as all other votes. +1 defend on A1."

**Stegmaier (A6):** "The 3 worked examples serve as the operationalization of 'motivated rationale.' A persona applying OP needs to match their rationale to the register-pair types demonstrated in the examples. This is teachable — the examples ARE the bar. +1 defend on A6."

**Vaccarino (A4):** "5-of-8 vs. 7-of-8 threshold is correctly tiered. The gap between majority and near-unanimous preserves a middle range where genuine orthogonality is recognized but doesn't generate double-credit. +1 defend on A4."

**Chvátil (A3):** "§Definitions: CR = one earns, one takes refutation. OP = both earn. This creates an asymmetry: a persona whose axis is in contested territory (accumulated partial-refute) benefits disproportionately from OP because it avoids the refutation. The mechanism isn't neutral — it advantages personas with under-pressure axes. Earnest concern. +1 contest on A3."

**K-K (C7):** "§Argument file notation gives a specific 6-line template for OP-collision record. This is precise action-menu design. +1 defend on C7."

### COLLISION — A3 (Chvátil) vs A6 (Stegmaier)

**Chvátil (A3):** "OP softens adversarial character structurally. The Parliament procedure's bet is that disagreement produces better reviews than consensus. OP lets two personas exit a collision without a loser. Over time, if OP becomes common, the argument record fills with mutual-preservation events rather than contested wins and losses. The adversarial signal degrades."

**Stegmaier (A6):** "OP doesn't reduce adversarial argument — it routes genuinely different claims to a different resolution. CR still governs same-register collisions. The adversarial character is preserved wherever axes genuinely compete. The teachability of the distinction ('same phenomenon vs. different registers') is what prevents OP from becoming a general escape hatch. Where the worked examples clearly show two different questions, novice and expert alike can distinguish. The adversarial signal is not degraded — it's supplemented with a new signal: 'these axes are asking different questions.'"

**Vote:** Rosenberg → Stegmaier (different registers is demonstrably real at Wingspan and Nemesis); Lacerda → Stegmaier (B4 transparency concern is about moderation, not adversarial character); Feld → Chvátil (B2 concern echoes the structural softening); K-K → Chvátil (D1 novice-asymmetry concern reinforces Chvátil); Vaccarino → Stegmaier (A4 threshold calibration gates against OP overuse); Knizia → Stegmaier (A1 elegance: one new mechanism that resolves real cases is better than forcing CR where it doesn't fit).

**Result: 4-2 Stegmaier. A6 earns. A3 takes a collision refutation.**

### §Ledger treatment and §Success criterion interaction (Moments 5-6)

**Rosenberg (B1):** "OP defend marks count identically to CR earns. Correct gearing — the adoption ledger measures axis relevance, not how the earn was generated. An axis that earns via OP is still demonstrating applicability to a specific moment. +1 defend on B1."

**Feld (C6):** "OP-earn vs. CR-earn: are they equivalent? §Ledger treatment says yes. But CR-earn requires winning an adversarial test — another persona argued your axis was wrong and the panel agreed it was right. OP-earn requires only that both axes are 'simultaneously valid.' These are epistemically different. The spec should acknowledge this asymmetry, even if it treats them identically in the ledger." Attack on B1.

**Rosenberg (B1) defends:** "C6 is correct that the epistemic status differs, but the ledger's purpose is adoption tracking, not epistemology ranking. An axis earns because it applied to a real moment — the route doesn't change that application. If we start weighting earn-type, we need a point-salad: CR-earn = 1.2, OP-earn = 0.8, LAST-CALL-earn = 1.5. That's the incommensurability problem. +1 defend on B1."

**Vaccarino (B6):** "Decisive OP double-counts toward success criterion §3. This is a contained multiplier — requires ≥7-of-8 unanimity. Rare enough to be a genuine signal. +1 defend on B6." 

**K-K (D1) attacks B6:** "Decisive OP double-counting could allow a game to satisfy criterion §3 with only one collision. §3 says '≥1 clean collision resolution.' If a Decisive OP counts as 2, you need 0.5 collisions to satisfy the criterion. That's wrong. Criterion §3's purpose is ensuring live argument happens — a single unanimous OP, however decisive, doesn't demonstrate that argument is alive."

**Vote on K-K's concern:** Knizia → K-K (A1 concern: double-credit stretches the elegance); Feld → K-K (B2 concern: this is a soft failure mode); Rosenberg → Vaccarino (rare; self-correcting); Chvátil → K-K (adversarial character concern reinforced); Stegmaier → K-K (A6: teachability of §3 is harmed if criteria can be satisfied in unintuitive ways); Lacerda → K-K (B4: transparency requires criterion text that reads correctly at face value).

**Result: 5-1 for K-K. B6 soft-refuted.** Decisive OP double-counting needs amendment — criterion §3 must still require at least one standard (non-Decisive-OP) collision.

### FORCED-ENGAGEMENT (pre-final)

Stakes at 0 marks at this point: C1 (Knizia), D4 (Rosenberg), C8 (Feld), B5 (Chvátil), D1 (K-K), D3 (Stegmaier), A2 (Vaccarino), B3 (Lacerda). Plus: A5 (Knizia), C3 (Chvátil), D2 (K-K), C2 (Stegmaier), C6 (Feld).

**Knizia (C1):** HOLD-EXPLICIT. "C1 tension-budget concern is real but not evidenced in the spec text — the spec doesn't model aggregate OP frequency. Holding without evidence."
**Knizia (A5):** RETIRE-EXPLICIT. "Pacing cost of OP vote is marginal and outside spec scope."
**Rosenberg (D4):** RETIRE-EXPLICIT. "FORCED-ENGAGEMENT ↔ OP interaction is a real gap, but it's outside v2.20's scope. Retire-explicit with annotation: gap exists."
**Feld (C8):** LAST-CALL. "Strategic adjacent-axis drafting to engineer OP outcomes — this is the §Non-goals 'abuse' scenario played forward. Final round: does the spec prevent it?"
**Feld (C6):** HOLD-EXPLICIT. "OP-earn vs. CR-earn epistemic asymmetry is real but spec correctly defers it (adoption ledger ignores earn-route). Hold."
**Chvátil (B5):** LAST-CALL. "OP as architectural novelty in adversarial-review systems — no prior art. Final-round earn attempt."
**Chvátil (C3):** RETIRE-EXPLICIT. "Scarcity of adjacent-axis pairings too speculative."
**K-K (D1):** Earned via attack on B6 above — no FE needed.
**K-K (D2):** RETIRE-EXPLICIT. "N/A."
**Stegmaier (D3):** LAST-CALL. "Robustness at reduced panel sizes — 'majority of designers present' covers this."
**Vaccarino (A2):** "OP creates a new decision at every collision: propose or accept CR. §Definitions names this decision implicitly. +1 defend on A2."
**Lacerda (B3):** HOLD-EXPLICIT. "The 3-step pipeline (draft adjacent → collide → vote OP) is real but short. The spec doesn't require a multi-step chain."

### Final moment

**Feld (C8, LAST-CALL):** "§Non-goals: 'OP only applies to collisions (axes that fire simultaneously on the same moment). Non-adjacent axes cannot produce an OP outcome.' This directly addresses strategic drafting — personas can only engineer OP if their axes are adjacent enough to collide in the first place. Adjacency is determined by the moment, not the draft. The §Non-goals clause closes the strategic-drafting abuse. +1 defend on C8."

**Chvátil (B5, LAST-CALL):** "B5 architectural novelty: TIGRIS has reviewed 21 games. The OP pattern emerged from game #16 (AFoS) — 5 instances over 6 games is reliable evidence of a real phenomenon, not artifact. The spec names a genuinely novel outcome type that no other review system has codified from first-principles evidence. +1 defend on B5."

**Stegmaier (D3, LAST-CALL):** "§Vote threshold: 'majority of present designers' fallback for reduced panels. Clear, no ambiguity at 5-7 personas present. +1 defend on D3."

## Earned stakes (11)

| Persona | Axis | Band | Status |
|---|---|---|---|
| **Lacerda (anchor)** | **B4 Info-Transparency-Cost** | B | Earned (motivated-rationale gap correctly identified; transparency concern acknowledged) |
| Lacerda | A7 Emergence-Replayability | A | Earned (OP enables varied argument shapes across games) |
| Knizia | A1 Elegance | A | Earned (compact mechanism; 2 additions for 5 instances) |
| Rosenberg | C4 Engine-Garden Dependency | C | Earned (OP correctly preserves C4 against A3-adjacent CR losses) |
| Rosenberg | B1 System Gearing | B | Earned (OP defend marks correctly gear into adoption ledger) |
| Feld | C8 First-Turn Compression | C | Earned (LAST-CALL; §Non-goals adjacency clause closes strategic-drafting concern) |
| Chvátil | B5 Architectural Novelty | B | Earned (LAST-CALL; no prior art for named OP in adversarial-review systems) |
| Stegmaier | A6 Teachability | A | Earned (collision winner; 3 worked examples cover the teaching surface) |
| Stegmaier | D3 Count-Robustness | D | Earned (LAST-CALL; reduced-panel fallback specified) |
| Vaccarino | A4 Variance Calibration | A | Earned (5-of-8 / 7-of-8 two-tier threshold well-calibrated) |
| Vaccarino | A2 Decision Density | A | Earned (OP proposal creates a meaningful new decision at every collision) |

## Refuted stakes (1) + Soft-refuted (1)

| Persona | Axis | Status |
|---|---|---|
| Chvátil | A3 Interaction | Refuted (collision loss 4-2 vs. A6; adversarial-escape concern overridden by teachability argument) |
| Vaccarino | B6 Scoring Multiplier Dependency | Soft-refuted (5-1 panel; Decisive OP double-counting toward criterion §3 needs amendment) |

## Hold-explicit stakes (3)

| Persona | Axis | Reason |
|---|---|---|
| Knizia | C1 Tension Budget | No frequency data; cannot model OP's aggregate tension effect |
| Feld | C6 Point-Salad Incommensurability | Real epistemic asymmetry but spec defers correctly; no ledger change |
| Lacerda | B3 Conversion Chain Depth | 3-step chain is real but short; no argument |

## Retire-explicit stakes (8)

Knizia A5; Rosenberg D4 (with gap annotation); Chvátil C3; K-K D2; Stegmaier C2; K-K D1 (earned via attack path); Feld B2 (earns via contest, not retire — see below); Vaccarino B6 (soft-refuted above).

Actually: **Feld B2 earns** — the B2 catastrophe concern (motivated-rationale adjudication gap) is a genuine spec gap that earned its argument, even though the amendment addresses it. B2 = Earned (contested).

Retire-explicit count: 7 (A5, D4, C3, D2, C2, D2, C6 hold-not-retire).

## Silent-retire: 0

## Required amendment before adoption

### A-spec-2.20-01 — Clarify Decisive OP and criterion §3

§Success criterion interaction currently states: "A Decisive OP (≥ 7-0) counts as **2** collision resolutions toward this criterion."

Amendment: add the following constraint:

> **Criterion §3 floor**: Decisive OP double-credit does not substitute for a standard collision. A game that produces only Decisive OP events (and no CR or standard OP collisions) does not satisfy criterion §3. At least one CR or standard-OP collision must occur independently of any Decisive OP. The Decisive OP double-credit supplements this baseline; it does not replace it.

This preserves the double-credit incentive for canonical OP recognition while ensuring that criterion §3 cannot be satisfied by unanimous preservation alone.

### A-spec-2.20-02 — Add moderator note to §Vote threshold

Add one sentence after "A bare 'both are valid' is not sufficient and must be ruled CR by default":

> The panel collectively adjudicates whether a rationale names orthogonal registers; the proposing persona cannot self-certify. If the panel disagrees on adequacy, a simple majority vote resolves: majority for adequate → OP vote proceeds; majority for inadequate → collision resolves immediately as CR.

This closes Lacerda's B4 transparency gap by making the adjudication mechanism explicit.

## Success criteria

| # | Criterion | Result |
|---|---|---|
| 1 | ≥ 5 earned | PASS (11 earned) |
| 2 | ≥ 2 on Band B or C | PASS (B4, B1, B5, C4, C8 earned) |
| 3 | ≥ 1 clean collision | PASS (A3↔A6 CR 4-2; Stegmaier wins) |
| 4 | Spec-amendment | **PASS — 2 required spec amendments (A-spec-2.20-01, A-spec-2.20-02)** |

## Verdict

**Spec approved for v2.20 adoption after A-spec-2.20-01 and A-spec-2.20-02 applied.**

Core protocol (OP as named collision outcome; 5-of-8 / 7-of-8 thresholds; motivated-rationale gate; identical ledger treatment to CR earns) is sound. The 5-instance corpus is sufficient evidence. The worked examples are well-chosen. The non-goals section correctly pre-empts the main abuse scenarios.

Two amendments required: Decisive OP cannot satisfy criterion §3 as sole collision (closes the double-credit loop K-K identified), and the panel-adjudicates-motivation rule must be stated explicitly (closes Lacerda's B4 moderator gap).

Recommended action: apply A-spec-2.20-01 and A-spec-2.20-02 to the spec, bump spec to v1.1, then formally adopt v2.20 and update `personas/playtest-rubric.md`.
