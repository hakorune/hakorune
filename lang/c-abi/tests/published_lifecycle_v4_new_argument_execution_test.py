#!/usr/bin/env python3
"""Execute dominated new-argument JSON emitted by the original-source Rust tests.

The checked compare lends one Normal-Integer view of the borrowed tagged
carrier; the dominated `new Item(p, 3)` argument spells the proven tagged
pair, the caller re-proves kind==1 at the call edge, and the birth unit's own
prologue reads the same pair — the constructed `Item` holds the carrier's
integer payload. Non-integer kinds still fault at the compare site before
the `new` is reached.
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

# check(p): i64 {
#   if p > me.limit { return 0 }
#   local h = new Item(p, 3)
#   return 1 }
# with me.limit = 10: 'ok' constructs Item with the tagged view payload and
# returns 1, 'over' exits before the `new`, and 'bool'/'object' fault at the
# compare's emitted kind==1 branch.
CASES = [
    ('ok', 1, False),
    ('over', 0, False),
    ('bool', 70, True),
    ('object', 70, True),
]


def checked(argv, **kwargs):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True, **kwargs)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


with tempfile.TemporaryDirectory(prefix='hako dominated new ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'source.o', 'source')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.home_release_plain_i64',
             'object.reclaim_unpublished']
    for suffix, expected, faulted in CASES:
        path = DIRECTORY / ('hako-issued-new-argument-' + suffix + '.json')
        data = json.loads(path.read_text())
        callee = next(f for f in data['functions'] if f['name'] == 'Store.check/1')
        rows = [row['instruction'] for b in callee['blocks']
                for row in b['instructions']]
        rows += [op for b in callee['blocks']
                 for op in [b['terminator']['instruction'].get('operation')]
                 if op]
        births = [row for row in rows if row.get('op') == 'birth_call'
                  or row.get('kind') == 'birth_call']
        assert len(births) == 1, 'one routed constructor call'
        args = births[0]['call']['args']
        assert args[0]['kind'] == 'tagged', 'dominated view spells tagged pair'
        assert args[1]['kind'] == 1, 'literal actual stays i64 payload'
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
        print(suffix, 'dominated new-argument view executes; call edge re-proves kind==1')
    print('3 checked-compare inputs execute unchanged; tagged birth actual transports')
