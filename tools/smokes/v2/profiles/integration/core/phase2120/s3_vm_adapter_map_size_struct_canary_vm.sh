#!/bin/bash
# VM Adapter (Hako): Map size（構造観測）→ set が無いので rc=0 で固定
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
    // Map size without prior set → rc=0（構造観測）
    local regs = new MapBox()
    local size_seg = "{\"op\":\"mir_call\",\"dst\":9,\"mir_call\":{\"callee\":{\"type\":\"Method\",\"box_name\":\"MapBox\",\"method\":\"size\",\"receiver\":1},\"args\":[],\"effects\":[]}}"
    MirCallV1HandlerBox.handle(size_seg, regs)
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
echo "[PASS] s3_vm_adapter_map_size_struct_canary_vm"
exit 0
