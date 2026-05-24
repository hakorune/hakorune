#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-realloc-aligned-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-19-MIMALLOC-COMPARISON-REALLOC-ALIGNED-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-18-MIMALLOC-COMPARISON-REALLOC-ALIGNED-EVIDENCE-RUN.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_realloc_aligned_closeout_guard.sh"
EVIDENCE_GUARD="tools/checks/k2_wide_phase295x_realloc_aligned_evidence_run_guard.sh"

echo "[$TAG] checking phase-295x realloc/aligned closeout"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PREV_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$EVIDENCE_GUARD"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$EVIDENCE_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-CLOSEOUT-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-WORKLOAD-SEAM-SELECTION-295X-001' "$CARD" "card must select mixed-size seam selection"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-CLOSEOUT-295X-001' "$PREV_CARD" "previous row must select this closeout"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MIXED-SIZE-WORKLOAD-SEAM-SELECTION-295X-001' "$TASKBOARD" "taskboard must expose mixed-size seam selection follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" "$EVIDENCE_GUARD" "$INDEX" "check script index must list evidence guard"
guard_expect_in_file "$TAG" 'winner claims still closed' "$CARD" "closeout must keep winner claims closed"
guard_expect_in_file "$TAG" 'moved/copy/RSS preserved as evidence-only fields' "$CARD" "closeout must preserve non-parity evidence boundary"

bash "$EVIDENCE_GUARD"

echo "[$TAG] ok"
