#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-177-LOCAL-SSA-DYNAMIC-WEIGHT-PROBE.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-176-OBJECT-LIFECYCLE-MIR-BODY-OWNER-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_ssa_dynamic_weight_probe_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
ATTRIBUTION="tools/allocator/mir_callsite_copy_attribution.py"
PROBE="tools/allocator/hako_mimalloc_local_ssa_dynamic_weight_probe.py"

[[ -f "$CARD" ]] || { echo "[row177-local-ssa-weight] missing card: $CARD" >&2; exit 1; }
[[ -f "$PROBE" ]] || { echo "[row177-local-ssa-weight] missing probe: $PROBE" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || { echo "[row177-local-ssa-weight] row177 card must be Current" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[row177-local-ssa-weight] row176 card must be Landed" >&2; exit 1; }
grep -q 'latest_card = "296x-177-LOCAL-SSA-DYNAMIC-WEIGHT-PROBE"' "$STATE" || { echo "[row177-local-ssa-weight] CURRENT_STATE latest_card must point to row177" >&2; exit 1; }
grep -q 'current_blocker_token = "LOCAL-SSA-DYNAMIC-WEIGHT-PROBE-296X-001"' "$STATE" || { echo "[row177-local-ssa-weight] CURRENT_STATE blocker must point to row177" >&2; exit 1; }
grep -q '| 176 | `OBJECT-LIFECYCLE-MIR-BODY-OWNER-SELECTION-296X-001` | Landed |' "$TASKBOARD" || { echo "[row177-local-ssa-weight] taskboard row176 must be Landed" >&2; exit 1; }
grep -q '| 177 | `LOCAL-SSA-DYNAMIC-WEIGHT-PROBE-296X-001` | Current |' "$TASKBOARD" || { echo "[row177-local-ssa-weight] taskboard row177 must be Current" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[row177-local-ssa-weight] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_row177_local_ssa_weight.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
probe_report="$tmp_dir/probe.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$ATTRIBUTION" --mir-json "$mir_json" --out "$attr_report"
python3 "$PROBE" --attribution "$attr_report" --out "$probe_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$probe_report"; then
    echo "[row177-local-ssa-weight] missing report line: $expected" >&2
    cat "$probe_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-local-ssa-dynamic-weight-probe-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "method_invocation_source=object-lifecycle-allocation-count"
require_line "method_invocation_count=524288"
require_line "dominant_dynamic_owner=local_ssa_copy_materialization"
require_line "selected_owner=local_ssa_copy_materialization"
require_line "rejected_recent_nonkeeper=local_ssa_same_block_field_get_reuse"
require_line "next_diagnostic=local_ssa_copy_kind_policy_selection"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

value="$(awk -F= '$1 == "local_ssa_copy_materialization_dynamic_ops" { print $2 }' "$probe_report")"
case "$value" in
  ''|*[!0-9]*|0)
    echo "[row177-local-ssa-weight] local_ssa dynamic ops must be positive" >&2
    cat "$probe_report" >&2
    exit 1
    ;;
esac

echo "[row177-local-ssa-weight] ok"
