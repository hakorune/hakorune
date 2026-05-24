#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mixed-size-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-23-MIMALLOC-COMPARISON-MIXED-SIZE-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-22-MIMALLOC-COMPARISON-MIXED-SIZE-EVIDENCE-RUN.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mixed_size_closeout_guard.sh"
EVIDENCE_GUARD="tools/checks/k2_wide_phase295x_mixed_size_evidence_run_guard.sh"

echo "[$TAG] checking phase-295x mixed-size closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$EVIDENCE_GUARD"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$EVIDENCE_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-CLOSEOUT-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HUGE-ISH-WORKLOAD-SEAM-SELECTION-295X-001' "$CARD" "card must select huge-ish seam selection"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-CLOSEOUT-295X-001' "$PREV_CARD" "previous row must select this closeout"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HUGE-ISH-WORKLOAD-SEAM-SELECTION-295X-001' "$TASKBOARD" "taskboard must expose huge-ish seam selection follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" "$EVIDENCE_GUARD" "$INDEX" "check script index must list evidence guard"
guard_expect_in_file "$TAG" 'winner claims still closed' "$CARD" "closeout must keep winner claims closed"

bash "$EVIDENCE_GUARD"

echo "[$TAG] ok"
