#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../../../../../.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"

if [ ! -x "$BIN" ]; then
  cargo build --release >/dev/null
fi

TMP=$(mktemp)
cat >"$TMP" <<'NY'
static box Main {
  method main(args) {
    print("\u0041")    // 'A' when decoded
    print("\uD83D\uDE00") // 😀 when decoded
    return 0
  }
}
NY

# OFF (default): expect literal sequences
set +e
OUT_OFF=$("$BIN" --backend vm "$TMP" 2>&1)
RC_OFF=$?
set -e

if [ $RC_OFF -ne 0 ]; then
  echo "[FAIL] unicode toggle OFF run failed rc=$RC_OFF" >&2
  echo "$OUT_OFF" >&2
  rm -f "$TMP"
  exit 1
fi

echo "$OUT_OFF" | grep -q '\\u0041' || { echo "[FAIL] expected literal \\u0041 when OFF" >&2; echo "$OUT_OFF" >&2; rm -f "$TMP"; exit 1; }
echo "$OUT_OFF" | grep -q '\\uD83D\\uDE00' || { echo "[FAIL] expected literal surrogate when OFF" >&2; echo "$OUT_OFF" >&2; rm -f "$TMP"; exit 1; }

# ON: enable decode, expect actual characters
set +e
OUT_ON=$(NYASH_PARSER_DECODE_UNICODE=1 "$BIN" --backend vm "$TMP" 2>&1)
RC_ON=$?
set -e

rm -f "$TMP" || true

if [ $RC_ON -ne 0 ]; then
  echo "[FAIL] unicode toggle ON run failed rc=$RC_ON" >&2
  echo "$OUT_ON" >&2
  exit 1
fi

echo "$OUT_ON" | grep -q 'A' || { echo "[FAIL] expected 'A' when ON" >&2; echo "$OUT_ON" >&2; exit 1; }
# The emoji may be present; just ensure the surrogate literal is gone
echo "$OUT_ON" | grep -q '\\uD83D\\uDE00' && { echo "[FAIL] surrogate literal present when ON" >&2; echo "$OUT_ON" >&2; exit 1; }

echo "[PASS] parser_unicode_decode_toggle_canary"

