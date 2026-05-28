#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-180-FIELD-GET-EXPRESSION-COPY-CHAIN-POLICY-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-179-EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_field_get_expression_copy_chain_policy_selection_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
ATTRIBUTION="tools/allocator/mir_callsite_copy_attribution.py"
WEIGHT="tools/allocator/hako_mimalloc_local_ssa_dynamic_weight_probe.py"
POSITION="tools/allocator/mir_local_ssa_copy_position_probe.py"
KIND_SELECTION="tools/allocator/hako_mimalloc_local_ssa_copy_kind_policy_selection.py"
ORIGIN="tools/allocator/hako_mimalloc_expression_materialization_copy_origin_probe.py"
CHAIN_SELECTION="tools/allocator/hako_mimalloc_field_get_expression_copy_chain_policy_selection.py"

[[ -f "$CARD" ]] || { echo "[row180-field-get-chain] missing card: $CARD" >&2; exit 1; }
[[ -f "$CHAIN_SELECTION" ]] || { echo "[row180-field-get-chain] missing chain selection: $CHAIN_SELECTION" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || { echo "[row180-field-get-chain] row180 card must be Current" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[row180-field-get-chain] row179 card must be Landed" >&2; exit 1; }
grep -q 'latest_card = "296x-180-FIELD-GET-EXPRESSION-COPY-CHAIN-POLICY-SELECTION"' "$STATE" || { echo "[row180-field-get-chain] CURRENT_STATE latest_card must point to row180" >&2; exit 1; }
grep -q 'current_blocker_token = "FIELD-GET-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-296X-001"' "$STATE" || { echo "[row180-field-get-chain] CURRENT_STATE blocker must point to row180" >&2; exit 1; }
grep -q '| 179 | `EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE-296X-001` | Landed |' "$TASKBOARD" || { echo "[row180-field-get-chain] taskboard row179 must be Landed" >&2; exit 1; }
grep -q '| 180 | `FIELD-GET-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-296X-001` | Current |' "$TASKBOARD" || { echo "[row180-field-get-chain] taskboard row180 must be Current" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[row180-field-get-chain] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_row180_field_get_chain.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
weight_report="$tmp_dir/weight.out"
position_report="$tmp_dir/position.out"
kind_report="$tmp_dir/kind.out"
origin_report="$tmp_dir/origin.out"
chain_report="$tmp_dir/chain.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$ATTRIBUTION" --mir-json "$mir_json" --out "$attr_report"
python3 "$WEIGHT" --attribution "$attr_report" --out "$weight_report"
python3 "$POSITION" --mir-json "$mir_json" --out "$position_report"
python3 "$KIND_SELECTION" --dynamic-weight "$weight_report" --position "$position_report" --out "$kind_report"
python3 "$ORIGIN" --mir-json "$mir_json" --selection "$kind_report" --out "$origin_report"
python3 "$CHAIN_SELECTION" --origin "$origin_report" --out "$chain_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$chain_report"; then
    echo "[row180-field-get-chain] missing report line: $expected" >&2
    cat "$chain_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-field-get-expression-copy-chain-policy-selection-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "field_get_origin_copy_count=23"
require_line "expression_materialization_copy_count=24"
require_line "field_get_origin_ratio_bp=9583"
require_line "compare_sink_copy_count=12"
require_line "selected_chain_policy=field_get_direct_consumer_value_forwarding"
require_line "selected_chain_policy_confidence=high"
require_line "next_diagnostic=field_get_direct_consumer_forwarding_candidate_probe"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row180-field-get-chain] ok"
