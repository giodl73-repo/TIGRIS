"""
TIGER BEAT Gap Analysis - 150-game corpus
"""
import sys, io
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
"""
Nine dimensions from real PCA. Finds sparse regions = design opportunities.
"""
import csv, math, io

# ── Dimension definitions from real PCA loadings ─────────────────────────────
# Each dim is computed as mean of its key axes (all on 0-10 scale)
# G is bipolar: Architecture pole minus Elegance pole, recentered to 0-10

DIMS = {
    # TIGER
    'T': ['C1','B2','B5','C2'],                     # Tension / Stakes
    'I': ['A3','B4','B5'],                           # Interaction / Conflict
    'G': (['B1','B3','C4'], ['A1','A6','C7']),       # Game-spectrum (bipolar)
    'E': ['A26','A27','A28'],                        # Experiential
    'R': ['A4','C7','D3'],                           # Range / Variability
    # BEAT
    'B': ['D2','D3','C7'],                           # Breadth / Multi-mode
    'EA':['A25'],                                    # Emotional Arc
    'A': ['D1','D4'],                                # Accessibility
    'TX':['C3','B3','C6'],                           # Texture / Scarcity
}

def score(row, dim):
    spec = DIMS[dim]
    def ax(a):
        v = row.get(a,'')
        try: return float(v) if v else None
        except: return None
    if dim == 'G':
        pos_axes, neg_axes = spec
        pos = [ax(a) for a in pos_axes if ax(a) is not None]
        neg = [ax(a) for a in neg_axes if ax(a) is not None]
        if not pos or not neg: return 5.0
        return (sum(pos)/len(pos) - sum(neg)/len(neg)) / 2 + 5  # recentered to ~5
    else:
        vals = [ax(a) for a in spec if ax(a) is not None]
        return sum(vals)/len(vals) if vals else 5.0

def load(path='data/axis-matrix.csv'):
    games, profiles = [], []
    with open(path, encoding='utf-8') as f:
        for row in csv.DictReader(f):
            try:
                p = {d: score(row, d) for d in DIMS}
                games.append({'num': row['game_num'], 'name': row['name'],
                              'weight': row.get('weight',''), 'type': row.get('type','')})
                profiles.append(p)
            except: pass
    return games, profiles

def stats(profiles, dim):
    vals = sorted(p[dim] for p in profiles)
    n = len(vals)
    mu = sum(vals)/n
    sd = math.sqrt(sum((v-mu)**2 for v in vals)/n)
    return {'mu':mu,'sd':sd,'q1':vals[n//4],'q3':vals[3*n//4],'lo':vals[0],'hi':vals[-1]}

def dist(a, b, dims):
    return math.sqrt(sum((a[d]-b[d])**2 for d in dims))

def nearest(point, profiles, games, dims):
    best = min(range(len(profiles)), key=lambda i: dist(point, profiles[i], dims))
    return dist(point, profiles[best], dims), games[best]

def main():
    games, profiles = load()
    print(f"TIGER BEAT GAP ANALYSIS - {len(games)} games x 9 dimensions")
    print("="*65)

    tiger_dims = ['T','I','G','E','R']
    beat_dims  = ['B','EA','A','TX']
    all_dims   = tiger_dims + beat_dims

    # ── 1. Distribution ───────────────────────────────────────────────────────
    print("\n1. DIMENSION DISTRIBUTIONS\n")
    dim_stats = {d: stats(profiles, d) for d in all_dims}
    for d in all_dims:
        s = dim_stats[d]
        label = {'T':'Tension','I':'Interaction','G':'Game-spectrum','E':'Experiential',
                 'R':'Range','B':'Breadth','EA':'Emotional Arc','A':'Accessibility',
                 'TX':'Texture'}[d]
        print(f"  {d}  {label:16s}  mean={s['mu']:.1f}  sd={s['sd']:.1f}  "
              f"[{s['lo']:.1f}-{s['hi']:.1f}]  Q1={s['q1']:.1f} Q3={s['q3']:.1f}")

    # ── 2. Top 5 games per dimension ─────────────────────────────────────────
    print("\n2. CORPUS LEADERS (top 5 per dimension)\n")
    for d in all_dims:
        ranked = sorted(zip([p[d] for p in profiles], [g['name'] for g in games]), reverse=True)[:5]
        names = '  '.join(f"{n[:16]}({v:.1f})" for v,n in ranked)
        print(f"  {d}: {names}")

    # ── 3. Sparse 2D pairs (TIGER only - the design-space map) ───────────────
    print("\n3. SPARSE TIGER PAIRS (few games high on both)\n")
    pairs = [(tiger_dims[i], tiger_dims[j])
             for i in range(len(tiger_dims)) for j in range(i+1,len(tiger_dims))]
    pair_rows = []
    for d1, d2 in pairs:
        q3_1 = dim_stats[d1]['q3']
        q3_2 = dim_stats[d2]['q3']
        hh = [(g,p) for g,p in zip(games,profiles) if p[d1]>q3_1 and p[d2]>q3_2]
        pair_rows.append((len(hh), d1, d2, hh))
    pair_rows.sort()
    for cnt, d1, d2, hh in pair_rows:
        names = '  '.join(g['name'][:18] for g,_ in hh[:4]) if hh else '(EMPTY)'
        print(f"  {d1}+{d2} high: {cnt:3d} games  |  {names}")

    # ── 4. Multiplicative gaps (TIGER 3-way) ─────────────────────────────────
    print("\n4. MULTIPLICATIVE GAPS (3 TIGER dims simultaneously high)\n")
    triples = [(tiger_dims[i],tiger_dims[j],tiger_dims[k])
               for i in range(5) for j in range(i+1,5) for k in range(j+1,5)]
    triple_rows = []
    for d1,d2,d3 in triples:
        q3 = [dim_stats[d]['q3'] for d in (d1,d2,d3)]
        hhh = [(g,p) for g,p in zip(games,profiles)
               if p[d1]>q3[0] and p[d2]>q3[1] and p[d3]>q3[2]]
        triple_rows.append((len(hhh), d1, d2, d3, hhh))
    triple_rows.sort()
    for cnt,d1,d2,d3,hhh in triple_rows[:8]:
        names = '  '.join(g['name'][:16] for g,_ in hhh[:3]) if hhh else '(EMPTY)'
        print(f"  {d1}+{d2}+{d3}: {cnt:3d} games  |  {names}")

    # ── 5. Named design gap opportunities ────────────────────────────────────
    print("\n5. DESIGN GAP OPPORTUNITIES (with nearest existing game)\n")
    gaps = [
        ('High-T + High-I + Low-G',
         'Tense direct-conflict filler: maximum stakes, maximum fighting, minimal rules',
         {'T':8,'I':8,'G':3,'E':5,'R':5,'B':5,'EA':5,'A':6,'TX':5}),
        ('High-G + High-E (Experiential)',
         'Deep engine game with strong social/physical character - Lacerda meets Gloomhaven',
         {'T':5,'I':5,'G':7,'E':8,'R':5,'B':5,'EA':6,'A':4,'TX':5}),
        ('High-T + High-E (Experiential) + Low-G',
         'Tense social-pressure light game: high stakes, strong social contract, simple rules',
         {'T':8,'I':5,'G':3,'E':8,'R':5,'B':5,'EA':5,'A':7,'TX':5}),
        ('High-R + High-A + Low-G',
         'Maximally replayable gateway: huge variability, highly accessible, no engine',
         {'T':4,'I':5,'G':2,'E':5,'R':9,'B':7,'EA':4,'A':9,'TX':4}),
        ('High-EA + High-I + Low-G',
         'Narrative social game: strong emotional journey through direct player conflict, light rules',
         {'T':5,'I':8,'G':2,'E':5,'R':5,'B':5,'EA':9,'A':6,'TX':4}),
        ('High-T + High-G + High-R',
         'The punishing replayable euro: deep engine, high stakes, vast strategic space',
         {'T':8,'I':4,'G':8,'E':4,'R':8,'B':5,'EA':4,'A':3,'TX':6}),
        ('High-B + High-A + High-R',
         'The universal game: solo+multi, accessible, endlessly replayable - near-impossible to design',
         {'T':4,'I':4,'G':3,'E':6,'R':8,'B':9,'EA':4,'A':9,'TX':4}),
        ('High-TX + High-I + Low-A',
         'Brutal negotiation/scarcity game: punishing resource competition, direct conflict, hard to learn',
         {'T':6,'I':8,'G':5,'E':4,'R':5,'B':4,'EA':5,'A':3,'TX':9}),
    ]

    for name, desc, point in gaps:
        d, near = nearest(point, profiles, games, all_dims)
        coords = ' '.join(f"{k}={v}" for k,v in point.items() if k in tiger_dims)
        print(f"  [{name}]")
        print(f"  {desc}")
        print(f"  TIGER: {coords}   nearest: {near['name'][:30]} (dist={d:.1f})")
        print()

    # ── 6. Most isolated games (unique in 9D space) ───────────────────────────
    print("6. MOST ISOLATED GAMES (unique fingerprints - no design neighbours)\n")
    isolation = []
    for i,(g,p) in enumerate(zip(games,profiles)):
        others = [profiles[j] for j in range(len(profiles)) if j!=i]
        other_g = [games[j] for j in range(len(games)) if j!=i]
        d, near = nearest(p, others, other_g, tiger_dims)
        isolation.append((d, g, p, near))
    isolation.sort(key=lambda x: x[0], reverse=True)
    for d, g, p, near in isolation[:10]:
        coords = ' '.join(f"{k}={p[k]:.1f}" for k in tiger_dims)
        print(f"  #{g['num']:3s} {g['name'][:28]:28s}  gap={d:.1f}  {coords}")
        print(f"       nearest: {near['name'][:35]}")

if __name__ == '__main__':
    main()
