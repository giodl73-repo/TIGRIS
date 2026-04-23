"""
TIGER Gap Analysis — Full 150-game corpus
Computes TIGER dimension scores from axis groupings, identifies sparse regions,
and ranks multiplicative design opportunities (gaps rare in 2+ dimensions).
"""
import csv, math, json

# Axis-to-TIGER dimension mapping (based on axis-pool definitions)
TIGER_MAP = {
    'T': ['B2', 'B5', 'C1', 'C2', 'C3', 'C6'],          # Tension: pressure, stakes, scarcity
    'I': ['A3', 'B3', 'B4', 'A26'],                       # Interaction: contest, info, social
    'G': ['A2', 'A5', 'B1', 'C4', 'C8', 'A28'],          # Game-Architecture: depth, complexity, engine
    'E': ['A1', 'A7', 'B6', 'D4'],                        # Elegance: parsimony, agency, catch-up, lang-ind
    'R': ['A4', 'C7', 'D1', 'D2', 'D3'],                  # Range: variability, replayability, accessibility
}

def load(path='data/axis-matrix.csv'):
    games, profiles = [], []
    with open(path, encoding='utf-8') as f:
        reader = csv.DictReader(f)
        for row in reader:
            try:
                coords = {}
                for dim, axes in TIGER_MAP.items():
                    vals = []
                    for ax in axes:
                        v = row.get(ax, '') or row.get(ax.lower(), '')
                        if v and v.strip():
                            try:
                                vals.append(float(v))
                            except ValueError:
                                pass
                    coords[dim] = sum(vals) / len(vals) if vals else 5.0
                games.append({
                    'num': row.get('game_num', '?'),
                    'name': row.get('name', '?'),
                    'type': row.get('type', '?'),
                    'weight': row.get('weight', '') or '2.5',
                })
                profiles.append(coords)
            except Exception as e:
                pass
    return games, profiles

def corpus_stats(profiles):
    dims = list(TIGER_MAP.keys())
    stats = {}
    for d in dims:
        vals = [p[d] for p in profiles]
        mu = sum(vals) / len(vals)
        sd = math.sqrt(sum((v - mu)**2 for v in vals) / len(vals))
        lo, hi = min(vals), max(vals)
        q1 = sorted(vals)[len(vals)//4]
        q3 = sorted(vals)[3*len(vals)//4]
        stats[d] = {'mu': mu, 'sd': sd, 'lo': lo, 'hi': hi, 'q1': q1, 'q3': q3}
    return stats

def euclidean(a, b, dims):
    return math.sqrt(sum((a[d] - b[d])**2 for d in dims))

def find_nearest(profile, profiles, games, dims, exclude_idx=None):
    best_d, best_game = 1e9, None
    for i, (p, g) in enumerate(zip(profiles, games)):
        if i == exclude_idx:
            continue
        d = euclidean(profile, p, dims)
        if d < best_d:
            best_d, best_game = d, g
    return best_d, best_game

def gap_score(point, profiles, dims, bandwidth=1.5):
    """Gaussian KDE density at a point (lower = sparser = bigger gap)."""
    n = len(profiles)
    total = 0.0
    for p in profiles:
        d2 = sum((point[d] - p[d])**2 for d in dims)
        total += math.exp(-d2 / (2 * bandwidth**2))
    return total / n

def percentile_rank(profiles, dims):
    """Return (q1, q3) threshold for each dim: games below q1 = low, above q3 = high."""
    result = {}
    for d in dims:
        vals = sorted(p[d] for p in profiles)
        n = len(vals)
        result[d] = {'q1': vals[n//4], 'median': vals[n//2], 'q3': vals[3*n//4]}
    return result

def main():
    games, profiles = load()
    if not games:
        print("No data. Run from C:/src/tigris"); return
    dims = list(TIGER_MAP.keys())
    stats = corpus_stats(profiles)
    pcts = percentile_rank(profiles, dims)

    print("=" * 70)
    print(f"TIGER FULL-CORPUS GAP ANALYSIS — {len(games)} games")
    print("=" * 70)

    # 1. Per-dimension distribution
    print("\n1. TIGER DIMENSION DISTRIBUTION (corpus mean ± sd, range)\n")
    for d in dims:
        s = stats[d]
        print(f"  {d}:  mean={s['mu']:.2f}  sd={s['sd']:.2f}  "
              f"range=[{s['lo']:.1f}–{s['hi']:.1f}]  "
              f"Q1={s['q1']:.2f}  Q3={s['q3']:.2f}")

    # 2. Top 8 per dimension (highest scorers — what already exists)
    print("\n2. CORPUS LEADERS — top 5 per dimension (already filling this space)\n")
    for d in dims:
        ranked = sorted(zip([p[d] for p in profiles], games), reverse=True)[:5]
        names = "  ".join(f"{g['name'][:18]}({v:.1f})" for v, g in ranked)
        print(f"  {d}: {names}")

    # 3. Sparse quadrant analysis — 2D gap pairs
    print("\n3. SPARSE 2D COMBINATIONS (multiplicative gaps — pairs of dimensions)\n")
    dim_pairs = [(dims[i], dims[j]) for i in range(len(dims)) for j in range(i+1, len(dims))]
    pair_results = []
    for d1, d2 in dim_pairs:
        # Count games in high-high quadrant (both > Q3)
        q3_1, q3_2 = pcts[d1]['q3'], pcts[d2]['q3']
        hh = [(g, p) for g, p in zip(games, profiles) if p[d1] > q3_1 and p[d2] > q3_2]
        # Count games in sparse "gap" quadrant (gap profile: both dimensions underrepresented relative to corpus)
        # A design opportunity: a game could be HIGH on BOTH, but few exist
        density = len(hh)
        pair_results.append((density, d1, d2, hh))
    pair_results.sort()  # lowest density first = biggest gap

    for density, d1, d2, hh in pair_results[:7]:
        names = "  ".join(g['name'][:20] for g, _ in hh[:4]) if hh else "(none)"
        print(f"  {d1}+{d2} HIGH:  {density} games occupy this region")
        if hh:
            print(f"    → Existing: {names}")
        else:
            print(f"    → Region is EMPTY in corpus")

    # 4. Sparse 3D combinations
    print("\n4. SPARSE 3D COMBINATIONS (highest multiplicative opportunity)\n")
    triples = [(dims[i], dims[j], dims[k])
               for i in range(len(dims))
               for j in range(i+1, len(dims))
               for k in range(j+1, len(dims))]
    triple_results = []
    for d1, d2, d3 in triples:
        q3_1, q3_2, q3_3 = pcts[d1]['q3'], pcts[d2]['q3'], pcts[d3]['q3']
        hhh = [(g, p) for g, p in zip(games, profiles)
               if p[d1] > q3_1 and p[d2] > q3_2 and p[d3] > q3_3]
        triple_results.append((len(hhh), d1, d2, d3, hhh))
    triple_results.sort()

    for density, d1, d2, d3, hhh in triple_results[:8]:
        names = "  ".join(g['name'][:16] for g, _ in hhh[:3]) if hhh else "(none)"
        label = "(EMPTY)" if not hhh else f"({density} games)"
        print(f"  {d1}+{d2}+{d3} HIGH {label}: {names}")

    # 5. Per-game isolation score (games farthest from any neighbour = unique)
    print("\n5. MOST ISOLATED GAMES (unique fingerprints — no near neighbours)\n")
    isolation = []
    for i, (g, p) in enumerate(zip(games, profiles)):
        d, nearest = find_nearest(p, profiles, games, dims, exclude_idx=i)
        isolation.append((d, g, p, nearest))
    isolation.sort(reverse=True)
    for d, g, p, nearest in isolation[:10]:
        coord = " ".join(f"{dim}={p[dim]:.1f}" for dim in dims)
        print(f"  #{g['num']:3s} {g['name'][:28]:28s}  gap={d:.2f}  coords: {coord}")
        if nearest:
            print(f"       → nearest: {nearest['name'][:30]}")

    # 6. Named design gap opportunities (with multiplicativity score)
    print("\n6. TOP DESIGN GAP OPPORTUNITIES (multiplicativity = rarity across dimensions)\n")

    # For each game, compute how far it sits from the corpus median in each dim
    # A "gap profile" is a hypothetical game that is 1 SD above mean on N dimensions simultaneously
    # Multiplicativity = product of (how rare high scores are on each chosen dimension)
    gap_candidates = [
        {'name': 'High-T + High-E (Tense Filler)',
         'T': stats['T']['mu'] + 1.5*stats['T']['sd'],
         'I': stats['I']['mu'],
         'G': stats['G']['mu'] - 0.5*stats['G']['sd'],
         'E': stats['E']['mu'] + 1.5*stats['E']['sd'],
         'R': stats['R']['mu']},
        {'name': 'High-I + High-R (Competitive Replayable)',
         'T': stats['T']['mu'],
         'I': stats['I']['mu'] + 1.5*stats['I']['sd'],
         'G': stats['G']['mu'] - 0.5*stats['G']['sd'],
         'E': stats['E']['mu'],
         'R': stats['R']['mu'] + 1.5*stats['R']['sd']},
        {'name': 'High-G + High-E (Deep but Elegant)',
         'T': stats['T']['mu'] - 0.5*stats['T']['sd'],
         'I': stats['I']['mu'],
         'G': stats['G']['mu'] + 1.5*stats['G']['sd'],
         'E': stats['E']['mu'] + 1.5*stats['E']['sd'],
         'R': stats['R']['mu']},
        {'name': 'High-T + High-G + High-R (Punishing Replayable Euro)',
         'T': stats['T']['mu'] + 1.5*stats['T']['sd'],
         'I': stats['I']['mu'],
         'G': stats['G']['mu'] + 1.5*stats['G']['sd'],
         'E': stats['E']['mu'],
         'R': stats['R']['mu'] + 1.5*stats['R']['sd']},
        {'name': 'High-E + High-R + Low-G (Elegant Replayable Filler)',
         'T': stats['T']['mu'],
         'I': stats['I']['mu'],
         'G': stats['G']['mu'] - 1.0*stats['G']['sd'],
         'E': stats['E']['mu'] + 1.5*stats['E']['sd'],
         'R': stats['R']['mu'] + 1.5*stats['R']['sd']},
        {'name': 'High-I + High-T + Low-G (Social Pressure Light)',
         'T': stats['T']['mu'] + 1.2*stats['T']['sd'],
         'I': stats['I']['mu'] + 1.2*stats['I']['sd'],
         'G': stats['G']['mu'] - 1.0*stats['G']['sd'],
         'E': stats['E']['mu'],
         'R': stats['R']['mu']},
        {'name': 'High-T + High-E + High-R (The Holy Grail)',
         'T': stats['T']['mu'] + 1.5*stats['T']['sd'],
         'I': stats['I']['mu'],
         'G': stats['G']['mu'] - 0.5*stats['G']['sd'],
         'E': stats['E']['mu'] + 1.5*stats['E']['sd'],
         'R': stats['R']['mu'] + 1.5*stats['R']['sd']},
    ]

    for cand in gap_candidates:
        point = {d: cand[d] for d in dims}
        density = gap_score(point, profiles, dims)
        d, nearest = find_nearest(point, profiles, games, dims)
        coord_str = " ".join(f"{d}={cand[d]:.1f}" for d in dims)
        print(f"  [{cand['name']}]")
        print(f"    coords: {coord_str}")
        print(f"    KDE density: {density:.4f}  (lower = sparser)")
        print(f"    nearest corpus game: {nearest['name'][:35]} (dist={d:.2f})")
        print()

    # 7. Games that partially fill each gap (closest to gap targets, potential "closest existing")
    print("7. EXISTING GAMES CLOSEST TO EACH GAP (what you could play today)\n")
    for cand in gap_candidates:
        point = {d: cand[d] for d in dims}
        ranked = sorted(
            [(euclidean(point, p, dims), g) for g, p in zip(games, profiles)]
        )[:3]
        names = "  |  ".join(f"{g['name'][:22]}(d={d:.2f})" for d, g in ranked)
        print(f"  {cand['name'][:38]:38s}: {names}")

if __name__ == '__main__':
    main()
