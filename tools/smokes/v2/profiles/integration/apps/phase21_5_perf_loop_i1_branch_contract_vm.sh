#!/bin/bash
# phase21_5_perf_loop_i1_branch_contract_vm.sh
#
# Contract pin:
# - loop compare used only by branch is kept as i1 in FAST mode.
# - avoid compare i1->i64->i1 round-trip in main loop.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_loop_i1_branch_contract_vm"
BENCH="$NYASH_ROOT/benchmarks/bench_box_create_destroy.hako"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
EMIT_TIMEOUT_SECS="${EMIT_TIMEOUT_SECS:-30}"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"

if [ ! -f "$BENCH" ]; then
  test_fail "$SMOKE_NAME: benchmark missing: $BENCH"
  exit 2
fi
if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "$SMOKE_NAME: emit route helper missing/executable: $EMIT_ROUTE"
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
"$EMIT_ROUTE" --route hako-helper --timeout-secs "$EMIT_TIMEOUT_SECS" --out "$TMP_MIR" --input "$BENCH" >"$TMP_LOG" 2>&1
emit_rc=$?
set -e
if [ "$emit_rc" -ne 0 ]; then
  tail -n 60 "$TMP_LOG" || true
  test_fail "$SMOKE_NAME: MIR emit failed rc=$emit_rc"
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

if ! grep -q 'icmp slt i64' "$TMP_MAIN"; then
  test_fail "$SMOKE_NAME: main missing integer compare in loop"
  exit 1
fi

if grep -Eq 'zext i1 .*cmp_.* to i64' "$TMP_MAIN"; then
  test_fail "$SMOKE_NAME: main still contains compare i1->i64 cast"
  exit 1
fi

if grep -Eq 'icmp ne i64 .*cond_i1|icmp ne i64 .*cmp_.*_i64' "$TMP_MAIN"; then
  test_fail "$SMOKE_NAME: main still contains i64->i1 cond compare"
  exit 1
fi

if ! grep -Eq 'br i1 %"?cmp_' "$TMP_MAIN"; then
  test_fail "$SMOKE_NAME: main branch does not use compare i1 directly"
  exit 1
fi

test_pass "$SMOKE_NAME"
