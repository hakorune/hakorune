#!/usr/bin/env python3
# Reproduce: python3 tools/mir_map/run.py --work /mnt/workdisk/claude-scratch/measure-1 --stage all \
#   --manifest /path/frozen-inputs.json --template /path/configured-environment.json \
#   --baseline /path/normal-hakorune --expected-failures /path/155-names.txt
# Time: coverage CLI 8m19s, 200-input equivalence ~4m, forced warnings1m44s, HIR6m7s.
# THIR failed with OOM on28GiB: later A/B/test phases were NOT run and their time is unknown.
# This is a partial HIR map, NOT a complete typed-method dependency graph.
# --stage render reuses held data without builds/runs; --write-docs writes README table.
# Outputs: tools/mir_map/out/ (ignored), raw records/isolated target under --work.
import argparse
import os
from pathlib import Path
import shutil
import subprocess
import sys
from support import ROOT, write


def main():
    parser = argparse.ArgumentParser(description='Reproducible MirBuilder measurements; resource stops remain explicit')
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--stage', choices=['all', 'render'], default='all')
    parser.add_argument('--manifest', type=Path)
    parser.add_argument('--template', type=Path)
    parser.add_argument('--baseline', type=Path)
    parser.add_argument('--expected-failures', type=Path)
    parser.add_argument('--write-docs', action='store_true')
    args = parser.parse_args()
    work = args.work.resolve()
    work.mkdir(parents=True, exist_ok=True)
    sys.path.insert(0, str(work / 'python'))
    if args.stage == 'all' and (work / 'resource-stop.json').exists():
        raise MemoryError('This run hit a resource stop. Use --stage render for held evidence; do not restart heavy stages.')
    if args.stage == 'render':
        from reporting import render
        render(work, args.write_docs)
        return
    for key in ['manifest', 'template', 'baseline', 'expected_failures']:
        if getattr(args, key) is None:
            parser.error('--' + key.replace('_', '-') + ' is required for all')
    subprocess.run([sys.executable, '-m', 'pip', 'install', '--target', str(work / 'python'),
                    '-r', str(Path(__file__).with_name('requirements.txt'))], check=True)
    shutil.copy2(args.baseline, work / 'hakorune.base')
    from compiler import build_cli, dumps, test
    from inputs import preflight, corpus
    from inventory import collect
    from unused import collect as warnings
    from dependencies import extract, graph
    from reporting import render
    build_cli(work)
    preflight(work, args.manifest, args.template)
    # Compiler dumps and the test build are serialized with all measurements.
    try:
        dumps(work)
    except (RuntimeError, MemoryError) as error:
        write(work / 'resource-stop.json', dict(reason=str(error), A='not measured', B='not measured', T='not measured'))
        # Preserve the completed HIR, if available. Do not fabricate missing typed references.
        if (work / 'hir.command.json').exists() and (work / 'hir.txt').exists():
            collect(work / 'inventory.json')
            warnings(work, __import__('json').loads((work / 'inventory.json').read_text()))
            write(work / 'graph.json', graph(extract(work)))
            render(work, args.write_docs)
        raise
    inventory = collect(work / 'inventory.json')
    warnings(work, inventory)
    write(work / 'graph.json', graph(extract(work)))
    test(work, args.template, args.expected_failures)
    for group in ['A', 'B']:
        corpus(work, args.manifest, args.template, group)
    # Current renderer intentionally reports the missing aggregate/M3 analyses as unmeasured.
    # A successful run is not sufficient to claim those unfinished analyses complete.
    render(work, args.write_docs)


if __name__ == '__main__':
    main()
