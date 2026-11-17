"""Fixed-input equivalence and per-input coverage collection."""
from pathlib import Path
import concurrent.futures
import hashlib
import json
import os
from support import ROOT, configured_env, fixed_inputs, run, sha, write
from coverage import save_counts


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


def corpus(work, manifest, template, group):
    measured = work / 'target/quick/hakorune'
    rows, executables = fixed_inputs(manifest)
    if group == 'B':
        rows = [dict(path=str(p.resolve()), sha256=sha(p)) for p in sorted((ROOT / 'lang/src').rglob('*.hako'))]
        assert len(rows) == 1192
    folder = work / group
    folder.mkdir(exist_ok=True)
    identity = dict(binary=sha(measured), manifest=sha(manifest), rows=rows,
                    reader=sha(Path(__file__).with_name('coverage.py')), runner=sha(__file__))
    identity_path = folder / 'identity.json'
    if identity_path.exists():
        assert json.loads(identity_path.read_text()) == identity, 'cannot resume changed measurement'
    else:
        write(identity_path, identity)

    def one(pair):
        index, row = pair
        directory = folder / f'{index:04d}'
        directory.mkdir(exist_ok=True)
        destination = directory / 'result.json'
        if destination.exists():
            return json.loads(destination.read_text())
        # A failed/incomplete attempt is not part of this input's completed record.
        for raw in directory.glob('profile-*.profraw'):
            raw.unlink()
        env = configured_env(template, measured, work)
        env['LLVM_PROFILE_FILE'] = str(directory / 'profile-%p-%m.profraw')
        runs = [run([measured, '--dump-mir', row['path']], directory / 'dump.log', env)]
        if group == 'A' and row['path'] in executables:
            exe = directory / 'program.exe'
            runs.append(run([measured, '--backend', 'mir', '--emit-exe', exe,
                '--emit-exe-nyrt', env['NYASH_EMIT_EXE_NYRT'], row['path']], directory / 'compile.log', env))
            exe.unlink(missing_ok=True)
        assert sha(row['path']) == row['sha256'], 'input changed during measurement'
        coverage = save_counts(directory, directory / 'coverage', measured)
        result = row | dict(runs=runs, coverage=coverage)
        write(destination, result)
        for raw in directory.glob('*.profraw'):
            raw.unlink()
        return result

    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        for index, result in enumerate(pool.map(one, enumerate(rows)), 1):
            if index % 24 == 0 or index == len(rows):
                print(group, index, '/', len(rows), flush=True)
    write(folder / 'complete.json', dict(inputs=len(rows), identity=identity))
