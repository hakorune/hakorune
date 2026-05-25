#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-high-res-timing-seam-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-233-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-HIGH-RES-TIMING-SEAM-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-232-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_high_res_timing_seam_selection_guard.sh"

echo "[$TAG] checking phase-295x abandoned-heap stress high-res timing seam selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD" "card must be current while the selection row is exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-HIGH-RES-TIMING-SEAM-SELECTION-295X-002' "$CARD" "card must identify the selection blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-LONG-PROCESS-REPEAT-TIMING-PACK-295X-002' "$CARD" "card must select the long process-repeat timing pack"
guard_expect_in_file "$TAG" 'timing_repeat_kind=process-invocation-v0' "$CARD" "card must preserve process-repeat timing kind"
guard_expect_in_file "$TAG" 'operation_repeat=128' "$CARD" "card must preserve the repeat count contract"
guard_expect_in_file "$TAG" 'sample_count=3' "$CARD" "card must preserve the repeated sample count"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous observation closeout row must be landed before selection"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-HIGH-RES-TIMING-SEAM-SELECTION-295X-002' "$PREV_CARD" "previous row must select this high-res timing seam"
guard_expect_in_file "$TAG" '| 232 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-CLOSEOUT-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the observation closeout as landed"
guard_expect_in_file "$TAG" '| 233 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-HIGH-RES-TIMING-SEAM-SELECTION-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the new selection row as current"
guard_expect_in_file "$TAG" '233' "$CURRENT_STATE" "current state must point at the selection card"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-LONG-PROCESS-REPEAT-TIMING-PACK-295X-002' "$CURRENT_STATE" "current state must expose the selected next blocker"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"
