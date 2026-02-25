#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../.." && pwd)
source "$SCRIPT_DIR/common.sh"

run() {
  pyvm_run_inline_capture $'static box Main {\n  main(args) {\n    local m = %{"name" => "Alice", "age" => 25}\n    return m.size()\n  }\n}' \
    "NYASH_SYNTAX_SUGAR_LEVEL=basic" \
    "NYASH_ENABLE_MAP_LITERAL=1"
}

set +e
OUT=$(run 2>&1)
CODE=$?
set -e
[[ "$CODE" -eq 2 ]] && echo "✅ PyVM: map literal basic (exit=2)" || {
  echo "❌ PyVM: map literal basic"
  echo "__EXIT_CODE__=$CODE"
  echo "$OUT"
  exit 1
}

echo "Map literal smoke PASS" >&2
