#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-221-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-220-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_closeout_guard.sh"

echo "[$TAG] checking phase-295x abandoned-heap stress closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD" "card must be landed after the closeout row is exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT-295X-002' "$CARD" "card must identify the closeout blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-002' "$CARD" "card must select the NyRT RSS checkpoint follow-on"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous diagnostic row must be landed before closeout"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT-295X-002' "$PREV_CARD" "previous row must select this closeout row"
guard_expect_in_file "$TAG" '| 220 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the diagnostic row as landed"
guard_expect_in_file "$TAG" '| 221 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the closeout row as landed"
guard_expect_in_file "$TAG" '| 222 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the NyRT diagnostic row as current"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"
