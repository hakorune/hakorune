#!/bin/bash
# If(Compare <=) internal canary — structure check
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

tmp="/tmp/mirbuilder_if_le_$$.hako"
cat > "$tmp" <<'HAKO'
static box Main { method main(args) {
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local a = new ArrayBox(); a.push(j)
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", a)
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

PROG='{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Compare","op":"<=","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":2}},"then":[{"type":"Return","expr":{"type":"Int","value":10}}],"else":[{"type":"Return","expr":{"type":"Int","value":20}}]}]}'

set +e; out="$(PROG_JSON="$PROG" HAKO_MIR_BUILDER_INTERNAL=1 run_nyash_vm "$tmp" 2>&1 )"; rc=$?; set -e
rm -f "$tmp" || true
mir=$(echo "$out" | awk '/\[MIR_BEGIN\]/{f=1;next}/\[MIR_END\]/{f=0}f')
if [ -n "$mir" ] && echo "$mir" | grep -q '"op":"compare"' && echo "$mir" | grep -q '"operation":"<="'; then echo "[PASS] mirbuilder_internal_if_compare_le_canary_vm"; exit 0; fi
echo "[FAIL] mirbuilder_internal_if_compare_le_canary_vm" >&2; exit 1
