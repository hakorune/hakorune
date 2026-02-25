#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../../.." && pwd)
source "$SCRIPT_DIR/common.sh"

pass() { echo "✅ $1" >&2; }
fail() { echo "❌ $1" >&2; echo "$2" >&2; exit 1; }

run_pyvm_exit() {
  local src="$1" code
  set +e
  pyvm_run_inline_capture "$src" \
    "NYASH_SYNTAX_SUGAR_LEVEL=basic" \
    "NYASH_ENABLE_MAP_LITERAL=1" >/dev/null 2>&1
  code=$?
  set -e
  echo "$code"
}

# ArrayBox minimal ops: [10,20] -> set index1 to 30 -> 10+30=40
CODE=$(run_pyvm_exit $'static box Main {\n  main(args) {\n    local arr = [10, 20]\n    arr.set(1, 30)\n    return arr.get(0) + arr.get(1)\n  }\n}')
[[ "$CODE" -eq 40 ]] && pass "PyVM: ArrayBox minimal ops (exit=40)" || fail "PyVM: ArrayBox minimal ops" "__EXIT_CODE__=$CODE"

# MapBox minimal ops: size(2) + get("a")(1) => 3
CODE=$(run_pyvm_exit $'static box Main {\n  main(args) {\n    local m = %{"a" => 1, "b" => 2}\n    return m.size() + m.get("a")\n  }\n}')
[[ "$CODE" -eq 3 ]] && pass "PyVM: MapBox minimal ops (exit=3)" || fail "PyVM: MapBox minimal ops" "__EXIT_CODE__=$CODE"

echo "All PyVM collections smokes PASS" >&2
