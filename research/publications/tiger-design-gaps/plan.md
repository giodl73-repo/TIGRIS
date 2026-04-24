# Paper Plan: TG-C2 — Design Space Gaps

## Paper ID
TG-C2 | Track C | Venue: CHI / FDG 2027

## Research Question
Does the TIGER fingerprint space contain sparse regions that correspond to genuinely
unexplored and commercially viable game design opportunities? Can computational
design space analysis surface opportunities that experienced designers validate as real?

## Primary Claim
K-means clustering (k=8) of the 150-game TIGER fingerprint space identifies ≥3 sparse
clusters, of which ≥2 are validated by an expert panel of 5 game designers as genuinely
unexplored and commercially viable.

## Key Sections
1. Introduction — design space as a navigable geometry, motivation for gap analysis
2. Related Work — procedural content generation gap analysis, market gap studies
3. Method — TIGER fingerprint space, k-means clustering, density estimation, expert briefs
4. Results — sparse clusters identified, TIGER space visualization, expert validation
5. Discussion — why some sparse regions are not opportunities, TIGER as generative tool
6. Conclusion — methodology generalizability, future work
7. Appendix — full TIGER profiles for all 150 corpus games

## Data Dependencies
- fingerprint-matrix.csv from TG-B2 (150 games × 32 z-scored axes)
- Python: sklearn (KMeans, KernelDensity), matplotlib (3D scatter)
- Expert panel: 5 game designers with ≥2 published titles (recruitment plan TBD)

## Key Figures
- Figure 1: TIGER space visualization — T-I plane and G-E plane with cluster overlays
- Figure 2: Cluster density map — kernel density estimate with sparse regions highlighted
- Figure 3: Sparse cluster radar charts (TIGER profiles for 3+ sparse cluster centroids)
- Figure 4: Expert validation results — viability ratings by cluster and designer

## Key Tables
- Table 1: K-means cluster summary (centroid TIGER coordinates, game count, density rank)
- Table 2: Sparse cluster design briefs (natural-language description of each opportunity)
- Table 3: Expert validation ratings (5 designers × sparse clusters)
- Appendix Table: TIGER profiles for all 150 corpus games

## Target Word Count
7000–9000 words (appendix adds ~2000 words)

## Dependencies on Other Papers
- Depends on: TG-A1, TG-A2, TG-B1, TG-B2 (full upstream chain)
- Appendix provides: reference resource for practitioners using TIGER

## Self-Score (design stage): 8.2/10
The generative application of TIGER (finding gaps) is the novel contribution beyond
the analytical papers. The expert validation study design is the key credibility
mechanism. Risk: recruiting 5 experienced designers willing to participate in a
structured interview is a practical obstacle not yet resolved.
