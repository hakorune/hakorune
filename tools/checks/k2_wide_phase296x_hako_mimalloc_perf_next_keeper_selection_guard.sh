#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-next-keeper-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_80="docs/development/current/main/phases/phase-296x/296x-80-HAKO-MIMALLOC-PERF-NEXT-KEEPER-SELECTION.md"
CARD_81="docs/development/current/main/phases/phase-296x/296x-81-HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_next_keeper_selection.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_next_keeper_selection_guard.sh"

echo "[$TAG] checking next keeper selection"

guard_require_files "$TAG" "$CARD_80" "$CARD_81" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_80" "next keeper selection card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_81" "select-page keeper card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-perf-next-keeper-selection-v0' "$CARD_80" "card must record output contract"
guard_expect_fixed_in_file "$TAG" 'previous_keeper=release_known_page_fast_path' "$CARD_80" "card must cite previous keeper"
guard_expect_fixed_in_file "$TAG" 'next_keeper=select_page_single_page_fast_path' "$CARD_80" "card must select one next keeper"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_80" "card must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$CARD_80" "card must keep replacement closed"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-80-HAKO-MIMALLOC-PERF-NEXT-KEEPER-SELECTION"' "$CURRENT_STATE" "current state latest card must advance to row 80"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH-296X-001"' "$CURRENT_STATE" "current state must select row 81"
guard_expect_fixed_in_file "$TAG" '| 80 | `HAKO-MIMALLOC-PERF-NEXT-KEEPER-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 80 must be landed"
guard_expect_fixed_in_file "$TAG" '| 81 | `HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH-296X-001` | Current |' "$TASKBOARD" "taskboard row 81 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_next_keeper.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-perf-next-keeper-selection-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=hako-mimalloc-perf-post-release-keeper-measurement-v0' "$report" "tool must consume row79"
guard_expect_fixed_in_file "$TAG" 'previous_keeper=release_known_page_fast_path' "$report" "tool must cite previous keeper"
guard_expect_fixed_in_file "$TAG" 'next_keeper=select_page_single_page_fast_path' "$report" "tool must select next keeper"
guard_expect_fixed_in_file "$TAG" 'implementation_row=HAKO-MIMALLOC-PERF-SELECT-PAGE-SINGLE-PAGE-FAST-PATH-296X-001' "$report" "tool must select implementation row"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
