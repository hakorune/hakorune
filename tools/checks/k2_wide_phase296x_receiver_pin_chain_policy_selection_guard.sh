#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
CARD="docs/development/current/main/phases/phase-296x/296x-184-RECEIVER-PIN-CHAIN-POLICY-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-183-RECEIVER-MATERIALIZATION-ATTRIBUTION-PROBE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_receiver_pin_chain_policy_selection_guard.sh"
ATTRIBUTION="tools/allocator/hako_mimalloc_receiver_materialization_attribution_probe.py"
SELECTION="tools/allocator/hako_mimalloc_receiver_pin_chain_policy_selection.py"

[[ -f "$CARD" ]] || { echo "[row184-receiver-policy] missing card: $CARD" >&2; exit 1; }
[[ -f "$SELECTION" ]] || { echo "[row184-receiver-policy] missing selection: $SELECTION" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || { echo "[row184-receiver-policy] row184 card must be Current" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[row184-receiver-policy] row183 card must be Landed" >&2; exit 1; }
grep -q 'latest_card = "296x-184-RECEIVER-PIN-CHAIN-POLICY-SELECTION"' "$STATE" || { echo "[row184-receiver-policy] CURRENT_STATE latest_card must point to row184" >&2; exit 1; }
grep -q 'current_blocker_token = "RECEIVER-PIN-CHAIN-POLICY-SELECTION-296X-001"' "$STATE" || { echo "[row184-receiver-policy] CURRENT_STATE blocker must point to row184" >&2; exit 1; }
grep -q '| 183 | `RECEIVER-MATERIALIZATION-ATTRIBUTION-PROBE-296X-001` | Landed |' "$TASKBOARD" || { echo "[row184-receiver-policy] taskboard row183 must be Landed" >&2; exit 1; }
grep -q '| 184 | `RECEIVER-PIN-CHAIN-POLICY-SELECTION-296X-001` | Current |' "$TASKBOARD" || { echo "[row184-receiver-policy] taskboard row184 must be Current" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[row184-receiver-policy] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_row184_receiver_policy.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
selection_report="$tmp_dir/selection.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$ATTRIBUTION" --mir-json "$mir_json" --out "$attr_report"
python3 "$SELECTION" --receiver-attribution "$attr_report" --out "$selection_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$selection_report"; then
    echo "[row184-receiver-policy] missing report line: $expected" >&2
    cat "$selection_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-receiver-pin-chain-policy-selection-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "receiver_attributed_copy_count=27"
require_line "unique_receiver_copy_count=24"
require_line "duplicate_receiver_attribution_count=3"
require_line "page_hotpath_receiver_copy_count=13"
require_line "selected_receiver_policy=receiver_pin_chain_narrowing"
require_line "rejected_receiver_policy=same_receiver_callsite_cache"
require_line "rejected_reason=duplicate_receiver_attribution_too_small"
require_line "next_diagnostic=receiver_pin_chain_keeper_design"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row184-receiver-policy] ok"
