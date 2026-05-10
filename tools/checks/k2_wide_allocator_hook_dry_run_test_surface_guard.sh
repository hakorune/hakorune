#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-hook-dry-run-test-surface"
cd "$ROOT_DIR"

RUNTIME_FILE="src/runtime/allocator_hook_dry_run.rs"
SSOT="docs/development/current/main/design/allocator-hook-dry-run-test-surface-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-111-M59-ALLOCATOR-HOOK-DRY-RUN-TEST-SURFACE.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M59 allocator hook dry-run test surface"

fail() {
  echo "[$TAG] ERROR: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing file: $path"
}

require_text() {
  local file="$1"
  local needle="$2"
  rg -F -q "$needle" "$file" || fail "missing text in $file: $needle"
}

require_file "$RUNTIME_FILE"
require_file "$SSOT"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$RUNTIME_FILE" "#[cfg(test)]"
require_text "$RUNTIME_FILE" "validate_allocator_hook_dry_run_reserved_fixtures_for_test"
require_text "$RUNTIME_FILE" "manifest_callsite_reports_ready_diagnostic_without_installing"
require_text "$SSOT" "Allocator Hook Dry-Run Test Surface (SSOT)"
require_text "$TASKBOARD" '| `M59 allocator hook dry-run test surface` | `live-narrow` |'
require_text "$TASKBOARD" '82. `M59 allocator hook dry-run test surface`'
require_text "$PHASE_README" '`293x-111`'
require_text "$REAL_APP_TASKBOARD" '`293x-111` M59 allocator hook dry-run test surface'
require_text "$INDEX" "tools/checks/k2_wide_allocator_hook_dry_run_test_surface_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_hook_dry_run_test_surface_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_hook_dry_run_test_surface_guard.sh"

cargo test -q allocator_hook_dry_run

if rg -n 'std::env|std::fs|read_to_string|var_os|std::alloc|GlobalAlloc|#\[global_allocator\]|malloc|realloc|free\(' \
  "$RUNTIME_FILE" >/tmp/"$TAG".forbidden_runtime 2>&1; then
  cat /tmp/"$TAG".forbidden_runtime >&2
  rm -f /tmp/"$TAG".forbidden_runtime
  fail "test surface must not add env/fs/allocator install behavior"
fi
rm -f /tmp/"$TAG".forbidden_runtime

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_activate|activate_allocator' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

echo "[$TAG] ok"
