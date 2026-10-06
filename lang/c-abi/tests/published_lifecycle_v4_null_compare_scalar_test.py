#!/usr/bin/env python3
"""Check eq/ne on non-null scalar carriers at the external physical boundary.

Input is the source-issued Store.check/1 null-comparison program from the
existing Rust test. Replace its carrier with a scalar physical producer;
this is a wire/emitter test, not authority for a new source-language shape.
Build the current shim in a temporary directory, leaving shared binaries alone.
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
SOURCE = Path(sys.argv[1])
ARCHIVE = Path(sys.argv[2])
ENV = dict(os.environ, NYASH_NYRT_SILENT_RESULT='1',
           HAKO_NYRT_PLUGIN_HOST='off', V4_PROBE_MODE='normal')
ENV.pop('V4_PROBE_FAULT_AT', None)


def checked(argv, **kwargs):
    result = subprocess.run([str(arg) for arg in argv], capture_output=True,
                            text=True, timeout=120, **kwargs)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


base = json.loads(SOURCE.read_text())
with tempfile.TemporaryDirectory(prefix='hako scalar null compare ') as directory:
    work = Path(directory)
    shim = ROOT / 'lang/c-abi/shims'
    yyjson = ROOT / 'plugins/nyash-json-plugin/c/yyjson'
    checked(['cc', '-fPIC', '-shared', '-I' + str(yyjson),
             '-o', work / 'libhako_llvmc_ffi.so', shim / 'hako_llvmc_ffi.c',
             shim / 'hako_aot.c', shim / 'hako_json_v1.c', yyjson / 'yyjson.c'])
    driver, obj, exe = [work / name for name in ('driver', 'case.o', 'case')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(work),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(work), '-o', driver])
    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.home_release_plain_i64',
             'object.reclaim_unpublished']
    count = 0
    for op, value in [('const_i64', 0), ('const_i64', 5),
                      ('const_bool', False), ('const_bool', True)]:
        for predicate in ['eq', 'ne']:
            for null_first in [False, True]:
                data = copy.deepcopy(base)
                callee = next(f for f in data['functions']
                              if f['name'] == 'Store.check/1')
                null_ids = {r['instruction']['dst'] for b in callee['blocks']
                            for r in b['instructions']
                            if r['instruction']['op'] == 'const_null'}
                next_id = 90000
                comparisons = 0
                for block in callee['blocks']:
                    rows = []
                    for row in block['instructions']:
                        ins = row['instruction']
                        if ins['op'] == 'borrowed_null_compare':
                            null_id = ins['lhs'] if ins['lhs'] in null_ids else ins['rhs']
                            assert null_id in null_ids
                            rows.append({'index': len(rows), 'instruction':
                                         {'op': op, 'dst': next_id, 'value': value}})
                            ins['predicate'] = predicate
                            ins['lhs'], ins['rhs'] = ((null_id, next_id) if null_first
                                                      else (next_id, null_id))
                            next_id += 1
                            comparisons += 1
                        row['index'] = len(rows)
                        rows.append(row)
                    block['instructions'] = rows
                    block['terminator']['index'] = len(rows)
                assert comparisons == 2, 'expected both source compare edge ports'
                path = work / 'input.json'
                path.write_text(json.dumps(data))
                checked([driver, path, obj], env=ENV)
                checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c',
                         ARCHIVE, *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                         '-lpthread', '-ldl', '-lm', '-o', exe])
                run = subprocess.run([str(exe)], env=ENV, capture_output=True,
                                     text=True, timeout=120)
                expected = 3 if predicate == 'eq' else 7
                assert run.returncode == expected, (op, value, predicate, null_first,
                                                    run.returncode, run.stdout, run.stderr)
                assert 'FAULT ' not in run.stdout, run.stdout
                count += 1
    assert count == 16
    print('16 scalar/null physical cases pass: both predicates and operand orders')
