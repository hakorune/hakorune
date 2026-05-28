#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
ATTR_TOOL="tools/allocator/mir_callsite_copy_attribution.py"
LOCAL_TOOL="tools/allocator/mir_local_ssa_copy_position_probe.py"
EXPR_TOOL="tools/allocator/mir_expression_materialization_owner_selection.py"
CARD="docs/development/current/main/phases/phase-296x/296x-162-FIELD-GET-RESULT-CHAIN-CLEANUP-IMPLEMENTATION.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-161-FIELD-GET-RESULT-CHAIN-CLEANUP-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
FIELDS="src/mir/builder/fields.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_field_get_result_chain_cleanup_implementation_guard.sh"

[[ -f "$APP" ]] || { echo "[row162-field-get-impl] missing app: $APP" >&2; exit 1; }
[[ -f "$ATTR_TOOL" ]] || { echo "[row162-field-get-impl] missing attribution tool: $ATTR_TOOL" >&2; exit 1; }
[[ -f "$LOCAL_TOOL" ]] || { echo "[row162-field-get-impl] missing local tool: $LOCAL_TOOL" >&2; exit 1; }
[[ -f "$EXPR_TOOL" ]] || { echo "[row162-field-get-impl] missing expression tool: $EXPR_TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row162-field-get-impl] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row162-field-get-impl] row162 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row162-field-get-impl] row161 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-162-FIELD-GET-RESULT-CHAIN-CLEANUP-IMPLEMENTATION"' "$STATE" || {
  echo "[row162-field-get-impl] CURRENT_STATE latest_card must point to row162" >&2
  exit 1
}
grep -q 'current_blocker_token = "FIELD-GET-RESULT-CHAIN-CLEANUP-IMPLEMENTATION-296X-001"' "$STATE" || {
  echo "[row162-field-get-impl] CURRENT_STATE blocker must point to row162" >&2
  exit 1
}
grep -q '| 161 | `FIELD-GET-RESULT-CHAIN-CLEANUP-SELECTION-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row162-field-get-impl] taskboard row161 must be Landed" >&2
  exit 1
}
grep -q '| 162 | `FIELD-GET-RESULT-CHAIN-CLEANUP-IMPLEMENTATION-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row162-field-get-impl] taskboard row162 must be Current" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row162-field-get-impl] check index missing guard entry" >&2
  exit 1
}
grep -q 'Ok(field_val)' "$FIELDS" || {
  echo "[row162-field-get-impl] fields.rs must return field_get result directly" >&2
  exit 1
}
if grep -q 'pin_to_slot(field_val, "@field")' "$FIELDS"; then
  echo "[row162-field-get-impl] fields.rs must not pin field_get result unconditionally" >&2
  exit 1
fi

tmp_dir="$(mktemp -d /tmp/hakorune_row162_field_get_impl.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
local_report="$tmp_dir/local.out"
expr_report="$tmp_dir/expression.out"
proof_log="$tmp_dir/proof.log"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$ATTR_TOOL" --mir-json "$mir_json" --out "$attr_report"
python3 "$LOCAL_TOOL" --mir-json "$mir_json" --out "$local_report"
python3 "$EXPR_TOOL" --mir-json "$mir_json" --out "$expr_report"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row162-field-get-impl] missing report line: $expected" >&2
    echo "[row162-field-get-impl] report follows ($file):" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line "$attr_report" "instruction_count=180"
require_line "$attr_report" "copy_count=88"
require_line "$attr_report" "local_ssa_copy_count=38"
require_line "$local_report" "expression_materialization_copy_count=24"
require_line "$expr_report" "expression_materialization_copy_count=24"
require_line "$expr_report" "selected_owner=field_get_result_chain"
require_line "$expr_report" "field_get_result_chain_copy_count=23"
require_line "$expr_report" "summary=ok"

timeout 180s env NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune "$APP" >"$proof_log"

require_line "$proof_log" "allocation_count=524288"
require_line "$proof_log" "free_count=524288"
require_line "$proof_log" "select_page_single_fast_path_count=524288"
require_line "$proof_log" "select_page_single_fallback_count=0"
require_line "$proof_log" "release_known_page_fast_path_count=524288"
require_line "$proof_log" "release_known_page_fallback_count=0"
require_line "$proof_log" "summary=ok"

echo "[row162-field-get-impl] ok"
