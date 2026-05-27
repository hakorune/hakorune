#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-post-page-acquire-usize-fast-path-measurement"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_153="docs/development/current/main/phases/phase-296x/296x-153-POST-PAGE-ACQUIRE-USIZE-FAST-PATH-MEASUREMENT.md"
CARD_154="docs/development/current/main/phases/phase-296x/296x-154-POST-PAGE-ACQUIRE-USIZE-SOURCE-MIR-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/post_page_acquire_usize_fast_path_measurement.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_page_acquire_usize_fast_path_measurement_guard.sh"

echo "[$TAG] checking post page acquire_usize fast path measurement"

guard_require_files "$TAG" "$CARD_153" "$CARD_154" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_153" "row153 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_154" "row154 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=post-page-acquire-usize-fast-path-measurement-v0' "$CARD_153" "row153 must record output contract"
guard_expect_fixed_in_file "$TAG" 'previous_checkpoint_median_ms=600' "$CARD_153" "row153 must record checkpoint"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-153-POST-PAGE-ACQUIRE-USIZE-FAST-PATH-MEASUREMENT"' "$CURRENT_STATE" "current state latest card must advance to row153"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "POST-PAGE-ACQUIRE-USIZE-SOURCE-MIR-REFRESH-296X-001"' "$CURRENT_STATE" "current state must select row154"
guard_expect_fixed_in_file "$TAG" '| 153 | `POST-PAGE-ACQUIRE-USIZE-FAST-PATH-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" "taskboard row153 must be landed"
guard_expect_fixed_in_file "$TAG" '| 154 | `POST-PAGE-ACQUIRE-USIZE-SOURCE-MIR-REFRESH-296X-001` | Current |' "$TASKBOARD" "taskboard row154 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_acquire_usize_measurement.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
"$TOOL" --sample-count 1 --timeout-seconds 240 --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=post-page-acquire-usize-fast-path-measurement-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "tool must measure full repeat"
guard_expect_fixed_in_file "$TAG" 'sample_count=1' "$report" "tool must run scout sample"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fast_path_count=524288' "$report" "tool must preserve fast path count"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fallback_count=0' "$report" "tool must preserve fallback count"
guard_expect_fixed_in_file "$TAG" 'previous_checkpoint_median_ms=600' "$report" "tool must compare to row149 checkpoint"
guard_expect_fixed_in_file "$TAG" 'keeper_effect=' "$report" "tool must classify keeper"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
