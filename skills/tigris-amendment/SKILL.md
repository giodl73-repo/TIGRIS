---
name: tigris-amendment
description: Run the deterministic TIER-C amendment pass for a completed game. Reads tierA-stakes.md + all playtests/PT*-argument.md for the game, classifies each stake (earned/refuted/contested/hold-explicit/retire-explicit/silent-retire), updates personas/axis-pool.md ledger, checks adoption/retirement thresholds, writes amendments.md, bumps rubric version if triggered. Pure rules-application; no creative generation.
---

# tigris-amendment — deterministic TIER-C pass

## 60-second quickstart

```
You:    "Run tigris-amendment on games/0003-facets/"
Claude: [reads tierA-stakes.md + all playtests/PT*-argument.md]
        [classifies each drafted stake by v2.1 state rules]
        [reads personas/axis-pool.md ledger]
        [cumulates per-axis counts across games]
        [checks adoption triggers: >=2 earned across >=2 games]
        [checks retirement triggers: >=2 refuted across >=2 games]
        [writes games/0003-facets/amendments.md]
        [updates personas/axis-pool.md ledger + changelog]
        [updates personas/playtest-innovations.md if new candidates surfaced]
        [bumps rubric_version if adoption/retirement triggered]
        [reports: "FACETS amendment: 2 axes adopted (B1, B2), 0 retired.
         Rubric v2.2 -> v2.3. 3 new innovations logged."]
```

## When to invoke

- After TIER-B ARGUMENT is complete and `playtests/PT*-argument.md` files are written.
- Before HANDOFF — amendment produces the evidence HANDOFF summarizes.
- Do NOT invoke on a game mid-argument; requires complete argument record.

## Preconditions

- `games/NNNN-<slug>/tierA-stakes.md` exists with all 21 drafted stakes.
- `games/NNNN-<slug>/playtests/PT<NN>-argument.md` exists with complete argument record (all moment-anchors resolved, FORCED-ENGAGEMENT phase complete per v2.1).
- `personas/axis-pool.md` exists with current Rubric Ledger.
- `personas/playtest-rubric.md` exists at current rubric_version.

## Procedure

### Step 1 — Load context

Read in order:
1. `games/<slug>/tierA-stakes.md` — the 21 drafted stakes with scores.
2. All `games/<slug>/playtests/PT*-argument.md` — attacks, defenses, collisions, FORCED-ENGAGEMENT resolutions per persona.
3. `personas/axis-pool.md` — current ledger.
4. `personas/playtest-rubric.md` — stake-state definitions and thresholds (currently v2.1).
5. `personas/playtest-innovations.md` — for logging new innovations.

### Step 2 — Classify each stake (per v2.1 rules)

For each of the 21 drafted stakes, determine its final state:

```
for each (persona, axis) pair in tierA-stakes:
    d = defend_marks in argument
    r = refute_marks in argument
    cw = collision_wins
    cl = collision_losses
    fe = FORCED-ENGAGEMENT declaration (hold-explicit / retire-explicit / last-call-stake / none)

    net_marks = d + cw

    if fe == "retire-explicit":
        state = "retire-explicit"
        refute_weight = 0.5
    elif fe == "hold-explicit" and net_marks == 0 and r == 0:
        state = "hold-explicit"
        (no amendment contribution)
    elif fe == "last-call-stake":
        # last-call outcome resolves in final round
        if last_call_defended: state = "earned", +1 defend
        elif last_call_attacked_and_failed: state = "refuted", +1 refute
        else (no attack, natural pass): state = "earned" (implicit defense), +1 defend
    elif net_marks >= 2 and r == 0:
        state = "earned"
    elif net_marks == 1 and cl >= 1:
        state = "contested"
    elif r >= 2 and d == 0:
        state = "refuted"
    elif d >= 1 and r >= 1:
        state = "contested"
    elif net_marks == 0 and r == 0 and fe == None:
        state = "silent-retire"
        refute_weight = 1.0 (full retirement contribution)
        persona_retirement_flag = True
    else:
        state = "weak-defended" (edge case; 1 mark with no refute)
```

Silent-retire detection is a v2.1 feature — should be 0 in most games if FORCED-ENGAGEMENT is followed correctly.

### Step 3 — Update the Rubric Ledger

For each axis, find its row in `personas/axis-pool.md` and update cumulative counts:

- Earned state: +1 to `Earned` column.
- Refuted state: +1 to `Refuted` column.
- Contested state: +1 to `Contested` column.
- Retire-explicit state: +0.5 to `Refuted` (half weight).
- Silent-retire state: +1 to `Refuted` (full weight).
- Hold-explicit / weak-defended: no ledger change.

### Step 4 — Check adoption triggers

For each axis currently in `live` or `queued-for-adoption` status:

- If `earned count across >= 2 games >= 2`, mark as **adopted**.
- Update status column in ledger.
- Add to "adopted axes this cycle" list for the amendment report.

### Step 5 — Check retirement triggers

For each axis currently in `live` or `queued-for-retirement` status:

- If `refuted count (including 0.5 weights summed) across >= 2 games >= 2`, mark as **retired**.
- Update status column in ledger.
- Add to "retired axes this cycle" list.

### Step 6 — Bump rubric version

- If ≥1 axis adopted OR ≥1 axis retired: bump `rubric_version` by 0.1 (e.g., v2.2 → v2.3).
- If no adoption/retirement: bump patch version (e.g., v2.2 → v2.2.1) to reflect ledger update.
- Per v2.1 rate limit: "At most one rubric amendment adopted per 2-game cycle." **Adoption events count toward rate limit.** If rate limit is exhausted this cycle, defer to next cycle — mark axis as "adopted pending next-cycle confirmation" instead.

Also update the rubric_version field in `personas/playtest-rubric.md` changelog and `personas/axis-pool.md` changelog.

### Step 7 — Detect new innovations

Scan the argument record for patterns not currently logged:

- **New adjacency candidates** (two stakes fired simultaneously on the same moment, not on the current adjacency chart) → trigger_pattern `new_adjacency_surfaced`.
- **Persona-retirement flags** (any persona with ≥3 silent-retires this game) → trigger_pattern `persona_retirement_candidate`.
- **LAST-CALL abuse patterns** (≥4 LAST-CALLs in one game by one persona) → trigger_pattern `last_call_abuse`.
- **Bet-validation moments** (architectural-novelty stakes earning; self-refutation of prior stakes) → trigger_pattern `novel_architecture_defended` or `self_refutation_by_evidence`.
- **Any pattern matching a currently-candidate amendment in `personas/playtest-innovations.md`** → update that amendment's cluster count.

Log each new innovation as a candidate entry in `personas/playtest-innovations.md`.

### Step 8 — Write amendments.md

```yaml
---
name: <Title> — Tier-C Amendments
slug: <slug>-tierC-amendments
game: NNNN-<slug>
stage: tierC
version: 1.0.0
rubric_version: <new version after bump>
bet_version: parliament
author: TIGRIS
created: <YYYY-MM-DD>
---

# Tier-C Amendment — <Title> (game #N)

## Reviewer-stake ledger update (game #N)
<Table: axis | state | marks | cumulative across games>

## Silent-retire count
<Should be 0 under v2.1 if FORCED-ENGAGEMENT was followed.>

## Promotion / retirement triggered
### ADOPTED (this session)
<list with justification>
### RETIRED (this session)
<list with justification, or "none"; retirement-protocol note>
### Queued-for-adoption
<...>
### Queued-for-retirement
<...>

## Rubric version bump
<v2.X → v2.X+1 reasoning>

## Innovations surfaced
<Any new I-<slug>-NN candidates>

## Success criteria check
<Pass/fail per the 4 criteria>

## Next step
HANDOFF.
```

### Step 9 — Update ledger and innovations files

- `personas/axis-pool.md`: update the Rubric Ledger table rows and the Changelog section.
- `personas/playtest-innovations.md`: append new innovations; update cluster status for existing candidates.

### Step 10 — Report

Report to user:
- Path to amendments.md.
- Adopted axes (if any), retired axes (if any).
- Silent-retire count (validates v2.1 if 0).
- Rubric version before and after.
- Success-criteria outcome.
- Innovations logged or cluster-promoted.

## Invariants

- This skill is **deterministic**. Given the same tier-A stakes and argument record, the same output must result. No creative generation allowed.
- Forward-only versioning: adoption/retirement applies to the CURRENT game's scoring version. Prior games' stake states are NOT re-classified.
- Rate limit: at most 1 adopted amendment per 2-game cycle. Adoption events count toward this limit.
- Silent-retire is a failure mode for the panel, not for the game. Report the count; don't hide it.
- If the argument record is incomplete (missing moment-anchors, unresolved collisions), abort and return to user for completion.

## Failure modes

- **Argument record incomplete** — return error; request user re-run TIER-B with FORCED-ENGAGEMENT per v2.1.
- **Ledger and innovation log mismatch** — if prior games' earned/refuted counts don't match innovation log amendment history, surface the inconsistency for manual review.
- **Rate limit conflict** — if ≥2 axes would adopt this cycle but rate limit allows 1, defer one by priority (most independent evidence first).
- **Circular dependency** — if an amendment candidate depends on an axis that's being adopted/retired this cycle, flag and defer.

## What this skill does NOT do

- Read arguments that weren't written.
- Generate creative scoring or interpretation — purely rules-application.
- Override forward-only versioning.
- Adopt amendments without rate-limit compliance.
- Retire personas automatically — persona-retirement requires explicit user decision even if flags accumulate.
