#!/usr/bin/env python3
"""Execute exact source-issued composed call results and exact cleanup in both finishing modes."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
TESTS = ROOT / 'lang/c-abi/tests'
INPUTS = Path(sys.argv[1])
ARCHIVE = ROOT / 'target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a'
ENV = dict(os.environ, NYASH_NYRT_SILENT_RESULT='1', HAKO_NYRT_PLUGIN_HOST='off')
ENV.pop('V4_PROBE_FAULT_AT', None)
WRAPS = ['fault.frame_init', 'fault.frame_dispose', 'fault.report_final',
         'object.checked_field_set', 'object.home_release_plain_i64', 'object.reclaim_unpublished']


def checked(argv):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


with tempfile.TemporaryDirectory(prefix='hako call result composition ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ('driver', 'issued.o', 'issued')]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(ROOT / 'target/release'),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    for reverse in [False, True]:
        for annotated in [False, True]:
            for optimize in [False, True]:
                for domain, expected, stores in [('object', 5, 4), ('null', 7, 3)]:
                    path = INPUTS / (
                        f'hako-issued-borrowed-call-result-{domain}'
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
