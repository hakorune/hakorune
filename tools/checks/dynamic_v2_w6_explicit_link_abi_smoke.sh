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

NYASH_EMIT_EXE_NYRT="$TMP_DIR/does-not-exist" \
python3 - "$FFI" "$TMP_DIR/input.o" "$TMP_DIR/explicit-runtime.a" "$TMP_DIR/valid.exe" <<'PY'
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
if result != 0 or not os.path.exists(exe_path):
    raise SystemExit(f"explicit archive link failed: rc={result}: {message}")
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

echo "[$TAG] ok"
