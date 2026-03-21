#!/bin/bash
# phase21_5_perf_numeric_hot_trace_contract_vm.sh
#
# Contract pin:
# - NYASH_LLVM_HOT_TRACE=1 emits one-line [llvm/hot] summary for numeric_mixed_medium.
# - Summary exposes %/compare and resolve fallback counters in stable numeric fields.
# - AOT benchmark still succeeds (status=ok) under FAST lane.
# - Contract is pinned on llvmlite backend (trace source: src/llvm_py/*).
# - Field schema SSOT is src/llvm_py/trace.py::HOT_SUMMARY_FIELDS.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_numeric_hot_trace_contract_vm"
KEY="numeric_mixed_medium"
BACKEND="${PERF_NUMERIC_HOT_TRACE_BACKEND:-crate}"
MAX_FALLBACK_BINOP="${PERF_NUMERIC_HOT_TRACE_MAX_FALLBACK_BINOP:-0}"
MAX_FALLBACK_COMPARE="${PERF_NUMERIC_HOT_TRACE_MAX_FALLBACK_COMPARE:-0}"

source "$(dirname "$0")/../../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../../lib/perf_hot_trace_contract.sh"
require_env || exit 2

BENCH_COMPARE="$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh"
TRACE_PY="$NYASH_ROOT/src/llvm_py/trace.py"
perf_hot_trace_require_file "$SMOKE_NAME" "$BENCH_COMPARE" || exit 2
perf_hot_trace_require_file "$SMOKE_NAME" "$TRACE_PY" || exit 2
perf_hot_trace_require_boundary_backend "$SMOKE_NAME" "PERF_NUMERIC_HOT_TRACE_BACKEND" "$BACKEND" || exit 2
perf_hot_trace_require_uint_env "$SMOKE_NAME" "PERF_NUMERIC_HOT_TRACE_MAX_FALLBACK_BINOP" "$MAX_FALLBACK_BINOP" || exit 2
perf_hot_trace_require_uint_env "$SMOKE_NAME" "PERF_NUMERIC_HOT_TRACE_MAX_FALLBACK_COMPARE" "$MAX_FALLBACK_COMPARE" || exit 2

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

HOT_FIELDS="$(perf_hot_trace_load_fields "$TRACE_PY")"
if [ -z "$HOT_FIELDS" ]; then
  test_fail "$SMOKE_NAME: failed to load HOT_SUMMARY_FIELDS from trace.py"
  exit 1
fi

for field in $HOT_FIELDS; do
  perf_hot_trace_require_numeric_field "$SMOKE_NAME" "$HOT_LINE" "$field" || exit 1
done

BINOP_MOD="$(perf_hot_trace_extract_field "$HOT_LINE" "binop_mod")"
COMPARE_TOTAL="$(perf_hot_trace_extract_field "$HOT_LINE" "compare_total")"
COMPARE_KEEP_I1="$(perf_hot_trace_extract_field "$HOT_LINE" "compare_keep_i1")"
COMPARE_TO_I64="$(perf_hot_trace_extract_field "$HOT_LINE" "compare_to_i64")"
FALLBACK_BINOP="$(perf_hot_trace_extract_field "$HOT_LINE" "resolve_fallback_binop")"
FALLBACK_COMPARE="$(perf_hot_trace_extract_field "$HOT_LINE" "resolve_fallback_compare")"

if [ "$BINOP_MOD" -le 0 ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: binop_mod must be > 0 for numeric loop"
  exit 1
fi
if [ "$COMPARE_TOTAL" -le 0 ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: compare_total must be > 0 for numeric loop"
  exit 1
fi
if [ "$COMPARE_KEEP_I1" -le 0 ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: compare_keep_i1 must be > 0 in FAST branch-only path"
  exit 1
fi
if [ "$COMPARE_TO_I64" -ne 0 ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: compare_to_i64 expected 0 for branch-only compare chain"
  exit 1
fi
if [ "$FALLBACK_BINOP" -gt "$MAX_FALLBACK_BINOP" ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: resolve_fallback_binop=${FALLBACK_BINOP} exceeds max=${MAX_FALLBACK_BINOP}"
  exit 1
fi
if [ "$FALLBACK_COMPARE" -gt "$MAX_FALLBACK_COMPARE" ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: resolve_fallback_compare=${FALLBACK_COMPARE} exceeds max=${MAX_FALLBACK_COMPARE}"
  exit 1
fi

test_pass "$SMOKE_NAME"
