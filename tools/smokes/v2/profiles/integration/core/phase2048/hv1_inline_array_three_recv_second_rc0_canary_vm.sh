#!/bin/bash
# hv1 inline: ArrayBox three receivers, push on first/third only; second size==0
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"mir_call","dst":0, "callee":{"type":"Constructor","box_type":"ArrayBox"}, "args":[], "effects":[]},
  {"op":"mir_call","dst":4, "callee":{"type":"Constructor","box_type":"ArrayBox"}, "args":[], "effects":[]},
  {"op":"mir_call","dst":8, "callee":{"type":"Constructor","box_type":"ArrayBox"}, "args":[], "effects":[]},
  {"op":"const","dst":10, "value": {"type": "i64", "value": 7}},
  {"op":"mir_call", "callee":{"type":"Method","box_name":"ArrayBox","method":"push","receiver":0}, "args":[10], "effects":[]},
  {"op":"mir_call", "callee":{"type":"Method","box_name":"ArrayBox","method":"push","receiver":8}, "args":[10], "effects":[]},
  {"op":"mir_call","dst":2, "callee":{"type":"Method","box_name":"ArrayBox","method":"size","receiver":4}, "args":[], "effects":[]},
  {"op":"ret","value":2}
]}]}]}'

tmp="/tmp/hv1_arr_3recv_$$.json"; printf '%s' "$json" > "$tmp"
set +e
rc=$(HAKO_PRIMARY_NO_FALLBACK=1 HAKO_VERIFY_PRIMARY=hakovm HAKO_VM_MIRCALL_SIZESTATE=1 HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=1 verify_v1_inline_file "$tmp" || true)
set -e
rm -f "$tmp"

if [[ "$rc" =~ ^-?[0-9]+$ ]] && [ "$rc" -eq 0 ]; then
  echo "[PASS] hv1_inline_array_three_recv_second_rc0_canary_vm"
  exit 0
fi
echo "[FAIL] hv1_inline_array_three_recv_second_rc0_canary_vm (rc=$rc, expect 0)" >&2
exit 1

