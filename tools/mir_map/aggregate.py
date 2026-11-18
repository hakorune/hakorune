"""Observed per-input membership and line sets; never convert missing traces to zero."""
from collections import Counter, defaultdict
from pathlib import Path
import gzip
import hashlib
import json
from coverage import read_lcov
from inventory import group_for
from support import write


def inputs(folder, catalog, total=None):
    counts, group_counts, lines = Counter(), Counter(), defaultdict(set)
    results, unknown_symbols = [], Counter()
    fingerprints = []
    expected = json.loads((folder / 'identity.json').read_text()) if (folder / 'identity.json').exists() else None
    if expected is not None:
        declared = len(expected['rows'])
        assert declared > 0 and len({row['path'] for row in expected['rows']}) == declared
        if total is not None:
            assert total == declared, (total, declared)
        total = declared
    missing, partial = 0, 0
    seen_ids = set()
    for path in sorted(folder.glob('[0-9][0-9][0-9][0-9]/result.json')):
        row = json.loads(path.read_text())
        if expected is not None:
            index = int(path.parent.name)
            original = expected['rows'][index]
            assert (row['path'], row['sha256']) == (original['path'], original['sha256']), 'mismatched input receipt'
            seen_ids.add(index)
        results.append(row)
        if not row['profile_available']:
            missing += 1
            fingerprints.append(dict(path=row['path'], function_set_sha256=None, complete_trace=False))
            continue
        partial += not row['complete_trace']
        with gzip.open(path.parent / 'membership.json.gz', 'rt') as stream:
            membership = json.load(stream)
        hit, unmapped = set(), set()
        for symbol in membership['functions']:
            identity = catalog['symbols'].get(symbol)
            if identity is None:
                unknown_symbols[symbol] += 1
                unmapped.add(symbol)
            else:
                hit.add(identity)
        counts.update(hit)  # generic instances are one membership per input
        function_set = json.dumps([sorted(hit), sorted(unmapped)], separators=(',', ':'))
        fingerprints.append(dict(path=row['path'],
                                 function_set_sha256=hashlib.sha256(function_set.encode()).hexdigest(),
                                 complete_trace=row['complete_trace']))
        group_counts.update({catalog['rows'][key]['group'] for key in hit})
        for file, values in membership['lines'].items():
            lines[file].update(values)
    return dict(total=total, completed=len(results), missing_profiles=missing,
                partial_profiles=partial, full=expected is not None and total is not None and len(results) == total and missing == 0 and partial == 0 and seen_ids == set(range(total)),
                counts=counts, group_counts=group_counts, lines=lines, results=results,
                unknown_symbols=dict(unknown_symbols), fingerprints=fingerprints)


def summarize(work, prior, test_catalog=None, test_membership=None):
    catalog = json.loads((work / 'cli-catalog.json').read_text())
    inventory = json.loads((work / 'inventory.json').read_text())
    policy = json.loads((Path(__file__).parent / 'groups.json').read_text())['files']
    A, B = inputs(work / 'A', catalog), inputs(work / 'B', catalog)
    fingerprints = {}
    for label, data in [('A', A), ('B', B)]:
        rows = data.pop('fingerprints')
        groups = Counter(row['function_set_sha256'] for row in rows if row['complete_trace'])
        fingerprints[label] = dict(
            full=data['full'], distinct_function_sets=len(groups),
            group_sizes=dict(sorted(groups.items())), inputs=rows)
    write(work / 'input-function-sets.json', fingerprints)
    _, _, universe = read_lcov(prior / 'probe/coverage.lcov')
    test_rows = {}
    test_lines, test_hits, unmatched_test = defaultdict(set), set(), []
    test_universe, all_test_hits = {}, set()
    test_measured = test_catalog is not None and test_membership is not None
    if test_measured:
        tc = json.loads(Path(test_catalog).read_text())
        test_rows = tc['rows']
        _, _, test_universe = read_lcov(work / 'T/all.lcov')
        with gzip.open(test_membership, 'rt') as stream:
            membership = json.load(stream)
        for symbol in membership['functions']:
            identity = tc['symbols'].get(symbol)
            if identity is not None:
                all_test_hits.add(identity)
            if identity in catalog['rows']:
                test_hits.add(identity)
            else:
                unmatched_test.append(symbol)
        for file, values in membership['lines'].items():
            test_lines[file].update(values)
    # Exact measured line universe of CLI and completed test artifacts.
    # cfg(test)-only syntax is excluded, uninstrumented source stays unknown.
    for file, values in test_universe.items():
        universe.setdefault(file, set()).update(values)
    for file, values in universe.items():
        values.difference_update(inventory['files'].get(file, {}).get('test_lines', []))
    grouped = defaultdict(lambda: dict(instrumented_lines=0, A_lines=0, B_only_lines=0,
                                      T_only_lines=0 if test_measured else None,
                                      known_zero_lines=0 if test_measured else None, AB_zero_lines=0,
                                      physical_production_syntax=0, source_syntax_errors=[]))
    file_rows = []
    for file, values in inventory['files'].items():
        group = group_for(file, policy)
        grouped[group]['physical_production_syntax'] += values['production_lines']
        if values['syntax_error']:
            grouped[group]['source_syntax_errors'].append(file)
    for file, measured in universe.items():
        group = group_for(file, policy)
        a = A['lines'].get(file, set()) & measured
        b = (B['lines'].get(file, set()) & measured) - a
        t = (test_lines.get(file, set()) & measured) - a - b
        zero = measured - a - b - t
        row = dict(file=file, group=group, instrumented_lines=len(measured), A_lines=len(a), AB_zero_lines=len(measured-a-b) if A['full'] and B['full'] else None,
                   B_only_lines=len(b), T_only_lines=len(t) if test_measured else None,
                   known_zero_lines=len(zero) if test_measured and A['full'] and B['full'] else None)
        file_rows.append(row)
        for field in ['instrumented_lines', 'A_lines', 'B_only_lines', 'T_only_lines', 'known_zero_lines', 'AB_zero_lines']:
            if row[field] is None:
                grouped[group][field] = None
            elif grouped[group][field] is not None:
                grouped[group][field] += row[field]
    rows, never, only_test = [], [], []
    for identity, descriptor in catalog['rows'].items():
        known = descriptor['represented']
        row = descriptor | dict(definition_LOC=(descriptor['end']-descriptor['start']+1) if descriptor['kind']!='unmatched_coverage_function' and not inventory['files'].get(descriptor['file'],{}).get('syntax_error',True) else None, A_inputs=A['counts'][identity] if known else None,
            B_inputs=B['counts'][identity] if known else None,
            T_hit=(True if identity in test_hits else (False if descriptor['kind'] != 'unmatched_coverage_function' and test_rows.get(identity, {}).get('represented') else None)) if known and test_measured else None,
            zero_proven=known and A['full'] and B['full'] and not A['counts'][identity] and not B['counts'][identity])
        rows.append(row)
        if row['zero_proven']:
            never.append(row)
            if row['T_hit']:
                only_test.append(row)
    observed_test_absent_cli = [descriptor | dict(T_hit=True, A_inputs=None, B_inputs=None,
        definition_LOC=(descriptor['end']-descriptor['start']+1) if descriptor['kind']!='unmatched_coverage_function' and not inventory['files'].get(descriptor['file'],{}).get('syntax_error',True) else None,
        reason='Observed in completed test; not represented in CLI; no inferred source zero')
        for identity, descriptor in test_rows.items() if identity in all_test_hits and
        not catalog['rows'].get(identity, {}).get('represented', False) and
        descriptor['start'] not in inventory['files'].get(descriptor['file'], {}).get('test_lines', [])]
    watch = []
    for entry in json.loads((Path(__file__).parent / 'entries.json').read_text()):
        candidates = [row for row in rows if row['file'] == entry['file'] and
                      row['kind'] == 'function_item' and row['start'] == entry['line'] and row['name'] == entry['fn']]
        candidate = candidates[0] if len(candidates) == 1 else None
        watch.append(entry | dict(A_inputs=candidate['A_inputs'] if candidate else None,
                                  B_inputs=candidate['B_inputs'] if candidate else None,
                                  joined=candidate is not None, represented=candidate['represented'] if candidate else False))
    group_rows = [dict(group=key, A_inputs=A['group_counts'][key] if value['instrumented_lines'] else None, B_inputs=B['group_counts'][key] if value['instrumented_lines'] else None, **value)
                  for key, value in sorted(grouped.items(), key=lambda kv: (-kv[1]['instrumented_lines'], kv[0]))]
    result = dict(line_units='Instrumented production source lines in CLI union completed test artifacts; cfg(test) syntax removed; physical syntax separate; uninstrumented unknown' ,
        A={k: v for k, v in A.items() if k not in ['counts', 'group_counts', 'lines', 'results']},
        B={k: v for k, v in B.items() if k not in ['counts', 'group_counts', 'lines', 'results']},
        test_measured=test_measured, unmatched_test_functions=unmatched_test, groups=group_rows,
        files=file_rows, watchlist=watch)
    write(work / 'runtime-summary.json', result)
    write(work / 'functions.json', dict(functions=rows, observed_never_A_or_B=never,
                                      test_only_qualified=only_test, observed_test_absent_cli=observed_test_absent_cli, complete_A_B=A['full'] and B['full']))
    return result
