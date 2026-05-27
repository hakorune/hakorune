#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-small-alloc-selected-page-cache-keeper"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_89="docs/development/current/main/phases/phase-296x/296x-89-HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-CACHE-KEEPER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_small_alloc_selected_page_cache_keeper.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_selected_page_cache_keeper_guard.sh"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"

echo "[$TAG] checking small-alloc selected-page cache keeper"

guard_require_files "$TAG" "$CARD_89" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT" "$FACADE"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_89" "row89 card must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-selected-page-cache-keeper-v0' "$CARD_89" "row89 must record output contract"
guard_expect_fixed_in_file "$TAG" 'selected_page_cache_reused=1' "$CARD_89" "row89 must record selected page cache reuse"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-89-HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-CACHE-KEEPER"' "$CURRENT_STATE" "current state latest card must advance to row89"
guard_expect_fixed_in_file "$TAG" '| 89 | `HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-CACHE-KEEPER-296X-001` | Landed |' "$TASKBOARD" "taskboard row 89 must be landed"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_small_alloc_cache_keeper.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-selected-page-cache-keeper-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'keeper=small_alloc_selected_page_cache_reuse' "$report" "tool must record keeper"
guard_expect_fixed_in_file "$TAG" 'keeper_kind=box_count' "$report" "tool must keep keeper kind narrow"
guard_expect_fixed_in_file "$TAG" 'selected_page_cache_reused=1' "$report" "tool must prove selected page cache reuse"
guard_expect_fixed_in_file "$TAG" 'removed_repeated_pages_get=1' "$report" "tool must prove repeated get removal"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
