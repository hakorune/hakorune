#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-838-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-TRIGGER-PROBE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-837-MIMALLOC-MAP-MISSING-KEY-OWNER-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_missing_empty_route_trigger_probe_guard.sh"
MAP_FUSION_PLAN="src/mir/map_lookup_fusion_plan.rs"
MAP_REPR_PLAN="src/mir/map_repr_plan.rs"
MAP_LOWERER="src/llvm_py/instructions/mir_call/collection_method_call.py"
LOWERING_SSOT="docs/development/current/main/design/lowering-plan-json-v0-ssot.md"
MAP_SSOT="docs/development/current/main/design/mapbox-proof-bearing-route-ssot.md"

[[ -f "$CARD" ]] || { echo "[map-missing-empty-route-trigger-probe] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[map-missing-empty-route-trigger-probe] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$MAP_FUSION_PLAN" ]] || { echo "[map-missing-empty-route-trigger-probe] missing map fusion plan" >&2; exit 1; }
[[ -f "$MAP_REPR_PLAN" ]] || { echo "[map-missing-empty-route-trigger-probe] missing map repr plan" >&2; exit 1; }
[[ -f "$MAP_LOWERER" ]] || { echo "[map-missing-empty-route-trigger-probe] missing map lowerer" >&2; exit 1; }
[[ -f "$LOWERING_SSOT" ]] || { echo "[map-missing-empty-route-trigger-probe] missing lowering SSOT" >&2; exit 1; }
[[ -f "$MAP_SSOT" ]] || { echo "[map-missing-empty-route-trigger-probe] missing MapBox SSOT" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[map-missing-empty-route-trigger-probe] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[map-missing-empty-route-trigger-probe] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[map-missing-empty-route-trigger-probe] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[map-missing-empty-route-trigger-probe] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-map-missing-empty-route-trigger-probe-v0" \
  "source_evidence=296x-837" \
  "row_kind=probe" \
  "implementation_started=0" \
  "perf_first_required=1" \
  "target_front=kilo_leaf_map_get_missing" \
  "mir_json_emit_route=direct_backend_mir_emit" \
  "jsonfrag_fallback_metadata_usable=0" \
  "newbox_mapbox_count=1" \
  "map_get_call_site=bb19.i3" \
  "map_get_receiver_value=3" \
  "map_get_key_value=30" \
  "map_get_key_route=i64_const" \
  "generic_method_route_count=1" \
  "generic_method_route_core_op=MapGet" \
  "generic_method_route_route_kind=runtime_data_load_any" \
  "generic_method_route_publication_policy=runtime_data_facade" \
  "generic_method_route_value_demand=runtime_i64_or_handle" \
  "generic_method_route_return_shape=mixed_runtime_i64_or_handle" \
  "generic_method_route_helper_symbol=nyash.runtime_data.get_hh" \
  "map_lookup_fusion_route_count=0" \
  "route_decision_count=0" \
  "current_front_map_get_count=1" \
  "current_front_map_has_count=0" \
  "existing_same_key_get_has_fusion_triggered=0" \
  "map_repr_plan_count=1" \
  "map_repr_plan_route_id=map_repr.generic_hash_runtime" \
  "map_repr_plan_repr_kind=generic_hash_runtime" \
  "map_repr_plan_source_helper_symbol=nyash.runtime_data.get_hh" \
  "missing_empty_map_route_exists=0" \
  "missing_empty_map_route_proof_required=1" \
  "selected_owner=missing_empty_map_route_design" \
  "selected_owner_confidence=medium" \
  "selected_next=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-DESIGN-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for proof_field in \
  "receiver_birth_is_new_mapbox=1" \
  "no_map_set_or_delete_before_get=1" \
  "receiver_not_published_before_get=1" \
  "receiver_not_escaped_before_get=1" \
  "result_shape_is_null_or_zero_missing=1" \
  "fallback_policy_is_explicit=1"; do
  grep -F -q "$proof_field" "$CARD" || {
    echo "[map-missing-empty-route-trigger-probe] missing required proof field: $proof_field" >&2
    exit 1
  }
done

for stop_line in \
  "do not fold get-only missing-key from generic_method_routes alone" \
  "do not consume map_lookup_fusion_routes without RouteDecision" \
  "do not add backend branches for literal key 0" \
  "do not add benchmark-name or helper-name branches" \
  "do not change MapBox public String-key semantics" \
  "do not replace MapBox storage in this row" \
  "do not open missing-empty-map lowering before proof fields are defined"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[map-missing-empty-route-trigger-probe] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q 'MapLookupSameKey' "$MAP_FUSION_PLAN" || {
  echo "[map-missing-empty-route-trigger-probe] same-key fusion scope missing" >&2
  exit 1
}
grep -F -q 'is_i64_map_has_route' "$MAP_FUSION_PLAN" || {
  echo "[map-missing-empty-route-trigger-probe] MapHas route predicate missing" >&2
  exit 1
}
grep -F -q 'source_helper_symbol: route.helper_symbol()' "$MAP_REPR_PLAN" || {
  echo "[map-missing-empty-route-trigger-probe] map repr source helper propagation missing" >&2
  exit 1
}
grep -F -q 'generic_hash_runtime' "$MAP_REPR_PLAN" || {
  echo "[map-missing-empty-route-trigger-probe] generic hash runtime repr missing" >&2
  exit 1
}
grep -F -q 'nyash.map.slot_load_hh' "$MAP_LOWERER" || {
  echo "[map-missing-empty-route-trigger-probe] map slot load fallback missing" >&2
  exit 1
}
grep -F -q '| `MapGet` | `ColdRuntime` | `nyash.runtime_data.get_hh` |' "$LOWERING_SSOT" || {
  echo "[map-missing-empty-route-trigger-probe] lowering SSOT missing ColdRuntime MapGet row" >&2
  exit 1
}
grep -F -q 'map_lookup_fusion_routes alone:' "$MAP_SSOT" || {
  echo "[map-missing-empty-route-trigger-probe] MapBox SSOT missing metadata-only stop line" >&2
  exit 1
}

echo "[map-missing-empty-route-trigger-probe] ok"
