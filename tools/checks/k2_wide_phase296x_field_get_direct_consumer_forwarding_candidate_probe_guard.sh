#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-181-FIELD-GET-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-180-FIELD-GET-EXPRESSION-COPY-CHAIN-POLICY-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_field_get_direct_consumer_forwarding_candidate_probe_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
ATTRIBUTION="tools/allocator/mir_callsite_copy_attribution.py"
WEIGHT="tools/allocator/hako_mimalloc_local_ssa_dynamic_weight_probe.py"
POSITION="tools/allocator/mir_local_ssa_copy_position_probe.py"
KIND_SELECTION="tools/allocator/hako_mimalloc_local_ssa_copy_kind_policy_selection.py"
ORIGIN="tools/allocator/hako_mimalloc_expression_materialization_copy_origin_probe.py"
CHAIN_SELECTION="tools/allocator/hako_mimalloc_field_get_expression_copy_chain_policy_selection.py"
CANDIDATE="tools/allocator/hako_mimalloc_field_get_direct_consumer_forwarding_candidate_probe.py"

[[ -f "$CARD" ]] || { echo "[row181-field-get-forwarding] missing card: $CARD" >&2; exit 1; }
[[ -f "$CANDIDATE" ]] || { echo "[row181-field-get-forwarding] missing candidate probe: $CANDIDATE" >&2; exit 1; }

grep -q '^Status: Current$' "$CARD" || { echo "[row181-field-get-forwarding] row181 card must be Current" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[row181-field-get-forwarding] row180 card must be Landed" >&2; exit 1; }
grep -q 'latest_card = "296x-181-FIELD-GET-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE"' "$STATE" || { echo "[row181-field-get-forwarding] CURRENT_STATE latest_card must point to row181" >&2; exit 1; }
grep -q 'current_blocker_token = "FIELD-GET-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-296X-001"' "$STATE" || { echo "[row181-field-get-forwarding] CURRENT_STATE blocker must point to row181" >&2; exit 1; }
grep -q '| 180 | `FIELD-GET-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-296X-001` | Landed |' "$TASKBOARD" || { echo "[row181-field-get-forwarding] taskboard row180 must be Landed" >&2; exit 1; }
grep -q '| 181 | `FIELD-GET-DIRECT-CONSUMER-FORWARDING-CANDIDATE-PROBE-296X-001` | Current |' "$TASKBOARD" || { echo "[row181-field-get-forwarding] taskboard row181 must be Current" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[row181-field-get-forwarding] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_row181_field_get_forwarding.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
weight_report="$tmp_dir/weight.out"
position_report="$tmp_dir/position.out"
kind_report="$tmp_dir/kind.out"
origin_report="$tmp_dir/origin.out"
chain_report="$tmp_dir/chain.out"
candidate_report="$tmp_dir/candidate.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$ATTRIBUTION" --mir-json "$mir_json" --out "$attr_report"
python3 "$WEIGHT" --attribution "$attr_report" --out "$weight_report"
python3 "$POSITION" --mir-json "$mir_json" --out "$position_report"
python3 "$KIND_SELECTION" --dynamic-weight "$weight_report" --position "$position_report" --out "$kind_report"
python3 "$ORIGIN" --mir-json "$mir_json" --selection "$kind_report" --out "$origin_report"
python3 "$CHAIN_SELECTION" --origin "$origin_report" --out "$chain_report"
python3 "$CANDIDATE" --mir-json "$mir_json" --chain-policy "$chain_report" --out "$candidate_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$candidate_report"; then
    echo "[row181-field-get-forwarding] missing report line: $expected" >&2
    cat "$candidate_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-field-get-direct-consumer-forwarding-candidate-probe-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "field_get_expression_copy_count=23"
require_line "consumer_reachable_copy_count=19"
require_line "forwarding_candidate_copy_count=11"
require_line "max_forwarding_chain_len=2"
require_line "dominant_candidate_sink=compare_eq"
require_line "selected_optimization_owner=mir_builder_expression_materialization_forwarding"
require_line "next_diagnostic=field_get_direct_consumer_forwarding_keeper_design"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "replacement_active=0"
require_line "hook_installed=0"
require_line "global_allocator=0"
require_line "summary=ok"

echo "[row181-field-get-forwarding] ok"
