#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-hook-activation-preflight"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-hook-activation-preflight-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-114-M62-ALLOCATOR-HOOK-ACTIVATION-PREFLIGHT.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"

echo "[$TAG] checking M62 allocator hook activation preflight boundary"

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

require_file "$SSOT"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"

require_text "$SSOT" "Allocator Hook Activation Preflight (SSOT)"
require_text "$SSOT" "reentrancy guard"
require_text "$SSOT" "bootstrap allocation path"
require_text "$SSOT" "no-allocation / no-safepoint contract"
require_text "$SSOT" "rollback condition"
require_text "$SSOT" "fail-fast diagnostic"
require_text "$SSOT" "would_activate = false"
require_text "$TASKBOARD" '| `M62 allocator hook activation preflight boundary` | `live-docs` |'
require_text "$TASKBOARD" '85. `M62 allocator hook activation preflight boundary`'
require_text "$PHASE_README" '`293x-114`'
require_text "$REAL_APP_TASKBOARD" '`293x-114` M62 allocator hook activation preflight boundary'
require_text "$INDEX" "tools/checks/k2_wide_allocator_hook_activation_preflight_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_hook_activation_preflight_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_hook_activation_preflight_guard.sh"

if rg -n 'hako_alloc_(install|replace)_allocator|allocator_replacement_hook|allocator_hook_activate|activate_allocator|HakoAllocatorReplacementHook|AllocatorReplacementHookBox|AllocatorHookPlan|HookPlan' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".implementation 2>&1; then
  cat /tmp/"$TAG".implementation >&2
  rm -f /tmp/"$TAG".implementation
  fail "activation implementation symbols must stay absent in M62"
fi
rm -f /tmp/"$TAG".implementation

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M62"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_activate|activate_allocator' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

echo "[$TAG] ok"
