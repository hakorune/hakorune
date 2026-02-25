#!/bin/bash
# Using alias resolution (selfhost.vm.entry) must work under quick profile
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"; if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then ROOT="$ROOT_GIT"; else ROOT="$(cd "$SCRIPT_DIR/../../../../../../../../.." && pwd)"; fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"; require_env || exit 2

# Phase S0.1: Canary tests are opt-in (SMOKES_ENABLE_SELFHOST=1)
# SSOT: docs/development/current/main/investigations/selfhost-integration-limitations.md
if [ "${SMOKES_ENABLE_SELFHOST:-0}" != "1" ]; then
  test_skip "using_alias_selfhost_vm_entry_canary_vm" "opt-in selfhost canary (SMOKES_ENABLE_SELFHOST=1). SSOT: investigations/selfhost-integration-limitations.md"
  exit 0
fi

code=$(cat <<'HCODE'
using selfhost.vm.entry as MiniVmEntryBox
static box Main { method main(args) {
  // Exercise alias resolution with a cross-box static call
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
  echo "[PASS] using_alias_selfhost_vm_entry_canary_vm"
  exit 0
fi
echo "[FAIL] using_alias_selfhost_vm_entry_canary_vm (rc=$rc)" >&2
printf '%s\n' "$out" | sed -n '1,120p' >&2
exit 1

