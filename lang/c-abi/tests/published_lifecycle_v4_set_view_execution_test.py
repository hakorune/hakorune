#!/usr/bin/env python3
"""Execute dominated-ArraySet JSON emitted by the original-source Rust tests.

The checked compare lends one Normal-Integer view of the borrowed tagged
carrier; the dominated `me.sizes.set(index, requested)` element write reads
that same projection through the routed `array_set` row and calls
`nyash.array.checked_set_i64_v1`. `index == len` is the array's own
append-at-end contract (index 0 on a fresh array), an out-of-range index is
a recorded bounds Fault at the exact site, and non-integer kinds still
fault at the compare's emitted kind==1 branch.
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
#   me.sizes.set(index, requested)
#   return 1 }
# with me.limit = 10 and sizes = new ArrayBox(): index 0 appends at the
# empty array's own end boundary; the 'oob' fixture uses index 3, which is
# a recorded bounds Fault. 'bool'/'object' fault at the compare before the
# write is ever reached.
CASES = [
    ('ok', 1, False),
    ('over', 0, False),
    ('oob', 70, True),
    ('bool', 70, True),
    ('object', 70, True),
]


def checked(argv, **kwargs):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True, **kwargs)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


with tempfile.TemporaryDirectory(prefix='hako dominated set ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'source.o', 'source')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.home_release_plain_i64',
             'object.reclaim_unpublished']
    for suffix, expected, faulted in CASES:
        path = DIRECTORY / ('hako-issued-set-view-' + suffix + '.json')
        data = json.loads(path.read_text())
        callee = next(f for f in data['functions'] if f['name'] == 'Store.check/1')
        rows = [row['instruction'] for b in callee['blocks']
                for row in b['instructions']]
        sets = [row for row in rows if row['op'] == 'array_set']
        assert len(sets) == 1, 'one routed element write'
        assert isinstance(sets[0]['site'], int), 'exact diagnostic site'
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
        print(suffix, 'dominated set view executes; kernel owns bounds and kind!=1 faults')
    print('3 checked-compare inputs execute unchanged; set-site bounds and kind!=1 fault')
