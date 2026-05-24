#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-result-ledger-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-06-MIMALLOC-COMPARISON-RESULT-LEDGER-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-05-MIMALLOC-COMPARISON-METHOD-CONSOLIDATION.md"
REFRESH_CARD="docs/development/current/main/phases/phase-295x/295x-04-MIMALLOC-COMPARISON-RESULT-LEDGER-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_result_ledger_closeout_guard.sh"
REFRESH_GUARD="tools/checks/k2_wide_phase295x_comparison_result_ledger_refresh_guard.sh"

echo "[$TAG] checking phase-295x comparison result ledger closeout"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$PREV_CARD" \
  "$REFRESH_CARD" \
  "$TASKBOARD" \
  "$INDEX" \
  "$SELF_SCRIPT" \
  "$REFRESH_GUARD"

guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$REFRESH_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RESULT-LEDGER-CLOSEOUT-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SAME-WORKLOAD-295X-REFRESH-001' "$CARD" "card must select the same-workload refresh follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RESULT-LEDGER-CLOSEOUT-295X-001' "$PREV_CARD" "previous row must select this closeout"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RESULT-LEDGER-295X-REFRESH-001' "$REFRESH_CARD" "refresh card must be the ledger evidence input"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SAME-WORKLOAD-295X-REFRESH-001' "$TASKBOARD" "taskboard must expose same-workload refresh follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'performance or memory winner claim' "$CARD" "closeout must keep winner claims closed"

bash "$REFRESH_GUARD"

echo "[$TAG] ok"
