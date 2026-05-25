#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-next-semantic-seam-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-240-MIMALLOC-COMPARISON-NEXT-SEMANTIC-SEAM-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-239-MIMALLOC-COMPARISON-PROCESS-REPEAT-PACK-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_next_semantic_seam_selection_guard.sh"

echo "[$TAG] checking phase-295x next semantic seam selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD" "semantic selection bridge must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-NEXT-SEMANTIC-SEAM-SELECTION-295X-002' "$CARD" "semantic selection blocker must be fixed"
guard_expect_fixed_in_file "$TAG" 'New rows are justified only when at least one of these changes:' "$CARD" "card must define row creation criteria"
guard_expect_fixed_in_file "$TAG" 'If only `workload_id` changes while the runner/schema/policy/stop-line stay the' "$CARD" "card must reject workload-id-only row growth"
guard_expect_fixed_in_file "$TAG" 'A. remote-free production facade' "$CARD" "card must include remote-free semantic candidate"
guard_expect_fixed_in_file "$TAG" 'B. abandoned-heap reclaim behavior' "$CARD" "card must include abandoned-heap semantic candidate"
guard_expect_fixed_in_file "$TAG" 'C. huge / OSVM / page-source / purge carryover' "$CARD" "card must include huge/OSVM carryover candidate"
guard_expect_fixed_in_file "$TAG" 'D. phase-295x closeout / carryover boundary' "$CARD" "card must include phase closeout candidate"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION-295X-002' "$CARD" "card must keep the remote-free production facade selected next"
guard_expect_fixed_in_file "$TAG" 'This row does not add samples, add a benchmark workload' "$CARD" "card must keep benchmark-only rows closed"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "process-repeat closeout must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-NEXT-SEMANTIC-SEAM-SELECTION-295X-002' "$PREV_CARD" "previous closeout must select this semantic seam"

guard_expect_fixed_in_file "$TAG" '| 239 | `MIMALLOC-COMPARISON-PROCESS-REPEAT-PACK-CLOSEOUT-295X-002` | Landed |' "$TASKBOARD" "taskboard must keep process-repeat closeout landed"
guard_expect_fixed_in_file "$TAG" '| 240 | `MIMALLOC-COMPARISON-NEXT-SEMANTIC-SEAM-SELECTION-295X-002` | Landed |' "$TASKBOARD" "taskboard must expose semantic selection bridge as landed"
guard_expect_fixed_in_file "$TAG" '| 241 | `MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION-295X-002` | Current |' "$TASKBOARD" "taskboard must expose remote-free selection current row"

guard_expect_fixed_in_file "$TAG" '295x-241-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION' "$CURRENT_STATE" "current state must move on to the remote-free selection card"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION-295X-002' "$CURRENT_STATE" "current state must expose the remote-free selection blocker"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list semantic selection guard"

echo "[$TAG] ok"
