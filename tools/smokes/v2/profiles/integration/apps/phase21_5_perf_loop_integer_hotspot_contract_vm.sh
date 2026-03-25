#!/bin/bash
# phase21_5_perf_loop_integer_hotspot_contract_vm.sh
#
# Contract pin:
# - FAST AOT loop hotspot in perf benches must stay integer-only in main loop path.
# - main must not call safepoint / any.length_h in these benches.
# - method_call_only keeps fast length route (nyrt_string_length or const-fold add(5)).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_loop_integer_hotspot_contract_vm"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
EMIT_TIMEOUT_SECS="${EMIT_TIMEOUT_SECS:-30}"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"

if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "$SMOKE_NAME: emit route helper missing/executable: $EMIT_ROUTE"
  exit 2
fi
if [ ! -f "$MIR_BUILDER" ]; then
  test_fail "$SMOKE_NAME: MIR builder missing: $MIR_BUILDER"
  exit 2
fi

run_case() {
  local case_name="$1"
  local bench="$2"
  local tmp_mir tmp_ir tmp_exe tmp_log tmp_main
  tmp_mir="$(mktemp "/tmp/${SMOKE_NAME}.${case_name}.XXXXXX.mir.json")"
  tmp_ir="$(mktemp "/tmp/${SMOKE_NAME}.${case_name}.XXXXXX.ll")"
  tmp_exe="$(mktemp "/tmp/${SMOKE_NAME}.${case_name}.XXXXXX.exe")"
  tmp_log="$(mktemp "/tmp/${SMOKE_NAME}.${case_name}.XXXXXX.log")"
  tmp_main="$(mktemp "/tmp/${SMOKE_NAME}.${case_name}.XXXXXX.main.ll")"

  cleanup_case() {
    rm -f "$tmp_mir" "$tmp_ir" "$tmp_exe" "$tmp_log" "$tmp_main" >/dev/null 2>&1 || true
  }
  trap cleanup_case RETURN

  if [ ! -f "$bench" ]; then
    test_fail "$SMOKE_NAME($case_name): benchmark missing: $bench"
    return 1
  fi

  set +e
  "$EMIT_ROUTE" --route hako-helper --timeout-secs "$EMIT_TIMEOUT_SECS" --out "$tmp_mir" --input "$bench" >"$tmp_log" 2>&1
  local emit_rc=$?
  set -e
  if [ "$emit_rc" -ne 0 ]; then
    tail -n 60 "$tmp_log" || true
    test_fail "$SMOKE_NAME($case_name): MIR emit failed rc=$emit_rc"
    return 1
  fi

  set +e
  NYASH_LLVM_FAST=1 \
  NYASH_LLVM_DUMP_IR="$tmp_ir" \
  bash "$MIR_BUILDER" --in "$tmp_mir" --emit exe -o "$tmp_exe" --quiet >>"$tmp_log" 2>&1
  local build_rc=$?
  set -e
  if [ "$build_rc" -ne 0 ]; then
    tail -n 80 "$tmp_log" || true
    test_fail "$SMOKE_NAME($case_name): AOT build failed rc=$build_rc"
    return 1
  fi

  if [ ! -s "$tmp_ir" ]; then
    test_fail "$SMOKE_NAME($case_name): expected IR dump is empty"
    return 1
  fi

  if ! extract_ir_entry_function "$tmp_ir" "$tmp_main"; then
    test_fail "$SMOKE_NAME($case_name): entry function not found in dumped IR"
    return 1
  fi

  if grep -q 'ny_check_safepoint' "$tmp_main"; then
    test_fail "$SMOKE_NAME($case_name): main still contains ny_check_safepoint"
    return 1
  fi

  if grep -q 'nyash.any.length_h' "$tmp_main"; then
    test_fail "$SMOKE_NAME($case_name): main still contains nyash.any.length_h"
    return 1
  fi

  if ! grep -q 'icmp slt i64' "$tmp_main"; then
    test_fail "$SMOKE_NAME($case_name): main missing integer loop compare"
    return 1
  fi

  if ! grep -q 'add i64' "$tmp_main"; then
    test_fail "$SMOKE_NAME($case_name): main missing integer add in loop path"
    return 1
  fi

  if [ "$case_name" = "method_call_only" ]; then
    if ! grep -q 'nyrt_string_length' "$tmp_main" && ! grep -Eq 'add i64 .*[, ]5' "$tmp_main"; then
      test_fail "$SMOKE_NAME($case_name): main missing fast strlen or const-folded add(5)"
      return 1
    fi
  fi

  if [ "$case_name" = "box_create_destroy" ]; then
    if grep -q 'nyrt_string_length' "$tmp_main"; then
      test_fail "$SMOKE_NAME($case_name): main unexpectedly contains nyrt_string_length"
      return 1
    fi
  fi

  return 0
}

run_case "method_call_only" "$NYASH_ROOT/benchmarks/bench_method_call_only.hako" || exit 1
run_case "box_create_destroy" "$NYASH_ROOT/benchmarks/bench_box_create_destroy.hako" || exit 1

test_pass "$SMOKE_NAME"
