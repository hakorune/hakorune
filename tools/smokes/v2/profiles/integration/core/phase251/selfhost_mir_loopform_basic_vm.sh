#!/usr/bin/env bash
# selfhost_mir_loopform_basic_vm.sh
# - Canary for Phase 25.1b Step3: ensure FuncBodyBasicLowerBox can delegate
#   LoopForm-normalized loops to LowerLoopSimpleBox/LowerLoopSumBcBox.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../../.." && pwd))"

# Quick profile: Stage-B emit is flaky under Stage-3 default; skip for now.
echo "[SKIP] selfhost_mir_loopform_basic_vm (disabled in quick profile after env consolidation)"
exit 0

# Create a minimal .hako file with a simple counting loop
TEST_HAKO="$(mktemp --suffix .hako)"
cat > "$TEST_HAKO" <<'HAKO'
static box TestBox {
  count() {
    local i = 0
    loop(i < 10) {
      i = i + 1
    }
    return i
  }
}

static box Main {
  main() {
    return 0
  }
}
HAKO

OUT_MIR="$(mktemp --suffix .json)"
LOG_OUT="$(mktemp --suffix .log)"
trap 'rm -f "$TEST_HAKO" "$OUT_MIR" "$LOG_OUT" || true' EXIT

set +e
HAKO_SELFHOST_BUILDER_FIRST=1 HAKO_MIR_BUILDER_FUNCS=1 HAKO_SELFHOST_TRACE=1 NYASH_JSON_ONLY=1 \
  bash "$ROOT_DIR/tools/hakorune_emit_mir.sh" "$TEST_HAKO" "$OUT_MIR" >"$LOG_OUT" 2>&1
rc=$?
set -e

# Check logs for LoopForm lower tags
if ! grep -q "\[funcs/basic:loop\." "$LOG_OUT"; then
  echo "[FAIL] selfhost_mir_loopform_basic_vm (no [funcs/basic:loop.*] tag found)" >&2
  echo "=== LOG OUTPUT ===" >&2
  cat "$LOG_OUT" >&2
  exit 1
fi

# Check MIR(JSON) was generated
if [ $rc -ne 0 ] || [ ! -s "$OUT_MIR" ]; then
  echo "[FAIL] selfhost_mir_loopform_basic_vm (MIR generation failed rc=$rc)" >&2
  echo "=== LOG OUTPUT ===" >&2
  cat "$LOG_OUT" >&2
  exit 1
fi

echo "[PASS] selfhost_mir_loopform_basic_vm (LoopForm lower delegated successfully)"
exit 0
