#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-small-alloc-page-acquire-usize-fast-path-implementation"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_152="docs/development/current/main/phases/phase-296x/296x-152-SMALL-ALLOC-PAGE-ACQUIRE-USIZE-FAST-PATH-IMPLEMENTATION.md"
CARD_153="docs/development/current/main/phases/phase-296x/296x-153-POST-PAGE-ACQUIRE-USIZE-FAST-PATH-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/small_alloc_page_acquire_usize_fast_path_implementation.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_small_alloc_page_acquire_usize_fast_path_implementation_guard.sh"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"

echo "[$TAG] checking small alloc page acquire_usize fast path implementation"

guard_require_files "$TAG" "$CARD_152" "$CARD_153" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT" "$FACADE" "$PAGE"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_152" "row152 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_153" "row153 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=small-alloc-page-acquire-usize-fast-path-implementation-v0' "$CARD_152" "row152 must record output contract"
guard_expect_fixed_in_file "$TAG" 'selected_keeper=small_alloc_page_acquire_usize_fast_path' "$CARD_152" "row152 must record selected keeper"
guard_expect_fixed_in_file "$TAG" 'full_repeat_measurement_executed=0' "$CARD_152" "row152 must leave full repeat to row153"
guard_expect_fixed_in_file "$TAG" 'local block_id = page.acquire_usize(size)' "$FACADE" "small alloc must call acquire_usize"
guard_expect_fixed_in_file "$TAG" 'return me.acquire(requested_size)' "$PAGE" "acquire_usize must preserve generic fallback"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-152-SMALL-ALLOC-PAGE-ACQUIRE-USIZE-FAST-PATH-IMPLEMENTATION"' "$CURRENT_STATE" "current state latest card must advance to row152"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "POST-PAGE-ACQUIRE-USIZE-FAST-PATH-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row153"
guard_expect_fixed_in_file "$TAG" '| 152 | `SMALL-ALLOC-PAGE-ACQUIRE-USIZE-FAST-PATH-IMPLEMENTATION-296X-001` | Landed |' "$TASKBOARD" "taskboard row152 must be landed"
guard_expect_fixed_in_file "$TAG" '| 153 | `POST-PAGE-ACQUIRE-USIZE-FAST-PATH-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row153 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_acquire_usize_impl.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
"$TOOL" --timeout-seconds 20 --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=small-alloc-page-acquire-usize-fast-path-implementation-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'keeper_applied=1' "$report" "tool must confirm keeper"
guard_expect_fixed_in_file "$TAG" 'generic_page_acquire_preserved=1' "$report" "tool must preserve generic acquire"
guard_expect_fixed_in_file "$TAG" 'lightweight_exact_exe_proof_ok=1' "$report" "tool must prove lightweight exact-EXE"
guard_expect_fixed_in_file "$TAG" 'output_summary_ok=1' "$report" "tool must preserve app summary"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fast_path_count=64' "$report" "tool must preserve light fast path count"
guard_expect_fixed_in_file "$TAG" 'release_known_page_fallback_count=0' "$report" "tool must preserve fallback count"
guard_expect_fixed_in_file "$TAG" 'full_repeat_measurement_executed=0' "$report" "tool must not run full repeat"
guard_expect_fixed_in_file "$TAG" 'semantic_summary=ok' "$report" "tool must report semantic ok"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
