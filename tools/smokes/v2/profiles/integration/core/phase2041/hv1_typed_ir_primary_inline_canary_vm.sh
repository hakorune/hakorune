#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../../../../../.." && pwd)
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Inline Hako code that dispatches hv1 on JSON from env
TMPCODE="/tmp/hv1_typed_ir_$$.hako"
cat >"$TMPCODE" <<'HCODE'
include "lang/src/vm/hakorune-vm/dispatcher_v1.hako"
static box Main { method main(args) {
  local j = env.get("NYASH_VERIFY_JSON")
  local r = NyVmDispatcherV1Box.run(j)
  print("" + r)
  return r
} }
HCODE

# Minimal v1-ish JSON (segment scan tolerant): return 12
JSON='{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"int","value":12}},{"op":"ret","value":1}]}]}]}'

set +e
OUT=$(HAKO_V1_TYPED_IR_PRIMARY=1 HAKO_V1_DISPATCHER_FLOW=1 \
      HAKO_V1_TYPED_IR_SHADOW=1 HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
      NYASH_VERIFY_JSON="$JSON" HAKO_PREINCLUDE=1 \
      run_nyash_vm -c "$(cat "$TMPCODE")" 2>&1)
RC=$?
set -e

if [ $RC -ne 12 ]; then
  echo "[FAIL] hv1_typed_ir_primary_inline_canary_vm: expected rc=12, got rc=$RC" >&2
  echo "$OUT" >&2
  exit 1
fi

echo "$OUT" | tail -n1 | grep -qx '12' || {
  echo "[FAIL] hv1_typed_ir_primary_inline_canary_vm: stdout tail not '12'" >&2
  echo "$OUT" >&2
  exit 1
}

echo "[PASS] hv1_typed_ir_primary_inline_canary_vm"

rm -f "$TMPCODE" || true
