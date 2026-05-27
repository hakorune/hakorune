#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-third-keeper-optimization"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_64="docs/development/current/main/phases/phase-296x/296x-64-HAKO-MIMALLOC-PERF-THIRD-KEEPER-OPTIMIZATION.md"
CARD_65="docs/development/current/main/phases/phase-296x/296x-65-HAKO-MIMALLOC-PERF-POST-THIRD-KEEPER-TAXONOMY-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
FULL_APP="apps/hako-alloc-mimalloc-comparison-in-process-small-block-proof/main.hako"
RESET_ALLOC_APP="apps/hako-alloc-mimalloc-comparison-in-process-reset-alloc-only-proof/main.hako"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_third_keeper_optimization_guard.sh"

echo "[$TAG] checking phase-296x third keeper optimization"

guard_require_files "$TAG" "$CARD_64" "$CARD_65" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$PAGE_BOX" "$FULL_APP" "$RESET_ALLOC_APP" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_64" "third keeper card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_65" "post-third taxonomy card must be current"
guard_expect_fixed_in_file "$TAG" 'optimization_kind=known_active_small_cycle_fast_path' "$CARD_64" "card must name optimization"
guard_expect_fixed_in_file "$TAG" 'target_phase=known_active_small_cycle' "$CARD_64" "card must keep target phase"
guard_expect_fixed_in_file "$TAG" 'after_full_elapsed_median_ms=240' "$CARD_64" "card must record after median"
guard_expect_fixed_in_file "$TAG" 'improvement_ms=10' "$CARD_64" "card must record improvement"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_64" "card must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'acquireFreshSmall(requested_size: usize)' "$PAGE_BOX" "page model must expose known-active acquire"
guard_expect_fixed_in_file "$TAG" 'releaseLocalKnownLive(block_id)' "$PAGE_BOX" "page model must expose known-live release"
guard_expect_fixed_in_file "$TAG" 'page.acquireFreshSmall(requested)' "$FULL_APP" "full app must use known-active acquire"
guard_expect_fixed_in_file "$TAG" 'page.releaseLocalKnownLive(released)' "$FULL_APP" "full app must use known-live release"
guard_expect_fixed_in_file "$TAG" 'page.acquireFreshSmall(requested)' "$RESET_ALLOC_APP" "reset-alloc app must use known-active acquire"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-64-HAKO-MIMALLOC-PERF-THIRD-KEEPER-OPTIMIZATION"' "$CURRENT_STATE" "current state latest card must advance to row 64"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-POST-THIRD-KEEPER-TAXONOMY-REFRESH-296X-001"' "$CURRENT_STATE" "current state must select row 65"
guard_expect_fixed_in_file "$TAG" '| 64 | `HAKO-MIMALLOC-PERF-THIRD-KEEPER-OPTIMIZATION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 64 must be landed"
guard_expect_fixed_in_file "$TAG" '| 65 | `HAKO-MIMALLOC-PERF-POST-THIRD-KEEPER-TAXONOMY-REFRESH-296X-001` | Current |' "$TASKBOARD" "taskboard row 65 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
