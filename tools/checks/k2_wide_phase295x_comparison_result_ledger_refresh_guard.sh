#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-comparison-result-ledger-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-04-MIMALLOC-COMPARISON-RESULT-LEDGER-REFRESH.md"
SELECTION_CARD="docs/development/current/main/phases/phase-295x/295x-03-MIMALLOC-COMPARISON-POST-C-RUNNER-EVIDENCE-ROW-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_comparison_result_ledger_refresh_guard.sh"
C_REFRESH_GUARD="tools/checks/k2_wide_phase295x_c_mimalloc_runner_evidence_refresh_guard.sh"
LEDGER_GUARD="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_guard.sh"
DIAGNOSTICS_GUARD="tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_diagnostics_guard.sh"

echo "[$TAG] checking phase-295x comparison result ledger refresh"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$SELECTION_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$C_REFRESH_GUARD" \
  "$LEDGER_GUARD" \
  "$DIAGNOSTICS_GUARD"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$C_REFRESH_GUARD" "$LEDGER_GUARD" "$DIAGNOSTICS_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RESULT-LEDGER-295X-REFRESH-001' "$CARD" "card must identify the blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-POST-RESULT-LEDGER-ROW-SELECTION-001' "$CARD" "card must select the follow-on row"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RESULT-LEDGER-295X-REFRESH-001' "$SELECTION_CARD" "selection card must select this blocker"
guard_expect_in_file "$TAG" '295x-04' "$TASKBOARD" "taskboard must record the landed ledger refresh row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'make performance or memory winner claims' "$CARD" "ledger refresh must keep winner claims closed"

bash "$C_REFRESH_GUARD"
bash "$LEDGER_GUARD" --level L2
bash "$DIAGNOSTICS_GUARD" --level L2

echo "[$TAG] ok"
