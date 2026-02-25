#!/bin/bash
# vm_budget_exceeded_vm.sh — VM step budget exceeded prints clear message

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

code='static box Main { main() { local i=0; loop(true){ i=i+1 } return i } }'
set +e
out=$(HAKO_VM_MAX_STEPS=10 NYASH_VM_MAX_STEPS=10 run_nyash_vm -c "$code")
rc=$?
set -e
if echo "$out" | grep -q "vm step budget exceeded"; then
  echo "[PASS] vm_budget_exceeded_vm"
else
  echo "[FAIL] vm_budget_exceeded_vm" >&2; echo "$out" >&2; exit 1
fi

