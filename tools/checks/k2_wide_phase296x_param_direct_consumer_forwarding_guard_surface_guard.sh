#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-667-PARAM-DIRECT-CONSUMER-FORWARDING-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-666-PARAM-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-001.md"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
ATTRIBUTION="tools/allocator/mir_callsite_copy_attribution.py"
WEIGHT="tools/allocator/hako_mimalloc_local_ssa_dynamic_weight_probe.py"
POSITION="tools/allocator/mir_local_ssa_copy_position_probe.py"
KIND_SELECTION="tools/allocator/hako_mimalloc_local_ssa_copy_kind_policy_selection.py"
ORIGIN="tools/allocator/hako_mimalloc_expression_materialization_copy_origin_probe.py"
PARAM_POLICY="tools/allocator/hako_mimalloc_param_expression_copy_chain_policy_selection.py"
CANDIDATE="tools/allocator/hako_mimalloc_param_direct_consumer_forwarding_candidate_probe.py"

[[ -f "$CARD" ]] || { echo "[param-forward-guard] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[param-forward-guard] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$CANDIDATE" ]] || { echo "[param-forward-guard] missing candidate probe: $CANDIDATE" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_param_forward_guard.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
weight_report="$tmp_dir/weight.out"
position_report="$tmp_dir/position.out"
kind_report="$tmp_dir/kind.out"
origin_report="$tmp_dir/origin.out"
param_policy_report="$tmp_dir/param_policy.out"
candidate_report="$tmp_dir/candidate.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$ATTRIBUTION" --mir-json "$mir_json" --out "$attr_report"
python3 "$WEIGHT" --attribution "$attr_report" --method-invocation-count 524288 --out "$weight_report"
python3 "$POSITION" --mir-json "$mir_json" --out "$position_report"
python3 "$KIND_SELECTION" --dynamic-weight "$weight_report" --position "$position_report" --out "$kind_report"
python3 "$ORIGIN" --mir-json "$mir_json" --selection "$kind_report" --out "$origin_report"
python3 "$PARAM_POLICY" --origin "$origin_report" --out "$param_policy_report"
python3 "$CANDIDATE" --mir-json "$mir_json" --chain-policy "$param_policy_report" --out "$candidate_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$candidate_report"; then
    echo "[param-forward-guard] missing report line: $expected" >&2
    cat "$candidate_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-param-direct-consumer-forwarding-candidate-probe-v0"
require_line "input_contract=hako-mimalloc-param-expression-copy-chain-policy-selection-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "param_candidate_copy_count=7"
require_line "safe_forward_total_count=7"
require_line "safe_forward_field_get_count=2"
require_line "safe_forward_field_set_count=2"
require_line "safe_forward_compare_count=3"
require_line "unsafe_forward_count=0"
require_line "selected_optimization_owner=mir_builder_param_direct_consumer_value_forwarding"
require_line "selected_owner_confidence=medium"
require_line "next_task=param_direct_consumer_forwarding_guard_surface"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "provider_active=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[param-forward-guard] ok"
