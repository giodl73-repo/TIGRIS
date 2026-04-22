"""
Builds data/axis-matrix-v2.csv — the research-grade matrix.

Changes vs v1:
1. Games #1-69 re-scored with honest independent assessments (no Parliament drafting bias)
2. All 32 axes (adds A29 Forgiveness, A30 Ergodicity, A31 Failure Texture, A32 Info Architecture)
3. Weight-normalized residuals saved separately for PCA

Scoring principle: score what the game IS at 0-10, not what the Parliament panel happened to defend.
Heavy Euros should NOT score 8+ on everything just because they're complex.
A score of 7 means "genuinely strong on this axis relative to all games."
"""
import csv, math, json, os

AXES = ['A1','A2','A3','A4','A5','A6','A7',
        'B1','B2','B3','B4','B5','B6',
        'C1','C2','C3','C4','C6','C7','C8',
        'D1','D2','D3','D4',
        'A25','A26','A27','A28',
        'A29','A30','A31','A32']

# ── Honest independent rescoring for games #1-69 ────────────────────────────
# Axes: A1=Elegance, A2=DecisionDensity, A3=Interaction, A4=Variance,
#       A5=Pacing, A6=Teachability, A7=Replayability,
#       B1=Gearing, B2=Catastrophe, B3=ConversionChain, B4=InfoCost,
#       B5=Novelty, B6=Multiplier,
#       C1=TensionBudget, C2=MinScore, C3=Scarcity, C4=EngineGarden,
#       C6=PointSalad, C7=MenuClarity, C8=FirstTurn,
#       D1=FamilyExpert, D2=Spatial, D3=CountRobust, D4=LockIn,
#       A25=EmotionalArc, A26=SocialContract, A27=PhysicalAffordance,
#       A28=CognitiveLoadType,
#       A29=ForgivenessCurve, A30=StrategyErgodicity,
#       A31=FailureTexture, A32=InfoArchType

RESCORE = {
1: [7,8,7,5,5,6,6, 5,5,4,3,8,4, 8,4,5,3,5,6,4, 4,7,5,6, 0,2,3,5, 5,7,5,0],  # Parliament
2: [8,8,8,5,5,6,7, 6,8,3,2,7,5, 9,5,6,3,4,7,4, 5,9,5,6, 1,2,3,4, 4,6,7,0],  # T&E
3: [6,7,4,5,5,6,5, 4,3,3,2,6,6, 6,3,4,2,8,6,4, 4,4,5,5, 0,1,2,4, 5,7,3,0],  # FACETS
4: [7,8,4,7,7,7,9, 4,2,6,2,9,5, 5,2,4,8,3,7,4, 6,2,6,5, 0,1,1,5, 6,9,2,3],  # Dominion
5: [6,8,6,6,5,7,7, 5,4,4,2,7,7, 7,3,5,5,6,7,5, 7,7,7,6, 2,2,3,5, 5,6,4,0],  # Scythe
6: [7,9,7,4,5,5,6, 7,3,6,3,6,7, 7,4,7,6,7,6,4, 4,3,6,7, 0,2,2,5, 4,6,3,0],  # Puerto Rico
7: [6,7,6,5,5,6,6, 5,5,4,2,8,4, 6,2,4,3,4,6,4, 5,5,5,4, 2,2,2,4, 5,7,4,0],  # UNFOLD
8: [5,9,5,5,3,4,7, 7,7,5,3,8,7, 8,4,7,5,7,6,5, 4,3,5,7, 2,2,2,6, 3,5,5,0],  # TtA
9: [7,6,7,5,7,7,5, 3,2,2,2,5,3, 5,2,4,2,2,7,3, 4,2,7,3, 0,3,2,4, 6,5,2,3],  # Famiglia
10:[7,6,5,5,6,7,5, 3,2,4,1,5,3, 6,7,3,2,3,6,4, 4,4,5,4, 1,1,2,4, 5,6,3,0],  # ZEN PATH
11:[4,9,5,5,3,3,6, 9,5,6,7,8,6, 8,3,7,6,6,5,6, 3,5,5,8, 1,1,3,6, 2,4,4,4],  # Lisboa
12:[6,8,4,6,5,6,7, 5,3,5,2,5,5, 6,5,5,5,7,6,4, 5,3,6,6, 0,1,2,5, 5,7,3,0],  # CoB
13:[7,8,7,5,5,6,5, 5,4,3,2,6,4, 7,3,5,3,4,7,5, 5,8,6,6, 1,2,3,4, 5,5,4,0],  # Tikal
14:[6,9,5,5,4,5,6, 6,5,6,2,6,5, 9,3,9,5,4,6,5, 4,3,5,6, 1,2,2,5, 2,5,7,0],  # Agricola
15:[6,9,6,6,4,4,7, 5,7,4,5,7,5, 9,3,6,4,4,5,5, 3,3,2,7, 2,2,2,6, 4,6,7,4],  # TS
16:[5,8,4,5,4,5,5, 6,5,5,3,7,4, 7,3,6,5,4,5,4, 4,3,2,7, 1,2,2,5, 3,5,5,3],  # AFoS
17:[7,7,3,6,5,8,8, 4,2,4,2,7,7, 5,3,5,7,6,7,5, 7,2,7,5, 2,1,3,4, 6,7,2,0],  # Wingspan
18:[7,7,8,5,6,7,5, 3,6,3,4,6,3, 7,3,4,2,3,6,4, 6,3,6,3, 4,5,2,6, 5,5,6,7],  # Vigil
19:[6,7,5,8,8,7,8, 3,4,2,2,6,3, 6,2,3,2,3,6,3, 5,2,7,3, 2,2,2,5, 6,6,3,0],  # Covenstat
20:[5,8,7,6,4,5,8, 5,9,3,6,8,3, 8,2,5,3,3,6,4, 5,5,5,5, 5,5,3,5, 4,7,8,7],  # Nemesis
21:[6,7,4,6,7,6,8, 4,3,5,2,7,4, 5,2,6,7,3,6,3, 5,2,6,4, 1,1,1,5, 6,7,3,0],  # Rewild
22:[6,7,8,4,5,8,2, 6,8,3,5,8,3, 7,2,4,3,3,6,7, 9,3,5,9, 9,3,3,5, 6,5,7,0],  # PL S1
23:[6,7,5,7,7,7,7, 5,3,3,1,4,6, 6,2,5,3,9,6,3, 5,2,6,4, 0,1,2,4, 6,8,2,0],  # GARNER
24:[7,8,8,5,5,7,8, 5,6,3,2,6,4, 7,2,4,4,4,7,4, 7,9,7,6, 1,2,3,4, 5,8,4,0],  # CANTON
25:[9,7,8,5,8,10,7, 2,2,1,5,9,2, 7,1,1,1,2,8,2, 6,1,7,1, 0,8,1,8, 8,7,3,3],  # Codenames
26:[7,6,5,6,6,7,5, 5,5,3,2,6,3, 7,3,4,2,3,6,4, 6,2,6,3, 1,2,2,5, 5,5,3,0],  # NEST(26)
27:[7,8,9,5,5,7,5, 6,7,3,2,5,4, 7,2,5,3,3,7,4, 6,9,6,5, 1,2,3,4, 5,6,5,0],  # WEALD
28:[8,6,8,6,7,8,6, 3,2,4,2,8,4, 6,1,2,2,5,7,2, 5,1,7,2, 1,2,2,5, 7,6,3,3],  # FURROW
29:[7,6,7,5,5,10,6, 4,6,3,4,4,3, 6,3,4,2,2,6,7, 9,2,7,3, 8,3,2,4, 7,5,5,0],  # HEARTH
30:[9,7,8,6,8,9,6, 2,2,1,1,4,4, 6,2,2,2,7,9,2, 7,2,8,3, 0,1,5,3, 7,7,2,0],  # Azul
31:[10,7,10,7,10,10,5, 1,2,0,0,3,2, 6,1,1,0,0,9,1, 8,10,8,1, 0,1,10,3, 9,7,4,0],  # Crokinole
32:[6,8,4,9,7,7,6, 5,2,4,1,6,5, 6,1,3,7,4,7,2, 5,2,7,4, 0,1,2,5, 6,8,2,0],  # WELLSPRING
33:[5,8,7,4,4,6,5, 7,3,3,9,4,3, 6,1,5,2,2,6,3, 5,4,5,6, 0,2,2,6, 4,5,3,5],  # CARTELL
34:[8,7,4,9,7,8,6, 3,9,2,1,4,3, 8,4,4,3,2,7,3, 7,2,7,3, 0,1,3,5, 5,6,5,0],  # Quacks
35:[6,7,8,4,5,6,6, 4,4,2,7,7,3, 6,1,3,2,3,6,3, 5,7,5,4, 1,8,2,7, 5,7,5,7],  # SCHISM
36:[7,7,4,5,5,7,5, 7,4,3,2,5,4, 7,2,5,3,8,6,4, 4,2,5,6, 2,1,2,4, 4,6,4,0],  # PERIHELION
37:[5,9,7,4,4,5,6, 9,6,7,3,6,4, 7,2,8,5,5,5,4, 5,6,5,9, 1,2,2,6, 3,5,5,0],  # Brass Bham
38:[8,7,8,5,7,9,5, 2,2,2,4,5,2, 7,1,2,1,1,7,2, 6,1,7,2, 3,6,1,6, 7,5,3,3],  # The Crew
39:[8,6,7,5,6,9,5, 3,2,2,2,4,3, 7,2,3,2,2,7,3, 7,7,7,5, 1,1,2,3, 6,6,3,0],  # TtR
40:[7,7,6,5,8,7,7, 4,3,4,2,6,5, 6,2,4,4,6,6,3, 5,2,7,4, 0,2,2,5, 6,7,2,0],  # 7W
41:[7,7,8,4,6,7,5, 4,4,3,2,5,3, 9,3,4,2,3,7,3, 5,3,7,4, 1,5,2,4, 6,6,3,0],  # QUORUM
42:[4,9,6,4,3,4,6, 7,4,5,3,6,6, 7,3,7,6,7,5,5, 4,5,5,7, 0,1,2,5, 3,5,3,0],  # TM
43:[5,8,4,8,5,6,7, 6,2,4,2,5,4, 5,2,5,5,3,5,4, 4,3,5,5, 0,1,2,5, 5,8,2,0],  # Orléans
44:[6,7,7,5,5,7,5, 7,4,5,2,5,3, 6,2,6,4,3,6,3, 5,7,6,8, 1,2,2,4, 4,5,4,0],  # RIVENER
45:[4,8,8,5,4,6,7, 5,7,3,3,7,3, 7,3,4,3,2,5,8, 7,4,5,6, 8,4,3,6, 5,6,7,0],  # Gloomhaven
46:[7,6,9,4,6,7,7, 1,4,0,5,7,1, 7,1,1,1,1,6,2, 4,1,6,1, 1,9,1,7, 7,7,5,7],  # Secret Hitler
47:[7,7,7,5,5,7,5, 6,5,3,2,5,3, 6,2,5,3,3,7,3, 6,9,6,6, 1,2,2,4, 5,5,4,0],  # TORRENT
48:[6,8,4,7,8,5,7, 5,3,4,3,6,5, 5,2,4,5,5,5,3, 4,2,6,4, 0,2,2,5, 5,8,2,3],  # RftG
49:[8,7,5,4,6,8,5, 3,2,2,1,4,3, 7,6,4,2,2,7,2, 4,3,4,4, 0,1,4,3, 6,5,2,0],  # Patchwork
50:[10,6,4,4,6,8,4, 2,2,2,1,4,3, 9,3,3,1,2,8,2, 4,1,5,2, 0,1,2,3, 6,5,1,0],  # SOLSTICE
51:[6,7,5,5,5,7,5, 8,6,9,1,3,5, 7,1,8,7,2,7,3, 5,3,6,7, 2,1,2,4, 4,5,4,0],  # HAUL
52:[6,8,5,5,5,7,5, 6,9,3,2,4,5, 8,1,4,3,8,7,3, 4,2,6,6, 4,2,3,5, 3,6,8,0],  # CRUCIBLE
53:[7,7,5,5,5,7,5, 8,3,5,8,4,4, 6,1,5,3,4,6,3, 5,5,5,6, 0,2,2,5, 4,5,3,4],  # VIADUCT
54:[8,7,7,6,6,8,5, 2,2,1,4,3,2, 8,2,2,1,3,7,2, 5,1,7,1, 0,2,2,5, 6,6,4,3],  # Ra
55:[7,7,6,5,5,7,5, 3,3,2,5,4,4, 8,2,2,1,2,7,2, 5,1,6,2, 0,2,2,5, 6,6,3,5],  # LEDGER
56:[7,7,3,8,8,7,5, 2,2,5,1,4,2, 5,3,2,1,2,7,5, 4,1,7,2, 0,1,2,4, 6,6,2,0],  # SCROLL
57:[5,9,7,4,4,5,6, 9,6,8,2,5,4, 7,2,8,4,5,5,3, 5,5,5,8, 0,2,2,6, 3,5,5,0],  # Brass Lancs
58:[8,6,8,4,6,8,4, 1,2,1,9,5,1, 7,1,1,0,1,7,2, 4,0,6,1, 1,9,1,7, 5,4,3,9],  # Hanabi
59:[7,6,7,5,8,8,5, 3,3,2,1,4,2, 6,2,2,1,2,7,2, 5,1,9,2, 1,3,1,3, 7,5,2,0],  # CHORUS
60:[6,7,5,5,5,7,6, 8,4,5,2,4,4, 6,2,6,5,4,6,5, 6,3,6,7, 3,2,3,4, 4,5,3,0],  # Viticulture
61:[7,6,4,5,6,7,5, 5,3,4,1,3,5, 7,9,4,3,3,7,2, 4,2,6,5, 1,1,2,4, 6,5,1,0],  # FLOOR
62:[5,8,7,4,4,6,7, 5,5,3,3,9,4, 6,1,4,3,4,6,3, 5,7,5,5, 1,2,2,4, 5,7,4,0],  # FRACTURE
63:[8,7,7,4,5,7,5, 4,3,3,2,5,3, 6,2,4,2,3,7,3, 6,9,6,5, 0,1,3,4, 5,5,3,0],  # MERIDIAN
64:[6,8,5,5,5,7,6, 7,9,3,2,4,5, 8,1,4,3,9,7,3, 4,2,6,6, 3,2,3,5, 3,6,8,0],  # CAULDRON
65:[5,9,6,4,4,5,6, 9,5,9,2,5,4, 7,2,8,4,4,5,3, 4,4,5,8, 1,2,2,6, 3,5,5,0],  # Le Havre
66:[5,7,8,5,4,6,7, 6,9,2,3,7,2, 7,1,3,2,2,5,3, 7,3,5,4, 7,4,3,6, 6,7,9,0],  # Spirit Island
67:[6,7,5,5,5,7,5, 7,5,7,1,4,3, 7,2,9,5,3,6,3, 5,3,5,7, 2,1,2,5, 4,5,5,0],  # DELUGE
68:[7,5,6,7,5,8,4, 2,3,1,1,3,2, 5,1,6,1,2,6,2, 5,6,7,3, 1,3,4,5, 3,3,2,0],  # Catan
69:[7,7,5,8,6,7,5, 3,3,5,1,5,4, 6,1,2,1,2,7,4, 4,1,7,2, 0,1,2,4, 6,7,2,0],  # MATRIX
}

# ── Load fast-track records for games #70-150 ────────────────────────────────
import re

def parse_record(txt):
    s = {}
    for ax in AXES:
        m = re.search(rf'\|{ax}\|(\d+)\|', txt)
        if m: s[ax] = int(m.group(1))
    # A29-A32 may not be in old records; default 5 (neutral)
    for ax in ['A29','A30','A31','A32']:
        if ax not in s: s[ax] = 5
    return s

rows_out = []

# Games 1-69 from RESCORE dict
for gnum, scores in RESCORE.items():
    if len(scores) != len(AXES):
        print(f'WARNING: game {gnum} has {len(scores)} scores, expected {len(AXES)}')
        continue
    row = {'game_num': str(gnum), 'type': 'rescored'}
    for i, ax in enumerate(AXES):
        row[ax] = scores[i]
    rows_out.append(row)

# Games 70-150 from record.md
for d in sorted(os.listdir('games')):
    m = re.match(r'0*(\d+)-', d)
    if not m: continue
    gnum = int(m.group(1))
    if gnum < 70 or gnum > 150: continue
    rec = f'games/{d}/record.md'
    if not os.path.exists(rec): continue
    with open(rec, encoding='utf-8', errors='ignore') as f: txt = f.read()
    scores = parse_record(txt)
    row = {'game_num': str(gnum), 'type': 'fast-track'}
    for ax in AXES:
        row[ax] = scores.get(ax, 0)
    rows_out.append(row)

rows_out.sort(key=lambda r: int(r['game_num']))
print(f'Total rows: {len(rows_out)}')

# ── Write v2 matrix ───────────────────────────────────────────────────────────
cols = ['game_num','type'] + AXES
with open('data/axis-matrix-v2.csv', 'w', encoding='utf-8', newline='') as f:
    w = csv.DictWriter(f, fieldnames=cols, extrasaction='ignore')
    w.writeheader()
    w.writerows(rows_out)

# ── Weight-normalized residuals ───────────────────────────────────────────────
# Load weights from v1 matrix
weights = {}
with open('data/axis-matrix.csv', encoding='utf-8') as f:
    for row in csv.DictReader(f):
        try: weights[row['game_num']] = float(row['weight'] or 2.5)
        except: weights[row['game_num']] = 2.5

matrix = []
game_nums = []
wts = []
for row in rows_out:
    gn = row['game_num']
    vec = [float(row.get(ax, 0)) for ax in AXES]
    matrix.append(vec)
    game_nums.append(gn)
    wts.append(weights.get(gn, 2.5))

n, p = len(matrix), len(matrix[0])

# For each axis, fit linear regression on weight, compute residuals
def linreg(x, y):
    n = len(x); xm = sum(x)/n; ym = sum(y)/n
    b = sum((x[i]-xm)*(y[i]-ym) for i in range(n)) / (sum((x[i]-xm)**2 for i in range(n)) or 1)
    a = ym - b*xm
    return a, b

resid = [[0.0]*p for _ in range(n)]
for j in range(p):
    col = [matrix[i][j] for i in range(n)]
    a, b = linreg(wts, col)
    for i in range(n):
        resid[i][j] = matrix[i][j] - (a + b*wts[i])

with open('data/axis-residuals-v2.csv', 'w', encoding='utf-8', newline='') as f:
    w = csv.writer(f)
    w.writerow(['game_num'] + AXES)
    for i, gn in enumerate(game_nums):
        w.writerow([gn] + [round(resid[i][j],3) for j in range(p)])

print('Written: data/axis-matrix-v2.csv (raw rescored)')
print('Written: data/axis-residuals-v2.csv (weight-normalized residuals)')
