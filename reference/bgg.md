---
name: BoardGameGeek — canonical reference
slug: bgg-reference
version: 1.0.0
rubric_version: v2.6
bet_version: parliament
author: TIGRIS
created: 2026-04-19
updated: 2026-04-19
---

# BoardGameGeek as the TIGRIS Bible

**BGG is the canonical reference for all published board games reviewed by TIGRIS.** Every review-mode game carries a BoardGameGeek ID in its frontmatter, enabling unambiguous reference across the factory's lifetime.

## Why BGG

1. **Unique IDs per game.** Puerto Rico is `3076`; no ambiguity with "Puerto Rico" (the location or any other game).
2. **Stable public URL format**: `https://boardgamegeek.com/boardgame/<id>` resolves to the game's canonical page.
3. **Community ground truth** for designer, publisher, year, player count, weight, rank, and strategic discussion.
4. **Edition tracking**: BGG distinguishes base games from expansions, reprints, and variants.
5. **Fair-use citation path**: review-mode `design.md` files cite the BGG entry as a source; this is established critical-commentary practice.

## Frontmatter contract (v2.6 standard)

All review-mode games MUST include `bgg_id` in their frontmatter. Originals designed by TIGRIS omit the field until they have a published BGG entry.

```yaml
---
name: <game> — Concept (Review import)
slug: <game-slug>
game: NNNN-<slug>
stage: concept
version: 1.0.0
rubric_version: v2.6
bet_version: parliament
author: TIGRIS
created: YYYY-MM-DD
updated: YYYY-MM-DD
anchor_persona: <slug>
anchor_axis: <slug>
review_type: imported-review
game_designer: <designer name>
game_first_published: <year> (<publisher>)
canonical_edition: <edition — year + publisher>
bgg_id: <integer>                     # REQUIRED for review-mode
bgg_url: https://boardgamegeek.com/boardgame/<id>   # derived; optional explicit
---
```

## BGG IDs for games already reviewed

| Game | Slug | BGG ID | URL |
|---|---|---:|---|
| Tigris & Euphrates | 0002-tigris-and-euphrates | **42** | https://boardgamegeek.com/boardgame/42 |
| Dominion | 0004-dominion | **36218** | https://boardgamegeek.com/boardgame/36218 |
| Scythe | 0005-scythe | **169786** | https://boardgamegeek.com/boardgame/169786 |
| Puerto Rico | 0006-puerto-rico | **3076** | https://boardgamegeek.com/boardgame/3076 |
| Parliament | 0001-parliament | — (TIGRIS original; no BGG yet) | — |
| FACETS | 0003-facets | — (TIGRIS original; no BGG yet) | — |

## BGG IDs for commonly-referenced candidate games

| Game | Designer | BGG ID |
|---|---|---:|
| Agricola | Rosenberg | 31260 |
| Le Havre | Rosenberg | 35677 |
| Caverna | Rosenberg | 102794 |
| A Feast for Odin | Rosenberg | 177736 |
| Castles of Burgundy | Feld | 84876 |
| In the Year of the Dragon | Feld | 31594 |
| Trajan | Feld | 102680 |
| Bruges | Feld | 136888 |
| Lisboa | Lacerda | 161533 |
| Vinhos (Deluxe) | Lacerda | 175238 |
| The Gallerist | Lacerda | 125153 |
| Kanban EV | Lacerda | 284083 |
| On Mars | Lacerda | 184267 |
| Through the Ages: A New Story | Chvátil | 182028 |
| Galaxy Trucker | Chvátil | 31481 |
| Tzolk'in | Chvátil | 126163 |
| Codenames | Chvátil | 178900 |
| Mage Knight | Chvátil | 96848 |
| Tikal | Kramer/Kiesling | 54 |
| Torres | Kramer/Kiesling | 88 |
| Mexica | Kramer/Kiesling | 4402 |
| Azul | Kiesling | 230802 |
| Viticulture EE | Stegmaier | 183394 |
| Wingspan | Hargrave (Stonemaier) | 266192 |
| Ra | Knizia | 12 |
| Modern Art | Knizia | 118 |
| Ingenious | Knizia | 15512 |
| Samurai | Knizia | 3 |
| Lost Cities | Knizia | 50 |
| Taj Mahal | Knizia | 475 |
| Medici | Knizia | 46 |
| Brass: Birmingham | Wallace/Brown | 224517 |
| Brass: Lancashire | Wallace | 28720 |
| Concordia | Gerdts | 124361 |
| Terraforming Mars | Fryxelius | 167791 |
| 7 Wonders | Bauza | 68448 |
| Splendor | André | 148228 |
| Jaipur | Pauchon | 54043 |
| Gloomhaven | Childres | 174430 |
| Spirit Island | Reed | 162886 |
| Glory to Rome | Chudyk | 19237 |
| Race for the Galaxy | Lehmann | 28143 |
| San Juan | Seyfarth | 8217 |
| Azul | Kiesling | 230802 |
| Crokinole | Traditional (Wülfing c.1876) | 521 |
| Pandemic Legacy Season 1 | Daviau / Leacock | 161936 |
| Bohnanza | Rosenberg | 11 |
| Patchwork | Rosenberg | 163412 |
| Kingdom Builder | Vaccarino | 107529 |
| Temporum | Vaccarino | 152942 |

## Protocol updates (v2.6)

**For future review-mode games**:
1. `/tigris-concept` (in review-mode) MUST prompt for BGG ID and populate the frontmatter.
2. `/tigris-design` (review-mode) MUST include the BGG URL in the Citations section.
3. Citations may reference BGG community commentary with the URL format above.

**For originals**: BGG ID is omitted until the game is published on BGG (if ever). The field may be added later as an update.

## Edition awareness

BGG distinguishes base games from expansions. The `canonical_edition` field in frontmatter captures which specific edition of the rules the review uses (e.g., "Rio Grande 2008 1st printing" vs "Rio Grande 2016 2nd printing"). When rules differ between editions, the BGG entry usually has notes about which edition is most commonly played; TIGRIS reviews should prefer the most-current-printing rules unless the review is specifically historical.

## Fair-use posture

BGG is a public community database. Referencing BGG IDs and citing BGG pages for rules summaries / community commentary falls within standard critical-commentary fair use. TIGRIS does NOT reproduce BGG's rule texts verbatim; we summarize for review purposes with citation.

## Retrofit schedule

As of v2.6, the following games' concept.md frontmatter should be updated to include `bgg_id`:

- ✅ 0002-tigris-and-euphrates → 42
- ✅ 0004-dominion → 36218
- ✅ 0005-scythe → 169786
- ✅ 0006-puerto-rico → 3076

Retrofit to be applied in the commit that creates this file.

## Changelog

- **v2.6** — 2026-04-19 — BGG reference established as canonical.
