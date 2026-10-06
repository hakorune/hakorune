#!/usr/bin/env python3
"""Execute routed `array_get` JSON emitted by the original-source Rust tests.

A proven `me.<ArrayBox>` `.get(index)` read lowers to the sole physical
`ArrayElementRead` owner and transports as the plain `array_get` op — no
checked export, no diagnostic site. The v4 emit arm calls the existing
`nyash.array.slot_load_hi` slot-read surface, which returns the
i64-or-handle carrier; an out-of-range index reads the lane's null
sentinel 0, matching `ArrayBox.get`'s Null-on-miss contract in normal
mode. Malformed rows (a null dst or an extra key) must fail closed at
physical validation — the lane never repairs shape drift.
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

# probe(n): i64 {
#   me.free.set(0, 7)
#   local v = me.free.get(<index>)
#   return v }
# with free = new ArrayBox(): 'ok' reads the stored 7; 'oob' reads index 5
# past the single slot and observes the null sentinel 0.
CASES = [
    ('ok', 7),
    ('oob', 0),
]


def checked(argv, **kwargs):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True, **kwargs)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


def get_row(path):
    data = json.loads(path.read_text())
    callee = next(f for f in data['functions'] if f['name'] == 'Page.probe/1')
    rows = [row['instruction'] for b in callee['blocks']
            for row in b['instructions']]
    gets = [row for row in rows if row['op'] == 'array_get']
    assert len(gets) == 1, 'one routed element read'
    return data, gets[0]


with tempfile.TemporaryDirectory(prefix='hako routed get ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'source.o', 'source')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.home_release_plain_i64',
             'object.reclaim_unpublished']
    for suffix, expected in CASES:
        path = DIRECTORY / ('hako-issued-get-view-' + suffix + '.json')
        _data, get = get_row(path)
        assert isinstance(get['dst'], int), 'read result dst value id'
        assert 'site' not in get, 'pure read carries no diagnostic site'
        checked([driver, path, obj], env=ENV)
        assert not list(work.glob('source.o.*')), 'temporary artifact leak'
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        run = subprocess.run([str(exe)], env=ENV, capture_output=True, text=True)
        assert run.returncode == expected, (suffix, run.returncode, run.stdout, run.stderr)
        assert 'FAULT ' not in run.stdout, (suffix, run.stdout)
        print(suffix, 'routed get view executes through slot_load_hi')
    # Fail-closed rows: a null dst (dead-read drift) and an extra key are
    # not republished — validation rejects the exact-shape violation.
    ok_path = DIRECTORY / 'hako-issued-get-view-ok.json'
    for label, mutate in [
        ('dst-null', lambda get: get.__setitem__('dst', None)),
        ('extra-site', lambda get: get.__setitem__('site', 0)),
    ]:
        data, get = get_row(ok_path)
        mutate(get)
        bad = work / ('hako-issued-get-view-' + label + '.json')
        bad.write_text(json.dumps(data))
        run = subprocess.run([driver, bad, obj], env=ENV, capture_output=True, text=True)
        assert run.returncode != 0, (label, run.returncode, run.stdout, run.stderr)
        print(label, 'malformed get row rejects at validation')
    print('2 routed get inputs execute; null sentinel and fail-closed shape hold')
