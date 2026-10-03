#!/usr/bin/env python3
"""Execute guarded formal `handle.field` JSON emitted by the Rust tests.

The borrowed kind/payload formal carries its sealed `object_view` on the
param row; the one `object_field_get` names exactly that canonical object
and runs under the admitted `== null` non-null successor. The callee
takes no release or End — the caller still owns the borrowed object.
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

# release(handle): if handle == null { return 0 }
#   local id = handle.page_id; if id > 0 { return 1 } return 0
# The caller passes check(5)'s Handle(page_id=5): main answers 1.
CASES = ['handle']


def checked(argv, **kwargs):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True, **kwargs)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


def instructions(data, name):
    callee = next(f for f in data['functions'] if f['name'] == name)
    return [row['instruction'] for b in callee['blocks']
            for row in b['instructions']]


with tempfile.TemporaryDirectory(prefix='hako param field ') as directory:
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

    issued = {}
    for suffix in CASES:
        path = DIRECTORY / ('hako-issued-param-field-' + suffix + '.json')
        data = json.loads(path.read_text())
        callee = next(f for f in data['functions'] if f['name'] == 'Store.release/1')
        param = callee['params'][0]
        assert param['representation'] == 'borrowed_kind_payload_v1', param
        reads = [row for row in instructions(data, 'Store.release/1')
                 if row['op'] == 'object_field_get']
        assert len(reads) == 1, 'exactly one guarded formal read'
        assert param['object_view'] == reads[0]['object_id']
        compile_input(data)
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_runtime_probe.c', ARCHIVE,
                 *['-Wl,--wrap=nyash.' + name + '_v1' for name in wraps],
                 '-lpthread', '-ldl', '-lm', '-o', exe])
        run = subprocess.run([str(exe)], env=ENV, capture_output=True, text=True)
        assert run.returncode == 1, (suffix, run.returncode, run.stdout, run.stderr)
        assert 'FAULT ' not in run.stdout, (suffix, run.stdout)
        issued[suffix] = data
        print(suffix, 'guarded formal field read executes; page_id=5 answers 1')

    base = issued['handle']

    def mutate(change):
        data = copy.deepcopy(base)
        callee = next(f for f in data['functions'] if f['name'] == 'Store.release/1')
        change(callee, data)
        compile_input(data, False)

    # The sealed view is mandatory on the borrowed param row.
    def drop_view(callee, _):
        del callee['params'][0]['object_view']
    mutate(drop_view)

    # A different canonical object is drift, never a second authority.
    def wrong_view(callee, _):
        callee['params'][0]['object_view'] += 1
    mutate(wrong_view)

    # The key belongs only to the borrowed kind/payload transport.
    def wrong_transport(callee, _):
        callee['params'][0]['representation'] = 'kind_payload_v1'
    mutate(wrong_transport)

    # A view naming no declared layout rejects.
    def foreign_view(callee, _):
        callee['params'][0]['object_view'] = 999999
    mutate(foreign_view)

    # An extra param key breaks the exact physical shape.
    def extra_key(callee, _):
        callee['params'][0]['extra'] = 1
    mutate(extra_key)

    # A second read is coverage drift; exactly one row serves the use.
    def second_read(callee, data):
        rows = [row for block in callee['blocks'] for row in block['instructions']]
        read = next(r['instruction'] for r in rows
                    if r['instruction']['op'] == 'object_field_get')
        block = callee['blocks'][0]
        block['instructions'].append({
            'index': len(block['instructions']),
            'instruction': dict(read)})
    mutate(second_read)

    # The read must name the sealed object, not another layout.
    def wrong_object(callee, _):
        for block in callee['blocks']:
            for row in block['instructions']:
                if row['instruction']['op'] == 'object_field_get':
                    row['instruction']['object_id'] += 1
    mutate(wrong_object)

    print('1 source-issued program executes; seven forged physical rows reject')
