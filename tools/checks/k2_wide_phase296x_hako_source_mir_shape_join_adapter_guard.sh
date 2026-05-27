#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-source-mir-shape-join-adapter"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_86="docs/development/current/main/phases/phase-296x/296x-86-HAKO-SOURCE-MIR-SHAPE-JOIN-ADAPTER.md"
CARD_87="docs/development/current/main/phases/phase-296x/296x-87-HAKO-MIR-METHOD-SHAPE-HAKO-MIGRATION-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_source_mir_shape_join.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_source_mir_shape_join_adapter_guard.sh"

echo "[$TAG] checking source/MIR shape join adapter"

guard_require_files "$TAG" "$CARD_86" "$CARD_87" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_86" "join card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_87" "migration selection card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-source-mir-shape-join-v0' "$CARD_86" "card must record output contract"
guard_expect_fixed_in_file "$TAG" 'source_risk_confirmed_in_mir=1' "$CARD_86" "card must record confirmed risk"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-86-HAKO-SOURCE-MIR-SHAPE-JOIN-ADAPTER"' "$CURRENT_STATE" "current state latest card must advance to row 86"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIR-METHOD-SHAPE-HAKO-MIGRATION-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select row 87"
guard_expect_fixed_in_file "$TAG" '| 86 | `HAKO-SOURCE-MIR-SHAPE-JOIN-ADAPTER-296X-001` | Landed |' "$TASKBOARD" "taskboard row 86 must be landed"
guard_expect_fixed_in_file "$TAG" '| 87 | `HAKO-MIR-METHOD-SHAPE-HAKO-MIGRATION-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row 87 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_source_mir_join.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/source.out" <<'EOF'
output_contract=hako-check-perf-surface-v1
target_method=selectPage
loop_method_call_count=7
loop_field_get_count=3
loop_field_set_count=3
loop_array_get_count=1
loop_array_length_count=0
summary=ok
EOF
cat > "$tmp_dir/mir.out" <<'EOF'
output_contract=hako-mir-method-shape-v0
selected_method=HakoAllocObjectLifecyclePageQueue.selectPage/0
call_count=9
field_get_count=9
field_set_count=6
array_get_call_count=1
array_length_call_count=0
summary=ok
EOF
report="$tmp_dir/report.out"
python3 "$TOOL" --source-report "$tmp_dir/source.out" --mir-report "$tmp_dir/mir.out" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-source-mir-shape-join-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'source_contract=hako-check-perf-surface-v1' "$report" "tool must record source contract"
guard_expect_fixed_in_file "$TAG" 'mir_contract=hako-mir-method-shape-v0' "$report" "tool must record MIR contract"
guard_expect_fixed_in_file "$TAG" 'selected_method=HakoAllocObjectLifecyclePageQueue.selectPage/0' "$report" "tool must select MIR method"
guard_expect_fixed_in_file "$TAG" 'source_risk_confirmed_in_mir=1' "$report" "tool must confirm risk"
guard_expect_fixed_in_file "$TAG" 'confirmed_risk_kind=array_access' "$report" "tool must prioritize array access confirmation"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=keeper_candidate_from_confirmed_source_mir_array_access' "$report" "tool must choose next diagnostic"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
