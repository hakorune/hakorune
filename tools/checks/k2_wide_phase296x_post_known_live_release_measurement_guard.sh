#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-post-known-live-release-measurement"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_149="docs/development/current/main/phases/phase-296x/296x-149-POST-KNOWN-LIVE-RELEASE-MEASUREMENT.md"
CARD_150="docs/development/current/main/phases/phase-296x/296x-150-POST-KNOWN-LIVE-RELEASE-SOURCE-MIR-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/post_known_live_release_measurement.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_known_live_release_measurement_guard.sh"

echo "[$TAG] checking post known-live release measurement"

guard_require_files "$TAG" "$CARD_149" "$CARD_150" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_149" "row149 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_150" "row150 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=post-known-live-release-measurement-v0' "$CARD_149" "row149 must record output contract"
guard_expect_fixed_in_file "$TAG" 'sample_count=1' "$CARD_149" "row149 must be a one-sample scout measurement"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_149" "row149 must keep winner claim closed"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-149-POST-KNOWN-LIVE-RELEASE-MEASUREMENT"' "$CURRENT_STATE" "current state latest card must advance to row149"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "POST-KNOWN-LIVE-RELEASE-SOURCE-MIR-REFRESH-296X-001"' "$CURRENT_STATE" "current state must select row150"
guard_expect_fixed_in_file "$TAG" '| 149 | `POST-KNOWN-LIVE-RELEASE-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" "taskboard row149 must be landed"
guard_expect_fixed_in_file "$TAG" '| 150 | `POST-KNOWN-LIVE-RELEASE-SOURCE-MIR-REFRESH-296X-001` | Current |' "$TASKBOARD" "taskboard row150 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_known_live_measurement.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
"$TOOL" --sample-count 1 --timeout-seconds 240 --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=post-known-live-release-measurement-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'sample_count=1' "$report" "tool must record sample count"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fast_path_count=524288' "$report" "tool must preserve fast path count"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fallback_count=0' "$report" "tool must preserve fallback count"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner claim closed"
guard_expect_fixed_in_file "$TAG" 'selected_next=post_known_live_release_source_mir_refresh' "$report" "tool must select source/MIR refresh"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
