#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-839-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-838-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-TRIGGER-PROBE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_missing_empty_route_design_guard.sh"
MAP_SSOT="docs/development/current/main/design/mapbox-proof-bearing-route-ssot.md"
LOWERING_SSOT="docs/development/current/main/design/lowering-plan-json-v0-ssot.md"
MAP_BOX="src/boxes/map_box.rs"
MAP_FUSION_PLAN="src/mir/map_lookup_fusion_plan.rs"

[[ -f "$CARD" ]] || { echo "[map-missing-empty-route-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[map-missing-empty-route-design] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$MAP_SSOT" ]] || { echo "[map-missing-empty-route-design] missing MapBox SSOT" >&2; exit 1; }
[[ -f "$LOWERING_SSOT" ]] || { echo "[map-missing-empty-route-design] missing lowering SSOT" >&2; exit 1; }
[[ -f "$MAP_BOX" ]] || { echo "[map-missing-empty-route-design] missing MapBox source" >&2; exit 1; }
[[ -f "$MAP_FUSION_PLAN" ]] || { echo "[map-missing-empty-route-design] missing map fusion plan" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[map-missing-empty-route-design] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[map-missing-empty-route-design] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[map-missing-empty-route-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[map-missing-empty-route-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-map-missing-empty-route-design-v0" \
  "source_evidence=296x-838" \
  "row_kind=design" \
  "implementation_started=0" \
  "perf_first_required=1" \
  "target_front=kilo_leaf_map_get_missing" \
  "route_family=MapMissingEmptyRoute" \
  "selected_route=map_get_missing_empty_const_zero" \
  "semantic_op=MapGet" \
  "result_missing_value=zero_null" \
  "fallback_route=generic_map_runtime_data_load_any" \
  "receiver_birth_is_new_mapbox_required=1" \
  "receiver_root_is_same_local_value_required=1" \
  "receiver_not_published_before_get_required=1" \
  "receiver_not_escaped_before_get_required=1" \
  "no_map_set_before_get_required=1" \
  "no_map_delete_before_get_required=1" \
  "no_map_clear_before_get_required=1" \
  "no_unknown_receiver_mutation_before_get_required=1" \
  "get_key_route_required=i64_const" \
  "generic_method_publication_policy_required=runtime_data_facade" \
  "generic_method_return_shape_required=mixed_runtime_i64_or_handle" \
  "backend_consumes_route_decision_only=1" \
  "map_lookup_fusion_route_reused=0" \
  "map_repr_plan_changed=0" \
  "mapbox_storage_changed=0" \
  "product_default_changed=0" \
  "selected_next=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-GUARD-SURFACE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for stop_line in \
  "do not implement before guard surface" \
  "do not fold missing get from helper names" \
  "do not fold missing get from literal key 0 alone" \
  "do not reuse MapLookupFusionRoute for get-only missing-empty proof" \
  "do not call generic MapBox a DirectMap" \
  "do not read raw MapBox storage from backend" \
  "do not change MapBox missing-key public semantics" \
  "do not silently fall back from a required selected route"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[map-missing-empty-route-design] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q 'proof-bearing Map route default' "$MAP_SSOT" || {
  echo "[map-missing-empty-route-design] MapBox proof-bearing route SSOT drifted" >&2
  exit 1
}
grep -F -q 'RouteDecision selected_route:' "$MAP_SSOT" || {
  echo "[map-missing-empty-route-design] MapBox SSOT missing selected route contract" >&2
  exit 1
}
grep -F -q '| `MapGet` | `ColdRuntime` | `nyash.runtime_data.get_hh` |' "$LOWERING_SSOT" || {
  echo "[map-missing-empty-route-design] lowering SSOT missing runtime MapGet row" >&2
  exit 1
}
grep -F -q 'data: Arc<RwLock<HashMap<String, Box<dyn NyashBox>>>>' "$MAP_BOX" || {
  echo "[map-missing-empty-route-design] MapBox storage shape changed in design row" >&2
  exit 1
}
grep -F -q 'MapLookupSameKey' "$MAP_FUSION_PLAN" || {
  echo "[map-missing-empty-route-design] MapLookupFusionRoute scope evidence missing" >&2
  exit 1
}

echo "[map-missing-empty-route-design] ok"
