#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-allocator-replacement-hook-boundary"
cd "$ROOT_DIR"

DESIGN="docs/development/current/main/design/allocator-replacement-hook-boundary-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-104-M52-ALLOCATOR-REPLACEMENT-HOOK-BOUNDARY.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
HAKO_ALLOC_README="lang/src/hako_alloc/README.md"
M51_GUARD="tools/checks/k2_wide_production_allocator_port_closeout_guard.sh"

echo "[$TAG] checking M52 allocator replacement hook boundary"

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

require_file "$DESIGN"
require_file "$TASKBOARD"
require_file "$CARD"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$HAKO_ALLOC_README"
require_file "$M51_GUARD"

require_text "$DESIGN" "Allocator Replacement Hook Boundary (SSOT)"
require_text "$DESIGN" "HookPlan:"
require_text "$DESIGN" "The backend must consume facts, not source names."
require_text "$DESIGN" "M53 allocator HookPlan vocabulary lock"
require_text "$DESIGN" "M54 allocator hook runtime dry-run guard"
require_text "$DESIGN" "M55 allocator hook activation proof"
require_text "$CARD" "M52 Allocator Replacement Hook Boundary"
require_text "$CARD" "live-docs"

require_text "$TASKBOARD" '| `M52 allocator replacement hook boundary` | `live-docs` |'
require_text "$TASKBOARD" '75. `M52 allocator replacement hook boundary`'
require_text "$REAL_APP_TASKBOARD" '`293x-104` M52 allocator replacement hook boundary'
require_text "$INDEX" "tools/checks/k2_wide_allocator_replacement_hook_boundary_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_allocator_replacement_hook_boundary_guard.sh"
require_text "$HAKO_ALLOC_README" "Allocator replacement hook boundary"
require_text "$HAKO_ALLOC_README" 'does not install the process hook'

if rg -n 'hako_alloc_(install|replace)_allocator|allocator_replacement_hook|HakoAllocatorReplacementHook|AllocatorReplacementHookBox|AllocatorHookPlan|HookPlan' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".implementation 2>&1; then
  cat /tmp/"$TAG".implementation >&2
  rm -f /tmp/"$TAG".implementation
  fail "allocator replacement hook implementation symbols must stay absent in M52"
fi
rm -f /tmp/"$TAG".implementation

if rg -n '#\[global_allocator\]|GlobalAlloc' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".global_allocator 2>&1; then
  cat /tmp/"$TAG".global_allocator >&2
  rm -f /tmp/"$TAG".global_allocator
  fail "process allocator replacement must stay inactive in M52"
fi
rm -f /tmp/"$TAG".global_allocator

if rg -n 'NYASH_.*ALLOC.*(HOOK|REPLACE)|HAKO_.*ALLOC.*HOOK|HAKORUNE_.*ALLOC.*HOOK' \
  src crates lang/c-abi/shims lang/src -g '!**/*.md' >/tmp/"$TAG".env 2>&1; then
  cat /tmp/"$TAG".env >&2
  rm -f /tmp/"$TAG".env
  fail "allocator hook environment toggles must not be introduced in M52"
fi
rm -f /tmp/"$TAG".env

if rg -n 'HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|AllocatorReplacement|allocator_replacement|replace_allocator|HookPlan' \
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
  fail "inactive allocator-adjacent rows must stay inactive in M52"
fi
rm -f /tmp/"$TAG".inactive_rows

echo "[$TAG] ok"
