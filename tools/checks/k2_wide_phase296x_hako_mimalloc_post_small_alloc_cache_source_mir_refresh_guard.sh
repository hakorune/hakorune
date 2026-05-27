#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-post-small-alloc-cache-source-mir-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_91="docs/development/current/main/phases/phase-296x/296x-91-HAKO-MIMALLOC-POST-SMALL-ALLOC-CACHE-SOURCE-MIR-REFRESH.md"
CARD_92="docs/development/current/main/phases/phase-296x/296x-92-HAKO-MIMALLOC-RELEASE-KNOWN-PAGE-OBJECT-CACHE-KEEPER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_small_alloc_cache_source_mir_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_post_small_alloc_cache_source_mir_refresh_guard.sh"

echo "[$TAG] checking post-small-alloc-cache source/MIR refresh"

guard_require_files "$TAG" "$CARD_91" "$CARD_92" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_91" "row91 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_92" "row92 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-small-alloc-cache-source-mir-refresh-v0' "$CARD_91" "row91 must record output contract"
guard_expect_fixed_in_file "$TAG" 'next_keeper=release_known_page_object_cache' "$CARD_91" "row91 must select release object cache"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-91-HAKO-MIMALLOC-POST-SMALL-ALLOC-CACHE-SOURCE-MIR-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row91"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-RELEASE-KNOWN-PAGE-OBJECT-CACHE-KEEPER-296X-001"' "$CURRENT_STATE" "current state must select row92"
guard_expect_fixed_in_file "$TAG" '| 91 | `HAKO-MIMALLOC-POST-SMALL-ALLOC-CACHE-SOURCE-MIR-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row91 must be landed"
guard_expect_fixed_in_file "$TAG" '| 92 | `HAKO-MIMALLOC-RELEASE-KNOWN-PAGE-OBJECT-CACHE-KEEPER-296X-001` | Current |' "$TASKBOARD" "taskboard row92 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_small_alloc_cache_refresh.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/measurement.out" <<'EOF'
output_contract=hako-mimalloc-post-small-alloc-cache-keeper-measurement-v0
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
summary=ok
EOF
cat > "$tmp_dir/small.join" <<'EOF'
output_contract=hako-source-mir-shape-join-v1
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_target_method=objectLifecycleSmallAlloc
method_hot_context=caller_repeated
source_risk_confirmed_in_mir=1
confirmed_risk_kind=field_access
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
  --join-report "$tmp_dir/small.join" \
  --join-report "$tmp_dir/release.join" \
  --join-report "$tmp_dir/select.join" \
  --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-small-alloc-cache-source-mir-refresh-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'method_count=3' "$report" "tool must record method count"
guard_expect_fixed_in_file "$TAG" 'confirmed_source_mir_risk_count=3' "$report" "tool must record confirmed count"
guard_expect_fixed_in_file "$TAG" 'inactive_risk=select_page_loop_inactive_for_single_page_workload' "$report" "tool must mark inactive select loop risk"
guard_expect_fixed_in_file "$TAG" 'selected_source_method=objectLifecycleReleaseBlock' "$report" "tool must select release method"
guard_expect_fixed_in_file "$TAG" 'selected_risk_kind=array_access' "$report" "tool must select array risk"
guard_expect_fixed_in_file "$TAG" 'next_keeper=release_known_page_object_cache' "$report" "tool must select keeper"
guard_expect_fixed_in_file "$TAG" 'next_keeper_kind=box_shape' "$report" "tool must classify keeper kind"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
