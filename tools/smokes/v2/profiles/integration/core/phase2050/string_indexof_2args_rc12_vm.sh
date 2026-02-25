#!/bin/bash
# VM/Core route: String.indexOf(search, fromIndex) -> rc=12
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

code='static box Main { method main(args) {
  local s = "hello world hello"
  local r = s.indexOf("hello", 1)
  print("" + r)
  return r
} }'

set +e
out=$(run_nyash_vm -c "$code" 2>&1)
rc=$?
set -e

if [ "$rc" -eq 12 ]; then
  echo "[PASS] string_indexof_2args_rc12_vm"
  exit 0
fi
echo "[FAIL] string_indexof_2args_rc12_vm (rc=$rc, expect 12)" >&2
echo "$out" | tail -n 30 >&2
exit 1

