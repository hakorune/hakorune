#!/usr/bin/env python3
"""Physical borrowed ABI execution/mutation checks; not source cutover evidence.

The supplied source-issued Pair input lends its existing layout only. The test
builds explicit physical graphs, and preserves caller Normal/Fault cleanup.
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
layout = json.loads(Path(sys.argv[1]).read_text())['layouts'][0]
OBJECT = layout['object_id']
FIELD = layout['fields'][0]['declaration_ordinal']
ARCHIVE = Path(os.environ.get('HAKO_TEST_LIFECYCLE_ARCHIVE',
                             ROOT / 'target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a'))
ENV = dict(os.environ, NYASH_NYRT_SILENT_RESULT='1', HAKO_NYRT_PLUGIN_HOST='off')


def checked(argv, **kw):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True, **kw)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


def block(number, rows, term, targets=()):
    return {'id': number, 'instructions': [
        {'index': i, 'instruction': row} for i, row in enumerate(rows)],
        'terminator': {'index': len(rows), 'instruction': term},
        'edges': [{'target': target, 'args': None} for target in targets]}


def invoke(operation, normal, fault):
    return {'op': 'invoke', 'operation': operation, 'fault_frame': 1,
            'normal': normal, 'fault': fault}


def call(target, receiver, kind, value):
    return {'kind': 'ordinary_call', 'call': {'target': target, 'receiver': receiver,
            'args': [{'kind': kind, 'value': value}], 'dst': None}, 'result': 'i64'}


def function(name, role, receiver, params, blocks):
    return {'name': name, 'role': role, 'receiver': receiver,
            'receiver_object': OBJECT if receiver is not None else None,
            'params': params, 'entry': 0, 'blocks': blocks}


def graph(kind, payload=1):
    arg_rows = [{'op': 'copy', 'dst': 10, 'src': 2}] if kind == 3 else [
        {'op': 'const_bool' if kind == 2 else 'const_i64', 'dst': 10, 'value': bool(payload) if kind == 2 else payload}]
    fault = {'op': 'return_fault', 'fault_frame': 1}
    release = lambda site: {'kind': 'home_release', 'object_id': OBJECT, 'value': 2, 'site': site}
    root = function('main', 'root_i64', None, [], [
        block(0, [{'op': 'fault_frame_enter', 'dst': 1, 'mode': 'root_owned'}],
              invoke({'kind': 'new_box', 'object_id': OBJECT, 'site': 0}, 1, 2), (1, 2)),
        block(1, [{'op': 'invoke_normal_result', 'invoke_block': 0, 'dst': 2}] + arg_rows,
              invoke(call(1, 2, kind, 10), 3, 5), (3, 5)),
        block(2, [], fault),
        block(3, [{'op': 'invoke_normal_result', 'invoke_block': 1, 'dst': 3}],
              invoke(release(1), 4, 2), (4, 2)),
        block(4, [], {'op': 'return', 'value': 3}),
        block(5, [], invoke(release(2), 6, 6), (6, 6)),
        block(6, [], fault)])
    param = [{'value': 3, 'representation': 'borrowed_kind_payload_v1', 'object_view': None}]
    forward = function('forward', 'ordinary_i64', 0, copy.deepcopy(param), [
        block(0, [{'op': 'fault_frame_enter', 'dst': 1, 'mode': 'borrowed'},
                  {'op': 'copy', 'dst': 4, 'src': 3}],
              invoke(call(2, 0, 'tagged', 4), 1, 2), (1, 2)),
        block(1, [{'op': 'invoke_normal_result', 'invoke_block': 0, 'dst': 5}],
              {'op': 'return', 'value': 5}), block(2, [], fault)])
    leaf = function('leaf', 'ordinary_i64', 0, copy.deepcopy(param), [
        block(0, [{'op': 'fault_frame_enter', 'dst': 1, 'mode': 'borrowed'},
                  {'op': 'const_i64', 'dst': 2, 'value': 7}],
              invoke({'kind': 'field_set', 'object_id': OBJECT, 'field_ordinal': FIELD,
                      'base': 0, 'value': 2, 'site': 77, 'exact_numeric_runtime_check': {'kind': 'none'}}, 1, 2), (1, 2)),
        block(1, [], {'op': 'return', 'value': 2}), block(2, [], fault)])
    return {'schema': 'hako.published-lifecycle-physical-program.v2',
            'fault_abi_version': 1, 'storage_profile': 1, 'process_result_site': 99,
            'functions': [root, forward, leaf], 'layouts': [copy.deepcopy(layout)]}


def other_handle_graph(nullable):
    """Keep all leases discharged, and force a layout/default-id collision."""
    data = graph(3)
    def remap(value):
        if isinstance(value, dict):
            for key, child in value.items():
                if key in ('object_id', 'receiver_object') and child is not None:
                    value[key] = 0
                else:
                    remap(child)
        elif isinstance(value, list):
            for child in value:
                remap(child)
    remap(data)
    root = data['functions'][0]
    producer = {'kind': 'array_new', 'site': 88}
    if nullable:
        producer = {'kind': 'ordinary_call', 'call': {'target': 3, 'receiver': 2,
                    'args': [], 'dst': None}, 'result': 'nullable_handle'}
        data['functions'].append(function('nullable', 'ordinary_nullable_handle', 0, [], [
            block(0, [{'op': 'fault_frame_enter', 'dst': 1, 'mode': 'borrowed'}],
                  invoke({'kind': 'new_box', 'object_id': 0, 'site': 92}, 1, 2), (1, 2)),
            block(1, [{'op': 'invoke_normal_result', 'invoke_block': 0, 'dst': 2}],
                  {'op': 'return', 'value': 2, 'result_origin': {
                      'kind': 'fresh', 'producer': {'block': 1, 'index': 0},
                      'invoke': {'block': 0, 'index': 1}, 'copies': []}}),
            block(2, [], {'op': 'return_fault', 'fault_frame': 1})]))
        data['functions'][3]['receiver_object'] = 0
    root['blocks'][0]['terminator']['instruction']['normal'] = 7
    root['blocks'][0]['edges'][0]['target'] = 7
    root['blocks'].append(block(7, [{'op': 'invoke_normal_result', 'invoke_block': 0, 'dst': 2}],
                                invoke(producer, 1, 5), (1, 5)))
    root['blocks'][1]['instructions'] = [
        {'index': 0, 'instruction': {'op': 'invoke_normal_result', 'invoke_block': 7, 'dst': 10}}]
    root['blocks'][1]['terminator']['index'] = 1
    root['blocks'][1]['terminator']['instruction']['fault'] = 9
    root['blocks'][1]['edges'][1]['target'] = 9
    # Intrinsic Array leases use the shared conditional-release transport;
    # they carry no ordinary object's exact layout identity.
    release = {'kind': 'home_release_if_live',
               'object_id': 0, 'value': 10, 'site': 89}
    root['blocks'][3]['terminator']['instruction'] = invoke(copy.deepcopy(release), 8, 5)
    root['blocks'][3]['edges'] = [{'target': 8, 'args': None}, {'target': 5, 'args': None}]
    root['blocks'].append(block(8, [], invoke(
        {'kind': 'home_release', 'object_id': 0, 'value': 2, 'site': 90}, 4, 2), (4, 2)))
    release['site'] = 91
    root['blocks'].append(block(9, [], invoke(copy.deepcopy(release), 5, 5), (5, 5)))
    return data


def received_field_read_graph(null_result):
    """Same Nullable receipt; a runtime Null is a checked source Fault."""
    data = other_handle_graph(True)
    root = data['functions'][0]
    root['blocks'][1]['terminator']['instruction']['operation'] = {
        'kind': 'field_get', 'object_id': 0, 'field_ordinal': FIELD, 'base': 10, 'site': 100}
    if null_result:
        nullable = data['functions'][3]
        fault = {'op': 'return_fault', 'fault_frame': 1}
        nullable['blocks'] = [
            block(0, [{'op': 'fault_frame_enter', 'dst': 1, 'mode': 'borrowed'},
                      {'op': 'const_bool', 'dst': 4, 'value': True}],
                  {'op': 'branch', 'condition': 4, 'then': 3, 'else': 4, 'then_args': None, 'else_args': None}, (3, 4)),
            block(1, [{'op': 'invoke_normal_result', 'invoke_block': 4, 'dst': 2}],
                  {'op': 'return', 'value': 2, 'result_origin': {
                      'kind': 'fresh', 'producer': {'block': 1, 'index': 0},
                      'invoke': {'block': 4, 'index': 0}, 'copies': []}}),
            block(2, [], fault),
            block(3, [{'op': 'const_null', 'dst': 3}], {'op': 'return', 'value': 3,
                'result_origin': {'kind': 'null', 'producer': {'block': 3, 'index': 0}, 'copies': []}}),
            block(4, [], invoke({'kind': 'new_box', 'object_id': 0, 'site': 92}, 1, 2), (1, 2)),
        ]
    # A nonzero stored value distinguishes the getter's output from a default
    # allocation slot. Store failure still discharges the callee's fresh Home.
    nullable = data['functions'][3]
    fresh = next(row for row in nullable['blocks'] if row['id'] == 1)
    returned = fresh['terminator']['instruction']
    fresh['instructions'].append({'index': 1, 'instruction': {'op': 'const_i64', 'dst': 5, 'value': 37}})
    fresh['terminator'] = {'index': 2, 'instruction': invoke({
        'kind': 'field_set', 'object_id': 0, 'field_ordinal': FIELD,
        'base': 2, 'value': 5, 'site': 93, 'exact_numeric_runtime_check': {'kind': 'none'}}, 5, 6)}
    fresh['edges'] = [{'target': 5, 'args': None}, {'target': 6, 'args': None}]
    nullable['blocks'].extend([
        block(5, [], returned),
        block(6, [], invoke({'kind': 'home_release', 'object_id': 0, 'value': 2, 'site': 94}, 2, 2), (2, 2)),
    ])
    return data


with tempfile.TemporaryDirectory(prefix='hako borrowed ABI ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'borrowed.o', 'borrowed')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.checked_residence_set', 'object.home_release_plain_i64', 'object.reclaim_unpublished']

    def compile_input(data, accepted=True):
        path = work / 'input.json'
        path.write_text(json.dumps(data))
        # A rejection must leave the pre-existing destination untouched.
        marker = b'unchanged artifact'
        obj.write_bytes(marker)
        result = subprocess.run([str(driver), str(path), str(obj)], env=ENV,
                                capture_output=True, text=True)
        assert (result.returncode == 0) == accepted, result.stderr
        if not accepted:
            assert '[freeze:contract]' in result.stderr, result.stderr
            assert obj.read_bytes() == marker
        assert not list(work.glob('borrowed.o.*'))

    for null_result in [False, True]:
        data = received_field_read_graph(null_result)
        compile_input(data)
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        result = subprocess.run([str(exe)], env=dict(ENV, V4_PROBE_MODE='normal'), capture_output=True, text=True)
        assert result.returncode == (70 if null_result else 37), (null_result, result)
        counts = '1 0 1 0 1 1' if null_result else '1 1 2 0 0 1'
        assert 'COUNTS ' + counts + '\n' in result.stdout, (null_result, result.stdout, result.stderr)
        if null_result:
            assert 'FAULT 101 100 0 0 HOME 1 RECLAIM 0\n' in result.stdout, result.stdout
        else:
            assert 'FAULT ' not in result.stdout, result.stdout
        print('received nullable field', null_result, 'Normal/Fault and remaining cleanup PASS')
    for mutation in ['extra-key', 'field', 'new-base', 'missing-normal', 'foreign-class', 'fault-cleanup']:
        data = received_field_read_graph(False)
        op = data['functions'][0]['blocks'][1]['terminator']['instruction']['operation']
        if mutation == 'extra-key':
            op['value'] = 10
        elif mutation == 'field':
            op['field_ordinal'] = 999
        elif mutation == 'new-base':
            op['base'] = 2
        elif mutation == 'missing-normal':
            data['functions'][0]['blocks'][3]['instructions'] = []
            data['functions'][0]['blocks'][3]['terminator']['index'] = 0
        elif mutation == 'foreign-class':
            foreign = copy.deepcopy(data['layouts'][0])
            foreign.update(object_id=1, runtime_type_id=2)
            data['layouts'].append(foreign)
            op['object_id'] = 1
        else:
            skipped = next(row for row in data['functions'][0]['blocks'] if row['id'] == 9)
            skipped['terminator'] = {'index': 0, 'instruction': {'op': 'jump', 'target': 5, 'args': None}}
            skipped['edges'] = [{'target': 5, 'args': None}]
        compile_input(data, False)
        print('reject received field', mutation)

    for kind, payload in [(1, 0), (1, -7), (1, 1), (2, 0), (2, 1), (3, 1)]:
        compile_input(graph(kind, payload))
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        for mode, status, counts in [('normal', 7, '1 1 1 0 0 1'),
                                     ('fault-first', 70, '1 1 1 0 1 1')]:
            result = subprocess.run([str(exe)], env=dict(ENV, V4_PROBE_MODE=mode),
                                    capture_output=True, text=True)
            assert result.returncode == status, (kind, payload, mode, result)
            assert 'COUNTS ' + counts + '\n' in result.stdout, result
            if mode == 'fault-first':
                assert 'FAULT 101 77 7 0 HOME 1 RECLAIM 0\n' in result.stdout, result
        print('borrowed kind/payload', kind, payload, 'Normal/Fault cleanup once')

    # Physical range policy is carried by the issued check. This probes the
    # Invoke Fault edge and absence of store, independently of source cutover.
    for stored in [0, 7, -7]:
        data = graph(1)
        leaf = data['functions'][2]
        leaf['blocks'][0]['instructions'][1]['instruction']['value'] = stored
        op = leaf['blocks'][0]['terminator']['instruction']['operation']
        op['exact_numeric_runtime_check'] = {'kind': 'dynamic_integer_range', 'declared_type': 'usize'}
        compile_input(data)
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        result = subprocess.run([str(exe)], env=dict(ENV, V4_PROBE_MODE='normal'), capture_output=True, text=True)
        assert result.returncode == (70 if stored < 0 else stored), result
        assert 'COUNTS ' + ('1 0 1 0 1 1' if stored < 0 else '1 1 1 0 0 1') + '\n' in result.stdout, result
        if stored < 0:
            assert 'FAULT 113 77 -7 0 HOME 1 RECLAIM 0\n' in result.stdout, result
        print('Invoke usize range', stored, 'store or exact Fault cleanup once')
    for check in [None, {'kind': 'unknown'}, {'kind': 'dynamic_integer_range', 'declared_type': 'i64'},
                  {'kind': 'none', 'declared_type': 'usize'}]:
        data = graph(1)
        op = data['functions'][2]['blocks'][0]['terminator']['instruction']['operation']
        if check is None:
            del op['exact_numeric_runtime_check']
        else:
            op['exact_numeric_runtime_check'] = check
        compile_input(data, False)
    print('Invoke range schema rejects missing and malformed checks')

    # Two original tagged formals: object-view base and Integer payload.
    # Explicit physical policy only; this does not establish source admission.
    for stored in [0, 7, -7]:
        data = graph(3)
        root, forward, leaf = data['functions']
        root_block = root['blocks'][1]
        root_block['instructions'].append({'index': 2, 'instruction': {'op': 'const_i64', 'dst': 12, 'value': stored}})
        root_block['terminator']['index'] = 3
        root_block['terminator']['instruction']['operation']['call']['args'].append({'kind': 1, 'value': 12})
        for fn in [forward, leaf]:
            fn['params'][0]['object_view'] = OBJECT
            fn['params'].append({'value': 6, 'representation': 'borrowed_kind_payload_v1', 'object_view': None})
        forwarding = forward['blocks'][0]
        forwarding['instructions'].append({'index': 2, 'instruction': {'op': 'copy', 'dst': 7, 'src': 6}})
        forwarding['terminator']['index'] = 3
        forwarding['terminator']['instruction']['operation']['call']['args'].append({'kind': 'tagged', 'value': 7})
        op = leaf['blocks'][0]['terminator']['instruction']['operation']
        op.update(base=3, value=6, exact_numeric_runtime_check={'kind': 'dynamic_integer_range', 'declared_type': 'usize'})
        compile_input(data)
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        result = subprocess.run([str(exe)], env=dict(ENV, V4_PROBE_MODE='normal'), capture_output=True, text=True)
        assert result.returncode == (70 if stored < 0 else 7), result
        assert 'COUNTS ' + ('1 0 1 0 1 1' if stored < 0 else '1 1 1 0 0 1') + '\n' in result.stdout, result
        if stored < 0:
            assert 'FAULT 113 77 -7 0 HOME 1 RECLAIM 0\n' in result.stdout, result
        for broken in ['missing-object-view', 'unchecked-tagged-store']:
            mutated = copy.deepcopy(data)
            if broken == 'missing-object-view':
                mutated['functions'][2]['params'][0]['object_view'] = None
            else:
                mutated['functions'][2]['blocks'][0]['terminator']['instruction']['operation']['exact_numeric_runtime_check'] = {'kind': 'none'}
            compile_input(mutated, False)
        print('borrowed usize receiver/value', stored, 'exact store/Fault and closed views')

    # I64 Return already accepts an original tagged formal behind its runtime
    # tag1 guard. Keep this as execution coverage, not an obsolete rejection.
    tagged_return = graph(1, 7)
    tagged_return['functions'][2]['blocks'][1]['terminator']['instruction']['value'] = 3
    compile_input(tagged_return)
    checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
             *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
             '-lpthread', '-ldl', '-lm', '-o', exe])
    result = subprocess.run([str(exe)], env=dict(ENV, V4_PROBE_MODE='normal'), capture_output=True, text=True)
    assert result.returncode == 7 and 'COUNTS 1 1 1 0 0 1\n' in result.stdout, result
    print('borrowed I64 Return keeps its runtime tag1 guard')

    receiver_actual = graph(3)
    forwarded = receiver_actual['functions'][1]['blocks'][0]['terminator']['instruction']['operation']['call']['args'][0]
    forwarded.update(kind=3, value=0)
    compile_input(receiver_actual)
    print('verified entry receiver is a typed borrowed actual')

    mutations = {'array-layout-id-collision': other_handle_graph(False),
                 'nullable-layout-id-collision': other_handle_graph(True)}
    base = graph(1)
    for label, tag in [('unknown-tag', 4), ('bool-from-integer', 2), ('object-from-integer', 3),
                       ('tagged-from-integer', 'tagged'), ('old-i64-entry', 'i64')]:
        data = copy.deepcopy(base)
        data['functions'][0]['blocks'][1]['terminator']['instruction']['operation']['call']['args'][0]['kind'] = tag
        mutations[label] = data
    data = graph(2)
    data['functions'][0]['blocks'][1]['instructions'][1]['instruction']['value'] = 2
    mutations['noncanonical-bool'] = data
    for label, representation in [('birth-representation', 'kind_payload_v1'),
                                  ('scalar-formal', 'i64')]:
        data = graph(1)
        data['functions'][1]['params'][0]['representation'] = representation
        mutations[label] = data
    data = graph(1)
    data['functions'][1]['receiver_object'] = OBJECT + 1
    mutations['foreign-receiver-layout'] = data
    data = graph(1)
    data['functions'][1]['receiver'] = None
    data['functions'][1]['receiver_object'] = None
    mutations['static-tagged-formal'] = data
    data = graph(1)
    data['functions'][1]['blocks'][0]['instructions'][1]['instruction']['src'] = 4
    mutations['copy-cycle'] = data
    data = graph(1)
    data['functions'][2]['blocks'][0]['terminator']['instruction']['operation']['value'] = 3
    mutations['tagged-store'] = data
    data = graph(1)
    rows = data['functions'][1]['blocks'][0]['instructions']
    rows[1]['instruction'] = {'op': 'add', 'dst': 4, 'lhs': 3, 'rhs': 3}
    mutations['tagged-arithmetic'] = data
    data = graph(3)
    # Keep the valid receiver but replace its borrowed alias by a null producer.
    data['functions'][0]['blocks'][1]['instructions'][1]['instruction'] = {'op': 'const_null', 'dst': 10}
    mutations['null-object-actual'] = data
    for nullable in [False, True]:
        # Remove only the questioned argument: the surrounding CFG/leases must
        # independently compile, so a leak or malformed producer cannot mask it.
        control = other_handle_graph(nullable)
        for fn in control['functions'][1:3]:
            fn['params'] = []
        control['functions'][1]['blocks'][0]['instructions'].pop()
        control['functions'][0]['blocks'][1]['terminator']['instruction']['operation']['call']['args'] = []
        control['functions'][1]['blocks'][0]['terminator']['instruction']['operation']['call']['args'] = []
        control['functions'][1]['blocks'][0]['terminator']['index'] = 1
        print("producer control", nullable)
        compile_input(control)
    for label, data in mutations.items():
        compile_input(data, False)
        print('reject', label)
    print('borrowed physical ABI acceptance; source cutover remains separate')
