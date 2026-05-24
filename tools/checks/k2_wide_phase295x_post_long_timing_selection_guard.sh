#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-post-long-timing-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-72-MIMALLOC-COMPARISON-POST-LONG-TIMING-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-71-MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_post_long_timing_selection_guard.sh"

echo "[$TAG] checking phase-295x post-long-timing selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-POST-LONG-TIMING-SELECTION-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-PACK-295X-001' "$CARD" "card must select presentation pack"
guard_expect_in_file "$TAG" 'timing_repeat_kind=process-invocation-v0' "$CARD" "card must preserve timing repeat kind"
guard_expect_in_file "$TAG" 'allocator-body timing' "$CARD" "card must distinguish allocator-body timing"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-POST-LONG-TIMING-SELECTION-295X-001' "$PREV_CARD" "previous card must select this row"
guard_expect_in_file "$TAG" '295x-73' "$TASKBOARD" "taskboard must expose selected presentation pack row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"
