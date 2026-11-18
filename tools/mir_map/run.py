#!/usr/bin/env python3
# Measure-1b entry: no HIR/THIR dump is executed.
# python3 tools/mir_map/run.py --work <work> \
#  --reuse <reuse> --stage all \
#  --manifest <manifest> --template <template> \
#  --expected-failures <expected-failures> --write-docs
# Existing SHA-verified coverage CLI and its 200-input equivalence receipt are reused.
# Resource watch starts before child commands: MemAvailable6GiB/RSS16GiB/disk20GB.
# M1 uses at most4 commands; M3 build2jobs, one guarded retry1job/CGU64; tests1thread.
# Reserve final45min of6h. --stage render reads held data without build/test/input runs.
# 2026-11-17 actual: M1 A91.1min+B21.5min; M3 build13.4min+test115.9s.
# Command/resource logs preserve timings; no HIR/THIR dump or compiler source edit.
import argparse
import json
from pathlib import Path
import subprocess
import sys
import time
from support import ROOT, write, sha


def prepare(work, reuse):
    sys.path.insert(0, str(reuse / 'python'))
    from inventory import collect
    from catalog import ensure as catalogue
    identity = json.loads((reuse / 'preflight/summary.json').read_text())
    binary = reuse / 'target/quick/hakorune'
    if identity['differences'] or sha(binary) != identity['identity']['measured']:
        raise RuntimeError('coverage CLI equivalence receipt no longer matches')
    subprocess.run(['git', 'diff', '--exit-code', '46457de489', '--', 'src', 'lang', 'crates',
                    ':(exclude)src/mir/README.md'], cwd=ROOT, check=True)
    if not (work / 'inventory.json').exists():
        collect(work / 'inventory.json')
    catalogue(reuse / 'probe/coverage.json', work / 'inventory.json', work / 'cli-catalog.json')
    return binary


def finish(work, reuse, write_docs):
    from refusals import catalogue, summarize
    from render_runtime import render
    path = work / 'refusal-catalogue.json'
    catalog = json.loads(path.read_text()) if path.exists() else catalogue(
        json.loads((work / 'inventory.json').read_text()), path)
    summarize(work, catalog)
    return render(work, reuse, write_docs)


def main():
    p = argparse.ArgumentParser(description='Guarded M1/M3 coverage; held HIR lower-bound map only')
    p.add_argument('--work', type=Path, required=True)
    p.add_argument('--reuse', type=Path, required=True)
    p.add_argument('--stage', choices=['all', 'coverage', 'test', 'render'], default='all')
    p.add_argument('--manifest', type=Path)
    p.add_argument('--template', type=Path)
    p.add_argument('--expected-failures', type=Path)
    p.add_argument('--baseline-results', type=Path)
    p.add_argument('--write-docs', action='store_true')
    a = p.parse_args()
    work, reuse = a.work.resolve(), a.reuse.resolve()
    work.mkdir(parents=True, exist_ok=True)
    if a.stage == 'render':
        sys.path.insert(0, str(reuse / 'python'))
        finish(work, reuse, a.write_docs)
        return
    from resources import activate
    watch = activate(work, 'prepare')
    binary = prepare(work, reuse)
    start_path = work / 'start.json'
    if not start_path.exists():
        write(start_path, dict(created_at=time.time(), authorization='measure1b coverage/test only'))
    deadline = json.loads(start_path.read_text())['created_at'] + 5.25 * 3600
    if a.template is None:
        p.error('--template is required for measured stages')
    try:
        if a.stage in ['all', 'coverage']:
            if a.manifest is None:
                p.error('--manifest is required')
            from batch import collect
            for group in ['A', 'B']:
                watch.stage = 'M1-' + group
                result = collect(work, binary, a.manifest, a.template, group, deadline, a.baseline_results, normal_binary=reuse / 'hakorune.base')
                if not result['full']:
                    finish(work, reuse, a.write_docs)
                    return
        if a.stage in ['all', 'test']:
            for group in ['A', 'B']:
                if not (work / group / 'summary.json').exists() or not json.loads((work / group / 'summary.json').read_text())['full']:
                    raise RuntimeError('finish M1 before building/running M3')
            if a.expected_failures is None:
                p.error('--expected-failures is required for M3')
            target = work / 'target'
            if not target.exists():
                target.symlink_to(reuse / 'target', target_is_directory=True)
            from aggregate import inputs
            catalog = json.loads((work / 'cli-catalog.json').read_text())
            for group in ['A', 'B']:
                trace = inputs(work / group, catalog)
                if not trace['full']:
                    write(work / 'M1-trace-limit.json', dict(group=group, missing=trace['missing_profiles'], partial=trace['partial_profiles']))
                    print('M1 has incomplete profiles; unknowns remain explicit', flush=True)
            from test_measure import measure
            measure(work, a.template, a.expected_failures, deadline)
    except (MemoryError, RuntimeError) as error:
        write(work / 'phase-stop.json', dict(stage=watch.stage, reason=str(error)))
        finish(work, reuse, a.write_docs)
        raise
    finish(work, reuse, a.write_docs)


if __name__ == '__main__':
    main()
