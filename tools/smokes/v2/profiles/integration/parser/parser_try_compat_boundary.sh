#!/usr/bin/env bash
set -euo pipefail

BIN="${NYASH_BIN:-./target/release/hakorune}"
if [ ! -x "$BIN" ]; then
  echo "nyash binary not found: $BIN" >&2
  exit 2
fi

PROG=$(mktemp /tmp/parser_try_compat_boundary.XXXXXX.hako)
cleanup() {
  rm -f "$PROG"
}
trap cleanup EXIT

cat >"$PROG" <<'HK'
static box Main {
  main() {
    local x = 0
    try {
      x = 1
    } catch (e) {
      x = 2
    } cleanup {
      x = x + 1
    }
    return x
  }
}
HK

# Baseline: try-compat ON (default migration window) should parse.
NYASH_FEATURES=stage3 \
  "$BIN" --dump-ast "$PROG" >/dev/null 2>&1

# Contract: no-try-compat must fail-fast with parser freeze tag.
set +e
ERR=$(
  NYASH_FEATURES=stage3,no-try-compat \
    "$BIN" --dump-ast "$PROG" 2>&1 >/dev/null
)
STATUS=$?
set -e

if [ "$STATUS" -eq 0 ]; then
  echo "expected parse failure with NYASH_FEATURES=no-try-compat, but command succeeded" >&2
  exit 1
fi

if ! printf '%s' "$ERR" | rg -q "\[freeze:contract\]\[parser/try_reserved\]"; then
  echo "expected freeze tag [freeze:contract][parser/try_reserved] was not found" >&2
  printf '%s\n' "$ERR" >&2
  exit 1
fi

echo "[PASS] parser_try_compat_boundary"
