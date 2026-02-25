#!/bin/bash
# phase21_5_perf_compare_reuse_aot_ceiling_contract_vm.sh
#
# Contract pin:
# - compare_reuse_small (aot) keeps status=ok under FAST lane.
# - ny_aot_ms stays under a conservative ceiling (default: 20ms).

set -euo pipefail

SMOKE_NAME="phase21_5_perf_compare_reuse_aot_ceiling_contract_vm"
KEY="compare_reuse_small"
MAX_AOT_MS="${PERF_COMPARE_REUSE_AOT_MAX_MS:-20}"

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/perf_hot_trace_contract.sh"
require_env || exit 2

BENCH_COMPARE="$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh"
BENCH_HAKO="$NYASH_ROOT/benchmarks/bench_${KEY}.hako"
BENCH_C="$NYASH_ROOT/benchmarks/c/bench_${KEY}.c"

for f in "$BENCH_COMPARE" "$BENCH_HAKO" "$BENCH_C"; do
  perf_hot_trace_require_file "$SMOKE_NAME" "$f" || exit 2
done
perf_hot_trace_require_uint_env "$SMOKE_NAME" "PERF_COMPARE_REUSE_AOT_MAX_MS" "$MAX_AOT_MS" || exit 2

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
