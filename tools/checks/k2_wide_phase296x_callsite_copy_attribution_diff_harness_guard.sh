#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
ATTR_TOOL="tools/allocator/mir_callsite_copy_attribution.py"
DIFF_TOOL="tools/allocator/mir_callsite_copy_attribution_diff.py"
CARD="docs/development/current/main/phases/phase-296x/296x-157-CALLSITE-COPY-ATTRIBUTION-DIFF-HARNESS.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-156-OBJECT-LIFECYCLE-SMALL-HOTPATH-CALLSITE-COPY-ATTRIBUTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_callsite_copy_attribution_diff_harness_guard.sh"

[[ -f "$APP" ]] || { echo "[row157-attribution-diff] missing app: $APP" >&2; exit 1; }
[[ -f "$ATTR_TOOL" ]] || { echo "[row157-attribution-diff] missing attribution tool: $ATTR_TOOL" >&2; exit 1; }
[[ -f "$DIFF_TOOL" ]] || { echo "[row157-attribution-diff] missing diff tool: $DIFF_TOOL" >&2; exit 1; }
[[ -f "$CARD" ]] || { echo "[row157-attribution-diff] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || {
  echo "[row157-attribution-diff] row157 card must be Current" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[row157-attribution-diff] row156 card must be Landed" >&2
  exit 1
}
grep -q 'latest_card = "296x-157-CALLSITE-COPY-ATTRIBUTION-DIFF-HARNESS"' "$STATE" || {
  echo "[row157-attribution-diff] CURRENT_STATE latest_card must point to row157" >&2
  exit 1
}
grep -q 'current_blocker_token = "CALLSITE-COPY-ATTRIBUTION-DIFF-HARNESS-296X-001"' "$STATE" || {
  echo "[row157-attribution-diff] CURRENT_STATE blocker must point to row157" >&2
  exit 1
}
grep -q '| 156 | `OBJECT-LIFECYCLE-SMALL-HOTPATH-CALLSITE-COPY-ATTRIBUTION-296X-001` | Landed |' "$TASKBOARD" || {
  echo "[row157-attribution-diff] taskboard row156 must be Landed" >&2
  exit 1
}
grep -q '| 157 | `CALLSITE-COPY-ATTRIBUTION-DIFF-HARNESS-296X-001` | Current |' "$TASKBOARD" || {
  echo "[row157-attribution-diff] taskboard row157 must be Current" >&2
  exit 1
}
grep -q "$DIFF_TOOL" "$INDEX" || {
  echo "[row157-attribution-diff] check index missing diff tool entry" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[row157-attribution-diff] check index missing guard entry" >&2
  exit 1
}

tmp_dir="$(mktemp -d /tmp/hakorune_row157_attr_diff.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
diff_report="$tmp_dir/diff.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$ATTR_TOOL" --mir-json "$mir_json" --out "$attr_report"
python3 "$DIFF_TOOL" --before "$attr_report" --after "$attr_report" --candidate-id self_smoke --out "$diff_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$diff_report"; then
    echo "[row157-attribution-diff] missing report line: $expected" >&2
    echo "[row157-attribution-diff] report follows:" >&2
    cat "$diff_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-callsite-copy-attribution-diff-v0"
require_line "input_contract=hako-mimalloc-callsite-copy-attribution-v0"
require_line "candidate_id=self_smoke"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "before_dominant_callee_family=page_hotpath_helpers"
require_line "after_dominant_callee_family=page_hotpath_helpers"
require_line "before_dominant_copy_owner=local_ssa_copy_materialization"
require_line "after_dominant_copy_owner=local_ssa_copy_materialization"
require_line "selected_delta_owner=local_ssa_copy_materialization"
require_line "structural_effect=no_effect"
require_line "exact_exe_required=0"
require_line "delta_instruction_count=0"
require_line "delta_call_count=0"
require_line "delta_copy_count=0"
require_line "delta_local_ssa_copy_count=0"
require_line "delta_receiver_copy_count=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row157-attribution-diff] ok"
