#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-static-scalar-call-lowering-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_137="docs/development/current/main/phases/phase-296x/296x-137-STATIC-SCALAR-CALL-LOWERING-SELECTION.md"
CARD_138="docs/development/current/main/phases/phase-296x/296x-138-STATIC-SCALAR-CALL-LOWERING-IMPLEMENTATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/static_scalar_call_lowering_selection.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_static_scalar_call_lowering_selection_guard.sh"

echo "[$TAG] checking static scalar call lowering selection"

guard_require_files "$TAG" "$CARD_137" "$CARD_138" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_137" "row137 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_138" "row138 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=static-scalar-call-lowering-selection-v0' "$CARD_137" "row137 must record output contract"
guard_expect_fixed_in_file "$TAG" 'lowering_route=handle_static_method_call_zero_arg_before_emit_unified_call' "$CARD_137" "row137 must select exact route"
guard_expect_fixed_in_file "$TAG" 'arg_policy=zero_args_only' "$CARD_137" "row137 must keep zero-arg policy"
guard_expect_fixed_in_file "$TAG" 'fallback_on_missing_fact=keep_call' "$CARD_137" "row137 must keep missing-fact calls"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-137-STATIC-SCALAR-CALL-LOWERING-SELECTION"' "$CURRENT_STATE" "current state latest card must advance to row137"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "STATIC-SCALAR-CALL-LOWERING-IMPLEMENTATION-296X-001"' "$CURRENT_STATE" "current state must select row138"
guard_expect_fixed_in_file "$TAG" '| 137 | `STATIC-SCALAR-CALL-LOWERING-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row137 must be landed"
guard_expect_fixed_in_file "$TAG" '| 138 | `STATIC-SCALAR-CALL-LOWERING-IMPLEMENTATION-296X-001` | Current |' "$TASKBOARD" "taskboard row138 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_static_scalar_lowering_selection.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
python3 "$TOOL" --out "$tmp_dir/report.out"

guard_expect_fixed_in_file "$TAG" 'output_contract=static-scalar-call-lowering-selection-v0' "$tmp_dir/report.out" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=static-scalar-method-fact-inference-v0' "$tmp_dir/report.out" "tool must record input"
guard_expect_fixed_in_file "$TAG" 'lowering_route=handle_static_method_call_zero_arg_before_emit_unified_call' "$tmp_dir/report.out" "tool must select route"
guard_expect_fixed_in_file "$TAG" 'guard_surface=object_lifecycle_reason_static_receiver_zero_arg' "$tmp_dir/report.out" "tool must record guard surface"
guard_expect_fixed_in_file "$TAG" 'fallback_on_missing_fact=keep_call' "$tmp_dir/report.out" "tool must record missing fact fallback"
guard_expect_fixed_in_file "$TAG" 'selected_next=static_scalar_call_lowering_implementation' "$tmp_dir/report.out" "tool must select implementation"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$tmp_dir/report.out" "tool must end ok"

echo "[$TAG] ok"
