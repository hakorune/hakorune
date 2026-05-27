#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-hot-owner-rank"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_105="docs/development/current/main/phases/phase-296x/296x-105-HAKO-MIMALLOC-POST-ROLLBACK-SOURCE-MIR-REFRESH.md"
CARD_106="docs/development/current/main/phases/phase-296x/296x-106-HAKO-MIMALLOC-SMALL-ALLOC-DIRECT-SINGLE-PAGE-SELECT-FAST-PATH-KEEPER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_hot_owner_rank.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_hot_owner_rank_guard.sh"

echo "[$TAG] checking hot-owner rank"

guard_require_files "$TAG" "$CARD_105" "$CARD_106" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_105" "row105 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_106" "row106 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-hot-owner-rank-v0' "$CARD_105" "row105 must record output contract"
guard_expect_fixed_in_file "$TAG" 'rejected_keeper=select_single_page_active_field_fast_path' "$CARD_105" "row105 must reject row101 non-keeper"
guard_expect_fixed_in_file "$TAG" 'next_keeper=small_alloc_direct_single_page_select_fast_path' "$CARD_105" "row105 must select next keeper"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-105-HAKO-MIMALLOC-POST-ROLLBACK-SOURCE-MIR-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row105"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SMALL-ALLOC-DIRECT-SINGLE-PAGE-SELECT-FAST-PATH-KEEPER-296X-001"' "$CURRENT_STATE" "current state must select row106"
guard_expect_fixed_in_file "$TAG" '| 105 | `HAKO-MIMALLOC-POST-ROLLBACK-SOURCE-MIR-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row105 must be landed"
guard_expect_fixed_in_file "$TAG" '| 106 | `HAKO-MIMALLOC-SMALL-ALLOC-DIRECT-SINGLE-PAGE-SELECT-FAST-PATH-KEEPER-296X-001` | Current |' "$TASKBOARD" "taskboard row106 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hot_owner_rank.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/measurement.out" <<'EOF'
output_contract=hako-mimalloc-post-rollback-active-field-fast-path-measurement-v0
allocation_count=524288
free_count=524288
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
mir_call_count=24
mir_field_access_count=16
mir_array_access_count=0
summary=ok
EOF
cat > "$tmp_dir/release-known.join" <<'EOF'
output_contract=hako-source-mir-shape-join-v1
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseKnownPageIndex/1
source_target_method=objectLifecycleReleaseKnownPageIndex
mir_call_count=1
mir_field_access_count=14
mir_array_access_count=0
summary=ok
EOF

report="$tmp_dir/report.out"
python3 "$TOOL" \
  --measurement-report "$tmp_dir/measurement.out" \
  --join-report "$tmp_dir/small.join" \
  --join-report "$tmp_dir/release-known.join" \
  --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-hot-owner-rank-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'rejected_keeper=select_single_page_active_field_fast_path' "$report" "tool must reject non-keeper"
guard_expect_fixed_in_file "$TAG" 'active_method_rank_0=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' "$report" "tool must rank small alloc first"
guard_expect_fixed_in_file "$TAG" 'active_method_rank_1_active_count=0' "$report" "tool must mark release lookup inactive"
guard_expect_fixed_in_file "$TAG" 'selected_risk_kind=method_call_surface' "$report" "tool must classify selected risk"
guard_expect_fixed_in_file "$TAG" 'next_keeper=small_alloc_direct_single_page_select_fast_path' "$report" "tool must select next keeper"
guard_expect_fixed_in_file "$TAG" 'confidence=medium' "$report" "tool must emit confidence"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
