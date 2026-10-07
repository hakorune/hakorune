#!/usr/bin/env python3
"""Execute original Static local source JSON and reject physical ABI drift.

The positive is unchanged output from the Rust original-cohort test. Mutations
are negative external-input tests and do not issue source-language authority.
Build a private current-source shim; leave shared binaries untouched.
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
SOURCE, ARCHIVE = map(Path, sys.argv[1:3])
ENV = dict(os.environ, NYASH_NYRT_SILENT_RESULT='1', HAKO_NYRT_PLUGIN_HOST='off')
base = json.loads(SOURCE.read_text())


def checked(argv):
    result = subprocess.run(list(map(str, argv)), capture_output=True,
                            text=True, timeout=120, env=ENV)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


def static_function(data):
    return next(f for f in data['functions'] if f['name'] == 'Layout.pick/1')


def static_call(data):
    target = data['functions'].index(static_function(data))
    for block in data['functions'][0]['blocks']:
        operation = block['terminator']['instruction'].get('operation', {})
        if operation.get('kind') == 'ordinary_call' and operation['call']['target'] == target:
            return operation['call']
    raise AssertionError('original root Static call missing')


with tempfile.TemporaryDirectory(prefix='hako static borrowed local ') as directory:
    work = Path(directory)
    shim = ROOT / 'lang/c-abi/shims'
    yyjson = ROOT / 'plugins/nyash-json-plugin/c/yyjson'
    checked(['cc', '-fPIC', '-shared', '-I' + str(yyjson), '-o',
             work / 'libhako_llvmc_ffi.so', shim / 'hako_llvmc_ffi.c',
             shim / 'hako_aot.c', shim / 'hako_json_v1.c', yyjson / 'yyjson.c'])
    driver, obj, exe = [work / name for name in ('driver', 'source.o', 'source')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(work),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(work), '-o', driver])
    checked([driver, SOURCE, obj])
    checked(['cc', obj, ARCHIVE, '-lpthread', '-ldl', '-lm', '-o', exe])
    checked([exe])
    for case in ['receiver-object', 'instance-receiver', 'nullable-static',
                 'invalid-tag', 'missing-actual', 'scalar-formal', 'missing-receiver',
                 'missing-receiver-object', 'map-static', 'call-receiver',
                 'duplicate-formal', 'raw-i64', 'malformed-bool', 'malformed-null']:
        data = copy.deepcopy(base)
        callee, call = static_function(data), static_call(data)
        if case == 'receiver-object':
            callee['receiver_object'] = 0
        elif case == 'instance-receiver':
            callee['receiver'], callee['receiver_object'] = 0, 0
        elif case == 'nullable-static':
            callee['role'] = 'ordinary_nullable_handle'
        elif case == 'invalid-tag':
            call['args'][0]['kind'] = 4
        elif case == 'missing-actual':
            call['args'] = []
        elif case == 'missing-receiver':
            del callee['receiver']
        elif case == 'missing-receiver-object':
            del callee['receiver_object']
        elif case == 'map-static':
            callee['role'] = 'ordinary_map'
        elif case == 'call-receiver':
            call['receiver'] = call['args'][0]['value']
        elif case == 'duplicate-formal':
            callee['params'].append(copy.deepcopy(callee['params'][0]))
        elif case == 'raw-i64':
            call['args'][0]['kind'] = 'i64'
        elif case == 'malformed-bool':
            call['args'][0]['kind'] = 2
        elif case == 'malformed-null':
            call['args'][0]['kind'] = 0
        else:
            callee['params'][0] = {'value': callee['params'][0]['value'],
                                   'representation': 'i64'}
        path, rejected = work / (case + '.json'), work / (case + '.o')
        path.write_text(json.dumps(data))
        result = subprocess.run(list(map(str, [driver, path, rejected])),
                                capture_output=True, text=True, timeout=120, env=ENV)
        assert result.returncode != 0, (case, result.stdout, result.stderr)
        assert '[freeze:contract]' in result.stderr, (case, result.stderr)
        assert not rejected.exists(), (case, 'rejected input published an artifact')
    print('Original Static source compile/link/run and fourteen ABI-drift negatives pass')
