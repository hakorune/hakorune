#!/usr/bin/env bash
# Purpose: the single active behavior probe for the historical
# `HAKO_CAPI_PURE=1` alias. Daily/active pure-keep callers must use
# `HAKO_BACKEND_COMPILE_RECIPE=pure-first` instead.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env >/dev/null || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/ret_const_min_v1.mir.json"
NY_LLVM_C="$NYASH_ROOT/target/release/ny-llvmc"
FFI_LIB="$NYASH_ROOT/target/release/libhako_llvmc_ffi.so"
OUT_ALIAS="${TMPDIR:-/tmp}/phase29ck_boundary_historical_alias_probe_alias_$$.o"
OUT_RECIPE="${TMPDIR:-/tmp}/phase29ck_boundary_historical_alias_probe_recipe_$$.o"
LOG_ALIAS="${TMPDIR:-/tmp}/phase29ck_boundary_historical_alias_probe_alias_$$.log"
LOG_RECIPE="${TMPDIR:-/tmp}/phase29ck_boundary_historical_alias_probe_recipe_$$.log"

cleanup() {
  rm -f "$OUT_ALIAS" "$OUT_RECIPE" "$LOG_ALIAS" "$LOG_RECIPE"
}
trap cleanup EXIT

if [ ! -f "$FIXTURE" ]; then
  echo "[FAIL] phase29ck_boundary_historical_alias_probe: fixture missing: $FIXTURE" >&2
  exit 1
fi

bash "$NYASH_ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null
cargo build --release -q -p nyash-llvm-compiler --bin ny-llvmc >/dev/null

if [ ! -x "$NY_LLVM_C" ]; then
  echo "[FAIL] phase29ck_boundary_historical_alias_probe: ny-llvmc missing: $NY_LLVM_C" >&2
  exit 1
fi

if [ ! -f "$FFI_LIB" ]; then
  echo "[FAIL] phase29ck_boundary_historical_alias_probe: ffi lib missing: $FFI_LIB" >&2
  exit 1
fi

run_probe_case() {
  local compile_recipe="$1"
  local capi_pure="$2"
  local out_obj="$3"
  local log_path="$4"
  local expected_mode="$5"
  local rc

  set +e
  timeout 90 python3 - "$FFI_LIB" "$FIXTURE" "$out_obj" "$compile_recipe" "$capi_pure" <<'PY' >"$log_path" 2>&1
import ctypes
import os
import sys

ffi_lib, fixture, out_obj, compile_recipe, capi_pure = sys.argv[1:]
lib = ctypes.CDLL(ffi_lib)
fn = lib.hako_llvmc_compile_json
fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
fn.restype = ctypes.c_int

os.environ.pop("NYASH_NY_LLVM_COMPILER", None)
os.environ.pop("HAKO_BACKEND_COMPILE_RECIPE", None)
os.environ.pop("HAKO_BACKEND_COMPAT_REPLAY", None)
os.environ.pop("HAKO_CAPI_PURE", None)
os.environ["NYASH_NY_LLVM_COMPILER"] = "/__missing__/ny-llvmc"
if compile_recipe:
    os.environ["HAKO_BACKEND_COMPILE_RECIPE"] = compile_recipe
if capi_pure:
    os.environ["HAKO_CAPI_PURE"] = capi_pure

err = ctypes.c_void_p()
rc = fn(fixture.encode(), out_obj.encode(), ctypes.byref(err))
if err.value:
    msg = ctypes.cast(err, ctypes.c_char_p).value.decode()
    print(msg)
sys.exit(rc)
PY
  rc=$?
  set -e

  if [ "$expected_mode" = "pass" ]; then
    if [ "$rc" -ne 0 ]; then
      echo "[FAIL] phase29ck_boundary_historical_alias_probe: historical alias did not keep generic export green (rc=$rc)" >&2
      cat "$log_path" >&2
      exit 1
    fi
    if [ ! -f "$out_obj" ]; then
      echo "[FAIL] phase29ck_boundary_historical_alias_probe: object missing on historical alias path: $out_obj" >&2
      exit 1
    fi
    if ! grep -Fq "[deprecate/env] 'HAKO_CAPI_PURE' is deprecated; use 'HAKO_BACKEND_COMPILE_RECIPE=pure-first'" "$log_path"; then
      echo "[FAIL] phase29ck_boundary_historical_alias_probe: missing historical alias deprecation warning in $log_path" >&2
      cat "$log_path" >&2
      exit 1
    fi
    return
  fi

  if [ "$rc" -eq 0 ]; then
    echo "[FAIL] phase29ck_boundary_historical_alias_probe: historical alias overrode explicit harness recipe" >&2
    cat "$log_path" >&2
    exit 1
  fi
  if [ -f "$out_obj" ]; then
    echo "[FAIL] phase29ck_boundary_historical_alias_probe: unexpected object when harness recipe should win: $out_obj" >&2
    exit 1
  fi
}

run_probe_case "" "1" "$OUT_ALIAS" "$LOG_ALIAS" pass
run_probe_case "harness" "1" "$OUT_RECIPE" "$LOG_RECIPE" fail

echo "[PASS] phase29ck_boundary_historical_alias_probe"
