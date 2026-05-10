#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-hook-runtime-owner"
cd "$ROOT_DIR"

SSOT="docs/development/current/main/design/allocator-hook-runtime-owner-ssot.md"
ACTIVATION_SSOT="docs/development/current/main/design/allocator-hook-activation-proof-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-108-M56-ALLOCATOR-HOOK-RUNTIME-OWNER.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GROUP="tools/checks/k2_wide_allocator_gate.sh"
M55_GUARD="tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh"

echo "[$TAG] checking M56 allocator hook runtime owner row"

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
require_file "$ACTIVATION_SSOT"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$ALLOCATOR_GROUP"
require_file "$M55_GUARD"

require_text "$SSOT" "Allocator Hook Runtime Owner (SSOT)"
require_text "$SSOT" "src/runtime/allocator_hook_dry_run.rs"
require_text "$SSOT" "runtime dry-run code:"
require_text "$SSOT" "absent"
require_text "$ACTIVATION_SSOT" "[allocator-hook/activation-proof-missing]"
require_text "$CARD" "M56 Allocator Hook Runtime Owner"
require_text "$TASKBOARD" '| `M56 allocator hook runtime owner row` | `live-docs` |'
require_text "$TASKBOARD" '79. `M56 allocator hook runtime owner row`'
require_text "$PHASE_README" '`293x-108`'
require_text "$REAL_APP_TASKBOARD" '`293x-108` M56 allocator hook runtime owner row'
require_text "$CURRENT_STATE" 'latest_card = "293x-108-M56-ALLOCATOR-HOOK-RUNTIME-OWNER"'
require_text "$INDEX" "tools/checks/k2_wide_allocator_hook_runtime_owner_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_hook_runtime_owner_guard.sh"
require_text "$ALLOCATOR_GROUP" "tools/checks/k2_wide_allocator_hook_runtime_owner_guard.sh"

[[ ! -f src/runtime/allocator_hook_dry_run.rs ]] || fail "runtime owner path is named but must stay unimplemented in M56"

if rg -n 'hako_alloc_(install|replace)_allocator|allocator_replacement_hook|allocator_hook_(dry_run|activate)|HakoAllocatorReplacementHook|AllocatorReplacementHookBox|AllocatorHookPlan|HookPlan' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".implementation 2>&1; then
  cat /tmp/"$TAG".implementation >&2
  rm -f /tmp/"$TAG".implementation
  fail "allocator hook implementation symbols must stay absent in M56"
fi
rm -f /tmp/"$TAG".implementation

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M56"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'NYASH_.*ALLOC.*(HOOK|REPLACE|DRY|ACTIVATE)|HAKO_.*ALLOC.*(HOOK|DRY|ACTIVATE)|HAKORUNE_.*ALLOC.*(HOOK|DRY|ACTIVATE)' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "allocator hook environment toggles must not be introduced in M56"
fi
rm -f /tmp/"$TAG".env

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan|allocator_hook_dry_run|allocator_hook_activate|activate_allocator' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  fail "allocator hook/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc

echo "[$TAG] ok"
