#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-direct-mapbox-scalar-demand-implementation"
CARD="docs/development/current/main/phases/phase-296x/296x-859-MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-858-MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_direct_mapbox_scalar_demand_implementation_guard.sh"
ROUTES="src/mir/generic_method_route_plan/collection_read_routes.rs"
TESTS="src/mir/generic_method_route_plan/tests/map_set_routes/map_get_scalar.rs"
DIRECT_TESTS="src/mir/generic_method_route_plan/tests/core_routes/direct_routes.rs"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$ROUTES" "$TESTS" "$DIRECT_TESTS"; do
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
  "output_contract=hako-mimalloc-map-get-direct-mapbox-scalar-demand-implementation-v0" \
  "source_evidence=296x-858" \
  "row_kind=implementation" \
  "direct_mapbox_get_scalar_route_enabled=1" \
  "direct_mapbox_get_scalar_route_kind=MapLoadScalarI64" \
  "direct_mapbox_get_scalar_route_tag=map_load_scalar_i64" \
  "direct_mapbox_get_scalar_helper=nyash.map.scalar_load_hi" \
  "direct_mapbox_get_scalar_return_shape=ScalarI64OrMissingZero" \
  "direct_mapbox_get_scalar_value_demand=ScalarI64" \
  "direct_mapbox_get_scalar_publication_policy=NoPublication" \
  "unproven_direct_mapbox_get_route_kind=MapLoadAny" \
  "unproven_direct_mapbox_get_helper=nyash.map.slot_load_hh" \
  "mixed_runtime_data_get_route_kind=RuntimeDataLoadAny" \
  "mixed_runtime_data_get_helper=nyash.runtime_data.get_hh" \
  "target_front=kilo_leaf_map_getset_has" \
  "target_front_loop_get_route=map_load_scalar_i64" \
  "target_front_final_get_route=map_load_scalar_i64" \
  "target_front_slot_load_hh_after=0" \
  "typed_i64_key_map_storage_enabled=0" \
  "stored_value_constant_emission_enabled=0" \
  "benchmark_source_changed=0" \
  "product_default_changed=0" \
  "winner_claim=0" \
  "selected_next=MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-VALIDATION-001" \
  "summary=ok"; do
  require_line "$expected"
done

for stop_line in \
  'do not route unproven direct `MapBox.get` to `MapLoadScalarI64`' \
  'do not change mixed `RuntimeDataBox.get` fallback' \
  'do not use the invalid C `map_getset_has` pair for winner claims' \
  'do not infer legality from helper symbol names' \
  'do not add typed i64-key map storage' \
  'do not emit stored-value constants in this route' \
  'do not change benchmark source'; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q "selected_next=MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-IMPLEMENTATION-001" "$PREV_CARD" || {
  echo "[$TAG] previous design row does not hand off to implementation" >&2
  exit 1
}

grep -F -q "if let Some(scalar_fact) = prove_scalar_i64_map_get_store_fact(" "$ROUTES" || {
  echo "[$TAG] direct MapBox.get branch missing scalar proof gate" >&2
  exit 1
}

grep -F -q "GenericMethodRouteKind::MapLoadScalarI64" "$ROUTES" || {
  echo "[$TAG] route producer missing MapLoadScalarI64" >&2
  exit 1
}

grep -F -q "GenericMethodPublicationPolicy::NoPublication" "$ROUTES" || {
  echo "[$TAG] route producer missing no-publication policy" >&2
  exit 1
}

grep -F -q "proves_same_block_direct_mapbox_get_scalar_i64_route" "$TESTS" || {
  echo "[$TAG] direct MapBox scalar proof test missing" >&2
  exit 1
}

grep -F -q "records_direct_mapbox_get_as_warm_core_method_route" "$DIRECT_TESTS" || {
  echo "[$TAG] unproven direct MapBox get regression test missing" >&2
  exit 1
}

echo "[$TAG] ok"
