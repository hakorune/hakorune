#!/usr/bin/env python3
"""Compile original stored-child source captures; exercise real callee Fault.

Pass the capture directory. Positive JSON is never repaired. Negative copies
keep the source graph and isolate the named wire boundary where possible.
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
DIRECTORY = Path(sys.argv[1])
ENV = dict(os.environ, NYASH_NYRT_SILENT_RESULT='1', HAKO_NYRT_PLUGIN_HOST='off',
           V4_PROBE_MODE='normal')
ENV.pop('V4_PROBE_FAULT_AT', None)


def run(args, **kwargs):
    return subprocess.run([str(arg) for arg in args], capture_output=True, text=True, **kwargs)


def checked(args, **kwargs):
    result = run(args, **kwargs)
    assert result.returncode == 0, (args, result.stdout, result.stderr)
    return result


def owned_slot_mutant(base, flavor):
    data = copy.deepcopy(base)
    fn = next(f for f in data['functions'] if f['name'] in ('Parent.first/1', 'Parent.read/1'))
    block = next(b for b in fn['blocks'] if any(r['instruction']['op'] == 'object_field_get' for r in b['instructions']))
    pos = next(i for i, r in enumerate(block['instructions']) if r['instruction']['op'] == 'object_field_get')
    read = block['instructions'][pos]['instruction']
    child = next(r['child_object_id'] for layout in data['layouts']
        if layout['object_id'] == read['object_id'] for r in layout['owned_object_residences']
        if r['field_ordinal'] == read['field_ordinal'])
    value = 1 + max([fn['receiver']] + [r['instruction'].get('dst', 0)
        for b in fn['blocks'] for r in b['instructions']])
    scalar = {'op': 'const_i64', 'dst': value, 'value': 0}
    store = {'kind': 'field_set', 'base': read['base'], 'object_id': read['object_id'],
        'field_ordinal': read['field_ordinal'], 'value': value, 'site': 777777}
    if flavor == 'scalar-row':
        block['instructions'][pos:pos] = [{'instruction': scalar}, {'instruction':
            dict(store, op='field_set', exact_numeric_runtime_check={'kind': 'none'})}]
        block['instructions'][pos + 1]['instruction'].pop('kind')
        for i, row in enumerate(block['instructions']): row['index'] = i
        block['terminator']['index'] = len(block['instructions'])
        return data
    fault = next(b['id'] for b in fn['blocks'] if b['terminator']['instruction']['op'] == 'return_fault')
    frame = next(b['terminator']['instruction']['fault_frame'] for b in fn['blocks']
        if b['terminator']['instruction']['op'] == 'invoke')
    new_id = max(b['id'] for b in fn['blocks']) + 1
    tail = {'id': new_id, 'instructions': block['instructions'][pos:],
        'terminator': block['terminator'], 'edges': block['edges']}
    for i, row in enumerate(tail['instructions']): row['index'] = i
    tail['terminator']['index'] = len(tail['instructions'])
    block['instructions'] = block['instructions'][:pos]
    fn['blocks'].append(tail)
    # Moving an existing Invoke also moves the origin of its result projection.
    for existing in fn['blocks']:
        for row in existing['instructions']:
            ins = row['instruction']
            if ins['op'] == 'invoke_normal_result' and ins['invoke_block'] == block['id']:
                ins['invoke_block'] = tail['id']
    def invoke(block_id, rows, operation, normal, failed):
        return {'id': block_id, 'instructions': rows,
            'terminator': {'index': len(rows), 'instruction': {'op': 'invoke',
                'operation': operation, 'fault_frame': frame, 'normal': normal, 'fault': failed}},
            'edges': [{'target': normal, 'args': None}, {'target': failed, 'args': None}]}
    if flavor == 'scalar-invoke':
        block['instructions'].append({'index': len(block['instructions']), 'instruction': scalar})
        replacement = invoke(block['id'], block['instructions'], store, tail['id'], fault)
    else:
        array = flavor in ('array-field', 'array-as-object-zero')
        foreign = next(r['object_id'] for r in data['layouts']
            if r['object_id'] not in (read['object_id'], child) and
            not r['owned_residences'] and not r.get('owned_object_residences'))
        acquire = {'kind': 'array_new', 'site': 777778} if array else {
            'kind': 'new_box', 'site': 777778, 'object_id': foreign}
        target = new_id + 1
        rows = [{'index': 0, 'instruction': {'op': 'invoke_normal_result',
            'invoke_block': block['id'], 'dst': value}}]
        failed = fault
        if flavor != 'array-field':
            store = dict(store, kind='object_field_set', child_object_id=child if array else foreign)
            cleanup = new_id + 2
            failed = cleanup
            normal_jump, fault_jump = cleanup + 1, cleanup + 2
            fn['blocks'].append(invoke(cleanup, [], {'kind': 'home_release',
                'value': value, 'object_id': child if array else foreign, 'site': 777779}, normal_jump, fault_jump))
            for id in (normal_jump, fault_jump):
                fn['blocks'].append({'id': id, 'instructions': [], 'terminator': {'index': 0,
                    'instruction': {'op': 'jump', 'target': fault, 'args': None}},
                    'edges': [{'target': fault, 'args': None}]})
        fn['blocks'].append(invoke(target, rows, store, tail['id'], failed))
        replacement = invoke(block['id'], block['instructions'], acquire, target, fault)
    block.update(replacement)
    return data


with tempfile.TemporaryDirectory(prefix='hako stored child receiver ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'source.o', 'source')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c',
             '-L' + str(ROOT / 'target/release'), '-lhako_llvmc_ffi',
             '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    paths = sorted(DIRECTORY.glob('hako-issued-borrowed-stored-child-*.json'))
    assert len(paths) == 96, ('original source witness count', len(paths))
    for path in paths:
        checked([driver, path, obj], env=ENV)
        assert not list(work.glob('source.o.*')), 'temporary artifact leak'
    print('96 unchanged source-issued stored-child inputs compile')

    base = json.loads(next(path for path in paths if
        path.name == 'hako-issued-borrowed-stored-child-opaque-object-first-reversefalse-annotatedfalse-optfalse.json').read_text())

    def reject(data, reason):
        path = work / 'negative.json'
        path.write_text(json.dumps(data))
        obj.unlink(missing_ok=True)
        result = run([driver, path, obj], env=ENV)
        assert result.returncode != 0 and reason in result.stderr, result
        assert not obj.exists() and not list(work.glob('source.o.*')), 'rejected artifact'
        print('reject', reason)

    def parent(data):
        return next(f for f in data['functions'] if f['name'] == 'Parent.first/1')

    def receiver_read(function):
        return next(row['instruction'] for block in function['blocks']
                    for row in block['instructions']
                    if row['instruction']['op'] == 'object_field_get')

    # A valid wire Copy checks producer-root traversal independently of the
    # unchanged source inputs above; it is not another source acceptance.
    data = copy.deepcopy(base)
    function = parent(data)
    block = next(b for b in function['blocks'] if
                 any(row['instruction']['op'] == 'object_field_get' for row in b['instructions']))
    read = receiver_read(function)
    alias = max(row['instruction'].get('dst', 0) for b in function['blocks']
                for row in b['instructions']) + 1
    position = next(i for i, row in enumerate(block['instructions']) if row['instruction'] is read)
    block['instructions'].insert(position, {'instruction': {'op': 'copy', 'dst': alias,
                                                           'src': function['receiver']}})
    read['base'] = alias
    for i, row in enumerate(block['instructions']):
        row['index'] = i
    block['terminator']['index'] = len(block['instructions'])
    copy_path = work / 'copy-root.json'
    copy_path.write_text(json.dumps(data))
    checked([driver, copy_path, obj], env=ENV)
    print('exact borrowed receiver Copy root admitted (wire witness)')

    data = copy.deepcopy(base)
    function = parent(data)
    receiver_read(function)['base'] = function['params'][0]['value']
    reject(data, 'published-lifecycle-v4/unsupported-cohort')

    data = copy.deepcopy(base)
    leaf = next(f for f in data['functions'] if f['name'] == 'Leaf.read/1')
    leaf['receiver_object'] = parent(data)['params'][0]['object_view']
    reject(data, 'published-lifecycle-v4/receiver-object-mismatch')

    for malformed in ('missing', 'foreign-child'):
        data = copy.deepcopy(base)
        function = parent(data)
        layout = next(row for row in data['layouts'] if
                      row['object_id'] == function['receiver_object'])
        ordinal = receiver_read(function)['field_ordinal']
        if malformed == 'missing':
            layout['owned_object_residences'] = [row for row in layout['owned_object_residences']
                                                if row['field_ordinal'] != ordinal]
        else:
            row = next(row for row in layout['owned_object_residences']
                       if row['field_ordinal'] == ordinal)
            row['child_object_id'] = function['params'][0]['object_view']
        reject(data, 'published-lifecycle-v4/receiver-object-mismatch')

    # Receiver borrowing does not authorize argument-carrier handoff.
    # Corroborate the unused formal's class so foreign class is not the cause.
    for actual in (3, 'nullable_typed_object', 'i64'):
        data = json.loads((DIRECTORY / 'hako-issued-borrowed-stored-child-scalar-object-first-reversefalse-annotatedfalse-optfalse.json').read_text())
        function = parent(data)
        read = receiver_read(function)
        call = next(b['terminator']['instruction']['operation']['call']
                    for b in function['blocks'] if
                    b['terminator']['instruction'].get('operation', {}).get('kind') == 'ordinary_call')
        callee = data['functions'][call['target']]
        if actual == 'i64':
            callee['params'][0]['object_view'] = None
        else:
            callee['params'][0]['object_view'] = callee['receiver_object']
        call['args'][0] = {'kind': 1 if actual == 'i64' else actual, 'value': read['dst']}
        reject(data, 'published-lifecycle-v4/unsupported-cohort')

    for release in ('child', 'child-copy', 'parent-field'):
        data = copy.deepcopy(base)
        function = parent(data)
        read = receiver_read(function)
        block = next(b for b in function['blocks'] if
                     b['terminator']['instruction'].get('operation', {}).get('kind') == 'ordinary_call')
        call = block['terminator']['instruction']
        landing = next(b for b in function['blocks'] if b['id'] == call['normal'])
        return_block = landing['terminator']['instruction']['target']
        fault_return = next(b['id'] for b in function['blocks'] if
                            b['terminator']['instruction']['op'] == 'return_fault')
        frame = call['fault_frame']
        new_id = max(b['id'] for b in function['blocks']) + 1
        child = data['functions'][call['operation']['call']['target']]['receiver_object']
        value = read['dst']
        if release == 'child-copy':
            value = max(row['instruction'].get('dst', 0) for b in function['blocks']
                        for row in b['instructions']) + 1
            landing['instructions'].append({'index': len(landing['instructions']),
                'instruction': {'op': 'copy', 'dst': value, 'src': read['dst']}})
            landing['terminator']['index'] = len(landing['instructions'])
        operation = {'kind': 'home_release', 'value': value, 'object_id': child, 'site': 100000}
        if release == 'parent-field':
            operation = {'kind': 'object_field_release', 'base': function['receiver'],
                         'object_id': function['receiver_object'], 'child_object_id': child,
                         'field_ordinal': read['field_ordinal'], 'site': 100000}
        landing['terminator']['instruction']['target'] = new_id
        landing['edges'] = [{'target': new_id, 'args': None}]
        function['blocks'].append({'id': new_id, 'instructions': [],
            'terminator': {'index': 0, 'instruction': {'op': 'invoke', 'operation': operation,
                'fault_frame': frame, 'normal': return_block, 'fault': fault_return}},
            'edges': [{'target': return_block, 'args': None}, {'target': fault_return, 'args': None}]})
        reject(data, 'published-lifecycle-v4/unsupported-cohort')

    for malformed in ('duplicate', 'range'):
        data = copy.deepcopy(base)
        layout = next(row for row in data['layouts'] if
                      row['object_id'] == parent(data)['receiver_object'])
        if malformed == 'duplicate':
            layout['owned_object_residences'].append(copy.deepcopy(layout['owned_object_residences'][0]))
        else:
            layout['owned_object_residences'][0]['field_ordinal'] = layout['field_count']
        reject(data, 'abi-layout')

    for flavor in ('scalar-row', 'scalar-invoke', 'array-field', 'foreign-object'):
        reject(owned_slot_mutant(base, flavor), 'published-lifecycle-v4/unsupported-cohort')
    zero_child = json.loads((DIRECTORY / 'hako-issued-stored-child-callee-fault-optfalse.json').read_text())
    reject(owned_slot_mutant(zero_child, 'array-as-object-zero'),
           'published-lifecycle-v4/unsupported-cohort')

    # The borrowed read is handle-only evidence: it cannot serve an i64
    # result or a `+` operand. A Copy chain must root at the original
    # receiver — a foreign root rejects. (A true Copy cycle is
    # unrepresentable: source availability demands an earlier dominating
    # definition, so mutual references never pass the parser stage.)
    for misuse in ('return', 'add', 'copy-foreign-root'):
        data = copy.deepcopy(base)
        function = parent(data)
        read = receiver_read(function)
        value = max(row['instruction'].get('dst', 0) for b in function['blocks']
                    for row in b['instructions']) + 1
        if misuse == 'return':
            terminator = next(b['terminator']['instruction']
                              for b in function['blocks']
                              if b['terminator']['instruction']['op'] == 'return')
            terminator['value'] = read['dst']
        else:
            block = next(b for b in function['blocks'] if any(
                row['instruction'] is read for row in b['instructions']))
            position = next(i for i, row in enumerate(block['instructions'])
                            if row['instruction'] is read)
            if misuse == 'add':
                block['instructions'].insert(position + 1, {'instruction': {
                    'op': 'add', 'dst': value, 'lhs': read['dst'], 'rhs': read['dst']}})
            else:
                block['instructions'].insert(position, {'instruction': {
                    'op': 'copy', 'dst': value, 'src': function['params'][0]['value']}})
                read['base'] = value
            for i, row in enumerate(block['instructions']):
                row['index'] = i
            block['terminator']['index'] = len(block['instructions'])
        reject(data, 'published-lifecycle-v4/unsupported-cohort')

    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.home_release_plain_i64',
             'object.reclaim_unpublished']
    for optimize in ('false', 'true'):
        path = DIRECTORY / ('hako-issued-stored-child-callee-fault-opt' + optimize + '.json')
        data = json.loads(path.read_text())
        leaf = next(f for f in data['functions'] if f['name'] == 'Leaf.read/1')
        assert leaf['receiver_object'] == 0, 'canonical child id zero is valid'
        scratch = next(f for f in data['functions'] if f['name'] == 'Scratch.birth/0')
        sites = [b['terminator']['instruction']['operation']['site'] for b in scratch['blocks']
                 if b['terminator']['instruction'].get('operation', {}).get('kind') == 'field_set']
        assert len(sites) == 1
        checked([driver, path, obj], env=ENV)
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        normal = run([exe], env=ENV)
        assert normal.returncode == 5 and 'COUNTS 1 4 4 0 0 1\n' in normal.stdout, normal
        fault = run([exe], env=dict(ENV, V4_PROBE_FAULT_AT='4'))
        assert fault.returncode == 70, fault
        assert 'COUNTS 1 4 3 1 1 1\n' in fault.stdout, fault
        assert f'FAULT 101 {sites[0]} 0 0 HOME 3 RECLAIM 1\n' in fault.stdout, fault
        print(optimize, 'original stored callee Normal/Fault cleanup once')
