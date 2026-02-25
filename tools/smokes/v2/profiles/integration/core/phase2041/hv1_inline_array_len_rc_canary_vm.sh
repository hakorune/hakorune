#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

JSON='{"kind":"MIR","schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"mir_call","dst":1,"mir_call":{"callee":{"type":"Constructor","box_type":"ArrayBox"},"args":[],"effects":[]}},{"op":"const","dst":2,"value":{"type":"i64","value":9}},{"op":"mir_call","mir_call":{"callee":{"type":"Method","box_name":"ArrayBox","method":"push","receiver":1},"args":[2],"effects":[]}},{"op":"mir_call","dst":3,"mir_call":{"callee":{"type":"Method","box_name":"ArrayBox","method":"len","receiver":1},"args":[],"effects":[]}},{"op":"ret","value":3}]}]}]}'

set +e
HAKO_V1_DISPATCHER_FLOW=1 HAKO_VM_MIRCALL_SIZESTATE=1 HAKO_VERIFY_PRIMARY=hakovm NYASH_VERIFY_JSON="$JSON" "$NYASH_BIN" --backend vm "$NYASH_ROOT/basic_test.nyash" >/dev/null 2>&1
rc=$?
set -e

if [ $rc -ne 1 ]; then
  echo "[FAIL] hv1_inline_array_len_rc_canary_vm rc=$rc (expected 1)" >&2
  exit 1
fi

echo "[PASS] hv1_inline_array_len_rc_canary_vm"
exit 0

