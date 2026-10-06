#!/usr/bin/env python3
"""Execute borrowed/null equality JSON emitted by the original-source Rust tests.

The dedicated `borrowed_null_compare` row reads the carrier's kind or
payload lane — never the Integer view — so a non-null tagged carrier
answers false with no fault branch and no payload read. The exact
`const_null` producer is the only admitted sibling.
"""
import copy
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
TESTS = ROOT / 'lang/c-abi/tests'
ARCHIVE = ROOT / 'target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a'
ENV = dict(os.environ, NYASH_NYRT_SILENT_RESULT='1', HAKO_NYRT_PLUGIN_HOST='off')
ENV.pop('V4_PROBE_FAULT_AT', None)
ENV['V4_PROBE_MODE'] = 'normal'
DIRECTORY = Path(sys.argv[1])

# check(handle): if handle == null { return 7 } return 3
# Every admitted caller argument is non-null, so all cases take 3.
CASES = ['hi', 'swapped', 'object']


def checked(argv, **kwargs):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True, **kwargs)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


def instructions(data, name):
    callee = next(f for f in data['functions'] if f['name'] == name)
    return [row['instruction'] for b in callee['blocks']
            for row in b['instructions']]


with tempfile.TemporaryDirectory(prefix='hako null compare ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'source.o', 'source')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.home_release_plain_i64', 'object.reclaim_unpublished']

    def compile_input(data, expected=True):
        path = work / 'input.json'
        path.write_text(json.dumps(data))
        result = subprocess.run([driver, path, obj], env=ENV, capture_output=True, text=True)
        assert (result.returncode == 0) == expected, (result.stdout, result.stderr)
        assert not list(work.glob('source.o.*')), 'temporary artifact leak'
        return result

    issued = {}
    for suffix in CASES:
        path = DIRECTORY / ('hako-issued-null-compare-' + suffix + '.json')
        data = json.loads(path.read_text())
        compares = [row for row in instructions(data, 'Store.check/1')
                    if row['op'] == 'borrowed_null_compare']
        assert len(compares) == 2, 'edge-port re-evaluation of the one source compare'
        for predicate in ['eq', 'ne']:
            # Eq preserves source-issued input; Ne additionally exercises the
            # physical predicate contract, without claiming source admission.
            case = copy.deepcopy(data)
            for row in instructions(case, 'Store.check/1'):
                if row['op'] == 'borrowed_null_compare':
                    row['predicate'] = predicate
            compile_input(case)
            checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                     *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                     '-lpthread', '-ldl', '-lm', '-o', exe])
            run = subprocess.run([str(exe)], env=ENV, capture_output=True, text=True)
            expected = 3 if predicate == 'eq' else 7
            assert run.returncode == expected, (suffix, predicate, run.returncode,
                                                 run.stdout, run.stderr)
            assert 'FAULT ' not in run.stdout, (suffix, run.stdout)
        issued[suffix] = data
        print(suffix, 'null equality executes; non-null carriers answer false')

    base = issued['hi']
    entry_rows = next(b['instructions'] for b in
                      next(f for f in base['functions'] if f['name'] == 'Store.check/1')['blocks']
                      if any(r['instruction']['op'] == 'borrowed_null_compare'
                             for r in b['instructions']))
    compare_row = next(r for r in entry_rows if r['instruction']['op'] == 'borrowed_null_compare')
    null_row = next(r for r in entry_rows if r['instruction']['op'] == 'const_null'
                    and r['instruction']['dst'] in (compare_row['instruction']['lhs'],
                                                    compare_row['instruction']['rhs']))
    carrier_id = (compare_row['instruction']['lhs']
                  if null_row['instruction']['dst'] == compare_row['instruction']['rhs']
                  else compare_row['instruction']['rhs'])

    def mutate(change):
        data = copy.deepcopy(base)
        rows = [r['instruction'] for b in
                next(f for f in data['functions'] if f['name'] == 'Store.check/1')['blocks']
                for r in b['instructions']]
        change(rows, data)
        compile_input(data, False)

    # Ordering stays unsupported; S0 admits exactly eq and ne.
    def wrong_predicate(rows, _):
        for row in rows:
            if row['op'] == 'borrowed_null_compare':
                row['predicate'] = 'slt'
    mutate(wrong_predicate)

    # Ordinary compare spelling never carries the null sentinel operand.
    def ordinary_spelling(rows, _):
        for row in rows:
            if row['op'] == 'borrowed_null_compare':
                row['op'] = 'compare'
    mutate(ordinary_spelling)

    # Both operands the carrier (no const_null sibling) is a forged row.
    def carrier_carrier(rows, _):
        for row in rows:
            if row['op'] == 'borrowed_null_compare':
                row['rhs'] = carrier_id
                row['lhs'] = carrier_id
    mutate(carrier_carrier)

    # Both operands the null sentinel has no carrier to read.
    def null_null(rows, _):
        for row in rows:
            if row['op'] == 'borrowed_null_compare':
                row['lhs'] = null_row['instruction']['dst']
                row['rhs'] = null_row['instruction']['dst']
    mutate(null_null)

    # An undefined operand fails availability before lane checks.
    def undefined_operand(rows, _):
        for row in rows:
            if row['op'] == 'borrowed_null_compare':
                row['rhs'] = 999999
                break
    mutate(undefined_operand)

    # An extra key breaks the exact physical shape.
    def extra_key(rows, _):
        for row in rows:
            if row['op'] == 'borrowed_null_compare':
                row['extra'] = 1
                break
    mutate(extra_key)

    # A forged const_null outside the nullable/null-compare lanes rejects.
    def forged_sentinel(rows, data):
        root = next(f for f in data['functions'] if f['name'] == 'main')
        block = root['blocks'][0]
        block['instructions'].append(
            {'index': len(block['instructions']),
             'instruction': {'op': 'const_null', 'dst': 9999}})
    mutate(forged_sentinel)

    print('3 source-issued programs and 3 physical ne variants execute; seven forged rows reject')
