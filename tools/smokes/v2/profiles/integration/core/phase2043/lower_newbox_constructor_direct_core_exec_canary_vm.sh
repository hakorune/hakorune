#!/usr/bin/env bash
# Direct lower: LowerNewboxConstructorBox.try_lower → MIR(JSON) → Core exec (rc=0)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

TMPMIR="/tmp/mir_lower_new_construct_$$.json"

CODE=$(cat <<'H'
using "hako.mir.builder.internal.lower_newbox_constructor" as LowerNewboxConstructorBox
static box Main { method main(args) {
  local j = '{"version":0,"kind":"Program","body":[{"type":"Local","name":"a","expr":{"type":"New","class":"ArrayBox","args":[]}}, {"type":"Return","expr":{"type":"Int","value":0}}]}'
  local out = LowerNewboxConstructorBox.try_lower(j)
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
  echo "[FAIL] lower_newbox_constructor_direct_core_exec: lower run rc=$RC" >&2
  echo "$OUT" | tail -n 80 >&2
  exit 1
fi

MIR=$(echo "$OUT" | awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag')
if ! echo "$MIR" | grep -q '"functions"'; then
  echo "[FAIL] lower_newbox_constructor_direct_core_exec: missing MIR JSON" >&2
  echo "$OUT" | tail -n 80 >&2
  exit 1
fi

echo "$MIR" > "$TMPMIR"

set +e
"$NYASH_BIN" --mir-json-file "$TMPMIR" >/dev/null 2>&1
RC2=$?
set -e
rm -f "$TMPMIR" || true

if [ $RC2 -ne 0 ]; then
  echo "[FAIL] lower_newbox_constructor_direct_core_exec: core rc=$RC2" >&2
  exit 1
fi

echo "[PASS] lower_newbox_constructor_direct_core_exec_canary_vm"
