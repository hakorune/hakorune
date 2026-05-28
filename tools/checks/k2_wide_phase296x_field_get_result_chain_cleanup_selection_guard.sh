#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
EXPR_TOOL="tools/allocator/mir_expression_materialization_owner_selection.py"
SELECT_TOOL="tools/allocator/mir_field_get_result_chain_cleanup_selection.py"
CARD="docs/development/current/main/phases/phase-296x/296x-161-FIELD-GET-RESULT-CHAIN-CLEANUP-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-160-EXPRESSION-MATERIALIZATION-OWNER-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_field_get_result_chain_cleanup_selection_guard.sh"

[[ -f "$APP" ]] || { echo "[row161-field-get-selection] missing app: $APP" >&2; exit 1; }
[[ -f "$EXPR_TOOL" ]] || { echo "[row161-field-get-selection] missing expression tool: $EXPR_TOOL" >&2; exit 1; }
[[ -f "$SELECT_TOOL" ]] || { echo "[row161-field-get-selection] missing selection tool: $SELECT_TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row161-field-get-selection] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row161-field-get-selection] row161 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row161-field-get-selection] row160 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-161-FIELD-GET-RESULT-CHAIN-CLEANUP-SELECTION"' "$STATE" || {
  echo "[row161-field-get-selection] CURRENT_STATE latest_card must point to row161" >&2
  exit 1
}
grep -q 'current_blocker_token = "FIELD-GET-RESULT-CHAIN-CLEANUP-SELECTION-296X-001"' "$STATE" || {
  echo "[row161-field-get-selection] CURRENT_STATE blocker must point to row161" >&2
  exit 1
}
grep -q '| 160 | `EXPRESSION-MATERIALIZATION-OWNER-SELECTION-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row161-field-get-selection] taskboard row160 must be Landed" >&2
  exit 1
}
grep -q '| 161 | `FIELD-GET-RESULT-CHAIN-CLEANUP-SELECTION-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row161-field-get-selection] taskboard row161 must be Current" >&2
  exit 1
}
grep -q "$SELECT_TOOL" "$INDEX" || {
  echo "[row161-field-get-selection] check index missing selection tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row161-field-get-selection] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row161_field_get_selection.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/app.mir.json"
expr_report="$tmp_dir/expression.out"
selection_report="$tmp_dir/selection.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$EXPR_TOOL" --mir-json "$mir_json" --out "$expr_report"
python3 "$SELECT_TOOL" --expression-report "$expr_report" --out "$selection_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$selection_report"; then
    echo "[row161-field-get-selection] missing report line: $expected" >&2
    echo "[row161-field-get-selection] report follows:" >&2
    cat "$selection_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-field-get-result-chain-cleanup-selection-v0"
require_line "input_contract=hako-mimalloc-expression-materialization-owner-selection-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "expression_materialization_copy_count=29"
require_line "field_get_result_chain_copy_count=28"
require_line "selected_expression_owner=field_get_result_chain"
require_line "selected_mir_owner=mir_builder_field_access_pin_to_slot_cleanup"
require_line "selected_file=src/mir/builder/fields.rs"
require_line "selected_function=MirBuilder::build_field_access"
require_line "rejected_owner=PlanLowerer::emit_effect(CoreEffectPlan::FieldGet)"
require_line "rejected_reason=core_effect_field_get_already_emits_selected_dst_directly"
require_line "owner_confidence=medium"
require_line "next_row=field_get_result_chain_cleanup_implementation"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row161-field-get-selection] ok"
