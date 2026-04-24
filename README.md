# TIGRIS

**A board game factory.** Named for *Tigris & Euphrates* (Knizia, 1997).

Claude Code-driven. Markdown-first. Euro tradition. TIGRIS reviews published games, designs new ones, and — most unusually — has built up an empirical map of what "design space" actually looks like by doing so at scale.

## What TIGRIS has done

**Reviewed 150 games.** Every review is a Parliament: eight designer personas (Knizia, Rosenberg, Feld, Lacerda, Chvátil, Kramer–Kiesling, Stegmaier, Vaccarino) plant incompatible stakes against the design, then a narrated playthrough attacks, defends, and collides them. 69 games went through the full three-phase pipeline; 81 more went through a faster variant to build out the corpus. [`TRACKER.md`](TRACKER.md) has every row.

**Discovered [TIGER BEAT](docs/tiger-beat.md) — a coordinate system for board games.** Once you have 150 games scored across 28 axes, you can ask: what are the real underlying dimensions? The corpus answers back with nine. Five big ones, four smaller ones. Every game now has a 9-number profile. That means you can plot games, find neighbors, find *gaps*, and design into the gaps on purpose.

**TIGER** — the five primary dimensions (72% of the variance in the corpus):

| | Dim | What it measures | Corpus leader |
|---|---|---|---|
| **T** | Tension | Stakes, pressure, fear of losing | Space Alert, Bridge (7.5–7.2) |
| **I** | Interaction | Direct conflict, hidden info, targeting | Letters from Whitechapel, Bridge (8.7) |
| **G** | Game-spectrum | Deep Architecture ↔ Elegant Minimalism (bipolar) | On Mars (7.8) ↔ Crokinole (0.3) |
| **E** | Experiential | Social contract, physical affordance, cognitive load | Secret Hitler, Hanabi, Codenames (5.7) |
| **R** | Range | Variability, replayability, strategic breadth | Welcome To, Sagrada, Qwixx, Crokinole (8.0) |

**BEAT** — the four secondary dimensions (the remaining 28%):

| | Dim | What it measures | Corpus leader |
|---|---|---|---|
| **B** | Breadth | Solo mode, player-count robustness, multi-mode flexibility | Crokinole (9.0) |
| **EA** | Emotional Arc | Narrative journey vs. pure mechanical replay | Pandemic Legacy S1 (9.0) |
| **A** | Accessibility | Onboarding ease, first-turn footprint, language independence | Pandemic Legacy S1 (9.0) |
| **TX** | Texture | Scarcity bite, pressure shape, floor mechanics | On Mars (7.7) |

The biggest single finding: **G is bipolar.** "Deep engine" and "elegant minimalism" aren't two different things you can score separately — they're opposite ends of the same axis. Lisboa is at 6.5. Crokinole is at 0.3. Nothing sits in the middle; the corpus *bifurcates*.

**Here's what TIGER BEAT looks like for games you know.** Every one of these was scored by TIGRIS through a full Parliament review:

| Game | T | I | G | E | R | Notable BEAT |
|---|---:|---:|---:|---:|---:|---|
| Tigris & Euphrates | 7.2 | 5.7 | 3.5 | 3.0 | 5.7 | — |
| Catan | 3.0 | 3.3 | 2.2 | 4.0 | 6.7 | — |
| Dominion | 4.5 | 5.0 | 4.5 | 2.3 | 6.7 | — |
| Agricola | 5.8 | 4.3 | 5.0 | 3.0 | 5.3 | TX 6.3 |
| 7 Wonders | 4.2 | 4.7 | 3.7 | 3.0 | 6.0 | — |
| Azul | 3.5 | 4.3 | 1.3 | 3.0 | 7.7 | — |
| Wingspan | 4.2 | 4.0 | 3.8 | 2.7 | 6.7 | — |
| Brass: Birmingham | 5.2 | 5.3 | 6.0 | 3.3 | 4.7 | TX 6.7 · A 7.0 |
| Gloomhaven | 6.0 | 6.0 | 4.3 | 4.3 | 5.0 | EA 8.0 |
| Pandemic Legacy S1 | 6.2 | 7.0 | 3.7 | 3.7 | 5.0 | **EA 9.0** · A 9.0 |
| Codenames | 4.8 | 7.3 | 1.2 | 5.7 | 6.7 | — |
| Crokinole | 3.0 | 4.3 | 0.3 | 4.7 | 8.0 | **B 9.0** |
| Through the Ages | 6.8 | 5.3 | 5.3 | 3.3 | 5.3 | TX 6.3 |
| Lisboa | 6.0 | 6.7 | 6.5 | 3.3 | 5.0 | TX 6.3 |

Read those rows. Crokinole and Azul both live at low G, high R — elegant and replayable. Pandemic Legacy is a story game (EA 9.0) that nothing else in the corpus comes near. Brass and Lisboa occupy the deep-architecture corner that Azul deliberately avoids. Codenames' E score of 5.7 makes it one of three games in 150 that really do the social-contract thing.

**Designed 31 original games.** Including [Parliament](games/0001-parliament/) (the factory's self-model — mechanics that *are* the pipeline), [CODEX](games/0151-codex/) (a Schools-of-Magic engine-builder, first game *designed to attack an empty region* of TIGER space), and [RITE](games/0153-rite/) — the first TIGRIS game to land in Gap 1, a region previously empty across 150 games.

**Found [5 design gaps](docs/tiger-beat-gaps.md).** Empty regions where no game sits in the corpus. Each one is a real unsolved problem — with nearest neighbor, a design concept, and five meetup-crowd inspirations to prime a designer's intuition. Two entered this month (Gap 1 via RITE, Gap 5 via CODEX). Three still open.

**Grew the rubric from 24 axes to 32.** The rubric is forward-only: axes earn adoption through repeated Parliament evidence, or they get retired when they consistently fail. 29 adopted, 1 retired (C5 Anti-Catch-up Pressure), 3 live and gathering data, 144 consecutive games with zero silent retirements. Live state: [`personas/axis-pool.md`](personas/axis-pool.md).

## How a review reads

Every evaluative claim in TIGRIS names (a) an axis from the Pool, (b) the persona making the claim, (c) a player count or moment anchor. "Fun," "well-designed," and "solid" are [forbidden words](personas/forbidden-words.md). The factory's job is **well-formed disagreement**, not smoothed consensus.

The shape of a Parliament: eight designers draft 3 axes each from the Pool in snake order. Each plants stakes. A narrated playthrough attacks, defends, and *collides* those stakes on adjacent axes. Then deterministic amendment promotes, retires, or holds axes based on what happened — the rubric evolves from the play record, not by committee.

Open any game's `panel/SUMMARY.md` for a completed review. [`games/0001-parliament/tierA-stakes.md`](games/0001-parliament/tierA-stakes.md) is a clean example of stakes-drafting.

## Where to go next

- **Explore the data.** [`docs/tiger-beat.md`](docs/tiger-beat.md) — the framework. [`docs/tiger-beat-gaps.md`](docs/tiger-beat-gaps.md) — the 5 open design problems. [`data/axis-matrix.csv`](data/axis-matrix.csv) — raw scores, 150 games × 28 axes.
- **Browse the games.** [`games/`](games/) — one directory per game, originals and reviews interleaved. Parliament is 0001; High Society was the 150-game milestone.
- **Understand the axes.** [`personas/axis-pool.md`](personas/axis-pool.md) — all 32 axis definitions plus the Rubric Ledger showing how each got adopted or retired.
- **Meet the personas.** [`personas/designers/`](personas/designers/) (8 designers, each with drafting preferences). [`personas/player-lenses/`](personas/player-lenses/) (5 player perspectives).
- **Current state.** [`TRACKER.md`](TRACKER.md) for per-game history, or the newest file in [`docs/handoff/`](docs/handoff/) for a session resume point.
- **Run the factory.** [`CLAUDE.md`](CLAUDE.md) (house rules), then the skills in [`.claude/skills/`](.claude/skills/): [`tigris-ideate`](.claude/skills/tigris-ideate/SKILL.md), [`tigris-concept`](.claude/skills/tigris-concept/SKILL.md), [`tigris-design`](.claude/skills/tigris-design/SKILL.md), [`tigris-panel`](.claude/skills/tigris-panel/SKILL.md), [`tigris-amendment`](.claude/skills/tigris-amendment/SKILL.md), [`tigris-handoff`](.claude/skills/tigris-handoff/SKILL.md).

## Status

Rubric v2.24.85 · 153 games · 32-axis Pool · latest: [RITE #153](games/0153-rite/) (Gap 1 entered) + [UPSTAGE](parlor/games/0001-upstage/) (PARLOR party-game pipeline launched). Resume point: [`docs/handoff/2026-04-23-post-rite-upstage.md`](docs/handoff/2026-04-23-post-rite-upstage.md). Full architecture spec: [`docs/specs/2026-04-19-tigris-v2.0-design.md`](docs/specs/2026-04-19-tigris-v2.0-design.md).

## Research

Six papers document the TIGER BEAT framework and its corpus. LaTeX sources in [`research/publications/`](research/publications/); build all PDFs with `make -C research`.

- [TIGER: A Five-Dimensional Framework for Game Design Analysis](research/publications/tiger-framework/) — core framework
- [The TIGRIS Corpus: Methodology for Multi-Axis Game Design Scoring](research/publications/tiger-corpus-methodology/)
- [Five Independent Dimensions of Game Design Space](research/publications/tiger-pca-dimensions/)
- [Design Fingerprinting: Within-Game Normalization](research/publications/tiger-fingerprinting/)
- [Predicting Player Experience from TIGER Profiles](research/publications/tiger-experience-prediction/)
- [Filling the Design Space: Using TIGER Profiles to Identify Gaps](research/publications/tiger-design-gaps/)

## License

[MIT](LICENSE) — © 2026 Gio Della-Libera.
