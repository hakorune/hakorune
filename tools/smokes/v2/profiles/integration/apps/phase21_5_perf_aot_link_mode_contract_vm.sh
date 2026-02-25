#!/bin/bash
# phase21_5_perf_aot_link_mode_contract_vm.sh
#
# Contract pin:
# - PERF AOT fast lane (NYASH_LLVM_FAST=1) emits non-PIE executable on Linux.
# - This keeps micro-bench AOT launch overhead stable.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_aot_link_mode_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! command -v readelf >/dev/null 2>&1; then
  test_fail "$SMOKE_NAME: readelf not found"
  exit 2
fi

BENCH="$NYASH_ROOT/benchmarks/bench_numeric_mixed_medium.hako"
EMIT_HELPER="$NYASH_ROOT/tools/hakorune_emit_mir.sh"
MIR_BUILDER="$NYASH_ROOT/tools/ny_mir_builder.sh"
TMP_MIR="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.mir.json")"
TMP_EXE="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.exe")"
TMP_LOG="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.log")"
cleanup() {
  rm -f "$TMP_MIR" "$TMP_EXE" "$TMP_LOG" >/dev/null 2>&1 || true
}
trap cleanup EXIT

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

# Ensure linker behavior is checked against current source, not stale ny-llvmc binary.
set +e
cargo build --release -p nyash-llvm-compiler >/dev/null 2>&1
build_compiler_rc=$?
set -e
if [ "$build_compiler_rc" -ne 0 ]; then
  test_fail "$SMOKE_NAME: failed to build nyash-llvm-compiler"
  exit 1
fi

set +e
bash "$EMIT_HELPER" "$BENCH" "$TMP_MIR" >"$TMP_LOG" 2>&1
emit_rc=$?
set -e
if [ "$emit_rc" -ne 0 ]; then
  tail -n 40 "$TMP_LOG" || true
  test_fail "$SMOKE_NAME: MIR emit failed rc=$emit_rc"
  exit 1
fi

set +e
NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
NYASH_LLVM_FAST=1 \
NYASH_LLVM_FAST_INT="${NYASH_LLVM_FAST_INT:-1}" \
bash "$MIR_BUILDER" --in "$TMP_MIR" --emit exe -o "$TMP_EXE" --quiet >>"$TMP_LOG" 2>&1
build_rc=$?
set -e
if [ "$build_rc" -ne 0 ]; then
  tail -n 80 "$TMP_LOG" || true
  test_fail "$SMOKE_NAME: AOT build failed rc=$build_rc"
  exit 1
fi

HEADER="$(readelf -h "$TMP_EXE" 2>/dev/null || true)"
if ! printf '%s\n' "$HEADER" | grep -Eq 'Type:[[:space:]]+EXEC'; then
  printf '%s\n' "$HEADER"
  test_fail "$SMOKE_NAME: expected non-PIE executable (Type EXEC) in fast lane"
  exit 1
fi

test_pass "$SMOKE_NAME"
