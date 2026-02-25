#!/bin/bash
# Test: Single-quoted strings with escape (\') in Stage-3 mode
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR"/../../../../../../../../.. && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

tmp_nyash="/tmp/test_single_quote_$$.hako"
cat > "$tmp_nyash" <<'NCODE'
local s1 = 'hello'
local s2 = 'it\'s working'
print(s2)
NCODE

set +e
# Test with Stage-3 enabled (single quotes should parse)
NYASH_FEATURES=stage3 \
  "$NYASH_BIN" --backend vm "$tmp_nyash" >/dev/null 2>&1
rc=$?
set -e

rm -f "$tmp_nyash"

# Expect successful parse and execution
if [ "$rc" -eq 0 ]; then
  echo "[PASS] parser_single_quote_canary"
  exit 0
fi
echo "[FAIL] parser_single_quote_canary (rc=$rc)" >&2
exit 1
