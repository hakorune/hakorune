#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-post-release-object-cache-source-mir-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_94="docs/development/current/main/phases/phase-296x/296x-94-HAKO-MIMALLOC-POST-RELEASE-OBJECT-CACHE-SOURCE-MIR-REFRESH.md"
CARD_95="docs/development/current/main/phases/phase-296x/296x-95-HAKO-MIMALLOC-RELEASE-DIRECT-CACHED-PAGE-FAST-PATH-KEEPER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_release_object_cache_source_mir_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_post_release_object_cache_source_mir_refresh_guard.sh"

echo "[$TAG] checking post-release object-cache source/MIR refresh"

guard_require_files "$TAG" "$CARD_94" "$CARD_95" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_94" "row94 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_95" "row95 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-release-object-cache-source-mir-refresh-v0' "$CARD_94" "row94 must record output contract"
guard_expect_fixed_in_file "$TAG" 'next_keeper=release_direct_cached_page_fast_path' "$CARD_94" "row94 must select release direct fast path"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-94-HAKO-MIMALLOC-POST-RELEASE-OBJECT-CACHE-SOURCE-MIR-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row94"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-RELEASE-DIRECT-CACHED-PAGE-FAST-PATH-KEEPER-296X-001"' "$CURRENT_STATE" "current state must select row95"
guard_expect_fixed_in_file "$TAG" '| 94 | `HAKO-MIMALLOC-POST-RELEASE-OBJECT-CACHE-SOURCE-MIR-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row94 must be landed"
guard_expect_fixed_in_file "$TAG" '| 95 | `HAKO-MIMALLOC-RELEASE-DIRECT-CACHED-PAGE-FAST-PATH-KEEPER-296X-001` | Current |' "$TASKBOARD" "taskboard row95 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_release_object_cache_refresh.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/measurement.out" <<'EOF'
output_contract=hako-mimalloc-post-release-object-cache-keeper-measurement-v0
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
summary=ok
EOF
cat > "$tmp_dir/release.join" <<'EOF'
output_contract=hako-source-mir-shape-join-v1
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
source_target_method=objectLifecycleReleaseBlock
method_hot_context=caller_repeated
source_risk_confirmed_in_mir=1
confirmed_risk_kind=array_access
summary=ok
EOF

report="$tmp_dir/report.out"
python3 "$TOOL" \
  --measurement-report "$tmp_dir/measurement.out" \
  --join-report "$tmp_dir/release.join" \
  --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-release-object-cache-source-mir-refresh-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fallback_count=0' "$report" "tool must record inactive release fallback"
guard_expect_fixed_in_file "$TAG" 'selected_reason=release_cache_hot_path_fallback_inactive' "$report" "tool must record selection reason"
guard_expect_fixed_in_file "$TAG" 'selected_source_method=objectLifecycleReleaseBlock' "$report" "tool must select release method"
guard_expect_fixed_in_file "$TAG" 'next_keeper=release_direct_cached_page_fast_path' "$report" "tool must select next keeper"
guard_expect_fixed_in_file "$TAG" 'next_keeper_kind=box_count' "$report" "tool must keep keeper kind narrow"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
