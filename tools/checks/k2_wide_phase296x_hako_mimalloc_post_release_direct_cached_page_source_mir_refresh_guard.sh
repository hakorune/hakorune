#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-post-release-direct-cached-page-source-mir-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_97="docs/development/current/main/phases/phase-296x/296x-97-HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-SOURCE-MIR-REFRESH.md"
CARD_98="docs/development/current/main/phases/phase-296x/296x-98-HAKO-MIMALLOC-SELECT-SINGLE-PAGE-FIRST-PAGE-CACHE-KEEPER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_release_direct_cached_page_source_mir_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_post_release_direct_cached_page_source_mir_refresh_guard.sh"

echo "[$TAG] checking post-release-direct cached-page source/MIR refresh"

guard_require_files "$TAG" "$CARD_97" "$CARD_98" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_97" "row97 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_98" "row98 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-release-direct-cached-page-source-mir-refresh-v0' "$CARD_97" "row97 must record output contract"
guard_expect_fixed_in_file "$TAG" 'next_keeper=select_single_page_first_page_cache' "$CARD_97" "row97 must select first-page cache keeper"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-97-HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-SOURCE-MIR-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row97"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SELECT-SINGLE-PAGE-FIRST-PAGE-CACHE-KEEPER-296X-001"' "$CURRENT_STATE" "current state must select row98"
guard_expect_fixed_in_file "$TAG" '| 97 | `HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-SOURCE-MIR-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row97 must be landed"
guard_expect_fixed_in_file "$TAG" '| 98 | `HAKO-MIMALLOC-SELECT-SINGLE-PAGE-FIRST-PAGE-CACHE-KEEPER-296X-001` | Current |' "$TASKBOARD" "taskboard row98 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_release_direct_refresh.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/measurement.out" <<'EOF'
output_contract=hako-mimalloc-post-release-direct-cached-page-keeper-measurement-v0
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
summary=ok
EOF
cat > "$tmp_dir/select.join" <<'EOF'
output_contract=hako-source-mir-shape-join-v1
selected_method=HakoAllocObjectLifecyclePageQueue.selectPage/0
source_target_method=selectPage
method_hot_context=direct_loop
source_risk_confirmed_in_mir=1
confirmed_risk_kind=array_access
summary=ok
EOF

report="$tmp_dir/report.out"
python3 "$TOOL" \
  --measurement-report "$tmp_dir/measurement.out" \
  --join-report "$tmp_dir/select.join" \
  --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-release-direct-cached-page-source-mir-refresh-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'selected_reason=single_page_select_hot_path_fallback_inactive' "$report" "tool must record selection reason"
guard_expect_fixed_in_file "$TAG" 'selected_source_method=selectPage' "$report" "tool must select selectPage"
guard_expect_fixed_in_file "$TAG" 'next_keeper=select_single_page_first_page_cache' "$report" "tool must select keeper"
guard_expect_fixed_in_file "$TAG" 'next_keeper_kind=box_count' "$report" "tool must keep keeper kind narrow"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
