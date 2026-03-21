#!/bin/bash
# phase21_5_perf_numeric_arith_cse_contract_vm.sh
#
# Contract pin:
# - numeric_mixed_medium (aot) arithmetic chain enables expr-cache reuse in FAST lane.
# - Hot summary exposes binop/compare expr-cache counters with stable numeric fields.
# - AOT benchmark still succeeds (status=ok).

set -euo pipefail

SMOKE_NAME="phase21_5_perf_numeric_arith_cse_contract_vm"
KEY="numeric_mixed_medium"
BACKEND="${PERF_NUMERIC_ARITH_CSE_BACKEND:-crate}"

source "$(dirname "$0")/../../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../../lib/perf_hot_trace_contract.sh"
require_env || exit 2

BENCH_COMPARE="$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh"
TRACE_PY="$NYASH_ROOT/src/llvm_py/trace.py"
perf_hot_trace_require_file "$SMOKE_NAME" "$BENCH_COMPARE" || exit 2
perf_hot_trace_require_file "$SMOKE_NAME" "$TRACE_PY" || exit 2
perf_hot_trace_require_boundary_backend "$SMOKE_NAME" "PERF_NUMERIC_ARITH_CSE_BACKEND" "$BACKEND" || exit 2

TRACE_FILE="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.trace.log")"
cleanup() {
  rm -f "$TRACE_FILE" >/dev/null 2>&1 || true
}
trap cleanup EXIT

set +e
OUT=$(NYASH_LLVM_FAST=1 \
  NYASH_LLVM_BACKEND="$BACKEND" \
  NYASH_LLVM_USE_HARNESS=0 \
  NYASH_LLVM_HOT_TRACE=1 \
  NYASH_LLVM_TRACE_OUT="$TRACE_FILE" \
  NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  PERF_AOT=1 \
  PERF_SKIP_VM_PREFLIGHT=1 \
  bash "$BENCH_COMPARE" "$KEY" 1 1 2>&1)
RC=$?
set -e
if [ "$RC" -ne 0 ]; then
  printf '%s\n' "$OUT"
  test_fail "$SMOKE_NAME: bench_compare failed rc=$RC"
  exit 1
fi

perf_hot_trace_assert_aot_ok "$SMOKE_NAME" "$KEY" "$OUT" || exit 1

HOT_LINE="$(perf_hot_trace_find_main_hot_line "$TRACE_FILE")"
if [ -z "$HOT_LINE" ]; then
  if [ -s "$TRACE_FILE" ]; then
    sed -n '1,60p' "$TRACE_FILE"
  fi
  printf '%s\n' "$OUT"
  test_fail "$SMOKE_NAME: missing [llvm/hot] fn=main line"
  exit 1
fi

for field in binop_expr_cache_hit binop_expr_cache_miss compare_expr_cache_hit compare_expr_cache_miss; do
  perf_hot_trace_require_numeric_field "$SMOKE_NAME" "$HOT_LINE" "$field" || exit 1
done

BINOP_HIT="$(perf_hot_trace_extract_field "$HOT_LINE" "binop_expr_cache_hit")"
BINOP_MISS="$(perf_hot_trace_extract_field "$HOT_LINE" "binop_expr_cache_miss")"
if [ "$BINOP_HIT" -le 0 ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: binop_expr_cache_hit must be > 0"
  exit 1
fi
if [ "$BINOP_MISS" -le 0 ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: binop_expr_cache_miss must be > 0"
  exit 1
fi

test_pass "$SMOKE_NAME"
