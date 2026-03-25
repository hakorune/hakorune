#!/bin/bash
# phase21_5_perf_strlen_ir_contract_vm.sh
#
# Contract pin:
# - bench_method_call_only_small lowered main-loop must not call generic length helper.
# - literal/faster lowering path must be observable in main body.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_strlen_ir_contract_vm"
BENCH="$NYASH_ROOT/benchmarks/bench_method_call_only_small.hako"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"
HAKO_BIN="${NYASH_BIN:-$NYASH_ROOT/target/release/hakorune}"

if [ ! -f "$BENCH" ]; then
  test_fail "$SMOKE_NAME: benchmark missing: $BENCH"
  exit 2
fi
if [ ! -x "$HAKO_BIN" ]; then
  test_fail "$SMOKE_NAME: hakorune binary missing: $HAKO_BIN"
  exit 2
fi
if [ ! -f "$MIR_BUILDER" ]; then
  test_fail "$SMOKE_NAME: MIR builder missing: $MIR_BUILDER"
  exit 2
fi

TMP_MIR="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.mir.json")"
TMP_IR="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.ll")"
TMP_EXE="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.exe")"
TMP_LOG="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.log")"
TMP_MAIN="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.main.ll")"
cleanup() {
  rm -f "$TMP_MIR" "$TMP_IR" "$TMP_EXE" "$TMP_LOG" "$TMP_MAIN" >/dev/null 2>&1 || true
}
trap cleanup EXIT

set +e
"$HAKO_BIN" --emit-mir-json "$TMP_MIR" "$BENCH" >"$TMP_LOG" 2>&1
emit_rc=$?
set -e
if [ "$emit_rc" -ne 0 ]; then
  tail -n 60 "$TMP_LOG" || true
  test_fail "$SMOKE_NAME: direct MIR emit failed rc=$emit_rc"
  exit 1
fi

set +e
NYASH_LLVM_FAST=1 \
NYASH_LLVM_DUMP_IR="$TMP_IR" \
bash "$MIR_BUILDER" --in "$TMP_MIR" --emit exe -o "$TMP_EXE" --quiet >>"$TMP_LOG" 2>&1
build_rc=$?
set -e
if [ "$build_rc" -ne 0 ]; then
  tail -n 80 "$TMP_LOG" || true
  test_fail "$SMOKE_NAME: AOT build failed rc=$build_rc"
  exit 1
fi

if [ ! -s "$TMP_IR" ]; then
  test_fail "$SMOKE_NAME: expected IR dump is empty"
  exit 1
fi

if ! extract_ir_entry_function "$TMP_IR" "$TMP_MAIN"; then
  test_fail "$SMOKE_NAME: entry function not found in dumped IR"
  exit 1
fi

if grep -q 'call i64 @"nyash.any.length_h"' "$TMP_MAIN"; then
  test_fail "$SMOKE_NAME: main still calls nyash.any.length_h"
  exit 1
fi

if grep -q 'call void @"ny_check_safepoint"' "$TMP_MAIN"; then
  test_fail "$SMOKE_NAME: main still contains ny_check_safepoint"
  exit 1
fi

if ! grep -Eq 'call i64 @"nyrt_string_length"|add i64 .*?, 5' "$TMP_MAIN"; then
  test_fail "$SMOKE_NAME: fast length evidence missing in main"
  exit 1
fi

test_pass "$SMOKE_NAME"
