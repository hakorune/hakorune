#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
CARD="docs/development/current/main/phases/phase-296x/296x-183-RECEIVER-MATERIALIZATION-ATTRIBUTION-PROBE.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-182-FIELD-GET-DIRECT-CONSUMER-FORWARDING-KEEPER-DESIGN.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_receiver_materialization_attribution_probe_guard.sh"
PROBE="tools/allocator/hako_mimalloc_receiver_materialization_attribution_probe.py"

[[ -f "$CARD" ]] || { echo "[row183-receiver-attribution] missing card: $CARD" >&2; exit 1; }
[[ -f "$PROBE" ]] || { echo "[row183-receiver-attribution] missing probe: $PROBE" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || { echo "[row183-receiver-attribution] row183 card must be Current" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[row183-receiver-attribution] row182 card must be Landed" >&2; exit 1; }
grep -q 'latest_card = "296x-183-RECEIVER-MATERIALIZATION-ATTRIBUTION-PROBE"' "$STATE" || { echo "[row183-receiver-attribution] CURRENT_STATE latest_card must point to row183" >&2; exit 1; }
grep -q 'current_blocker_token = "RECEIVER-MATERIALIZATION-ATTRIBUTION-PROBE-296X-001"' "$STATE" || { echo "[row183-receiver-attribution] CURRENT_STATE blocker must point to row183" >&2; exit 1; }
grep -q '| 182 | `FIELD-GET-DIRECT-CONSUMER-FORWARDING-KEEPER-DESIGN-296X-001` | Landed |' "$TASKBOARD" || { echo "[row183-receiver-attribution] taskboard row182 must be Landed" >&2; exit 1; }
grep -q '| 183 | `RECEIVER-MATERIALIZATION-ATTRIBUTION-PROBE-296X-001` | Current |' "$TASKBOARD" || { echo "[row183-receiver-attribution] taskboard row183 must be Current" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[row183-receiver-attribution] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_row183_receiver_attribution.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"

NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$PROBE" --mir-json "$mir_json" --out "$report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$report"; then
    echo "[row183-receiver-attribution] missing report line: $expected" >&2
    cat "$report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-receiver-materialization-attribution-probe-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "receiver_attributed_copy_count=27"
require_line "unique_receiver_copy_count=24"
require_line "duplicate_receiver_attribution_count=3"
require_line "page_hotpath_receiver_copy_count=13"
require_line "other_receiver_copy_count=12"
require_line "facade_result_receiver_copy_count=2"
require_line "dominant_receiver_family=page_hotpath_helpers"
require_line "dominant_receiver_chain_len=2"
require_line "selected_receiver_policy=receiver_pin_chain_policy_selection"
require_line "next_diagnostic=receiver_pin_chain_policy_selection"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row183-receiver-attribution] ok"
