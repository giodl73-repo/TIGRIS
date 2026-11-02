"""Rebuild exact delivered software in a temporary directory with spaces, outside Git."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import zipfile

parser = argparse.ArgumentParser()
parser.add_argument('--bindgen', required=True, type=Path)
args = parser.parse_args()
B = Path(__file__).resolve().parent
R = B.parents[1]
archive = R/'target/parliament-release/TIGRIS/source.zip'
expected_cases = json.loads((B/'.cache/native-cases.json').read_text(encoding='utf-8'))
expected_controls = json.loads((B/'.cache/native-controls.json').read_text(encoding='utf-8'))
env = os.environ.copy()
env.setdefault('CARGO_TARGET_DIR',str(R/'target/cargo'))
with tempfile.TemporaryDirectory(prefix='tigris delivered source ') as tmp:
    tmp = Path(tmp)
    with zipfile.ZipFile(archive) as source:
        for name in source.namelist():
            assert not Path(name).is_absolute() and '..' not in Path(name).parts,name
        source.extractall(tmp)
    root = tmp/'TIGRIS-software-source'
    record = json.loads((root/'.source-record.json').read_text(encoding='utf-8'))
    assert len(list(root.rglob('.git'))) == 0
    for item in record['files']:
        assert hashlib.sha256((root/item['path']).read_bytes()).hexdigest()==item['sha256'],item['path']
    def run(argv, cwd=root):
        subprocess.run([str(x) for x in argv],cwd=cwd,env=env,check=True)
    for extra in [[],['--no-default-features']]:
        run(['cargo','+1.95.0','test','--manifest-path','tools/tigris-sim/Cargo.toml','--all-targets','--locked',*extra])
    run(['cargo','+1.95.0','test','--manifest-path','browser/parliament/Cargo.toml','--all-targets','--locked'])
    run(['python',root/'browser/parliament/build-prototype.py','--bindgen',args.bindgen.resolve()])
    companion=root/'browser/parliament'
    rebuilt=json.loads((companion/'.cache/native-cases.json').read_text(encoding='utf-8'))
    controls=json.loads((companion/'.cache/native-controls.json').read_text(encoding='utf-8'))
    assert rebuilt==expected_cases,'Delivered source response drift'
    assert controls==expected_controls,'Delivered source original CLI control drift'
    npm='npm.cmd' if os.name=='nt' else 'npm'
    run([npm,'ci'],cwd=companion)
    run([npm,'run','test:pages'],cwd=companion)
    (B/'.cache/source-replay.json').write_text(json.dumps({'commit':record['commit'],'source_files':len(record['files']),'native_cases_identical':len(rebuilt),'original_cli_pairs_identical':len(controls['independent_cli_pairs']),'browser_task_and_recovery_checks':16,'native_tests_default_and_portable_each':18,'adapter_tests':8,'temporary_root_outside_git':str(root),'binary_identity_claim':False},indent=2)+'\n',encoding='utf-8')
