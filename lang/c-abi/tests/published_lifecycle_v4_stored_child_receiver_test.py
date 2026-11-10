#!/usr/bin/env python3
"""Compile original stored-child source captures; exercise real callee Fault.

Pass the capture directory. Positive JSON is never repaired. Negative copies
keep the source graph and isolate the named wire boundary where possible.
"""
import copy
import json
import re
from pathlib import Path
import sys
import tempfile
from published_lifecycle_v4_test_support import TESTS, ARCHIVE, checked, env, release_driver, run

ENV = env(V4_PROBE_MODE='normal')
ENV.pop('V4_PROBE_FAULT_AT', None)


def local_child_path(path):
    """Check an original caller-owned child path and nullable-wire corruption."""
    original = json.loads(path.read_text())
    main = original['functions'][0]
    rows = [r['instruction'] for b in main['blocks'] for r in b['instructions']]
    copies = {r['dst']: r['src'] for r in rows if r['op'] == 'copy'}

    def root(value):
        seen = set()
        while value in copies:
            assert value not in seen
            seen.add(value)
            value = copies[value]
        return value

    reads = [(b, r['instruction']) for b in main['blocks'] for r in b['instructions']
             if r['instruction']['op'] == 'object_field_get']
    paths = [(b, r) for b, r in reads if any(t['base'] == r['dst'] for _, t in reads)]
    assert len(paths) == 1
    read_block, read = paths[0]
    births = [b for b in main['blocks'] if
              b['terminator']['instruction'].get('operation', {}).get('kind') == 'birth_call'
              and root(b['terminator']['instruction']['operation']['call']['receiver']) == root(read['base'])]
    assert len(births) == 1
    born = births[0]
    with tempfile.TemporaryDirectory(prefix='hako local child path ') as directory:
        work = Path(directory)
        driver = release_driver(work)
        obj, exe = work / 'child.o', work / 'child'
        checked([driver, path, obj], env=ENV)
        checked(['cc', obj, ARCHIVE, '-lpthread', '-ldl', '-lm', '-o', exe])
        assert run([exe], env=ENV).returncode == 6
        for flavor in ('nullable-actual', 'ambiguous-writer'):
            data = copy.deepcopy(original)
            caller = data['functions'][0]
            block = next(b for b in caller['blocks'] if b['id'] == born['id'])
            call = block['terminator']['instruction']['operation']['call']
            if flavor == 'nullable-actual':
                fresh = 1 + max(r['dst'] for r in rows if 'dst' in r)
                block['instructions'].append({'index': len(block['instructions']),
                    'instruction': {'op': 'const_null', 'dst': fresh}})
                block['terminator']['index'] = len(block['instructions'])
                assert len(call['args']) == 1
                call['args'][0] = {'kind': 0, 'value': fresh}
            else:
                duplicate = copy.deepcopy(data['functions'][call['target']])
                duplicate['name'] += '_duplicate'
                def sites(node):
                    if isinstance(node, dict):
                        for key, value in node.items():
                            if key == 'site':
                                yield value
                            else:
                                yield from sites(value)
                    elif isinstance(node, list):
                        for value in node:
                            yield from sites(value)
                renumber = {old: max(max(sites(data)), data['process_result_site']) + i + 1
                            for i, old in enumerate(sorted(set(sites(duplicate))))}
                def fresh_sites(node):
                    if isinstance(node, dict):
                        for key, value in node.items():
                            if key == 'site':
                                node[key] = renumber[value]
                            else:
                                fresh_sites(value)
                    elif isinstance(node, list):
                        for value in node:
                            fresh_sites(value)
                fresh_sites(duplicate)
                call['target'] = len(data['functions'])
                data['functions'].append(duplicate)
            negative = work / (flavor + '.json')
            negative.write_text(json.dumps(data))
            obj.unlink(missing_ok=True)
            result = run([driver, negative, obj], env=ENV)
            assert result.returncode != 0 and f'block={read_block["id"]} phase=flow' in result.stderr, result
            assert not obj.exists() and not list(work.glob('child.o.*'))
            print('reject local child path', flavor)
        print('original local child path EXE exit 6; nullable and ambiguous writers rejected at read')


if sys.argv[1] == '--child-path':
    local_child_path(Path(sys.argv[2]))
    sys.exit(0)

DIRECTORY = Path(sys.argv[1])


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
        'field_ordinal': read['field_ordinal'], 'value': value, 'site': 777777,
        'exact_numeric_runtime_check': {'kind': 'none'}}
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
            store.pop('exact_numeric_runtime_check')
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
    driver = release_driver(work)
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
        reasons = reason if isinstance(reason, tuple) else (reason,)
        assert result.returncode != 0 and any(item in result.stderr for item in reasons), result
        assert not obj.exists() and not list(work.glob('source.o.*')), 'rejected artifact'
        print('reject', reason)

    # Direct placement reuses these issued source/ABI checks, including
    # malformed layouts and the distinction between allocation and Birth.
    direct_path = DIRECTORY / 'hako-issued-direct-scalar-updatefalse.json'
    if direct_path.exists():
        direct = json.loads(direct_path.read_text())
        for update, expected in (('false', 30), ('true', 31)):
            path = DIRECTORY / f'hako-issued-direct-scalar-update{update}.json'
            checked([driver, path, obj], env=ENV)
            checked(['cc', '-fuse-ld=lld', obj, ARCHIVE, '-lpthread', '-ldl', '-lm', '-o', exe])
            assert run([exe], env=ENV).returncode == expected
        checked([driver, direct_path, obj], env=ENV)
        probe_wraps = ('fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
                       'object.checked_field_set', 'object.checked_residence_set',
                       'object.home_release_plain_i64', 'object.reclaim_unpublished')
        checked(['cc', '-fuse-ld=lld', '-DHAKO_TEST_DIRECT_OBJECT', obj,
                 TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in probe_wraps],
                 '-Wl,--wrap=malloc', '-Wl,--wrap=free', '-lpthread', '-ldl', '-lm', '-o', exe])
        direct_env = dict(ENV, V4_PROBE_DIRECT_SIZE=str(direct['layouts'][0]['placement']['size']))
        normal = run([exe], env=direct_env)
        assert normal.returncode == 30 and 'DIRECT 1 1\n' in normal.stdout, normal
        oom = run([exe], env=dict(direct_env, V4_PROBE_DIRECT_OOM='1'))
        assert oom.returncode == 70 and 'DIRECT 1 0\n' in oom.stdout, oom
        assert 'FAULT 100 0 0 0 HOME 0 RECLAIM 0\n' in oom.stdout, oom
        for key, bad in (('size', 0), ('align', 4), ('field_offsets', [16, 16]),
                         ('representation', 'runtime_handle'), ('descriptor_object_id', 100),
                         ('end', []), ('parent_offset', 8), ('cursor_offset', 0)):
            data = copy.deepcopy(direct)
            data['layouts'][0]['placement'][key] = bad
            reject(data, 'abi-layout')
        data = copy.deepcopy(direct)
        data['storage_profile'] = 1
        reject(data, 'placement-profile')
        for skip in ('birth-store', 'birth-call'):
            data = copy.deepcopy(direct)
            function = next(f for f in data['functions'] if f['name'] ==
                            ('Pair.birth/2' if skip == 'birth-store' else 'main'))
            block = next(b for b in function['blocks'] if
                         b['terminator']['instruction'].get('operation', {}).get('kind') ==
                         ('field_set' if skip == 'birth-store' else 'birth_call'))
            target = block['terminator']['instruction']['normal']
            block['terminator']['instruction'] = {'op': 'jump', 'target': target, 'args': None}
            block['edges'] = [{'target': target, 'args': None}]
            reject(data, ('published-lifecycle-v4/unsupported-cohort',
                          'published-lifecycle-v4/receiver-object-mismatch'))
        data = copy.deepcopy(direct)
        birth = next(f for f in data['functions'] if f['name'] == 'main')
        call = next(b['terminator']['instruction']['operation']['call'] for b in birth['blocks']
                    if b['terminator']['instruction'].get('operation', {}).get('kind') == 'ordinary_call')
        call['args'] = [{'kind': 3, 'value': call['receiver']}]
        callee = data['functions'][call['target']]
        unused = 1 + max(r['instruction'].get('dst', 0) for b in callee['blocks'] for r in b['instructions'])
        callee['params'].append(
            {'value': unused, 'representation': 'borrowed_kind_payload_v1', 'object_view': None})
        reject(data, 'published-lifecycle-v4/unsupported-cohort')
        data = copy.deepcopy(direct)
        block = next(f for f in data['functions'] if f['name'] == 'Pair.sum/0')['blocks'][0]
        pos = next(i for i, r in enumerate(block['instructions']) if r['instruction']['op'] == 'object_field_get')
        read = block['instructions'][pos]['instruction']
        block['instructions'].insert(pos + 1, {'instruction': {'op': 'field_set', 'base': read['base'],
            'object_id': read['object_id'], 'field_ordinal': read['field_ordinal'], 'value': read['dst'], 'site': 777777,
            'exact_numeric_runtime_check': {'kind': 'dynamic_integer_range', 'declared_type': 'usize'}}})
        for i, row in enumerate(block['instructions']): row['index'] = i
        block['terminator']['index'] = len(block['instructions'])
        reject(data, 'published-lifecycle-v4/unsupported-cohort')
        print('direct scalar source EXEs, layout, initialization and boundary refusals')

    for capture, expected, order in [('child', 10, [3, 1, 2]), ('birth-prefix', 5, [3, 2, 1]),
                                     ('tree', 3, [4, 2, 1, 3, 5])]:
        child_path = DIRECTORY / f'hako-issued-direct-{capture}.json'
        if not child_path.exists(): continue
        child = json.loads(child_path.read_text())
        checked([driver, child_path, obj], env=ENV)
        checked(['cc', '-fuse-ld=lld', '-DHAKO_TEST_DIRECT_OBJECT', obj,
                 TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in probe_wraps],
                 '-Wl,--wrap=malloc', '-Wl,--wrap=free', '-lpthread', '-ldl', '-lm', '-o', exe])
        sizes = ','.join(str(row['placement']['size']) for row in child['layouts'])
        normal = run([exe], env=dict(ENV, V4_PROBE_DIRECT_SIZE=sizes))
        assert normal.returncode == expected and f'DIRECT {len(order)} {len(order)}\n' in normal.stdout, normal
        assert [int(line.split()[1]) for line in normal.stdout.splitlines()
                if line.startswith('DIRECT_FREE ')] == order, normal
        for nth in range(1, len(order) + 1):
            fault = run([exe], env=dict(ENV, V4_PROBE_DIRECT_SIZE=sizes, V4_PROBE_DIRECT_OOM_AT=str(nth)))
            assert fault.returncode == 70 and f'DIRECT {nth} {nth - 1}\n' in fault.stdout, fault
        layout = next(row for row in child['layouts'] if row['owned_object_residences'])
        for flavor in ('missing-end-child', 'duplicate-end-child', 'wrong-child', 'mixed-representation'):
            data = copy.deepcopy(child)
            row = next(row for row in data['layouts'] if row['object_id'] == layout['object_id'])
            end = row['placement']['end']
            if flavor == 'missing-end-child': del end[0]
            elif flavor == 'duplicate-end-child': end.insert(0, copy.deepcopy(end[0]))
            elif flavor == 'wrong-child': end[0]['child_object_id'] = 1 + max(r['object_id'] for r in data['layouts'])
            else: del row['placement']
            reject(data, ('abi-layout', 'placement-profile'))
        for flavor in ('missing-drain', 'published-reclaim'):
            data = copy.deepcopy(child)
            main = next(f for f in data['functions'] if f['name'] == 'main')
            block = next(b for b in main['blocks'] if
                b['terminator']['instruction'].get('operation', {}).get('kind') ==
                ('object_field_release' if flavor == 'missing-drain' else 'home_release') and
                b['terminator']['instruction']['operation']['object_id'] == layout['object_id'])
            term = block['terminator']['instruction']
            if flavor == 'missing-drain':
                target = term['normal']
                block['terminator']['instruction'] = {'op': 'jump', 'target': target, 'args': None}
                block['edges'] = [{'target': target, 'args': None}]
            else: term['operation']['kind'] = 'reclaim_unpublished'
            reject(data, 'published-lifecycle-v4/unsupported-cohort')
        if capture in ('birth-prefix', 'tree'):
            data = copy.deepcopy(child)
            fn = next(f for f in data['functions'] if f['name'] ==
                      ('Parent.birth/0' if capture == 'birth-prefix' else 'main'))
            block = next(b for b in fn['blocks'] if
                         b['terminator']['instruction'].get('operation', {}).get('kind') == 'birth_call')
            term = block['terminator']['instruction']
            if capture == 'birth-prefix':
                target = term['normal']
                block['terminator']['instruction'] = {'op': 'jump', 'target': target, 'args': None}
                block['edges'] = [{'target': target, 'args': None}]
            else:
                call = term['operation']['call']
                for arg in call['args'][:2]: arg.update(kind=3, value=call['receiver'])
            reject(data, 'published-lifecycle-v4/unsupported-cohort')
        print('direct owned child EXE, nullable/move, allocation Faults and complete End')

    for placement in ('typed', 'direct'):
        factory_path = DIRECTORY / f'hako-issued-{placement}-binary-trees.json'
        if not factory_path.exists(): continue
        factory = json.loads(factory_path.read_text())
        checked([driver, factory_path, obj], env=ENV)
        if placement == 'direct':
            index = next(i for i, fn in enumerate(factory['functions']) if fn['name'] == 'TreeNode.itemCheck/0')
            symbol = f'hako_lifecycle_ordinary_{index}'
            row = next(line.split() for line in checked(['nm', '-S', '--defined-only', obj]).stdout.splitlines()
                       if line.split()[-1] == symbol)
            start, size = int(row[0], 16), int(row[1], 16)
            assembly = checked(['objdump', '-dr', '--disassemble=' + symbol, obj]).stdout
            # objdump also prints earlier relocations from the same section.
            # Inspect only addresses inside this exact function's symbol range.
            body = '\n'.join(line for line in assembly.splitlines()
                if (at := re.match(r'^\s*([0-9a-f]+):', line)) and start <= int(at[1], 16) < start + size)
            assert size and body and 'nyash.object.' not in body, body
            data = copy.deepcopy(factory)
            fn = next(fn for fn in data['functions'] if fn['name'] == 'BinaryTreeBuilder.make/2')
            block = next(b for b in fn['blocks'] if
                b['terminator']['instruction'].get('operation', {}).get('kind') == 'birth_call')
            target = block['terminator']['instruction']['normal']
            block['terminator']['instruction'] = {'op': 'jump', 'target': target, 'args': None}
            block['edges'] = [{'target': target, 'args': None}]
            reject(data, 'published-lifecycle-v4/unsupported-cohort')
            checked([driver, factory_path, obj], env=ENV)
        checked(['cc', '-fuse-ld=lld', obj, ARCHIVE, '-lpthread', '-ldl', '-lm', '-o', exe])
        normal = run([exe], env=ENV)
        assert normal.returncode == 0 and 'summary=ok\n' in normal.stdout, normal
        print(placement, 'recursive factory EXE')
        if placement == 'direct':
            import resource
            plan = next(row for row in factory['layouts'] if row.get('placement'))
            owned = plan['owned_object_residences']
            assert len(owned) == 2 and all(row['child_object_id'] == plan['object_id'] for row in owned)
            offsets = plan['placement']['field_offsets']
            exposed = work / 'end-depth.o'
            checked(['objcopy', '--globalize-symbol=hako_direct_end', obj, exposed])
            checked(['cc', '-fuse-ld=lld', '-DHAKO_TEST_DIRECT_OBJECT', '-DHAKO_TEST_DIRECT_END_DEPTH',
                f"-DV4_DEEP_SIZE={plan['placement']['size']}", f"-DV4_DEEP_ID={plan['object_id']}",
                f"-DV4_DEEP_LEFT={offsets[owned[0]['field_ordinal']]}",
                f"-DV4_DEEP_RIGHT={offsets[owned[1]['field_ordinal']]}", '-DV4_DEEP_CHILDREN=2',
                exposed, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                *['-Wl,--wrap=nyash.' + name + '_v1' for name in probe_wraps],
                '-Wl,--wrap=main', '-Wl,--wrap=malloc', '-Wl,--wrap=free', '-lpthread', '-ldl', '-lm', '-o', exe])
            deep = run([exe], env=ENV, timeout=20,
                preexec_fn=lambda: resource.setrlimit(resource.RLIMIT_STACK, (256 * 1024, 256 * 1024)))
            assert deep.returncode == 0 and 'DIRECT_DEPTH 100000 FREE 200001\n' in deep.stdout, deep
            print('generated End depth100000 nodes200001 stack256KiB')


    # Optional source-issued typed nullable field read; retain the existing
    # driver and reject the same physical contract after isolated mutations.
    typed_path = DIRECTORY / 'hako-issued-typed-null-field-borrow.json'
    if typed_path.exists():
        typed = json.loads(typed_path.read_text())
        checked([driver, typed_path, obj], env=ENV)
        for mutation in ('missing-tuple', 'duplicate-tuple', 'foreign-child', 'nonnull', 'wrong-writer'):
            data = copy.deepcopy(typed)
            function = next(f for f in data['functions'] if f['name'] == 'TreeNode.check/0')
            op = next(b['terminator']['instruction']['operation'] for b in function['blocks']
                      if b['terminator']['instruction'].get('operation', {}).get('kind') == 'object_field_borrow')
            layout = next(row for row in data['layouts'] if row['object_id'] == op['object_id'])
            if mutation == 'missing-tuple':
                layout['owned_object_residences'] = [row for row in layout['owned_object_residences']
                                                     if row['field_ordinal'] != op['field_ordinal']]
            elif mutation == 'duplicate-tuple':
                layout['owned_object_residences'].insert(1, copy.deepcopy(layout['owned_object_residences'][0]))
            elif mutation == 'foreign-child':
                op['expected_object_id'] = max(row['object_id'] for row in data['layouts']) + 1
            elif mutation == 'nonnull':
                op['nullable'] = False
            else:
                writer = next(f for f in data['functions'] if f['name'] == 'TreeNode.birth/3')
                store = next(b['terminator']['instruction']['operation'] for b in writer['blocks']
                             if b['terminator']['instruction'].get('operation', {}).get('kind') == 'object_field_set'
                             and b['terminator']['instruction']['operation']['field_ordinal'] == op['field_ordinal'])
                store['field_ordinal'] = 1
            reject(data, ('published-lifecycle-physical-parser', 'published-lifecycle-v4/unsupported-cohort'))

    for null_kind in ('initialization', 'default'):
        null_path = DIRECTORY / f'hako-issued-null-field-{null_kind}.json'
        if null_path.exists():
            original = json.loads(null_path.read_text())
            checked([driver, null_path, obj], env=ENV)
            wraps = ('fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
                     'object.checked_field_set', 'object.checked_residence_set',
                     'object.home_release_plain_i64', 'object.reclaim_unpublished')
            checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                     *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                     '-lpthread', '-ldl', '-lm', '-o', exe])
            normal = run([exe], env=ENV)
            assert normal.returncode == 5 and 'COUNTS 1 4 2 0 0 1\n' in normal.stdout, normal
            # Null is installed first; a Fresh child is then installed before the
            # final scalar write faults. Only the live child releases, once.
            for at, counts in (('1', '1 1 0 1 1 1'), ('4', '1 4 1 1 1 1')):
                fault = run([exe], env=dict(ENV, V4_PROBE_FAULT_AT=at))
                assert fault.returncode == 70 and f'COUNTS {counts}\n' in fault.stdout, fault
            for mutation in ('false-null', 'wrong-child', 'duplicate-install', 'null-release', 'nonnull'):
                data = copy.deepcopy(original)
                birth = next(f for f in data['functions'] if f['name'] == 'Parent.birth/0')
                null = next(r['instruction'] for b in birth['blocks'] for r in b['instructions']
                            if r['instruction']['op'] == 'const_null')
                stores = [b['terminator']['instruction']['operation'] for b in birth['blocks']
                          if b['terminator']['instruction'].get('operation', {}).get('kind') == 'object_field_set']
                first = next(op for op in stores if op['value'] == null['dst'])
                if mutation == 'false-null':
                    null.update(op='const_i64', value=0)
                elif mutation == 'wrong-child':
                    first['child_object_id'] = max(row['object_id'] for row in data['layouts']) + 1
                elif mutation == 'duplicate-install':
                    second = next(op for op in stores if op is not first)
                    second['field_ordinal'] = first['field_ordinal']
                elif mutation == 'null-release':
                    release = next(b['terminator']['instruction']['operation'] for b in birth['blocks']
                                   if b['terminator']['instruction'].get('operation', {}).get('kind') == 'object_field_release')
                    release['field_ordinal'] = first['field_ordinal']
                else:
                    check = next(f for f in data['functions'] if f['name'] == 'Parent.check/0')
                    borrow = next(b['terminator']['instruction']['operation'] for b in check['blocks']
                                  if b['terminator']['instruction'].get('operation', {}).get('kind') == 'object_field_borrow')
                    borrow['nullable'] = False
                reject(data, ('published-lifecycle-physical-parser', 'published-lifecycle-v4/unsupported-cohort'))
            print('Null/Fresh initializer: normal and Fault cleanup; forged Null/tuple/reinstall reject')


    recursive_path = DIRECTORY / 'hako-issued-typed-recursive-field-borrow.json'
    if recursive_path.exists():
        recursive = json.loads(recursive_path.read_text())
        checked([driver, recursive_path, obj], env=ENV)
        for mutation in ('guard-edge', 'asymmetric-birth', 'recursive-write'):
            data = copy.deepcopy(recursive)
            function = next(f for f in data['functions'] if f['name'] == 'TreeNode.itemCheck/0')
            if mutation == 'guard-edge':
                compare = next(row['instruction'] for b in function['blocks'] for row in b['instructions']
                               if row['instruction']['op'] == 'borrowed_null_compare')
                compare['predicate'] = 'ne'
            elif mutation == 'asymmetric-birth':
                caller = data['functions'][0]
                null = next(row['instruction']['dst'] for b in caller['blocks'] for row in b['instructions']
                            if row['instruction']['op'] == 'const_null')
                call = next(b['terminator']['instruction']['operation']['call'] for b in caller['blocks']
                            if b['terminator']['instruction'].get('operation', {}).get('kind') == 'birth_call'
                            and b['terminator']['instruction']['operation']['call']['args'][0]['kind'] == 3)
                call['args'][1] = {'kind': 0, 'value': null}
            else:
                block = next(b for b in function['blocks'] if
                    b['terminator']['instruction'].get('operation', {}).get('kind') == 'object_field_borrow'
                    and not b['terminator']['instruction']['operation']['nullable'])
                scalar = next(row['instruction'] for row in block['instructions']
                              if row['instruction']['op'] == 'object_field_get')
                block['instructions'].append({'index': len(block['instructions']), 'instruction': {
                    'op': 'field_set', 'object_id': scalar['object_id'], 'field_ordinal': scalar['field_ordinal'],
                    'base': scalar['base'], 'value': scalar['dst'], 'site': 777778,
                    'exact_numeric_runtime_check': {'kind': 'none'}}})
                block['terminator']['index'] = len(block['instructions'])
            reject(data, 'published-lifecycle-v4/unsupported-cohort')
        print('typed recursive guard, paired Birth and strict nonmutation refusals')

    def parent(data):
        return next(f for f in data['functions'] if f['name'] == 'Parent.first/1')

    def receiver_read(function):
        return next(row['instruction'] for block in function['blocks']
                    for row in block['instructions']
                    if row['instruction']['op'] == 'object_field_get')

    # Physical alias witnesses are independent of unchanged source acceptance.
    for kind in ('copy', 'phi'):
        data = copy.deepcopy(base)
        function = parent(data)
        block = next(b for b in function['blocks'] if
                     any(row['instruction']['op'] == 'object_field_get' for row in b['instructions']))
        read = receiver_read(function)
        alias = max(row['instruction'].get('dst', 0) for b in function['blocks']
                    for row in b['instructions']) + 1
        position = next(i for i, row in enumerate(block['instructions']) if row['instruction'] is read)
        instruction = {'op': 'copy', 'dst': alias, 'src': function['receiver']}
        if kind == 'phi':
            # Keep the frame at entry, before a distinct Phi landing.
            assert block['id'] == function['entry']
            frame = block['instructions'].pop(0)
            assert frame['instruction']['op'] == 'fault_frame_enter'
            entry = max(b['id'] for b in function['blocks']) + 1
            function['blocks'].append({'id': entry, 'instructions': [frame],
                'terminator': {'index': 1, 'instruction': {'op': 'jump', 'target': block['id'], 'args': None}},
                'edges': [{'target': block['id'], 'args': None}]})
            function['entry'] = entry
            instruction = {'op': 'phi', 'dst': alias,
                           'inputs': [{'block': entry, 'value': function['receiver']}]}
            position = 0
        block['instructions'].insert(position, {'instruction': instruction})
        read['base'] = alias
        for i, row in enumerate(block['instructions']): row['index'] = i
        block['terminator']['index'] = len(block['instructions'])
        path = work / (kind + '-root.json')
        path.write_text(json.dumps(data))
        checked([driver, path, obj], env=ENV)
        print('exact borrowed receiver', kind, 'root admitted (wire witness)')
        if kind == 'phi':
            for malformed in ('missing', 'duplicate', 'foreign', 'cycle'):
                bad = copy.deepcopy(data)
                phi = next(r['instruction'] for b in parent(bad)['blocks']
                           for r in b['instructions'] if r['instruction']['op'] == 'phi')
                if malformed == 'missing': phi['inputs'] = []
                elif malformed == 'duplicate': phi['inputs'] *= 2
                elif malformed == 'foreign': phi['inputs'][0]['value'] = 999999
                else: phi['inputs'][0]['value'] = alias
                reject(bad, 'published-lifecycle-physical-parser/')

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
        # One layout mutation must reject before artifacts. It can invalidate
        # the original Birth/Call representation before indexed provider flow;
        # checker ordering is not the layout-correspondence safety contract.
        reason = 'unsupported-cohort' if malformed == 'foreign-child' else 'receiver-object-mismatch'
        reject(data, ('published-lifecycle-physical-parser/function-body',
                      'published-lifecycle-v4/' + reason))

    # Exact tag-3 provided loans can now BorrowForCall (source acceptance is
    # binary-trees). That does not grant nullable or scalar argument spellings.
    # Corroborate the unused formal's class so foreign class is not the cause.
    for actual in ('nullable_typed_object', 'i64'):
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
             'object.checked_field_set', 'object.checked_residence_set', 'object.home_release_plain_i64',
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
