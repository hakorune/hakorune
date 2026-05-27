#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-post-release-direct-cached-page-keeper-measurement"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_96="docs/development/current/main/phases/phase-296x/296x-96-HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-KEEPER-MEASUREMENT.md"
CARD_97="docs/development/current/main/phases/phase-296x/296x-97-HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-SOURCE-MIR-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_post_release_direct_cached_page_keeper_measurement.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_post_release_direct_cached_page_keeper_measurement_guard.sh"

echo "[$TAG] checking post-release-direct-cached-page keeper measurement"

guard_require_files "$TAG" "$CARD_96" "$CARD_97" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_96" "row96 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_97" "row97 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-release-direct-cached-page-keeper-measurement-v0' "$CARD_96" "row96 must record output contract"
guard_expect_fixed_in_file "$TAG" 'after_hako_elapsed_median_ms=650' "$CARD_96" "row96 must record median"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-96-HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-KEEPER-MEASUREMENT"' "$CURRENT_STATE" "current state latest card must advance to row96"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-SOURCE-MIR-REFRESH-296X-001"' "$CURRENT_STATE" "current state must select row97"
guard_expect_fixed_in_file "$TAG" '| 96 | `HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-KEEPER-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" "taskboard row96 must be landed"
guard_expect_fixed_in_file "$TAG" '| 97 | `HAKO-MIMALLOC-POST-RELEASE-DIRECT-CACHED-PAGE-SOURCE-MIR-REFRESH-296X-001` | Current |' "$TASKBOARD" "taskboard row97 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_release_direct_measurement.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --sample-count 1 --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-post-release-direct-cached-page-keeper-measurement-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'input_contract=hako-mimalloc-release-direct-cached-page-fast-path-keeper-v0' "$report" "tool must record input contract"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "tool must use 8192 repeat"
guard_expect_fixed_in_file "$TAG" 'sample_count=1' "$report" "tool must honor sample count"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fast_path_count=524288' "$report" "tool must preserve release fast count"
guard_expect_fixed_in_file "$TAG" 'after_hako_elapsed_median_ms=' "$report" "tool must emit median"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
