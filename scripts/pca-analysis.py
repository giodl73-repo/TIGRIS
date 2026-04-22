"""
TIGRIS Eigenaxis Analysis — Pure Python PCA
Reads data/axis-matrix.csv and computes principal components of the
28-axis game design space.
"""
import csv, math, os

AXES = ['A1','A2','A3','A4','A5','A6','A7',
        'B1','B2','B3','B4','B5','B6',
        'C1','C2','C3','C4','C6','C7','C8',
        'D1','D2','D3','D4','A25','A26','A27','A28']

def load(path='data/axis-matrix.csv'):
    games, matrix = [], []
    with open(path, encoding='utf-8') as f:
        reader = csv.DictReader(f)
        for row in reader:
            try:
                vec = [float(row.get(ax, 0) or 0) for ax in AXES]
                matrix.append(vec)
                games.append({'num': row['game_num'], 'name': row['name'],
                               'weight': row.get('weight',''), 'earned': row.get('earned','')})
            except: pass
    return games, matrix

def standardize(matrix):
    n, p = len(matrix), len(matrix[0])
    means = [sum(r[j] for r in matrix)/n for j in range(p)]
    stds  = [math.sqrt(sum((r[j]-means[j])**2 for r in matrix)/n) or 1 for j in range(p)]
    return [[( matrix[i][j]-means[j])/stds[j] for j in range(p)] for i in range(n)], means, stds

def dot(a, b): return sum(x*y for x,y in zip(a,b))
def norm(v): return math.sqrt(dot(v,v)) or 1
def normalize(v): n=norm(v); return [x/n for x in v]
def matmul_v(M, v): return [dot(row, v) for row in M]

def cov(std_matrix):
    n, p = len(std_matrix), len(std_matrix[0])
    C = [[0.0]*p for _ in range(p)]
    for i in range(p):
        for j in range(i, p):
            val = sum(std_matrix[k][i]*std_matrix[k][j] for k in range(n)) / n
            C[i][j] = C[j][i] = val
    return C

def power_iter(C, n_iter=500):
    """Power iteration with deflation to find top eigenvectors."""
    p = len(C)
    eigenvalues, eigenvectors = [], []
    Cwork = [row[:] for row in C]
    for _ in range(min(12, p)):
        v = normalize([1.0 if i==_ else 0.1 for i in range(p)])
        for __ in range(n_iter):
            v2 = matmul_v(Cwork, v)
            ev = norm(v2)
            v = normalize(v2)
        # deflate
        for i in range(p):
            for j in range(p):
                Cwork[i][j] -= ev * v[i] * v[j]
        eigenvalues.append(ev)
        eigenvectors.append(v)
    return eigenvalues, eigenvectors

def pearson(std_matrix):
    """Correlation matrix."""
    p = len(std_matrix[0])
    n = len(std_matrix)
    C = [[0.0]*p for _ in range(p)]
    for i in range(p):
        for j in range(i, p):
            num = sum(std_matrix[k][i]*std_matrix[k][j] for k in range(n))
            di = math.sqrt(sum(std_matrix[k][i]**2 for k in range(n)))
            dj = math.sqrt(sum(std_matrix[k][j]**2 for k in range(n)))
            r = num/(di*dj) if di*dj else 0
            C[i][j] = C[j][i] = r
    return C

def design_efficiency(games, matrix, eigenvalues, eigenvectors):
    """Score games by earned_count * op_bonus / weight."""
    scores = []
    for g, row in zip(games, matrix):
        try:
            earned = int(g['earned']) if g['earned'] else 0
            weight = float(g['weight']) if g['weight'] else 2.5
            # project onto top 3 PCs as proxy for axis breadth
            breadth = sum(abs(dot(row, ev)) for ev in eigenvectors[:3])
            eff = (earned * (1 + breadth/10)) / max(weight, 0.5)
            scores.append((eff, g['num'], g['name']))
        except: pass
    return sorted(scores, reverse=True)

def main():
    games, matrix = load()
    if not matrix:
        print("No data found. Run from C:/src/tigris/"); return

    std, means, stds = standardize(matrix)
    C = cov(std)
    evs, pcs = power_iter(C)
    total = sum(evs)
    cumvar = 0

    print("=" * 60)
    print(f"TIGRIS EIGENAXIS ANALYSIS — {len(matrix)} games × {len(AXES)} axes")
    print("=" * 60)
    print()
    print("PRINCIPAL COMPONENTS (variance explained):")
    n80 = None
    for i,(ev,pc) in enumerate(zip(evs,pcs)):
        pct = ev/total*100
        cumvar += pct
        top3 = sorted(zip(AXES,pc), key=lambda x:-abs(x[1]))[:3]
        top_str = "  ".join(f"{a}({v:+.2f})" for a,v in top3)
        print(f"  PC{i+1}: {pct:5.1f}%  cum={cumvar:5.1f}%  → {top_str}")
        if cumvar >= 80 and n80 is None:
            n80 = i+1
            print(f"  *** {n80} components explain 80% of variance ***")
    print()

    print("TOP AXIS CORRELATIONS (candidates for same PC / redundancy):")
    corr = pearson(std)
    pairs = [(corr[i][j], AXES[i], AXES[j])
             for i in range(len(AXES)) for j in range(i+1, len(AXES))]
    for r,a,b in sorted(pairs, reverse=True)[:12]:
        print(f"  {a} ↔ {b}: r={r:+.3f}")
    print()

    print("LEAST CORRELATED PAIRS (candidates for genuine orthogonality):")
    for r,a,b in sorted(pairs)[:8]:
        print(f"  {a} ↔ {b}: r={r:+.3f}")
    print()

    print("DESIGN EFFICIENCY TOP 10 (earned × breadth / weight):")
    eff = design_efficiency(games, std, evs, pcs)
    for score,num,name in eff[:10]:
        print(f"  #{num:3s}  {name[:30]:30s}  {score:.2f}")

    # Save results
    os.makedirs('data', exist_ok=True)
    with open('data/pca-results.txt', 'w', encoding='utf-8') as f:
        f.write(f"Games: {len(matrix)}  Axes: {len(AXES)}\n")
        f.write(f"Components for 80% variance: {n80}\n\n")
        for i,(ev,pc) in enumerate(zip(evs,pcs)):
            top5 = sorted(zip(AXES,pc), key=lambda x:-abs(x[1]))[:5]
            f.write(f"PC{i+1} ({ev/total*100:.1f}%): "+" ".join(f"{a}({v:+.2f})" for a,v in top5)+"\n")
        f.write("\nTop correlations:\n")
        for r,a,b in sorted(pairs,reverse=True)[:15]:
            f.write(f"  {a}↔{b}: {r:+.3f}\n")
    print("\nResults saved to data/pca-results.txt")

if __name__ == '__main__':
    main()
