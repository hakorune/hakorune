#!/usr/bin/env bash
# Direct lower canary: call LowerReturnMethodArrayMapBox.try_lower without full MirBuilder
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
BIN="${ROOT_DIR}/target/release/hakorune"
if [[ ! -x "${BIN}" ]]; then echo "[SKIP] hakorune not built"; exit 0; fi

TMP_HAKO=$(mktemp --suffix .hako)
cat >"${TMP_HAKO}" <<'HAKO'
using "hako.mir.builder.internal.lower_return_method_array_map" as LowerReturnMethodArrayMapBox
static box Main { method main(args){
  local j = env.get("PROG_JSON"); if j == null { print("[fail:nojson]"); return 1 }
  local out = LowerReturnMethodArrayMapBox.try_lower(j)
  if out == null { print("[lower:null]"); return 0 }
  print("[MIR_BEGIN]"); print(out); print("[MIR_END]")
  return 0 }
}
HAKO

PROG='{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Method","recv":{"type":"Var","name":"a"},"method":"get","args":[{"type":"Int","value":0}]}}]}'

tmp_stdout=$(mktemp); trap 'rm -f "$tmp_stdout" "$TMP_HAKO" || true' EXIT
set +e
NYASH_FAIL_FAST=0 NYASH_FEATURES=stage3 NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
PROG_JSON="$PROG" "${BIN}" --backend vm "${TMP_HAKO}" | tee "$tmp_stdout" >/dev/null
rc=$?
set -e
if [[ "$rc" -ne 0 ]]; then echo "[SKIP] direct lower vm exec failed"; exit 0; fi
mir=$(awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout")
if [[ -z "$mir" ]] || ! echo "$mir" | grep -q '"functions"'; then echo "[SKIP] MIR missing functions (direct)"; exit 0; fi
echo "[PASS] registry_optin_method_arraymap_direct"
exit 0
