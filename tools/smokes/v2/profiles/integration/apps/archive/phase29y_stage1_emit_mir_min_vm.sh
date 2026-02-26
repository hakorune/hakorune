#!/bin/bash
# Phase 29y RVP-0-min2: stage1 minimum emit-mir smoke
#
# Contract pin:
# 1) --hako-emit-mir-json route does not timeout on a minimal stage1 entry.
# 2) minimal stage1 entry that does not emit MIR fails fast with stable contract.
# 3) invalid stage1 entry fails fast with stable "entry not found" contract.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

INPUT="${1:-$NYASH_ROOT/apps/examples/string_p0.hako}"
STAGE1_ENTRY="$NYASH_ROOT/apps/examples/string_p0.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TIMEOUT_MS="${NYASH_NY_COMPILER_TIMEOUT_MS:-6000}"
TMP_MIR="$(mktemp /tmp/phase29y_stage1_emit_mir_min.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

if ! [[ "$TIMEOUT_MS" =~ ^[0-9]+$ ]]; then
  test_fail "phase29y_stage1_emit_mir_min_vm: timeout must be integer: $TIMEOUT_MS"
  exit 2
fi

if [[ "$INPUT" != /* ]]; then
  INPUT="$NYASH_ROOT/$INPUT"
fi

if [ ! -f "$INPUT" ]; then
  test_fail "phase29y_stage1_emit_mir_min_vm: fixture missing: $INPUT"
  exit 2
fi
if [ ! -f "$STAGE1_ENTRY" ]; then
  test_fail "phase29y_stage1_emit_mir_min_vm: stage1 entry missing: $STAGE1_ENTRY"
  exit 2
fi

set +e
OUT_OK=$(timeout "$RUN_TIMEOUT_SECS" env \
  STAGE1_CLI_ENTRY="$STAGE1_ENTRY" \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_USE_FALLBACK=0 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_JOINIR_DEV=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_STRICT=0 \
  HAKO_JOINIR_PLANNER_REQUIRED=0 \
  NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS" \
  "$NYASH_BIN" --hako-emit-mir-json "$TMP_MIR" "$INPUT" 2>&1)
RC_OK=$?
set -e

if [ "$RC_OK" -eq 124 ]; then
  test_fail "phase29y_stage1_emit_mir_min_vm: emit-mir timed out (>${RUN_TIMEOUT_SECS}s)"
  exit 1
fi
if [ "$RC_OK" -eq 0 ]; then
  if [ ! -s "$TMP_MIR" ]; then
    test_fail "phase29y_stage1_emit_mir_min_vm: output MIR file missing/empty"
    exit 1
  fi
  if ! jq -e '.functions | length >= 1' "$TMP_MIR" >/dev/null 2>&1; then
    test_fail "phase29y_stage1_emit_mir_min_vm: output is not valid MIR(JSON v0)"
    exit 1
  fi
else
  if ! printf '%s\n' "$OUT_OK" | rg -q '\[stage1-cli\] emit-mir: no MIR\(JSON v0\) found in stub output'; then
    echo "$OUT_OK" | tail -n 50 || true
    test_fail "phase29y_stage1_emit_mir_min_vm: unexpected failure contract on minimal stage1 entry (rc=$RC_OK)"
    exit 1
  fi
fi

set +e
OUT_BAD=$(timeout "$RUN_TIMEOUT_SECS" env \
  STAGE1_CLI_ENTRY=/tmp/phase29y_missing_stage1_cli.hako \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_USE_FALLBACK=0 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_JOINIR_DEV=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_STRICT=0 \
  HAKO_JOINIR_PLANNER_REQUIRED=0 \
  NYASH_NY_COMPILER_TIMEOUT_MS="$TIMEOUT_MS" \
  "$NYASH_BIN" --hako-emit-mir-json "$TMP_MIR" "$INPUT" 2>&1)
RC_BAD=$?
set -e

if [ "$RC_BAD" -eq 124 ]; then
  test_fail "phase29y_stage1_emit_mir_min_vm: invalid-entry probe timed out (>${RUN_TIMEOUT_SECS}s)"
  exit 1
fi
if [ "$RC_BAD" -eq 0 ]; then
  test_fail "phase29y_stage1_emit_mir_min_vm: invalid-entry probe unexpectedly succeeded"
  exit 1
fi
if ! printf '%s\n' "$OUT_BAD" | rg -q '\[stage1-cli\] entry not found:'; then
  echo "$OUT_BAD" | tail -n 50 || true
  test_fail "phase29y_stage1_emit_mir_min_vm: missing stage1 entry-not-found contract"
  exit 1
fi

test_pass "phase29y_stage1_emit_mir_min_vm: PASS (stage1 emit-mir liveliness pinned)"
