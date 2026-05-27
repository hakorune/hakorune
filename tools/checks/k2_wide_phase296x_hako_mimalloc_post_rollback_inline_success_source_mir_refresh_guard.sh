#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-post-rollback-inline-success-source-mir-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_113="docs/development/current/main/phases/phase-296x/296x-113-HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-SOURCE-MIR-REFRESH.md"
CARD_114="docs/development/current/main/phases/phase-296x/296x-114-HAKO-MIMALLOC-SMALL-ALLOC-MIR-SHAPE-DEEP-DIVE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_rollback_inline_success_source_mir_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_post_rollback_inline_success_source_mir_refresh_guard.sh"

echo "[$TAG] checking post-rollback inline success source/MIR refresh"

guard_require_files "$TAG" "$CARD_113" "$CARD_114" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_113" "row113 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_114" "row114 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-rollback-inline-success-source-mir-refresh-v0' "$CARD_113" "row113 must record output contract"
guard_expect_fixed_in_file "$TAG" 'rejected_keeper_count=2' "$CARD_113" "row113 must record both non-keepers"
guard_expect_fixed_in_file "$TAG" 'selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' "$CARD_113" "row113 must keep small alloc as owner"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=small_alloc_mir_shape_deep_dive' "$CARD_113" "row113 must select MIR diagnostic"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-113-HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-SOURCE-MIR-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row113"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SMALL-ALLOC-MIR-SHAPE-DEEP-DIVE-296X-001"' "$CURRENT_STATE" "current state must select row114"
guard_expect_fixed_in_file "$TAG" '| 113 | `HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-SOURCE-MIR-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row113 must be landed"
guard_expect_fixed_in_file "$TAG" '| 114 | `HAKO-MIMALLOC-SMALL-ALLOC-MIR-SHAPE-DEEP-DIVE-296X-001` | Current |' "$TASKBOARD" "taskboard row114 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_rollback_inline_source_mir.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/measurement.out" <<'EOF'
output_contract=hako-mimalloc-post-rollback-inline-success-result-measurement-v0
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
summary=ok
EOF
cat > "$tmp_dir/small.join" <<'EOF'
output_contract=hako-source-mir-shape-join-v1
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_target_method=objectLifecycleSmallAlloc
mir_call_count=26
mir_field_access_count=13
mir_array_access_count=0
confirmed_risk_kind=field_access
summary=ok
EOF
cat > "$tmp_dir/release.join" <<'EOF'
output_contract=hako-source-mir-shape-join-v1
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
source_target_method=objectLifecycleReleaseBlock
mir_call_count=22
mir_field_access_count=4
mir_array_access_count=1
confirmed_risk_kind=array_access
summary=ok
EOF
cat > "$tmp_dir/select-single.join" <<'EOF'
output_contract=hako-source-mir-shape-join-v1
selected_method=HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
source_target_method=selectSinglePageFastPath
mir_call_count=6
mir_field_access_count=20
mir_array_access_count=0
confirmed_risk_kind=field_access
summary=ok
EOF
cat > "$tmp_dir/select-page.join" <<'EOF'
output_contract=hako-source-mir-shape-join-v1
selected_method=HakoAllocObjectLifecyclePageQueue.selectPage/0
source_target_method=selectPage
mir_call_count=9
mir_field_access_count=15
mir_array_access_count=1
confirmed_risk_kind=array_access
summary=ok
EOF

report="$tmp_dir/report.out"
python3 "$TOOL" \
  --measurement-report "$tmp_dir/measurement.out" \
  --join-report "$tmp_dir/small.join" \
  --join-report "$tmp_dir/release.join" \
  --join-report "$tmp_dir/select-single.join" \
  --join-report "$tmp_dir/select-page.join" \
  --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-rollback-inline-success-source-mir-refresh-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'rejected_keeper_count=2' "$report" "tool must reject two non-keepers"
guard_expect_fixed_in_file "$TAG" 'rejected_keeper_0=select_single_page_active_field_fast_path' "$report" "tool must reject active field keeper"
guard_expect_fixed_in_file "$TAG" 'rejected_keeper_1=small_alloc_inline_success_result_fast_path' "$report" "tool must reject inline success keeper"
guard_expect_fixed_in_file "$TAG" 'active_method_rank_0=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' "$report" "tool must rank small alloc first"
guard_expect_fixed_in_file "$TAG" 'active_method_rank_3_active_count=0' "$report" "tool must keep generic select inactive"
guard_expect_fixed_in_file "$TAG" 'selected_next_kind=mir_diagnostic' "$report" "tool must select diagnostic, not keeper"
guard_expect_fixed_in_file "$TAG" 'next_keeper=none' "$report" "tool must not select another keeper"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=small_alloc_mir_shape_deep_dive' "$report" "tool must select deep dive"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
