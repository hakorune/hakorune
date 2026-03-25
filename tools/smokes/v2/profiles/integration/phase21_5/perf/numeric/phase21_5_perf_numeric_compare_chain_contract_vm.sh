#!/bin/bash
# phase21_5_perf_numeric_compare_chain_contract_vm.sh
#
# Contract pin:
# - numeric_mixed_medium AOT main loop keeps expected integer `%`/compare chain.
# - loop guard compare must not collapse to `< 0` due PHI/alias regression.
# - branch compares stay i1 (no i1->i64->i1 round-trip).

set -euo pipefail

source "$(dirname "$0")/../../../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_numeric_compare_chain_contract_vm"
BENCH="$NYASH_ROOT/benchmarks/bench_numeric_mixed_medium.hako"
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
NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
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

extract_block_body() {
  local label="$1"
  awk -v want="${label}:" '
    $0 == want { in_block=1; next }
    /^bb[0-9]+:$/ { if (in_block) exit }
    in_block { print }
  ' "$TMP_MAIN"
}

require_block_contains() {
  local label="$1"
  local pattern="$2"
  local message="$3"
  local body
  body="$(extract_block_body "$label")"
  if [ -z "$body" ]; then
    test_fail "$SMOKE_NAME: missing block ${label}"
    exit 1
  fi
  if ! printf '%s\n' "$body" | grep -Eq "$pattern"; then
    test_fail "$SMOKE_NAME: ${message} (block=${label})"
    exit 1
  fi
}

guard_line="$(extract_block_body bb1 | grep -m1 'icmp slt i64' || true)"
if [ -z "$guard_line" ]; then
  test_fail "$SMOKE_NAME: missing bb1 loop guard icmp slt i64"
  exit 1
fi
if printf '%s\n' "$guard_line" | grep -Eq ',[[:space:]]*0([[:space:]]|$)'; then
  test_fail "$SMOKE_NAME: loop guard collapsed to < 0 (unexpected alias regression)"
  exit 1
fi

require_block_contains bb1 'icmp slt i64' 'loop guard compare not found'
require_block_contains bb1 'br i1 ' 'loop guard branch does not use i1'
require_block_contains bb2 'srem i64 .*31' 'missing modulo 31 in hot loop entry'
require_block_contains bb2 'icmp slt i64 .*10' 'missing <10 compare in first branch split'
require_block_contains bb2 'br i1 ' 'first branch split does not use i1'
require_block_contains bb6 'srem i64 .*97' 'missing modulo 97 in then branch'
require_block_contains bb8 'icmp slt i64 .*20' 'missing <20 compare in second branch split'
require_block_contains bb8 'br i1 ' 'second branch split does not use i1'
require_block_contains bb9 'srem i64 .*89' 'missing modulo 89 in mid branch'
require_block_contains bb10 'srem i64 .*53' 'missing modulo 53 in else branch'
require_block_contains bb11 'srem i64 .*17' 'missing modulo 17 in accumulator update'

if grep -Eq 'zext i1 .*cmp_.* to i64' "$TMP_MAIN"; then
  test_fail "$SMOKE_NAME: compare i1->i64 cast detected in numeric compare chain"
  exit 1
fi

test_pass "$SMOKE_NAME"
