#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="rust-lifecycle-current-state-pointer-realign-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
TASK_ORDER="$ROOT_DIR/docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
ROADMAP="$ROOT_DIR/docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md"
CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1773-CURRENT-STATE-POINTER-REALIGN-001.md"
NEXT_CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/1774-MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-CLOSEOUT-001.md"
OLD_CARD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-1650-MIRBUILDER-CONVERTER-NEXT-SLICE-DESIGN-STOP-001.md"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$STATE" "$TASK_ORDER" "$ROADMAP" "$CARD" "$NEXT_CARD" "$OLD_CARD" "$INDEX" "$0"

guard_expect_fixed_in_file "$TAG" 'latest_card = "MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-CLOSEOUT-001"' "$STATE" "CURRENT_STATE latest_card must point at VariableContext closeout"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-CLOSEOUT-001"' "$STATE" "CURRENT_STATE current blocker must point at VariableContext closeout"
guard_expect_fixed_in_file "$TAG" '1774-MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-CLOSEOUT-001.md' "$STATE" "CURRENT_STATE latest_card_path must point at 1774"

if rg -n 'current_blocker_token = "MIRBUILDER-CONVERTER-NEXT-SLICE-DESIGN-STOP-001"' "$STATE" >/dev/null; then
  guard_fail "$TAG" "stale 1650 design stop is still the active blocker"
fi

guard_expect_fixed_in_file "$TAG" 'MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-CLOSEOUT-001' "$TASK_ORDER" "task-order must select VariableContext closeout"
guard_expect_fixed_in_file "$TAG" 'old_1650_design_stop = provenance_only' "$TASK_ORDER" "task-order must keep 1650 as provenance only"
guard_expect_fixed_in_file "$TAG" 'MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-CLOSEOUT-001' "$TASK_ORDER" "task-order must route to VariableContext closeout next"

task_order_lines="$(wc -l < "$TASK_ORDER" | tr -d '[:space:]')"
if (( task_order_lines > 1000 )); then
  guard_fail "$TAG" "task-order exceeds 1000 lines: $task_order_lines"
fi

guard_expect_fixed_in_file "$TAG" 'MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-CLOSEOUT-001' "$ROADMAP" "roadmap must use route matrix closeout as candidate source"
guard_expect_fixed_in_file "$TAG" 'not from a handwritten `none` row' "$ROADMAP" "roadmap must not use handwritten none as candidate truth"
guard_expect_fixed_in_file "$TAG" 'Status: Closed' "$CARD" "pointer realign card must be closed"
guard_expect_fixed_in_file "$TAG" 'MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-CLOSEOUT-001' "$NEXT_CARD" "next closeout card must exist"
guard_expect_fixed_in_file "$TAG" 'Status: Active' "$NEXT_CARD" "next closeout card must be active"
guard_expect_fixed_in_file "$TAG" 'Status: Closed by 1773-CURRENT-STATE-POINTER-REALIGN-001' "$OLD_CARD" "1650 design stop must be closed by pointer realign"
guard_expect_fixed_in_file "$TAG" 'rust_lifecycle_current_state_pointer_realign_guard.sh' "$INDEX" "check index must list this guard"

echo "[$TAG] ok"
