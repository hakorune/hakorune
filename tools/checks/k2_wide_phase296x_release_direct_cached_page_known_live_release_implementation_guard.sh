#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-release-direct-cached-page-known-live-release-implementation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_148="docs/development/current/main/phases/phase-296x/296x-148-RELEASE-DIRECT-CACHED-PAGE-KNOWN-LIVE-RELEASE-IMPLEMENTATION.md"
CARD_149="docs/development/current/main/phases/phase-296x/296x-149-POST-KNOWN-LIVE-RELEASE-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/release_direct_cached_page_known_live_release_implementation.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_release_direct_cached_page_known_live_release_implementation_guard.sh"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
SMOKE_APP="apps/hako-alloc-mimalloc-comparison-object-lifecycle-known-live-release-smoke/main.hako"

echo "[$TAG] checking direct cached-page known-live release implementation"

guard_require_files "$TAG" "$CARD_148" "$CARD_149" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT" "$FACADE" "$SMOKE_APP"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_148" "row148 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_149" "row149 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=release-direct-cached-page-known-live-release-implementation-v0' "$CARD_148" "row148 must record output contract"
guard_expect_fixed_in_file "$TAG" 'proof_scope=lightweight_known_live_release_smoke' "$CARD_148" "row148 must use lightweight proof"
guard_expect_fixed_in_file "$TAG" 'full_repeat_measurement_executed=0' "$CARD_148" "row148 must not run full repeat measurement"
guard_expect_fixed_in_file "$TAG" 'page.releaseLocalKnownLive(block_id)' "$FACADE" "direct cached release must use known-live release"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-148-RELEASE-DIRECT-CACHED-PAGE-KNOWN-LIVE-RELEASE-IMPLEMENTATION"' "$CURRENT_STATE" "current state latest card must advance to row148"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "POST-KNOWN-LIVE-RELEASE-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row149"
guard_expect_fixed_in_file "$TAG" '| 148 | `RELEASE-DIRECT-CACHED-PAGE-KNOWN-LIVE-RELEASE-IMPLEMENTATION-296X-001` | Landed |' "$TASKBOARD" "taskboard row148 must be landed"
guard_expect_fixed_in_file "$TAG" '| 149 | `POST-KNOWN-LIVE-RELEASE-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row149 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_known_live_release_light.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
"$TOOL" --timeout-seconds 20 --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=release-direct-cached-page-known-live-release-implementation-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'keeper_applied=1' "$report" "tool must confirm keeper"
guard_expect_fixed_in_file "$TAG" 'generic_release_fallback_preserved=1' "$report" "tool must preserve fallback"
guard_expect_fixed_in_file "$TAG" 'lightweight_exact_exe_proof_ok=1' "$report" "tool must prove lightweight exact-EXE"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fast_path_count=64' "$report" "tool must preserve light fast path count"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fallback_count=0' "$report" "tool must preserve fallback count"
guard_expect_fixed_in_file "$TAG" 'full_repeat_measurement_executed=0' "$report" "tool must not run full repeat"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
