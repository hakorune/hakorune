"""Bounded M-1 collection. Keep per-input membership; discard profiles immediately."""
from pathlib import Path
import concurrent.futures
import gzip
import json
import os
import re
import shutil
import time
from support import ROOT, configured_env, fixed_inputs, run, sha, write
from coverage import merge, export, read_lcov
from resources import active


def result_kind(run_result, text):
    if run_result['timeout']:
        return dict(kind='timeout', first_refusal=None)
    if run_result['exit'] == 0:
        return dict(kind='success', first_refusal=None)
    terminal = [line for line in text.splitlines() if re.search(
        r'(?:MIR compilation error:|\bError:|\berror:|Parse error|Parser error)', line, re.I)]
    line = terminal[0] if terminal else ''
    parser = bool(re.search(r'(?:parse|parser|syntax).{0,25}(?:error|failed)|expected .*found', line, re.I))
    tags = re.search(r'(\[[^\]\n]+\](?:\[[^\]\n]+\])*)', line)
    return dict(kind='parser_refusal' if parser else 'refusal', first_refusal=tags[1] if tags else None,
                terminal=line)


def one(work, binary, template, folder, index, row, emit_exe, timeout=600, baseline=None, normal_binary=None):
    destination = folder / f'{index:04d}'
    destination.mkdir(parents=True, exist_ok=True)
    record = destination / 'result.json'
    if record.exists():
        return json.loads(record.read_text())
    if active().failure:
        raise MemoryError('batch canceled by resource guard')
    for path in destination.glob('*.profraw'):
        path.unlink()
    env = configured_env(template, binary, work)
    env['LLVM_PROFILE_FILE'] = str(destination / 'profile-%p-%m.profraw')
    commands = [run([binary, '--dump-mir', row['path']], destination / 'dump.log', env, timeout)]
    comparison = None
    if baseline and row['path'] in baseline and not commands[0]['timeout']:
        expected = baseline[row['path']]
        normalized = Path(commands[0]['log']).read_bytes().replace(os.fsencode(str(binary)), b'$COMPILER')
        matched = (commands[0]['exit'] == expected['dump']['exit'] and
                   __import__('hashlib').sha256(normalized).hexdigest() == expected['dump']['output_sha256'])
        comparison = dict(held_baseline_equal=matched)
        if not matched:
            if normal_binary is None:
                raise ValueError('normal comparison binary must be supplied from --reuse')
            normal = Path(normal_binary)
            normal_env = configured_env(template, normal, work)
            normal_env.pop('LLVM_PROFILE_FILE', None)
            control = run([normal, '--dump-mir', row['path']], destination / 'normal-reference.log', normal_env, timeout)
            left = Path(control['log']).read_bytes().replace(os.fsencode(str(normal)), b'$COMPILER')
            current_equal = (control['exit'], control['timeout'], left) == (commands[0]['exit'], False, normalized)
            comparison.update(current_normal_equal=current_equal, control=control)
            if not current_equal:
                write(destination / 'equivalence-failure.json', row | comparison | dict(measured=commands[0]))
                raise RuntimeError(f"normal/coverage CLI difference: {row['path']}")
    emitted = False
    if emit_exe:
        exe = destination / 'program.exe'
        commands.append(run([binary, '--backend', 'mir', '--emit-exe', exe, '--emit-exe-nyrt',
                             env['NYASH_EMIT_EXE_NYRT'], row['path']], destination / 'compile.log', env, timeout))
        emitted = exe.exists()
        exe.unlink(missing_ok=True)
    counts, lines, universe = {}, {}, {}
    raws = list(destination.glob('*.profraw'))
    if raws:
        profile, lcov = destination / 'input.profdata', destination / 'input.lcov'
        merge(raws, profile)
        export(binary, profile, lcov, format='lcov')
        counts, lines, universe = read_lcov(lcov)
        with gzip.open(destination / 'membership.json.gz', 'wt') as stream:
            json.dump(dict(functions=sorted(counts), lines=lines), stream, separators=(',', ':'))
        # Logical invocation counts intentionally omitted under the 1b membership contract.
        profile.unlink()
        lcov.unlink()
        for path in raws:
            path.unlink()
    assert sha(row['path']) == row['sha256'], 'input changed during measurement'
    result = row | dict(commands=commands, emitted=emitted, comparison=comparison, profile_available=bool(raws),
                        functions_observed=len(counts), instrumented_lines=sum(map(len, universe.values())),
                        complete_trace=bool(raws) and not any(c['timeout'] for c in commands),
                        outcomes=[result_kind(c, Path(c['log']).read_text(errors='replace')) for c in commands])
    write(record, result)
    return result


def collect(work, binary, manifest, template, group, deadline, baseline_path=None, normal_binary=None):
    rows, exes = fixed_inputs(manifest)
    baseline = {r['source']:r for r in json.loads(Path(baseline_path).read_text())} if baseline_path else None
    if group == 'B':
        rows = [dict(path=str(p.resolve()), sha256=sha(p)) for p in sorted((ROOT / 'lang/src').rglob('*.hako'))]
        assert len(rows) == 1192
    folder = work / group
    folder.mkdir(exist_ok=True)
    identity = dict(binary=sha(binary), rows=rows, reader=sha(Path(__file__).with_name('coverage.py')),
                    collector=sha(__file__), template=sha(template), supervisor=sha(Path(__file__).with_name('resources.py')), group=group, timeout_seconds=600)
    if (folder / 'identity.json').exists():
        assert json.loads((folder / 'identity.json').read_text()) == identity, 'changed batch cannot resume silently'
    else:
        write(folder / 'identity.json', identity)
    pool = concurrent.futures.ThreadPoolExecutor(max_workers=4)
    pending, next_index, finished = {}, 0, 0
    try:
        while pending or next_index < len(rows):
            while len(pending) < 4 and next_index < len(rows) and time.time() < deadline:
                row = rows[next_index]
                future = pool.submit(one, work, binary, template, folder, next_index, row,
                                     group == 'A' and row['path'] in exes, 600, baseline, normal_binary)
                pending[future] = next_index
                next_index += 1
            if not pending:
                break
            ready, _ = concurrent.futures.wait(pending, return_when=concurrent.futures.FIRST_COMPLETED)
            for future in ready:
                future.result()
                pending.pop(future)
                finished += 1
                if finished % 24 == 0:
                    print(group, finished, '/', len(rows), flush=True)
            if active().failure:
                raise MemoryError('M-1 resource stop; no more input commands')
    except BaseException:
        pool.shutdown(wait=True, cancel_futures=True)
        raise
    else:
        pool.shutdown(wait=True)
    summary = dict(group=group, total=len(rows), completed=finished, full=finished == len(rows))
    write(folder / 'summary.json', summary)
    print(summary, flush=True)
    return summary
