#!/bin/bash
# phase29y_joinir_reject_detail_vm.sh
# Contract:
# - Unsupported loop freeze must include [joinir/reject_detail] with route/reject context.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

INPUT="${1:-$NYASH_ROOT/apps/tests/phase29y_joinir_reject_detail_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -f "$INPUT" ]; then
  test_fail "phase29y_joinir_reject_detail_vm: fixture missing: $INPUT"
  exit 2
fi

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  test_fail "phase29y_joinir_reject_detail_vm: timed out (>${RUN_TIMEOUT_SECS}s)"
  exit 1
fi

if [ "$EXIT_CODE" -eq 0 ]; then
  echo "$OUTPUT" | tail -n 80 || true
  test_fail "phase29y_joinir_reject_detail_vm: expected non-zero exit"
  exit 1
fi

if ! printf '%s\n' "$OUTPUT" | rg -q 'Loop lowering failed: JoinIR does not support this pattern'; then
  echo "$OUTPUT" | tail -n 120 || true
  test_fail "phase29y_joinir_reject_detail_vm: missing joinir freeze summary"
  exit 1
fi

if ! printf '%s\n' "$OUTPUT" | rg -q '\[joinir/reject_detail\]'; then
  echo "$OUTPUT" | tail -n 120 || true
  test_fail "phase29y_joinir_reject_detail_vm: missing reject detail tag"
  exit 1
fi

if ! printf '%s\n' "$OUTPUT" | rg -q 'route_exhausted|reason='; then
  echo "$OUTPUT" | tail -n 120 || true
  test_fail "phase29y_joinir_reject_detail_vm: missing route/reason context"
  exit 1
fi

test_pass "phase29y_joinir_reject_detail_vm: PASS (reject detail surfaced)"
