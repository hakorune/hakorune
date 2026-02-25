#!/bin/bash
# phase21_5_perf_method_call_hot_trace_contract_vm.sh
#
# Contract pin:
# - NYASH_LLVM_HOT_TRACE=1 emits [llvm/hot] summary for method_call_only.
# - main summary includes call_total > 0 (call-path observation lock).
# - resolve_fallback_call stays under configurable ceiling (default 0).
# - Contract is pinned on llvmlite backend (trace source: src/llvm_py/*).
# - Field schema SSOT is src/llvm_py/trace.py::HOT_SUMMARY_FIELDS.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_method_call_hot_trace_contract_vm"
KEY="method_call_only"
BACKEND="${PERF_METHOD_CALL_HOT_TRACE_BACKEND:-llvmlite}"
MAX_FALLBACK_CALL="${PERF_METHOD_CALL_HOT_TRACE_MAX_FALLBACK_CALL:-0}"

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/perf_hot_trace_contract.sh"
require_env || exit 2

BENCH_COMPARE="$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh"
TRACE_PY="$NYASH_ROOT/src/llvm_py/trace.py"
perf_hot_trace_require_file "$SMOKE_NAME" "$BENCH_COMPARE" || exit 2
perf_hot_trace_require_file "$SMOKE_NAME" "$TRACE_PY" || exit 2
perf_hot_trace_require_llvmlite_backend "$SMOKE_NAME" "PERF_METHOD_CALL_HOT_TRACE_BACKEND" "$BACKEND" || exit 2
perf_hot_trace_require_uint_env "$SMOKE_NAME" "PERF_METHOD_CALL_HOT_TRACE_MAX_FALLBACK_CALL" "$MAX_FALLBACK_CALL" || exit 2

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

HOT_FIELDS="$(perf_hot_trace_load_fields "$TRACE_PY")"
if [ -z "$HOT_FIELDS" ]; then
  test_fail "$SMOKE_NAME: failed to load HOT_SUMMARY_FIELDS from trace.py"
  exit 1
fi

for field in $HOT_FIELDS; do
  perf_hot_trace_require_numeric_field "$SMOKE_NAME" "$HOT_LINE" "$field" || exit 1
done

CALL_TOTAL="$(perf_hot_trace_extract_field "$HOT_LINE" "call_total")"
FALLBACK_CALL="$(perf_hot_trace_extract_field "$HOT_LINE" "resolve_fallback_call")"

if [ "$CALL_TOTAL" -le 0 ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: call_total must be > 0 for method_call_only"
  exit 1
fi
if [ "$FALLBACK_CALL" -gt "$MAX_FALLBACK_CALL" ]; then
  printf '%s\n' "$HOT_LINE"
  test_fail "$SMOKE_NAME: resolve_fallback_call=${FALLBACK_CALL} exceeds max=${MAX_FALLBACK_CALL}"
  exit 1
fi

test_pass "$SMOKE_NAME"
