#!/bin/bash
# phase21_5_perf_fast_native_codegen_contract_vm.sh
#
# Contract pin:
# - NYASH_LLVM_FAST_NATIVE toggles host CPU/feature target tuning in FAST lane.
# - Both ON/OFF modes compile+run AOT successfully (status=ok) under FAST lane.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_fast_native_codegen_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

BUILD_OPTS="$NYASH_ROOT/src/llvm_py/build_opts.py"
BENCH_COMPARE="$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh"
KEY="numeric_mixed_medium"

for f in "$BUILD_OPTS" "$BENCH_COMPARE"; do
  if [ ! -f "$f" ]; then
    test_fail "$SMOKE_NAME: missing file: $f"
    exit 2
  fi
done

# Static contract: FAST native env exists and defaults to ON in FAST lane.
if ! grep -q "_FAST_NATIVE_ENV_KEYS" "$BUILD_OPTS"; then
  test_fail "$SMOKE_NAME: FAST_NATIVE env keys not found in build_opts"
  exit 1
fi
if ! grep -q "NYASH_LLVM_FAST_NATIVE" "$BUILD_OPTS"; then
  test_fail "$SMOKE_NAME: NYASH_LLVM_FAST_NATIVE wiring not found in build_opts"
  exit 1
fi
if ! grep -q "if os.environ.get(\"NYASH_LLVM_FAST\") != \"1\"" "$BUILD_OPTS"; then
  test_fail "$SMOKE_NAME: FAST lane guard not found in build_opts"
  exit 1
fi

run_case() {
  local mode="$1"
  local out=""
  set +e
  out=$(NYASH_LLVM_FAST=1 \
    NYASH_LLVM_FAST_NATIVE="$mode" \
    PERF_AOT=1 \
    PERF_SKIP_VM_PREFLIGHT=1 \
    NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
    bash "$BENCH_COMPARE" "$KEY" 1 1 2>&1)
  local rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    printf '%s\n' "$out"
    test_fail "$SMOKE_NAME: bench_compare failed for NYASH_LLVM_FAST_NATIVE=$mode (rc=$rc)"
    exit 1
  fi

  if ! printf '%s\n' "$out" | grep -q "\\[bench\\] name=${KEY} (aot)"; then
    printf '%s\n' "$out"
    test_fail "$SMOKE_NAME: missing AOT bench line for mode=$mode"
    exit 1
  fi
  if ! printf '%s\n' "$out" | grep -q "\\[bench\\] name=${KEY} (aot).*status=ok"; then
    printf '%s\n' "$out"
    test_fail "$SMOKE_NAME: AOT status not ok for mode=$mode"
    exit 1
  fi
}

run_case 1
run_case 0

test_pass "$SMOKE_NAME"
