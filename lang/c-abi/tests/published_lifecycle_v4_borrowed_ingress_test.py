#!/usr/bin/env python3
"""Physical borrowed ABI execution/mutation checks; not source cutover evidence.

The supplied source-issued Pair input lends its existing layout only. The test
builds explicit physical graphs, and preserves caller Normal/Fault cleanup.
"""
import copy
import json
import os
import shlex
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
FFI = Path(os.environ.get('HAKO_TEST_FFI_DIR', ROOT / 'target/release'))
TESTS = ROOT / 'lang/c-abi/tests'
layout = json.loads(Path(sys.argv[1]).read_text())['layouts'][0]
OBJECT = layout['object_id']
FIELD = layout['fields'][0]['declaration_ordinal']
ARCHIVE = Path(os.environ.get('HAKO_TEST_LIFECYCLE_ARCHIVE',
                             ROOT / 'target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a'))
CC = shlex.split(os.environ.get('CC', 'cc'))
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


def graph(kind, payload=1, integer_view=False):
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
    if integer_view:
        for fn in [forward, leaf]:
            fn['params'][0]['integer_view'] = True
        forward['blocks'][0]['terminator']['instruction']['operation']['call']['args'][0]['kind'] = 1
        store = copy.deepcopy(leaf['blocks'][0]['terminator']['instruction']['operation'])
        store['value'] = 5
        leaf['blocks'] = [
            block(0, [{'op': 'fault_frame_enter', 'dst': 1, 'mode': 'borrowed'},
                      {'op': 'const_i64', 'dst': 2, 'value': 7},
                      {'op': 'const_i64', 'dst': 4, 'value': 1}],
                  {'op': 'jump', 'target': 3, 'args': None}, (3,)),
            block(3, [{'op': 'phi', 'dst': 5, 'inputs': [{'block': 0, 'value': 3}, {'block': 4, 'value': 7}]},
                      {'op': 'compare', 'dst': 6, 'lhs': 5, 'rhs': 2, 'predicate': 'slt'}],
                  {'op': 'branch', 'condition': 6, 'then': 4, 'else': 5,
                   'then_args': None, 'else_args': None}, (4, 5)),
            block(4, [{'op': 'add', 'dst': 7, 'lhs': 5, 'rhs': 4}],
                  {'op': 'jump', 'target': 3, 'args': None}, (3,)),
            block(5, [], invoke(store, 1, 2), (1, 2)),
            block(1, [], {'op': 'return', 'value': 5}), block(2, [], fault)]
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


def borrowed_text_graph(owned=False, add=False, two=False, text="猫\x00🐱", result="i64"):
    """A caller descriptor, two unchanged-pointer forwards, and a read-only leaf."""
    data = graph(1)
    for fn in data['functions'][1:]:
        fn['params'][0]['text_view'] = True
    middle = copy.deepcopy(data['functions'][1])
    middle['name'] = 'forward2'
    data['functions'][1]['blocks'][0]['terminator']['instruction']['operation']['call']['target'] = 3
    data['functions'].append(middle)
    root, leaf = data['functions'][0], data['functions'][2]
    root['blocks'][1]['instructions'] = root['blocks'][1]['instructions'][:1]
    root['blocks'][1]['terminator']['index'] = 1
    actual = {'kind': 'borrowed_text', 'operand': {'kind': 'static_text', 'utf8': text}}
    root['blocks'][1]['terminator']['instruction']['operation']['call']['args'] = [actual]
    old_store = copy.deepcopy(leaf['blocks'][0]['terminator']['instruction']['operation'])
    leaf['blocks'][0]['instructions'].append({'index': 2, 'instruction': {'op': 'copy', 'dst': 4, 'src': 3}})
    leaf['blocks'][0]['terminator'] = {'index': 3, 'instruction': invoke(
        {'kind': 'string_print', 'value': {'kind': 'borrowed_text', 'value': 4}, 'site': 76}, 3, 2)}
    leaf['blocks'][0]['edges'] = [{'target': 3, 'args': None}, {'target': 2, 'args': None}]
    leaf['blocks'].append(block(3, [], invoke(old_store, 1, 2), (1, 2)))
    if owned:
        old_call = copy.deepcopy(root['blocks'][1]['terminator']['instruction']['operation'])
        old_call['call']['args'][0]['operand'] = {'kind': 'owned_string', 'value': 10}
        old_release = root['blocks'][3]['terminator']['instruction']['operation']
        root['blocks'][1]['terminator']['instruction'] = invoke(
            {'kind': 'string_add', 'left': {'kind': 'static_text', 'utf8': text},
             'right': {'kind': 'static_text', 'utf8': ''}, 'site': 70}, 7, 5)
        root['blocks'][1]['edges'] = [{'target': 7, 'args': None}, {'target': 5, 'args': None}]
        root['blocks'].append(block(7, [{'op': 'invoke_normal_result', 'invoke_block': 1, 'dst': 10}],
                                  invoke(old_call, 3, 8), (3, 8)))
        root['blocks'][3]['instructions'][0]['instruction']['invoke_block'] = 7
        root['blocks'][3]['terminator']['instruction'] = invoke({'kind': 'string_end', 'value': 10, 'site': 71}, 9, 5)
        root['blocks'][3]['edges'] = [{'target': 9, 'args': None}, {'target': 5, 'args': None}]
        root['blocks'].extend([block(8, [], invoke({'kind': 'string_end', 'value': 10, 'site': 72}, 5, 5), (5, 5)),
                               block(9, [], invoke(old_release, 4, 2), (4, 2))])
    if add:
        print_op = {'kind': 'string_print', 'value': {'kind': 'owned_string', 'value': 11}, 'site': 78}
        leaf['blocks'][0]['terminator']['instruction'] = invoke(
            {'kind': 'string_add', 'left': {'kind': 'static_text', 'utf8': 'prefix:'},
             'right': {'kind': 'borrowed_text', 'value': 4}, 'site': 76}, 4, 2)
        leaf['blocks'][0]['edges'] = [{'target': 4, 'args': None}, {'target': 2, 'args': None}]
        leaf['blocks'].extend([
            block(4, [{'op': 'invoke_normal_result', 'invoke_block': 0, 'dst': 11}], invoke(print_op, 5, 6), (5, 6)),
            block(5, [], invoke({'kind': 'string_end', 'value': 11, 'site': 79}, 3, 2), (3, 2)),
            block(6, [], invoke({'kind': 'string_end', 'value': 11, 'site': 80}, 2, 2), (2, 2))])
        text = 'prefix:' + text
    if two:
        assert add
        for fn in data['functions'][1:]:
            fn['params'].append({'value': 6, 'representation': 'borrowed_kind_payload_v1',
                                 'object_view': None, 'text_view': True})
            rows = fn['blocks'][0]['instructions']
            rows.append({'index': len(rows), 'instruction': {'op': 'copy', 'dst': 7, 'src': 6}})
            fn['blocks'][0]['terminator']['index'] = len(rows)
            if fn is not leaf:
                fn['blocks'][0]['terminator']['instruction']['operation']['call']['args'].append({'kind': 'tagged', 'value': 7})
        source = next(b for b in root['blocks'] if b['id'] == (7 if owned else 1))
        actual = source['terminator']['instruction']['operation']['call']['args'][0]
        second = copy.deepcopy(actual) if owned else {'kind': 'borrowed_text', 'operand': {'kind': 'static_text', 'utf8': 'second'}}
        source['terminator']['instruction']['operation']['call']['args'].append(second)
        leaf['blocks'][0]['terminator']['instruction']['operation']['left'] = {'kind': 'borrowed_text', 'value': 4}
        leaf['blocks'][0]['terminator']['instruction']['operation']['right'] = {'kind': 'borrowed_text', 'value': 7}
        original = text[len('prefix:'):]
        text = original + (original if owned else 'second')
    if result == 'text':
        assert add and not two
        leaf['role'] = 'ordinary_text'
        leaf['blocks'][1]['terminator']['instruction']['value'] = 11
        acquired = next(b for b in leaf['blocks'] if b['id'] == 4)
        acquired['instructions'].append({'index': len(acquired['instructions']),
            'instruction': {'op': 'string_residence_install', 'value': 11}})
        acquired['terminator']['index'] = len(acquired['instructions'])
        ended = next(b for b in leaf['blocks'] if b['id'] == 5)
        ended['terminator']['instruction'] = {'op': 'jump', 'target': 3, 'args': None}
        ended['edges'] = [{'target': 3, 'args': None}]
        store = next(b for b in leaf['blocks'] if b['id'] == 3)
        store['terminator']['instruction']['fault'] = 6
        store['edges'] = [{'target': 1, 'args': None}, {'target': 6, 'args': None}]
        # The returned Home is newer than another installed Home. Normal Ends
        # only that sibling; Fault retains the original reverse acquisition order.
        entry = leaf['blocks'][0]
        original_add = copy.deepcopy(entry['terminator']['instruction']['operation'])
        entry['terminator']['instruction'] = invoke({'kind': 'string_materialize',
            'value': {'kind': 'static_text', 'utf8': 'older'}, 'site': 83}, 7, 2)
        entry['edges'] = [{'target': 7, 'args': None}, {'target': 2, 'args': None}]
        leaf['blocks'].extend([
            block(7, [{'op': 'invoke_normal_result', 'invoke_block': 0, 'dst': 12},
                      {'op': 'string_residence_install', 'value': 12}],
                  invoke(original_add, 4, 8), (4, 8)),
            block(8, [], invoke({'kind': 'string_end', 'value': 12, 'site': 84}, 2, 2), (2, 2)),
            block(9, [], invoke({'kind': 'string_end', 'value': 11, 'site': 85}, 2, 2), (2, 2))])
        acquired['instructions'][0]['instruction']['invoke_block'] = 7
        ended['instructions'] = [{'index': 0, 'instruction':
            {'op': 'string_residence_install', 'value': 11, 'returning': True}}]
        ended['terminator'] = {'index': 1, 'instruction':
            invoke({'kind': 'string_end', 'value': 12, 'site': 86}, 3, 9)}
        ended['edges'] = [{'target': 3, 'args': None}, {'target': 9, 'args': None}]
        failed_print = next(b for b in leaf['blocks'] if b['id'] == 6)
        failed_print['terminator']['instruction'] = invoke(
            {'kind': 'string_end', 'value': 11, 'site': 80}, 8, 8)
        failed_print['edges'] = [{'target': 8, 'args': None}, {'target': 8, 'args': None}]
        store['terminator']['instruction']['fault'] = 9
        store['edges'] = [{'target': 1, 'args': None}, {'target': 9, 'args': None}]

        # The SAME Text Call retains its result below two computed argument
        # temporaries; only the outer value is passed to the callee.
        caller = middle['blocks'][0]
        text_call = copy.deepcopy(caller['terminator']['instruction']['operation'])
        text_call['result'] = 'text'
        text_call['call']['args'][0] = {'kind': 'borrowed_text',
            'operand': {'kind': 'owned_string', 'value': 13}}
        text_call['text_argument_cleanup'] = [13, 14]
        caller['terminator']['instruction'] = invoke({'kind': 'string_materialize',
            'value': {'kind': 'borrowed_text', 'value': 4}, 'site': 87}, 8, 2)
        caller['edges'] = [{'target': 8, 'args': None}, {'target': 2, 'args': None}]
        normal = middle['blocks'][1]
        normal['instructions'][0]['instruction']['invoke_block'] = 4
        normal['instructions'].append({'index': len(normal['instructions']),
            'instruction': {'op': 'const_i64', 'dst': 8, 'value': 7}})
        normal['terminator'] = {'index': len(normal['instructions']),
            'instruction': invoke({'kind': 'string_end', 'value': 13, 'site': 89}, 10, 11)}
        normal['edges'] = [{'target': 10, 'args': None}, {'target': 11, 'args': None}]
        middle['blocks'].extend([
            block(3, [], {'op': 'return', 'value': 8}),
            block(8, [{'op': 'invoke_normal_result', 'invoke_block': 0, 'dst': 14}],
                invoke({'kind': 'string_materialize', 'value': {'kind': 'owned_string', 'value': 14}, 'site': 88}, 4, 9), (4, 9)),
            block(4, [{'op': 'invoke_normal_result', 'invoke_block': 8, 'dst': 13}],
                invoke(text_call, 1, 5), (1, 5)),
            block(5, [], invoke({'kind': 'string_end', 'value': 13, 'site': 90}, 9, 9), (9, 9)),
            block(9, [], invoke({'kind': 'string_end', 'value': 14, 'site': 91}, 2, 2), (2, 2)),
            block(10, [], invoke({'kind': 'string_end', 'value': 14, 'site': 92}, 6, 7), (6, 7)),
            block(6, [], invoke({'kind': 'string_end', 'value': 5, 'site': 81}, 3, 2), (3, 2)),
            block(11, [], invoke({'kind': 'string_end', 'value': 14, 'site': 93}, 7, 7), (7, 7)),
            block(7, [], invoke({'kind': 'string_end', 'value': 5, 'site': 94}, 2, 2), (2, 2))])
        return data, text
    if result != 'i64':
        assert result in ('unit', 'bool') and not add and not two
        leaf['role'] = 'ordinary_' + result
        returned = leaf['blocks'][1]
        if result == 'unit':
            returned['terminator']['instruction']['value'] = None
        else:
            returned['instructions'] = [{'index': 0, 'instruction': {'op': 'const_bool', 'dst': 8, 'value': True}}]
            returned['terminator']['index'] = 1
            returned['terminator']['instruction']['value'] = 8
        middle['blocks'][0]['terminator']['instruction']['operation']['result'] = result
        normal = middle['blocks'][1]
        if result == 'unit':
            normal['instructions'] = []  # Unit has no projected value.
        normal['instructions'].append({'index': len(normal['instructions']),
            'instruction': {'op': 'const_i64', 'dst': 8, 'value': 7}})
        normal['terminator']['index'] = len(normal['instructions'])
        normal['terminator']['instruction']['value'] = 8
    return data, text


with tempfile.TemporaryDirectory(prefix='hako borrowed ABI ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'borrowed.o', 'borrowed')]
    checked([*CC, TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(FFI),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(FFI), '-o', driver])
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

    for owned, result_kind in [(False, 'i64'), (True, 'i64'), (True, 'unit'), (True, 'bool')]:
        data, text = borrowed_text_graph(owned, result=result_kind)
        compile_input(data)
        checked([*CC, '-DHAKO_TEST_BORROWED_TEXT', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps + ['string.checked_end', 'string.checked_print', 'string.checked_add']],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        for mode, expected_exit in [('normal', 7), ('fault-first', 70)]:
            result = subprocess.run([str(exe)], env=dict(ENV, V4_PROBE_MODE=mode), capture_output=True, text=True)
            assert result.returncode == expected_exit, (owned, mode, result)
            assert text + '\n' in result.stdout, (owned, mode, result)
            assert 'TEXT_ENDS ' + str(int(owned)) + '\n' in result.stdout, result
            assert ('COUNTS 1 1 1 0 ' + ('1' if mode == 'fault-first' else '0') + ' 1\n') in result.stdout, result
        for mutation in ['numeric4', 'scalar-incoming', 'scalar-operand', 'forged-pointer', 'object-view', 'lost-domain',
                         'false-domain', 'end', 'escape', 'arithmetic', 'store']:
            changed = copy.deepcopy(data)
            root, forward, leaf, middle = changed['functions']
            call_block = next(b for b in root['blocks'] if b['id'] == (7 if owned else 1))
            args = call_block['terminator']['instruction']['operation']['call']['args']
            if mutation == 'numeric4': args[0] = {'kind': 4, 'value': 10 if owned else 2}
            elif mutation == 'scalar-incoming':
                rows = call_block['instructions']
                rows.append({'index': len(rows), 'instruction': {'op': 'const_i64', 'dst': 12, 'value': 8}})
                call_block['terminator']['index'] = len(rows)
                args[0] = {'kind': 1, 'value': 12}
            elif mutation == 'scalar-operand': args[0]['operand'] = {'kind': 'normal_integer', 'value': 2}
            elif mutation == 'forged-pointer': leaf['blocks'][0]['terminator']['instruction']['operation']['value']['value'] = 2
            elif mutation == 'object-view': forward['params'][0]['object_view'] = OBJECT
            elif mutation == 'lost-domain': del middle['params'][0]['text_view']
            elif mutation == 'false-domain': forward['params'][0]['text_view'] = False
            elif mutation == 'end': leaf['blocks'][0]['terminator']['instruction']['operation'] = {'kind': 'string_end', 'value': 4, 'site': 76}
            elif mutation == 'escape': leaf['blocks'][1]['terminator']['instruction']['value'] = 4
            elif mutation == 'store': leaf['blocks'][3]['terminator']['instruction']['operation']['value'] = 4
            elif mutation == 'arithmetic':
                leaf['blocks'][1]['instructions'] = [{'index': 0, 'instruction': {'op': 'add', 'dst': 9, 'lhs': 4, 'rhs': 2}}]
                leaf['blocks'][1]['terminator']['index'] = 1
                leaf['blocks'][1]['terminator']['instruction']['value'] = 9
            compile_input(changed, False)
        if owned:
            changed = copy.deepcopy(data)
            root = changed['functions'][0]
            before = next(b for b in root['blocks'] if b['id'] == 7)
            original_call = copy.deepcopy(before['terminator']['instruction']['operation'])
            before['terminator']['instruction'] = invoke({'kind': 'string_end', 'value': 10, 'site': 73}, 11, 5)
            before['edges'] = [{'target': 11, 'args': None}, {'target': 5, 'args': None}]
            root['blocks'].append(block(11, [], invoke(original_call, 3, 8), (3, 8)))
            root['blocks'][3]['instructions'][0]['instruction']['invoke_block'] = 11
            compile_input(changed, False)
            for missing in [3, 8]:
                changed = copy.deepcopy(data)
                end = next(b for b in changed['functions'][0]['blocks'] if b['id'] == missing)
                target = 9 if missing == 3 else 5
                end['terminator']['instruction'] = {'op': 'jump', 'target': target, 'args': None}
                end['edges'] = [{'target': target, 'args': None}]
                compile_input(changed, False)
        print('borrowed Text', result_kind, 'owned' if owned else 'UTF-8/NUL', 'two forwards, Normal/Fault, exact caller End and domain/escape negatives PASS')
    for owned, two, result_kind, profile in [(False, False, 'i64', 1), (True, False, 'i64', 1),
            (False, True, 'i64', 1), (True, True, 'i64', 1), (False, False, 'text', 1), (False, False, 'text', 2)]:
        data, text = borrowed_text_graph(owned, add=True, two=two, result=result_kind)
        data['storage_profile'] = profile
        # Reuse the existing Add variants for finite Text/I64 transport.
        # The Text-return rows retain their independent held-result coverage.
        if result_kind == 'i64' and (not two or not owned):
            ordinal = int(two)
            for fn in data['functions'][2:] if not two else data['functions'][1:]:
                del fn['params'][ordinal]['text_view']
                fn['params'][ordinal]['finite_domain'] = ['i64', 'text']
            data['functions'][2]['blocks'][0]['terminator']['instruction']['operation']['right']['kind'] = 'tagged_dynamic'
            if two:
                root = data['functions'][0]['blocks'][1]
                args = root['terminator']['instruction']['operation']['call']['args']
                text = args[0]['operand']['utf8'] + '7'
                root['instructions'].append({'index': len(root['instructions']),
                    'instruction': {'op': 'const_i64', 'dst': 12, 'value': 7}})
                root['terminator']['index'] = len(root['instructions'])
                args[1] = {'kind': 1, 'value': 12}
        compile_input(data)
        checked([*CC, '-DHAKO_TEST_BORROWED_TEXT', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps +
                   ['string.checked_end', 'string.checked_print', 'string.checked_add']],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        for mode in ['normal', 'fault-first', 'text-print-fault', 'text-add-fault']:
            fault = mode != 'normal'
            extra = {'V4_PROBE_STRING_ADD_FAULT_AT': str(2 if owned else 1)} if mode == 'text-add-fault' else {}
            result = subprocess.run([str(exe)], env=dict(ENV, V4_PROBE_MODE=mode,
                HAKO_TYPED_OBJECT_STORE='single_thread_exact' if profile == 2 else 'safe_mutex', **extra),
                capture_output=True, text=True)
            assert result.returncode == (70 if fault else 7), (owned, mode, result)
            assert (text + '\n' in result.stdout) == (mode in ['normal', 'fault-first']), result
            ends = int(owned) + int(mode != 'text-add-fault') + 3 * int(result_kind == 'text')
            assert 'TEXT_ENDS ' + str(ends) + '\n' in result.stdout, (owned, mode, result)
            stores = int(mode in ['normal', 'fault-first'])
            assert f'COUNTS 1 {stores} 1 0 {int(fault)} 1\n' in result.stdout, result
        if result_kind == 'text':
            for mutation in ['result-kind', 'spent-result', 'fault-omission', 'double-hold', 'hold-replaces', 'normal-held-end', 'suffix-order', 'suffix-duplicate', 'suffix-foreign', 'early-result', 'result-cancel']:
                changed = copy.deepcopy(data)
                leaf, middle = changed['functions'][2:]
                call_block = next(b for b in middle['blocks']
                    if b['terminator']['instruction'].get('operation', {}).get('kind') == 'ordinary_call')
                if profile == 2 and mutation in ['result-kind', 'early-result', 'suffix-foreign']:
                    # Reuse three duplicate profile2 rows for canonical domain,
                    # exclusive Text use and concrete actual membership refusal.
                    fn = middle if mutation == 'early-result' else leaf
                    del fn['params'][0]['text_view']
                    fn['params'][0]['finite_domain'] = {
                        'result-kind': ['text', 'i64'], 'early-result': ['i64', 'text'],
                        'suffix-foreign': ['i64']}[mutation]
                elif mutation == 'result-kind': call_block['terminator']['instruction']['operation']['result'] = 'i64'
                elif mutation.startswith('suffix-'):
                    call_block['terminator']['instruction']['operation']['text_argument_cleanup'] = {
                        'suffix-order': [14, 13], 'suffix-duplicate': [13, 13], 'suffix-foreign': [4]}[mutation]
                elif mutation == 'early-result':
                    middle['blocks'][1]['terminator']['instruction']['operation']['value'] = 5
                elif mutation == 'result-cancel':
                    end = next(b for b in middle['blocks'] if b['id'] == 7)
                    end['terminator']['instruction'] = {'op': 'jump', 'target': 2, 'args': None}
                    end['edges'] = [{'target': 2, 'args': None}]
                elif mutation == 'spent-result':
                    end = next(b for b in leaf['blocks'] if b['id'] == 1)
                    end['terminator']['instruction'] = invoke({'kind': 'string_end', 'value': 11, 'site': 82}, 10, 2)
                    end['edges'] = [{'target': 10, 'args': None}, {'target': 2, 'args': None}]
                    leaf['blocks'].append(block(10, [], {'op': 'return', 'value': 11}))
                elif mutation in ['double-hold', 'hold-replaces', 'normal-held-end']:
                    end = next(b for b in leaf['blocks'] if b['id'] == 5)
                    if mutation == 'double-hold':
                        end['instructions'].append({'index': 1,
                            'instruction': copy.deepcopy(end['instructions'][0]['instruction'])})
                        end['terminator']['index'] = 2
                    elif mutation == 'hold-replaces': end['instructions'][0]['instruction']['replaces'] = 12
                    else: end['terminator']['instruction']['operation']['value'] = 11
                else:
                    end = next(b for b in leaf['blocks'] if b['id'] == 6)
                    end['terminator']['instruction'] = {'op': 'jump', 'target': 2, 'args': None}
                    end['edges'] = [{'target': 2, 'args': None}]
                compile_input(changed, False)
        print('borrowed Text Add', result_kind, owned, 'two ordinals', two, 'profile', profile, 'separate callee acquisition, Print/Call Fault, failed Add without End PASS')
    empty, _ = borrowed_text_graph(text='')
    compile_input(empty)
    checked([*CC, '-DHAKO_TEST_BORROWED_TEXT', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
             *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps +
               ['string.checked_end', 'string.checked_print', 'string.checked_add']],
             '-lpthread', '-ldl', '-lm', '-o', exe])
    result = subprocess.run([str(exe)], env=dict(ENV, V4_PROBE_MODE='normal'), capture_output=True, text=True)
    assert result.returncode == 7 and result.stdout.startswith('\n') and 'TEXT_ENDS 0\n' in result.stdout, result
    print('borrowed empty Text descriptor executes without a fabricated SSA literal PASS')
    if '--text-only' in sys.argv:
        raise SystemExit(0)

    data = graph(1, integer_view=True)
    compile_input(data)
    checked([*CC, obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
             *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
             '-lpthread', '-ldl', '-lm', '-o', exe])
    for mode, expected in [('normal', 7), ('fault-first', 70)]:
        result = subprocess.run([str(exe)], env=dict(ENV, V4_PROBE_MODE=mode), capture_output=True, text=True)
        assert result.returncode == expected, (mode, result)
        assert 'COUNTS ' + ('1 1 1 0 0 1' if mode == 'normal' else '1 1 1 0 1 1') + '\n' in result.stdout, result
    for mutation in ['bool', 'forged-integer', 'null', 'forward-tagged', 'missing-view', 'false-view', 'text-view', 'object-view']:
        changed = copy.deepcopy(data)
        root, forward, leaf = changed['functions']
        if mutation in ('bool', 'forged-integer'):
            root['blocks'][1]['instructions'][1]['instruction'] = {'op': 'const_bool', 'dst': 10, 'value': True}
            if mutation == 'bool': root['blocks'][1]['terminator']['instruction']['operation']['call']['args'][0]['kind'] = 2
        elif mutation == 'null':
            root['blocks'][1]['instructions'][1]['instruction'] = {'op': 'const_null', 'dst': 10}
            root['blocks'][1]['terminator']['instruction']['operation']['call']['args'][0]['kind'] = 0
        elif mutation == 'forward-tagged':
            forward['blocks'][0]['terminator']['instruction']['operation']['call']['args'][0]['kind'] = 'tagged'
        elif mutation == 'missing-view': del leaf['params'][0]['integer_view']
        elif mutation == 'false-view': forward['params'][0]['integer_view'] = False
        elif mutation == 'text-view': forward['params'][0]['text_view'] = True
        else: forward['params'][0]['object_view'] = OBJECT
        compile_input(changed, False)
    # JsonLine's entry is Static: the same projection has no receiver lane.
    static = copy.deepcopy(data)
    for fn in static['functions'][1:]:
        fn['receiver'] = fn['receiver_object'] = None
    del static['functions'][0]['blocks'][1]['terminator']['instruction']['operation']['call']['receiver']
    del static['functions'][1]['blocks'][0]['terminator']['instruction']['operation']['call']['receiver']
    tail = next(b for b in static['functions'][2]['blocks'] if b['id'] == 5)
    tail['terminator']['instruction'] = {'op': 'jump', 'target': 1, 'args': None}
    tail['edges'] = [{'target': 1, 'args': None}]
    static['functions'][2]['blocks'] = [b for b in static['functions'][2]['blocks'] if b['id'] != 2]
    compile_input(static)
    checked([*CC, obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
             *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
             '-lpthread', '-ldl', '-lm', '-o', exe])
    result = subprocess.run([str(exe)], env=dict(ENV, V4_PROBE_MODE='normal'), capture_output=True, text=True)
    assert result.returncode == 7 and 'COUNTS 1 0 1 0 0 1\n' in result.stdout, result
    print('borrowed Integer entry projection: Static/Instance Copy/Phi, Normal/Fault cleanup and independent domain negatives PASS')

    # An opaque caller keeps its pair until the exact Integer Call use.
    # Bool must take the caller's Fault cleanup, without entering the leaf.
    for tag, expected, counts in [(1, 7, '1 1 1 0 0 1'), (2, 70, '1 0 1 0 1 1')]:
        checked_pair = copy.deepcopy(data)
        root, forward, _ = checked_pair['functions']
        del forward['params'][0]['integer_view']
        forward['blocks'][0]['terminator']['instruction']['operation']['call']['args'][0]['kind'] = 'tagged'
        if tag == 2:
            root['blocks'][1]['instructions'][1]['instruction'] = {'op': 'const_bool', 'dst': 10, 'value': True}
            root['blocks'][1]['terminator']['instruction']['operation']['call']['args'][0]['kind'] = 2
        compile_input(checked_pair)
        checked([*CC, obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        result = subprocess.run([str(exe)], env=dict(ENV, V4_PROBE_MODE='normal'), capture_output=True, text=True)
        assert result.returncode == expected and f'COUNTS {counts}\n' in result.stdout, (tag, result)
    print('opaque pair to checked Integer Call: payload preserved, wrong tag takes caller Fault cleanup PASS')

    for null_result in [False, True]:
        data = received_field_read_graph(null_result)
        compile_input(data)
        checked([*CC, obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
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
        checked([*CC, obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
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
        checked([*CC, obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
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

    # Original object-view base plus raw or entry-projected Integer payload.
    # Explicit physical policy only; this does not establish source admission.
    for stored, projected in [(0, False), (7, False), (-7, False), (7, True), (-7, True)]:
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
        if projected:
            for fn in [forward, leaf]:
                fn['params'][1]['integer_view'] = True
            forwarding['terminator']['instruction']['operation']['call']['args'][1]['kind'] = 1
        op = leaf['blocks'][0]['terminator']['instruction']['operation']
        op.update(base=3, value=6, exact_numeric_runtime_check={'kind': 'dynamic_integer_range', 'declared_type': 'usize'})
        compile_input(data)
        checked([*CC, obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
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
        print('borrowed usize receiver/value', stored, 'projected', projected, 'exact store/Fault and closed views')

    # I64 Return already accepts an original tagged formal behind its runtime
    # tag1 guard. Keep this as execution coverage, not an obsolete rejection.
    tagged_return = graph(1, 7)
    tagged_return['functions'][2]['blocks'][1]['terminator']['instruction']['value'] = 3
    compile_input(tagged_return)
    checked([*CC, obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
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
