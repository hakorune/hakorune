#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
source "$SCRIPT_DIR/common.sh"

pass() { echo "✅ $1" >&2; }
fail() { echo "❌ $1" >&2; echo "$2" | sed -n '1,160p' >&2; exit 1; }

run_pyvm_src() {
  local src="$1"
  local code
  set +e
  pyvm_run_inline_capture "$src" >/dev/null 2>&1
  code=$?
  set -e
  echo "__EXIT_CODE__=${code}"
}

OUT=$(run_pyvm_src $'static box Main {\n  main(args){ return 0 - 3 + 5 }\n}')
echo "$OUT" | rg -q '^__EXIT_CODE__=2$' && pass "arithmetic minus: 0-3+5 -> 2" || fail "arithmetic minus" "$OUT"

echo "All Stage-2 unary smokes (PyVM) PASS" >&2
