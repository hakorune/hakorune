#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-provider-real-entrypoint-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_67="docs/development/current/main/phases/phase-296x/296x-67-HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-SELECTION.md"
CARD_68="docs/development/current/main/phases/phase-296x/296x-68-HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_provider_real_entrypoint_selection.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_provider_real_entrypoint_selection_guard.sh"
SURFACE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"

echo "[$TAG] checking phase-296x hako mimalloc provider real entrypoint selection"

guard_require_files "$TAG" "$CARD_67" "$CARD_68" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT" "$SURFACE"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_67" "real entrypoint selection card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_68" "real entrypoint pilot card must be current"
guard_expect_fixed_in_file "$TAG" 'selected_entrypoint=object_lifecycle_small_alloc_release_v0' "$CARD_67" "card must record selected entrypoint"
guard_expect_fixed_in_file "$TAG" 'selected_surface_owner=HakoAllocObjectLifecycleFacade' "$CARD_67" "card must record selected owner"
guard_expect_fixed_in_file "$TAG" 'provider_call_allowed=1' "$CARD_67" "card must allow explicit provider call"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_67" "card must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$CARD_67" "card must keep hooks closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$CARD_67" "card must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_67" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'objectLifecycleSmallAlloc(size)' "$SURFACE" "selected surface must expose alloc method"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleReleaseBlock(page_id, block_id)' "$SURFACE" "selected surface must expose release method"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleAllocPageId()' "$SURFACE" "selected surface must expose page id observer"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleAllocBlockId()' "$SURFACE" "selected surface must expose block id observer"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-67-HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-SELECTION"' "$CURRENT_STATE" "current state latest card must advance to row 67"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT-296X-001"' "$CURRENT_STATE" "current state must select row 68 pilot"
guard_expect_fixed_in_file "$TAG" '| 67 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 67 must be landed"
guard_expect_fixed_in_file "$TAG" '| 68 | `HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT-296X-001` | Current |' "$TASKBOARD" "taskboard row 68 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list entrypoint tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_entrypoint_selection.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --repo-root "$ROOT_DIR" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-provider-real-entrypoint-selection-v0' "$report" "tool must emit selection contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=hako-mimalloc-port-feature-gap-inventory-v0' "$report" "tool must consume inventory contract"
guard_expect_fixed_in_file "$TAG" 'selected_entrypoint=object_lifecycle_small_alloc_release_v0' "$report" "tool must select object lifecycle entrypoint"
guard_expect_fixed_in_file "$TAG" 'selected_surface_owner=HakoAllocObjectLifecycleFacade' "$report" "tool must select object lifecycle owner"
guard_expect_fixed_in_file "$TAG" 'selected_alloc_method=objectLifecycleSmallAlloc' "$report" "tool must select alloc method"
guard_expect_fixed_in_file "$TAG" 'selected_release_method=objectLifecycleReleaseBlock' "$report" "tool must select release method"
guard_expect_fixed_in_file "$TAG" 'provider_call_allowed=1' "$report" "tool must allow explicit provider call"
guard_expect_fixed_in_file "$TAG" 'provider_active=0' "$report" "tool must keep provider inactive"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'hook_installed=0' "$report" "tool must keep hook closed"
guard_expect_fixed_in_file "$TAG" 'global_allocator=0' "$report" "tool must keep global allocator closed"
guard_expect_fixed_in_file "$TAG" 'ld_preload_shim_ready=0' "$report" "tool must keep LD_PRELOAD later"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'next_row=HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT-296X-001' "$report" "tool must select pilot row"
guard_expect_fixed_in_file "$TAG" 'rejected_0_entrypoint=production_facade_basic_alloc_release_v0' "$report" "tool must reject legacy production facade for this row"
guard_expect_fixed_in_file "$TAG" 'rejected_1_entrypoint=ld_preload_malloc_free_v0' "$report" "tool must keep LD_PRELOAD later"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
