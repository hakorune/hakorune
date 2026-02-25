#!/bin/bash
# VM/Core route: String.indexOf not found -> return -1 -> exit rc=255
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

code='static box Main { method main(args) {
  local s = "abc"
  local r = s.indexOf("z")  // -1
  return r
} }'

set +e
out=$(run_nyash_vm -c "$code" 2>&1)
rc=$?
set -e

if [ "$rc" -eq 255 ]; then
  echo "[PASS] string_indexof_notfound_rc255_vm"
  exit 0
fi
echo "[FAIL] string_indexof_notfound_rc255_vm (rc=$rc, expect 255)" >&2
echo "$out" | tail -n 20 >&2
exit 1

