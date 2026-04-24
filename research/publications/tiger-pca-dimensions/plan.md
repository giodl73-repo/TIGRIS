# Paper Plan: TG-A2 — Five Independent Dimensions

## Paper ID
TG-A2 | Track A | Venue: CHI / FDG 2027

## Research Question
How many genuinely independent dimensions of design variation exist in the TIGRIS corpus?
Does the choice of normalization method determine whether PCA reveals one general
quality factor or multiple independent design dimensions?

## Primary Claim
Within-game z-score normalization reduces PC1 from 81% to ≤55% and reveals 4–5
genuinely independent components explaining ≥80% of within-game normalized variance.
These components are labeled T-I-G-E-R.

## Key Sections
1. Introduction — the general factor problem in multi-axis design scoring
2. Related Work — PCA in design research, the g-factor problem in psychometrics
3. Methods — three normalization approaches (raw, weight-normalized, within-game z-score)
4. Results — scree plots, component loadings, TIGER dimension naming
5. Discussion — what each PC represents, limitations of normalization approach
6. Conclusion

## Data Dependencies
- C:/src/tigris/data/axis-matrix-v2.csv via TG-A1
- Python: sklearn.decomposition.PCA, scipy.stats
- Output: pca-results.csv (component loadings, variance explained)

## Key Figures
- Figure 1: Three-panel scree plot (N1 / N2 / N3)
- Figure 2: Component loading heatmap for N3 top 5 PCs
- Figure 3: Biplot showing 150 games in T-I space and G-E space
- Figure 4: Top axis correlates of each TIGER dimension

## Key Tables
- Table 1: Variance explained by top 5 PCs under each normalization method
- Table 2: Top 5 axis loadings per PC under N3
- Table 3: Examples of games at each extreme of each TIGER dimension

## Target Word Count
6000–8000 words

## Dependencies on Other Papers
- Depends on: TG-A1 (corpus reliability must be established first)
- Provides: TIGER dimension definitions used in TG-B1, TG-B2, TG-C1, TG-C2

## Self-Score (design stage): 8.2/10
The normalization insight is the core contribution and is methodologically clean.
Risk: the component labels (T-I-G-E-R) are post-hoc; reviewers may challenge whether
the dimensions are truly independent or partially confounded. Addressed by reporting
component correlations and confirming orthogonality.
