#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-post-single-eval-fixes-measurement"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_133="docs/development/current/main/phases/phase-296x/296x-133-HAKO-MIMALLOC-POST-SINGLE-EVAL-FIXES-MEASUREMENT.md"
CARD_134="docs/development/current/main/phases/phase-296x/296x-134-MIR-BUILDER-SINGLE-EVAL-SURFACE-SWEEP.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_single_eval_fixes_measurement.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_post_single_eval_fixes_measurement_guard.sh"

echo "[$TAG] checking post single-eval fixes measurement"

guard_require_files "$TAG" "$CARD_133" "$CARD_134" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_133" "row133 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_134" "row134 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-single-eval-fixes-measurement-v0' "$CARD_133" "row133 must record output contract"
guard_expect_fixed_in_file "$TAG" 'measurement_profile=object_lifecycle_facade_exact_exe' "$CARD_133" "row133 must record measurement profile"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$CARD_133" "row133 must record sample count"
guard_expect_fixed_in_file "$TAG" 'previous_checkpoint_hako_elapsed_median_ms=610' "$CARD_133" "row133 must record previous checkpoint"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-133-HAKO-MIMALLOC-POST-SINGLE-EVAL-FIXES-MEASUREMENT"' "$CURRENT_STATE" "current state latest card must advance to row133"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIR-BUILDER-SINGLE-EVAL-SURFACE-SWEEP-296X-001"' "$CURRENT_STATE" "current state must select row134"
guard_expect_fixed_in_file "$TAG" '| 133 | `HAKO-MIMALLOC-POST-SINGLE-EVAL-FIXES-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" "taskboard row133 must be landed"
guard_expect_fixed_in_file "$TAG" '| 134 | `MIR-BUILDER-SINGLE-EVAL-SURFACE-SWEEP-296X-001` | Current |' "$TASKBOARD" "taskboard row134 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_single_eval_measurement.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --sample-count 1 --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-single-eval-fixes-measurement-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'measurement_scope=object_lifecycle_facade_exact_exe_after_single_eval_fixes' "$report" "tool must name scope"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "tool must use 8192 repeat"
guard_expect_fixed_in_file "$TAG" 'sample_count=1' "$report" "tool must support cheap guard sample count"
guard_expect_fixed_in_file "$TAG" 'previous_checkpoint_hako_elapsed_median_ms=610' "$report" "tool must record previous checkpoint"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
