#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-perf-second-keeper-optimization"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_61="docs/development/current/main/phases/phase-296x/296x-61-HAKO-MIMALLOC-PERF-SECOND-KEEPER-OPTIMIZATION.md"
CARD_62="docs/development/current/main/phases/phase-296x/296x-62-HAKO-MIMALLOC-PERF-POST-SECOND-KEEPER-TAXONOMY-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_perf_second_keeper_optimization_guard.sh"

echo "[$TAG] checking phase-296x second keeper optimization"

guard_require_files "$TAG" "$CARD_61" "$CARD_62" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$PAGE_BOX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_61" "second keeper card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_62" "post-second taxonomy card must be current"
guard_expect_fixed_in_file "$TAG" 'optimization_kind=acquire_usize_free_top_fast_path' "$CARD_61" "card must name optimization"
guard_expect_fixed_in_file "$TAG" 'target_phase=alloc' "$CARD_61" "card must keep target phase"
guard_expect_fixed_in_file "$TAG" 'after_full_elapsed_median_ms=260' "$CARD_61" "card must record after median"
guard_expect_fixed_in_file "$TAG" 'improvement_ms=20' "$CARD_61" "card must record improvement"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$CARD_61" "card must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'acquire_usize(requested_size: usize)' "$PAGE_BOX" "page model must keep acquire_usize entry"
guard_expect_fixed_in_file "$TAG" 'if free_top == 0 {' "$PAGE_BOX" "fast path must fall back for empty free stack"
guard_expect_fixed_in_file "$TAG" 'return me.acquire(requested_size)' "$PAGE_BOX" "fast path must preserve local_free fallback"
guard_expect_fixed_in_file "$TAG" 'local block_id = me.free.get(free_top)' "$PAGE_BOX" "fast path must pop from free stack directly"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-61-HAKO-MIMALLOC-PERF-SECOND-KEEPER-OPTIMIZATION"' "$CURRENT_STATE" "current state latest card must advance to row 61"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-PERF-POST-SECOND-KEEPER-TAXONOMY-REFRESH-296X-001"' "$CURRENT_STATE" "current state must select row 62"
guard_expect_fixed_in_file "$TAG" '| 61 | `HAKO-MIMALLOC-PERF-SECOND-KEEPER-OPTIMIZATION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 61 must be landed"
guard_expect_fixed_in_file "$TAG" '| 62 | `HAKO-MIMALLOC-PERF-POST-SECOND-KEEPER-TAXONOMY-REFRESH-296X-001` | Current |' "$TASKBOARD" "taskboard row 62 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
