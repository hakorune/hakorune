#!/usr/bin/env bash
# Purpose: canonical non-alias compat replay probe. This intentionally clears
# `HAKO_CAPI_PURE` and uses the pure-first export plus explicit
# `HAKO_BACKEND_COMPAT_REPLAY=harness`.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env >/dev/null || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
FFI_LIB="$NYASH_ROOT/target/release/libhako_llvmc_ffi.so"
OUT_FAIL="${TMPDIR:-/tmp}/phase29ck_boundary_explicit_compat_probe_fail_$$.o"
OUT_PASS="${TMPDIR:-/tmp}/phase29ck_boundary_explicit_compat_probe_pass_$$.o"
LOG_FAIL="${TMPDIR:-/tmp}/phase29ck_boundary_explicit_compat_probe_fail_$$.log"
LOG_PASS="${TMPDIR:-/tmp}/phase29ck_boundary_explicit_compat_probe_pass_$$.log"

cleanup() {
  rm -f "$OUT_FAIL" "$OUT_PASS" "$LOG_FAIL" "$LOG_PASS"
}
trap cleanup EXIT

if [ ! -f "$FIXTURE" ]; then
  echo "[FAIL] phase29ck_boundary_explicit_compat_probe: fixture missing: $FIXTURE" >&2
  exit 1
fi

bash "$NYASH_ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release -q -p nyash-llvm-compiler --bin ny-llvmc >/dev/null

if [ ! -x "$NY_LLVM_C" ]; then
  echo "[FAIL] phase29ck_boundary_explicit_compat_probe: ny-llvmc missing: $NY_LLVM_C" >&2
  exit 1
fi

if [ ! -f "$FFI_LIB" ]; then
  echo "[FAIL] phase29ck_boundary_explicit_compat_probe: ffi lib missing: $FFI_LIB" >&2
  exit 1
fi

run_probe_case() {
  local compat_replay="$1"
  local out_obj="$2"
  local log_path="$3"
  local expected_mode="$4"
  local rc

  set +e
  timeout 90 python3 - "$FFI_LIB" "$FIXTURE" "$out_obj" "$NY_LLVM_C" "$compat_replay" <<'PY' >"$log_path" 2>&1
import ctypes
import os
import sys

ffi_lib, fixture, out_obj, ny_llvmc, compat_replay = sys.argv[1:]
lib = ctypes.CDLL(ffi_lib)
fn = lib.hako_llvmc_compile_json_pure_first
fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
fn.restype = ctypes.c_int

os.environ.pop("HAKO_CAPI_PURE", None)
os.environ.pop("HAKO_BACKEND_COMPILE_RECIPE", None)
os.environ.pop("HAKO_BACKEND_COMPAT_REPLAY", None)
os.environ["NYASH_NY_LLVM_COMPILER"] = ny_llvmc
if compat_replay:
    os.environ["HAKO_BACKEND_COMPAT_REPLAY"] = compat_replay

err = ctypes.c_void_p()
rc = fn(fixture.encode(), out_obj.encode(), ctypes.byref(err))
if err.value:
    msg = ctypes.cast(err, ctypes.c_char_p).value.decode()
    print(msg)
sys.exit(rc)
PY
  rc=$?
  set -e

  if [ "$expected_mode" = "fail" ]; then
    if [ "$rc" -eq 0 ]; then
      echo "[FAIL] phase29ck_boundary_explicit_compat_probe: unsupported pure-first export succeeded without explicit compat replay" >&2
      cat "$log_path" >&2
      exit 1
    fi
    if [ -f "$out_obj" ]; then
      echo "[FAIL] phase29ck_boundary_explicit_compat_probe: unexpected object on fail-fast path: $out_obj" >&2
      exit 1
    fi
    if ! grep -Fq "unsupported pure shape for current backend recipe" "$log_path"; then
      echo "[FAIL] phase29ck_boundary_explicit_compat_probe: missing fail-fast tag in $log_path" >&2
      cat "$log_path" >&2
      exit 1
    fi
    return
  fi

  if [ "$rc" -ne 0 ]; then
    echo "[FAIL] phase29ck_boundary_explicit_compat_probe: explicit compat replay failed (rc=$rc)" >&2
    cat "$log_path" >&2
    exit 1
  fi
  if [ ! -f "$out_obj" ]; then
    echo "[FAIL] phase29ck_boundary_explicit_compat_probe: object missing on explicit compat path: $out_obj" >&2
    exit 1
  fi
}

run_probe_case "" "$OUT_FAIL" "$LOG_FAIL" fail
run_probe_case "harness" "$OUT_PASS" "$LOG_PASS" pass

echo "[PASS] phase29ck_boundary_explicit_compat_probe"
