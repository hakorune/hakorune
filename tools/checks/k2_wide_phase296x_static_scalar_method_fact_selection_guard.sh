#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-static-scalar-method-fact-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_135="docs/development/current/main/phases/phase-296x/296x-135-STATIC-SCALAR-METHOD-FACT-SELECTION.md"
CARD_136="docs/development/current/main/phases/phase-296x/296x-136-STATIC-SCALAR-METHOD-FACT-INFERENCE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SOURCE="lang/src/hako_alloc/memory/object_lifecycle_facade_reason_box.hako"
TOOL="tools/allocator/static_scalar_method_fact_selection.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_static_scalar_method_fact_selection_guard.sh"

echo "[$TAG] checking static scalar method fact selection"

guard_require_files "$TAG" "$CARD_135" "$CARD_136" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SOURCE" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_135" "row135 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_136" "row136 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=static-scalar-method-fact-selection-v0' "$CARD_135" "row135 must record output contract"
guard_expect_fixed_in_file "$TAG" 'candidate_family=object_lifecycle_facade_reason_zero_arg_return_literal_i64' "$CARD_135" "row135 must select candidate family"
guard_expect_fixed_in_file "$TAG" 'generic_cse=0' "$CARD_135" "row135 must keep generic CSE closed"
guard_expect_fixed_in_file "$TAG" 'whole_box_pure=0' "$CARD_135" "row135 must avoid whole-box pure marker"
guard_expect_fixed_in_file "$TAG" 'const_lowering=0' "$CARD_135" "row135 must keep const lowering closed"
guard_expect_fixed_in_file "$TAG" 'selected_next=static_scalar_method_fact_inference' "$CARD_135" "row135 must select inference"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-135-STATIC-SCALAR-METHOD-FACT-SELECTION"' "$CURRENT_STATE" "current state latest card must advance to row135"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "STATIC-SCALAR-METHOD-FACT-INFERENCE-296X-001"' "$CURRENT_STATE" "current state must select row136"
guard_expect_fixed_in_file "$TAG" '| 135 | `STATIC-SCALAR-METHOD-FACT-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row135 must be landed"
guard_expect_fixed_in_file "$TAG" '| 136 | `STATIC-SCALAR-METHOD-FACT-INFERENCE-296X-001` | Current |' "$TASKBOARD" "taskboard row136 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_static_scalar_selection.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
python3 "$TOOL" --source "$SOURCE" --out "$tmp_dir/report.out"

guard_expect_fixed_in_file "$TAG" 'output_contract=static-scalar-method-fact-selection-v0' "$tmp_dir/report.out" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'candidate_family=object_lifecycle_facade_reason_zero_arg_return_literal_i64' "$tmp_dir/report.out" "tool must emit candidate family"
guard_expect_fixed_in_file "$TAG" 'selection=verified_static_method_return_literal_shape' "$tmp_dir/report.out" "tool must emit selection"
guard_expect_fixed_in_file "$TAG" 'candidate_count=19' "$tmp_dir/report.out" "tool must record current reason candidate count"
guard_expect_fixed_in_file "$TAG" 'generic_cse=0' "$tmp_dir/report.out" "tool must keep generic CSE closed"
guard_expect_fixed_in_file "$TAG" 'whole_box_pure=0' "$tmp_dir/report.out" "tool must keep whole-box pure closed"
guard_expect_fixed_in_file "$TAG" 'const_lowering=0' "$tmp_dir/report.out" "tool must keep lowering closed"
guard_expect_fixed_in_file "$TAG" 'failure_mode=keep_call' "$tmp_dir/report.out" "tool must record unverified fallback mode"
guard_expect_fixed_in_file "$TAG" 'selected_next=static_scalar_method_fact_inference' "$tmp_dir/report.out" "tool must select inference"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$tmp_dir/report.out" "tool must end ok"

echo "[$TAG] ok"
