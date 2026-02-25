#!/bin/bash
# phase21_5_perf_compare_expr_cse_contract_vm.sh
#
# Contract pin:
# - compare expr-cache counters are observable on a dedicated repeated-compare bench.
# - compare_expr_cache_hit must be > 0 in FAST lane on llvmlite backend.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_compare_expr_cse_contract_vm"
KEY="compare_reuse_small"
BACKEND="${PERF_COMPARE_EXPR_CSE_BACKEND:-llvmlite}"

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/perf_hot_trace_contract.sh"
require_env || exit 2

BENCH_COMPARE="$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh"
TRACE_PY="$NYASH_ROOT/src/llvm_py/trace.py"
BENCH_HAKO="$NYASH_ROOT/benchmarks/bench_${KEY}.hako"
BENCH_C="$NYASH_ROOT/benchmarks/c/bench_${KEY}.c"

for f in "$BENCH_COMPARE" "$TRACE_PY" "$BENCH_HAKO" "$BENCH_C"; do
  perf_hot_trace_require_file "$SMOKE_NAME" "$f" || exit 2
done
perf_hot_trace_require_llvmlite_backend "$SMOKE_NAME" "PERF_COMPARE_EXPR_CSE_BACKEND" "$BACKEND" || exit 2

TRACE_FILE="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.trace.log")"
cleanup() {
  rm -f "$TRACE_FILE" >/dev/null 2>&1 || true
}
trap cleanup EXIT

set +e
OUT=$(NYASH_LLVM_FAST=1 \
  NYASH_LLVM_BACKEND="$BACKEND" \
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

for field in compare_total compare_expr_cache_hit compare_expr_cache_miss; do
  perf_hot_trace_require_numeric_field "$SMOKE_NAME" "$HOT_LINE" "$field" || exit 1
done

COMPARE_TOTAL="$(perf_hot_trace_extract_field "$HOT_LINE" "compare_total")"
COMPARE_HIT="$(perf_hot_trace_extract_field "$HOT_LINE" "compare_expr_cache_hit")"
COMPARE_MISS="$(perf_hot_trace_extract_field "$HOT_LINE" "compare_expr_cache_miss")"

if [ "$COMPARE_TOTAL" -le 0 ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: compare_total must be > 0"
  exit 1
fi
if [ "$COMPARE_HIT" -le 0 ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: compare_expr_cache_hit must be > 0"
  exit 1
fi
if [ "$COMPARE_MISS" -le 0 ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: compare_expr_cache_miss must be > 0"
  exit 1
fi

test_pass "$SMOKE_NAME"
