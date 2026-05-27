#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-page-array-keeper-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_147="docs/development/current/main/phases/phase-296x/296x-147-PAGE-ARRAY-KEEPER-SELECTION.md"
CARD_148="docs/development/current/main/phases/phase-296x/296x-148-RELEASE-DIRECT-CACHED-PAGE-KNOWN-LIVE-RELEASE-IMPLEMENTATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/page_array_keeper_selection.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_page_array_keeper_selection_guard.sh"

echo "[$TAG] checking page-array keeper selection"

guard_require_files "$TAG" "$CARD_147" "$CARD_148" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_147" "row147 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_148" "row148 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=page-array-keeper-selection-v0' "$CARD_147" "row147 must record output contract"
guard_expect_fixed_in_file "$TAG" 'selected_keeper=release_direct_cached_page_known_live_release' "$CARD_147" "row147 must select known-live release keeper"
guard_expect_fixed_in_file "$TAG" 'fallback_preservation=generic_releaseLocal_unchanged' "$CARD_147" "row147 must preserve generic fallback"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-147-PAGE-ARRAY-KEEPER-SELECTION"' "$CURRENT_STATE" "current state latest card must advance to row147"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "RELEASE-DIRECT-CACHED-PAGE-KNOWN-LIVE-RELEASE-IMPLEMENTATION-296X-001"' "$CURRENT_STATE" "current state must select row148"
guard_expect_fixed_in_file "$TAG" '| 147 | `PAGE-ARRAY-KEEPER-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row147 must be landed"
guard_expect_fixed_in_file "$TAG" '| 148 | `RELEASE-DIRECT-CACHED-PAGE-KNOWN-LIVE-RELEASE-IMPLEMENTATION-296X-001` | Current |' "$TASKBOARD" "taskboard row148 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_page_array_keeper.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
"$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=page-array-keeper-selection-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'selected_keeper=release_direct_cached_page_known_live_release' "$report" "tool must select known-live release keeper"
guard_expect_fixed_in_file "$TAG" 'expected_dynamic_weight_reduction=524288' "$report" "tool must record expected reduction"
guard_expect_fixed_in_file "$TAG" 'expected_dynamic_weight_reduction_percent=12' "$report" "tool must record expected percent"
guard_expect_fixed_in_file "$TAG" 'fallback_preservation=generic_releaseLocal_unchanged' "$report" "tool must preserve generic release"
guard_expect_fixed_in_file "$TAG" 'selected_next=release_direct_cached_page_known_live_release_implementation' "$report" "tool must select implementation"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
