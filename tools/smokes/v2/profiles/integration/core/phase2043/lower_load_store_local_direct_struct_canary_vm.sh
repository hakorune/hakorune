#!/usr/bin/env bash
# Direct lower: LowerLoadStoreLocalBox.try_lower → MIR(JSON) 構造検査（load/store）
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

CODE=$(cat <<'H'
using "hako.mir.builder.internal.lower_load_store_local" as LowerLoadStoreLocalBox
static box Main { method main(args) {
  local j = '{"version":0,"kind":"Program","body":[{"type":"Local","name":"x","expr":{"type":"Int","value":7}},{"type":"Local","name":"y","expr":{"type":"Var","name":"x"}},{"type":"Return","expr":{"type":"Var","name":"y"}}]}'
  local out = LowerLoadStoreLocalBox.try_lower(j)
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
  echo "[FAIL] lower_load_store_local_direct_struct: lower run rc=$RC" >&2
  echo "$OUT" | tail -n 80 >&2
  exit 1
fi

MIR=$(echo "$OUT" | awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag')
if ! echo "$MIR" | grep -q '"op":"store"' || ! echo "$MIR" | grep -q '"op":"load"'; then
  echo "[FAIL] lower_load_store_local_direct_struct: expected store/load ops" >&2
  echo "$MIR" >&2
  exit 1
fi

echo "[PASS] lower_load_store_local_direct_struct_canary_vm"
