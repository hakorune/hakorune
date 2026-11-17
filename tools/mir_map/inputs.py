"""Fixed-input equivalence and per-input coverage collection."""
from pathlib import Path
import concurrent.futures
import hashlib
import json
import os
from support import ROOT, configured_env, fixed_inputs, run, sha, write


def normalized(log, binary):
    return Path(log).read_bytes().replace(os.fsencode(str(binary)), b'$COMPILER')


def preflight(work, manifest, template):
    rows, _ = fixed_inputs(manifest)
    base, measured = work / 'hakorune.base', work / 'target/quick/hakorune'
    selected = [rows[i * (len(rows) - 1) // 199] for i in range(200)]
    folder = work / 'preflight'
    folder.mkdir(exist_ok=True)
    identity = dict(base=sha(base), measured=sha(measured), manifest=sha(manifest),
                    selected=[r['path'] for r in selected])
    write(folder / 'identity.json', identity)

    def one(pair):
        index, row = pair
        directory = folder / str(index)
        directory.mkdir(exist_ok=True)
        runs = []
        for binary, label in [(base, 'base'), (measured, 'coverage')]:
            env = configured_env(template, binary, work)
            env['LLVM_PROFILE_FILE'] = str(directory / 'profile-%p-%m.profraw')
            runs.append(run([binary, '--dump-mir', row['path']], directory / (label + '.log'), env))
        equal = (runs[0]['exit'], runs[0]['timeout'], normalized(runs[0]['log'], base)) == (
            runs[1]['exit'], runs[1]['timeout'], normalized(runs[1]['log'], measured))
        for raw in directory.glob('*.profraw'):
            raw.unlink()
        return dict(source=row['path'], source_sha256=row['sha256'], equal=equal, runs=runs)

    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        results = list(pool.map(one, enumerate(selected)))
    write(folder / 'results.json', results)
    differences = [r['source'] for r in results if not r['equal']]
    write(folder / 'summary.json', dict(total=len(results), differences=differences, identity=identity))
    print('equivalence', len(results), 'differences', len(differences), flush=True)
    if differences:
        raise RuntimeError('measurement CLI differs; stop, do not alter compiler sources')
