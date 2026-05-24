#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase294x-usize-comparison-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-294x/294x-270-PHASE-294X-USIZE-COMPARISON-CLOSEOUT.md"
SELECTION_CARD="docs/development/current/main/phases/phase-294x/294x-269-HAKO-ALLOC-USIZE-NEXT-FIELD-GROUP-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase294x_usize_comparison_closeout_guard.sh"
REFRESH_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_page_heap_usize_refresh_guard.sh"

echo "[$TAG] checking phase-294x usize comparison closeout"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$SELECTION_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$REFRESH_GUARD"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$REFRESH_GUARD"

guard_expect_in_file "$TAG" 'PHASE-294X-USIZE-COMPARISON-CLOSEOUT-001' "$CARD" "closeout card must identify the blocker"
guard_expect_in_file "$TAG" 'PHASE-294X-POST-CLOSEOUT-ROW-SELECTION-001' "$CARD" "closeout card must select the post-closeout row"
guard_expect_in_file "$TAG" 'PHASE-294X-USIZE-COMPARISON-CLOSEOUT-001' "$SELECTION_CARD" "selection card must select this closeout"
guard_expect_in_file "$TAG" '294x-270' "$TASKBOARD" "taskboard must record the landed closeout row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'Do not extend phase-294x to drain remaining allocator fields.' "$CARD" "closeout must stop broad field migration"

bash "$REFRESH_GUARD"

echo "[$TAG] ok"
