#!/usr/bin/env python3
"""PARAMFIELD-ACCEPTANCE-R0: consolidated source -> JSON -> C -> OBJ -> EXE.

The selected frontier shapes execute through the real driver on the
source-issued inputs — the unused formal/I64 result, the local-new
TypedHome call, the exact null guard (literal-null and object actuals),
the guarded field initializer, terminal return and order-compare
operands, and the literal null/received-nullable actuals. Each family
runs its normal, null-state or injected-fault lane; inverse forged rows
still reject before OBJ.
Caller cleanup is exactly once per required exit and the callee never
disposes the borrowed formal.
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


def all_rows(data, name):
    callee = next(f for f in data['functions'] if f['name'] == name)
    rows = [row['instruction'] for b in callee['blocks']
            for row in b['instructions']]
    rows += [b['terminator']['instruction'] for b in callee['blocks']]
    return rows


def op_of(row):
    if 'op' in row:
        return row['op']
    return row.get('operation', {}).get('kind')


def field_set_sites(data, name):
    callee = next(f for f in data['functions'] if f['name'] == name)
    return [b['terminator']['instruction']['operation']['site']
            for b in callee['blocks']
            if b['terminator']['instruction'].get('operation', {}).get('kind')
            == 'field_set']


with tempfile.TemporaryDirectory(prefix='hako param acceptance ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'source.o', 'source')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.home_release_plain_i64',
             'object.reclaim_unpublished']

    def compile_input(data, expected=True):
        path = work / 'input.json'
        path.write_text(json.dumps(data))
        result = subprocess.run([driver, path, obj], env=ENV, capture_output=True, text=True)
        assert (result.returncode == 0) == expected, (result.stdout, result.stderr)
        assert not list(work.glob('source.o.*')), 'temporary artifact leak'
        return result

    def link():
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])

    def run(env=ENV):
        return subprocess.run([str(exe)], env=env, capture_output=True, text=True)

    def execute(data, expected, homes, stores):
        compile_input(data)
        link()
        normal = run()
        assert normal.returncode == expected, (normal.returncode, normal.stdout, normal.stderr)
        assert 'FAULT ' not in normal.stdout, normal.stdout
        assert f'COUNTS 1 {stores} {homes} 0 0 1\n' in normal.stdout, normal.stdout

    issued = {}
    for suffix in ('unused-i64', 'localnew', 'null-guard', 'null-guard-object',
                   'field-init', 'field-return', 'cmp', 'cmp-ge'):
        name = {'unused-i64': 'param-acceptance-unused-i64',
                'localnew': 'param-acceptance-localnew',
                'null-guard': 'param-acceptance-null-guard',
                'null-guard-object': 'null-compare-object',
                'field-init': 'param-field-handle',
                'field-return': 'param-field-return',
                'cmp': 'param-field-cmp',
                'cmp-ge': 'param-field-cmp-ge'}[suffix]
        data = json.loads((DIRECTORY / ('hako-issued-' + name + '.json')).read_text())
        issued[suffix] = data

    # 1. Unused opaque formal + unannotated I64 result: direct caller
    #    return executes; only the Store home releases, once.
    execute(issued['unused-i64'], 0, 1, 1)
    print('unused-i64: unused formal + I64 result executes; caller cleanup once')

    # 2. Local-new TypedHome call: caller constructs Item and passes it;
    #    both homes release exactly once on the normal exit.
    execute(issued['localnew'], 0, 2, 3)
    print('localnew: local-new TypedHome actual executes; two homes release once')

    # 3. Exact null guard: literal-null actual takes the null arm (7);
    #    the object actual answers 3 — the compare reads the carrier kind,
    #    never the Integer view.
    execute(issued['null-guard'], 7, 1, 1)
    execute(issued['null-guard-object'], 3, 1, 1)
    print('null-guard: null actual answers 7; object actual answers 3')

    # 4/5. Guarded field initializer and direct terminal return: the
    #    sealed object view serves exactly one object_field_get on the
    #    formal base; received-nullable actuals cross as
    #    nullable_typed_object.
    for suffix, expected in (('field-init', 1), ('field-return', 5)):
        data = issued[suffix]
        callee = next(f for f in data['functions'] if f['name'] == 'Store.release/1')
        param = callee['params'][0]
        assert param['representation'] == 'borrowed_kind_payload_v1', param
        reads = [row for row in instructions(data, 'Store.release/1')
                 if row['op'] == 'object_field_get']
        assert len(reads) == 1, (suffix, 'exactly one guarded formal read')
        assert param['object_view'] == reads[0]['object_id']
        # The callee never disposes the borrowed formal.
        assert not any(op_of(row) in ('home_release', 'home_release_if_live',
                                      'reclaim_unpublished', 'object_field_release')
                       for row in all_rows(data, 'Store.release/1')), suffix
        execute(data, expected, 2, 3)
        print(suffix, 'guarded field read executes; page_id=5 answers', expected)

    # 6/7. Guarded order-compare operands: `<`/`>=` read the same sealed
    #      formal — `slt`/`sge` compare the claimed object_field_get and
    #      the `me.limit` receiver read rides the same issued root.
    for suffix, predicate in (('cmp', 'slt'), ('cmp-ge', 'sge')):
        data = issued[suffix]
        callee = next(f for f in data['functions'] if f['name'] == 'Store.release/1')
        reads = [row for row in instructions(data, 'Store.release/1')
                 if row['op'] == 'object_field_get']
        assert callee['params'][0]['object_view'] == reads[0]['object_id'], suffix
        compares = [row for row in instructions(data, 'Store.release/1')
                    if row['op'] == 'compare']
        assert any(row['predicate'] == predicate for row in compares), (suffix, compares)
        assert not any(op_of(row) in ('home_release', 'home_release_if_live',
                                      'reclaim_unpublished', 'object_field_release')
                       and row['operation'].get('value') == callee['params'][0]['value']
                       for row in all_rows(data, 'Store.release/1')), suffix
        execute(data, 1, 2, 3)
        print(suffix, 'order-compare operand executes; predicate', predicate)

    # Null state: the same issued field-return input with check(20)
    # selecting the Void pair — the callee's own guard arm answers 0 and
    # release-if-live touches nothing.
    null_state = copy.deepcopy(issued['field-return'])
    for row in instructions(null_state, 'main'):
        if row.get('op') == 'const_i64' and row.get('value') == 5:
            row['value'] = 20
    execute(null_state, 0, 1, 1)
    print('field-return-null: Void state transports (0,0); guard arm answers 0')

    # Literal null and live received-nullable actuals from their sibling
    # witnesses: tag 0 beside const_null; live nullable selects tag 3.
    null_actual = json.loads((DIRECTORY / 'hako-issued-null-actual-null.json').read_text())
    execute(null_actual, 0, 1, 1)
    print('null-actual: literal null executes; the Void sentinel is never released')
    nullable = json.loads((DIRECTORY / 'hako-issued-null-actual-nullable.json').read_text())
    execute(nullable, 0, 2, 3)
    print('nullable-actual: live received nullable executes; h and s release once')

    # Injected fault: the Item/Handle birth store fails at runtime — the
    # unpublished object reclaims, the completed Store home releases once,
    # and the fault reports the exact birth site.
    for suffix, fname in (('localnew', 'Item.birth/2'), ('field-return', 'Handle.birth/2')):
        data = issued[suffix]
        sites = field_set_sites(data, fname)
        assert sites, (suffix, 'birth field_set sites')
        compile_input(data)
        link()
        fault = run(dict(ENV, V4_PROBE_FAULT_AT='2'))
        assert fault.returncode == 70, (suffix, fault.returncode, fault.stdout)
        assert 'COUNTS 1 2 1 1 1 1\n' in fault.stdout, (suffix, fault.stdout)
        assert any(line.startswith(f'FAULT 101 {sites[0]} ')
                   and 'HOME 1 RECLAIM 1' in line
                   for line in fault.stdout.splitlines()), (suffix, fault.stdout)
        print(suffix, 'injected birth-store fault unwinds; cleanup once + one reclaim')

    # Inverse failures: forged physical rows still reject before OBJ.
    def mutate(base_key, change):
        data = copy.deepcopy(issued[base_key])
        change(data)
        compile_input(data, False)

    # The sealed view is mandatory on the borrowed param row.
    def drop_view(data):
        callee = next(f for f in data['functions'] if f['name'] == 'Store.release/1')
        del callee['params'][0]['object_view']
    mutate('field-return', drop_view)

    # The read must name the sealed object, not another layout.
    def wrong_object(data):
        for row in instructions(data, 'Store.release/1'):
            if row['op'] == 'object_field_get':
                row['object_id'] += 1
    mutate('field-return', wrong_object)

    # A forged Integer tag on a null actual has no real object behind it.
    def forged_tag(data):
        main = next(f for f in data['functions'] if f['name'] == 'main')
        for block in main['blocks']:
            term = block['terminator']['instruction']
            call = term.get('operation', {}).get('call')
            if not call:
                continue
            for arg in call['args']:
                if arg['kind'] == 0:
                    arg['kind'] = 1
                    return
        raise AssertionError('null actual missing')
    mutate('null-guard', forged_tag)

    # Removing the exact const_null producer leaves a carrier without its
    # admitted sibling.
    def drop_null(data):
        callee = next(f for f in data['functions'] if f['name'] == 'Store.check/1')
        for block in callee['blocks']:
            block['instructions'] = [
                row for row in block['instructions']
                if row['instruction']['op'] != 'const_null']
    mutate('null-guard', drop_null)

    # An out-of-range wire tag is ABI drift at the parser.
    def tag_over_range(data):
        main = next(f for f in data['functions'] if f['name'] == 'main')
        for block in main['blocks']:
            term = block['terminator']['instruction']
            call = term.get('operation', {}).get('call')
            if not call:
                continue
            for arg in call['args']:
                if arg['kind'] == 'nullable_typed_object':
                    arg['kind'] = 4
                    return
        raise AssertionError('nullable actual missing')
    mutate('field-return', tag_over_range)

    print('10 source-issued inputs execute (normal/null/fault); five forged rows reject')
