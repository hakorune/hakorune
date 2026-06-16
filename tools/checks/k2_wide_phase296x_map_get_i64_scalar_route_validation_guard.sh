#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-i64-scalar-route-validation"
CARD="docs/development/current/main/phases/phase-296x/296x-857-MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-VALIDATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-856-MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-IMPLEMENTATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_i64_scalar_route_validation_guard.sh"
FIXTURE="apps/tests/mir_shape_guard/lowering_plan_map_get_scalar_i64_directabi_min_v1.mir.json"
MIXED_FIXTURE="apps/tests/mir_shape_guard/lowering_plan_runtime_data_map_get_min_v1.mir.json"
DIRECT_FIXTURE="apps/tests/mir_shape_guard/lowering_plan_map_get_directabi_min_v1.mir.json"
SHIM="target/release/libhako_llvmc_ffi.so"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$SELF_SCRIPT" "$FIXTURE" "$MIXED_FIXTURE" "$DIRECT_FIXTURE"; do
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
  "output_contract=hako-mimalloc-map-get-i64-scalar-route-validation-v0" \
  "source_evidence=296x-856" \
  "row_kind=validation" \
  "validation_fixture=apps/tests/mir_shape_guard/lowering_plan_map_get_scalar_i64_directabi_min_v1.mir.json" \
  "validation_guard=tools/checks/k2_wide_phase296x_map_get_i64_scalar_route_validation_guard.sh" \
  "c_shim_rebuild_required=1" \
  "c_shim_contains_map_load_scalar_i64=1" \
  "c_shim_contains_scalar_helper=1" \
  "scalar_validation_object_emitted=1" \
  "scalar_validation_symbol_present=nyash.map.scalar_load_hi" \
  "scalar_validation_map_birth_symbol_present=nyash.map.birth_h" \
  "scalar_validation_runtime_data_get_hh_symbol_present=0" \
  "scalar_validation_slot_load_hh_symbol_present=0" \
  "scalar_validation_slot_load_hi_symbol_present=0" \
  "mixed_get_fixture_still_runtime_data_get_hh=1" \
  "direct_mapbox_get_fixture_still_slot_load_hh=1" \
  "map_get_scalar_i64_route_kind_present=1" \
  "map_get_scalar_i64_route_tag=map_load_scalar_i64" \
  "map_get_scalar_i64_helper=nyash.map.scalar_load_hi" \
  "benchmark_source_changed=0" \
  "product_default_changed=0" \
  "stored_value_constant_emission_enabled=0" \
  "typed_i64_key_map_storage_enabled=0" \
  "string_key_map_storage_changed=0" \
  "winner_claim=0" \
  "selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-MEASUREMENT-001" \
  "summary=ok"; do
  require_line "$expected"
done

for stop_line in \
  "do not change C benchmark source" \
  "do not claim Hako-vs-C map_get win from the invalid old C pair" \
  "do not route unproven get calls to MapLoadScalarI64" \
  "do not use nyash.map.slot_load_hi as scalar helper" \
  "do not change mixed RuntimeDataBox.get fallback" \
  "do not change direct MapBox.get handle return contract" \
  "do not add typed i64-key map storage" \
  "do not emit stored_value constants in this helper route" \
  "do not skip C shim rebuild in validation"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q "selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-VALIDATION-001" "$PREV_CARD" || {
  echo "[$TAG] previous implementation row does not hand off to validation" >&2
  exit 1
}

grep -F -q '"route_kind": "map_load_scalar_i64"' "$FIXTURE" || {
  echo "[$TAG] scalar fixture missing scalar route kind" >&2
  exit 1
}

grep -F -q '"symbol": "nyash.map.scalar_load_hi"' "$FIXTURE" || {
  echo "[$TAG] scalar fixture missing scalar helper" >&2
  exit 1
}

grep -F -q '"publication_policy": "no_publication"' "$FIXTURE" || {
  echo "[$TAG] scalar fixture must be no-publication" >&2
  exit 1
}

grep -F -q '"route_kind": "runtime_data_load_any"' "$MIXED_FIXTURE" || {
  echo "[$TAG] mixed fixture route drifted away from runtime fallback" >&2
  exit 1
}

grep -F -q '"symbol": "nyash.runtime_data.get_hh"' "$MIXED_FIXTURE" || {
  echo "[$TAG] mixed fixture symbol drifted away from runtime fallback" >&2
  exit 1
}

grep -F -q '"route_kind": "map_load_any"' "$DIRECT_FIXTURE" || {
  echo "[$TAG] direct fixture route drifted away from handle load" >&2
  exit 1
}

grep -F -q '"symbol": "nyash.map.slot_load_hh"' "$DIRECT_FIXTURE" || {
  echo "[$TAG] direct fixture symbol drifted away from handle load" >&2
  exit 1
}

bash tools/build_hako_llvmc_ffi.sh >/tmp/hako_llvmc_ffi_build.log
[[ -f "$SHIM" ]] || { echo "[$TAG] C shim not built: $SHIM" >&2; exit 1; }

TMP_DIR="target/tmp/${TAG}"
mkdir -p "$TMP_DIR"
strings "$SHIM" > "$TMP_DIR/shim.strings"

grep -F -q "map_load_scalar_i64" "$TMP_DIR/shim.strings" || {
  echo "[$TAG] C shim missing scalar route tag after rebuild" >&2
  exit 1
}

grep -F -q "nyash.map.scalar_load_hi" "$TMP_DIR/shim.strings" || {
  echo "[$TAG] C shim missing scalar helper after rebuild" >&2
  exit 1
}

emit_obj() {
  local fixture="$1"
  local obj="$2"
  NYASH_LLVM_SKIP_BUILD=1 bash tools/ny_mir_builder.sh --in "$fixture" --emit obj -o "$obj" --quiet
  [[ -f "$obj" ]] || {
    echo "[$TAG] object not emitted: $obj" >&2
    exit 1
  }
}

emit_obj "$FIXTURE" "$TMP_DIR/scalar.o"
emit_obj "$MIXED_FIXTURE" "$TMP_DIR/mixed.o"
emit_obj "$DIRECT_FIXTURE" "$TMP_DIR/direct.o"

nm -u "$TMP_DIR/scalar.o" > "$TMP_DIR/scalar.nm"
nm -u "$TMP_DIR/mixed.o" > "$TMP_DIR/mixed.nm"
nm -u "$TMP_DIR/direct.o" > "$TMP_DIR/direct.nm"

grep -F -q "nyash.map.scalar_load_hi" "$TMP_DIR/scalar.nm" || {
  echo "[$TAG] scalar object missing scalar helper symbol" >&2
  cat "$TMP_DIR/scalar.nm" >&2 || true
  exit 1
}

grep -F -q "nyash.map.birth_h" "$TMP_DIR/scalar.nm" || {
  echo "[$TAG] scalar object missing MapBox birth symbol" >&2
  cat "$TMP_DIR/scalar.nm" >&2 || true
  exit 1
}

for forbidden in "nyash.runtime_data.get_hh" "nyash.map.slot_load_hh" "nyash.map.slot_load_hi"; do
  if grep -F -q "$forbidden" "$TMP_DIR/scalar.nm"; then
    echo "[$TAG] scalar object unexpectedly imports $forbidden" >&2
    cat "$TMP_DIR/scalar.nm" >&2 || true
    exit 1
  fi
done

grep -F -q "nyash.runtime_data.get_hh" "$TMP_DIR/mixed.nm" || {
  echo "[$TAG] mixed fallback fixture no longer imports runtime_data.get_hh" >&2
  cat "$TMP_DIR/mixed.nm" >&2 || true
  exit 1
}

grep -F -q "nyash.map.slot_load_hh" "$TMP_DIR/direct.nm" || {
  echo "[$TAG] direct handle fixture no longer imports slot_load_hh" >&2
  cat "$TMP_DIR/direct.nm" >&2 || true
  exit 1
}

echo "[$TAG] ok"
