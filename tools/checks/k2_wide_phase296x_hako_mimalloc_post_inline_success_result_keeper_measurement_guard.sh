#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-post-inline-success-result-keeper-measurement"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_110="docs/development/current/main/phases/phase-296x/296x-110-HAKO-MIMALLOC-POST-INLINE-SUCCESS-RESULT-KEEPER-MEASUREMENT.md"
CARD_111="docs/development/current/main/phases/phase-296x/296x-111-HAKO-MIMALLOC-ROLLBACK-INLINE-SUCCESS-RESULT-KEEPER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_inline_success_result_keeper_measurement.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_post_inline_success_result_keeper_measurement_guard.sh"

echo "[$TAG] checking post-inline-success-result keeper measurement"

guard_require_files "$TAG" "$CARD_110" "$CARD_111" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_110" "row110 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_111" "row111 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-inline-success-result-keeper-measurement-v0' "$CARD_110" "row110 must record output contract"
guard_expect_fixed_in_file "$TAG" 'after_hako_elapsed_median_ms=630' "$CARD_110" "row110 must record median"
guard_expect_fixed_in_file "$TAG" 'keeper_effect=regressed' "$CARD_110" "row110 must mark regression"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-110-HAKO-MIMALLOC-POST-INLINE-SUCCESS-RESULT-KEEPER-MEASUREMENT"' "$CURRENT_STATE" "current state latest card must advance to row110"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-ROLLBACK-INLINE-SUCCESS-RESULT-KEEPER-296X-001"' "$CURRENT_STATE" "current state must select row111"
guard_expect_fixed_in_file "$TAG" '| 110 | `HAKO-MIMALLOC-POST-INLINE-SUCCESS-RESULT-KEEPER-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" "taskboard row110 must be landed"
guard_expect_fixed_in_file "$TAG" '| 111 | `HAKO-MIMALLOC-ROLLBACK-INLINE-SUCCESS-RESULT-KEEPER-296X-001` | Current |' "$TASKBOARD" "taskboard row111 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_inline_success_measurement.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --sample-count 1 --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-inline-success-result-keeper-measurement-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=hako-mimalloc-small-alloc-inline-success-result-fast-path-keeper-v0' "$report" "tool must record input contract"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "tool must use 8192 repeat"
guard_expect_fixed_in_file "$TAG" 'sample_count=1' "$report" "tool must honor sample count"
guard_expect_fixed_in_file "$TAG" 'select_page_single_fallback_count=0' "$report" "tool must preserve select fallback count"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fast_path_count=524288' "$report" "tool must preserve release fast count"
guard_expect_fixed_in_file "$TAG" 'after_hako_elapsed_median_ms=' "$report" "tool must emit median"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
