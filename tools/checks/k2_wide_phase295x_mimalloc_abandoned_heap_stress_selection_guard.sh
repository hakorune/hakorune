#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-214-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-213-MIMALLOC-COMPARISON-PAR-STRESS-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_selection_guard.sh"

echo "[$TAG] checking phase-295x abandoned-heap stress selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Current' "$CARD" "card must remain current while the selection row is being exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SELECTION-295X-002' "$CARD" "card must identify the selection blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CONTRACT-REFRESH-295X-002' "$CARD" "card must select the abandoned-heap stress contract refresh"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous closeout row must be landed before selection"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SELECTION-295X-002' "$PREV_CARD" "previous row must select this selection row"
guard_expect_in_file "$TAG" '| 213 | `MIMALLOC-COMPARISON-PAR-STRESS-CLOSEOUT-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the closeout row as landed"
guard_expect_in_file "$TAG" '| 214 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SELECTION-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the selection row as current"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"
