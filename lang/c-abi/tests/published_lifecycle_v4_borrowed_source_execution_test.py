#!/usr/bin/env python3
"""Execute unchanged JSON emitted by the original borrowed-source Rust tests.

No synthetic Call, carrier, CFG or cleanup repair: the source-issued input is
passed directly to the existing C driver and linked with the real kernel.
"""
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


with tempfile.TemporaryDirectory(prefix='hako original borrowed source ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'source.o', 'source')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    wraps = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
             'object.checked_field_set', 'object.home_release_plain_i64', 'object.reclaim_unpublished']
    profiles = [(shape, domain) for domain in ('zero', 'negative', 'true', 'false', 'object', 'scalar', 'bool-scalar')
                for shape in ('local', 'discard', 'return', 'nested')]
    profiles.extend([('forwarded', None), ('entry-receiver', None)])
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
    print('30 original-source inputs execute unchanged; no carrier/CFG repair')
