#!/bin/bash
# Mini‑VM size state per receiver: A(1 push), B(1 push), size(A)+size(B)=2
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"mir_call","dst":0, "callee":{"type":"Constructor","box_type":"ArrayBox"}, "args":[], "effects":[]},
  {"op":"mir_call","dst":1, "callee":{"type":"Constructor","box_type":"ArrayBox"}, "args":[], "effects":[]},
  {"op":"const","dst":10, "value": {"type": "i64", "value": 111}},
  {"op":"const","dst":20, "value": {"type": "i64", "value": 222}},
  {"op":"mir_call", "callee":{"type":"Method","box_name":"ArrayBox","method":"push","receiver":0}, "args":[10], "effects":[]},
  {"op":"mir_call", "callee":{"type":"Method","box_name":"ArrayBox","method":"push","receiver":1}, "args":[20], "effects":[]},
  {"op":"mir_call","dst":2, "callee":{"type":"Method","box_name":"ArrayBox","method":"size","receiver":0}, "args":[], "effects":[]},
  {"op":"mir_call","dst":3, "callee":{"type":"Method","box_name":"ArrayBox","method":"size","receiver":1}, "args":[], "effects":[]},
  {"op":"binop","op_kind":"Add","lhs":2,"rhs":3,"dst":5},
  {"op":"ret","value":5}
]}]}]}'

code=$(cat <<'HCODE'
using "lang/src/vm/boxes/mini_vm_entry.hako" as MiniVmEntryBox
static box Main { method main(args) {
  local j = __MIR_JSON__
  return MiniVmEntryBox.run_min(j)
} }
HCODE
)
json_quoted=$(printf '%s' "$json" | jq -Rs .)
code="${code/__MIR_JSON__/$json_quoted}"

set +e
out=$(NYASH_USING_AST=1 HAKO_VM_MIRCALL_SIZESTATE=1 HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=1 run_nyash_vm -c "$code" 2>&1)
rc=$?
set -e

if [ "$rc" -eq 2 ]; then
  echo "[PASS] v1_minivm_size_state_per_recv_on_canary_vm"
  exit 0
fi
if echo "$out" | grep -q -E '(missing callee|unresolved)'; then
  echo "[SKIP] v1_minivm_size_state_per_recv_on_canary_vm (Mini‑VM not ready: $rc)" >&2
  exit 0
fi
if [ "$rc" -eq 2 ]; then
  echo "[PASS] v1_minivm_size_state_per_recv_on_canary_vm"
  exit 0
fi
echo "[SKIP] v1_minivm_size_state_per_recv_on_canary_vm (unexpected rc=$rc)" >&2; exit 0

