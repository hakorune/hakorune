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
ARCHIVE = Path(os.environ.get('HAKO_TEST_LIFECYCLE_ARCHIVE',
    ROOT / 'target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a'))
ENV = dict(os.environ, NYASH_NYRT_SILENT_RESULT='1', HAKO_NYRT_PLUGIN_HOST='off')
ENV.pop('V4_PROBE_FAULT_AT', None)
ENV.pop('V4_PROBE_CLASS_DRIFT', None)
WRAPS = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
         'object.checked_field_set', 'object.checked_residence_set', 'object.home_release_plain_i64', 'object.reclaim_unpublished']


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

    # Independent wire boundary: completed singleton Object narrows the existing
    # class loan to nonnull, without changing legacy nullable source witnesses.
    def nonnull_classes(data):
        for row in data['functions']:
            for param in row['params']:
                if type(param.get('object_view')) is int:
                    param['object_nonnull_view'] = True
        return data

    strict = nonnull_classes(copy.deepcopy(issued[('declared-copy', 'object')]))
    compile_input(strict)
    checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c',
             TESTS / 'published_lifecycle_v4_declared_borrow_probe.c', ARCHIVE,
             *['-Wl,--wrap=nyash.' + name + '_v1' for name in WRAPS],
             '-Wl,--wrap=nyash.object.type_id_h', '-lpthread', '-ldl', '-lm', '-o', exe])
    run = subprocess.run([exe], env=ENV, capture_output=True, text=True)
    assert run.returncode == 11 and 'FAULT ' not in run.stdout, run
    drift = subprocess.run([exe], env=dict(ENV, V4_PROBE_CLASS_DRIFT='1'), capture_output=True, text=True)
    assert drift.returncode == 70 and 'CLASS_DRIFT ' in drift.stderr, drift
    print('nonnull class: same borrowed lifetime; runtime class mismatch refused')

    negatives = []
    data = copy.deepcopy(strict)
    function(data, 'Transport.bridge/1')['params'][0]['object_nonnull_view'] = False
    negatives.append(('nonnull-marker-must-be-true', data))
    data = nonnull_classes(copy.deepcopy(issued[('declared-copy', 'null')]))
    negatives.append(('nonnull-class-rejects-null-actual', data))
    data = copy.deepcopy(strict)
    del function(data, 'Transport.bridge/1')['params'][0]['object_nonnull_view']
    negatives.append(('nullable-class-cannot-forward-to-nonnull-class', data))
    data = copy.deepcopy(strict)
    for block in function(data, 'main')['blocks']:
        operation = block['terminator']['instruction'].get('operation', {})
        if operation.get('kind') == 'ordinary_call':
            operation['call']['args'][0]['kind'] = 'nullable_typed_object'
    negatives.append(('nonnull-class-rejects-nullable-spelling', data))
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

    # The same read/result safety boundary covers formal and real receiver support.
    for suffix, symbol, expected in [('entry-field-phi-0', 'Leaf.maybe/2', 0), ('entry-field-phi-1', 'Leaf.maybe/2', 7), ('entry-field-phi-0-unoptimized', 'Leaf.maybe/2', 0), ('entry-field-phi-1-unoptimized', 'Leaf.maybe/2', 7), ('entry-field-0', 'Leaf.maybe/2', 0), ('entry-field-1', 'Leaf.maybe/2', 7), ('phi-forward-0', 'Leaf.forward/2', 7), ('phi-forward-1', 'Leaf.forward/2', 7), ('phi-forward-0-unoptimized', 'Leaf.forward/2', 7), ('phi-forward-1-unoptimized', 'Leaf.forward/2', 7), ('branch-0', 'Leaf.identity/1', 7), ('branch-1', 'Leaf.identity/1', 7), ('branch-0-unoptimized', 'Leaf.identity/1', 7), ('branch-1-unoptimized', 'Leaf.identity/1', 7), ('rebind-forward', 'Leaf.forward/1', 7), ('rebind-direct', 'Leaf.identity/1', 7), ('rebind-alias', 'Leaf.identity/1', 7), ('receiver-reborrow-alias', 'Leaf.identity/0', 7), ('receiver-reborrow', 'Leaf.identity/0', 7), ('receiver-reborrow-child', 'Leaf.identity/0', 7), ('reborrow-read-after', 'Leaf.identity/1', 7), ('reborrow-child', 'Leaf.identity/1', 7), ('reborrow', 'Leaf.identity/1', 7), ('reborrow-null-0', 'Leaf.identity/1', 0), ('reborrow-null-1', 'Leaf.identity/1', 7), ('local', 'Leaf.identity/1', 7), ('alias', 'Leaf.identity/1', 7), ('guarded-alias-0', 'Leaf.maybe/2', 0), ('guarded-alias-1', 'Leaf.maybe/2', 7), ('receiver', 'Leaf.identity/0', 7), ('receiver-forward', 'Leaf.forward/0', 7), ('field', 'Parent.identity/0', 7), ('field-forward', 'Parent.forward/0', 7), ('forward', 'Leaf.forward/2', 7), ('nullable-0', 'Leaf.maybe/2', 7), ('nullable-1', 'Leaf.maybe/2', 7), ('guarded-0', 'Leaf.maybe/2', 7), ('guarded-1', 'Leaf.maybe/2', 9), ('guarded-field-0', 'Leaf.maybe/2', 0), ('guarded-field-1', 'Leaf.maybe/2', 7)]:
        caller = 'Leaf.probe/2' if suffix.startswith('entry-field-') else ('Leaf.probe/1' if suffix.startswith('branch-') else 'main')
        # Copy and unanimous Phi retain the original explicit read Call.
        read = json.loads((INPUTS / f'hako-issued-declared-read-{suffix}.json').read_text())
        compile_input(read)
        checked(['cc', obj, ARCHIVE, '-lpthread', '-ldl', '-lm', '-o', exe])
        run = subprocess.run([exe], env=ENV, capture_output=True, text=True)
        assert run.returncode == expected, (run.returncode, run.stdout, run.stderr)
        print(f'source-issued From {suffix} support: canonical EXE exit {expected}')
        if suffix.startswith(('phi-forward-', 'entry-field-phi-')):
            changed = copy.deepcopy(read)
            forwarded = function(changed, 'Leaf.forward/2' if suffix.startswith('phi-forward-') else caller)
            phi = next(row['instruction'] for block in forwarded['blocks']
                       for row in block['instructions'] if row['instruction']['op'] == 'phi')
            phi['inputs'][0]['value'] = forwarded['receiver']
            compile_input(changed, accepted=False)
            print('reject foreign same-class readonly Phi input')
        if suffix == 'reborrow-child':
            changed = copy.deepcopy(read)
            field = next(block['terminator']['instruction']['operation'] for block in function(changed, caller)['blocks']
                         if block['terminator']['instruction'].get('operation', {}).get('kind') == 'field_get')
            field['object_id'] = function(changed, 'Parent.get/0')['receiver_object']
            field['field_ordinal'] = 1  # Parent.marker is I64 too; payload is still Leaf.
            compile_input(changed, accepted=False)
            print('reject anchor class substituted for Child payload class')
        if suffix.startswith(('guarded-field-', 'guarded-alias-', 'reborrow-null-', 'entry-field-')):
            changed = copy.deepcopy(read)
            guards = [row['instruction'] for block in function(changed, caller)['blocks']
                      for row in block['instructions']
                      if row['instruction']['op'] == 'borrowed_null_compare']
            assert len(guards) == 1
            guards[0]['predicate'] = 'ne'
            compile_input(changed, accepted=False)
            print('reject swapped nullable field guard')

        for side in ['call', 'callee']:
            for marker in [None, False]:
                data = copy.deepcopy(read)
                callee = function(data, symbol)
                call = next(block['terminator']['instruction']['operation']
                            for block in function(data, caller)['blocks']
                            if block['terminator']['instruction'].get('operation', {}).get('kind') == 'ordinary_call')
                target = call if side == 'call' else callee
                if marker is None:
                    del target['result_read']
                else:
                    target['result_read'] = marker
                compile_input(data, accepted=False)
        data = copy.deepcopy(read)
        blocks = function(data, caller)['blocks']
        call = next(block['terminator']['instruction'] for block in blocks
                    if block['terminator']['instruction'].get('operation', {}).get('result_read') is True)
        landing = next(block for block in blocks if block['id'] == call['normal'])
        projected = next(row['instruction']['dst'] for row in landing['instructions']
                         if row['instruction'].get('op') == 'invoke_normal_result')
        read_alias = next((row['instruction']['dst'] for row in landing['instructions']
                           if row['instruction'].get('op') == 'copy' and row['instruction']['src'] == projected), projected)
        end = next((block['terminator']['instruction']['operation'] for block in reversed(blocks)
                    if block['terminator']['instruction'].get('operation', {}).get('kind') == 'home_release'), None)
        if end is None:
            assert suffix.startswith('entry-field-'), 'owner-bearing cases retain their End mutation'
            print('reject read qualifier erasure/false; borrowed-entry caller has no local Home End')
        else:
            end['value'] = read_alias
            compile_input(data, accepted=False)
            print('reject read qualifier erasure/false and End of borrowed result')

        if suffix in ['receiver-forward', 'field-forward']:
            for support in [None, {'kind': 'formal', 'ordinal': 0}, {'kind': 'receiver', 'extra': 0}]:
                data = copy.deepcopy(read)
                relay = function(data, symbol)
                origin = next(block['terminator']['instruction']['result_origin'] for block in relay['blocks']
                              if block['terminator']['instruction'].get('op') == 'return')
                if support is None:
                    del origin['result_support']
                else:
                    origin['result_support'] = support
                compile_input(data, accepted=False)
            print('reject receiver Call return support loss, formal disguise and extra key')

        if suffix == 'forward':
            for corruption in ['outer-formal', 'inner-formal', 'inner-read', 'producer', 'normal']:
                data = copy.deepcopy(read)
                relay = function(data, symbol)
                returned = next(block['terminator']['instruction'] for block in relay['blocks']
                                if block['terminator']['instruction'].get('op') == 'return')
                inner = next(block['terminator']['instruction'] for block in relay['blocks']
                             if block['terminator']['instruction'].get('operation', {}).get('kind') == 'ordinary_call')
                if corruption == 'outer-formal':
                    returned['result_origin']['formal_ordinal'] = 0
                elif corruption == 'inner-formal':
                    inner['operation']['result_formal_ordinal'] = 1
                elif corruption == 'inner-read':
                    del inner['operation']['result_read']
                elif corruption == 'producer':
                    returned['result_origin']['producer'] = {'block': 999999, 'instruction': 0}
                else:
                    inner['normal'] = inner['fault']
                compile_input(data, accepted=False)

        if suffix in ['receiver', 'receiver-forward', 'field', 'field-forward']:
            for side in ['call', 'callee']:
                for corruption in ['extra-ordinal', 'unknown-support', 'extra-support-key']:
                    data = copy.deepcopy(read)
                    target = function(data, symbol) if side == 'callee' else next(
                        block['terminator']['instruction']['operation'] for block in function(data, caller)['blocks']
                        if block['terminator']['instruction'].get('operation', {}).get('kind') == 'ordinary_call')
                    if corruption == 'extra-ordinal':
                        target['result_formal_ordinal'] = 0
                    elif corruption == 'unknown-support':
                        target['result_support']['kind'] = 'unknown'
                    else:
                        target['result_support']['formal_ordinal'] = 0
                    compile_input(data, accepted=False)

        if suffix == 'field':
            for corruption in ['child-is-parent', 'unknown-field', 'nullable-without-child-guard', 'missing-invoke-coordinate', 'invalid-producer-coordinate']:
                data = copy.deepcopy(read)
                callee = function(data, symbol)
                returned = next(block['terminator']['instruction'] for block in callee['blocks']
                                if block['terminator']['instruction'].get('op') == 'return')
                origin = returned['result_origin']
                if corruption == 'child-is-parent':
                    origin['object_id'] = callee['receiver_object']
                elif corruption == 'unknown-field':
                    origin['field']['field_ordinal'] += 1
                elif corruption == 'missing-invoke-coordinate':
                    origin['invoke'] = None
                elif corruption == 'invalid-producer-coordinate':
                    origin['producer'] = {'block': 4294967295, 'index': 0}
                else:
                    invoke = next(block['terminator']['instruction'] for block in callee['blocks']
                                  if block['terminator']['instruction'].get('operation', {}).get('kind') == 'object_field_borrow')
                    invoke['operation']['nullable'] = True
                compile_input(data, accepted=False)
