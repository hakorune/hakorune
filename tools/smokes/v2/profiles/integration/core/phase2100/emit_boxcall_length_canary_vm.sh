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
if ! NYASH_JSON_ONLY=1 bash "$ROOT/tools/smokes/v2/lib/emit_mir_route.sh" --route hako-helper --timeout-secs "${HAKO_BUILD_TIMEOUT:-10}" --out "$TMP_JSON" --input "$TMP_HAKO" >/dev/null 2>&1; then
  echo "[FAIL] emit_boxcall_length: emit-mir-json failed"; exit 1
fi

# Expect length call in either legacy boxcall or unified mir_call form.
if rg -n '"op"\s*:\s*"boxcall"' "$TMP_JSON" >/dev/null 2>&1; then
  if ! rg -n '"method"\s*:\s*"length"' "$TMP_JSON" >/dev/null 2>&1; then
    echo "[FAIL] emit_boxcall_length: boxcall method length not found"; head -n 120 "$TMP_JSON" >&2; exit 1
  fi
elif rg -n '"op"\s*:\s*"mir_call"' "$TMP_JSON" >/dev/null 2>&1; then
  if ! rg -n '"name"\s*:\s*"length"' "$TMP_JSON" >/dev/null 2>&1; then
    echo "[FAIL] emit_boxcall_length: mir_call callee length not found"; head -n 120 "$TMP_JSON" >&2; exit 1
  fi
else
  echo "[FAIL] emit_boxcall_length: neither boxcall nor mir_call found in MIR JSON"; head -n 120 "$TMP_JSON" >&2; exit 1
fi

echo "[PASS] emit_boxcall_length_canary_vm"
exit 0
