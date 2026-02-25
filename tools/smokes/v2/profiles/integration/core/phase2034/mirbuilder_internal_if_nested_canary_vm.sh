#!/bin/bash
# Nested If: if (1<2) return 3; else if (2<1) return 4; else return 5;
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_if_nested_$$.hako"
cat > "$tmp_hako" <<'HAKO'
static box Main { method main(args) {
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local a = new ArrayBox(); a.push(j)
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", a)
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

PROG='{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Compare","op":"<","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":2}},"then":[{"type":"Return","expr":{"type":"Int","value":3}}],"else":[{"type":"If","cond":{"type":"Compare","op":"<","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":1}},"then":[{"type":"Return","expr":{"type":"Int","value":4}}],"else":[{"type":"Return","expr":{"type":"Int","value":5}}]}]}]}'

set +e
out="$(PROG_JSON="$PROG" HAKO_MIR_BUILDER_INTERNAL=1 run_nyash_vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" || true

mir=$(echo "$out" | awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag')
if [ -n "$mir" ] && echo "$mir" | grep -q '"op":"compare"' && echo "$mir" | grep -q '"op":"branch"'; then
  echo "[PASS] mirbuilder_internal_if_nested_canary_vm"; exit 0; fi
echo "[FAIL] mirbuilder_internal_if_nested_canary_vm" >&2; exit 1
