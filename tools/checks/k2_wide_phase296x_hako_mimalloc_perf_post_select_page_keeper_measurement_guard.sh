#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-post-select-page-keeper-measurement"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_82="docs/development/current/main/phases/phase-296x/296x-82-HAKO-MIMALLOC-PERF-POST-SELECT-PAGE-KEEPER-MEASUREMENT.md"
CARD_83="docs/development/current/main/phases/phase-296x/296x-83-HAKO-CHECK-PERF-SURFACE-V1-MINIMAL.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_select_page_keeper_measurement.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_post_select_page_keeper_measurement_guard.sh"

echo "[$TAG] checking post-selectPage keeper measurement"

guard_require_files "$TAG" "$CARD_82" "$CARD_83" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_82" "measurement card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_83" "v1 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-perf-post-select-page-keeper-measurement-v0' "$CARD_82" "card must record output contract"
guard_expect_fixed_in_file "$TAG" 'select_page_single_fast_path_count=524288' "$CARD_82" "card must record fast count"
guard_expect_fixed_in_file "$TAG" 'select_page_single_fallback_count=0' "$CARD_82" "card must record fallback count"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-82-HAKO-MIMALLOC-PERF-POST-SELECT-PAGE-KEEPER-MEASUREMENT"' "$CURRENT_STATE" "current state latest card must advance to row 82"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-CHECK-PERF-SURFACE-V1-MINIMAL-296X-001"' "$CURRENT_STATE" "current state must select row 83"
guard_expect_fixed_in_file "$TAG" '| 82 | `HAKO-MIMALLOC-PERF-POST-SELECT-PAGE-KEEPER-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 82 must be landed"
guard_expect_fixed_in_file "$TAG" '| 83 | `HAKO-CHECK-PERF-SURFACE-V1-MINIMAL-296X-001` | Current |' "$TASKBOARD" "taskboard row 83 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_select_measurement.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --sample-count 3 --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-perf-post-select-page-keeper-measurement-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "tool must use 8192 repeat"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$report" "tool must collect 3 samples"
guard_expect_fixed_in_file "$TAG" 'select_page_single_fast_path_count=524288' "$report" "tool must preserve select fast count"
guard_expect_fixed_in_file "$TAG" 'select_page_single_fallback_count=0' "$report" "tool must preserve select fallback count"
guard_expect_fixed_in_file "$TAG" 'after_hako_elapsed_median_ms=' "$report" "tool must emit median"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
