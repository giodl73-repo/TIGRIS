# Paper Plan: TG-A1 — The TIGRIS Corpus

## Paper ID
TG-A1 | Track A | Venue: CHI / FDG 2027

## Research Question
Can a structured multi-axis scoring protocol produce reliable (Cronbach's α ≥ 0.80)
evaluations of board game design across a 150-game corpus? How does scoring reliability
vary across the four axis bands (A/B/C/D), and what is the fidelity of fast-track vs.
full-pipeline scores?

## Primary Claim
The TIGRIS corpus achieves Cronbach's α ≥ 0.80 overall, establishing it as a reliable
foundation for downstream PCA and predictive modeling. Fast-track scores correlate
with full-pipeline scores at r ≥ 0.85 on the 30-game validation subset.

## Key Sections
1. Introduction — motivation for a reliable game design corpus
2. Related Work — game evaluation rubrics, BGG weight, psychometric measurement
3. Corpus Construction — game selection, axis definitions, scoring protocol
4. Reliability Results — Cronbach's α per axis and overall, fast-track vs. full-pipeline
5. Discussion — limitations, bias, implications for downstream analysis
6. Conclusion

## Data Dependencies
- C:/src/tigris/data/axis-matrix-v2.csv — 150 games × 32 axes (primary corpus)
- 30-game full-pipeline validation subset (to be collected)
- BGG weight and rating data via BGG XML API

## Key Figures
- Figure 1: Corpus construction flowchart (sampling strategy, scoring modes)
- Figure 2: Per-axis Cronbach's α bar chart by band
- Figure 3: Fast-track vs. full-pipeline scatter plot for 30-game validation subset
- Figure 4: Corpus coverage heatmap (weight × year)

## Key Tables
- Table 1: Corpus summary statistics (weight distribution, publication year, player count)
- Table 2: Axis definitions and band assignments (32 axes)
- Table 3: Per-axis Cronbach's α with confidence intervals
- Table 4: Fast-track vs. full-pipeline mean absolute deviation per axis

## Target Word Count
6000–8000 words (FDG: 10 pages, CHI: 10 pages)

## Dependencies on Other Papers
- Provides data foundation for TG-A2, TG-B1, TG-B2, TG-C1, TG-C2
- Must be submitted or available as preprint before other TIGER papers can cite it

## Self-Score (design stage): 8.2/10
Strong empirical foundation, clear methodology, open data release. Weakness: corpus
size (150 games) is modest by ML standards; claim that fast-track scores are reliable
enough for PCA depends on validation results not yet collected.
