"""Render held evidence; an absent measurement always remains null, never zero."""
from collections import defaultdict
from pathlib import Path
import json
import os
from support import ROOT, write


def render(work, write_docs=False):
    inventory = json.loads((work / 'inventory.json').read_text())
    graph = json.loads((work / 'graph.json').read_text()) if (work / 'graph.json').exists() else None
    unused = json.loads((work / 'unused.json').read_text()) if (work / 'unused.json').exists() else None
    groups = {}
    for file, values in inventory['files'].items():
        group = groups.setdefault(values['group'], dict(group=values['group'], physical_production=0,
            physical_test=0, syntax_errors=[], A_inputs=None, B_inputs=None, never_lines=None, test_only_lines=None,
            incoming_HIR=None, outgoing_HIR=None, used_items_HIR=None, detected_HIR_cycle=None))
        group['physical_production'] += values['production_lines']
        group['physical_test'] += len(values['test_lines'])
        if values['syntax_error']:
            group['syntax_errors'].append(file)
    if graph:
        cyclic = {group for cycle in graph['cycles'] for group in cycle['groups']}
        for name, group in groups.items():
            measured = graph['groups'].get(name)
            if measured:
                group.update(incoming_HIR=measured['incoming'], outgoing_HIR=measured['outgoing'],
                             used_items_HIR=measured['used_items'])
            if name in cyclic:
                group['detected_HIR_cycle'] = True
    rows = sorted(groups.values(), key=lambda x: (-x['physical_production'], x['group']))
    result = dict(status='PARTIAL: resource stop; A/B/test coverage not measured', groups=rows,
                  units='physical production/test lines by explicit syntax cfg masks; syntax-error groups uncertain; HIR item references exclude namespace prefix references',
                  M1='not measured (200 equivalence probes are not corpus coverage)', M2='forced diagnostics collected; unmatched spans retained',
                  M3='not measured', M4='HIR lower bound only; THIR OOM, typed dispatch unmeasured')
    out = ROOT / 'tools/mir_map/out'
    out.mkdir(exist_ok=True)
    write(out / 'groups.json', result)
    functions = [item | dict(A_invocations=None, A_inputs=None, B_invocations=None, B_inputs=None,
                           T_invocations=None, test_judgment='未判定（本文未審査）')
                 for item in inventory['items'] if item['kind'] == 'function_item']
    write(out / 'functions.json', dict(status=result['status'], functions=functions))
    # Preserve source provenance and exact compiler diagnostic/cycle witnesses, not copied estimates.
    for name in ['inventory.json', 'unused.json', 'dependencies.json', 'graph.json', 'resource-stop.json']:
        original, destination = work / name, out / name
        if original.exists():
            destination.unlink(missing_ok=True)
            os.link(original, destination)
    write(out / 'measurement-status.json', {k: v for k, v in result.items() if k != 'groups'})
    if write_docs:
        table = ['| 群 | 本体行数(構文cfg) | 責務 | A入力数 | B file数 | 未通過行 | testのみ行 | 入/出(HIRのみ) | HIRの輪 |',
                 '|---|---:|---|---:|---:|---:|---:|---:|---|']
        for row in rows[:60]:
            body = '構文未確定' if row['syntax_errors'] else str(row['physical_production'])
            deps = '未測定' if row['incoming_HIR'] is None else f"{row['incoming_HIR']}/{row['outgoing_HIR']}"
            cycle = '検出済' if row['detected_HIR_cycle'] else '未確定'
            table.append(f"| `{row['group']}` | {body} | | 未測定 | 未測定 | 未測定 | 未測定 | {deps} | {cycle} |")
        (ROOT / 'src/mir/README.md').write_text('\n'.join(table) + '\n')
    write(work / 'render-summary.json', dict(groups=len(rows), function_definitions=len(functions),
        syntax_error_files=len(inventory['syntax_error_files']), warning_items=len(unused['items']) if unused else None,
        HIR_cycles=len(graph['cycles']) if graph else None, largest_HIR_cycle=max((c['size'] for c in graph['cycles']), default=None) if graph else None))
    return result
