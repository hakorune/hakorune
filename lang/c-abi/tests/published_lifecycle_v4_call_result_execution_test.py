#!/usr/bin/env python3
"""Execute exact source-issued composed call results and exact cleanup in both finishing modes."""
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
STEM = sys.argv[2] if len(sys.argv) > 2 else 'hako-issued-borrowed-call-result'
OWNED = len(sys.argv) > 3 and sys.argv[3] == 'owned'
ARCHIVE = ROOT / 'target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a'
ENV = dict(os.environ, NYASH_NYRT_SILENT_RESULT='1', HAKO_NYRT_PLUGIN_HOST='off')
for key in ['V4_PROBE_FAULT_AT', 'V4_PROBE_NEW_FAULT_AT', 'V4_PROBE_HOME_FAULT_AT']:
    ENV.pop(key, None)
WRAPS = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
         'object.checked_field_set', 'object.home_release_plain_i64', 'object.reclaim_unpublished']


def checked(argv):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


def owned_results(driver, obj, exe, work):
    paths = sorted(INPUTS.glob(f'{STEM}-received*-nullable*-opt*.json'))
    assert len(paths) == 16, ('original typed8 and opaque8 required', len(paths))
    wraps = WRAPS + ['object.checked_new']

    def execute(path, homes=3):
        checked([driver, path, obj])
        checked(['cc', '-DHAKO_TEST_OWNED_CALL_RESULT', obj,
                 TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        for kind, boundary in [('normal', 0),
                               *[('new', n) for n in range(1, homes + 1)],
                               *[('home', n) for n in range(1, homes + 1)]]:
            env = dict(ENV, V4_PROBE_MODE='normal')
            if kind != 'normal':
                env[f'V4_PROBE_{kind.upper()}_FAULT_AT'] = str(boundary)
            run = subprocess.run([exe], env=env, capture_output=True, text=True)
            expected = 0 if kind == 'normal' else 70
            released = boundary - 1 if kind == 'new' else homes
            attempts = boundary if kind == 'new' else homes
            assert run.returncode == expected, (path, kind, boundary, run.stdout, run.stderr)
            assert f'COUNTS 1 0 {released} 0 {int(bool(boundary))} 1\n' in run.stdout, (
                path, kind, boundary, run.stdout)
            assert f'OWNED {attempts} {released} {released}\n' in run.stdout, (
                path, kind, boundary, run.stdout)
            if boundary:
                reason = 101 if kind == 'new' else 102
                assert f'FAULT {reason} ' in run.stdout, run.stdout
            else:
                assert 'FAULT ' not in run.stdout, run.stdout

    for path in paths:
        execute(path)
        print(path.name, 'owned Normal/allocation Fault/cleanup Fault PASS')

    def changed(label, source, mutate, accept=False, homes=3):
        graph = copy.deepcopy(source)
        mutate(graph)
        path = work / (label + '.json')
        path.write_text(json.dumps(graph))
        if accept:
            execute(path, homes)
        else:
            result = subprocess.run([driver, path, obj], capture_output=True, text=True)
            assert result.returncode != 0 and '[freeze:contract]' in result.stderr, (
                label, result.returncode, result.stdout, result.stderr)
        print(label, 'PASS')

    def function(graph, name):
        return next(f for f in graph['functions'] if f['name'] == name)

    def terminators(fn):
        return [b['terminator']['instruction'] for b in fn['blocks']]

    base = json.loads(next(p for p in paths if 'receivedfalse-nullablefalse-optfalse.json' in p.name).read_text())
    nullable = json.loads(next(p for p in paths if 'receivedfalse-nullabletrue-optfalse.json' in p.name).read_text())

    def role_drift(graph):
        function(graph, 'Maker.make/1')['role'] = 'ordinary_nullable_handle'
    changed('wrong-callee-role', base, role_drift)

    def kind_drift(graph):
        term = next(t for t in terminators(function(graph, 'Maker.relay/0'))
                    if t.get('operation', {}).get('kind') == 'ordinary_call')
        term['operation']['result'] = 'nullable_handle'
    changed('wrong-call-result', base, kind_drift)

    def wrong_release(graph):
        term = next(t for t in terminators(graph['functions'][0])
                    if t.get('operation', {}).get('kind') == 'home_release'
                    and t['operation']['object_id'] == 0)
        term['operation']['object_id'] = 1
    changed('wrong-release-object', base, wrong_release)

    def wrong_new(graph):
        term = next(t for t in terminators(function(graph, 'Maker.make/1'))
                    if t.get('operation', {}).get('kind') == 'new_box')
        term['operation']['object_id'] = 1
    changed('callee-object-disagrees-with-caller', base, wrong_new)

    def borrowed_return(graph):
        fn = function(graph, 'Maker.make/1')
        next(t for t in terminators(fn) if t['op'] == 'return')['value'] = fn['receiver']
    changed('borrowed-handle-return', base, borrowed_return)

    def null_return(graph):
        fn = function(graph, 'Maker.make/1')
        for block in fn['blocks']:
            for row in block['instructions']:
                ins = row['instruction']
                if ins['op'] == 'invoke_normal_result':
                    row['instruction'] = {'op': 'const_null', 'dst': ins['dst']}
    changed('null-handle-return', base, null_return)

    def cycle(graph):
        fn = function(graph, 'Maker.make/1')
        term = next(t for t in terminators(fn) if t.get('operation', {}).get('kind') == 'new_box')
        term['operation'] = {'kind': 'ordinary_call', 'result': 'handle',
            'call': {'target': 1, 'receiver': fn['receiver'], 'dst': None,
                     'args': [{'kind': 'i64', 'value': fn['params'][0]['value']}]}}
    changed('cyclic-handle-summary', base, cycle)

    def foreign_landing(graph):
        fn = function(graph, 'Maker.make/1')
        for block in fn['blocks']:
            for row in block['instructions']:
                if row['instruction']['op'] == 'invoke_normal_result':
                    row['instruction']['invoke_block'] = 99999
    changed('foreign-normal-landing', base, foreign_landing)

    def spent_return(graph):
        fn = function(graph, 'Maker.make/1')
        block = next(b for b in fn['blocks'] if b['terminator']['instruction']['op'] == 'return')
        returned = copy.deepcopy(block['terminator']['instruction'])
        frame = next(r['instruction']['dst'] for b in fn['blocks'] for r in b['instructions']
                     if r['instruction']['op'] == 'fault_frame_enter')
        fault = next(b['id'] for b in fn['blocks'] if b['terminator']['instruction']['op'] == 'return_fault')
        block['terminator']['instruction'] = {'op': 'invoke', 'fault_frame': frame,
            'normal': 999, 'fault': fault, 'operation': {'kind': 'home_release',
                'object_id': 0, 'value': returned['value'], 'site': 999}}
        block['edges'] = [{'target': 999, 'args': None}, {'target': fault, 'args': None}]
        fn['blocks'].append({'id': 999, 'instructions': [{'index': 0, 'instruction': {
            'op': 'invoke_normal_result', 'invoke_block': block['id'], 'dst': 10001}}],
            'terminator': {'index': 1, 'instruction': returned}, 'edges': []})
    changed('spent-owned-handle-return', base, spent_return)

    def copy_result(graph):
        fn = function(graph, 'Maker.make/1')
        dst = 10000
        value = None
        for block in fn['blocks']:
            for row in list(block['instructions']):
                if row['instruction']['op'] == 'invoke_normal_result':
                    value = row['instruction']['dst']
                    block['instructions'].append({'index': len(block['instructions']),
                        'instruction': {'op': 'copy', 'src': value, 'dst': dst}})
                    block['terminator']['index'] = len(block['instructions'])
        assert value is not None
        next(t for t in terminators(fn) if t['op'] == 'return')['value'] = dst
    changed('exact-copy-owned-result', base, copy_result, accept=True)

    def reverse_callees(graph):
        graph['functions'][1], graph['functions'][2] = graph['functions'][2], graph['functions'][1]
        for fn in graph['functions']:
            for term in terminators(fn):
                if term.get('operation', {}).get('kind') == 'ordinary_call':
                    call = term['operation']['call']
                    call['target'] = {1: 2, 2: 1}[call['target']]
    changed('callee-index-order-independent', base, reverse_callees, accept=True)

    def null_actual(graph):
        fn = function(graph, 'Maker.relay/0')
        term = next(t for t in terminators(fn) if t.get('operation', {}).get('kind') == 'ordinary_call')
        argument = term['operation']['call']['args'][0]['value']
        ins = next(row['instruction'] for block in fn['blocks'] for row in block['instructions']
                   if row['instruction'].get('dst') == argument)
        assert ins['op'] == 'const_i64' and ins['value'] == 7
        ins['value'] = 0
    # Physical input range probes are separate from the unchanged source cutover.
    changed('nullable-null-physical-arm', nullable, null_actual, accept=True, homes=2)

    def disagreeing_returns(graph):
        for fn in graph['functions'][1:]:
            fn['role'] = 'ordinary_handle'
        for fn in graph['functions']:
            for term in terminators(fn):
                if term.get('operation', {}).get('kind') == 'ordinary_call':
                    term['operation']['result'] = 'handle'
        fn = function(graph, 'Maker.make/1')
        block = next(b for b in fn['blocks'] if any(
            r['instruction']['op'] == 'const_null' for r in b['instructions']))
        value = next(r['instruction']['dst'] for r in block['instructions']
                     if r['instruction']['op'] == 'const_null')
        frame = next(r['instruction']['dst'] for b in fn['blocks'] for r in b['instructions']
                     if r['instruction']['op'] == 'fault_frame_enter')
        fault = next(b['id'] for b in fn['blocks'] if b['terminator']['instruction']['op'] == 'return_fault')
        block['instructions'] = [r for r in block['instructions'] if r['instruction']['op'] != 'const_null']
        block['terminator'] = {'index': len(block['instructions']), 'instruction': {
            'op': 'invoke', 'fault_frame': frame, 'normal': 999, 'fault': fault,
            'operation': {'kind': 'new_box', 'object_id': 1, 'site': 999}}}
        block['edges'] = [{'target': 999, 'args': None}, {'target': fault, 'args': None}]
        fn['blocks'].append({'id': 999, 'instructions': [{'index': 0, 'instruction': {
            'op': 'invoke_normal_result', 'invoke_block': block['id'], 'dst': value}}],
            'terminator': {'index': 1, 'instruction': {'op': 'jump', 'target': 8, 'args': None}},
            'edges': [{'target': 8, 'args': None}]})
    changed('all-return-object-agreement', nullable, disagreeing_returns)


with tempfile.TemporaryDirectory(prefix='hako call result composition ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'issued.o', 'issued')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    if OWNED:
        owned_results(driver, obj, exe, work)
        sys.exit(0)
    for reverse in [False, True]:
        for annotated in [False, True]:
            for optimize in [False, True]:
                for domain, expected, stores in [('object', 5, 4), ('null', 7, 3)]:
                    path = INPUTS / (
                        f'{STEM}-{domain}'
                        f'-reverse{str(reverse).lower()}-annotated{str(annotated).lower()}'
                        f'-opt{str(optimize).lower()}.json'
                    )
                    checked([driver, path, obj])
                    checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                             *['-Wl,--wrap=nyash.' + name + '_v1' for name in WRAPS],
                             '-lpthread', '-ldl', '-lm', '-o', exe])
                    run = subprocess.run([exe], env=dict(ENV, V4_PROBE_MODE='normal'), capture_output=True, text=True)
                    assert run.returncode == expected, (path, run.returncode, run.stdout, run.stderr)
                    assert f'COUNTS 1 {stores} {stores} 0 0 1\n' in run.stdout, (path, run.stdout)
                    assert 'FAULT ' not in run.stdout, run.stdout
                    for fault_at in range(1, stores + 1):
                        run = subprocess.run([exe], env=dict(ENV, V4_PROBE_MODE='normal', V4_PROBE_FAULT_AT=str(fault_at)),
                                             capture_output=True, text=True)
                        assert run.returncode == 70, (path, fault_at, run.returncode, run.stdout, run.stderr)
                        assert f'COUNTS 1 {fault_at} {fault_at - 1} 1 1 1\n' in run.stdout, (path, fault_at, run.stdout)
                        assert 'FAULT 101 ' in run.stdout, run.stdout
                    print(domain, reverse, annotated, optimize, 'composed result and Normal/Fault cleanup PASS')
