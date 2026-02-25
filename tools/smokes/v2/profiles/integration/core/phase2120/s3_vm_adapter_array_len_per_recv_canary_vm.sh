#!/bin/bash
# VM Adapter (Hako): per-recv len separation → recv1 push; recv2 len=0
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

export HAKO_ABI_ADAPTER=${HAKO_ABI_ADAPTER:-1}
export HAKO_VM_MIRCALL_SIZESTATE=${HAKO_VM_MIRCALL_SIZESTATE:-1}
export HAKO_VM_MIRCALL_SIZESTATE_PER_RECV=${HAKO_VM_MIRCALL_SIZESTATE_PER_RECV:-1}

code=$(cat <<'HCODE'
using selfhost.vm.helpers.mir_call_v1_handler as MirCallV1HandlerBox
using selfhost.shared.json.utils.json_frag as JsonFragBox

static box Main {
  method main(args) {
    // push on recv=1, then len on recv=2 → rc=0 (per-recv)
    local regs = new MapBox()
    local p1 = "{\"op\":\"mir_call\",\"dst\":2,\"mir_call\":{\"callee\":{\"type\":\"Method\",\"box_name\":\"ArrayBox\",\"method\":\"push\",\"receiver\":1},\"args\":[3],\"effects\":[]}}"
    MirCallV1HandlerBox.handle(p1, regs)
    local len2 = "{\"op\":\"mir_call\",\"dst\":9,\"mir_call\":{\"callee\":{\"type\":\"Method\",\"box_name\":\"ArrayBox\",\"method\":\"len\",\"receiver\":2},\"args\":[],\"effects\":[]}}"
    MirCallV1HandlerBox.handle(len2, regs)
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
if [[ "$rc" -ne 0 ]]; then
  echo "[FAIL] rc=$rc (expect 0)" >&2; exit 1
fi
echo "[PASS] s3_vm_adapter_array_len_per_recv_canary_vm"
exit 0
