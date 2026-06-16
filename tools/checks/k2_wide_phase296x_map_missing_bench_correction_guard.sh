#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-missing-bench-correction"
CARD="docs/development/current/main/phases/phase-296x/296x-852-MIMALLOC-MAP-MISSING-BENCH-CORRECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_missing_bench_correction_guard.sh"
C_BENCH="benchmarks/c/bench_kilo_leaf_map_get_missing.c"
HAKO_BENCH="benchmarks/bench_kilo_leaf_map_get_missing.hako"
GET_POLICY="lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc"
HAS_POLICY="lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc"
MODEL="src/mir/generic_method_route_plan/model.rs"
RUNTIME_DATA="crates/nyash_kernel/src/plugin/map_runtime_data.rs"

for file in "$CARD" "$INDEX" "$C_BENCH" "$HAKO_BENCH" "$GET_POLICY" "$HAS_POLICY" "$MODEL" "$RUNTIME_DATA"; do
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
  "output_contract=hako-mimalloc-map-missing-bench-correction-v0" \
  "row_kind=correction" \
  "implementation_started=0" \
  "perf_claim_correction=1" \
  "target_front=kilo_leaf_map_get_missing" \
  "c_pair_performs_map_lookup=0" \
  "c_pair_shape=volatile_i64_compare_only" \
  "hako_source_performs_map_get=1" \
  "benchmark_pair_apples_to_oranges=1" \
  "map_missing_c_comparison_valid=0" \
  "map_missing_route_winner_claim_retracted=1" \
  "map_missing_previous_ratio_claim_valid=0" \
  "map_missing_empty_route_semantic_fact_invalidated=0" \
  "map_has_i64_scalar_route_present=1" \
  "map_get_i64_scalar_route_present=0" \
  "map_get_runtime_data_facade_visible=1" \
  "map_get_i64_key_string_conversion_visible=1" \
  "array_text_loop_session_plan_surface_still_landed=1" \
  "array_text_loop_session_inventory_resume_after_map_correction=1" \
  "benchmark_source_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_helper_changed=0" \
  "product_default_changed=0" \
  "selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-INVENTORY-001" \
  "summary=ok"; do
  require_line "$expected"
done

for stop_line in \
  "do not claim Hako beat C on kilo_leaf_map_get_missing from the old C pair" \
  "do not use volatile-compare C timing as map lookup evidence" \
  "do not invalidate the semantic MapMissingEmptyRoute solely from benchmark hygiene" \
  "do not add MapGetI64 lowering before route inventory" \
  "do not infer MapGetI64 legality from helper symbols alone" \
  "do not change benchmark source in this correction row" \
  "do not lose the 296x-851 ArrayTextLoopSessionPlan resume pointer"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q 'volatile int64_t key = 0;' "$C_BENCH" || {
  echo "[$TAG] C bench no longer shows volatile compare evidence" >&2
  exit 1
}

grep -F -q 'if (key == 0)' "$C_BENCH" || {
  echo "[$TAG] C bench missing volatile compare branch" >&2
  exit 1
}

for forbidden in hashmap unordered_map map_get MapBox 'get('; do
  if grep -F -q "$forbidden" "$C_BENCH"; then
    echo "[$TAG] C bench appears to contain a map lookup token: $forbidden" >&2
    echo "[$TAG] correction row needs refresh" >&2
    exit 1
  fi
done

grep -F -q 'local v = map.get(0)' "$HAKO_BENCH" || {
  echo "[$TAG] Hako bench missing map.get(0) evidence" >&2
  exit 1
}

grep -F -q 'nyash.map.probe_hi' "$HAS_POLICY" || {
  echo "[$TAG] MapHas i64 probe route missing" >&2
  exit 1
}

grep -F -q 'MapContainsI64' "$MODEL" || {
  echo "[$TAG] model missing MapContainsI64 route" >&2
  exit 1
}

grep -F -q 'nyash.runtime_data.get_hh' "$GET_POLICY" || {
  echo "[$TAG] MapGet runtime_data facade evidence missing" >&2
  exit 1
}

grep -F -q 'map_key_string_from_i64' "$RUNTIME_DATA" || {
  echo "[$TAG] map get i64 key string-conversion evidence missing" >&2
  exit 1
}

echo "[$TAG] ok"
