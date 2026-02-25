#!/bin/bash
# If(Compare Var vs Var) with prior Local Ints → compare+branch+ret
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_if_varvar_$$.hako"
cat > "$tmp_hako" <<'HAKO'
static box Main { method main(args) {
  // Local a=1, b=2; if (a < b) return 7; else return 9;
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local a = new ArrayBox(); a.push(j)
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", a)
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

PROG='{"version":0,"kind":"Program","body":[{"type":"Local","name":"a","expr":{"type":"Int","value":1}},{"type":"Local","name":"b","expr":{"type":"Int","value":2}},{"type":"If","cond":{"type":"Compare","op":"<","lhs":{"type":"Var","name":"a"},"rhs":{"type":"Var","name":"b"}},"then":[{"type":"Return","expr":{"type":"Int","value":7}}],"else":[{"type":"Return","expr":{"type":"Int","value":9}}]}]}'

set +e
out="$(PROG_JSON="$PROG" HAKO_MIR_BUILDER_INTERNAL=1 run_nyash_vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" || true

mir=$(echo "$out" | awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag')
if [ -n "$mir" ] && echo "$mir" | grep -q '"op":"compare"' && echo "$mir" | grep -q '"op":"branch"'; then
  echo "[PASS] mirbuilder_internal_if_compare_varvar_canary_vm"; exit 0; fi
echo "[FAIL] mirbuilder_internal_if_compare_varvar_canary_vm" >&2; exit 1
