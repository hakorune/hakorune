#!/bin/bash
# phase21_5_perf_const_hoist_contract_vm.sh
#
# Contract pin:
# - In FAST mode, bench_box_create_destroy string const boxing is hoisted to main entry.
# - from_i8_string_const call must not remain in loop body blocks.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_const_hoist_contract_vm"
BENCH="$NYASH_ROOT/benchmarks/bench_box_create_destroy.hako"
EMIT_HELPER="$NYASH_ROOT/tools/hakorune_emit_mir.sh"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"

if [ ! -f "$BENCH" ]; then
  test_fail "$SMOKE_NAME: benchmark missing: $BENCH"
  exit 2
fi
if [ ! -f "$EMIT_HELPER" ]; then
  test_fail "$SMOKE_NAME: emit helper missing: $EMIT_HELPER"
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
bash "$EMIT_HELPER" "$BENCH" "$TMP_MIR" >"$TMP_LOG" 2>&1
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

awk '
  /^define .*@"main"\(/ { in_main=1 }
  in_main { print }
  in_main && /^}$/ { exit }
' "$TMP_IR" >"$TMP_MAIN"

if ! grep -q '^define .*@"main"' "$TMP_MAIN"; then
  test_fail "$SMOKE_NAME: main function not found in dumped IR"
  exit 1
fi

if ! grep -q 'call i64 @"nyash.box.from_i8_string_const"' "$TMP_MAIN"; then
  test_fail "$SMOKE_NAME: main has no from_i8_string_const call"
  exit 1
fi

first_label="$(awk '
  /^[[:space:]]*[[:graph:]]+:[[:space:]]*$/ {
    sub(/:[[:space:]]*$/, "", $1)
    print $1
    exit
  }
' "$TMP_MAIN")"

mapfile -t call_labels < <(awk '
  /^[[:space:]]*[[:graph:]]+:[[:space:]]*$/ {
    label=$1
    sub(/:[[:space:]]*$/, "", label)
  }
  /call i64 @"nyash\.box\.from_i8_string_const"/ {
    if (label != "") print label
  }
' "$TMP_MAIN" | sort -u)

if [ "${#call_labels[@]}" -ne 1 ]; then
  test_fail "$SMOKE_NAME: expected exactly one labeled call site, got ${#call_labels[@]}"
  exit 1
fi

if [ -z "$first_label" ]; then
  test_fail "$SMOKE_NAME: could not determine main first label"
  exit 1
fi

if [ "${call_labels[0]}" != "$first_label" ]; then
  test_fail "$SMOKE_NAME: const boxer call not hoisted to entry (label=${call_labels[0]}, entry=$first_label)"
  exit 1
fi

test_pass "$SMOKE_NAME"
