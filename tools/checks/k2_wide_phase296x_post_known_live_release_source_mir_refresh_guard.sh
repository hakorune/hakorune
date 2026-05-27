#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-post-known-live-release-source-mir-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_150="docs/development/current/main/phases/phase-296x/296x-150-POST-KNOWN-LIVE-RELEASE-SOURCE-MIR-REFRESH.md"
CARD_151="docs/development/current/main/phases/phase-296x/296x-151-PAGE-ACQUIRE-FAST-PATH-KEEPER-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/post_known_live_release_source_mir_refresh.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_known_live_release_source_mir_refresh_guard.sh"

echo "[$TAG] checking post known-live release source/MIR refresh"

guard_require_files "$TAG" "$CARD_150" "$CARD_151" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_150" "row150 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_151" "row151 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=post-known-live-release-source-mir-refresh-v0' "$CARD_150" "row150 must record output contract"
guard_expect_fixed_in_file "$TAG" 'active_owner=allocator_page_array_surface' "$CARD_150" "row150 must select page-array owner"
guard_expect_fixed_in_file "$TAG" 'secondary_owner=compiler_helper_copy' "$CARD_150" "row150 must park compiler helper copy"
guard_expect_fixed_in_file "$TAG" 'selected_next=page_acquire_fast_path_keeper_selection' "$CARD_150" "row150 must select page acquire keeper selection"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-150-POST-KNOWN-LIVE-RELEASE-SOURCE-MIR-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row150"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "PAGE-ACQUIRE-FAST-PATH-KEEPER-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select row151"
guard_expect_fixed_in_file "$TAG" '| 150 | `POST-KNOWN-LIVE-RELEASE-SOURCE-MIR-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row150 must be landed"
guard_expect_fixed_in_file "$TAG" '| 151 | `PAGE-ACQUIRE-FAST-PATH-KEEPER-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row151 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_known_live_refresh.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
"$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=post-known-live-release-source-mir-refresh-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'direct_release_known_live_call_count=1' "$report" "tool must see known-live call"
guard_expect_fixed_in_file "$TAG" 'page_release_known_live_array_get_call_count=0' "$report" "tool must see known-live get removal"
guard_expect_fixed_in_file "$TAG" 'page_acquire_array_get_call_count=2' "$report" "tool must record acquire array gets"
guard_expect_fixed_in_file "$TAG" 'active_owner=allocator_page_array_surface' "$report" "tool must select active owner"
guard_expect_fixed_in_file "$TAG" 'selected_next=page_acquire_fast_path_keeper_selection' "$report" "tool must select next"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
