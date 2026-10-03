#!/usr/bin/env python3
"""Execute dominated-Add JSON emitted by the original-source Rust tests.

The checked compare lends one Normal-Integer view of the borrowed tagged
carrier; the dominated ordered-Add reads that same projection and its fresh
i64 result writes back through the usize store lane. A negative result is
rejected by the emitted write-side range check, and non-integer kinds still
fault at the compare site.
"""
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

# check(requested): i64 {
#   if requested > me.limit { return 0 }
#   me.total = me.total + requested
#   return 1 }
# with me.limit = 10 and me.total = 0 (both usize): a dominated negative Add
# result faults at the write lane; non-integer kinds fault at the compare.
CASES = [
    ('hi', 1, False),
    ('over', 0, False),
    ('neg', 70, True),
    ('bool', 70, True),
    ('object', 70, True),
]


def checked(argv, **kwargs):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True, **kwargs)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


with tempfile.TemporaryDirectory(prefix='hako dominated add ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'source.o', 'source')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.home_release_plain_i64', 'object.reclaim_unpublished']
    for suffix, expected, faulted in CASES:
        path = DIRECTORY / ('hako-issued-add-view-' + suffix + '.json')
        data = json.loads(path.read_text())
        callee = next(f for f in data['functions'] if f['name'] == 'Counter.check/1')
        rows = [row['instruction'] for b in callee['blocks']
                for row in b['instructions']]
        adds = [row for row in rows if row['op'] == 'add']
        assert len(adds) == 2, 'edge-port re-evaluation of the one source add'
        stores = [row for row in rows if row['op'] == 'field_set']
        assert len(stores) == 1, 'one dominated usize write'
        assert stores[0]['exact_numeric_runtime_check'] == {
            'declared_type': 'usize', 'kind': 'dynamic_integer_range'}
        checked([driver, path, obj], env=ENV)
        assert not list(work.glob('source.o.*')), 'temporary artifact leak'
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        run = subprocess.run([str(exe)], env=ENV, capture_output=True, text=True)
        assert run.returncode == expected, (suffix, run.returncode, run.stdout, run.stderr)
        if faulted:
            assert 'FAULT ' in run.stdout, (suffix, run.stdout)
        else:
            assert 'FAULT ' not in run.stdout, (suffix, run.stdout)
        print(suffix, 'dominated add view executes; write lane enforces the usize range')
    print('3 checked-compare inputs execute unchanged; write-site range and kind!=1 fault')
