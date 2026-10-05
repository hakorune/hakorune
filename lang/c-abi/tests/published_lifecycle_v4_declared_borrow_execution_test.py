#!/usr/bin/env python3
"""Run source-issued declared/class-forwarding borrows and forged wire negatives."""
import copy
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
TESTS = ROOT / 'lang/c-abi/tests'
INPUTS = Path(sys.argv[1])
ARCHIVE = ROOT / 'target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a'
ENV = dict(os.environ, NYASH_NYRT_SILENT_RESULT='1', HAKO_NYRT_PLUGIN_HOST='off')
ENV.pop('V4_PROBE_FAULT_AT', None)
ENV.pop('V4_PROBE_CLASS_DRIFT', None)
WRAPS = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
         'object.checked_field_set', 'object.home_release_plain_i64', 'object.reclaim_unpublished']


def checked(argv):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


def function(data, name):
    return next(row for row in data['functions'] if row['name'] == name)


with tempfile.TemporaryDirectory(prefix='hako declared borrow ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'issued.o', 'issued')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])

    def compile_input(data, accepted=True):
        path = work / 'input.json'
        path.write_text(json.dumps(data))
        marker = b'preserved before rejected artifact'
        obj.write_bytes(marker)
        result = subprocess.run([driver, path, obj], env=ENV, capture_output=True, text=True)
        assert (result.returncode == 0) == accepted, (result.stdout, result.stderr)
        if not accepted:
            assert '[freeze:contract]' in result.stderr, result.stderr
            assert obj.read_bytes() == marker
        assert not list(work.glob('issued.o.*')), 'temporary artifact leak'

    issued = {}
    total = 0
    for optimize in [False, True]:
        for shape in ['declared', 'declared-copy', 'opaque-forward', 'opaque-copy', 'ignored']:
            for domain in ['null', 'object']:
                path = INPUTS / f'hako-issued-declared-borrow-{shape}-{domain}-opt{str(optimize).lower()}.json'
                data = json.loads(path.read_text())
                bridge = function(data, 'Transport.bridge/1')
                view = bridge['params'][0]['object_view']
                assert isinstance(view, int)
                layout = next(row for row in data['layouts'] if row['object_id'] == view)
                assert layout['runtime_type_id'] != view, 'witness must distinguish runtime type from canonical ordinal'
                compile_input(data)
                checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c',
                         TESTS / 'published_lifecycle_v4_declared_borrow_probe.c', ARCHIVE,
                         *['-Wl,--wrap=nyash.' + name + '_v1' for name in WRAPS],
                         '-Wl,--wrap=nyash.object.type_id_h',
                         '-lpthread', '-ldl', '-lm', '-o', exe])
                opaque_null = shape.startswith('opaque-') and domain == 'null'
                stores = 1 + (domain == 'object' or opaque_null) + (shape != 'ignored') + opaque_null
                expected = 11
                run = subprocess.run([exe], env=dict(ENV, V4_PROBE_MODE='normal'), capture_output=True, text=True)
                assert run.returncode == expected, (path, run.returncode, run.stdout, run.stderr)
                assert f'COUNTS 1 {stores} {stores} 0 0 1\n' in run.stdout, (path, run.stdout)
                assert 'FAULT ' not in run.stdout, run.stdout
                if shape == 'declared-copy':
                    drift = subprocess.run([exe], env=dict(ENV, V4_PROBE_MODE='normal', V4_PROBE_CLASS_DRIFT='1'),
                                           capture_output=True, text=True)
                    if domain == 'null':
                        assert drift.returncode == expected, drift
                        assert 'CLASS_DRIFT ' not in drift.stderr, drift.stderr
                        assert f'COUNTS 1 {stores} {stores} 0 0 1\n' in drift.stdout, drift.stdout
                    else:
                        assert drift.returncode == 70, drift
                        assert 'CLASS_DRIFT ' in drift.stderr, drift.stderr
                        assert 'COUNTS 1 2 0 0 0 1\n' in drift.stdout, drift.stdout
                        assert 'FAULT ' not in drift.stdout, drift.stdout
                for fault_at in range(1, stores + 1):
                    run = subprocess.run([exe], env=dict(ENV, V4_PROBE_MODE='normal', V4_PROBE_FAULT_AT=str(fault_at)),
                                         capture_output=True, text=True)
                    assert run.returncode == 70, (path, fault_at, run.returncode, run.stdout, run.stderr)
                    assert f'COUNTS 1 {fault_at} {fault_at - 1} 1 1 1\n' in run.stdout, (path, fault_at, run.stdout)
                    assert 'FAULT 101 ' in run.stdout, run.stdout
                issued[(shape, domain)] = data
                total += 1
    print(total, 'source-issued both-opt programs: Normal/Fault caller cleanup exactly once')

    negatives = []
    for shape in ['declared-copy', 'ignored']:
        base = issued[(shape, 'object')]
        for label, value in [('missing', ...), ('unknown', 999999), ('scalar', True)]:
            data = copy.deepcopy(base)
            param = function(data, 'Transport.bridge/1')['params'][0]
            if value is ...:
                del param['object_view']
            else:
                param['object_view'] = value
            negatives.append((shape + '-view-' + label, data))
        data = copy.deepcopy(base)
        bridge = function(data, 'Transport.bridge/1')
        bridge['params'][0]['object_view'] = bridge['receiver_object']
        negatives.append((shape + '-known-foreign-class', data))
        data = copy.deepcopy(base)
        object_id = function(data, 'Transport.bridge/1')['params'][0]['object_view']
        data['layouts'] = [row for row in data['layouts'] if row['object_id'] != object_id]
        negatives.append((shape + '-missing-class-layout', data))

    data = copy.deepcopy(issued[('declared-copy', 'object')])
    reader = function(data, 'Transport.read/1')
    reader['params'][0]['object_view'] = reader['receiver_object']
    negatives.append(('forwarded-view-class-mismatch', data))

    # A known live New cannot discard its class by using nullable spelling.
    data = copy.deepcopy(issued[('ignored', 'object')])
    bridge = function(data, 'Transport.bridge/1')
    bridge['params'][0]['object_view'] = bridge['receiver_object']
    for block in function(data, 'main')['blocks']:
        operation = block['terminator']['instruction'].get('operation', {})
        if operation.get('kind') == 'ordinary_call':
            operation['call']['args'][0]['kind'] = 'nullable_typed_object'
    negatives.append(('known-foreign-class-through-nullable-spelling', data))

    data = copy.deepcopy(issued[('declared-copy', 'object')])
    bridge = function(data, 'Transport.bridge/1')
    for block in bridge['blocks']:
        for row in block['instructions']:
            instruction = row['instruction']
            if instruction['op'] == 'copy' and instruction['src'] == bridge['params'][0]['value']:
                instruction['src'] = bridge['receiver']
    negatives.append(('foreign-receiver-copy-as-class-formal', data))

    data = copy.deepcopy(issued[('declared', 'object')])
    bridge = function(data, 'Transport.bridge/1')
    replaced = False
    for block in bridge['blocks']:
        operation = block['terminator']['instruction'].get('operation', {})
        if operation.get('kind') == 'home_release':
            operation['value'] = bridge['params'][0]['value']
            operation['object_id'] = bridge['params'][0]['object_view']
            replaced = True
    assert replaced, 'source witness retains caller-owned temporary cleanup'
    negatives.append(('borrowed-formal-cannot-be-released', data))

    data = copy.deepcopy(issued[('ignored', 'null')])
    root = function(data, 'main')
    for block in root['blocks']:
        for row in block['instructions']:
            if row['instruction']['op'] == 'const_null':
                row['instruction'].update(op='const_i64', value=0)
        operation = block['terminator']['instruction'].get('operation', {})
        if operation.get('kind') == 'ordinary_call':
            operation['call']['args'][0]['kind'] = 1
    negatives.append(('scalar-actual-to-declared-domain', data))

    for label, data in negatives:
        compile_input(data, accepted=False)
        print('reject', label)
