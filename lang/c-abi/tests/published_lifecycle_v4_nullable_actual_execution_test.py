#!/usr/bin/env python3
"""Execute borrowed actual JSON emitted by the original-source Rust tests.

`s.release(null)` spells tag 0 with the exact `const_null` producer, and
`s.release(h)` on a received `NullableObject` result spells
`nullable_typed_object` — the ABI owner selects tag 0/3 at runtime, the
callee revalidates the pair, and the caller's checked release touches the
kernel only when the wire value is live. The Void sentinel is never
released and the callee still ends at zero.
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


def checked(argv, **kwargs):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True, **kwargs)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


def instructions(data, name):
    callee = next(f for f in data['functions'] if f['name'] == name)
    return [row['instruction'] for b in callee['blocks']
            for row in b['instructions']]


def release_arg(data):
    """main's `s.release` actual — the non-i64 borrowed argument."""
    callee = next(f for f in data['functions'] if f['name'] == 'main')
    for block in callee['blocks']:
        term = block['terminator']['instruction']
        call = term.get('operation', {}).get('call')
        if not call:
            continue
        for arg in call['args']:
            if arg['kind'] != 1:
                return arg
    raise AssertionError('release actual missing')


with tempfile.TemporaryDirectory(prefix='hako null actual ') as directory:
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

    def run_input(data, homes):
        compile_input(data)
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        run = subprocess.run([str(exe)], env=ENV, capture_output=True, text=True)
        assert run.returncode == 0, (run.returncode, run.stdout, run.stderr)
        assert 'FAULT ' not in run.stdout, run.stdout
        counts = next(line for line in run.stdout.splitlines() if line.startswith('COUNTS '))
        assert int(counts.split()[3]) == homes, (counts, homes)

    # s.release(null): tag 0 names the exact const_null producer; only the
    # Store home is released — the sentinel itself never owns a lease.
    null_data = json.loads((DIRECTORY / 'hako-issued-null-actual-null.json').read_text())
    arg = release_arg(null_data)
    assert arg['kind'] == 0
    assert any(row['op'] == 'const_null' and row['dst'] == arg['value']
               for row in instructions(null_data, 'main'))
    run_input(null_data, 1)
    print('null: tag-0 actual executes; the Void sentinel is never released')

    # s.release(h) on a live received nullable: the runtime tag select
    # hands (3, handle) to the callee; the caller's checked release fires
    # exactly once for h plus once for s.
    base = json.loads((DIRECTORY / 'hako-issued-null-actual-nullable.json').read_text())
    arg = release_arg(base)
    assert arg['kind'] == 'nullable_typed_object'
    assert any(row['op'] == 'invoke_normal_result' and row['dst'] == arg['value']
               for row in instructions(base, 'main'))
    run_input(base, 2)
    print('nullable-live: selected tag reaches the callee; h releases once')

    # The same shape with check(20) returning null: the pair is (0, 0), the
    # callee still runs, and release-if-live skips the kernel entirely.
    null_state = copy.deepcopy(base)
    for row in instructions(null_state, 'main'):
        if row.get('op') == 'const_i64' and row.get('value') == 5:
            row['value'] = 20
    run_input(null_state, 1)
    print('nullable-null: Void state transports (0,0) and releases nothing')

    def mutate(change):
        data = copy.deepcopy(base)
        main = next(f for f in data['functions'] if f['name'] == 'main')
        change(main, data)
        compile_input(data, False)

    def set_release_arg(main, change):
        for block in main['blocks']:
            term = block['terminator']['instruction']
            call = term.get('operation', {}).get('call')
            if call:
                for arg in call['args']:
                    if arg['kind'] == 'nullable_typed_object':
                        change(arg)
                        return
        raise AssertionError('nullable actual missing')

    # A numeric tag above the closed range is ABI drift at the parser.
    def tag_over_range(main, _):
        set_release_arg(main, lambda arg: arg.update(kind=4))
    mutate(tag_over_range)

    # Tag 0 without the exact const_null producer is a forged null.
    def forged_null(main, _):
        i64_dst = next(row['dst'] for row in instructions(
            {'functions': [main]}, 'main') if row['op'] == 'const_i64')
        set_release_arg(main, lambda arg: arg.update(kind=0, value=i64_dst))
    mutate(forged_null)

    # A literal tag 3 claims a dominated `new` TypedHome: the received
    # call result is not one, so the stale spelling rejects.
    def stale_tag3(main, _):
        set_release_arg(main, lambda arg: arg.update(kind=3))
    mutate(stale_tag3)

    # The nullable pair never points at a scalar lane.
    def scalar_value(main, _):
        i64_dst = next(row['dst'] for row in instructions(
            {'functions': [main]}, 'main') if row['op'] == 'const_i64')
        set_release_arg(main, lambda arg: arg.update(value=i64_dst))
    mutate(scalar_value)

    # An extra key breaks the exact physical shape.
    def extra_key(main, _):
        set_release_arg(main, lambda arg: arg.update(extra=1))
    mutate(extra_key)

    # The call-site result word and the callee role stay one corroborated
    # pair: claiming i64 against a nullable callee is ABI drift.
    def result_drift(main, _):
        for block in main['blocks']:
            term = block['terminator']['instruction']
            op = term.get('operation', {})
            if op.get('result') == 'nullable_handle':
                op['result'] = 'i64'
    mutate(result_drift)

    # A forged const_null in a function with no admitted null lane rejects.
    def forged_sentinel(_, data):
        root = next(f for f in data['functions'] if f['name'] == 'Store.release/1')
        block = root['blocks'][0]
        block['instructions'].insert(0, {
            'index': 0,
            'instruction': {'op': 'const_null', 'dst': 9999}})
        for i, row in enumerate(block['instructions']):
            row['index'] = i
    mutate(forged_sentinel)

    print('3 source-issued programs execute; seven forged physical rows reject')
