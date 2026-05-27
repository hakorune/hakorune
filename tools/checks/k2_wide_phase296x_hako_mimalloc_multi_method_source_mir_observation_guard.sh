#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-multi-method-source-mir-observation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_88="docs/development/current/main/phases/phase-296x/296x-88-HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION.md"
CARD_89="docs/development/current/main/phases/phase-296x/296x-89-HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-CACHE-KEEPER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_multi_method_source_mir_observation.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_multi_method_source_mir_observation_guard.sh"

echo "[$TAG] checking multi-method source/MIR observation"

guard_require_files "$TAG" "$CARD_88" "$CARD_89" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_88" "row88 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_89" "row89 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-multi-method-source-mir-observation-v0' "$CARD_88" "row88 must record output contract"
guard_expect_fixed_in_file "$TAG" 'next_keeper=small_alloc_selected_page_cache_reuse' "$CARD_88" "row88 must select the next keeper"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-88-HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION"' "$CURRENT_STATE" "current state latest card must advance to row 88"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-CACHE-KEEPER-296X-001"' "$CURRENT_STATE" "current state must select row 89"
guard_expect_fixed_in_file "$TAG" '| 88 | `HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 88 must be landed"
guard_expect_fixed_in_file "$TAG" '| 89 | `HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-CACHE-KEEPER-296X-001` | Current |' "$TASKBOARD" "taskboard row 89 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_multi_method_source_mir.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

cat > "$tmp_dir/small.join" <<'EOF'
output_contract=hako-source-mir-shape-join-v1
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_target_method=objectLifecycleSmallAlloc
source_loop_array_access_count=0
source_array_access_count=1
method_hot_context=caller_repeated
source_risk_confirmed_in_mir=1
confirmed_risk_kind=array_access
summary=ok
EOF
cat > "$tmp_dir/release.join" <<'EOF'
output_contract=hako-source-mir-shape-join-v1
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
source_target_method=objectLifecycleReleaseBlock
source_loop_array_access_count=0
source_array_access_count=1
method_hot_context=caller_repeated
source_risk_confirmed_in_mir=1
confirmed_risk_kind=array_access
summary=ok
EOF
cat > "$tmp_dir/select.join" <<'EOF'
output_contract=hako-source-mir-shape-join-v1
selected_method=HakoAllocObjectLifecyclePageQueue.selectPage/0
source_target_method=selectPage
source_loop_array_access_count=1
source_array_access_count=1
method_hot_context=direct_loop
source_risk_confirmed_in_mir=1
confirmed_risk_kind=array_access
summary=ok
EOF

report="$tmp_dir/report.out"
python3 "$TOOL" \
  --join-report "$tmp_dir/small.join" \
  --join-report "$tmp_dir/release.join" \
  --join-report "$tmp_dir/select.join" \
  --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-multi-method-source-mir-observation-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=hako-source-mir-shape-join-v1' "$report" "tool must record input contract"
guard_expect_fixed_in_file "$TAG" 'method_count=3' "$report" "tool must record method count"
guard_expect_fixed_in_file "$TAG" 'confirmed_source_mir_risk_count=3' "$report" "tool must record confirmed count"
guard_expect_fixed_in_file "$TAG" 'selected_source_method=objectLifecycleSmallAlloc' "$report" "tool must select small alloc"
guard_expect_fixed_in_file "$TAG" 'selected_hot_context=caller_repeated' "$report" "tool must record hot context"
guard_expect_fixed_in_file "$TAG" 'selected_risk_kind=array_access' "$report" "tool must record selected risk"
guard_expect_fixed_in_file "$TAG" 'next_keeper=small_alloc_selected_page_cache_reuse' "$report" "tool must select keeper"
guard_expect_fixed_in_file "$TAG" 'next_keeper_kind=box_count' "$report" "tool must keep keeper kind narrow"
guard_expect_fixed_in_file "$TAG" 'next_row=HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-CACHE-KEEPER-296X-001' "$report" "tool must select row 89"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
