#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../.." && pwd)
source "$SCRIPT_DIR/common.sh"

run() {
  pyvm_run_inline_capture $'static box Main {\n  main(args) {\n    local x = 1\n    local arr = [x, 2, 3]\n    return arr.size()\n  }\n}' \
    "NYASH_SYNTAX_SUGAR_LEVEL=basic"
}

set +e
OUT=$(run 2>&1)
CODE=$?
set -e
[[ "$CODE" -eq 3 ]] && echo "✅ PyVM: array literal basic (exit=3)" || {
  echo "❌ PyVM: array literal basic"
  echo "__EXIT_CODE__=$CODE"
  echo "$OUT"
  exit 1
}

echo "Array literal smoke PASS" >&2
