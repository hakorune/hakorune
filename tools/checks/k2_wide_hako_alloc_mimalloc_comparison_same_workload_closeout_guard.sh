#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-same-workload-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-294x/294x-163-MIMALLOC-COMPARISON-SAME-WORKLOAD-PACK-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-294x/294x-162-MIMALLOC-COMPARISON-SAME-WORKLOAD-MEMORY-REPORT.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SAME_WORKLOAD_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_same_workload_memory_report_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_same_workload_closeout_guard.sh"

echo "[$TAG] checking same-workload memory report closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SAME_WORKLOAD_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SAME_WORKLOAD_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "same-workload memory report row must be landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$CARD" "closeout card must be landed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SAME-WORKLOAD-PACK-CLOSEOUT-001' "$CARD" "closeout card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RSS-PRESENTATION-001' "$CARD" "closeout card must select RSS presentation follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RSS-PRESENTATION-001' "$TASKBOARD" "taskboard must expose RSS presentation follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this closeout guard"
guard_expect_in_file "$TAG" 'winner claim' "$CARD" "closeout must keep winner claim wording explicit"

bash "$SAME_WORKLOAD_GUARD"

echo "[$TAG] ok"
