#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-production-allocator-port-closeout"
cd "$ROOT_DIR"

TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-103-M51-PRODUCTION-ALLOCATOR-PORT-CLOSEOUT-GUARD.md"
INDEX="docs/tools/check-scripts-index.md"
DEV_GATE="tools/checks/dev_gate.sh"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
REAL_APP_TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
HAKO_ALLOC_README="lang/src/hako_alloc/README.md"
HAKO_ALLOC_MODULE="lang/src/hako_alloc/hako_module.toml"
FACADE="lang/src/hako_alloc/memory/allocator_facade_box.hako"
REMOTE_POLICY="lang/src/hako_alloc/memory/remote_free_policy_box.hako"
PAGE_SOURCE_POLICY="lang/src/hako_alloc/memory/page_source_policy_box.hako"

echo "[$TAG] checking M51 production allocator port closeout coverage"

fail() {
  echo "[$TAG] ERROR: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing file: $path"
}

require_dir() {
  local path="$1"
  [[ -d "$path" ]] || fail "missing directory: $path"
}

require_text() {
  local file="$1"
  local needle="$2"
  rg -F -q "$needle" "$file" || fail "missing text in $file: $needle"
}

require_file "$TASKBOARD"
require_file "$CARD"
require_file "$INDEX"
require_file "$DEV_GATE"
require_file "$PHASE_README"
require_file "$REAL_APP_TASKBOARD"
require_file "$HAKO_ALLOC_README"
require_file "$HAKO_ALLOC_MODULE"
require_file "$FACADE"
require_file "$REMOTE_POLICY"
require_file "$PAGE_SOURCE_POLICY"

required_apps=(
  "apps/hako-alloc-production-facade-proof"
  "apps/hako-alloc-local-page-policy-proof"
  "apps/hako-alloc-remote-free-policy-proof"
  "apps/hako-alloc-page-source-policy-proof"
  "apps/hako-alloc-production-facade-stress"
  "apps/allocator-stress"
)

required_guards=(
  "tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh"
  "tools/checks/k2_wide_hako_alloc_local_page_policy_exe_guard.sh"
  "tools/checks/k2_wide_hako_alloc_remote_free_policy_exe_guard.sh"
  "tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh"
  "tools/checks/k2_wide_hako_alloc_production_facade_stress_exe_guard.sh"
  "tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh"
)

required_cards=(
  "docs/development/current/main/phases/phase-293x/293x-098-M46-HAKO-ALLOC-PRODUCTION-FACADE-BOUNDARY.md"
  "docs/development/current/main/phases/phase-293x/293x-099-M47-ALLOCATOR-LOCAL-PAGE-POLICY-PROOF.md"
  "docs/development/current/main/phases/phase-293x/293x-100-M48-ALLOCATOR-REMOTE-FREE-POLICY-PROOF.md"
  "docs/development/current/main/phases/phase-293x/293x-101-M49-ALLOCATOR-OSVM-PAGE-SOURCE-PROOF.md"
  "docs/development/current/main/phases/phase-293x/293x-102-M50-ALLOCATOR-STRESS-PRODUCTION-FACADE-PARITY.md"
)

for app in "${required_apps[@]}"; do
  require_dir "$app"
  require_file "$app/main.hako"
  require_file "$app/README.md"
  require_file "$app/test.sh"
done

for guard in "${required_guards[@]}"; do
  require_file "$guard"
  require_text "$INDEX" "$guard"
  require_text "$DEV_GATE" "$guard"
done

for card in "${required_cards[@]}"; do
  require_file "$card"
done

require_text "$INDEX" "tools/checks/k2_wide_production_allocator_port_closeout_guard.sh"
require_text "$DEV_GATE" "tools/checks/k2_wide_production_allocator_port_closeout_guard.sh"

require_text "$TASKBOARD" '| `M46 hako_alloc production facade boundary` | `live-narrow` |'
require_text "$TASKBOARD" '| `M47 allocator local page policy proof` | `live-narrow` |'
require_text "$TASKBOARD" '| `M48 allocator remote-free policy proof` | `live-narrow` |'
require_text "$TASKBOARD" '| `M49 allocator OSVM page-source proof` | `live-narrow` |'
require_text "$TASKBOARD" '| `M50 allocator stress production-facade parity` | `live-narrow` |'
require_text "$TASKBOARD" '| `M51 production allocator port closeout guard` | `live-narrow` |'
require_text "$CARD" "M51 Production Allocator Port Closeout Guard"
require_text "$PHASE_README" '`293x-103`'
require_text "$REAL_APP_TASKBOARD" '`293x-103` M51 production allocator port closeout guard'

require_text "$HAKO_ALLOC_MODULE" 'memory.allocator_facade_box = "memory/allocator_facade_box.hako"'
require_text "$HAKO_ALLOC_MODULE" 'memory.remote_free_policy_box = "memory/remote_free_policy_box.hako"'
require_text "$HAKO_ALLOC_MODULE" 'memory.page_source_policy_box = "memory/page_source_policy_box.hako"'
require_text "$HAKO_ALLOC_README" "Production allocator port entry"
require_text "$FACADE" "box HakoAllocProductionFacade"
require_text "$REMOTE_POLICY" "static box HakoAllocRemoteFreePolicy"
require_text "$PAGE_SOURCE_POLICY" "static box HakoAllocPageSourcePolicy"

if rg -n 'hako-alloc-(production-facade|local-page-policy|remote-free-policy|page-source-policy)|HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|HakoAllocPageSourcePolicy|HakoAllocProductionFacadeStress' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  fail "production allocator app/facade/policy matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".app_specific.inc

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add|hako_osvm_(unreserve|release)|unreserve_bytes|release_bytes|replace_allocator\(|#\[global_allocator\]|GlobalAlloc' \
  src lang/c-abi/shims crates/nyash_kernel lang/src >/tmp/"$TAG".inactive_rows 2>&1; then
  cat /tmp/"$TAG".inactive_rows >&2
  rm -f /tmp/"$TAG".inactive_rows
  fail "inactive allocator rows must stay inactive after M51"
fi
rm -f /tmp/"$TAG".inactive_rows

echo "[$TAG] ok"
