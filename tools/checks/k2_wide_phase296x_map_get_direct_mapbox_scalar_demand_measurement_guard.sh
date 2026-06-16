#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-direct-mapbox-scalar-demand-measurement"
CARD="docs/development/current/main/phases/phase-296x/296x-861-MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-MEASUREMENT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-860-MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-VALIDATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_direct_mapbox_scalar_demand_measurement_guard.sh"
BENCH="benchmarks/bench_kilo_leaf_map_getset_has.hako"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$BENCH"; do
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
  "output_contract=hako-mimalloc-map-get-direct-mapbox-scalar-demand-measurement-v0" \
  "source_evidence=296x-860" \
  "row_kind=measurement_attribution" \
  "target_front=kilo_leaf_map_getset_has" \
  "c_pair_comparison_valid=0" \
  "c_pair_measurement_used=0" \
  "winner_claim=0" \
  "source_mir_scalar_route_count_min=2" \
  "source_mir_slot_load_hh_count=0" \
  "source_mir_runtime_data_get_hh_count=0" \
  "source_aot_object_scalar_helper_symbol=nyash.map.scalar_load_hi" \
  "source_aot_object_birth_symbol=nyash.map.birth_h" \
  "source_aot_object_store_symbol=nyash.map.slot_store_hhh" \
  "source_aot_object_slot_load_hh_symbol_present=0" \
  "source_aot_object_runtime_data_get_hh_symbol_present=0" \
  "source_aot_exe_ny_main_body_shape=folded_single_store_single_scalar_load" \
  "body_loop_repeated_map_get_measurement_available=0" \
  "loader_dominated_perf_sample_observed=1" \
  "measurement_keeper_claim=0" \
  "implementation_keeper_claim=route_reaches_aot_only" \
  "selected_next=MIMALLOC-MAP-GET-NONFOLDED-SCALAR-FRONT-SELECTION-001" \
  "summary=ok"; do
  require_line "$expected"
done

for stop_line in \
  'do not claim Hako-vs-C winner from `kilo_leaf_map_getset_has`' \
  'do not use the invalid C map pair as map lookup evidence' \
  'do not use loader-dominated perf samples as kernel evidence' \
  'do not add benchmark-specific route branches' \
  'do not change benchmark source in this row' \
  'do not pursue more MapGet implementation without a non-folded scalar front'; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q "selected_next=MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-MEASUREMENT-001" "$PREV_CARD" || {
  echo "[$TAG] previous validation row does not hand off to measurement" >&2
  exit 1
}

TMP_DIR="target/tmp/${TAG}"
mkdir -p "$TMP_DIR"
MIR_JSON="$TMP_DIR/map_getset_has.mir.json"
OBJ="$TMP_DIR/map_getset_has.o"

NYASH_GC_MODE=off NYASH_DISABLE_PLUGINS=1 NYASH_SKIP_TOML_ENV=1 \
  target/release/hakorune --backend mir --emit-mir-json "$MIR_JSON" "$BENCH" >/tmp/${TAG}.emit_mir.log 2>&1

scalar_route_count="$(grep -F '"route_kind": "map_load_scalar_i64"' "$MIR_JSON" | wc -l | tr -d ' ')"
if [[ "$scalar_route_count" -lt 2 ]]; then
  echo "[$TAG] expected at least two scalar map get route metadata entries, got $scalar_route_count" >&2
  exit 1
fi

if grep -F -q '"symbol": "nyash.map.slot_load_hh"' "$MIR_JSON"; then
  echo "[$TAG] source MIR contains slot_load_hh after scalar-demand validation" >&2
  exit 1
fi

if grep -F -q '"symbol": "nyash.runtime_data.get_hh"' "$MIR_JSON"; then
  echo "[$TAG] source MIR contains runtime_data.get_hh after scalar-demand validation" >&2
  exit 1
fi

NYASH_LLVM_SKIP_BUILD=1 bash tools/ny_mir_builder.sh --in "$MIR_JSON" --emit obj -o "$OBJ" --quiet
nm -u "$OBJ" > "$TMP_DIR/map_getset_has.nm"

for required in "nyash.map.scalar_load_hi" "nyash.map.birth_h" "nyash.map.slot_store_hhh"; do
  grep -F -q "$required" "$TMP_DIR/map_getset_has.nm" || {
    echo "[$TAG] object missing required symbol: $required" >&2
    cat "$TMP_DIR/map_getset_has.nm" >&2 || true
    exit 1
  }
done

for forbidden in "nyash.map.slot_load_hh" "nyash.runtime_data.get_hh"; do
  if grep -F -q "$forbidden" "$TMP_DIR/map_getset_has.nm"; then
    echo "[$TAG] object unexpectedly imports $forbidden" >&2
    cat "$TMP_DIR/map_getset_has.nm" >&2 || true
    exit 1
  fi
done

echo "[$TAG] ok"
