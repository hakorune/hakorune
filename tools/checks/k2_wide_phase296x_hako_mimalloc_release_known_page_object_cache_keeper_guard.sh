#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-release-known-page-object-cache-keeper"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_92="docs/development/current/main/phases/phase-296x/296x-92-HAKO-MIMALLOC-RELEASE-KNOWN-PAGE-OBJECT-CACHE-KEEPER.md"
CARD_93="docs/development/current/main/phases/phase-296x/296x-93-HAKO-MIMALLOC-POST-RELEASE-OBJECT-CACHE-KEEPER-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_release_known_page_object_cache_keeper.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_release_known_page_object_cache_keeper_guard.sh"

echo "[$TAG] checking release known-page object cache keeper"

guard_require_files "$TAG" "$CARD_92" "$CARD_93" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_92" "row92 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_93" "row93 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-release-known-page-object-cache-keeper-v0' "$CARD_92" "row92 must record output contract"
guard_expect_fixed_in_file "$TAG" 'release_known_page_object_cache_reused=1' "$CARD_92" "row92 must record object cache reuse"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-92-HAKO-MIMALLOC-RELEASE-KNOWN-PAGE-OBJECT-CACHE-KEEPER"' "$CURRENT_STATE" "current state latest card must advance to row92"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-POST-RELEASE-OBJECT-CACHE-KEEPER-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row93"
guard_expect_fixed_in_file "$TAG" '| 92 | `HAKO-MIMALLOC-RELEASE-KNOWN-PAGE-OBJECT-CACHE-KEEPER-296X-001` | Landed |' "$TASKBOARD" "taskboard row92 must be landed"
guard_expect_fixed_in_file "$TAG" '| 93 | `HAKO-MIMALLOC-POST-RELEASE-OBJECT-CACHE-KEEPER-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row93 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_release_object_cache_keeper.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-release-known-page-object-cache-keeper-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'keeper=release_known_page_object_cache' "$report" "tool must record keeper"
guard_expect_fixed_in_file "$TAG" 'keeper_kind=box_shape' "$report" "tool must record keeper kind"
guard_expect_fixed_in_file "$TAG" 'release_known_page_object_cache_reused=1' "$report" "tool must prove cache reuse"
guard_expect_fixed_in_file "$TAG" 'fallback_pages_get_preserved=1' "$report" "tool must preserve fallback get"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
