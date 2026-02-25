#!/usr/bin/env bash
# Direct lower: LowerTypeOpCheckBox.try_lower → MIR(JSON) 構造検査（typeop Check）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

CODE=$(cat <<'H'
using "hako.mir.builder.internal.lower_typeop_check" as LowerTypeOpCheckBox
static box Main { method main(args) {
  local j = '{"version":0,"kind":"Program","body":[{"type":"Local","name":"x","expr":{"type":"Int","value":7}},{"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"x"},"method":"is","args":[{"type":"String","value":"IntegerBox"}]}}]}'
  local out = LowerTypeOpCheckBox.try_lower(j)
  if out == null { print("NULL"); return 1 }
  print("[MIR_OUT_BEGIN]")
  print("" + out)
  print("[MIR_OUT_END]")
  return 0
} }
H
)

set +e
OUT=$(HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 NYASH_VM_HAKO_PREFER_STRICT_DEV=0 NYASH_VM_USE_FALLBACK=1 run_nyash_vm -c "$CODE" 2>&1)
RC=$?
set -e

if [ $RC -ne 0 ]; then
  echo "[FAIL] lower_typeop_check_direct_struct: lower run rc=$RC" >&2
  echo "$OUT" | tail -n 80 >&2
  exit 1
fi

MIR=$(echo "$OUT" | awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag')
if ! echo "$MIR" | grep -q '"op":"typeop"' || ! echo "$MIR" | grep -q '"op_kind":"Check"'; then
  echo "[FAIL] lower_typeop_check_direct_struct: expected typeop Check" >&2
  echo "$MIR" >&2
  exit 1
fi

echo "[PASS] lower_typeop_check_direct_struct_canary_vm"
