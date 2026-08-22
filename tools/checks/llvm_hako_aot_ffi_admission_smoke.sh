#!/usr/bin/env bash
# Runtime proof for the generic-vs-named Hako AOT C-ABI boundary.
# The generic entry must never inherit harness replay; only the named
# compatibility entry may reach the frozen llvmlite keep lane.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="llvm-hako-aot-ffi-admission-smoke"

bash "$ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null

ROOT="$ROOT" python3 - <<'PY'
import ctypes
import os
import pathlib
import tempfile

root = pathlib.Path(os.environ["ROOT"])
lib = ctypes.CDLL(str(root / "target/release/libhako_llvmc_ffi.so"))
for name in (
    "hako_aot_compile_json",
    "hako_aot_compile_json_compat_harness",
    "hako_llvmc_compile_json",
    "hako_llvmc_compile_json_compat_harness",
):
    fn = getattr(lib, name)
    fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
    fn.restype = ctypes.c_int
lib.hako_mem_free.argtypes = [ctypes.c_void_p]

fixture = root / "apps/tests/mir_shape_guard/ret_const_min_v1.mir.json"
if not fixture.is_file():
    raise SystemExit(f"missing fixture: {fixture}")

def call(name, env, out):
    old = os.environ.copy()
    os.environ.update(env)
    err = ctypes.c_void_p()
    rc = getattr(lib, name)(str(fixture).encode(), str(out).encode(), ctypes.byref(err))
    message = ctypes.string_at(err.value).decode(errors="replace") if err.value else ""
    if err.value:
        lib.hako_mem_free(err)
    os.environ.clear()
    os.environ.update(old)
    return rc, out.is_file(), message

with tempfile.TemporaryDirectory(prefix="hako-aot-ffi-admission-") as temp:
    out = pathlib.Path(temp)
    rc, exists, message = call(
        "hako_aot_compile_json",
        {"HAKO_AOT_USE_FFI": "0", "HAKO_BACKEND_COMPAT_REPLAY": "harness"},
        out / "generic-direct.o",
    )
    if rc == 0 or exists or "aot-compat-admission-required" not in message:
        raise SystemExit("generic direct AOT inherited harness replay")

    rc, exists, message = call(
        "hako_aot_compile_json_compat_harness",
        {"HAKO_AOT_USE_FFI": "0", "HAKO_BACKEND_COMPAT_REPLAY": "none"},
        out / "named-direct.o",
    )
    if rc != 0 or not exists:
        raise SystemExit(f"named direct compatibility lane failed: {message}")

    rc, exists, message = call(
        "hako_aot_compile_json",
        {
            "HAKO_AOT_USE_FFI": "1",
            "HAKO_BACKEND_COMPILE_RECIPE": "pure-first",
            "HAKO_BACKEND_COMPAT_REPLAY": "harness",
        },
        out / "generic-ffi-replay.o",
    )
    if rc == 0 or exists or "aot-compat-admission-required" not in message:
        raise SystemExit("generic FFI AOT inherited harness replay")

    rc, exists, message = call(
        "hako_aot_compile_json_compat_harness",
        {"HAKO_AOT_USE_FFI": "1", "HAKO_BACKEND_COMPAT_REPLAY": "none"},
        out / "named-ffi.o",
    )
    if rc != 0 or not exists:
        raise SystemExit(f"named FFI compatibility lane failed: {message}")

    rc, exists, message = call(
        "hako_llvmc_compile_json",
        {"HAKO_BACKEND_COMPILE_RECIPE": "pure-first", "HAKO_BACKEND_COMPAT_REPLAY": "harness"},
        out / "generic-capi-replay.o",
    )
    if rc == 0 or exists or "generic-capi-compat-admission-required" not in message:
        raise SystemExit("generic C ABI inherited harness replay")

    rc, exists, message = call(
        "hako_llvmc_compile_json_compat_harness",
        {"HAKO_BACKEND_COMPAT_REPLAY": "none"},
        out / "named-capi.o",
    )
    if rc != 0 or not exists:
        raise SystemExit(f"named C ABI compatibility lane failed: {message}")

print("[llvm-hako-aot-ffi-admission-smoke] ok (generic AOT/C replay rejected; named direct/FFI/C keep lanes succeeded)")
PY
