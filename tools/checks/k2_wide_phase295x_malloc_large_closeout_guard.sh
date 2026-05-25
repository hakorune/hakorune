#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-malloc-large-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-189-MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-188-MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-EVIDENCE-RUN.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_malloc_large_closeout_guard.sh"
EVIDENCE_GUARD="tools/checks/k2_wide_phase295x_malloc_large_evidence_run_guard.sh"

echo "[$TAG] checking phase-295x malloc-large closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$EVIDENCE_GUARD"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$EVIDENCE_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-NEXT-WORKLOAD-SEAM-SELECTION-295X-001' "$CARD" "card must select the next workload seam"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT-295X-001' "$PREV_CARD" "previous row must select this closeout"
guard_expect_in_file "$TAG" '| 188 | `MIMALLOC-COMPARISON-MIMALLOC-BENCH-MALLOC-LARGE-EVIDENCE-295X-RUN-001` | Landed |' "$TASKBOARD" "taskboard must keep the evidence row landed"
guard_expect_in_file "$TAG" '| 189 | `MIMALLOC-COMPARISON-MALLOC-LARGE-CLOSEOUT-295X-001` | Current |' "$TASKBOARD" "taskboard must expose the closeout row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" "$EVIDENCE_GUARD" "$INDEX" "check script index must list the evidence guard"
guard_expect_in_file "$TAG" 'malloc-large' "$CARD" "closeout must keep the workload family name"
guard_expect_in_file "$TAG" 'next workload seam' "$CARD" "closeout must keep the follow-on seam intent"

bash "$EVIDENCE_GUARD"

echo "[$TAG] ok"
