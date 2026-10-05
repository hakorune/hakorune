#!/usr/bin/env python3
"""Run unchanged supported Birth inventory captures with each runtime Fault."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
ROOT = Path(__file__).resolve().parents[3]
TESTS = ROOT / 'lang/c-abi/tests'
ARCHIVE = Path(os.environ.get('HAKO_TEST_LIFECYCLE_ARCHIVE',
    ROOT / 'target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a'))
ENV = dict(os.environ, NYASH_NYRT_SILENT_RESULT='1', HAKO_NYRT_PLUGIN_HOST='off')
def checked(args, **kwargs):
    result = subprocess.run([str(x) for x in args], capture_output=True, text=True, **kwargs)
    assert result.returncode == 0, (args, result.stdout, result.stderr)
    return result
assert len(sys.argv) == 3 and ARCHIVE.is_file()
with tempfile.TemporaryDirectory(prefix='hako owned slots ') as directory:
    work = Path(directory)
    driver, obj, exe = [work / x for x in ['driver', 'source.o', 'source']]
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c',
        '-L' + str(ROOT / 'target/release'), '-lhako_llvmc_ffi',
        '-Wl,-rpath,' + str(ROOT / 'target/release'), '-o', driver])
    wraps = ['object.checked_new_v1', 'array.checked_new_v1',
        'object.checked_field_set_v1', 'object.home_release_plain_i64_v1',
        'object.reclaim_unpublished_v1', 'fault.frame_init_v1',
        'fault.frame_dispose_v1', 'fault.report_final_v1']
    for capture in sys.argv[1:]:
        checked([driver, capture, obj], env=ENV)
        checked(['cc', obj, TESTS / 'published_lifecycle_v4_owned_slot_consumer_probe.c',
            ARCHIVE, *['-Wl,--wrap=nyash.' + name for name in wraps],
            '-Wl,--wrap=nyrt_handle_release_h', '-lpthread', '-ldl', '-lm', '-o', exe])
        cases = [('normal', 0)] + [(lane, n) for lane, limit in
            [('object', 3), ('array', 5), ('store', 7)] for n in range(1, limit + 1)]
        for lane, n in cases:
            result = subprocess.run([str(exe)], capture_output=True, text=True,
                env=dict(ENV, OWNED_SLOT_FAULT=lane, OWNED_SLOT_AT=str(n)))
            assert result.returncode == (70 if n else 0), (
                capture, lane, n, result.returncode, result.stdout, result.stderr)
            assert f'OWNED_SLOT_OK {lane} {n} ' in result.stdout, result
            print(Path(capture).name, result.stdout.strip())
