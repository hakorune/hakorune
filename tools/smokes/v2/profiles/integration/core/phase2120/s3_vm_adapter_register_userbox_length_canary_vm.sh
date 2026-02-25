#!/bin/bash
# VM Adapter (Hako): register UserArrayBox push/length → two pushes then length → rc=2
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

export HAKO_ABI_ADAPTER=${HAKO_ABI_ADAPTER:-1}
export HAKO_ABI_ADAPTER_DEV=${HAKO_ABI_ADAPTER_DEV:-1}
export HAKO_VM_MIRCALL_SIZESTATE=${HAKO_VM_MIRCALL_SIZESTATE:-0}

code=$(cat <<'HCODE'
using selfhost.vm.boxes.abi_adapter_registry as AbiAdapterRegistryBox
using selfhost.vm.helpers.mir_call_v1_handler as MirCallV1HandlerBox
using selfhost.shared.json.utils.json_frag as JsonFragBox

static box Main {
  method main(args) {
    // Register mappings for UserArrayBox
    AbiAdapterRegistryBox.register("UserArrayBox", "push", "nyash.array.push_h", "h", "none")
    AbiAdapterRegistryBox.register("UserArrayBox", "length", "nyash.array.len_h", "h", "none")
    // Simulate: push, push, length → rc=2
    local regs = new MapBox()
    local p = "{\"op\":\"mir_call\",\"dst\":2,\"mir_call\":{\"callee\":{\"type\":\"Method\",\"box_name\":\"UserArrayBox\",\"method\":\"push\",\"receiver\":1},\"args\":[3],\"effects\":[]}}"
    MirCallV1HandlerBox.handle(p, regs)
    MirCallV1HandlerBox.handle(p, regs)
    local l = "{\"op\":\"mir_call\",\"dst\":9,\"mir_call\":{\"callee\":{\"type\":\"Method\",\"box_name\":\"UserArrayBox\",\"method\":\"length\",\"receiver\":1},\"args\":[],\"effects\":[]}}"
    MirCallV1HandlerBox.handle(l, regs)
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
echo "[PASS] s3_vm_adapter_register_userbox_length_canary_vm"
exit 0
