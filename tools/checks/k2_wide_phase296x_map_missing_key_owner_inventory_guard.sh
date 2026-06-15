#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-837-MIMALLOC-MAP-MISSING-KEY-OWNER-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-836-MIMALLOC-FRESH-FRONT-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_missing_key_owner_inventory_guard.sh"
MAP_BOX="src/boxes/map_box.rs"
MAP_FUSION_PLAN="src/mir/map_lookup_fusion_plan.rs"
MAP_LOWERER="src/llvm_py/instructions/mir_call/collection_method_call.py"
MAP_METADATA_LOADER="src/llvm_py/builders/function_metadata.py"

[[ -f "$CARD" ]] || { echo "[map-missing-key-owner-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[map-missing-key-owner-inventory] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$MAP_BOX" ]] || { echo "[map-missing-key-owner-inventory] missing MapBox source" >&2; exit 1; }
[[ -f "$MAP_FUSION_PLAN" ]] || { echo "[map-missing-key-owner-inventory] missing map fusion plan" >&2; exit 1; }
[[ -f "$MAP_LOWERER" ]] || { echo "[map-missing-key-owner-inventory] missing map lowerer" >&2; exit 1; }
[[ -f "$MAP_METADATA_LOADER" ]] || { echo "[map-missing-key-owner-inventory] missing map metadata loader" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[map-missing-key-owner-inventory] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[map-missing-key-owner-inventory] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[map-missing-key-owner-inventory] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[map-missing-key-owner-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-map-missing-key-owner-inventory-v0" \
  "source_evidence=296x-836" \
  "row_kind=inventory" \
  "implementation_started=0" \
  "perf_first_required=1" \
  "target_front=kilo_leaf_map_get_missing" \
  "target_source=benchmarks/bench_kilo_leaf_map_get_missing.hako" \
  "c_pair_source=benchmarks/c/bench_kilo_leaf_map_get_missing.c" \
  "c_pair_semantic_cost_mismatch_visible=1" \
  "ny_loop_call_symbol=nyash.runtime_data.get_hh" \
  "asm_top_symbol_0=nyash_rust::boxes::map_box::MapBox::get_opt_key_str" \
  "asm_top_symbol_0_percent=58.32" \
  "asm_top_symbol_1=<i64 as alloc::string::SpecToString>::spec_to_string" \
  "asm_top_symbol_1_percent=41.67" \
  "map_key_source=i64_const_zero" \
  "map_key_runtime_conversion=i64_to_string" \
  "map_storage_key_type=String" \
  "map_storage_shape=Arc<RwLock<HashMap<String, Box<dyn NyashBox>>>>" \
  "map_visible_get_missing_allocates_string_error=1" \
  "map_lookup_fusion_route_seam_exists=1" \
  "map_lookup_fusion_existing_scope=same_receiver_same_i64_key_get_has_pair" \
  "current_front_has_map_get=1" \
  "current_front_has_map_has=0" \
  "current_front_matches_existing_fusion_scope=0" \
  "selected_owner=missing_empty_map_route_trigger_probe" \
  "selected_owner_confidence=medium" \
  "selected_next=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-TRIGGER-PROBE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for stop_line in \
  "do not patch MapBox before missing-empty-map route trigger probe" \
  "do not add key-specific special case for literal 0" \
  "do not change MapBox public key semantics" \
  "do not replace String-key storage in this row" \
  "do not treat the C pair as a cost-equivalent HashMap/RwLock/string-conversion implementation" \
  "do not infer keeper status from nyash.map.slot_load_hh or MapBox::get_opt_key_str alone"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[map-missing-key-owner-inventory] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q 'data: Arc<RwLock<HashMap<String, Box<dyn NyashBox>>>>' "$MAP_BOX" || {
  echo "[map-missing-key-owner-inventory] MapBox storage shape drifted" >&2
  exit 1
}
grep -F -q 'key.to_string_box().value' "$MAP_BOX" || {
  echo "[map-missing-key-owner-inventory] MapBox key conversion evidence missing" >&2
  exit 1
}
grep -F -q 'pub fn get_opt_key_str(&self, key: &str)' "$MAP_BOX" || {
  echo "[map-missing-key-owner-inventory] MapBox raw lookup helper missing" >&2
  exit 1
}

grep -F -q 'MapLookupFusionRoute' "$MAP_FUSION_PLAN" || {
  echo "[map-missing-key-owner-inventory] MapLookupFusionRoute missing" >&2
  exit 1
}
grep -F -q 'MapLookupSameKey' "$MAP_FUSION_PLAN" || {
  echo "[map-missing-key-owner-inventory] same-key fusion scope missing" >&2
  exit 1
}
grep -F -q 'is_scalar_map_get_route' "$MAP_FUSION_PLAN" || {
  echo "[map-missing-key-owner-inventory] MapGet route predicate missing" >&2
  exit 1
}
grep -F -q 'is_i64_map_has_route' "$MAP_FUSION_PLAN" || {
  echo "[map-missing-key-owner-inventory] MapHas route predicate missing" >&2
  exit 1
}

grep -F -q 'MAP_LOOKUP_CONST_FOLD_ROUTE = "map_lookup_const_fold"' "$MAP_LOWERER" || {
  echo "[map-missing-key-owner-inventory] backend map lookup route constant missing" >&2
  exit 1
}
grep -F -q '_current_map_lookup_fusion_decision' "$MAP_LOWERER" || {
  echo "[map-missing-key-owner-inventory] backend map lookup decision hook missing" >&2
  exit 1
}
grep -F -q 'nyash.map.slot_load_hh' "$MAP_LOWERER" || {
  echo "[map-missing-key-owner-inventory] runtime map slot load fallback missing" >&2
  exit 1
}
grep -F -q 'map_lookup_fusion_routes' "$MAP_METADATA_LOADER" || {
  echo "[map-missing-key-owner-inventory] map lookup metadata loader missing" >&2
  exit 1
}

echo "[map-missing-key-owner-inventory] ok"
