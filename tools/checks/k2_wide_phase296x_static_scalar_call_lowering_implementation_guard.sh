#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-static-scalar-call-lowering-implementation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_138="docs/development/current/main/phases/phase-296x/296x-138-STATIC-SCALAR-CALL-LOWERING-IMPLEMENTATION.md"
CARD_139="docs/development/current/main/phases/phase-296x/296x-139-POST-STATIC-SCALAR-CALL-LOWERING-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
APP="apps/mimalloc-facade-realloc-grow-proof/main.hako"
SOURCE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
TOOL="tools/allocator/static_scalar_call_lowering_implementation.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_static_scalar_call_lowering_implementation_guard.sh"
RUST_FACTS="src/mir/builder/static_scalar_facts.rs"
RUST_HANDLER="src/mir/builder/method_call_handlers.rs"

echo "[$TAG] checking static scalar call lowering implementation"

guard_require_files "$TAG" "$CARD_138" "$CARD_139" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$APP" "$SOURCE" "$TOOL" "$SELF_SCRIPT" "$RUST_FACTS" "$RUST_HANDLER"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_138" "row138 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_139" "row139 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=static-scalar-call-lowering-implementation-v0' "$CARD_138" "row138 must record output contract"
guard_expect_fixed_in_file "$TAG" 'remaining_reason_call_count=0' "$CARD_138" "row138 must remove selected facade reason calls"
guard_expect_fixed_in_file "$TAG" 'generic_cse=0' "$CARD_138" "row138 must keep generic CSE closed"
guard_expect_fixed_in_file "$TAG" 'whole_box_pure=0' "$CARD_138" "row138 must keep whole-box pure closed"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-138-STATIC-SCALAR-CALL-LOWERING-IMPLEMENTATION"' "$CURRENT_STATE" "current state latest card must advance to row138"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "POST-STATIC-SCALAR-CALL-LOWERING-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row139"
guard_expect_fixed_in_file "$TAG" '| 138 | `STATIC-SCALAR-CALL-LOWERING-IMPLEMENTATION-296X-001` | Landed |' "$TASKBOARD" "taskboard row138 must be landed"
guard_expect_fixed_in_file "$TAG" '| 139 | `POST-STATIC-SCALAR-CALL-LOWERING-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row139 must be current"
guard_expect_fixed_in_file "$TAG" 'emit_static_scalar_fact_const' "$RUST_FACTS" "static scalar fact const emitter must exist"
guard_expect_fixed_in_file "$TAG" 'static_scalar_method_fact(&func_name)' "$RUST_HANDLER" "static call handler must consult verified facts"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_static_scalar_lowering.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
target/release/hakorune --backend mir --emit-mir-json "$tmp_dir/facade.mir.json" "$APP" >/dev/null
python3 "$TOOL" --mir-json "$tmp_dir/facade.mir.json" --source "$SOURCE" --out "$tmp_dir/report.out"

guard_expect_fixed_in_file "$TAG" 'output_contract=static-scalar-call-lowering-implementation-v0' "$tmp_dir/report.out" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=static-scalar-call-lowering-selection-v0' "$tmp_dir/report.out" "tool must record input"
guard_expect_fixed_in_file "$TAG" 'remaining_reason_call_count=0' "$tmp_dir/report.out" "tool must record no remaining selected reason calls"
guard_expect_fixed_in_file "$TAG" 'missing_fact_keep_call_count=0' "$tmp_dir/report.out" "tool must record no selected missing facts"
guard_expect_fixed_in_file "$TAG" 'generic_cse=0' "$tmp_dir/report.out" "tool must keep generic CSE closed"
guard_expect_fixed_in_file "$TAG" 'whole_box_pure=0' "$tmp_dir/report.out" "tool must keep whole-box pure closed"
guard_expect_fixed_in_file "$TAG" 'selected_next=post_static_scalar_call_lowering_measurement' "$tmp_dir/report.out" "tool must select measurement"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$tmp_dir/report.out" "tool must end ok"

echo "[$TAG] ok"
