#!/bin/bash
# last_index_not_found_vm.sh — String.lastIndexOf not found returns -1

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

code='static box Main { main() { local s="abcdef"; print(s.lastIndexOf("@")); return 0 } }'
out=$(run_nyash_vm -c "$code")
if [ "$out" = "-1" ]; then
  echo "[PASS] last_index_not_found_vm"
  exit 0
else
  echo "[FAIL] last_index_not_found_vm (got '$out')" >&2
  exit 1
fi

