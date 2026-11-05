"""Shared setup for the published lifecycle V4 Python tests (import only).

Owns the lifecycle archive choice, the process environment, the `cc` call
(including the linker choice) and the C shim `.so` build, so a test file keeps
only what it checks. `python3 <this file> build-shims <dir>` builds the shims
once; point `HAKO_TEST_LLVMC_SHIM_DIR` at that directory to skip the per-test
build.
"""
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
TESTS = ROOT / 'lang/c-abi/tests'
TARGET = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target'))
ARCHIVE = Path(os.environ.get('HAKO_TEST_LIFECYCLE_ARCHIVE',
                              TARGET / 'lifecycle-kernel/release/libnyash_lifecycle_kernel.a'))
CC = shlex.split(os.environ.get('CC') or 'cc')
SHIM_SOURCES = [ROOT / 'lang/c-abi/shims' / name
                for name in ('hako_llvmc_ffi.c', 'hako_aot.c', 'hako_json_v1.c')]
YYJSON = ROOT / 'plugins/nyash-json-plugin/c/yyjson'


def env(**extra):
    return dict(os.environ, NYASH_NYRT_SILENT_RESULT='1', HAKO_NYRT_PLUGIN_HOST='off', **extra)


def link_flags():
    """Same choice as the product AOT (hako_aot_shared_impl.inc): mold when it
    is installed, `HAKO_AOT_FUSE_LD=default` keeps the compiler default and
    `HAKO_AOT_FUSE_LD=<name>` picks that linker."""
    want = os.environ.get('HAKO_AOT_FUSE_LD')
    if want:
        return [] if want == 'default' or not re.fullmatch(r'[a-z0-9_-]+', want) else ['-fuse-ld=' + want]
    return ['-fuse-ld=mold'] if shutil.which('mold') else []


LINK_FLAGS = link_flags()


def run(argv, **kwargs):
    command = [str(x) for x in argv]
    if command and command[0] == 'cc':
        flags = [] if any(x.startswith('-fuse-ld=') for x in command) else LINK_FLAGS
        command = CC + flags + command[1:]
    return subprocess.run(command, capture_output=True, text=True, **kwargs)


def checked(argv, **kwargs):
    result = run(argv, **kwargs)
    assert result.returncode == 0, (argv, result.stdout, result.stderr)
    return result


def build_shim(target, seam=False):
    checked(['cc', *(['-DHAKO_LLVMC_LIFECYCLE_TEST_SEAM'] if seam else []), '-fPIC', '-shared',
             '-I' + str(YYJSON), '-o', target, *SHIM_SOURCES, YYJSON / 'yyjson.c'])


def release_driver(work, directory=ROOT / 'target/release'):
    """Link the V4 driver against the cargo-built `libhako_llvmc_ffi.so` in `directory`."""
    driver = Path(work) / 'driver'
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(directory),
             '-lhako_llvmc_ffi', '-Wl,-rpath,' + str(directory), '-o', driver])
    return driver


def compile_driver(work, seam=False):
    """Link the V4 driver against the C shim: a prebuilt one from
    `HAKO_TEST_LLVMC_SHIM_DIR`, else one built into `work`. `seam` selects the
    shim that exposes the lifecycle test seam."""
    name = 'libhako_llvmc_ffi_test.so' if seam else 'libhako_llvmc_ffi.so'
    prebuilt = os.environ.get('HAKO_TEST_LLVMC_SHIM_DIR')
    directory = Path(prebuilt) if prebuilt else Path(work)
    if not (directory / name).is_file():
        assert not prebuilt, ('HAKO_TEST_LLVMC_SHIM_DIR lacks ' + name, directory)
        build_shim(directory / name, seam)
    driver = Path(work) / 'driver'
    checked(['cc', TESTS / 'published_lifecycle_v4_driver.c', '-L' + str(directory),
             '-l' + name[3:-3], '-Wl,-rpath,' + str(directory), '-o', driver])
    return driver


if __name__ == '__main__':
    assert len(sys.argv) == 3 and sys.argv[1] == 'build-shims', 'usage: build-shims <dir>'
    Path(sys.argv[2]).mkdir(parents=True, exist_ok=True)
    build_shim(Path(sys.argv[2]) / 'libhako_llvmc_ffi.so')
    build_shim(Path(sys.argv[2]) / 'libhako_llvmc_ffi_test.so', seam=True)
