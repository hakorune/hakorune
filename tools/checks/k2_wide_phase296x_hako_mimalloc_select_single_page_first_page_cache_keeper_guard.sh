#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-select-single-page-first-page-cache-keeper"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_98="docs/development/current/main/phases/phase-296x/296x-98-HAKO-MIMALLOC-SELECT-SINGLE-PAGE-FIRST-PAGE-CACHE-KEEPER.md"
CARD_99="docs/development/current/main/phases/phase-296x/296x-99-HAKO-MIMALLOC-POST-SELECT-FIRST-PAGE-CACHE-KEEPER-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_select_single_page_first_page_cache_keeper.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_select_single_page_first_page_cache_keeper_guard.sh"

echo "[$TAG] checking select single-page first-page cache keeper"

guard_require_files "$TAG" "$CARD_98" "$CARD_99" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_98" "row98 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_99" "row99 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-select-single-page-first-page-cache-keeper-v0' "$CARD_98" "row98 must record output contract"
guard_expect_fixed_in_file "$TAG" 'first_page_cache_used=1' "$CARD_98" "row98 must record first-page cache"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-98-HAKO-MIMALLOC-SELECT-SINGLE-PAGE-FIRST-PAGE-CACHE-KEEPER"' "$CURRENT_STATE" "current state latest card must advance to row98"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-POST-SELECT-FIRST-PAGE-CACHE-KEEPER-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row99"
guard_expect_fixed_in_file "$TAG" '| 98 | `HAKO-MIMALLOC-SELECT-SINGLE-PAGE-FIRST-PAGE-CACHE-KEEPER-296X-001` | Landed |' "$TASKBOARD" "taskboard row98 must be landed"
guard_expect_fixed_in_file "$TAG" '| 99 | `HAKO-MIMALLOC-POST-SELECT-FIRST-PAGE-CACHE-KEEPER-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row99 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_select_first_page_cache_keeper.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-select-single-page-first-page-cache-keeper-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'keeper=select_single_page_first_page_cache' "$report" "tool must record keeper"
guard_expect_fixed_in_file "$TAG" 'keeper_kind=box_count' "$report" "tool must record keeper kind"
guard_expect_fixed_in_file "$TAG" 'first_page_cache_used=1' "$report" "tool must prove first-page cache"
guard_expect_fixed_in_file "$TAG" 'removed_single_page_pages_get=1' "$report" "tool must prove get removal"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
