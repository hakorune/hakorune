#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../.." && pwd)
source "$SCRIPT_DIR/common.sh"

run() {
  pyvm_run_inline_capture $'static box Main {\n  main(args) {\n    local m = %{name => "A", age => 2}\n    return m.size()\n  }\n}' \
    "NYASH_SYNTAX_SUGAR_LEVEL=full" \
    "NYASH_ENABLE_MAP_LITERAL=1"
}

set +e
OUT=$(run 2>&1)
CODE=$?
set -e
if [[ "$CODE" -ne 0 ]] && echo "$OUT" | rg -q 'expected string key in `%\{\.\.\.\}` map literal'; then
  echo "✅ PyVM: map literal ident-key is rejected (current contract)"
else
  echo "❌ PyVM: map literal ident-key contract mismatch"
  echo "__EXIT_CODE__=$CODE"
  echo "$OUT"
  exit 1
fi

echo "Map literal ident-key smoke PASS" >&2
