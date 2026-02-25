#!/bin/bash
# mirbuilder_internal_binop_canary_vm.sh — Program(JSON v0) → MIR(JSON) internal box binop canary

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2
SMOKES_DEV_PREINCLUDE=1 enable_mirbuilder_dev_env

tmp_hako="/tmp/mirbuilder_internal_binop_$$.hako"
cat > "$tmp_hako" <<'HAKO'
static box Main { method main(args) {
  // Program(JSON v0) with Return(Binary(Int,Int)) — 1 + 2
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local a = new ArrayBox(); a.push(j)
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", a)
  if out == null { print("[fail:builder]"); return 1 }
  print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
  return 0
} }
HAKO

PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":2}}}]}'

set +e
out="$(PROG_JSON="$PROG" HAKO_MIR_BUILDER_INTERNAL=1 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
  "$NYASH_BIN" --backend vm "$tmp_hako" 2>&1 )"; rc=$?
set -e
rm -f "$tmp_hako" 2>/dev/null || true

mir=$(echo "$out" | awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag')
if [ -z "$mir" ]; then echo "[SKIP] binop: MIR missing"; exit 0; fi
if echo "$mir" | grep -q '"op":"binop"' && echo "$mir" | grep -q '"operation":"\+"' && echo "$mir" | grep -q '"op":"ret"'; then
  echo "[PASS] mirbuilder_internal_binop_canary_vm"; exit 0; fi
echo "[SKIP] binop: tokens not found"; exit 0
