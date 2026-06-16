#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-scalar-load-i64-borrowed-lookup-implementation"
CARD="docs/development/current/main/phases/phase-296x/296x-869-MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-868-MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_scalar_load_i64_borrowed_lookup_implementation_guard.sh"
MAP_BOX="src/boxes/map_box.rs"
MAP_ALIASES="crates/nyash_kernel/src/plugin/map_aliases.rs"
MAP_KEY_CODEC="crates/nyash_kernel/src/plugin/map_key_codec.rs"
MAP_SLOT_LOAD="crates/nyash_kernel/src/plugin/map_slot_load.rs"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$MAP_BOX" "$MAP_ALIASES" "$MAP_KEY_CODEC" "$MAP_SLOT_LOAD"; do
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
  "output_contract=hako-mimalloc-map-scalar-load-i64-borrowed-lookup-implementation-v0" \
  "source_evidence=296x-868" \
  "row_kind=implementation" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "implemented_shape=scalar_helper_borrowed_lookup" \
  "implemented_helper=nyash.map.scalar_load_hi" \
  "implemented_key_text_helper=map_key_text_from_i64" \
  "implemented_mapbox_helper=get_scalar_i64_key_str" \
  "scalar_load_hi_uses_key_text=1" \
  "scalar_load_hi_uses_borrowed_scalar_read=1" \
  "scalar_load_hi_uses_map_key_string_from_i64=0" \
  "scalar_load_hi_uses_visible_read_clone=0" \
  "slot_load_hi_changed=0" \
  "slot_load_hh_changed=0" \
  "map_key_string_from_i64_kept=1" \
  "mapbox_get_opt_key_str_kept=1" \
  "mapbox_clone_for_visible_read_kept=1" \
  "mapbox_storage_change_enabled=0" \
  "i64_sidecar_storage_enabled=0" \
  "mapbox_public_get_contract_changed=0" \
  "mapbox_public_set_contract_changed=0" \
  "runtime_data_get_route_change_enabled=0" \
  "product_default_changed=0" \
  "winner_claim=0" \
  "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-VALIDATION-001" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-IMPLEMENTATION-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to this implementation" >&2
  exit 1
}

grep -F -q "pub(crate) fn map_key_text_from_i64(key_i64: i64) -> I64KeyText" "$MAP_KEY_CODEC" || {
  echo "[$TAG] missing no-allocation i64 key text helper" >&2
  exit 1
}

grep -F -q "pub(crate) fn map_key_string_from_i64(key_i64: i64) -> String" "$MAP_KEY_CODEC" || {
  echo "[$TAG] public/materialized i64 key String helper was removed" >&2
  exit 1
}

grep -F -q "pub fn get_scalar_i64_key_str(&self, key: &str) -> Option<i64>" "$MAP_BOX" || {
  echo "[$TAG] missing MapBox borrowed scalar helper" >&2
  exit 1
}

grep -F -q "fn clone_for_visible_read(value: &dyn NyashBox)" "$MAP_BOX" || {
  echo "[$TAG] visible read clone seam removed" >&2
  exit 1
}

grep -F -q "pub fn get_opt_key_str(&self, key: &str) -> Option<Box<dyn NyashBox>>" "$MAP_BOX" || {
  echo "[$TAG] public/raw visible get seam removed" >&2
  exit 1
}

grep -F -q "HashMap<String, Box<dyn NyashBox>>" "$MAP_BOX" || {
  echo "[$TAG] MapBox storage changed in scalar helper row" >&2
  exit 1
}

grep -F -q "pub(super) fn map_scalar_load_i64_str(handle: i64, key_str: &str) -> i64" "$MAP_SLOT_LOAD" || {
  echo "[$TAG] missing borrowed scalar slot-load helper" >&2
  exit 1
}

grep -F -q "map.get_scalar_i64_key_str(key_str).unwrap_or(0)" "$MAP_SLOT_LOAD" || {
  echo "[$TAG] borrowed scalar slot-load helper must not use visible get" >&2
  exit 1
}

grep -F -q "let key_text = map_key_text_from_i64(key_i64);" "$MAP_ALIASES" || {
  echo "[$TAG] scalar_load_hi must use map_key_text_from_i64" >&2
  exit 1
}

grep -F -q "map_scalar_load_i64_str(handle, key_text.as_str())" "$MAP_ALIASES" || {
  echo "[$TAG] scalar_load_hi must use borrowed scalar helper" >&2
  exit 1
}

if awk '/nyash_map_scalar_load_hi_alias/,/^}/ { print }' "$MAP_ALIASES" | grep -F -q "map_key_string_from_i64"; then
  echo "[$TAG] scalar_load_hi still uses allocating map_key_string_from_i64" >&2
  exit 1
fi

if awk '/nyash_map_scalar_load_hi_alias/,/^}/ { print }' "$MAP_ALIASES" | grep -F -q "map_slot_load_str"; then
  echo "[$TAG] scalar_load_hi still uses visible slot-load seam" >&2
  exit 1
fi

echo "[$TAG] ok"
