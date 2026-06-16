#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-nonfolded-scalar-front-selection"
CARD="docs/development/current/main/phases/phase-296x/296x-862-MIMALLOC-MAP-GET-NONFOLDED-SCALAR-FRONT-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-861-MIMALLOC-MAP-GET-DIRECT-MAPBOX-SCALAR-DEMAND-MEASUREMENT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_nonfolded_scalar_front_selection_guard.sh"
BENCH="benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako"

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
  "output_contract=hako-mimalloc-map-get-nonfolded-scalar-front-selection-v0" \
  "source_evidence=296x-861" \
  "row_kind=front_selection" \
  "selected_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "selected_source=benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako" \
  "selected_front_is_hako_only=1" \
  "c_pair_comparison_valid=0" \
  "c_pair_measurement_used=0" \
  "winner_claim=0" \
  "selected_front_shape=preseed_const_i64_values_dynamic_i64_key_get_loop" \
  "loop_key_shape=i_mod_3" \
  "preseeded_key_count=3" \
  "loop_repeated_map_get_present=1" \
  "loop_repeated_map_get_helper_current=nyash.runtime_data.get_hh" \
  "loop_repeated_map_get_scalar_route_current=0" \
  "final_const_key_get_helper_current=nyash.map.slot_load_hh" \
  "rejected_front=kilo_leaf_map_getset_has" \
  "rejected_reason=folded_single_store_single_scalar_load" \
  "rejected_body_loop_repeated_map_get_measurement_available=0" \
  "scratch_set_get_const_value_route_current=map_load_any_or_constant_eliminated" \
  "scratch_set_get_dynamic_value_route_current=map_load_any_or_value_eliminated" \
  "scratch_env_guard_route_current=map_load_scalar_i64_hoisted_out_of_loop" \
  "selected_next=MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-DESIGN-001" \
  "implementation_started=0" \
  "summary=ok"; do
  require_line "$expected"
done

for stop_line in \
  'do not claim Hako-vs-C winner from this Hako-only front' \
  'do not use invalid C volatile comparison pairs as map lookup evidence' \
  'do not add benchmark-specific route branches' \
  'do not infer scalar legality from `nyash.runtime_data.get_hh` alone' \
  'do not treat preseeded key coverage as proven until the next design row' \
  'do not change product MapBox semantics' \
  'do not change MapBox storage representation in this row'; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q "selected_next=MIMALLOC-MAP-GET-NONFOLDED-SCALAR-FRONT-SELECTION-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to front selection" >&2
  exit 1
}

for src_line in \
  "map.set(0, 1)" \
  "map.set(1, 2)" \
  "map.set(2, 3)" \
  "local k = i % 3" \
  "local v = map.get(k)" \
  "return sum + map.get(1)"; do
  grep -F -q "$src_line" "$BENCH" || {
    echo "[$TAG] selected bench missing source shape line: $src_line" >&2
    exit 1
  }
done

TMP_DIR="target/tmp/${TAG}"
mkdir -p "$TMP_DIR"
MIR_JSON="$TMP_DIR/map_get_dynamic_covered_i64.mir.json"
OBJ="$TMP_DIR/map_get_dynamic_covered_i64.o"

NYASH_GC_MODE=off NYASH_DISABLE_PLUGINS=1 NYASH_SKIP_TOML_ENV=1 \
  target/release/hakorune --backend mir --emit-mir-json "$MIR_JSON" "$BENCH" >/tmp/${TAG}.emit_mir.log 2>&1

runtime_get_count="$({ grep -F '"symbol": "nyash.runtime_data.get_hh"' "$MIR_JSON" || true; } | wc -l | tr -d ' ')"
if [[ "$runtime_get_count" -lt 1 ]]; then
  echo "[$TAG] expected selected front to expose current runtime_data.get_hh loop owner" >&2
  exit 1
fi

scalar_route_count="$({ grep -F '"route_kind": "map_load_scalar_i64"' "$MIR_JSON" || true; } | wc -l | tr -d ' ')"
if [[ "$scalar_route_count" -ne 0 ]]; then
  echo "[$TAG] selected front unexpectedly already has scalar route count=$scalar_route_count" >&2
  exit 1
fi

NYASH_LLVM_SKIP_BUILD=1 bash tools/ny_mir_builder.sh --in "$MIR_JSON" --emit obj -o "$OBJ" --quiet
nm -u "$OBJ" > "$TMP_DIR/map_get_dynamic_covered_i64.nm"

for required in "nyash.map.birth_h" "nyash.map.slot_store_hhh" "nyash.runtime_data.get_hh"; do
  grep -F -q "$required" "$TMP_DIR/map_get_dynamic_covered_i64.nm" || {
    echo "[$TAG] object missing required current-owner symbol: $required" >&2
    cat "$TMP_DIR/map_get_dynamic_covered_i64.nm" >&2 || true
    exit 1
  }
done

echo "[$TAG] ok"
