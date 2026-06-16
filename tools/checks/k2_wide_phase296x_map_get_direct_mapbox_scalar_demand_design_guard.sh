#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-direct-mapbox-scalar-demand-design"
CARD="docs/development/current/main/phases/phase-296x/296x-858-MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-857-MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-VALIDATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_direct_mapbox_scalar_demand_design_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

grep -q '^Status: Landed$' "$CARD" || {
  echo "[$TAG] card must be Landed" >&2
  exit 1
}

grep -F -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[$TAG] check index missing guard entry" >&2
  exit 1
}

require_line() {
  local expected="$1"
  if ! grep -F -x -q "$expected" "$CARD"; then
    echo "[$TAG] missing card line: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-map-get-direct-mapbox-scalar-demand-design-v0" \
  "row_kind=design" \
  "implementation_started=0" \
  "target_front=kilo_leaf_map_getset_has" \
  "c_pair_comparison_valid=0" \
  "c_pair_measurement_used=0" \
  "runtime_data_scalar_route_validated=1" \
  "source_front_runtime_data_loop_get_route=map_load_scalar_i64" \
  "source_front_final_direct_mapbox_get_route=map_load_any" \
  "source_front_final_direct_mapbox_get_helper=nyash.map.slot_load_hh" \
  "selected_next_owner=direct_mapbox_get_scalar_demand_route" \
  "selected_route_kind=MapLoadScalarI64" \
  "selected_helper=nyash.map.scalar_load_hi" \
  "selected_surface=MapBox.get" \
  "selected_proofs=map_set_scalar_i64_same_key_no_escape,map_set_scalar_i64_dominates_no_escape" \
  "slot_load_hh_unproven_route_retained=1" \
  "runtime_data_load_any_mixed_route_retained=1" \
  "typed_i64_key_map_storage_enabled=0" \
  "stored_value_constant_emission_enabled=0" \
  "benchmark_source_changed=0" \
  "product_default_changed=0" \
  "winner_claim=0" \
  "selected_next=MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-IMPLEMENTATION-001" \
  "summary=ok"; do
  require_line "$expected"
done

for stop_line in \
  'do not use the invalid C `map_getset_has` pair for winner claims' \
  'do not infer legality from `nyash.map.slot_load_hh` or `nyash.map.scalar_load_hi` symbols' \
  'do not route unproven direct `MapBox.get` to `MapLoadScalarI64`' \
  'do not change mixed `RuntimeDataBox.get` fallback' \
  'do not change direct `MapBox.get` handle contract when scalar proof is absent' \
  'do not add typed i64-key map storage' \
  'do not emit stored-value constants in this route' \
  'do not change benchmark source'; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q "selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-MEASUREMENT-001" "$PREV_CARD" || {
  echo "[$TAG] previous validation row must hand off to measurement before this design retarget" >&2
  exit 1
}

echo "[$TAG] ok"
