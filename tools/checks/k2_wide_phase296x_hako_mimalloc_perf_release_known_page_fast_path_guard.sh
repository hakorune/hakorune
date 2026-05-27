#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-release-known-page-fast-path"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_78="docs/development/current/main/phases/phase-296x/296x-78-HAKO-MIMALLOC-PERF-RELEASE-KNOWN-PAGE-FAST-PATH.md"
CARD_79="docs/development/current/main/phases/phase-296x/296x-79-HAKO-MIMALLOC-PERF-POST-RELEASE-KEEPER-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TARGET="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
PROOF_APP="apps/mimalloc-facade-release-one-block-proof/main.hako"
TOOL="tools/allocator/hako_mimalloc_release_known_page_fast_path.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_release_known_page_fast_path_guard.sh"

echo "[$TAG] checking release known-page fast path keeper"

guard_require_files "$TAG" "$CARD_78" "$CARD_79" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TARGET" "$PROOF_APP" "$TOOL" "$SELF_SCRIPT" "target/debug/hakorune"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT" "target/debug/hakorune"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_78" "release keeper card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_79" "post-keeper measurement card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-perf-release-known-page-fast-path-v0' "$CARD_78" "card must record output contract"
guard_expect_fixed_in_file "$TAG" 'keeper=release_known_page_fast_path' "$CARD_78" "card must name keeper"
guard_expect_fixed_in_file "$TAG" 'release_uses_known_page_fast_path=1' "$CARD_78" "card must record keeper"
guard_expect_fixed_in_file "$TAG" 'normal_release_route_intact=1' "$CARD_78" "card must keep route intact"

guard_expect_fixed_in_file "$TAG" 'recordLastAllocPage(index, page_id)' "$TARGET" "facade must cache last allocation page"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleReleaseKnownPageIndex(page_id)' "$TARGET" "facade must expose release lookup"
guard_expect_fixed_in_file "$TAG" 'local known_index = me.objectLifecycleReleaseKnownPageIndex(page_id)' "$TARGET" "release must use fast-path lookup"
guard_expect_fixed_in_file "$TAG" 'return me.objectLifecycleKnownPageIndexById(page_id)' "$TARGET" "fallback scan must remain"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleReleaseKnownPageFastPathCount()' "$TARGET" "fast observer must exist"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleReleaseKnownPageFallbackCount()' "$TARGET" "fallback observer must exist"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleReleaseKnownPageFastPathCount()' "$PROOF_APP" "proof app must read fast-path count"
guard_expect_fixed_in_file "$TAG" 'objectLifecycleReleaseKnownPageFallbackCount()' "$PROOF_APP" "proof app must read fallback count"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-78-HAKO-MIMALLOC-PERF-RELEASE-KNOWN-PAGE-FAST-PATH"' "$CURRENT_STATE" "current state latest card must advance to row 78"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-POST-RELEASE-KEEPER-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row 79"
guard_expect_fixed_in_file "$TAG" '| 78 | `HAKO-MIMALLOC-PERF-RELEASE-KNOWN-PAGE-FAST-PATH-296X-001` | Landed |' "$TASKBOARD" "taskboard row 78 must be landed"
guard_expect_fixed_in_file "$TAG" '| 79 | `HAKO-MIMALLOC-PERF-POST-RELEASE-KEEPER-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row 79 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_release_fast_path.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"
target/debug/hakorune --backend vm "$PROOF_APP" > "$tmp_dir/proof.out"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-perf-release-known-page-fast-path-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'keeper=release_known_page_fast_path' "$report" "tool must name keeper"
guard_expect_fixed_in_file "$TAG" 'release_uses_known_page_fast_path=1' "$report" "tool must validate release lookup"
guard_expect_fixed_in_file "$TAG" 'normal_release_route_intact=1' "$report" "tool must keep fallback"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"
guard_expect_fixed_in_file "$TAG" 'release_known_page=1,0' "$tmp_dir/proof.out" "proof must hit fast path without fallback"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$tmp_dir/proof.out" "proof must pass"

echo "[$TAG] ok"
