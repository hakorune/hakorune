#!/bin/bash
# Return(Logical &&/|| with Bool literal lhs/rhs) → branch+ret structure
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_return_logical_$$.hako"
cat > "$tmp_hako" <<'HAKO'
static box Main { method main(args) {
  // Return(true && false)
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  // Prefer provider-direct to avoid heavy using fragility on this host
  local a = new ArrayBox(); a.push(j)
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", a)
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Logical","op":"&&","lhs":{"type":"Bool","value":true},"rhs":{"type":"Bool","value":false}}}]}'

set +e
out="$(PROG_JSON="$PROG" HAKO_MIR_BUILDER_INTERNAL=1 run_nyash_vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" || true

mir=$(echo "$out" | awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag')
if [ -n "$mir" ] && echo "$mir" | grep -q '"op":"branch"' && echo "$mir" | grep -q '"op":"ret"'; then
  echo "[PASS] mirbuilder_internal_return_logical_canary_vm"; exit 0; fi
echo "[FAIL] mirbuilder_internal_return_logical_canary_vm" >&2; exit 1
