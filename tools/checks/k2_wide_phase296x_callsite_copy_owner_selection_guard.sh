#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
ATTR_TOOL="tools/allocator/mir_callsite_copy_attribution.py"
DIFF_TOOL="tools/allocator/mir_callsite_copy_attribution_diff.py"
SELECT_TOOL="tools/allocator/mir_callsite_copy_owner_selection.py"
CARD="docs/development/current/main/phases/phase-296x/296x-158-CALLSITE-COPY-OWNER-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-157-CALLSITE-COPY-ATTRIBUTION-DIFF-HARNESS.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_callsite_copy_owner_selection_guard.sh"

[[ -f "$APP" ]] || { echo "[row158-owner-selection] missing app: $APP" >&2; exit 1; }
[[ -f "$ATTR_TOOL" ]] || { echo "[row158-owner-selection] missing attribution tool: $ATTR_TOOL" >&2; exit 1; }
[[ -f "$DIFF_TOOL" ]] || { echo "[row158-owner-selection] missing diff tool: $DIFF_TOOL" >&2; exit 1; }
[[ -f "$SELECT_TOOL" ]] || { echo "[row158-owner-selection] missing selection tool: $SELECT_TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row158-owner-selection] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row158-owner-selection] row158 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row158-owner-selection] row157 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-158-CALLSITE-COPY-OWNER-SELECTION"' "$STATE" || {
  echo "[row158-owner-selection] CURRENT_STATE latest_card must point to row158" >&2
  exit 1
}
grep -q 'current_blocker_token = "CALLSITE-COPY-OWNER-SELECTION-296X-001"' "$STATE" || {
  echo "[row158-owner-selection] CURRENT_STATE blocker must point to row158" >&2
  exit 1
}
grep -q '| 157 | `CALLSITE-COPY-ATTRIBUTION-DIFF-HARNESS-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row158-owner-selection] taskboard row157 must be Landed" >&2
  exit 1
}
grep -q '| 158 | `CALLSITE-COPY-OWNER-SELECTION-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row158-owner-selection] taskboard row158 must be Current" >&2
  exit 1
}
grep -q "$SELECT_TOOL" "$INDEX" || {
  echo "[row158-owner-selection] check index missing selection tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row158-owner-selection] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row158_owner_selection.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
diff_report="$tmp_dir/diff.out"
selection_report="$tmp_dir/selection.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$ATTR_TOOL" --mir-json "$mir_json" --out "$attr_report"
python3 "$DIFF_TOOL" --before "$attr_report" --after "$attr_report" --candidate-id self_smoke --out "$diff_report"
python3 "$SELECT_TOOL" --attribution "$attr_report" --diff "$diff_report" --out "$selection_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$selection_report"; then
    echo "[row158-owner-selection] missing report line: $expected" >&2
    echo "[row158-owner-selection] report follows:" >&2
    cat "$selection_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-callsite-copy-owner-selection-v0"
require_line "input_contract=hako-mimalloc-callsite-copy-attribution-v0"
require_line "diff_contract=hako-mimalloc-callsite-copy-attribution-diff-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "dominant_callee_family=page_hotpath_helpers"
require_line "dominant_copy_owner=local_ssa_copy_materialization"
require_line "diff_structural_effect=no_effect"
require_line "selected_owner=local_ssa_copy_materialization"
require_line "owner_confidence=medium"
require_line "owner_reason=dominant_baseline_copy_owner"
require_line "next_diagnostic=local_ssa_block_position_probe"
require_line "optimization_open=0"
require_line "local_ssa_copy_materialization_copy_count=48"
require_line "receiver_materialization_copy_count=27"
require_line "top_callsite_callee=acquire_usize"
require_line "top_callsite_attributed_copy_count=9"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row158-owner-selection] ok"
