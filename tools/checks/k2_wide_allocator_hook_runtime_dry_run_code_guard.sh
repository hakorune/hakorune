#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-hook-runtime-dry-run-code"
cd "$ROOT_DIR"

RUNTIME_FILE="src/runtime/allocator_hook_dry_run.rs"
RUNTIME_MOD="src/runtime/mod.rs"
OWNER_SSOT="docs/development/current/main/design/allocator-hook-runtime-owner-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-109-M57-ALLOCATOR-HOOK-RUNTIME-DRY-RUN-CODE.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M57 allocator hook runtime dry-run code"

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
require_file "$RUNTIME_MOD"
require_file "$OWNER_SSOT"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$RUNTIME_MOD" "pub mod allocator_hook_dry_run;"
require_text "$RUNTIME_FILE" "validate_allocator_hook_dry_run"
require_text "$RUNTIME_FILE" "DIAG_DRY_RUN_MISSING_PLAN"
require_text "$RUNTIME_FILE" "DIAG_ACTIVATION_PROOF_MISSING"
require_text "$RUNTIME_FILE" "would_install: false"
require_text "$RUNTIME_FILE" "dry_run_ready_is_still_diagnostic_only"
require_text "$OWNER_SSOT" "src/runtime/allocator_hook_dry_run.rs"
require_text "$TASKBOARD" '| `M57 allocator hook runtime dry-run code` | `live-narrow` |'
require_text "$TASKBOARD" '80. `M57 allocator hook runtime dry-run code`'
require_text "$PHASE_README" '`293x-109`'
require_text "$REAL_APP_TASKBOARD" '`293x-109` M57 allocator hook runtime dry-run code'
require_text "$INDEX" "tools/checks/k2_wide_allocator_hook_runtime_dry_run_code_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_hook_runtime_dry_run_code_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_hook_runtime_dry_run_code_guard.sh"

cargo test -q allocator_hook_dry_run

if rg -n 'std::env|var_os|std::alloc|GlobalAlloc|#\[global_allocator\]|malloc|realloc|free\(' \
  "$RUNTIME_FILE" >/tmp/"$TAG".forbidden_runtime 2>&1; then
  cat /tmp/"$TAG".forbidden_runtime >&2
  rm -f /tmp/"$TAG".forbidden_runtime
  fail "dry-run runtime file must stay diagnostic-only"
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
