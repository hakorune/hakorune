#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-next-workload-seam-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-15-MIMALLOC-COMPARISON-NEXT-WORKLOAD-SEAM-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-14-MIMALLOC-COMPARISON-COUNT-EVIDENCE-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_next_workload_seam_selection_guard.sh"

echo "[$TAG] checking phase-295x next workload seam selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NEXT-WORKLOAD-SEAM-SELECTION-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-CONTRACT-295X-REFRESH-001' "$CARD" "card must select realloc/aligned contract refresh"
guard_expect_in_file "$TAG" 'representative-realloc-aligned-v0' "$CARD" "card must select the realloc/aligned workload id"
guard_expect_in_file "$TAG" 'operation_sequence_id=representative-realloc-aligned-v0-seq' "$CARD" "card must require operation sequence id"
guard_expect_in_file "$TAG" 'free_order_id=ascending-release-v0' "$CARD" "card must require free order id"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NEXT-WORKLOAD-SEAM-SELECTION-295X-001' "$PREV_CARD" "previous row must select this seam selection"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REALLOC-ALIGNED-CONTRACT-295X-REFRESH-001' "$TASKBOARD" "taskboard must expose realloc/aligned contract refresh"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'winner claims remain closed' "$CARD" "selection must keep winner claims closed"

echo "[$TAG] ok"
