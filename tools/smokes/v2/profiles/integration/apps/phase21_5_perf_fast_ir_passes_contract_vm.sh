#!/bin/bash
# phase21_5_perf_fast_ir_passes_contract_vm.sh
#
# Contract pin:
# - NYASH_LLVM_FAST_IR_PASSES is wired in llvm_builder FAST lane.
# - Both ON/OFF modes compile+run AOT successfully (status=ok) under FAST lane.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_fast_ir_passes_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

LLVM_BUILDER="$NYASH_ROOT/src/llvm_py/llvm_builder.py"
BENCH_COMPARE="$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh"
KEY="method_call_only_small"

for f in "$LLVM_BUILDER" "$BENCH_COMPARE"; do
  if [ ! -f "$f" ]; then
    test_fail "$SMOKE_NAME: missing file: $f"
    exit 2
  fi
done

# Static contract: FAST IR pass gate exists and defaults to ON in FAST mode.
if ! grep -q "os.environ.get('NYASH_LLVM_FAST') == '1'" "$LLVM_BUILDER"; then
  test_fail "$SMOKE_NAME: FAST lane gate not found in llvm_builder"
  exit 1
fi
if ! grep -q "NYASH_LLVM_FAST_IR_PASSES', '1'" "$LLVM_BUILDER"; then
  test_fail "$SMOKE_NAME: FAST_IR_PASSES default-on gate not found in llvm_builder"
  exit 1
fi
if ! grep -q "create_pass_manager_builder" "$LLVM_BUILDER"; then
  test_fail "$SMOKE_NAME: module pass manager builder not found in llvm_builder"
  exit 1
fi

run_case() {
  local mode="$1"
  local out=""
  set +e
  out=$(NYASH_LLVM_FAST=1 \
    NYASH_LLVM_FAST_IR_PASSES="$mode" \
    PERF_AOT=1 \
    PERF_SKIP_VM_PREFLIGHT=1 \
    NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
    bash "$BENCH_COMPARE" "$KEY" 1 1 2>&1)
  local rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    printf '%s\n' "$out"
    test_fail "$SMOKE_NAME: bench_compare failed for NYASH_LLVM_FAST_IR_PASSES=$mode (rc=$rc)"
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
