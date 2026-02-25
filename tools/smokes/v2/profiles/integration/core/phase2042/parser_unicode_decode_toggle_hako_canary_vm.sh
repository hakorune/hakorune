#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../../../../../.." && pwd)
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

TMP=$(mktemp)
cat >"$TMP" <<'HKO'
using "selfhost.shared.json.utils.json_frag" as JsonFragBox
static box Main { method main(args) {
  // Read JSON snippet with escapes from env (avoid parser-level escape handling)
  local t = env.get("HAKO_TEST_JSON")
  if t == null { print("nojson"); return 1 }
  local k = JsonFragBox.get_str(t, "k")
  local e = JsonFragBox.get_str(t, "e")
  print(k)
  print(e)
  return 0
} }
HKO

# OFF default: expect literals
set +e
JSON_INPUT='{"k":"\\u0041","e":"\\uD83D\\uDE00"}'
OUT_OFF=$(HAKO_TEST_JSON="$JSON_INPUT" HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 HAKO_ROUTE_HAKOVM=1 run_nyash_vm "$TMP" 2>&1)
RC_OFF=$?
set -e

if [ $RC_OFF -ne 0 ]; then
  echo "[FAIL] hako unicode toggle OFF run failed rc=$RC_OFF" >&2
  echo "$OUT_OFF" >&2
  rm -f "$TMP"; exit 1
fi

echo "$OUT_OFF" | grep -q 'u0041' || { echo "[FAIL] expected substring u0041 when OFF" >&2; echo "$OUT_OFF" >&2; rm -f "$TMP"; exit 1; }
echo "$OUT_OFF" | grep -q 'D83D' || { echo "[FAIL] expected substring D83D when OFF" >&2; echo "$OUT_OFF" >&2; rm -f "$TMP"; exit 1; }

# ON: enable Hako-side unicode decode
set +e
OUT_ON=$(HAKO_TEST_JSON="$JSON_INPUT" HAKO_PARSER_DECODE_UNICODE=1 HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 HAKO_ROUTE_HAKOVM=1 run_nyash_vm "$TMP" 2>&1)
RC_ON=$?
set -e

rm -f "$TMP" || true

if [ $RC_ON -ne 0 ]; then
  echo "[FAIL] hako unicode toggle ON run failed rc=$RC_ON" >&2
  echo "$OUT_ON" >&2
  exit 1
fi

echo "$OUT_ON" | grep -q 'A' || { echo "[FAIL] expected 'A' decoded when ON" >&2; echo "$OUT_ON" >&2; exit 1; }
# Ensure surrogate literal is gone (we accept placeholder replacement)
echo "$OUT_ON" | grep -q 'D83D' && { echo "[FAIL] surrogate literal present when ON" >&2; echo "$OUT_ON" >&2; exit 1; }

echo "[PASS] parser_unicode_decode_toggle_hako_canary_vm"
