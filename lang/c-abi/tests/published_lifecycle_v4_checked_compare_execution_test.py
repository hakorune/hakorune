#!/usr/bin/env python3
"""Execute checked-compare JSON emitted by the original-source Rust tests.

The borrowed tagged carrier keeps its kind/payload lanes end to end: the
Normal-Integer view is lent only while the runtime kind lane is exactly 1,
and any other kind at the compare site is the callee's own semantic Fault.
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

# check(requested): i64 { if requested > me.limit { return 7 } return 3 }
# with me.limit = 10 (usize); non-integer kinds fault at the compare site.
CASES = [
    ('hi', 7, False),
    ('lo', 3, False),
    ('neg', 3, False),
    ('bool', 70, True),
    ('object', 70, True),
    ('literal', 7, False),
]


def checked(argv, **kwargs):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True, **kwargs)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


with tempfile.TemporaryDirectory(prefix='hako checked compare ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'source.o', 'source')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.home_release_plain_i64', 'object.reclaim_unpublished']
    for suffix, expected, faulted in CASES:
        path = DIRECTORY / ('hako-issued-checked-compare-' + suffix + '.json')
        data = json.loads(path.read_text())
        callee = next(f for f in data['functions'] if f['name'] == 'Counter.check/1')
        compares = [row['instruction'] for b in callee['blocks']
                    for row in b['instructions'] if row['instruction']['op'] == 'compare']
        assert len(compares) == 2, 'edge-port re-evaluation of the one source compare'
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
        print(suffix, 'checked-compare view executes; kind lane governs the lent projection')
    print('4 checked-compare inputs execute unchanged; kind!=1 faults at the view site')
