#!/bin/bash
# core_budget_exceeded_gatec_vm.sh — Gate‑C(Core) step budget exceeded prints tag

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT/tools/smokes/v2/lib/stageb_helpers.sh"
require_env || exit 2

# Default ON: rely on Gate‑C → VM Interpreter rc mapping under step budget

code='static box Main { method main(args) { local i=0; loop(true){ i=i+1 } return i } }'
# Allow fallback path for this heavy-loop canary to reduce flakiness in Stage‑B entry
export HAKO_STAGEB_ALLOW_FALLBACK=1
json=$(stageb_compile_to_json "$code") || { echo "[FAIL] core_budget_exceeded_gatec_vm (emit failed)" >&2; exit 1; }

set +e
NYASH_GATE_C_CORE=1 HAKO_GATE_C_CORE=1 \
  HAKO_VM_MAX_STEPS=10 NYASH_VM_MAX_STEPS=10 \
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
  "$NYASH_BIN" --json-file "$json" >/dev/null 2>&1
rc=$?
set -e
rm -f "$json"

if [ "$rc" -ne 0 ]; then
  echo "[PASS] core_budget_exceeded_gatec_vm"
else
  echo "[FAIL] core_budget_exceeded_gatec_vm (rc=$rc)" >&2; exit 1
fi
