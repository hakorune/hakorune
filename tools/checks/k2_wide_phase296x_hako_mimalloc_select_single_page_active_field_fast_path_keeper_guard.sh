#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-select-single-page-active-field-fast-path-keeper"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_101="docs/development/current/main/phases/phase-296x/296x-101-HAKO-MIMALLOC-SELECT-SINGLE-PAGE-ACTIVE-FIELD-FAST-PATH-KEEPER.md"
CARD_102="docs/development/current/main/phases/phase-296x/296x-102-HAKO-MIMALLOC-POST-ACTIVE-FIELD-FAST-PATH-KEEPER-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_select_single_page_active_field_fast_path_keeper.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_select_single_page_active_field_fast_path_keeper_guard.sh"

echo "[$TAG] checking select single-page active field fast path keeper"

guard_require_files "$TAG" "$CARD_101" "$CARD_102" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_101" "row101 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_102" "row102 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-select-single-page-active-field-fast-path-keeper-v0' "$CARD_101" "row101 must record output contract"
guard_expect_fixed_in_file "$TAG" 'active_field_fast_path_used=1' "$CARD_101" "row101 must record active field fast path"
guard_expect_fixed_in_file "$TAG" 'generic_lifecycle_fallback_preserved=1' "$CARD_101" "row101 must preserve generic fallback"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-101-HAKO-MIMALLOC-SELECT-SINGLE-PAGE-ACTIVE-FIELD-FAST-PATH-KEEPER"' "$CURRENT_STATE" "current state latest card must advance to row101"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-POST-ACTIVE-FIELD-FAST-PATH-KEEPER-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row102"
guard_expect_fixed_in_file "$TAG" '| 101 | `HAKO-MIMALLOC-SELECT-SINGLE-PAGE-ACTIVE-FIELD-FAST-PATH-KEEPER-296X-001` | Landed |' "$TASKBOARD" "taskboard row101 must be landed"
guard_expect_fixed_in_file "$TAG" '| 102 | `HAKO-MIMALLOC-POST-ACTIVE-FIELD-FAST-PATH-KEEPER-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row102 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_select_active_field_keeper.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-select-single-page-active-field-fast-path-keeper-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'keeper=select_single_page_active_field_fast_path' "$report" "tool must record keeper"
guard_expect_fixed_in_file "$TAG" 'keeper_kind=box_count' "$report" "tool must record keeper kind"
guard_expect_fixed_in_file "$TAG" 'active_field_fast_path_used=1' "$report" "tool must prove active field fast path"
guard_expect_fixed_in_file "$TAG" 'generic_lifecycle_fallback_preserved=1' "$report" "tool must prove fallback preservation"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
