#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-post-static-scalar-call-lowering-measurement"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_139="docs/development/current/main/phases/phase-296x/296x-139-POST-STATIC-SCALAR-CALL-LOWERING-MEASUREMENT.md"
CARD_140="docs/development/current/main/phases/phase-296x/296x-140-POST-STATIC-SCALAR-SOURCE-MIR-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/post_static_scalar_call_lowering_measurement.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_static_scalar_call_lowering_measurement_guard.sh"

echo "[$TAG] checking post static-scalar call lowering measurement"

guard_require_files "$TAG" "$CARD_139" "$CARD_140" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_139" "row139 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_140" "row140 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=post-static-scalar-call-lowering-measurement-v0' "$CARD_139" "row139 must record output contract"
guard_expect_fixed_in_file "$TAG" 'measurement_profile=object_lifecycle_facade_exact_exe' "$CARD_139" "row139 must record measurement profile"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$CARD_139" "row139 must record sample count"
guard_expect_fixed_in_file "$TAG" 'previous_checkpoint_hako_elapsed_median_ms=610' "$CARD_139" "row139 must record previous checkpoint"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_139" "row139 must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-139-POST-STATIC-SCALAR-CALL-LOWERING-MEASUREMENT"' "$CURRENT_STATE" "current state latest card must advance to row139"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "POST-STATIC-SCALAR-SOURCE-MIR-REFRESH-296X-001"' "$CURRENT_STATE" "current state must select row140"
guard_expect_fixed_in_file "$TAG" '| 139 | `POST-STATIC-SCALAR-CALL-LOWERING-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" "taskboard row139 must be landed"
guard_expect_fixed_in_file "$TAG" '| 140 | `POST-STATIC-SCALAR-SOURCE-MIR-REFRESH-296X-001` | Current |' "$TASKBOARD" "taskboard row140 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_static_scalar_measurement.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --sample-count 1 --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=post-static-scalar-call-lowering-measurement-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'measurement_scope=object_lifecycle_facade_exact_exe_after_static_scalar_call_lowering' "$report" "tool must name scope"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "tool must use 8192 repeat"
guard_expect_fixed_in_file "$TAG" 'sample_count=1' "$report" "tool must support cheap guard sample count"
guard_expect_fixed_in_file "$TAG" 'previous_checkpoint_hako_elapsed_median_ms=610' "$report" "tool must record previous checkpoint"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'selected_next=post_static_scalar_source_mir_refresh' "$report" "tool must select refresh"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
