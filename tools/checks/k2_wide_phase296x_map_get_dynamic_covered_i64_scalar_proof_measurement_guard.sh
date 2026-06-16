#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-dynamic-covered-i64-scalar-proof-measurement"
CARD="docs/development/current/main/phases/phase-296x/296x-866-MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-MEASUREMENT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-865-MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-IMPLEMENTATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_dynamic_covered_i64_scalar_proof_measurement_guard.sh"

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
  "output_contract=hako-mimalloc-map-get-dynamic-covered-i64-scalar-proof-measurement-v0" \
  "source_evidence=296x-865" \
  "row_kind=measurement" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "target_source=benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako" \
  "measurement_kind=hako_only_microasm" \
  "c_pair_measurement_used=0" \
  "winner_claim=0" \
  "route_reaches_aot=1" \
  "ny_main_loop_helper=nyash.map.scalar_load_hi" \
  "ny_main_runtime_data_get_hh_import=0" \
  "ny_main_final_const_get_helper=nyash.map.slot_load_hh" \
  "perf_top_symbol_0=<i64 as alloc::string::SpecToString>::spec_to_string" \
  "perf_top_symbol_1=core::hash::BuildHasher::hash_one" \
  "perf_top_symbol_2=nyash_rust::boxes::map_box::MapBox::get_opt_key_str" \
  "perf_top_symbol_3=<nyash_rust::boxes::map_box::MapBox as nyash_rust::box_trait::NyashBox>::share_box" \
  "perf_top_symbol_4=nyash.map.scalar_load_hi" \
  "observed_owner_shift=map_scalar_helper_i64_key_string_storage" \
  "route_proof_next_owner=none" \
  "compiler_route_next_owner=none" \
  "map_storage_next_owner=MapBox_i64_key_storage_or_scalar_helper_key_encoding" \
  "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-STORAGE-OWNER-SELECTION-001" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-MEASUREMENT-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to measurement" >&2
  exit 1
}

for evidence in \
  "call   nyash.map.scalar_load_hi" \
  "SpecToString" \
  "BuildHasher::hash_one" \
  "MapBox::get_opt_key_str"; do
  grep -F -q "$evidence" "$CARD" || {
    echo "[$TAG] missing evidence text: $evidence" >&2
    exit 1
  }
done

echo "[$TAG] ok"
