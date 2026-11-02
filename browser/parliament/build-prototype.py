"""Build the pilot, preserve native controls, stage an executable project-prefix site.

Source/legal packaging and hosted/live qualification are separate release gates.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import tomllib

parser = argparse.ArgumentParser()
parser.add_argument('--bindgen', required=True, type=Path)
args = parser.parse_args()
B = Path(__file__).resolve().parent
R = B.parents[1]
C = B / '.cache'
C.mkdir(exist_ok=True)
env = os.environ.copy()
target = Path(env.get('CARGO_TARGET_DIR', R / 'target/cargo')).resolve()
env['CARGO_TARGET_DIR'] = str(target)
env['CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS'] = '-C link-arg=--max-memory=67108864'

def run(argv, **kwargs):
    return subprocess.run([str(x) for x in argv], cwd=R, env=env, check=True, **kwargs)

assert subprocess.check_output([str(args.bindgen.resolve()), '--version']).decode().strip() == 'wasm-bindgen 0.2.127'
# Audit shared package identities; companion-only additions/removals are recorded.
native = tomllib.loads((R/'tools/tigris-sim/Cargo.lock').read_text(encoding='utf-8'))['package']
adapter = tomllib.loads((B/'Cargo.lock').read_text(encoding='utf-8'))['package']
def identities(packages):
    return {(p['name'], p['version']): (p.get('source'), p.get('checksum')) for p in packages}
n, a = identities(native), identities(adapter)
native_names = {p['name'] for p in native}
for key in a:
    if key[0] in native_names:
        assert key in n, ('Shared package version drift', key)
for key in n.keys() & a.keys():
    assert n[key] == a[key], ('Shared package source drift', key)
for package in adapter:
    if package['name'].startswith('muddle-'):
        assert package['source'] == 'git+https://github.com/giodl73-repo/MUDDLE.git?rev=08aea37b9849a2e4ec69ea3786efbc09ba57f8d1#08aea37b9849a2e4ec69ea3786efbc09ba57f8d1'
(C/'dependency-identities.json').write_text(json.dumps({'shared_identical': sorted('/'.join(k) for k in n.keys() & a.keys()), 'companion_only': sorted('/'.join(k) for k in a.keys() - n.keys())}, indent=2)+'\n', encoding='utf-8')
for extra in [[], ['--target', 'wasm32-unknown-unknown', '--lib']]:
    run(['cargo', '+1.95.0', 'build', '--locked', '--manifest-path', B/'Cargo.toml', '--release', *extra])
run(['cargo', '+1.95.0', 'build', '--locked', '--manifest-path', R/'tools/tigris-sim/Cargo.toml', '--bin', 'tigris-muddle'])
site = R/'target/parliament-prototype/TIGRIS'
site.mkdir(parents=True, exist_ok=True)
shutil.copytree(B/'web', site, dirs_exist_ok=True)
run([args.bindgen.resolve(), target/'wasm32-unknown-unknown/release/tigris_parliament_workbench.wasm', '--target', 'web', '--out-dir', site/'pkg'])
for file in (site/'pkg').glob('*.d.ts'):
    file.unlink()
suffix = '.exe' if os.name == 'nt' else ''
control = target/('release/tigris-parliament-control'+suffix)
cli = target/('debug/tigris-muddle'+suffix)
def evaluate(request):
    raw = request if isinstance(request, str) else json.dumps(request, ensure_ascii=False, separators=(',', ':'))
    response = json.loads(run([control], input=raw.encode(), capture_output=True).stdout)
    return {'request': raw, 'response': response}

paths = [
    ['look', 'inspect table', 'choose persona', 'look', 'go board', 'inspect board', 'draft axis', 'stake claim', 'inspect collision', 'reveal collision', 'place tiger', 'end turn', 'inspect pressure', 'challenge ai', 'go score', 'inspect ledger', 'score amendment', 'close parliament'],
    ['go score', 'close parliament', 'choose persona', 'go board', 'stake claim', 'draft axis', 'stake claim', 'reveal collision', 'place tiger', 'end turn', 'go score', 'score amendment', 'close parliament'],
    ['not a command', 'go board', 'draft axis', 'place tiger', 'end turn', 'end turn', 'challenge ai', 'status'],
]
records, pairs = [], []
for sequence in paths:
    for length in sorted({0, 1, len(sequence)//2, len(sequence)}):
        commands = sequence[:length]
        case = evaluate({'commands': commands})
        assert case['response']['kind'] == 'report'
        records.append(case)
        with tempfile.TemporaryDirectory(prefix='tigris original cli ') as tmp:
            tmp = Path(tmp)
            run([cli, '--save', tmp/'native.save', '--transcript', tmp/'native.txt'], input=('\n'.join(commands+['quit'])+'\n').encode(), stdout=subprocess.DEVNULL)
            for filename, field in [('native.save', 'save_text'), ('native.txt', 'transcript_text')]:
                data = (tmp/filename).read_bytes()
                assert data == case['response'][field].encode(), (commands, field)
            pairs.append({'commands': commands, 'save_sha256': hashlib.sha256((tmp/'native.save').read_bytes()).hexdigest(), 'transcript_sha256': hashlib.sha256((tmp/'native.txt').read_bytes()).hexdigest()})
        for field in ['save_text', 'replay_save_text']:
            records.append(evaluate({'save_text': case['response'][field], 'commands': ['look']}))
initial = evaluate({})['response']['save_text']
for request in ['{', 'null', '{"commands":[],"commands":[]}', {'unknown': True}, {'commands': ['<script>alert(1)</script>', 'look']}, {'commands': ['look']*128}, {'commands': ['look']*129}, {'commands': ['é'*513]}, {'save_text': 'broken'}, {'save_text': 'x'*131073}, {'save_text':'é'*300000}, ' '*524289]:
    records.append(evaluate(request))
for field, value in [('round', '4294967295'), ('round', '4096'), ('round', '4097'), ('human_score', '-2147483648'), ('ai_pressure', '4097')]:
    # Native field encoding escapes semicolons/equal signs in its checkpoint line.
    import re
    replacement = re.sub(rf'{field}%3D[^;\n]*', f'{field}%3D{value}', initial)
    assert replacement != initial, (field, initial)
    case = evaluate({'save_text': replacement})
    assert case['response']['kind'] == ('report' if value == '4096' else 'browser_refusal')
    records.append(case)
for case in records:
    assert evaluate(case['request']) == case
(C/'native-cases.json').write_text(json.dumps(records, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
(C/'native-controls.json').write_text(json.dumps({'native_wasm_cases': len(records), 'each_native_repeated': 2, 'independent_cli_pairs': pairs}, indent=2)+'\n', encoding='utf-8')
print(json.dumps({'staged': str(site), 'native_cases': len(records), 'independent_cli_pairs': len(pairs)}))
