#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
source "$ROOT/tools/smokes/v2/lib/test_runner.sh" || true

require_env || exit 1

TMP_HAKO=$(mktemp --suffix .hako)
TMP_JSON=$(mktemp --suffix .json)
trap 'rm -f "$TMP_HAKO" "$TMP_JSON" 2>/dev/null || true' EXIT

cat >"$TMP_HAKO" <<'HAKO'
static box Main { method main(args){
  local s = new StringBox("nyash");
  return s.length()
} }
HAKO

# Emit MIR JSON via selfhost wrapper (quiet JSON)
if ! NYASH_JSON_ONLY=1 timeout "${HAKO_BUILD_TIMEOUT:-10}" bash "$ROOT/tools/hakorune_emit_mir.sh" "$TMP_HAKO" "$TMP_JSON" >/dev/null 2>&1; then
  echo "[FAIL] emit_boxcall_length: emit-mir-json failed"; exit 1
fi

# Expect a boxcall length
if ! rg -n '"op"\s*:\s*"boxcall"' "$TMP_JSON" >/dev/null 2>&1; then
  echo "[FAIL] emit_boxcall_length: boxcall not found in MIR JSON"; head -n 120 "$TMP_JSON" >&2; exit 1
fi
if ! rg -n '"method"\s*:\s*"length"' "$TMP_JSON" >/dev/null 2>&1; then
  echo "[FAIL] emit_boxcall_length: method length not found"; head -n 120 "$TMP_JSON" >&2; exit 1
fi

echo "[PASS] emit_boxcall_length_canary_vm"
exit 0
