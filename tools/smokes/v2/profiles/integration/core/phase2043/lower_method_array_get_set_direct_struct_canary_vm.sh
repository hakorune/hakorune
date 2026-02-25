#!/usr/bin/env bash
# Direct lower: LowerMethodArrayGetSetBox.try_lower → MIR(JSON) 構造検査（ArrayBox get/set）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

CODE=$(cat <<'H'
using "hako.mir.builder.internal.lower_method_array_get_set" as LowerMethodArrayGetSetBox
static box Main { method main(args) {
  local j = '{"version":0,"kind":"Program","body":[{"type":"Local","name":"a","expr":{"type":"New","class":"ArrayBox","args":[]}}, {"type":"Return","expr":{"type":"Int","value":0}}]}'
  local out = LowerMethodArrayGetSetBox.try_lower(j)
  if out == null { print("NULL"); return 1 }
  print("[MIR_OUT_BEGIN]")
  print("" + out)
  print("[MIR_OUT_END]")
  return 0
} }
H
)

set +e
OUT=$(HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 run_nyash_vm -c "$CODE" 2>&1)
RC=$?
set -e

if [ $RC -ne 0 ]; then
  echo "[FAIL] lower_method_array_get_set_direct_struct: lower run rc=$RC" >&2
  echo "$OUT" | tail -n 80 >&2
  exit 1
fi

MIR=$(echo "$OUT" | awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag')
if ! echo "$MIR" | grep -q '"method":"set"' || ! echo "$MIR" | grep -q '"method":"get"'; then
  echo "[FAIL] lower_method_array_get_set_direct_struct: expected set/get on ArrayBox" >&2
  echo "$MIR" >&2
  exit 1
fi

echo "[PASS] lower_method_array_get_set_direct_struct_canary_vm"

