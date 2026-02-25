#!/bin/bash
# array_empty_pop_tag_vm.sh — Array.pop on empty array emits [array/empty/pop] under strict

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

code='static box Main { main() { local a=[]; print(a.pop()); return 0 } }'
out=$(HAKO_OOB_STRICT=1 NYASH_OOB_STRICT=1 run_nyash_vm -c "$code")
if echo "$out" | grep -q "\[array/empty/pop\]"; then
  echo "[PASS] array_empty_pop_tag_vm"
  exit 0
else
  echo "[FAIL] array_empty_pop_tag_vm" >&2
  echo "--- output ---" >&2
  echo "$out" >&2
  exit 1
fi

