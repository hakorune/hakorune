#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-small-alloc-direct-single-page-select-fast-path-keeper"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_106="docs/development/current/main/phases/phase-296x/296x-106-HAKO-MIMALLOC-SMALL-ALLOC-DIRECT-SINGLE-PAGE-SELECT-FAST-PATH-KEEPER.md"
CARD_107="docs/development/current/main/phases/phase-296x/296x-107-HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-KEEPER-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_small_alloc_direct_single_page_select_fast_path_keeper.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_direct_single_page_select_fast_path_keeper_guard.sh"

echo "[$TAG] checking small-alloc direct single-page select keeper"

guard_require_files "$TAG" "$CARD_106" "$CARD_107" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_106" "row106 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_107" "row107 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-direct-single-page-select-fast-path-keeper-v0' "$CARD_106" "row106 must record output contract"
guard_expect_fixed_in_file "$TAG" 'direct_single_page_select_used=1' "$CARD_106" "row106 must record direct select"
guard_expect_fixed_in_file "$TAG" 'generic_select_page_fallback_preserved=1' "$CARD_106" "row106 must preserve fallback"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-106-HAKO-MIMALLOC-SMALL-ALLOC-DIRECT-SINGLE-PAGE-SELECT-FAST-PATH-KEEPER"' "$CURRENT_STATE" "current state latest card must advance to row106"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-KEEPER-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row107"
guard_expect_fixed_in_file "$TAG" '| 106 | `HAKO-MIMALLOC-SMALL-ALLOC-DIRECT-SINGLE-PAGE-SELECT-FAST-PATH-KEEPER-296X-001` | Landed |' "$TASKBOARD" "taskboard row106 must be landed"
guard_expect_fixed_in_file "$TAG" '| 107 | `HAKO-MIMALLOC-POST-SMALL-ALLOC-DIRECT-SELECT-KEEPER-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row107 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_small_alloc_direct_select.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-direct-single-page-select-fast-path-keeper-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'keeper=small_alloc_direct_single_page_select_fast_path' "$report" "tool must record keeper"
guard_expect_fixed_in_file "$TAG" 'keeper_kind=box_count' "$report" "tool must record keeper kind"
guard_expect_fixed_in_file "$TAG" 'direct_single_page_select_used=1' "$report" "tool must prove direct select"
guard_expect_fixed_in_file "$TAG" 'generic_select_page_fallback_preserved=1' "$report" "tool must preserve fallback"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
