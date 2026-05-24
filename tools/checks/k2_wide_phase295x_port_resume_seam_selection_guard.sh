#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-port-resume-seam-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-78-MIMALLOC-COMPARISON-PORT-RESUME-SEAM-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-77-MIMALLOC-COMPARISON-HAKO-BODY-TIMING-FEASIBILITY-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_port_resume_seam_selection_guard.sh"

echo "[$TAG] checking phase-295x port resume seam selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PORT-RESUME-SEAM-SELECTION-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-CONTRACT-295X-001' "$CARD" "card must select reuse-cycle contract"
guard_expect_in_file "$TAG" 'workload=representative-reuse-cycle-small-v0' "$CARD" "card must define workload id"
guard_expect_in_file "$TAG" 'operation_family=reuse-cycle-small' "$CARD" "card must define operation family"
guard_expect_in_file "$TAG" 'free_order_id=even-odd-release-then-reacquire-v0' "$CARD" "card must define free order"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PORT-RESUME-SEAM-SELECTION-295X-001' "$PREV_CARD" "previous card must select port resume seam"
guard_expect_in_file "$TAG" '295x-79' "$TASKBOARD" "taskboard must expose reuse-cycle contract"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"
