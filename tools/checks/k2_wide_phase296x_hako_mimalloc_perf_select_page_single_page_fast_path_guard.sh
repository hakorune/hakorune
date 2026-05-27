#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-select-page-single-page-fast-path"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_81="docs/development/current/main/phases/phase-296x/296x-81-HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH.md"
CARD_82="docs/development/current/main/phases/phase-296x/296x-82-HAKO-MIMALLOC-PERF-POST-SELECT-PAGE-KEEPER-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
QUEUE="lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
RUNNER="tools/allocator/hako_exe_memory_runner.sh"
TOOL="tools/allocator/hako_mimalloc_select_page_single_page_fast_path.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_select_page_single_page_fast_path_guard.sh"

echo "[$TAG] checking selectPage single-page fast path keeper"

guard_require_files "$TAG" "$CARD_81" "$CARD_82" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$QUEUE" "$FACADE" "$APP" "$RUNNER" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$RUNNER" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_81" "selectPage keeper card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_82" "post-select measurement card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-perf-select-page-single-page-fast-path-v0' "$CARD_81" "card must record output contract"
guard_expect_fixed_in_file "$TAG" 'keeper=select_page_single_page_fast_path' "$CARD_81" "card must name keeper"
guard_expect_fixed_in_file "$TAG" 'proof_expected_select_page_single=524288,0' "$CARD_81" "card must record proof count"

guard_expect_fixed_in_file "$TAG" 'selectSinglePageFastPath()' "$QUEUE" "queue must expose fast path method"
guard_expect_fixed_in_file "$TAG" 'if me.page_count == 1' "$QUEUE" "queue must gate single page fast path"
guard_expect_fixed_in_file "$TAG" 'return me.selectSinglePageFastPath()' "$QUEUE" "selectPage must use fast path"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleSinglePageFastPathCount()' "$FACADE" "facade must expose fast count"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleSinglePageFallbackCount()' "$FACADE" "facade must expose fallback count"
guard_expect_fixed_in_file "$TAG" 'select_page_single_fast_path_count=' "$APP" "proof app must emit fast count"
guard_expect_fixed_in_file "$TAG" 'select_page_single_fallback_count=' "$RUNNER" "runner must preserve fallback count"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-81-HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH"' "$CURRENT_STATE" "current state latest card must advance to row 81"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-POST-SELECT-PAGE-KEEPER-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row 82"
guard_expect_fixed_in_file "$TAG" '| 81 | `HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH-296X-001` | Landed |' "$TASKBOARD" "taskboard row 81 must be landed"
guard_expect_fixed_in_file "$TAG" '| 82 | `HAKO-MIMALLOC-PERF-POST-SELECT-PAGE-KEEPER-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row 82 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_select_page_keeper.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
runner_report="$tmp_dir/runner.out"
python3 "$TOOL" --out "$report"
bash "$RUNNER" --app "$APP" --workload representative-object-lifecycle-small-block-v0 --runtime-config empty --operation-repeat 1 --out "$runner_report" >/dev/null

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-perf-select-page-single-page-fast-path-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'keeper=select_page_single_page_fast_path' "$report" "tool must name keeper"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"
guard_expect_fixed_in_file "$TAG" 'select_page_single_fast_path_count=524288' "$runner_report" "proof must hit select fast path"
guard_expect_fixed_in_file "$TAG" 'select_page_single_fallback_count=0' "$runner_report" "proof must avoid select fallback"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$runner_report" "proof must pass"

echo "[$TAG] ok"
