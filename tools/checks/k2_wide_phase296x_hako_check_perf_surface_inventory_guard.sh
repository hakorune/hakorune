#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-check-perf-surface-inventory"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_77="docs/development/current/main/phases/phase-296x/296x-77-HAKO-CHECK-PERF-SURFACE-INVENTORY.md"
CARD_78="docs/development/current/main/phases/phase-296x/296x-78-HAKO-MIMALLOC-PERF-RELEASE-KNOWN-PAGE-FAST-PATH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/hako_check/perf_surface_inventory.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_check_perf_surface_inventory_guard.sh"
TARGET="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"

echo "[$TAG] checking hako_check perf-surface inventory"

guard_require_files "$TAG" "$CARD_77" "$CARD_78" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT" "$TARGET"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_77" "inventory card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_78" "release keeper card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-check-perf-surface-inventory-v0' "$CARD_77" "card must record inventory contract"
guard_expect_fixed_in_file "$TAG" 'target_method=objectLifecycleReleaseBlock' "$CARD_77" "card must select release method"
guard_expect_fixed_in_file "$TAG" 'linear_search_candidate=1' "$CARD_77" "card must record linear search candidate"
guard_expect_fixed_in_file "$TAG" 'suggested_next=release_known_page_fast_path' "$CARD_77" "card must select release keeper"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-77-HAKO-CHECK-PERF-SURFACE-INVENTORY"' "$CURRENT_STATE" "current state latest card must advance to row 77"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-RELEASE-KNOWN-PAGE-FAST-PATH-296X-001"' "$CURRENT_STATE" "current state must select row 78"
guard_expect_fixed_in_file "$TAG" '| 77 | `HAKO-CHECK-PERF-SURFACE-INVENTORY-296X-001` | Landed |' "$TASKBOARD" "taskboard row 77 must be landed"
guard_expect_fixed_in_file "$TAG" '| 78 | `HAKO-MIMALLOC-PERF-RELEASE-KNOWN-PAGE-FAST-PATH-296X-001` | Current |' "$TASKBOARD" "taskboard row 78 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hako_check_inventory.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-check-perf-surface-inventory-v0' "$report" "tool must emit inventory contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=hako-check-perf-surface-contract-v0' "$report" "tool must consume contract"
guard_expect_fixed_in_file "$TAG" 'target_file=lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako' "$report" "tool must inventory target file"
guard_expect_fixed_in_file "$TAG" 'target_box=HakoAllocObjectLifecycleFacade' "$report" "tool must inventory target box"
guard_expect_fixed_in_file "$TAG" 'target_method_0=objectLifecycleSmallAlloc' "$report" "tool must inventory small alloc"
guard_expect_fixed_in_file "$TAG" 'target_method_1=objectLifecycleReleaseBlock' "$report" "tool must inventory release block"
guard_expect_fixed_in_file "$TAG" 'target_method=objectLifecycleReleaseBlock' "$report" "tool must select release block"
guard_expect_fixed_in_file "$TAG" 'linear_search_candidate=1' "$report" "tool must find release linear search candidate"
guard_expect_fixed_in_file "$TAG" 'suggested_next=release_known_page_fast_path' "$report" "tool must select release keeper"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
