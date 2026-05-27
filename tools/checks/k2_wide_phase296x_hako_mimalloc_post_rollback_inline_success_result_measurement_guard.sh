#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-post-rollback-inline-success-result-measurement"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_112="docs/development/current/main/phases/phase-296x/296x-112-HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-RESULT-MEASUREMENT.md"
CARD_113="docs/development/current/main/phases/phase-296x/296x-113-HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-SOURCE-MIR-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_rollback_inline_success_result_measurement.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_post_rollback_inline_success_result_measurement_guard.sh"

echo "[$TAG] checking post-rollback inline success result measurement"

guard_require_files "$TAG" "$CARD_112" "$CARD_113" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_112" "row112 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_113" "row113 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-rollback-inline-success-result-measurement-v0' "$CARD_112" "row112 must record output contract"
guard_expect_fixed_in_file "$TAG" 'after_hako_elapsed_median_ms=620' "$CARD_112" "row112 must record median"
guard_expect_fixed_in_file "$TAG" 'baseline_exact_reproduced=1' "$CARD_112" "row112 must reproduce baseline"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-112-HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-RESULT-MEASUREMENT"' "$CURRENT_STATE" "current state latest card must advance to row112"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-SOURCE-MIR-REFRESH-296X-001"' "$CURRENT_STATE" "current state must select row113"
guard_expect_fixed_in_file "$TAG" '| 112 | `HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-RESULT-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" "taskboard row112 must be landed"
guard_expect_fixed_in_file "$TAG" '| 113 | `HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-SOURCE-MIR-REFRESH-296X-001` | Current |' "$TASKBOARD" "taskboard row113 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_rollback_inline_measurement.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --sample-count 1 --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-rollback-inline-success-result-measurement-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=hako-mimalloc-rollback-inline-success-result-keeper-v0' "$report" "tool must record input contract"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "tool must use 8192 repeat"
guard_expect_fixed_in_file "$TAG" 'sample_count=1' "$report" "tool must honor sample count"
guard_expect_fixed_in_file "$TAG" 'select_page_single_fallback_count=0' "$report" "tool must preserve select fallback count"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fast_path_count=524288' "$report" "tool must preserve release fast count"
guard_expect_fixed_in_file "$TAG" 'after_hako_elapsed_median_ms=' "$report" "tool must emit median"
guard_expect_fixed_in_file "$TAG" 'rollback_confirmed=1' "$report" "tool must confirm rollback"
guard_expect_fixed_in_file "$TAG" 'baseline_exact_reproduced=1' "$report" "tool must reproduce baseline"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
