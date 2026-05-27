#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-post-static-scalar-source-mir-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_140="docs/development/current/main/phases/phase-296x/296x-140-POST-STATIC-SCALAR-SOURCE-MIR-REFRESH.md"
CARD_141="docs/development/current/main/phases/phase-296x/296x-141-SMALL-ALLOC-CALL-COPY-SHAPE-DEEP-DIVE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/post_static_scalar_source_mir_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_static_scalar_source_mir_refresh_guard.sh"

echo "[$TAG] checking post static-scalar source/MIR refresh"

guard_require_files "$TAG" "$CARD_140" "$CARD_141" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_140" "row140 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_141" "row141 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=post-static-scalar-source-mir-refresh-v0' "$CARD_140" "row140 must record output contract"
guard_expect_fixed_in_file "$TAG" 'selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' "$CARD_140" "row140 must select small alloc"
guard_expect_fixed_in_file "$TAG" 'remaining_call_surface=facade_result_helpers_and_page_hotpath' "$CARD_140" "row140 must record remaining call surface"
guard_expect_fixed_in_file "$TAG" 'selected_next=small_alloc_call_copy_shape_deep_dive' "$CARD_140" "row140 must select deep dive"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-140-POST-STATIC-SCALAR-SOURCE-MIR-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row140"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "SMALL-ALLOC-CALL-COPY-SHAPE-DEEP-DIVE-296X-001"' "$CURRENT_STATE" "current state must select row141"
guard_expect_fixed_in_file "$TAG" '| 140 | `POST-STATIC-SCALAR-SOURCE-MIR-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row140 must be landed"
guard_expect_fixed_in_file "$TAG" '| 141 | `SMALL-ALLOC-CALL-COPY-SHAPE-DEEP-DIVE-296X-001` | Current |' "$TASKBOARD" "taskboard row141 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_static_scalar_refresh.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=post-static-scalar-source-mir-refresh-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'small_alloc_call_count=16' "$report" "tool must record small alloc call count"
guard_expect_fixed_in_file "$TAG" 'small_alloc_copy_count=99' "$report" "tool must record small alloc copy count"
guard_expect_fixed_in_file "$TAG" 'small_alloc_record_failure_call_count=5' "$report" "tool must record failure helper calls"
guard_expect_fixed_in_file "$TAG" 'page_acquire_array_get_call_count=2' "$report" "tool must record page acquire array get calls"
guard_expect_fixed_in_file "$TAG" 'gap_owner=compiler_lowering' "$report" "tool must classify owner"
guard_expect_fixed_in_file "$TAG" 'selected_next=small_alloc_call_copy_shape_deep_dive' "$report" "tool must select next"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
