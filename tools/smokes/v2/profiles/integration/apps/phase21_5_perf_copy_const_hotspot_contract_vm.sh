#!/bin/bash
# phase21_5_perf_copy_const_hotspot_contract_vm.sh
#
# Contract pin:
# - copy/const fast-path wiring exists in LLVM Python backend FAST lane.
# - numeric_mixed_medium AOT remains status=ok with the wiring enabled.
# - ny_aot_ms stays under a conservative ceiling (default: 20ms).

set -euo pipefail

SMOKE_NAME="phase21_5_perf_copy_const_hotspot_contract_vm"
KEY="numeric_mixed_medium"
MAX_AOT_MS="${PERF_COPY_CONST_HOTSPOT_MAX_AOT_MS:-20}"

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/perf_hot_trace_contract.sh"
require_env || exit 2

COPY_PY="$NYASH_ROOT/src/llvm_py/instructions/copy.py"
CONST_PY="$NYASH_ROOT/src/llvm_py/instructions/const.py"
BENCH_COMPARE="$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh"

perf_hot_trace_require_file "$SMOKE_NAME" "$COPY_PY" || exit 2
perf_hot_trace_require_file "$SMOKE_NAME" "$CONST_PY" || exit 2
perf_hot_trace_require_file "$SMOKE_NAME" "$BENCH_COMPARE" || exit 2
perf_hot_trace_require_uint_env "$SMOKE_NAME" "PERF_COPY_CONST_HOTSPOT_MAX_AOT_MS" "$MAX_AOT_MS" || exit 2

# Static contract: copy FAST lane shortcut + const i64 cache exist.
if ! grep -q "os.environ.get('NYASH_LLVM_FAST') == '1'" "$COPY_PY"; then
  test_fail "$SMOKE_NAME: FAST copy lane gate not found in copy.py"
  exit 1
fi
if ! grep -q "_I64_CONST_CACHE" "$CONST_PY"; then
  test_fail "$SMOKE_NAME: i64 const cache not found in const.py"
  exit 1
fi

set +e
OUT=$(NYASH_LLVM_FAST=1 \
  PERF_AOT=1 \
  PERF_SKIP_VM_PREFLIGHT=1 \
  NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  bash "$BENCH_COMPARE" "$KEY" 1 3 2>&1)
RC=$?
set -e
if [ "$RC" -ne 0 ]; then
  printf '%s\n' "$OUT"
  test_fail "$SMOKE_NAME: bench_compare failed rc=$RC"
  exit 1
fi

perf_hot_trace_assert_aot_ceiling "$SMOKE_NAME" "$KEY" "$OUT" "$MAX_AOT_MS" || exit 1

test_pass "$SMOKE_NAME"
