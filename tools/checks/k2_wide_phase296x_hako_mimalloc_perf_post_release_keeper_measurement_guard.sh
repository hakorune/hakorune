#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-post-release-keeper-measurement"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_79="docs/development/current/main/phases/phase-296x/296x-79-HAKO-MIMALLOC-PERF-POST-RELEASE-KEEPER-MEASUREMENT.md"
CARD_80="docs/development/current/main/phases/phase-296x/296x-80-HAKO-MIMALLOC-PERF-NEXT-KEEPER-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
RUNNER="tools/allocator/hako_exe_memory_runner.sh"
TOOL="tools/allocator/hako_mimalloc_post_release_keeper_measurement.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_post_release_keeper_measurement_guard.sh"

echo "[$TAG] checking post-release keeper measurement"

guard_require_files "$TAG" "$CARD_79" "$CARD_80" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$APP" "$RUNNER" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$RUNNER" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_79" "measurement card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_80" "next keeper card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-perf-post-release-keeper-measurement-v0' "$CARD_79" "card must record output contract"
guard_expect_fixed_in_file "$TAG" 'previous_checkpoint_comparable=0' "$CARD_79" "card must avoid false checkpoint comparison"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_79" "card must keep winner closed"

guard_expect_fixed_in_file "$TAG" 'release_known_page_fast_path_count=' "$APP" "app must emit fast-path count"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fallback_count=' "$APP" "app must emit fallback count"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fast_path_count=' "$RUNNER" "runner must preserve fast-path count"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fallback_count=' "$RUNNER" "runner must preserve fallback count"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-79-HAKO-MIMALLOC-PERF-POST-RELEASE-KEEPER-MEASUREMENT"' "$CURRENT_STATE" "current state latest card must advance to row 79"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-NEXT-KEEPER-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select row 80"
guard_expect_fixed_in_file "$TAG" '| 79 | `HAKO-MIMALLOC-PERF-POST-RELEASE-KEEPER-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" "taskboard row 79 must be landed"
guard_expect_fixed_in_file "$TAG" '| 80 | `HAKO-MIMALLOC-PERF-NEXT-KEEPER-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row 80 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_release_measurement.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --sample-count 3 --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-perf-post-release-keeper-measurement-v0' "$report" "tool must emit measurement contract"
guard_expect_fixed_in_file "$TAG" 'measurement_scope=object_lifecycle_facade_exact_exe_after_keeper' "$report" "tool must name measurement scope"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "tool must use 8192 in-process repeat"
guard_expect_fixed_in_file "$TAG" 'sample_count=3' "$report" "tool must collect 3 samples"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fast_path_count=524288' "$report" "tool must preserve fast count"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fallback_count=0' "$report" "tool must preserve fallback count"
guard_expect_fixed_in_file "$TAG" 'after_hako_elapsed_median_ms=' "$report" "tool must emit median"
guard_expect_fixed_in_file "$TAG" 'previous_checkpoint_comparable=0' "$report" "tool must avoid false comparison"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
