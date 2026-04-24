# Paper Plan: TG-B2 — Design Fingerprinting

## Paper ID
TG-B2 | Track B | Venue: CHI / FDG 2027

## Research Question
Does within-game z-score normalization produce a useful design fingerprint for board
games? Does it reduce the dominance of the general quality factor to the point where
games in similar quality tiers have meaningfully different fingerprints?

## Primary Claim
Within-game z-score normalization reduces PC1 from 81% to ≤55%, and the resulting
fingerprints have mean pairwise cosine distance ≥0.45, confirming that games genuinely
differ in design character even at similar quality levels.

## Key Sections
1. Introduction — the general factor problem and why it obscures design character
2. Related Work — fingerprinting in music/visual art, ipsative scoring in psychometrics
3. Three Normalization Methods — raw, weight-normalized, within-game z-score
4. Design Fingerprints — 8 example games with fingerprint visualizations
5. Analysis of Fingerprint Space — pairwise distances, designer clustering
6. Discussion — implications for designers, limitations
7. Conclusion

## Data Dependencies
- C:/src/tigris/data/axis-matrix-v2.csv
- Python: numpy, scipy (cosine distance), sklearn (KMeans)
- Output: fingerprint-matrix.csv (150 games × 32 z-scored axes)

## Key Figures
- Figure 1: Normalization comparison — PC1 variance explained bar chart (N1/N2/N3)
- Figure 2: Eight fingerprint radar charts (2×4 grid)
- Figure 3: Distribution of pairwise cosine distances (raw vs. z-scored)
- Figure 4: Designer fingerprint clustering (t-SNE visualization with designer labels)

## Key Tables
- Table 1: Normalization method comparison (PC1%, mean pairwise distance)
- Table 2: Top 5 highest and lowest z-score axes for 8 example games
- Table 3: Designer clustering results (within-designer vs. between-designer distance)

## Target Word Count
6000–8000 words

## Dependencies on Other Papers
- Depends on: TG-A1 (corpus), TG-A2 (PCA), TG-B1 (TIGER dimensions)
- Provides: fingerprint-matrix.csv used in TG-C2 (clustering for design gaps)

## Self-Score (design stage): 8.2/10
The normalization technique is the core methodological contribution. The designer
clustering analysis (if it holds) is a strong validation result. Risk: the cosine
distance threshold (≥0.45) is set without a gold standard; reviewers may ask what
distance is "large enough" to be meaningful.
