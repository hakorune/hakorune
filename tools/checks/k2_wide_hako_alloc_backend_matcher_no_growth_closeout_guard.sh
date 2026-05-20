#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-backend-matcher-no-growth-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

CARD_352A="docs/development/current/main/phases/phase-293x/293x-968-MIMAP-352A-PROVIDER-INACTIVE-BOUNDARY-INVENTORY.md"
CARD_353A="docs/development/current/main/phases/phase-293x/293x-969-MIMAP-353A-POST-PROVIDER-INACTIVE-BOUNDARY-ROW-SELECTION.md"
CARD="docs/development/current/main/phases/phase-293x/293x-970-MIMAP-354A-BACKEND-MATCHER-NO-GROWTH-CLOSEOUT.md"
NEXT_CARD="docs/development/current/main/phases/phase-293x/293x-971-MIMAP-355A-POST-BACKEND-MATCHER-NO-GROWTH-ROW-SELECTION.md"
DESIGN="docs/development/current/main/design/hako-alloc-backend-matcher-no-growth-closeout-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROVIDER_GUARD="tools/checks/k2_wide_hako_alloc_provider_inactive_boundary_inventory_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_backend_matcher_no_growth_closeout_guard.sh"

printf '[%s] checking MIMAP-354A backend matcher no-growth closeout\n' "$TAG"

guard_require_files "$TAG" "$CARD_352A" "$CARD_353A" "$CARD" "$NEXT_CARD" "$DESIGN" "$INDEX" "$PROVIDER_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$PROVIDER_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD_352A" "MIMAP-352A provider inactive boundary must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD_353A" "MIMAP-353A row-selection card must be landed"
guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-354A card must be landed"
guard_expect_in_file "$TAG" 'Status: (selected current|landed)' "$NEXT_CARD" "MIMAP-355A must be selected current or landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-354A design must be accepted"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-354A guard"

bash "$PROVIDER_GUARD" --level L2

patterns=(
  'hako-alloc-segment-arena-backing-no-escape-pointer-residence-pilot-proof'
  'HakoAllocSegmentArenaBackingNoEscapePointerResidencePilot'
  'hako-alloc-segment-arena-backing-handle-pilot-proof'
  'HakoAllocSegmentArenaBackingHandlePilot'
  'hako-alloc-segment-arena-backing-pointer-derived-lookup-execution-pilot-proof'
  'HakoAllocSegmentArenaBackingPointerDerivedLookupExecutionPilot'
  'hako-alloc-segment-map-mutation-pilot-proof'
  'HakoAllocSegmentMapMutationPilot'
  'hako-alloc-atomic-bitmap-pilot-proof'
  'HakoAllocAtomicBitmapPilot'
  'hako-alloc-osvm-page-source-pilot-proof'
  'HakoAllocOSVMPageSourcePilot'
  'hako-alloc-worker-tls-pilot-proof'
  'HakoAllocWorkerTlsPilot'
  'hako-alloc-provider-inactive-boundary-inventory-proof'
  'HakoAllocProviderInactiveBoundaryInventory'
)

for pattern in "${patterns[@]}"; do
  if rg -n -F "$pattern" lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
    echo "[$TAG] ERROR: allocator owner/app matcher leaked into .inc: $pattern" >&2
    cat /tmp/"$TAG".inc_leak >&2
    rm -f /tmp/"$TAG".inc_leak
    exit 1
  fi
done
rm -f /tmp/"$TAG".inc_leak

if rg -n 'providerActivate|replace_process_allocator|install_hook|global_allocator|owner-name matcher|by app name|by owner name' lang/c-abi/shims >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: provider/replacement/hook/backend owner-name matcher wording leaked into .inc" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

printf '[%s] ok\n' "$TAG"
