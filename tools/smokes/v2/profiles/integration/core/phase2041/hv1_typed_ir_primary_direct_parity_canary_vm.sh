#!/usr/bin/env bash
# hv1 direct (env JSON) parity: PRIMARY=0 vs PRIMARY=1 should have identical rc
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

JSON='{"functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"int","value":21}},{"op":"ret","value":1}]}]}]}'

# Run with PRIMARY=0 (default)
set +e
OUT0=$(HAKO_VERIFY_PRIMARY=hakovm NYASH_VERIFY_JSON="$JSON" "$NYASH_BIN" --backend vm "$NYASH_ROOT/basic_test.hako" 2>&1)
RC0=$?
set -e

# Run with PRIMARY=1 (flow path + shadow)
set +e
OUT1=$(HAKO_VERIFY_PRIMARY=hakovm HAKO_V1_DISPATCHER_FLOW=1 HAKO_V1_TYPED_IR_PRIMARY=1 HAKO_V1_TYPED_IR_SHADOW=1 NYASH_VERIFY_JSON="$JSON" "$NYASH_BIN" --backend vm "$NYASH_ROOT/basic_test.hako" 2>&1)
RC1=$?
set -e

if [ $RC0 -ne $RC1 ] || [ $RC0 -ne 21 ]; then
  echo "[FAIL] hv1_typed_ir_primary_direct_parity_canary_vm: rc0=$RC0 rc1=$RC1" >&2
  echo "--- OUT0 ---" >&2
  echo "$OUT0" >&2
  echo "--- OUT1 ---" >&2
  echo "$OUT1" >&2
  exit 1
fi

echo "[PASS] hv1_typed_ir_primary_direct_parity_canary_vm"

