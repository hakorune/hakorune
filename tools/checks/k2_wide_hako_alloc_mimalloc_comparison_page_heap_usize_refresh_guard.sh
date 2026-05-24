#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-page-heap-usize-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-294x/294x-268-MIMALLOC-COMPARISON-VSLICE-PAGE-HEAP-USIZE-REFRESH.md"
SELECTION_CARD="docs/development/current/main/phases/phase-294x/294x-267-HAKO-ALLOC-USIZE-NEXT-FIELD-GROUP-SELECTION.md"
PAGE_HEAP_CLOSEOUT_CARD="docs/development/current/main/phases/phase-294x/294x-266-HAKO-ALLOC-USIZE-PAGE-HEAP-NON-ID-CLOSEOUT.md"
VS_REFRESH_CARD="docs/development/current/main/phases/phase-294x/294x-228-MIMALLOC-COMPARISON-VSLICE-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_page_heap_usize_refresh_guard.sh"
PAGE_HEAP_GUARD="tools/checks/k2_wide_hako_alloc_usize_page_heap_non_id_closeout_guard.sh"
VS_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_vertical_slice_closeout_guard.sh"

echo "[$TAG] checking page-heap usize refreshed mimalloc comparison vertical slice"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$SELECTION_CARD" \
  "$PAGE_HEAP_CLOSEOUT_CARD" \
  "$VS_REFRESH_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$PAGE_HEAP_GUARD" \
  "$VS_GUARD"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$PAGE_HEAP_GUARD" "$VS_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-VSLICE-PAGE-HEAP-USIZE-REFRESH-001' "$CARD" "card must identify the refresh blocker"
guard_expect_in_file "$TAG" 'HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-011' "$CARD" "card must select the next field-group decision token"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-VSLICE-PAGE-HEAP-USIZE-REFRESH-001' "$SELECTION_CARD" "selection card must select this blocker"
guard_expect_in_file "$TAG" '294x-268' "$TASKBOARD" "taskboard must record the landed refresh row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'schema=vertical-slice-v1' "$CARD" "card must preserve the V5 schema evidence"
guard_expect_in_file "$TAG" 'hako_requested=48,216,4194321,4194585' "$CARD" "card must preserve hako requested evidence"
guard_expect_in_file "$TAG" 'c_mimalloc=1,1,1,1,64,64,33254,4096,4096,0,1' "$CARD" "card must preserve C mimalloc evidence"

bash "$PAGE_HEAP_GUARD"
bash "$VS_GUARD"

echo "[$TAG] ok"
