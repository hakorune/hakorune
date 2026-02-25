#!/bin/bash
# Using alias resolution via workspace alias (hakorune.vm.entry)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

code=$(cat <<'HCODE'
using hakorune.vm.entry as MiniVmEntryBox
static box Main { method main(args) {
  // workspace alias での解決確認（静的メソッド呼び出し）
  local _s = MiniVmEntryBox.int_to_str(0)
  return 0
} }
HCODE
)

set +e
out=$(NYASH_USING_AST=1 run_nyash_vm -c "$code" 2>&1)
rc=$?
set -e

if [ "$rc" -eq 0 ]; then
  echo "[PASS] using_alias_workspace_entry_canary_vm"
  exit 0
fi
echo "[FAIL] using_alias_workspace_entry_canary_vm (rc=$rc)" >&2
printf '%s\n' "$out" | sed -n '1,120p' >&2
exit 1

