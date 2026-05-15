#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-hook-runtime-dry-run"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-hook-runtime-dry-run-ssot.md"
PLAN_SSOT="docs/development/current/main/design/allocator-hook-plan-v0-ssot.md"
PLAN_MANIFEST="docs/development/current/main/design/allocator-hook-plan-v0.toml"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-106-M54-ALLOCATOR-HOOK-RUNTIME-DRY-RUN.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
M53_GUARD="tools/checks/k2_wide_allocator_hook_plan_vocab_guard.sh"

echo "[$TAG] checking M54 allocator hook runtime dry-run boundary"

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
require_file "$PLAN_SSOT"
require_file "$PLAN_MANIFEST"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"
require_file "$M53_GUARD"

require_text "$SSOT" "Allocator Hook Runtime Dry-Run Boundary (SSOT)"
require_text "$SSOT" "[allocator-hook/dry-run-missing-plan]"
require_text "$SSOT" "runtime hook install:"
require_text "$SSOT" "absent"
require_text "$PLAN_SSOT" "No active HookPlan row exists yet."
require_text "$PLAN_MANIFEST" 'active = false'
require_text "$CARD" "M54 Allocator Hook Runtime Dry-Run Boundary"
require_text "$TASKBOARD" '| `M54 allocator hook runtime dry-run boundary` | `live-docs` |'
require_text "$TASKBOARD" '77. `M54 allocator hook runtime dry-run boundary`'
require_text "$REAL_APP_TASKBOARD" '`293x-106` M54 allocator hook runtime dry-run boundary'
require_text "$INDEX" "tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh"

if rg -n 'hako_alloc_(install|replace)_allocator|allocator_replacement_hook|HakoAllocatorReplacementHook|AllocatorReplacementHookBox|AllocatorHookPlan|HookPlan' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".implementation 2>&1; then
  cat /tmp/"$TAG".implementation >&2
  rm -f /tmp/"$TAG".implementation
  fail "allocator hook implementation symbols must stay absent in M54"
fi
rm -f /tmp/"$TAG".implementation

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M54"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'NYASH_.*ALLOC.*(HOOK|REPLACE|DRY)|HAKO_.*ALLOC.*(HOOK|DRY)|HAKORUNE_.*ALLOC.*(HOOK|DRY)' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "allocator hook environment toggles must not be introduced in M54"
fi
rm -f /tmp/"$TAG".env

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|dry_run' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add|hako_osvm_(unreserve|release)|unreserve_bytes|release_bytes' \
  src lang/c-abi/shims crates/nyash_kernel lang/src -g '!**/*.md' >/tmp/"$TAG".inactive_rows 2>&1; then
  cat /tmp/"$TAG".inactive_rows >&2
  rm -f /tmp/"$TAG".inactive_rows
  fail "inactive allocator-adjacent rows must stay inactive in M54"
fi
rm -f /tmp/"$TAG".inactive_rows

echo "[$TAG] ok"
