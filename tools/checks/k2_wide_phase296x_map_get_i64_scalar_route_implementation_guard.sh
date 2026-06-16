#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-i64-scalar-route-implementation"
CARD="docs/development/current/main/phases/phase-296x/296x-856-MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-855-MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_i64_scalar_route_implementation_guard.sh"

MODEL="src/mir/generic_method_route_plan/model.rs"
ROUTES="src/mir/generic_method_route_plan/collection_read_routes.rs"
TEST_SUPPORT="src/mir/generic_method_route_plan/test_support.rs"
SCALAR_TEST="src/mir/generic_method_route_plan/tests/scalar_proof.rs"
MAP_SET_TEST="src/mir/generic_method_route_plan/tests/map_set_routes/map_get_scalar.rs"
MAP_ALIASES="crates/nyash_kernel/src/plugin/map_aliases.rs"
MAP_TEST="crates/nyash_kernel/src/plugin/map.rs"
GET_POLICY="lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc"
GENERIC_MATCH="lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc"
MIR_ROUTE_POLICY="lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc"
NEED_RULES="lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_metadata_rules.inc"
PRESCAN="lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc"

for file in \
  "$CARD" "$PREV_CARD" "$INDEX" "$MODEL" "$ROUTES" "$TEST_SUPPORT" \
  "$SCALAR_TEST" "$MAP_SET_TEST" "$MAP_ALIASES" "$MAP_TEST" \
  "$GET_POLICY" "$GENERIC_MATCH" "$MIR_ROUTE_POLICY" "$NEED_RULES" "$PRESCAN"; do
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
  "output_contract=hako-mimalloc-map-get-i64-scalar-route-implementation-v0" \
  "source_evidence=296x-854,296x-855" \
  "row_kind=implementation" \
  "map_get_scalar_i64_route_kind_present=1" \
  "map_get_scalar_i64_route_tag=map_load_scalar_i64" \
  "map_get_scalar_i64_helper=nyash.map.scalar_load_hi" \
  "scalar_proof_runtime_data_get_route_kind=MapLoadScalarI64" \
  "scalar_proof_lowering_tier=WarmDirectAbi" \
  "scalar_proof_publication_policy=NoPublication" \
  "mixed_runtime_data_get_route_kind=RuntimeDataLoadAny" \
  "mixed_runtime_data_get_helper=nyash.runtime_data.get_hh" \
  "direct_mapbox_get_route_kind=MapLoadAny" \
  "direct_mapbox_get_helper=nyash.map.slot_load_hh" \
  "slot_load_hi_scalar_route_usage=0" \
  "benchmark_source_changed=0" \
  "product_default_changed=0" \
  "stored_value_constant_emission_enabled=0" \
  "typed_i64_key_map_storage_enabled=0" \
  "string_key_map_storage_changed=0" \
  "selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-VALIDATION-001" \
  "summary=ok"; do
  require_line "$expected"
done

for stop_line in \
  "do not route unproven get calls to MapLoadScalarI64" \
  "do not use nyash.map.slot_load_hi as scalar helper" \
  "do not change C benchmark source" \
  "do not add typed i64-key map storage" \
  "do not emit stored_value constants in this helper route" \
  "do not change RuntimeDataBox.get mixed return contract" \
  "do not change direct MapBox.get handle return contract" \
  "do not claim String-key conversion is removed" \
  "do not claim Hako-vs-C map_get win from the invalid old C pair"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q "selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-IMPLEMENTATION-001" "$PREV_CARD" || {
  echo "[$TAG] previous guard surface does not hand off to implementation" >&2
  exit 1
}

grep -F -q "MapLoadScalarI64" "$MODEL" || {
  echo "[$TAG] Rust route kind missing" >&2
  exit 1
}

grep -F -q "Self::MapLoadScalarI64 => \"nyash.map.scalar_load_hi\"" "$MODEL" || {
  echo "[$TAG] Rust route kind missing scalar helper mapping" >&2
  exit 1
}

grep -F -q "Self::MapLoadScalarI64 => \"map_load_scalar_i64\"" "$MODEL" || {
  echo "[$TAG] Rust route kind missing metadata tag mapping" >&2
  exit 1
}

grep -F -q "GenericMethodRouteKind::MapLoadScalarI64" "$ROUTES" || {
  echo "[$TAG] route producer does not emit MapLoadScalarI64" >&2
  exit 1
}

grep -F -q "CoreMethodLoweringTier::WarmDirectAbi" "$ROUTES" || {
  echo "[$TAG] scalar route producer missing WarmDirectAbi tier" >&2
  exit 1
}

grep -F -q "MapSetScalarI64SameKeyNoEscape" "$MAP_SET_TEST" || {
  echo "[$TAG] same-key scalar proof test coverage missing" >&2
  exit 1
}

grep -F -q "MapSetScalarI64DominatesNoEscape" "$SCALAR_TEST" || {
  echo "[$TAG] dominance scalar proof test coverage missing" >&2
  exit 1
}

grep -F -q "route_kind: GenericMethodRouteKind::MapLoadScalarI64" "$TEST_SUPPORT" || {
  echo "[$TAG] scalar test support route kind not updated" >&2
  exit 1
}

for test_file in "$SCALAR_TEST" "$MAP_SET_TEST"; do
  grep -F -q "nyash.map.scalar_load_hi" "$test_file" || {
    echo "[$TAG] scalar proof test missing scalar helper expectation: $test_file" >&2
    exit 1
  }
  grep -F -q "MapLoadScalarI64" "$test_file" || {
    echo "[$TAG] scalar proof test missing route kind expectation: $test_file" >&2
    exit 1
  }
done

grep -F -q '#[export_name = "nyash.map.scalar_load_hi"]' "$MAP_ALIASES" || {
  echo "[$TAG] runtime scalar helper export missing" >&2
  exit 1
}

grep -F -q "map_slot_load_str_with" "$MAP_ALIASES" || {
  echo "[$TAG] runtime scalar helper must use map slot load substrate" >&2
  exit 1
}

grep -F -q "scalar_load_hi_keeps_no_publication_scalar_contract" "$MAP_TEST" || {
  echo "[$TAG] runtime scalar helper test missing" >&2
  exit 1
}

grep -F -q "nyash_map_scalar_load_hi_alias(handle, -71002), 0" "$MAP_TEST" || {
  echo "[$TAG] runtime scalar helper test must reject handle publication" >&2
  exit 1
}

grep -F -q "HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_SCALAR_I64" "$GET_POLICY" || {
  echo "[$TAG] C get policy scalar route enum missing" >&2
  exit 1
}

grep -F -q '"map_load_scalar_i64"' "$GET_POLICY" || {
  echo "[$TAG] C get policy scalar route parser missing" >&2
  exit 1
}

grep -F -q 'nyash.map.scalar_load_hi' "$GET_POLICY" || {
  echo "[$TAG] C get policy scalar helper emit missing" >&2
  exit 1
}

grep -F -q '"map_load_scalar_i64"' "$GENERIC_MATCH" || {
  echo "[$TAG] generic method match scalar route missing" >&2
  exit 1
}

grep -F -q '"map_load_scalar_i64"' "$MIR_ROUTE_POLICY" || {
  echo "[$TAG] MIR call route policy scalar route missing" >&2
  exit 1
}

grep -F -q '"nyash.map.scalar_load_hi"' "$NEED_RULES" || {
  echo "[$TAG] metadata need rules scalar helper missing" >&2
  exit 1
}

grep -F -q 'declare i64 @\"nyash.map.scalar_load_hi\"' "$PRESCAN" || {
  echo "[$TAG] pure compile prescan scalar helper declaration missing" >&2
  exit 1
}

if grep -F -q "nyash.map.slot_load_hi\";" "$MODEL"; then
  echo "[$TAG] model must not use slot_load_hi as map scalar helper" >&2
  exit 1
fi

echo "[$TAG] ok"
