#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-scalar-load-i64-borrowed-lookup-measurement"
CARD="docs/development/current/main/phases/phase-296x/296x-871-MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-MEASUREMENT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-870-MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-VALIDATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_scalar_load_i64_borrowed_lookup_measurement_guard.sh"

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
  "output_contract=hako-mimalloc-map-scalar-load-i64-borrowed-lookup-measurement-v0" \
  "source_evidence=296x-870" \
  "row_kind=measurement" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "measurement_kind=hako_only_microasm" \
  "c_pair_measurement_used=0" \
  "winner_claim=0" \
  "route_reaches_aot=1" \
  "ny_main_loop_helper=nyash.map.scalar_load_hi" \
  "ny_main_runtime_data_get_hh_import=0" \
  "ny_main_final_const_get_helper=nyash.map.slot_load_hh" \
  "perf_top_symbol_0=nyash.map.scalar_load_hi" \
  "perf_top_symbol_1=core::hash::BuildHasher::hash_one" \
  "perf_top_symbol_2=nyash_rust::boxes::map_box::MapBox::get_scalar_i64_key_str" \
  "previous_spec_to_string_top_symbol_removed=1" \
  "previous_share_box_top_symbol_removed=1" \
  "previous_get_opt_key_str_top_symbol_removed=1" \
  "narrow_slice_effect_observed=1" \
  "observed_owner_shift=map_scalar_helper_string_hash_key_domain" \
  "route_proof_next_owner=none" \
  "compiler_route_next_owner=none" \
  "map_storage_next_owner=MapBox_string_key_hash_domain_or_key_alias_plan" \
  "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-HASHMAP-KEY-DOMAIN-OWNER-SELECTION-001" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-MEASUREMENT-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to this measurement" >&2
  exit 1
}

for evidence in \
  "SpecToString" \
  "MapBox::share_box" \
  "MapBox::get_opt_key_str" \
  "HashMap<String" \
  "stringified-key alias semantics"; do
  grep -F -q "$evidence" "$CARD" || {
    echo "[$TAG] missing evidence/decision text: $evidence" >&2
    exit 1
  }
done

echo "[$TAG] ok"
