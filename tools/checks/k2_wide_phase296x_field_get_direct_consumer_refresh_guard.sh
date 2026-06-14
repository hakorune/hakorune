#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-670-FIELD-GET-DIRECT-CONSUMER-FORWARDING-REFRESH-002.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-669-PARAM-ALIAS-COPY-OWNER-REFRESH-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_field_get_direct_consumer_refresh_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
ATTRIBUTION="tools/allocator/mir_callsite_copy_attribution.py"
WEIGHT="tools/allocator/hako_mimalloc_local_ssa_dynamic_weight_probe.py"
POSITION="tools/allocator/mir_local_ssa_copy_position_probe.py"
KIND_SELECTION="tools/allocator/hako_mimalloc_local_ssa_copy_kind_policy_selection.py"
ORIGIN="tools/allocator/hako_mimalloc_expression_materialization_copy_origin_probe.py"
CHAIN_SELECTION="tools/allocator/hako_mimalloc_field_get_expression_copy_chain_policy_selection.py"
REFRESH="tools/allocator/hako_mimalloc_field_get_direct_consumer_refresh_probe.py"

[[ -f "$CARD" ]] || { echo "[field-get-refresh] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[field-get-refresh] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$REFRESH" ]] || { echo "[field-get-refresh] missing refresh probe: $REFRESH" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[field-get-refresh] row670 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[field-get-refresh] row669 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[field-get-refresh] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_field_get_refresh.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
weight_report="$tmp_dir/weight.out"
position_report="$tmp_dir/position.out"
kind_report="$tmp_dir/kind.out"
origin_report="$tmp_dir/origin.out"
chain_report="$tmp_dir/chain.out"
refresh_report="$tmp_dir/refresh.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$ATTRIBUTION" --mir-json "$mir_json" --out "$attr_report"
python3 "$WEIGHT" --attribution "$attr_report" --method-invocation-count 524288 --out "$weight_report"
python3 "$POSITION" --mir-json "$mir_json" --out "$position_report"
python3 "$KIND_SELECTION" --dynamic-weight "$weight_report" --position "$position_report" --out "$kind_report"
python3 "$ORIGIN" --mir-json "$mir_json" --selection "$kind_report" --out "$origin_report"
python3 "$CHAIN_SELECTION" --origin "$origin_report" --out "$chain_report"
python3 "$REFRESH" --mir-json "$mir_json" --chain-policy "$chain_report" --out "$refresh_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$refresh_report"; then
    echo "[field-get-refresh] missing report line: $expected" >&2
    cat "$refresh_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-field-get-direct-consumer-refresh-v2"
require_line "input_contract=hako-mimalloc-field-get-expression-copy-chain-policy-selection-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "field_get_expression_copy_count=7"
require_line "forwarding_candidate_copy_count=4"
require_line "same_block_candidate_count=1"
require_line "cross_block_candidate_count=3"
require_line "covered_by_existing_rule_count=0"
require_line "dominant_candidate_sink=field_get"
require_line "dominant_candidate_field=object_lifecycle_queue"
require_line "selected_owner=cross_block_field_get_alias_copy_chain"
require_line "selected_owner_confidence=medium"
require_line "next_task=cross_block_field_get_alias_forwarding_design"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[field-get-refresh] ok"
