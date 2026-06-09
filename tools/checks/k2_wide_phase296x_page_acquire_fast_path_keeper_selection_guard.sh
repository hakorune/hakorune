#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-page-acquire-fast-path-keeper-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_151="docs/development/current/main/phases/phase-296x/296x-151-PAGE-ACQUIRE-FAST-PATH-KEEPER-SELECTION.md"
CARD_152="docs/development/current/main/phases/phase-296x/296x-152-SMALL-ALLOC-PAGE-ACQUIRE-USIZE-FAST-PATH-IMPLEMENTATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/page_acquire_fast_path_keeper_selection.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_page_acquire_fast_path_keeper_selection_guard.sh"

echo "[$TAG] checking page acquire fast path keeper selection"

guard_require_files "$TAG" "$CARD_151" "$CARD_152" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_151" "row151 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_152" "row152 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=page-acquire-fast-path-keeper-selection-v0' "$CARD_151" "row151 must record output contract"
guard_expect_fixed_in_file "$TAG" 'selected_keeper=small_alloc_page_acquire_usize_fast_path' "$CARD_151" "row151 must select acquire_usize keeper"
guard_expect_fixed_in_file "$TAG" 'fallback_preservation=generic_page_acquire_preserved_when_free_top_is_zero' "$CARD_151" "row151 must preserve generic fallback"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-151-PAGE-ACQUIRE-FAST-PATH-KEEPER-SELECTION"' "$CURRENT_STATE" "current state latest card must advance to row151"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "SMALL-ALLOC-PAGE-ACQUIRE-USIZE-FAST-PATH-IMPLEMENTATION-296X-001"' "$CURRENT_STATE" "current state must select row152"
guard_expect_fixed_in_file "$TAG" '| 151 | `PAGE-ACQUIRE-FAST-PATH-KEEPER-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row151 must be landed"
guard_expect_fixed_in_file "$TAG" '| 152 | `SMALL-ALLOC-PAGE-ACQUIRE-USIZE-FAST-PATH-IMPLEMENTATION-296X-001` | Current |' "$TASKBOARD" "taskboard row152 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_page_acquire_selection.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
"$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=page-acquire-fast-path-keeper-selection-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'active_owner=allocator_page_array_surface' "$report" "tool must preserve active owner"
guard_expect_fixed_in_file "$TAG" 'baseline_page_acquire_mir_instruction_count=185' "$report" "tool must record baseline acquire shape"
guard_expect_fixed_in_file "$TAG" 'candidate_0=small_alloc_page_acquire_usize_fast_path' "$report" "tool must include acquire_usize candidate"
guard_expect_fixed_in_file "$TAG" 'candidate_0_mir_instruction_count=104' "$report" "tool must record acquire_usize shape"
guard_expect_fixed_in_file "$TAG" 'candidate_1=small_alloc_page_acquire_fresh_small_fast_path' "$report" "tool must include fresh-small candidate"
guard_expect_fixed_in_file "$TAG" 'selected_keeper=small_alloc_page_acquire_usize_fast_path' "$report" "tool must select acquire_usize keeper"
guard_expect_fixed_in_file "$TAG" 'rejected_keeper=small_alloc_page_acquire_fresh_small_fast_path' "$report" "tool must reject narrower keeper"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
