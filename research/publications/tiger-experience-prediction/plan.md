# Paper Plan: TG-C1 — Predicting Player Experience

## Paper ID
TG-C1 | Track C | Venue: CHI / FDG 2027

## Research Question
Do TIGER profiles predict BGG ratings better than weight alone? Which TIGER dimensions
contribute most to satisfaction prediction?

## Primary Claim
TIGER-only regression achieves r ≥ 0.60 and ≥15% RMSE improvement over weight-only
baseline on a held-out test set of 28 games. Elegance and Game-Architecture are the
highest-contributing dimensions.

## Hypothesis
TIGER profiles capture design character variation that correlates with player satisfaction
independently of complexity (weight). Games with higher Elegance and Game-Architecture
scores are better-liked by the BGG community, even controlling for weight.

## Key Sections
1. Introduction — BGG rating as player experience proxy, weight as baseline predictor
2. Related Work — player experience modeling, BGG prediction studies
3. Hypothesis — formal statement of prediction hypothesis and threshold
4. Data and Methods — corpus, BGG API, train/test split, Ridge regression, feature sets
5. Results — prediction accuracy table, regression coefficients, ablation results
6. Discussion — practical implications, BGG community bias, limitations
7. Conclusion

## Data Dependencies
- C:/src/tigris/data/axis-matrix-v2.csv
- BGG XML API (ratings for 150 games, retrieved March 2026)
- Python: sklearn (Ridge, cross_val_score), scipy.stats (pearsonr)

## Key Figures
- Figure 1: Predicted vs. actual BGG rating scatter plot (F2 model, test set)
- Figure 2: Ablation impact per TIGER dimension (bar chart of RMSE change)
- Figure 3: Regression coefficient plot (F2 model with confidence intervals)
- Figure 4: Residual analysis — which games are worst-predicted and why

## Key Tables
- Table 1: Model comparison (F1/F2/F3 — Pearson r, RMSE, RMSE vs. baseline)
- Table 2: Ridge regression coefficients with 95% confidence intervals (F2 model)
- Table 3: Leave-one-dimension-out ablation results

## Target Word Count
6000–8000 words

## Dependencies on Other Papers
- Depends on: TG-A1 (corpus), TG-A2 (PCA), TG-B1 (TIGER dimensions)
- Provides: predictive validation of TIGER dimensions referenced in TG-B1 discussion

## Self-Score (design stage): 8.2/10
The regression study is methodologically standard but the prediction target (BGG rating)
is noisy and community-biased. The held-out test set (N=28) is small for regression
evaluation. The r ≥ 0.60 threshold is ambitious but defensible given prior work showing
weight alone achieves r ≈ 0.40.
