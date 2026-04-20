---
name: Famiglia — Tier-C Amendments
slug: famiglia-tierC-amendments
game: 0009-famiglia
stage: tierC
version: 1.0.0
rubric_version: v2.9
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# Tier-C Amendment — Famiglia (game #9)

## Headline

**1 new adoption** (A3 Interaction). Pool reaches **19 of 25 (76%)**. Vaccarino's first published-anchor validates B3 canonically. Heavy retire-explicit session (8; highest in TIGRIS history) but 0 silent-retire (8 consecutive). A1 Elegance earns first time in 9 games.

## Adoption: A3 Interaction

- **Prior earning**: TtA (Chvátil first earn).
- **UNFOLD**: weak-defend only (counted as partial earn in that session's classification, but conservatively counted as 1 defend in this pass).
- **Famiglia**: 1 defend (Chvátil) — second earning.

Cumulative: TtA earned + Famiglia earned = 2 games earned → **ADOPTED (19th axis)**.

Pool status: **19 adopted / 25 (76%)**.

## Queued-for-adoption (new first-earnings)

- **A1 Elegance** (Knizia) — first earning in 9 games. Friese-signature Famiglia validated it. Queued.
- **A5 Downtime-Pacing** (Vaccarino, LAST-CALL) — first earning. Queued.

## Retire-explicits (heaviest session)

8 adopted axes retire-explicit on diagnostic-low stakes: C6, B6, B5, B2, C2, D2, D3, D1 all at +0.5 refute. All adopted status preserved (single +0.5 refute doesn't un-adopt).

This is the **rubric working as designed**: a light 2p game with narrow mechanics correctly scores low on many adopted axes without triggering retirement. Diagnostic integrity confirmed.

## Rubric version: v2.8 → v2.9

Minor bump for 1 adoption.

## Innovations

### I-famiglia-01 — First Vaccarino-on-published-game anchor

- **Trigger:** `vaccarino_published_anchor`
- **Source:** Famiglia is Vaccarino's first anchor on a non-original published game (after UNFOLD original).
- **Status:** adopted (log entry)
- **Note:** Vaccarino's B3 axis validates canonically on a Friese game. Completes another designer-on-published-game; 6 of 8 personas now have anchored.

### I-famiglia-02 — D3 Count-Robustness behavior on 2p-only games

- **Trigger:** `d3_count_robustness_on_single_count_game`
- **Source:** Famiglia is 2p-only; Stegmaier retire-explicits D3.
- **Status:** observational
- **Note:** D3 axis is meaningfully zero on a game with only one supported player count. Retire-explicit handles correctly. No protocol change needed; just a pattern to watch.

### I-famiglia-03 — First A1 Elegance earning in 9 games

- **Trigger:** `dormant_axis_first_earning`
- **Source:** Friese's signature compression on Famiglia.
- **Status:** observational
- **Note:** A1 Elegance was drafted multiple times across 8 games but never earned. Famiglia's Friese-class compression validates it. Pattern: some axes wait for their canonical case.

## Success criteria

All PASS.

## Designer-on-own-game pattern

6 of 8 personas now have anchored a review:
- Knizia on T&E ✓
- Feld on FACETS (original) ✓
- Stegmaier on Scythe ✓
- Chvátil on UNFOLD + TtA ✓
- **Vaccarino on UNFOLD-adjacent + Famiglia (closest to Vaccarino lens, proxied) ✓ (first published)**

Still pending: Rosenberg, Lacerda, K-K.

## State after 9 games

- **19 adopted axes / 25 (76%)**
- 1 retired (C5)
- 3 salvages (C4, C3, B2)
- 5 protocol amendments
- 8 games 0% silent-retire
- 9 games total, 73+ earned stakes, mean 8.1/game
- 4 queued-for-adoption: A1, A5, C8 (queued post-Scythe), D3 (UNFOLD queued)

## Verdict

Famiglia proves the rubric handles light, narrow-scope games cleanly. 8 retire-explicits are a healthy signal — the game genuinely doesn't fit many adopted axes, and the panel reports honestly.

Proceeding to HANDOFF.
