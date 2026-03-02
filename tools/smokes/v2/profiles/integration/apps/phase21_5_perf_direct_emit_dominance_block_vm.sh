#!/bin/bash
# phase21_5_perf_direct_emit_dominance_block_vm.sh
#
# Contract pin:
# - direct emit routes (`--emit-mir-json` / `--emit-exe`) must fail-fast on verifier errors.
# - matrix: route=vm|mir × emit=mir|exe
# - helper route (`tools/hakorune_emit_mir.sh`) must still produce buildable MIR.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_direct_emit_dominance_block_vm"
BENCH="$NYASH_ROOT/benchmarks/bench_box_create_destroy.hako"
EMIT_ROUTE="$NYASH_ROOT/tools/smokes/v2/lib/emit_mir_route.sh"
EMIT_TIMEOUT_SECS="${EMIT_TIMEOUT_SECS:-30}"
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
if [ ! -x "$EMIT_ROUTE" ]; then
  test_fail "$SMOKE_NAME: emit route helper missing/executable: $EMIT_ROUTE"
  exit 2
fi
if [ ! -f "$MIR_BUILDER" ]; then
  test_fail "$SMOKE_NAME: MIR builder missing: $MIR_BUILDER"
  exit 2
fi

TMP_DIRECT_VM_MIR="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.direct.vm.mir.json")"
TMP_DIRECT_VM_MIR_LOG="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.direct.vm.mir.log")"
TMP_DIRECT_VM_EXE="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.direct.vm.exe")"
TMP_DIRECT_VM_EXE_LOG="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.direct.vm.exe.log")"
TMP_DIRECT_MIR_ROUTE_MIR="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.direct.mir_route.mir.json")"
TMP_DIRECT_MIR_ROUTE_MIR_LOG="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.direct.mir_route.mir.log")"
TMP_DIRECT_MIR_ROUTE_EXE="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.direct.mir_route.exe")"
TMP_DIRECT_MIR_ROUTE_EXE_LOG="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.direct.mir_route.exe.log")"
TMP_HELPER_MIR="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.helper.mir.json")"
TMP_HELPER_LOG="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.helper.log")"
TMP_EXE="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.exe")"
cleanup() {
  rm -f \
    "$TMP_DIRECT_VM_MIR" "$TMP_DIRECT_VM_MIR_LOG" \
    "$TMP_DIRECT_VM_EXE" "$TMP_DIRECT_VM_EXE_LOG" \
    "$TMP_DIRECT_MIR_ROUTE_MIR" "$TMP_DIRECT_MIR_ROUTE_MIR_LOG" \
    "$TMP_DIRECT_MIR_ROUTE_EXE" "$TMP_DIRECT_MIR_ROUTE_EXE_LOG" \
    "$TMP_HELPER_MIR" "$TMP_HELPER_LOG" "$TMP_EXE" >/dev/null 2>&1 || true
}
trap cleanup EXIT

run_expect_failfast() {
  local label="$1"
  local route="$2"
  local contract_tag="$3"
  local log_file="$4"
  shift 4

  set +e
  "$HAKO_BIN" "$@" >"$log_file" 2>&1
  local rc=$?
  set -e

  if [ "$rc" -eq 0 ]; then
    test_fail "$SMOKE_NAME: ${label} unexpectedly succeeded"
    exit 1
  fi
  if ! grep -Fq "[freeze:contract][${contract_tag}] route=${route} errors=" "$log_file"; then
    test_fail "$SMOKE_NAME: ${label} missing freeze contract tag"
    exit 1
  fi
  if ! grep -Fq "[${contract_tag}] route=${route} detail=" "$log_file"; then
    test_fail "$SMOKE_NAME: ${label} missing verifier detail tag"
    exit 1
  fi
}

# 1) direct emit matrix must fail-fast on verifier errors.
run_expect_failfast "route=vm emit-mir" "vm" "emit-mir/direct-verify" "$TMP_DIRECT_VM_MIR_LOG" \
  --emit-mir-json "$TMP_DIRECT_VM_MIR" "$BENCH"
run_expect_failfast "route=vm emit-exe" "vm" "emit-exe/direct-verify" "$TMP_DIRECT_VM_EXE_LOG" \
  --emit-exe "$TMP_DIRECT_VM_EXE" "$BENCH"
run_expect_failfast "route=mir emit-mir" "mir" "emit-mir/direct-verify" "$TMP_DIRECT_MIR_ROUTE_MIR_LOG" \
  --backend mir --emit-mir-json "$TMP_DIRECT_MIR_ROUTE_MIR" "$BENCH"
run_expect_failfast "route=mir emit-exe" "mir" "emit-exe/direct-verify" "$TMP_DIRECT_MIR_ROUTE_EXE_LOG" \
  --backend mir --emit-exe "$TMP_DIRECT_MIR_ROUTE_EXE" "$BENCH"

# 2) helper route must still work and produce buildable MIR.
set +e
"$EMIT_ROUTE" --route hako-helper --timeout-secs "$EMIT_TIMEOUT_SECS" --out "$TMP_HELPER_MIR" --input "$BENCH" >"$TMP_HELPER_LOG" 2>&1
helper_rc=$?
set -e
if [ "$helper_rc" -ne 0 ]; then
  tail -n 80 "$TMP_HELPER_LOG" || true
  test_fail "$SMOKE_NAME: helper emit failed rc=$helper_rc"
  exit 1
fi

set +e
NYASH_LLVM_FAST=1 NYASH_LLVM_SKIP_BUILD=1 \
bash "$MIR_BUILDER" --in "$TMP_HELPER_MIR" --emit exe -o "$TMP_EXE" --quiet >>"$TMP_HELPER_LOG" 2>&1
build_rc=$?
set -e
if [ "$build_rc" -ne 0 ]; then
  tail -n 120 "$TMP_HELPER_LOG" || true
  test_fail "$SMOKE_NAME: helper MIR not buildable rc=$build_rc"
  exit 1
fi

test_pass "$SMOKE_NAME"
