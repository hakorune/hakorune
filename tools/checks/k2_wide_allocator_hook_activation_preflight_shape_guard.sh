#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-hook-activation-preflight-shape"
cd "$ROOT_DIR"

RUNTIME_FILE="src/runtime/allocator_hook_dry_run.rs"
SSOT="docs/development/current/main/design/allocator-hook-activation-preflight-shape-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-115-M63-ALLOCATOR-HOOK-ACTIVATION-PREFLIGHT-SHAPE.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M63 allocator hook activation preflight shape"

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
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$RUNTIME_FILE" "AllocatorHookActivationPreflightFacts"
require_text "$RUNTIME_FILE" "AllocatorHookActivationPreflightReport"
require_text "$RUNTIME_FILE" "validate_allocator_hook_activation_preflight"
require_text "$RUNTIME_FILE" "validate_allocator_hook_activation_preflight_from_manifest_texts"
require_text "$RUNTIME_FILE" "DIAG_ACTIVATION_PREFLIGHT_READY"
require_text "$RUNTIME_FILE" "DIAG_ACTIVATION_PREFLIGHT_MISSING"
require_text "$RUNTIME_FILE" "missing_facts"
require_text "$RUNTIME_FILE" "would_activate: false"
require_text "$RUNTIME_FILE" "activation_preflight_fixture_reports_ready_without_activating"
require_text "$RUNTIME_FILE" "activation_preflight_direct_facts_report_missing_names"
require_text "$SSOT" "Allocator Hook Activation Preflight Shape (SSOT)"
require_text "$SSOT" "missing_facts"
require_text "$TASKBOARD" '| `M63 allocator hook activation preflight shape` | `live-narrow` |'
require_text "$TASKBOARD" '86. `M63 allocator hook activation preflight shape`'
require_text "$REAL_APP_TASKBOARD" '`293x-115` M63 allocator hook activation preflight shape'
require_text "$INDEX" "tools/checks/k2_wide_allocator_hook_activation_preflight_shape_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_hook_activation_preflight_shape_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_hook_activation_preflight_shape_guard.sh"

cargo test -q allocator_hook_dry_run

if rg -n 'std::env|std::fs|read_to_string|var_os|std::alloc|GlobalAlloc|#\[global_allocator\]|malloc|realloc|free\(' \
  "$RUNTIME_FILE" >/tmp/"$TAG".forbidden_runtime 2>&1; then
  cat /tmp/"$TAG".forbidden_runtime >&2
  rm -f /tmp/"$TAG".forbidden_runtime
  fail "preflight shape must not add env/fs/allocator replacement behavior"
fi
rm -f /tmp/"$TAG".forbidden_runtime

if rg -n 'allocator_hook_activate|activate_allocator|HakoAllocatorReplacementHook|AllocatorReplacementHookBox|AllocatorHookPlan|HookPlan' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".implementation 2>&1; then
  cat /tmp/"$TAG".implementation >&2
  rm -f /tmp/"$TAG".implementation
  fail "activation implementation symbols must stay absent in M63"
fi
rm -f /tmp/"$TAG".implementation

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_activate|activate_allocator' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

echo "[$TAG] ok"
