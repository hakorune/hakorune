#!/usr/bin/env python3
"""Execute checked-compare JSON emitted by the original-source Rust tests.

The borrowed tagged carrier keeps its kind/payload lanes end to end: the
Normal-Integer view is lent only while the runtime kind lane is exactly 1,
and any other kind at the compare site is the callee's own semantic Fault.
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
TARGET = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target'))
ARCHIVE = TARGET / 'lifecycle-kernel/release/libnyash_lifecycle_kernel.a'
ENV = dict(os.environ, NYASH_NYRT_SILENT_RESULT='1', HAKO_NYRT_PLUGIN_HOST='off')
ENV.pop('V4_PROBE_FAULT_AT', None)
ENV['V4_PROBE_MODE'] = 'normal'
DIRECTORY = Path(sys.argv[1])
RETURN_MODE = len(sys.argv) > 2 and sys.argv[2] == 'integer-return'

# check(requested): i64 { if requested > me.limit { return 7 } return 3 }
# with me.limit = 10 (usize); non-integer kinds fault at the compare site.
CASES = [
    ('hi', 7, False),
    ('lo', 3, False),
    ('neg', 3, False),
    ('bool', 70, True),
    ('null', 70, True),
    ('object', 70, True),
    ('literal', 7, False),
    ('alias-hi', 7, False),
    ('alias-lo', 3, False),
    ('alias-bool', 70, True),
    ('alias-null', 70, True),
    ('alias-object', 70, True),
    ('le-hi', 3, False),
    ('le-lo', 7, False),
    ('le-edge', 7, False),
    ('le-neg', 7, False),
    ('le-bool', 70, True),
    ('le-null', 70, True),
    ('le-object', 70, True),
    ('le-alias-hi', 3, False),
    ('le-alias-lo', 7, False),
    ('le-alias-bool', 70, True),
]


if RETURN_MODE:
    # Original Rust-source output, both finishing modes and source alias paths.
    CASES = [(label + '-' + mode, result, False)
             for label, result in [('formal', 15), ('alias', 15),
                                   ('sibling-positive', 15), ('sibling-zero', 9)]
             for mode in ['false', 'true']]


def checked(argv, **kwargs):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True, **kwargs)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


with tempfile.TemporaryDirectory(prefix='hako checked compare ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'source.o', 'source')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(TARGET / 'release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(TARGET / 'release'), '-o', driver])
    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.home_release_plain_i64', 'object.reclaim_unpublished']
    for suffix, expected, faulted in CASES:
        prefix = 'hako-issued-borrowed-integer-return-' if RETURN_MODE else 'hako-issued-checked-compare-'
        path = DIRECTORY / (prefix + suffix + '.json')
        data = json.loads(path.read_text())
        callee = next(f for f in data['functions'] if f['name'].startswith('Counter.check/'))
        compares = [row['instruction'] for b in callee['blocks']
                    for row in b['instructions'] if row['instruction']['op'] == 'compare']
        assert len(compares) == 1, 'the source checked comparison must execute once'
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
        if RETURN_MODE and suffix.startswith(('formal-', 'alias-')):
            # External input/runtime safety probes, not new source-language acceptance.
            # Keep the original Compare and Return; vary only the caller's primitive actual.
            for label, kind, constant, status in [
                ('zero', 1, {'op': 'const_i64', 'value': 0}, 0),
                ('bool', 2, {'op': 'const_bool', 'value': True}, 70),
                ('null', 0, {'op': 'const_null'}, 70),
            ]:
                variant = copy.deepcopy(data)
                target = next(i for i, f in enumerate(variant['functions'])
                              if f['name'].startswith('Counter.check/'))
                calls = [b['terminator']['instruction'].get('operation', {}).get('call')
                         for b in variant['functions'][0]['blocks']
                         if b['terminator']['instruction'].get('operation', {}).get('kind') == 'ordinary_call']
                call = next(c for c in calls if c['target'] == target)
                actual = call['args'][0]
                value = actual['value']
                actual['kind'] = kind
                producers = [row for b in variant['functions'][0]['blocks'] for row in b['instructions']
                             if row['instruction'].get('dst') == value]
                assert len(producers) == 1 and producers[0]['instruction']['op'] == 'const_i64'
                producers[0]['instruction'] = dict(constant, dst=value)
                alternate = work / (suffix + '-' + label + '.json')
                alternate.write_text(json.dumps(variant))
                checked([driver, alternate, obj], env=ENV)
                checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                         *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                         '-lpthread', '-ldl', '-lm', '-o', exe])
                result = subprocess.run([str(exe)], env=ENV, capture_output=True, text=True)
                assert result.returncode == status, (suffix, label, result.stdout, result.stderr)
                assert ('FAULT ' in result.stdout) == (status == 70), (suffix, label, result.stdout)
                print(suffix, label, 'Integer result or semantic Fault')

    print(str(len(CASES)) + (' source-issued Integer Return inputs plus twelve external kind probes' if RETURN_MODE else ' checked-compare inputs: direct/alias Integer Normal; Null/Bool/Object Fault before condition result'))
