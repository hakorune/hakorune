#!/usr/bin/env python3
"""Execute unchanged JSON emitted by the original borrowed-source Rust tests.

Source execution positives pass issued input directly to the existing C driver
and real kernel. Explicitly labeled physical-input probes separately check
backend admission; their graph mutations are not Rust source acceptance.
"""
import copy
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[3]
TESTS = ROOT / 'lang/c-abi/tests'
ARCHIVE = Path(os.environ.get('HAKO_TEST_LIFECYCLE_ARCHIVE',
    ROOT / 'target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a'))
ENV = dict(os.environ, NYASH_NYRT_SILENT_RESULT='1', HAKO_NYRT_PLUGIN_HOST='off')
ENV.pop('V4_PROBE_FAULT_AT', None)
ENV['V4_PROBE_MODE'] = 'normal'
DIRECTORY = Path(sys.argv[1])


def checked(argv, **kwargs):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True, **kwargs)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


with tempfile.TemporaryDirectory(prefix='hako original borrowed source ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'source.o', 'source')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.checked_residence_set', 'object.home_release_plain_i64', 'object.reclaim_unpublished']
    profiles = [(shape, domain) for domain in ('zero', 'negative', 'true', 'false', 'object', 'scalar', 'bool-scalar')
                for shape in ('local', 'discard', 'return', 'nested')]
    profiles.extend([('forwarded', None), ('entry-receiver', None)])
    if any(flag in sys.argv for flag in ('--unit-only', '--bool-discard-only', '--print-alias-only', '--argv-only', '--constructor-unit-only', '--birth-handle-only')):
        profiles = []
    for shape, domain in profiles:
        suffix = shape if domain is None else shape + '-' + domain
        path = DIRECTORY / ('hako-issued-borrowed-ingress-' + suffix + '.json')
        data = json.loads(path.read_text())
        birth = next(f for f in data['functions'] if f['name'] == 'Transport.birth/0')
        sites = [b['terminator']['instruction']['operation']['site'] for b in birth['blocks']
                 if b['terminator']['instruction'].get('operation', {}).get('kind') == 'field_set']
        assert len(sites) == 1, 'original callee Birth Fault boundary'
        checked([driver, path, obj], env=ENV)
        assert not list(work.glob('source.o.*')), 'temporary artifact leak'
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        # Each constructed Home stores once inside the callee's own Birth and
        # releases through home_release_plain_i64 (scratch included).
        homes = 3 if domain is None else 2
        stores = 3 if domain is None else 2
        normal = subprocess.run([str(exe)], env=ENV, capture_output=True, text=True)
        assert normal.returncode == 7, (suffix, normal)
        assert f'COUNTS 1 {stores} {homes} 0 0 1\n' in normal.stdout, (suffix, normal)
        assert 'FAULT ' not in normal.stdout, normal
        # Inject only runtime failure at the original callee Birth store:
        # scratch is unpublished so it unwinds via reclaim_unpublished, while
        # every completed Home takes home_release. Forwarded source constructs
        # an additional Home before reaching that store.
        fault_at = 3 if domain is None else 2
        fault = subprocess.run([str(exe)], env=dict(ENV, V4_PROBE_FAULT_AT=str(fault_at)),
                               capture_output=True, text=True)
        assert fault.returncode == 70, (suffix, fault)
        assert f'COUNTS 1 {fault_at} {homes - 1} 1 1 1\n' in fault.stdout, (suffix, fault)
        assert f'FAULT 101 {sites[0]} 0 0 HOME {homes - 1} RECLAIM 1\n' in fault.stdout, \
            (suffix, fault)
        print(suffix, 'original source/ABI/OBJ/EXE Normal and callee Fault cleanup once')
    if profiles:
        print('30 original-source inputs execute unchanged; no carrier/CFG repair')

# Unit calls share the original source, receiver, frame and cleanup path, with
# no result lease/out-slot. Both bare Return and the real implicit const_unit
# are produced by the existing Rust discard test, never repaired here.
if any(flag in sys.argv for flag in ('--unit', '--unit-only', '--constructor-unit-only')):
    with tempfile.TemporaryDirectory(prefix='hako original unit source ') as directory:
        work = Path(directory)
        driver, obj, exe = [work / name for name in ('driver', 'unit.o', 'unit')]
        checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
                 '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
        unit_wraps = wraps + ['object.checked_new', 'string.checked_print']
        for case in ([6] if '--constructor-unit-only' in sys.argv else range(7)):
            path = DIRECTORY / ('hako-issued-constructor-unit.json' if case == 6 else
                                f'hako-issued-borrowed-unit-{case}.json')
            started = time.monotonic()
            checked([driver, path, obj], env=ENV)
            compile_seconds = time.monotonic() - started
            if case == 2:
                original = json.loads(path.read_text())
                def unit_function(data):
                    return next(f for f in data['functions'] if f['role'] == 'ordinary_unit')
                def unit_call(data):
                    return next(b for f in data['functions'] for b in f['blocks']
                                if b['terminator']['instruction'].get('operation', {}).get('result') == 'unit')
                for mutation in ('role', 'result', 'value-return', 'projection'):
                    data = copy.deepcopy(original)
                    if mutation == 'role':
                        unit_function(data)['role'] = 'ordinary_i64'
                    elif mutation == 'result':
                        unit_call(data)['terminator']['instruction']['operation']['result'] = 'i64'
                    elif mutation == 'value-return':
                        function = unit_function(data)
                        integer = next(r['instruction']['dst'] for b in function['blocks']
                                       for r in b['instructions'] if r['instruction']['op'] == 'const_i64')
                        block = next(b for b in function['blocks'] if b['terminator']['instruction']['op'] == 'return')
                        block['terminator']['instruction']['value'] = integer
                    else:
                        call = unit_call(data)
                        block = next(b for b in data['functions'][0]['blocks']
                                     if b['id'] == call['terminator']['instruction']['normal'])
                        block['instructions'].insert(0, {'index': 0, 'instruction': {
                            'op': 'invoke_normal_result', 'invoke_block': call['id'], 'dst': 999}})
                        for index, row in enumerate(block['instructions']):
                            row['index'] = index
                        block['terminator']['index'] = len(block['instructions'])
                    changed = work / (mutation + '.json')
                    changed.write_text(json.dumps(data))
                    bad_obj = work / (mutation + '.o')
                    rejected = subprocess.run([str(driver), str(changed), str(bad_obj)],
                                              env=ENV, capture_output=True, text=True)
                    assert rejected.returncode != 0 and not bad_obj.exists(), (mutation, rejected)
                    assert '[freeze:contract]' in rejected.stderr, (mutation, rejected)
            if case == 6:
                original = json.loads(path.read_text())
                for mutation in ('duplicate-commit', 'read-before-commit', 'unborn-child',
                                 'foreign-base', 'result', 'missing-fault-discharge'):
                    data = copy.deepcopy(original)
                    fn = next(f for f in data['functions'] if f['name'] == 'Pair.birth/0')
                    stores = [b for b in fn['blocks'] if b['terminator']['instruction']
                              .get('operation', {}).get('kind') == 'object_field_set']
                    calls = [b for b in fn['blocks'] if b['terminator']['instruction']
                             .get('operation', {}).get('kind') == 'ordinary_call']
                    read_block = next(b for b in fn['blocks'] if any(
                        r['instruction']['op'] == 'object_field_get' for r in b['instructions']))
                    read = next(r for r in read_block['instructions']
                                if r['instruction']['op'] == 'object_field_get')
                    bypass = None
                    if mutation == 'duplicate-commit':
                        stores[1]['terminator']['instruction']['operation']['field_ordinal'] = 0
                    elif mutation == 'read-before-commit':
                        read_block['instructions'].remove(read)
                        stores[0]['instructions'].append(read)
                        for block in (read_block, stores[0]):
                            for i, row in enumerate(block['instructions']): row['index'] = i
                            block['terminator']['index'] = len(block['instructions'])
                    elif mutation == 'unborn-child':
                        bypass = next(b for b in fn['blocks'] if b['terminator']['instruction']
                                      .get('operation', {}).get('kind') == 'birth_call')
                    elif mutation == 'foreign-base':
                        read['instruction']['base'] = stores[0]['terminator']['instruction']['operation']['value']
                    elif mutation == 'result':
                        calls[0]['terminator']['instruction']['operation']['result'] = 'i64'
                    else:
                        failed = calls[0]['terminator']['instruction']['fault']
                        bypass = next(b for b in fn['blocks'] if b['id'] == failed)
                        assert bypass['terminator']['instruction']['operation']['kind'] == 'object_field_release'
                    if bypass:
                        normal = bypass['terminator']['instruction']['normal']
                        bypass['terminator']['instruction'] = {'op': 'jump', 'target': normal, 'args': None}
                        bypass['edges'] = [{'target': normal, 'args': None}]
                    changed, bad_obj = work / (mutation + '.json'), work / (mutation + '.o')
                    changed.write_text(json.dumps(data))
                    rejected = subprocess.run([str(driver), str(changed), str(bad_obj)],
                                              env=ENV, capture_output=True, text=True)
                    assert rejected.returncode != 0 and not bad_obj.exists(), (mutation, rejected)
                    assert '[freeze:contract]' in rejected.stderr, (mutation, rejected)
                    print('constructor-unit', mutation, 'rejected', flush=True)
            started = time.monotonic()
            checked(['cc', '-DHAKO_TEST_OWNED_CALL_RESULT', '-DHAKO_TEST_UNIT_CALL', obj,
                     TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                     *['-Wl,--wrap=nyash.' + name + '_v1' for name in unit_wraps],
                     '-lpthread', '-ldl', '-lm', '-o', exe])
            link_seconds = time.monotonic() - started
            execution_seconds = 0.0
            homes = 3 if case in (4, 6) else 1
            modes = [('normal', {}, homes, homes, homes, 0)]
            if case >= 2:
                modes.append(('print-fault', {'V4_PROBE_MODE': 'unit-print-fault'}, homes, homes, homes, 1))
            if case == 4:
                modes.extend((f'new-fault-{n}', {'V4_PROBE_NEW_FAULT_AT': str(n)}, n, n-1, n-1, 1)
                             for n in (2, 3))
                modes.extend((f'home-fault-{n}', {'V4_PROBE_HOME_FAULT_AT': str(n)}, homes, homes, homes, 1)
                             for n in (1, 2, 3))
            for mode_name, extra, attempts, acquired, released, reports in modes:
                started = time.monotonic()
                result = subprocess.run([str(exe)], env=dict(ENV, **extra), capture_output=True, text=True)
                execution_seconds += time.monotonic() - started
                assert result.returncode == (70 if reports else 0), (case, mode_name, result)
                reclaims = int(case == 6 and reports)
                stores = 4 if case == 6 else 0
                assert f'COUNTS 1 {stores} {released-reclaims} {reclaims} {reports} 1\n' in result.stdout, (case, mode_name, result)
                assert f'OWNED {attempts} {acquired} {released}\n' in result.stdout, (case, mode_name, result)
                prints = (2 if case == 6 and not reports else
                          int(not mode_name.startswith('new-fault')))
                assert sum(line.startswith('UNIT_PRINT ') for line in result.stdout.splitlines()) == prints, (case, mode_name, result)
                assert result.stdout.splitlines().count('7') == (prints if mode_name != 'print-fault' else 0), \
                    (case, mode_name, result)
            print('unit', case, 'original source/ABI/OBJ/EXE and selected Fault cleanup once',
                  f'compile={compile_seconds:.6f}s link={link_seconds:.6f}s run={execution_seconds:.6f}s modes={len(modes)}', flush=True)

# The source-issued Bool discard cohort also exercises a nullable owned result
# committed into an Array slot before its original Get becomes a borrowed actual.
if '--bool-discard' in sys.argv or '--bool-discard-only' in sys.argv:
    with tempfile.TemporaryDirectory(prefix='hako original bool discard ') as directory:
        work = Path(directory)
        driver, obj, exe = [work / name for name in ('driver', 'bool.o', 'bool')]
        checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
                 '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
        for returned in ('true', 'false'):
            path = DIRECTORY / f'hako-issued-borrowed-bool-discard-{returned}.json'
            original = json.loads(path.read_text())
            birth = next(f for f in original['functions'] if f['name'] == 'Token.birth/1')
            stores = [b['terminator']['instruction']['operation']['site'] for b in birth['blocks']
                      if b['terminator']['instruction'].get('operation', {}).get('kind') == 'field_set']
            assert len(stores) == 1
            checked([driver, path, obj], env=ENV)
            checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                     *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                     '-lpthread', '-ldl', '-lm', '-o', exe])
            for faulted in (False, True):
                result = subprocess.run([str(exe)], env=dict(ENV, **({'V4_PROBE_FAULT_AT': '1'} if faulted else {})),
                                        capture_output=True, text=True)
                assert result.returncode == (70 if faulted else 0), (returned, faulted, result)
                expected = 'COUNTS 1 1 1 1 1 1' if faulted else 'COUNTS 1 1 2 0 0 1'
                assert result.stdout.splitlines().count(expected) == 1, (returned, faulted, result)
                if faulted:
                    assert f'FAULT 101 {stores[0]} 1 0 HOME 1 RECLAIM 1\n' in result.stdout, result
                else:
                    assert 'FAULT ' not in result.stdout, result
            for mutation in ('nonnullable', 'wrong-object', 'nonnull-return'):
                data = copy.deepcopy(original)
                if mutation == 'nonnull-return':
                    next(f for f in data['functions'] if f['name'] == 'Transport.make/1')['role'] = 'ordinary_handle'
                for function in data['functions']:
                    for block in function['blocks']:
                        op = block['terminator']['instruction'].get('operation', {})
                        if op.get('kind', '').startswith('array_owned_'):
                            if mutation == 'nonnullable':
                                op['nullable'] = False
                            elif mutation == 'wrong-object':
                                op['object_id'] = 1
                        if mutation == 'nonnull-return' and op.get('result') == 'nullable_handle':
                            op['result'] = 'handle'
                changed, rejected_obj = work / (mutation + '.json'), work / (mutation + '.o')
                changed.write_text(json.dumps(data))
                rejected = subprocess.run([str(driver), str(changed), str(rejected_obj)],
                                          env=ENV, capture_output=True, text=True)
                assert rejected.returncode != 0 and not rejected_obj.exists(), (returned, mutation, rejected)
                assert '[freeze:contract]' in rejected.stderr, (returned, mutation, rejected)
            print('bool-discard', returned, 'original source/ABI/OBJ/EXE Normal and callee Fault; nullable/object/return drift rejected', flush=True)

# The immutable source-issued alias input reaches native execution without a
# source/graph repair. Mutations below are rejection tests, never acceptance.
if '--print-alias' in sys.argv or '--print-alias-only' in sys.argv:
    with tempfile.TemporaryDirectory(prefix='hako original print alias ') as directory:
        work = Path(directory)
        driver, obj, exe = [work / name for name in ('driver', 'alias.o', 'alias')]
        checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
                 '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
        path = DIRECTORY / 'hako-issued-print-alias.json'
        original = json.loads(path.read_text())
        checked([driver, path, obj], env=ENV)
        checked(['cc', obj, ARCHIVE, '-lpthread', '-ldl', '-lm', '-o', exe])
        run = subprocess.run([str(exe)], env=ENV, capture_output=True, text=True)
        assert (run.returncode, run.stdout, run.stderr) == (0, 'n=7\n', ''), run
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        birth = next(f for f in original['functions'] if f['name'] == 'Leaf.birth/0')
        store = next(b['terminator']['instruction']['operation']['site'] for b in birth['blocks']
                     if b['terminator']['instruction'].get('operation', {}).get('kind') == 'field_set')
        for faulted in (False, True):
            run = subprocess.run([str(exe)], env=dict(ENV, **({'V4_PROBE_FAULT_AT': '1'} if faulted else {})),
                                 capture_output=True, text=True)
            assert run.returncode == (70 if faulted else 0), run
            assert ('COUNTS 1 1 0 2 1 1' if faulted else 'COUNTS 1 2 2 0 0 1') in run.stdout.splitlines(), run
            assert run.stdout.splitlines().count('n=7') == int(not faulted), run
            if faulted:
                assert f'FAULT 101 {store} 7 0 HOME 0 RECLAIM 2\n' in run.stdout, run

        def reindex(block):
            for index, row in enumerate(block['instructions']):
                row['index'] = index
            block['terminator']['index'] = len(block['instructions'])

        for mutation in ('missing-residence', 'wrong-child', 'before-birth-read', 'released-before-call',
                         'alias-home-end', 'alias-nullable-return', 'nonbirth-writer', 'shared-birth-normal'):
            data = copy.deepcopy(original)
            main = data['functions'][0]
            call = next(b for b in main['blocks']
                        if b['terminator']['instruction'].get('operation', {}).get('kind') == 'ordinary_call')
            receiver = call['terminator']['instruction']['operation']['call']['receiver']
            alias_copy = next(r['instruction'] for b in main['blocks'] for r in b['instructions']
                              if r['instruction']['op'] == 'copy' and r['instruction']['dst'] == receiver)
            read = next(r['instruction'] for b in main['blocks'] for r in b['instructions']
                        if r['instruction'].get('dst') == alias_copy['src'])
            parent, ordinal = read['object_id'], read['field_ordinal']
            layout = next(row for row in data['layouts'] if row['object_id'] == parent)
            residence = next(row for row in layout['owned_object_residences'] if row['field_ordinal'] == ordinal)
            if mutation == 'missing-residence':
                layout['owned_object_residences'].remove(residence)
            elif mutation == 'wrong-child':
                residence['child_object_id'] = parent
            elif mutation == 'before-birth-read':
                born = next(b for b in main['blocks']
                            if b['terminator']['instruction'].get('operation', {}).get('kind') == 'birth_call')
                moved = []
                for block in main['blocks']:
                    keep = []
                    for row in block['instructions']:
                        if row['instruction'].get('dst') in (read['base'], read['dst'], receiver):
                            moved.append(row)
                        else:
                            keep.append(row)
                    block['instructions'] = keep
                    reindex(block)
                assert len(moved) == 3
                born['instructions'].extend(moved)
                reindex(born)
            elif mutation == 'released-before-call':
                release = copy.deepcopy(next(b['terminator']['instruction'] for b in main['blocks']
                    if b['terminator']['instruction'].get('operation', {}).get('kind') == 'object_field_release'))
                fresh = max(b['id'] for b in main['blocks']) + 1
                for block in main['blocks']:
                    term = block['terminator']['instruction']
                    for key in ('target', 'normal', 'fault', 'then', 'else'):
                        if term.get(key) == call['id']:
                            term[key] = fresh
                    for edge in block['edges']:
                        if edge['target'] == call['id']:
                            edge['target'] = fresh
                release['normal'] = call['id']
                release['fault'] = call['terminator']['instruction']['fault']
                main['blocks'].append({'id': fresh, 'instructions': [],
                    'terminator': {'index': 0, 'instruction': release},
                    'edges': [{'target': release['normal'], 'args': None}, {'target': release['fault'], 'args': None}]})
            elif mutation == 'shared-birth-normal':
                born = next(b['terminator']['instruction']['normal'] for b in main['blocks']
                            if b['terminator']['instruction'].get('operation', {}).get('kind') == 'birth_call')
                release = max((b for b in main['blocks']
                               if b['terminator']['instruction'].get('operation', {}).get('kind') == 'object_field_release'),
                              key=lambda b: b['id'])
                normal = next(b for b in main['blocks'] if b['id'] == release['terminator']['instruction']['normal'])
                assert normal['terminator']['instruction']['op'] == 'jump'
                normal['terminator']['instruction']['target'] = born
                normal['edges'] = [{'target': born, 'args': None}]
            elif mutation == 'alias-home-end':
                release = next(b['terminator']['instruction']['operation'] for b in main['blocks']
                               if b['terminator']['instruction'].get('operation', {}).get('kind') == 'home_release')
                release['value'], release['object_id'] = receiver, residence['child_object_id']
            elif mutation == 'alias-nullable-return':
                returned = copy.deepcopy(main)
                returned['name'], returned['role'] = 'alias_returner', 'ordinary_nullable_handle'
                for block in returned['blocks']:
                    for row in block['instructions']:
                        if row['instruction']['op'] == 'fault_frame_enter':
                            row['instruction']['mode'] = 'borrowed'
                    if block['terminator']['instruction']['op'] == 'return':
                        block['terminator']['instruction']['value'] = receiver
                data['functions'].append(returned)
            else:
                writer = copy.deepcopy(next(f for f in data['functions'] if f['name'] == 'Parent.birth/0'))
                writer['name'], writer['role'], writer['receiver_object'] = 'replace_child', 'ordinary_unit', parent
                data['functions'].append(writer)
            changed, bad_obj = work / (mutation + '.json'), work / (mutation + '.o')
            changed.write_text(json.dumps(data))
            rejected = subprocess.run([str(driver), str(changed), str(bad_obj)], env=ENV, capture_output=True, text=True)
            assert rejected.returncode != 0 and not bad_obj.exists(), (mutation, rejected)
            assert '[freeze:contract]' in rejected.stderr, (mutation, rejected)
            print('print-alias', mutation, 'rejected', flush=True)
        print('print-alias original source/ABI/OBJ/EXE n=7 and Birth Fault cleanup once PASS', flush=True)

# Same Rust-issued Main entry snapshot, without editing the issued graph.
if '--argv-only' in sys.argv:
    with tempfile.TemporaryDirectory(prefix='hako original argv source ') as directory:
        work = Path(directory)
        driver, obj, exe = [work / name for name in ('driver', 'argv.o', 'argv')]
        checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
                 '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
        for shape, count in [('empty', 0), ('text', 4), ('body-empty', 0), ('body-text', 4)]:
            path = DIRECTORY / f'hako-issued-main-arguments-{shape}.json'
            checked([driver, path, obj], env=ENV)
            checked(['cc', '-DHAKO_TEST_SCRIPT_ARGUMENTS', '-DHAKO_TEST_UNIT_CALL', obj,
                     TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                     *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                     '-Wl,--wrap=nyash.array.checked_script_arguments_new_v1',
                     '-Wl,--wrap=nyash.string.checked_print_v1',
                     '-Wl,--wrap=nyrt_handle_release_h', '-lpthread', '-ldl', '-lm', '-o', exe])
            for mode in (['normal', 'argv-fault', 'unit-print-fault'] if shape.startswith('body-') else ['normal', 'argv-fault']):
                result = subprocess.run([str(exe)], capture_output=True, text=True,
                    env=dict(ENV, V4_PROBE_MODE=mode, V4_PROBE_ARGV_COUNT=str(count),
                             **({'V4_PROBE_ARGV_BODY': '1'} if shape.startswith('body-') else {})))
                assert result.returncode == (0 if mode == 'normal' else 70), (shape, mode, result)
                expected = 'ARGV 1 0 0' if mode == 'argv-fault' else 'ARGV 1 1 1'
                assert expected in result.stdout, (shape, mode, result)
                assert ('FAULT 102 ' in result.stdout) == (mode == 'argv-fault'), result
                homes = 1 if shape.startswith('body-') and mode != 'argv-fault' else 0
                assert f'DISPOSE HOME {homes} RECLAIM 0 REPORT ' in result.stdout, result
                assert ('FAULT 103 ' in result.stdout) == (mode == 'unit-print-fault'), result
                print('argv', shape, mode, expected, 'PASS', flush=True)

        # Backend-only peer regression: the existing shared nullable release also
        # accepts intrinsic Array leases. No object outcome is manufactured for them.
        original = json.loads((DIRECTORY / 'hako-issued-main-arguments-empty.json').read_text())
        original['layouts'] = json.loads((DIRECTORY / 'hako-issued-main-arguments-body-text.json').read_text())['layouts']
        for kind in ('script_arguments_new', 'array_new'):
            data = copy.deepcopy(original)
            fn = data['functions'][0]
            entry = next(block for block in fn['blocks'] if block['id'] == fn['entry'])
            if kind == 'array_new': entry['terminator']['instruction']['operation'] = {'kind': kind, 'site': 0}
            block = next(block for block in fn['blocks'] if any(
                row['instruction']['op'] == 'array_residence_release' for row in block['instructions']))
            index = next(i for i, row in enumerate(block['instructions']) if row['instruction']['op'] == 'array_residence_release')
            value = block['instructions'][index]['instruction']['value']
            normal = copy.deepcopy(block)
            normal['id'] = max(row['id'] for row in fn['blocks']) + 1
            normal['instructions'] = normal['instructions'][index + 1:]
            for i, row in enumerate(normal['instructions']): row['index'] = i
            normal['terminator']['index'] = len(normal['instructions'])
            fn['blocks'].append(normal)
            first = entry['terminator']['instruction']
            block['instructions'] = block['instructions'][:index]
            block['terminator'] = {'index': index, 'instruction': {
                'op': 'invoke', 'fault_frame': first['fault_frame'], 'normal': normal['id'], 'fault': first['fault'],
                'operation': {'kind': 'home_release_if_live', 'object_id': data['layouts'][0]['object_id'], 'value': value, 'site': 999991}}}
            block['edges'] = [{'target': normal['id'], 'args': None}, {'target': first['fault'], 'args': None}]
            path = work / (kind + '-shared-release.json')
            path.write_text(json.dumps(data))
            checked([driver, path, obj], env=ENV)
            print(kind, 'shared nullable release physical admission PASS', flush=True)

# Provided arguments move only on Birth Normal. The exact source snapshots
# cover a Fault before, at and after the staged field store.
if '--birth-handle-only' in sys.argv:
    with tempfile.TemporaryDirectory(prefix='hako provided birth source ') as directory:
        work = Path(directory)
        driver, obj, exe = [work / name for name in ('driver', 'provided.o', 'provided')]
        checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
                 '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
        for shape in ('only', 'before', 'after', 'nullable-wire'):
            path = DIRECTORY / f'hako-issued-birth-handle-actual-{shape}.json'
            if shape == 'nullable-wire':
                data = json.loads((DIRECTORY / 'hako-issued-birth-handle-actual-only.json').read_text())
                for fn in data['functions']:
                    for block in fn['blocks']:
                        op = block['terminator']['instruction'].get('operation', {})
                        if op.get('kind') == 'birth_call':
                            for arg in op['call']['args']:
                                if arg['kind'] == 3: arg['kind'] = 'nullable_typed_object'
                path = work / 'nullable-wire.json'
                path.write_text(json.dumps(data))
            checked([driver, path, obj], env=ENV)
            checked(['cc', '-DHAKO_TEST_OWNED_CALL_RESULT', obj,
                     TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                     *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps + ['object.checked_new']],
                     '-lpthread', '-ldl', '-lm', '-o', exe])
            total = 2 if shape in ('only', 'nullable-wire') else 3
            modes = [('normal', {}, total, 2, 2, 2, 0, 0),
                     ('allocation', {'V4_PROBE_NEW_FAULT_AT': '2'}, 1, 2, 1, 1, 0, 1),
                     ('inner-birth', {'V4_PROBE_FAULT_AT': '1'}, 1, 1, 1, 0, 1, 1)]
            modes.extend((f'store-{n}', {'V4_PROBE_FAULT_AT': str(n)}, n, 2, 2, 1, 1, 1)
                         for n in range(2, total + 1))
            for name, extra, stores, attempts, acquired, homes, reclaims, reports in modes:
                result = subprocess.run([str(exe)], env=dict(ENV, **extra), capture_output=True, text=True)
                assert result.returncode == (70 if reports else 0), (shape, name, result)
                assert f'COUNTS 1 {stores} {homes} {reclaims} {reports} 1\n' in result.stdout, (shape, name, result)
                assert f'OWNED {attempts} {acquired} {acquired}\n' in result.stdout, (shape, name, result)
                assert ('FAULT 101 ' in result.stdout) == bool(reports), (shape, name, result)
            print('birth-handle', shape, 'Normal and every selected allocation/store Fault PASS', flush=True)
            if shape == 'nullable-wire':
                original = json.loads(path.read_text())
                for mutation in ('i64', 'foreign-class', 'scalar-formal', 'duplicate-slot-demand', 'duplicate-lease-demand'):
                    data = copy.deepcopy(original)
                    root = data['functions'][0]
                    call_block = next(b for b in root['blocks'] if b['terminator']['instruction']
                        .get('operation', {}).get('call', {}).get('args', []))
                    call = call_block['terminator']['instruction']['operation']['call']
                    callee = data['functions'][call['target']]
                    if mutation == 'i64':
                        dst = 1 + max(row['instruction'].get('dst', 0) for b in root['blocks'] for row in b['instructions'])
                        call_block['instructions'].append({'index': len(call_block['instructions']),
                            'instruction': {'op': 'const_i64', 'dst': dst, 'value': 7}})
                        call_block['terminator']['index'] = len(call_block['instructions'])
                        call['args'][0]['value'] = dst
                    elif mutation == 'foreign-class': call['args'][0]['value'] = call['receiver']
                    elif mutation == 'scalar-formal': callee['params'][0]['representation'] = 'kind_payload_v1'
                    else:
                        store = next(b for b in callee['blocks'] if b['terminator']['instruction']
                            .get('operation', {}).get('kind') == 'object_field_set')
                        layout = next(row for row in data['layouts'] if row['object_id'] ==
                            store['terminator']['instruction']['operation']['object_id'])
                        layout['fields'].append({'declaration_ordinal': 1, 'runtime_slot': 1, 'storage_kind': 1})
                        layout['field_count'] = 2
                        layout['owned_object_residences'].append(dict(layout['owned_object_residences'][0], field_ordinal=1))
                        next_store = copy.deepcopy(store)
                        next_store['id'] = 1 + max(b['id'] for b in callee['blocks'])
                        next_store['instructions'] = []
                        next_store['terminator']['index'] = 0
                        next_store['terminator']['instruction']['operation']['field_ordinal'] = 1
                        if mutation == 'duplicate-lease-demand':
                            dst = 1 + max([callee['receiver']] + [p['value'] for p in callee['params']] +
                                [r['instruction'].get('dst', 0) for b in callee['blocks'] for r in b['instructions']])
                            callee['params'].append({'value': dst, 'representation': 'nullable_kind_payload_v1'})
                            call['args'].append(dict(call['args'][0]))
                            next_store['terminator']['instruction']['operation']['value'] = dst
                        store['terminator']['instruction']['normal'] = next_store['id']
                        store['edges'][0]['target'] = next_store['id']
                        callee['blocks'].append(next_store)
                    changed, bad_obj = work / (mutation + '.json'), work / (mutation + '.o')
                    changed.write_text(json.dumps(data))
                    rejected = subprocess.run([str(driver), str(changed), str(bad_obj)], env=ENV,
                                              capture_output=True, text=True)
                    assert rejected.returncode != 0 and '[freeze:contract]' in rejected.stderr, (mutation, rejected)
                    assert not bad_obj.exists(), (mutation, rejected)
                    print('birth-handle nullable wire', mutation, 'rejected', flush=True)
            if shape != 'only': continue
            original = json.loads(path.read_text())
            for mutation in ('before', 'normal', 'fault'):
                data = copy.deepcopy(original)
                fn = next(f for f in data['functions'] if f['name'] == 'Holder.birth/1')
                store = next(b for b in fn['blocks'] if b['terminator']['instruction']
                             .get('operation', {}).get('kind') == 'object_field_set')
                term = store['terminator']['instruction']
                operation = term['operation']
                first = max(b['id'] for b in fn['blocks']) + 1
                frame, failed = term['fault_frame'], term['fault']
                if mutation == 'before':
                    target = first + 3
                    fn['blocks'].append({'id': target, 'instructions': [],
                        'terminator': {'index': 0, 'instruction': term}, 'edges': store['edges']})
                    store['terminator'] = {'index': len(store['instructions']), 'instruction':
                        {'op': 'jump', 'target': first, 'args': None}}
                    store['edges'] = [{'target': first, 'args': None}]
                else:
                    edge = mutation
                    target = term[edge]
                    term[edge] = first
                    store['edges'][0 if edge == 'normal' else 1]['target'] = first
                fn['blocks'].append({'id': first, 'instructions': [], 'terminator': {'index': 0,
                    'instruction': {'op': 'invoke', 'fault_frame': frame, 'normal': first+1, 'fault': first+2,
                        'operation': {'kind': 'home_release_if_live', 'value': operation['value'],
                            'object_id': operation['child_object_id'], 'site': 777777}}},
                    'edges': [{'target': first+1, 'args': None}, {'target': first+2, 'args': None}]})
                for id, target in ((first+1, target), (first+2, failed)):
                    fn['blocks'].append({'id': id, 'instructions': [], 'terminator': {'index': 0,
                        'instruction': {'op': 'jump', 'target': target, 'args': None}},
                        'edges': [{'target': target, 'args': None}]})
                changed, bad_obj = work / (mutation + '.json'), work / (mutation + '.o')
                changed.write_text(json.dumps(data))
                rejected = subprocess.run([str(driver), str(changed), str(bad_obj)], env=ENV,
                                          capture_output=True, text=True)
                assert rejected.returncode != 0 and '[freeze:contract]' in rejected.stderr, (mutation, rejected)
                assert not bad_obj.exists(), (mutation, rejected)
                print('birth-handle premature Provided release', mutation, 'rejected', flush=True)
