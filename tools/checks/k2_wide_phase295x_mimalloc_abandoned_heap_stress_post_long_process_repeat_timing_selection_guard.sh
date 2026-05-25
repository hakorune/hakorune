#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-post-long-process-repeat-timing-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-235-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-POST-LONG-PROCESS-REPEAT-TIMING-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-234-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-LONG-PROCESS-REPEAT-TIMING-PACK.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_post_long_process_repeat_timing_selection_guard.sh"

echo "[$TAG] checking phase-295x abandoned-heap stress post-long-process-repeat timing selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD" "card must remain landed after the post-long-process selection is exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-POST-LONG-PROCESS-REPEAT-TIMING-SELECTION-295X-002' "$CARD" "card must identify the selection blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-PROCESS-REPEAT-PACK-295X-002' "$CARD" "card must select the reuse-cycle small process-repeat pack"
guard_expect_in_file "$TAG" 'resume the actual `.hako` mimalloc port work' "$CARD" "card must explain why this row resumes port work"
guard_expect_in_file "$TAG" 'allocator-body timing and presentation-only alternatives parked' "$CARD" "card must keep other timing alternatives parked"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous row must be landed before the selection row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-POST-LONG-PROCESS-REPEAT-TIMING-SELECTION-295X-002' "$PREV_CARD" "previous row must select this post-long-process selection"
guard_expect_in_file "$TAG" '| 234 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-LONG-PROCESS-REPEAT-TIMING-PACK-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the long process-repeat pack landed"
guard_expect_in_file "$TAG" '| 235 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-POST-LONG-PROCESS-REPEAT-TIMING-SELECTION-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the post-long-process selection landed"
guard_expect_in_file "$TAG" '| 236 | `MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-PROCESS-REPEAT-PACK-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the reuse-cycle pack as current"
guard_expect_in_file "$TAG" '236' "$CURRENT_STATE" "current state must point at the pack card"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-PROCESS-REPEAT-PACK-295X-002' "$CURRENT_STATE" "current state must expose the pack blocker"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"
