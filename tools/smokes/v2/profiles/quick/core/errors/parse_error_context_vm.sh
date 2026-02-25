#!/bin/bash
# parse_error_context_vm.sh — Ensure parse errors include filename
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

tmp="/tmp/ny_parse_err_$$.hako"
cat >"$tmp" <<'SRC'
box Main { static method main() { local ; } }
SRC
set +e
out=$(NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 "$NYASH_BIN" --backend vm "$tmp" 2>&1)
rc=$?
set -e
rm -f "$tmp"

if [ $rc -eq 0 ]; then
  echo "[FAIL] parse_error_context_vm: expected failure" >&2; exit 1
fi
echo "$out" | grep -q "Parse error in $tmp:" && echo "[PASS] parse_error_context_vm" && exit 0
echo "[FAIL] parse_error_context_vm: filename not included" >&2
echo "--- output ---" >&2
echo "$out" >&2
exit 1
