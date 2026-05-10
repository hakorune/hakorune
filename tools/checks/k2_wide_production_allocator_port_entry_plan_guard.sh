#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-production-allocator-port-entry-plan"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-293x/293x-097-M45-PRODUCTION-ALLOCATOR-PORT-ENTRY-PLAN.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
HAKO_ALLOC_README="lang/src/hako_alloc/README.md"
SUBSTRATE_README="lang/src/runtime/substrate/README.md"
M44_GUARD="tools/checks/k2_wide_mimalloc_allocator_substrate_closeout_guard.sh"

echo "[$TAG] checking M45 production allocator port entry plan"

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

require_file "$CARD"
require_file "$TASKBOARD"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$CURRENT_STATE"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$HAKO_ALLOC_README"
require_file "$SUBSTRATE_README"
require_file "$M44_GUARD"

require_text "$CARD" "Production Port Meaning"
require_text "$CARD" "First Implementation Order"
require_text "$CARD" "M46 hako_alloc production facade boundary"
require_text "$CARD" "M47 allocator local page policy proof"
require_text "$CARD" "M48 allocator remote-free policy proof"
require_text "$CARD" "M49 allocator OSVM page-source proof"
require_text "$CARD" "M50 allocator stress production-facade parity"
require_text "$CARD" "must not branch on allocator app names"
require_text "$CARD" 'native pointer `fetch_add`'
require_text "$CARD" "noalias / nonnull / dereferenceable export widening"

require_text "$TASKBOARD" '| `M45 production allocator port entry plan` | `live-narrow` |'
require_text "$TASKBOARD" '| `M46 hako_alloc production facade boundary` | `live-narrow` |'
require_text "$TASKBOARD" '| `M47 allocator local page policy proof` | `live-narrow` |'
require_text "$TASKBOARD" '| `M48 allocator remote-free policy proof` | `live-narrow` |'
require_text "$TASKBOARD" '| `M49 allocator OSVM page-source proof` | `next-card` |'
require_text "$PHASE_README" '`293x-097`'
require_text "$REAL_APP_TASKBOARD" '`293x-097` M45 production allocator port entry plan'
require_text "$INDEX" "tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh"

require_text "$HAKO_ALLOC_README" "Production allocator port entry"
require_text "$HAKO_ALLOC_README" '`runtime/substrate` owns raw capability facades'
require_text "$SUBSTRATE_README" "capability substrate"

if rg -n 'mimalloc-(production|allocator-port|prod)|HakoAllocProduction|AllocatorProduction|ProductionAllocator|AllocatorRemoteFree(List|Retry)?Policy' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  fail "production allocator name/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".app_specific.inc

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".inactive_pointer_rows 2>&1; then
  cat /tmp/"$TAG".inactive_pointer_rows >&2
  rm -f /tmp/"$TAG".inactive_pointer_rows
  fail "pointer atomic fetch_add rows must stay inactive for M45"
fi
rm -f /tmp/"$TAG".inactive_pointer_rows

echo "[$TAG] ok"
