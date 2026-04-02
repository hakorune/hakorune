#!/bin/bash
# VM Adapter (Hako): Array push→len via MirCallV1Handler + AdapterRegistry → rc=2
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

export HAKO_ABI_ADAPTER=${HAKO_ABI_ADAPTER:-1}
export HAKO_VM_MIRCALL_SIZESTATE=${HAKO_VM_MIRCALL_SIZESTATE:-1}
export HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=${HAKO_VM_MIRCALL_SIZESTATE_PER_RECV:-0}

code=$(cat <<'HCODE'
using selfhost.vm.helpers.mir_call_v1_handler as MirCallV1HandlerBox
using selfhost.shared.json.utils.json_frag as JsonFragBox

static box Main {
  method main(args) {
    // Simulate: push, push, len → rc=2
    local regs = new MapBox()
    // receiver = 1 (arbitrary); dst is ignored for push
    local push1 = "{\"op\":\"mir_call\",\"dst\":2,\"mir_call\":{\"callee\":{\"type\":\"Method\",\"box_name\":\"ArrayBox\",\"method\":\"push\",\"receiver\":1},\"args\":[3],\"effects\":[]}}"
    local push2 = push1
    MirCallV1HandlerBox.handle(push1, regs)
    MirCallV1HandlerBox.handle(push2, regs)
    local len_seg = "{\"op\":\"mir_call\",\"dst\":9,\"mir_call\":{\"callee\":{\"type\":\"Method\",\"box_name\":\"ArrayBox\",\"method\":\"len\",\"receiver\":1},\"args\":[],\"effects\":[]}}"
    MirCallV1HandlerBox.handle(len_seg, regs)
    local raw = regs.getField("9")
    if raw == null { return 0 }
    return JsonFragBox._str_to_int(raw)
  }
}
HCODE
)

out=$(run_nyash_vm -c "$code")
rc=$(echo "$out" | awk '/^RC:/{print $2}' | tail -n1)
test -z "$rc" && rc=$(echo "$out" | tail -n1)
if [[ "$rc" -ne 2 ]]; then
  echo "[FAIL] rc=$rc (expect 2)" >&2; exit 1
fi
echo "[PASS] s3_vm_adapter_array_len_canary_vm"
exit 0
