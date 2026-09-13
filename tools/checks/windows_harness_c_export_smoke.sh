#!/usr/bin/env bash
# Native Windows proof for the direct named C harness executor.
set -euo pipefail

ROOT_POSIX="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
if command -v pwd >/dev/null 2>&1 && pwd -W >/dev/null 2>&1; then
  ROOT_NATIVE="$(cd "$ROOT_POSIX" && pwd -W)"
else
  ROOT_NATIVE="$ROOT_POSIX"
fi

cc_cmd=${CC:-}
if [[ -z "$cc_cmd" ]]; then
  if command -v clang >/dev/null 2>&1; then
    cc_cmd=clang
  elif command -v gcc >/dev/null 2>&1; then
    cc_cmd=gcc
  elif command -v cc >/dev/null 2>&1; then
    cc_cmd=cc
  fi
fi
if [[ -z "$cc_cmd" ]]; then
  echo "native C compiler (clang, gcc, or cc) is required" >&2
  exit 2
fi

CC="$cc_cmd" bash "$ROOT_POSIX/tools/build_hako_llvmc_ffi.sh" >/dev/null
ROOT_NATIVE="$ROOT_NATIVE" CC="$cc_cmd" python - <<'PY'
import ctypes
import os
import pathlib
import shutil
import subprocess
import tempfile

root = pathlib.Path(os.environ["ROOT_NATIVE"])
compiler = os.environ["CC"]
library_path = root / "target" / "release" / "hako_llvmc_ffi.dll"
if os.name != "nt":
    raise SystemExit("native Windows Python is required")
if not library_path.is_file():
    raise SystemExit(f"missing native DLL: {library_path}")

child_source = root / "tools" / "checks" / "lib" / "harness_direct_child_probe.c"
temp_path = pathlib.Path(tempfile.mkdtemp(prefix="hako direct harness "))
try:
    child_path = temp_path / "fake compiler with spaces.exe"
    subprocess.run(
        [compiler, "-std=c11", "-Wall", "-Wextra", "-Werror",
         str(child_source), "-o", str(child_path)],
        check=True,
    )
    input_path = temp_path / "input with spaces.json"
    input_path.write_bytes(b'{"functions":[]}')
    output_path = temp_path / "output with spaces.o"
    log_dir = temp_path / "tmp logs with spaces"
    log_dir.mkdir()

    lib = ctypes.CDLL(str(library_path))
    harness = lib.hako_llvmc_compile_json_compat_harness
    harness.argtypes = [ctypes.c_char_p, ctypes.c_char_p,
                        ctypes.POINTER(ctypes.c_void_p)]
    harness.restype = ctypes.c_int
    lib.hako_mem_free.argtypes = [ctypes.c_void_p]

    saved = os.environ.copy()
    try:
        os.environ["NYASH_NY_LLVM_COMPILER"] = str(child_path)
        os.environ["TMPDIR"] = str(log_dir)

        def call(mode):
            if mode is None:
                os.environ.pop("HAKO_HARNESS_TEST_MODE", None)
            else:
                os.environ["HAKO_HARNESS_TEST_MODE"] = mode
            error = ctypes.c_void_p()
            rc = harness(str(input_path).encode(), str(output_path).encode(),
                         ctypes.byref(error))
            message = (ctypes.string_at(error.value).decode(errors="replace")
                       if error.value else "")
            if error.value:
                lib.hako_mem_free(error)
            return rc, message

        rc, message = call(None)
        if rc != 0 or output_path.read_bytes() != b"native-child-object":
            raise SystemExit(f"direct C success failed: rc={rc} {message!r}")
        if list(log_dir.iterdir()):
            raise SystemExit("success left an invocation-owned harness log")

        output_path.unlink()
        rc, message = call("fail")
        if rc == 0 or "direct child failure" not in message or "second line" in message:
            raise SystemExit(f"direct C child failure projection failed: rc={rc} {message!r}")
        if output_path.exists() or list(log_dir.iterdir()):
            raise SystemExit("child failure left output or an invocation-owned log")

        rc, message = call("missing")
        if rc == 0 or "finished without object" not in message:
            raise SystemExit(f"direct C no-object projection failed: rc={rc} {message!r}")
        if output_path.exists() or list(log_dir.iterdir()):
            raise SystemExit("no-object child left output or an invocation-owned log")
    finally:
        os.environ.clear()
        os.environ.update(saved)
    print("direct C harness: native child reopen, failure projection, and log cleanup PASS")
finally:
    shutil.rmtree(temp_path, ignore_errors=True)
PY
