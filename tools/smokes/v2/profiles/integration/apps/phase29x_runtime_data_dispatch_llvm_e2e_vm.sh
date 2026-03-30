#!/bin/bash
# Phase 29x: RuntimeDataBox LLVM E2E contract smoke
#
# Contract:
# - RuntimeDataBox method calls in prebuilt MIR must lower via shared runtime_data dispatch.
# - Route may be either nyash.runtime_data.* or AS-03 array mono-route nyash.array.*.
# - AOT executable must preserve the expected result (rc=4).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29x_runtime_data_dispatch_llvm_e2e_vm"
MIR_FIXTURE="$NYASH_ROOT/apps/tests/phase29x_runtime_data_dispatch_e2e_min_v1.mir.json"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"

if [ ! -f "$MIR_FIXTURE" ]; then
  test_fail "$SMOKE_NAME: fixture missing: $MIR_FIXTURE"
  exit 2
fi
if [ ! -f "$MIR_BUILDER" ]; then
  test_fail "$SMOKE_NAME: MIR builder missing: $MIR_BUILDER"
  exit 2
fi

TMP_EXE="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.exe")"
TMP_IR="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.ll")"
TMP_LOG="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.log")"
cleanup() {
  rm -f "$TMP_EXE" "$TMP_IR" "$TMP_LOG" >/dev/null 2>&1 || true
}
trap cleanup EXIT

set +e
NYASH_LLVM_FAST=1 \
NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
NYASH_LLVM_DUMP_IR="$TMP_IR" \
bash "$MIR_BUILDER" --in "$MIR_FIXTURE" --emit exe -o "$TMP_EXE" --quiet >"$TMP_LOG" 2>&1
build_rc=$?
set -e
if [ "$build_rc" -ne 0 ]; then
  tail -n 80 "$TMP_LOG" || true
  test_fail "$SMOKE_NAME: AOT build failed rc=$build_rc"
  exit 1
fi

if [ ! -s "$TMP_IR" ]; then
  test_fail "$SMOKE_NAME: expected IR dump missing"
  exit 1
fi

check_route_any() {
  local label="$1"
  shift
  local symbols=("$@")
  local sym
  for sym in "${symbols[@]}"; do
    if grep -Fq "$sym" "$TMP_IR"; then
      return 0
    fi
  done
  test_fail "$SMOKE_NAME: missing dispatch symbol for ${label} (${symbols[*]})"
  exit 1
}

check_route_any "push" "nyash.runtime_data.push_hh" "nyash.array.slot_append_hh"
check_route_any "get" "nyash.runtime_data.get_hh" "nyash.array.slot_load_hi"
check_route_any "has" "nyash.runtime_data.has_hh"
check_route_any "set" "nyash.runtime_data.set_hhh" "nyash.array.slot_store_hih" "nyash.array.slot_store_hii"

set +e
"$TMP_EXE" >/dev/null 2>>"$TMP_LOG"
run_rc=$?
set -e
if [ "$run_rc" -ne 4 ]; then
  tail -n 80 "$TMP_LOG" || true
  test_fail "$SMOKE_NAME: expected executable rc=4, got rc=$run_rc"
  exit 1
fi

test_pass "$SMOKE_NAME: RuntimeDataBox LLVM dispatch path locked (rc=4)"
