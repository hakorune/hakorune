#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-c-mimalloc-runner-evidence-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-02-MIMALLOC-COMPARISON-C-RUNNER-EVIDENCE-CONTRACT-REFRESH.md"
SELECTION_CARD="docs/development/current/main/phases/phase-295x/295x-01-MIMALLOC-COMPARISON-EXECUTION-ROW-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_c_mimalloc_runner_evidence_refresh_guard.sh"
HAKO_REFRESH_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_page_heap_usize_refresh_guard.sh"
C_RUNNER_GUARD="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh"

echo "[$TAG] checking phase-295x C mimalloc runner evidence refresh"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$SELECTION_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$HAKO_REFRESH_GUARD" \
  "$C_RUNNER_GUARD"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$HAKO_REFRESH_GUARD" "$C_RUNNER_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-C-RUNNER-EVIDENCE-CONTRACT-REFRESH-001' "$CARD" "card must identify the blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-POST-C-RUNNER-EVIDENCE-ROW-SELECTION-001' "$CARD" "card must select the follow-on row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-C-RUNNER-EVIDENCE-CONTRACT-REFRESH-001' "$SELECTION_CARD" "selection card must select this blocker"
guard_expect_in_file "$TAG" '295x-02' "$TASKBOARD" "taskboard must record the landed refresh row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'allocator-comparison-c-mimalloc-explicit-runner-v0' "$CARD" "card must preserve the stable C runner contract"
guard_expect_in_file "$TAG" 'process allocator replacement' "$CARD" "card must keep replacement seams closed"

bash "$HAKO_REFRESH_GUARD"
bash "$C_RUNNER_GUARD" --level L2

echo "[$TAG] ok"
