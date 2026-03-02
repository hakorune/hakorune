#!/usr/bin/env bash
set -euo pipefail

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$ROOT"

TMP_SRC=$(mktemp --suffix .hako)
cat >"$TMP_SRC" <<'HAKO'
static box Main {
  method add(a,b){ return a + b }
  method main(){ return add(2,3) }
}
HAKO

TMP_JSON=$(mktemp --suffix .json)

# Force methodize checks on direct v1 route
HAKO_STAGEB_FUNC_SCAN=1 \
HAKO_MIR_BUILDER_FUNCS=1 \
HAKO_MIR_BUILDER_CALL_RESOLVE=1 \
HAKO_MIR_BUILDER_METHODIZE=1 \
NYASH_JSON_SCHEMA_V1=1 NYASH_MIR_UNIFIED_CALL=1 \
  bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route direct --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$TMP_JSON" --input "$TMP_SRC" >/dev/null

# Require v1 + mir_call(Method) strictly
rg -q '"schema_version"' "$TMP_JSON" || { echo "[FAIL] missing schema_version in output" >&2; exit 1; }
rg -q '"op"\s*:\s*"mir_call"' "$TMP_JSON" || { echo "[FAIL] missing mir_call op in output" >&2; exit 1; }
rg -q '"type"\s*:\s*"Method"' "$TMP_JSON" || { echo "[FAIL] missing Method callee in mir_call" >&2; exit 1; }
rg -q '"box_name"\s*:\s*"Main"' "$TMP_JSON" || { echo "[FAIL] missing Main method call target" >&2; exit 1; }
rg -q '"name"\s*:\s*"add"' "$TMP_JSON" || { echo "[FAIL] missing add method call target" >&2; exit 1; }

echo "[PASS] methodize_json_strict (v1 + mir_call(Method))"

rm -f "$TMP_SRC" "$TMP_JSON" 2>/dev/null || true
exit 0
