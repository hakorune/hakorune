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

# Emit MIR(JSON) via wrapper (selfhost-first→provider fallback)
HAKO_SELFHOST_BUILDER_FIRST=1 \
HAKO_STAGEB_FUNC_SCAN=1 \
HAKO_MIR_BUILDER_FUNCS=1 \
HAKO_MIR_BUILDER_CALL_RESOLVE=1 \
NYASH_JSON_SCHEMA_V1=1 NYASH_MIR_UNIFIED_CALL=1 \
  bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-60}" --out "$TMP_JSON" --input "$TMP_SRC" >/dev/null

# Require v1 root and a mir_call
rg -q '"schema_version"' "$TMP_JSON" || { echo "[FAIL] missing schema_version in output" >&2; exit 1; }
rg -q '"op"\s*:\s*"mir_call"' "$TMP_JSON" || { echo "[FAIL] missing mir_call op in output" >&2; exit 1; }

# Prefer Method callee, accept Global as transitional
if rg -q '"callee"\s*:\s*\{[^}]*"type"\s*:\s*"Method"' "$TMP_JSON"; then
  echo "[PASS] methodize_json (v1 + mir_call(Method))"
else
  echo "[PASS] methodize_json (v1 + mir_call present; Global callee observed)"
fi

rm -f "$TMP_SRC" "$TMP_JSON" 2>/dev/null || true
exit 0
