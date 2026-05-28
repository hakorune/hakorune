#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-178-LOCAL-SSA-COPY-KIND-POLICY-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-177-LOCAL-SSA-DYNAMIC-WEIGHT-PROBE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_ssa_copy_kind_policy_selection_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
ATTRIBUTION="tools/allocator/mir_callsite_copy_attribution.py"
WEIGHT="tools/allocator/hako_mimalloc_local_ssa_dynamic_weight_probe.py"
POSITION="tools/allocator/mir_local_ssa_copy_position_probe.py"
SELECTION="tools/allocator/hako_mimalloc_local_ssa_copy_kind_policy_selection.py"

[[ -f "$CARD" ]] || { echo "[row178-copy-kind] missing card: $CARD" >&2; exit 1; }
[[ -f "$SELECTION" ]] || { echo "[row178-copy-kind] missing selection: $SELECTION" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || { echo "[row178-copy-kind] row178 card must be Current" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[row178-copy-kind] row177 card must be Landed" >&2; exit 1; }
grep -q 'latest_card = "296x-178-LOCAL-SSA-COPY-KIND-POLICY-SELECTION"' "$STATE" || { echo "[row178-copy-kind] CURRENT_STATE latest_card must point to row178" >&2; exit 1; }
grep -q 'current_blocker_token = "LOCAL-SSA-COPY-KIND-POLICY-SELECTION-296X-001"' "$STATE" || { echo "[row178-copy-kind] CURRENT_STATE blocker must point to row178" >&2; exit 1; }
grep -q '| 177 | `LOCAL-SSA-DYNAMIC-WEIGHT-PROBE-296X-001` | Landed |' "$TASKBOARD" || { echo "[row178-copy-kind] taskboard row177 must be Landed" >&2; exit 1; }
grep -q '| 178 | `LOCAL-SSA-COPY-KIND-POLICY-SELECTION-296X-001` | Current |' "$TASKBOARD" || { echo "[row178-copy-kind] taskboard row178 must be Current" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[row178-copy-kind] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_row178_copy_kind.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
weight_report="$tmp_dir/weight.out"
position_report="$tmp_dir/position.out"
selection_report="$tmp_dir/selection.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$ATTRIBUTION" --mir-json "$mir_json" --out "$attr_report"
python3 "$WEIGHT" --attribution "$attr_report" --out "$weight_report"
python3 "$POSITION" --mir-json "$mir_json" --out "$position_report"
python3 "$SELECTION" --dynamic-weight "$weight_report" --position "$position_report" --out "$selection_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$selection_report"; then
    echo "[row178-copy-kind] missing report line: $expected" >&2
    cat "$selection_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-local-ssa-copy-kind-policy-selection-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "dominant_dynamic_owner=local_ssa_copy_materialization"
require_line "dominant_local_like_position=expression_materialization"
require_line "selected_copy_kind_policy=expression_materialization_copy_policy"
require_line "rejected_policy=local_ssa_same_block_field_get_reuse"
require_line "rejected_reason=recent_nonkeeper_regressed_exact_exe_body"
require_line "next_diagnostic=expression_materialization_copy_origin_probe"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row178-copy-kind] ok"
