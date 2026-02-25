#!/bin/bash
# VM/Core route: String.substring(start>=len) -> "" so length()==0 -> rc=0
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

code='static box Main { method main(args) {
  local s = "hi"
  local t = s.substring(100)
  return t.length()
} }'

set +e
out=$(run_nyash_vm -c "$code" 2>&1)
rc=$?
set -e

if [ "$rc" -eq 0 ]; then
  echo "[PASS] string_substring_start_beyond_len_rc0_vm"
  exit 0
fi
echo "[FAIL] string_substring_start_beyond_len_rc0_vm (rc=$rc, expect 0)" >&2
echo "$out" | tail -n 30 >&2
exit 1

