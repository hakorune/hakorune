#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-post-small-alloc-direct-select-source-mir-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_108="docs/development/current/main/phases/phase-296x/296x-108-HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-SOURCE-MIR-REFRESH.md"
CARD_109="docs/development/current/main/phases/phase-296x/296x-109-HAKO-MIMALLOC-SMALL-ALLOC-INLINE-SUCCESS-RESULT-FAST-PATH-KEEPER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_small_alloc_direct_select_source_mir_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_post_small_alloc_direct_select_source_mir_refresh_guard.sh"

echo "[$TAG] checking post-small-alloc-direct-select source/MIR refresh"

guard_require_files "$TAG" "$CARD_108" "$CARD_109" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_108" "row108 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_109" "row109 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-small-alloc-direct-select-source-mir-refresh-v0' "$CARD_108" "row108 must record output contract"
guard_expect_fixed_in_file "$TAG" 'accepted_keeper=small_alloc_direct_single_page_select_fast_path' "$CARD_108" "row108 must record accepted keeper"
guard_expect_fixed_in_file "$TAG" 'next_keeper=small_alloc_inline_success_result_fast_path' "$CARD_108" "row108 must select next keeper"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-108-HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-SOURCE-MIR-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row108"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SMALL-ALLOC-INLINE-SUCCESS-RESULT-FAST-PATH-KEEPER-296X-001"' "$CURRENT_STATE" "current state must select row109"
guard_expect_fixed_in_file "$TAG" '| 108 | `HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-SOURCE-MIR-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row108 must be landed"
guard_expect_fixed_in_file "$TAG" '| 109 | `HAKO-MIMALLOC-SMALL-ALLOC-INLINE-SUCCESS-RESULT-FAST-PATH-KEEPER-296X-001` | Current |' "$TASKBOARD" "taskboard row109 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_small_alloc_direct_refresh.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/measurement.out" <<'EOF'
output_contract=hako-mimalloc-post-small-alloc-direct-select-keeper-measurement-v0
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
mir_call_count=26
mir_field_access_count=13
mir_array_access_count=0
summary=ok
EOF
cat > "$tmp_dir/release.join" <<'EOF'
output_contract=hako-source-mir-shape-join-v1
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
source_target_method=objectLifecycleReleaseBlock
mir_call_count=22
mir_field_access_count=4
mir_array_access_count=1
summary=ok
EOF

report="$tmp_dir/report.out"
python3 "$TOOL" \
  --measurement-report "$tmp_dir/measurement.out" \
  --join-report "$tmp_dir/small.join" \
  --join-report "$tmp_dir/release.join" \
  --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-small-alloc-direct-select-source-mir-refresh-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'accepted_keeper=small_alloc_direct_single_page_select_fast_path' "$report" "tool must record accepted keeper"
guard_expect_fixed_in_file "$TAG" 'selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' "$report" "tool must select small alloc"
guard_expect_fixed_in_file "$TAG" 'selected_risk_kind=method_call_surface' "$report" "tool must classify selected risk"
guard_expect_fixed_in_file "$TAG" 'next_keeper=small_alloc_inline_success_result_fast_path' "$report" "tool must select next keeper"
guard_expect_fixed_in_file "$TAG" 'confidence=medium' "$report" "tool must emit confidence"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
