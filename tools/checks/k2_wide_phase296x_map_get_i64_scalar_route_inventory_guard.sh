#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-i64-scalar-route-inventory"
CARD="docs/development/current/main/phases/phase-296x/296x-853-MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_i64_scalar_route_inventory_guard.sh"
MODEL="src/mir/generic_method_route_plan/model.rs"
COLLECTION_READ="src/mir/generic_method_route_plan/collection_read_routes.rs"
SCALAR_PROOF="src/mir/generic_method_route_plan/map_set_scalar_proof.rs"
TEST_SUPPORT="src/mir/generic_method_route_plan/test_support.rs"
GET_POLICY="lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc"
HAS_POLICY="lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc"
MAP_ALIASES="crates/nyash_kernel/src/plugin/map_aliases.rs"
MAP_KEY_CODEC="crates/nyash_kernel/src/plugin/map_key_codec.rs"
RUNTIME_DATA="crates/nyash_kernel/src/plugin/map_runtime_data.rs"

for file in "$CARD" "$INDEX" "$MODEL" "$COLLECTION_READ" "$SCALAR_PROOF" "$TEST_SUPPORT" "$GET_POLICY" "$HAS_POLICY" "$MAP_ALIASES" "$MAP_KEY_CODEC" "$RUNTIME_DATA"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

grep -q '^Status: Landed$' "$CARD" || {
  echo "[$TAG] card must be Landed" >&2
  exit 1
}

grep -q "$SELF_SCRIPT" "$INDEX" || {
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
  "output_contract=hako-mimalloc-map-get-i64-scalar-route-inventory-v0" \
  "row_kind=inventory" \
  "implementation_started=0" \
  "target_front=kilo_leaf_map_get_missing" \
  "old_c_pair_valid=0" \
  "semantic_route_still_valid=1" \
  "map_has_i64_route_kind=MapContainsI64" \
  "map_has_i64_route_tag=map_contains_i64" \
  "map_has_i64_helper=nyash.map.probe_hi" \
  "map_get_i64_route_kind_present=0" \
  "map_get_i64_route_tag_present=0" \
  "map_get_i64_helper_present_in_route_model=0" \
  "map_get_i64_substrate_helper_present=1" \
  "map_get_i64_substrate_helper=nyash.map.slot_load_hi" \
  "scalar_map_get_store_proof_present=1" \
  "scalar_map_get_store_proof_route_kind=RuntimeDataLoadAny" \
  "scalar_map_get_store_proof_helper=nyash.runtime_data.get_hh" \
  "scalar_map_get_store_proof_tier=ColdFallback" \
  "scalar_map_get_store_proof_return_shape=scalar_i64_or_missing_zero" \
  "scalar_map_get_store_proof_publication_policy=no_publication" \
  "runtime_data_map_get_facade_visible=1" \
  "map_get_i64_key_string_conversion_visible=1" \
  "typed_i64_key_map_storage_enabled=0" \
  "host_handle_boundary_still_visible=1" \
  "benchmark_source_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_helper_changed=0" \
  "product_default_changed=0" \
  "selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-DESIGN-001" \
  "summary=ok"; do
  require_line "$expected"
done

for stop_line in \
  "do not implement MapGetI64 in this inventory row" \
  "do not change benchmark source in this inventory row" \
  "do not infer route legality from helper names alone" \
  "do not claim typed i64-key MapBox storage exists" \
  "do not claim String key conversion is removed by nyash.map.slot_load_hi" \
  "do not broaden to generic MapBox storage replacement" \
  "do not invalidate RuntimeDataBox.get mixed return semantics" \
  "do not reopen the old Hako-vs-C winner claim"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q 'MapContainsI64' "$MODEL" || {
  echo "[$TAG] MapContainsI64 route kind missing" >&2
  exit 1
}

if grep -F -q 'MapLoadI64' "$MODEL"; then
  echo "[$TAG] MapLoadI64 already exists; inventory must be refreshed" >&2
  exit 1
fi

grep -F -q 'nyash.map.probe_hi' "$HAS_POLICY" || {
  echo "[$TAG] MapHas i64 helper missing from has policy" >&2
  exit 1
}

grep -F -q 'nyash.runtime_data.get_hh' "$GET_POLICY" || {
  echo "[$TAG] MapGet runtime_data helper missing from get policy" >&2
  exit 1
}

grep -F -q 'nyash.map.slot_load_hh' "$GET_POLICY" || {
  echo "[$TAG] MapGet map_load_any helper missing from get policy" >&2
  exit 1
}

grep -F -q 'GenericMethodRouteKind::RuntimeDataLoadAny' "$TEST_SUPPORT" || {
  echo "[$TAG] scalar proof fixture no longer uses RuntimeDataLoadAny" >&2
  exit 1
}

grep -F -q 'GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape' "$TEST_SUPPORT" || {
  echo "[$TAG] scalar same-key proof fixture missing" >&2
  exit 1
}

grep -F -q 'GenericMethodRouteProof::MapSetScalarI64DominatesNoEscape' "$SCALAR_PROOF" || {
  echo "[$TAG] scalar dominating proof producer missing" >&2
  exit 1
}

grep -F -q 'nyash.map.slot_load_hi' "$MAP_ALIASES" || {
  echo "[$TAG] substrate map slot_load_hi helper missing" >&2
  exit 1
}

grep -F -q 'map_key_string_from_i64' "$MAP_ALIASES" || {
  echo "[$TAG] slot_load_hi key string conversion evidence missing" >&2
  exit 1
}

grep -F -q 'map_key_string_from_i64' "$RUNTIME_DATA" || {
  echo "[$TAG] runtime_data map get key string conversion evidence missing" >&2
  exit 1
}

grep -F -q 'key_i64.to_string()' "$MAP_KEY_CODEC" || {
  echo "[$TAG] map key i64 stringification evidence missing" >&2
  exit 1
}

echo "[$TAG] ok"
