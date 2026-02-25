#!/bin/bash
# phase21_5_perf_opt_level_contract_vm.sh
#
# Contract pin:
# - NYASH_LLVM_OPT_LEVEL=2/3 are accepted in FAST lane AOT flow.
# - Both levels compile+run AOT successfully (status=ok) on numeric_mixed_medium.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_opt_level_contract_vm"
KEY="numeric_mixed_medium"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

BUILD_OPTS="$NYASH_ROOT/src/llvm_py/build_opts.py"
BENCH_COMPARE="$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh"

for f in "$BUILD_OPTS" "$BENCH_COMPARE"; do
  if [ ! -f "$f" ]; then
    test_fail "$SMOKE_NAME: missing file: $f"
    exit 2
  fi
done

# Static contract: opt-level env keys are wired in one place.
if ! grep -q "_OPT_ENV_KEYS" "$BUILD_OPTS"; then
  test_fail "$SMOKE_NAME: _OPT_ENV_KEYS not found in build_opts"
  exit 1
fi
if ! grep -q "NYASH_LLVM_OPT_LEVEL" "$BUILD_OPTS"; then
  test_fail "$SMOKE_NAME: NYASH_LLVM_OPT_LEVEL wiring not found in build_opts"
  exit 1
fi
if ! grep -q "HAKO_LLVM_OPT_LEVEL" "$BUILD_OPTS"; then
  test_fail "$SMOKE_NAME: HAKO_LLVM_OPT_LEVEL compatibility wiring not found in build_opts"
  exit 1
fi

run_case() {
  local level="$1"
  local out=""
  set +e
  out=$(NYASH_LLVM_FAST=1 \
    NYASH_LLVM_OPT_LEVEL="$level" \
    PERF_AOT=1 \
    PERF_SKIP_VM_PREFLIGHT=1 \
    NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
    bash "$BENCH_COMPARE" "$KEY" 1 1 2>&1)
  local rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    printf '%s\n' "$out"
    test_fail "$SMOKE_NAME: bench_compare failed for NYASH_LLVM_OPT_LEVEL=$level (rc=$rc)"
    exit 1
  fi
  if ! printf '%s\n' "$out" | grep -q "\\[bench\\] name=${KEY} (aot)"; then
    printf '%s\n' "$out"
    test_fail "$SMOKE_NAME: missing AOT bench line for opt-level=$level"
    exit 1
  fi
  if ! printf '%s\n' "$out" | grep -q "\\[bench\\] name=${KEY} (aot).*status=ok"; then
    printf '%s\n' "$out"
    test_fail "$SMOKE_NAME: AOT status not ok for opt-level=$level"
    exit 1
  fi
}

run_case 2
run_case 3

test_pass "$SMOKE_NAME"
