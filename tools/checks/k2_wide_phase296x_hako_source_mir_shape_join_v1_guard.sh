#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-source-mir-shape-join-v1"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_88="docs/development/current/main/phases/phase-296x/296x-88-HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION.md"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_source_mir_shape_join.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_source_mir_shape_join_v1_guard.sh"

echo "[$TAG] checking source/MIR shape join v1 adapter"

guard_require_files "$TAG" "$CARD_88" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_88" "row88 card must remain current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-source-mir-shape-join-v1' "$CARD_88" "row88 card must record v1 contract"
guard_expect_fixed_in_file "$TAG" 'method_hot_context=direct_loop|caller_repeated|unknown' "$CARD_88" "row88 card must record hot context contract"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_source_mir_join_v1.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

cat > "$tmp_dir/caller_source.out" <<'EOF'
output_contract=hako-check-perf-surface-v1
target_method_0=objectLifecycleSmallAlloc
target_method_0_method_call_count=5
target_method_0_array_access_count=1
target_method_0_field_get_count=16
target_method_0_field_set_count=0
target_method=objectLifecycleSmallAlloc
loop_method_call_count=0
loop_field_get_count=0
loop_field_set_count=0
loop_array_get_count=0
loop_array_length_count=0
summary=ok
EOF
cat > "$tmp_dir/caller_mir.out" <<'EOF'
output_contract=hako-mir-method-shape-v0
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
call_count=25
field_get_count=16
field_set_count=0
array_get_call_count=1
array_length_call_count=0
summary=ok
EOF
caller_report="$tmp_dir/caller_report.out"
python3 "$TOOL" \
  --source-report "$tmp_dir/caller_source.out" \
  --mir-report "$tmp_dir/caller_mir.out" \
  --out "$caller_report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-source-mir-shape-join-v1' "$caller_report" "tool must emit v1 output contract"
guard_expect_fixed_in_file "$TAG" 'method_hot_context=caller_repeated' "$caller_report" "tool must infer repeated caller context"
guard_expect_fixed_in_file "$TAG" 'source_method_call_count=5' "$caller_report" "tool must report method-level source calls"
guard_expect_fixed_in_file "$TAG" 'source_field_get_count=16' "$caller_report" "tool must report method-level field gets"
guard_expect_fixed_in_file "$TAG" 'source_array_access_count=1' "$caller_report" "tool must report method-level array access"
guard_expect_fixed_in_file "$TAG" 'source_risk_confirmed_in_mir=1' "$caller_report" "tool must confirm caller-repeated risk"
guard_expect_fixed_in_file "$TAG" 'confirmed_risk_kind=array_access' "$caller_report" "tool must prioritize array access confirmation"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=keeper_candidate_from_confirmed_caller_repeated_array_access' "$caller_report" "tool must choose caller-repeated next diagnostic"

cat > "$tmp_dir/loop_source.out" <<'EOF'
output_contract=hako-check-perf-surface-v1
target_method_0=selectPage
target_method_0_method_call_count=0
target_method_0_array_access_count=1
target_method_0_field_get_count=3
target_method_0_field_set_count=3
target_method=selectPage
loop_method_call_count=0
loop_field_get_count=3
loop_field_set_count=3
loop_array_get_count=1
loop_array_length_count=0
summary=ok
EOF
cat > "$tmp_dir/loop_mir.out" <<'EOF'
output_contract=hako-mir-method-shape-v0
selected_method=HakoAllocObjectLifecyclePageQueue.selectPage/0
call_count=9
field_get_count=9
field_set_count=6
array_get_call_count=1
array_length_call_count=0
summary=ok
EOF
loop_report="$tmp_dir/loop_report.out"
python3 "$TOOL" \
  --source-report "$tmp_dir/loop_source.out" \
  --mir-report "$tmp_dir/loop_mir.out" \
  --out "$loop_report"

guard_expect_fixed_in_file "$TAG" 'method_hot_context=direct_loop' "$loop_report" "tool must keep direct-loop context"
guard_expect_fixed_in_file "$TAG" 'source_risk_confirmed_in_mir=1' "$loop_report" "tool must confirm direct-loop risk"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=keeper_candidate_from_confirmed_source_mir_array_access' "$loop_report" "tool must preserve direct-loop diagnostic"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$loop_report" "tool must end ok"

echo "[$TAG] ok"
