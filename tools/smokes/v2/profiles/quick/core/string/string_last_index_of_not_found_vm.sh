#!/bin/bash
# string_last_index_of_not_found_vm.sh — String.lastIndexOf negative (not found -> -1)

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

code='static box Main { main() { local s="hello"; print(s.lastIndexOf("@")); return 0 } }'
out=$(run_nyash_vm -c "$code")
if echo "$out" | grep -q "^-1$"; then
  echo "[PASS] string_last_index_of_not_found_vm"
else
  echo "[FAIL] string_last_index_of_not_found_vm" >&2; echo "$out" >&2; exit 1
fi

