#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="dynamic-v2-w6-explicit-link-abi"
FFI="$ROOT_DIR/target/release/libhako_llvmc_ffi.so"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/$TAG.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null
printf '%s\n' 'int main(void) { return 0; }' | cc -x c -c -o "$TMP_DIR/input.o" -
ar rcs "$TMP_DIR/explicit-runtime.a"
cp "$TMP_DIR/explicit-runtime.a" "$TMP_DIR/libhako_kernel.a"

HAKO_AOT_USE_FFI="" NYASH_EMIT_EXE_NYRT="$TMP_DIR/does-not-exist" \
python3 - "$FFI" "$TMP_DIR/input.o" "$TMP_DIR/explicit-runtime.a" "$TMP_DIR/valid.exe" <<'PY'
import ctypes
import os
import sys

ffi_path, obj_path, archive_path, exe_path = sys.argv[1:]
lib = ctypes.CDLL(ffi_path)
libc = ctypes.CDLL(None)
libc.getenv.argtypes = [ctypes.c_char_p]
libc.getenv.restype = ctypes.c_char_p

def env_value():
    value = libc.getenv(b"HAKO_AOT_USE_FFI")
    return None if value is None else value

link_fn = lib.hako_llvmc_link_obj_v2
link_fn.argtypes = [
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.POINTER(ctypes.c_void_p),
]
link_fn.restype = ctypes.c_int
error = ctypes.c_void_p()
before = env_value()
result = link_fn(
    obj_path.encode(),
    exe_path.encode(),
    archive_path.encode(),
    None,
    ctypes.byref(error),
)
message = ctypes.cast(error, ctypes.c_char_p).value.decode() if error.value else ""
if error.value:
    ctypes.CDLL(None).free(error)
if result != 0 or not os.path.exists(exe_path):
    raise SystemExit(f"explicit archive link failed: rc={result}: {message}")
if env_value() != before:
    raise SystemExit(f"empty HAKO_AOT_USE_FFI changed: before={before!r} after={env_value()!r}")
PY

HAKO_AOT_USE_FFI=1 NYASH_EMIT_EXE_NYRT="$TMP_DIR" \
python3 - "$FFI" "$TMP_DIR/input.o" "$TMP_DIR/v1.exe" <<'PY'
import ctypes
import os
import sys

ffi_path, obj_path, exe_path = sys.argv[1:]
lib = ctypes.CDLL(ffi_path)
libc = ctypes.CDLL(None)
libc.getenv.argtypes = [ctypes.c_char_p]
libc.getenv.restype = ctypes.c_char_p
link_fn = lib.hako_llvmc_link_obj
link_fn.argtypes = [
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.POINTER(ctypes.c_void_p),
]
link_fn.restype = ctypes.c_int
error = ctypes.c_void_p()
before = libc.getenv(b"HAKO_AOT_USE_FFI")
result = link_fn(obj_path.encode(), exe_path.encode(), None, ctypes.byref(error))
message = ctypes.cast(error, ctypes.c_char_p).value.decode() if error.value else ""
if error.value:
    libc.free(error)
if result != 0 or not os.path.exists(exe_path):
    raise SystemExit(f"v1 compatibility link failed: rc={result}: {message}")
if libc.getenv(b"HAKO_AOT_USE_FFI") != before:
    raise SystemExit("present HAKO_AOT_USE_FFI changed across v1 link")
PY

python3 - "$FFI" "$TMP_DIR/input.o" "$TMP_DIR/missing.a" "$TMP_DIR/invalid.exe" <<'PY'
import ctypes
import os
import sys

ffi_path, obj_path, archive_path, exe_path = sys.argv[1:]
lib = ctypes.CDLL(ffi_path)
link_fn = lib.hako_llvmc_link_obj_v2
link_fn.argtypes = [
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.POINTER(ctypes.c_void_p),
]
link_fn.restype = ctypes.c_int
error = ctypes.c_void_p()
result = link_fn(
    obj_path.encode(),
    exe_path.encode(),
    archive_path.encode(),
    None,
    ctypes.byref(error),
)
message = ctypes.cast(error, ctypes.c_char_p).value.decode() if error.value else ""
if error.value:
    ctypes.CDLL(None).free(error)
if result == 0 or os.path.exists(exe_path) or "explicit runtime archive" not in message:
    raise SystemExit(f"missing archive was not rejected: rc={result}: {message}")
PY

python3 - "$FFI" "$TMP_DIR/input.o" "$TMP_DIR/explicit-runtime.a" "$TMP_DIR/link-failure.exe" <<'PY'
import ctypes
import os
import sys

ffi_path, obj_path, archive_path, exe_path = sys.argv[1:]
lib = ctypes.CDLL(ffi_path)
link_fn = lib.hako_llvmc_link_obj_v2
link_fn.argtypes = [
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.POINTER(ctypes.c_void_p),
]
link_fn.restype = ctypes.c_int
error = ctypes.c_void_p()
result = link_fn(
    obj_path.encode(),
    exe_path.encode(),
    archive_path.encode(),
    b"-Wl,--hako-link-option-does-not-exist",
    ctypes.byref(error),
)
if error.value:
    ctypes.CDLL(None).free(error)
if result == 0 or os.path.exists(exe_path):
    raise SystemExit("linker failure was not rejected before executable publication")
PY

echo "[$TAG] ok"
