#!/bin/bash
# Phase 29ck boundary forwarder compile canary
#
# Contract pin:
# 1) default `hako_llvmc_compile_json` export surface still reads as a thin
#    forwarder when `HAKO_CAPI_PURE` is unset.
# 2) that forwarder now reaches `ny-llvmc --driver boundary`, not
#    `ny-llvmc --driver harness`.
# 3) supported `ret_const_min_v1.mir.json` still emits an object through that
#    default forwarder path.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ] && ! can_run_llvm; then
    test_skip "phase29ck_boundary_forwarder_min: LLVM backend not available"
    exit 0
fi

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase29ck_boundary_forwarder_min: llc not found"
    exit 0
fi

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/ret_const_min_v1.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
FFI_LIB="$NYASH_ROOT/target/release/libhako_llvmc_ffi.so"
OUT_OBJ="${TMPDIR:-/tmp}/phase29ck_boundary_forwarder_min_$$.o"
BUILD_LOG="${TMPDIR:-/tmp}/phase29ck_boundary_forwarder_min_$$.log"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"

cleanup() {
    rm -f "$OUT_OBJ" "$BUILD_LOG"
}
trap cleanup EXIT

if [ ! -f "$FIXTURE" ]; then
    test_fail "phase29ck_boundary_forwarder_min: fixture missing: $FIXTURE"
    exit 1
fi

bash "$NYASH_ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release -q -p nyash-llvm-compiler --bin ny-llvmc >/dev/null

if [ ! -x "$NY_LLVM_C" ]; then
    test_fail "phase29ck_boundary_forwarder_min: ny-llvmc missing: $NY_LLVM_C"
    exit 1
fi

if [ ! -f "$FFI_LIB" ]; then
    test_fail "phase29ck_boundary_forwarder_min: ffi lib missing: $FFI_LIB"
    exit 1
fi

set +e
BUILD_OUT=$(
  timeout "$RUN_TIMEOUT_SECS" python3 - "$FFI_LIB" "$FIXTURE" "$OUT_OBJ" "$NY_LLVM_C" <<'PY'
import ctypes
import os
import sys

ffi_lib, fixture, out_obj, ny_llvmc = sys.argv[1:]
lib = ctypes.CDLL(ffi_lib)
fn = lib.hako_llvmc_compile_json
fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
fn.restype = ctypes.c_int

os.environ.pop("HAKO_CAPI_PURE", None)
os.environ["NYASH_NY_LLVM_COMPILER"] = ny_llvmc

err = ctypes.c_void_p()
rc = fn(fixture.encode(), out_obj.encode(), ctypes.byref(err))
if rc != 0:
    if err.value:
      msg = ctypes.cast(err, ctypes.c_char_p).value.decode()
      print(msg)
    sys.exit(rc)
PY
  2>&1
)
BUILD_RC=$?
set -e

echo "$BUILD_OUT" >"$BUILD_LOG"

if [ "$BUILD_RC" -eq 124 ]; then
    test_fail "phase29ck_boundary_forwarder_min: compile timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$BUILD_RC" -ne 0 ]; then
    echo "[INFO] compile output:"
    tail -n 120 "$BUILD_LOG" || true
    test_fail "phase29ck_boundary_forwarder_min: default forwarder compile failed (rc=$BUILD_RC)"
    exit 1
fi

if [ ! -f "$OUT_OBJ" ]; then
    test_fail "phase29ck_boundary_forwarder_min: object missing: $OUT_OBJ"
    exit 1
fi

test_pass "phase29ck_boundary_forwarder_min: PASS (default hako_llvmc forwarder reaches boundary command path)"
