"""Numerical runtime map. Physical source lines and uninstrumented remain separate."""
from pathlib import Path
import json
import shutil
from aggregate import summarize
from catalog import ensure as catalogue
from support import ROOT, write, sha


def render(work, reuse, write_docs=False):
    catalogue(reuse / 'probe/coverage.json', work / 'inventory.json', work / 'cli-catalog.json')
    test_catalog = work / 'test-catalog.json'
    test_membership = work / 'T/membership.json.gz'
    status = {}
    if (work / 'T-status.json').exists():
        status = json.loads((work / 'T-status.json').read_text())
        if status.get('measured'):
            assert sha(work / 'T/membership.json.gz') == status['membership_sha256']
            assert sha(work / 'T/all.lcov') == status['lcov_sha256']
            assert sha(status['binary']) == status['binary_sha256']
            assert sha(reuse / 'target/quick/hakorune') == status['cli_sha256']
            catalogue(work / 'T/coverage.json', work / 'inventory.json', test_catalog)
    measured = bool(status.get('measured')) and test_catalog.exists() and test_membership.exists()
    data = summarize(work, reuse, test_catalog if measured else None, test_membership if measured else None)
    out = ROOT / 'tools/mir_map/out'
    out.mkdir(exist_ok=True)
    for name in ['runtime-summary.json', 'functions.json', 'input-function-sets.json',
                 'refusal-summary.json', 'refusal-catalogue.json']:
        if (work / name).exists():
            shutil.copy2(work / name, out / name)
    graph = json.loads((reuse / 'graph.json').read_text())
    cyclic = {g for cycle in graph['cycles'] for g in cycle['groups']}
    if write_docs:
        table = ['| 群 | 本体行(計測artifact実行可能) | 責務 | A入力 | B file | A行 | Bのみ行 | testのみ行 | 実測0行 | 入/出(HIR下限) | HIR輪 |',
                 '|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---|']
        for group in data['groups'][:60]:
            name = group['group']
            deps = graph['groups'].get(name)
            numbers = f"{deps['incoming']}/{deps['outgoing']}" if deps else '未測定'
            def count(value):
                return '未測定' if value is None else str(value)
            a = count(group['A_inputs']) if data['A']['full'] or group['A_inputs'] is None else '>=' + str(group['A_inputs'])
            b = count(group['B_inputs']) if data['B']['full'] or group['B_inputs'] is None else '>=' + str(group['B_inputs'])
            cycle = '検出' if name in cyclic else '未確定'
            table.append(f"| `{name}` | {group['instrumented_lines']} | | {a} | {b} | {group['A_lines']} | {group['B_only_lines']} | {count(group['T_only_lines'])} | {count(group['known_zero_lines'])} | {numbers} | {cycle} |")
        (ROOT / 'src/mir/README.md').write_text('\n'.join(table) + '\n')
    return data
