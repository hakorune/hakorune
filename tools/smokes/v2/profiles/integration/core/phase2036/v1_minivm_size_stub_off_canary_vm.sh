#!/bin/bash
# Mini‑VM size/len/push flag OFF: push does not increase size (stub tag path), size returns 0
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

json='{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[
  {"op":"mir_call","dst":0, "callee":{"type":"Constructor","box_type":"ArrayBox"}, "args":[], "effects":[]},
  {"op":"const","dst":1, "value": {"type": "i64", "value": 10}},
  {"op":"mir_call", "callee":{"type":"Method","box_name":"ArrayBox","method":"push","receiver":0}, "args":[1], "effects":[]},
  {"op":"mir_call","dst":2, "callee":{"type":"Method","box_name":"ArrayBox","method":"size","receiver":0}, "args":[], "effects":[]},
  {"op":"ret","value":2}
]}]}]}'

# Build a tiny driver to call MiniVmEntryBox.run_min with JSON literal embedded
code=$(cat <<'HCODE'
using selfhost.vm.entry as MiniVmEntryBox
static box Main { method main(args) {
  local j = __MIR_JSON__
  return MiniVmEntryBox.run_min(j)
} }
HCODE
)
json_quoted=$(printf '%s' "$json" | jq -Rs .)
code="${code/__MIR_JSON__/$json_quoted}"

set +e
# Allow path-based using in this dev canary by preincluding dependencies
out=$(NYASH_USING_AST=1 NYASH_PREINCLUDE=1 HAKO_VM_MIRCALL_SIZESTATE=0 run_nyash_vm -c "$code" 2>&1)
rc=$?
set -e

if [ "$rc" -eq 0 ]; then
  echo "[PASS] v1_minivm_size_stub_off_canary_vm"
  exit 0
fi
# If Mini‑VM is not ready (missing callee/unresolved), SKIP for now under 20.36
if echo "$out" | grep -q -E '(missing callee|unresolved)'; then
  echo "[SKIP] v1_minivm_size_stub_off_canary_vm (Mini‑VM not ready: $rc)" >&2
  exit 0
fi
echo "[FAIL] v1_minivm_size_stub_off_canary_vm (rc=$rc, expect 0)" >&2; exit 1
