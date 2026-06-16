#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-scalar-load-i64-key-storage-owner-selection"
CARD="docs/development/current/main/phases/phase-296x/296x-867-MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-STORAGE-OWNER-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-866-MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-MEASUREMENT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_scalar_load_i64_key_storage_owner_selection_guard.sh"
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
  "output_contract=hako-mimalloc-map-scalar-load-i64-key-storage-owner-selection-v0" \
  "source_evidence=296x-866" \
  "row_kind=owner_selection" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "route_proof_status=closed" \
  "route_proof_next_owner=none" \
  "compiler_route_next_owner=none" \
  "current_loop_helper=nyash.map.scalar_load_hi" \
  "current_i64_key_codec=map_key_string_from_i64" \
  "current_i64_key_codec_allocates_string=1" \
  "current_scalar_load_uses_visible_read_clone=1" \
  "current_map_storage_key_domain=String" \
  "current_map_storage_value_domain=Box<dyn NyashBox>" \
  "map_public_semantics_key_domain=stringified_key" \
  "i64_string_key_alias_semantics_preserved=1" \
  "selected_owner=map_scalar_no_publication_borrowed_lookup" \
  "selected_owner_scope=scalar_load_hi_internal" \
  "selected_first_slice=borrowed_scalar_read_plus_i64_key_text" \
  "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-DESIGN-001" \
  "no_publication_scalar_read_owner=MapBox" \
  "no_publication_scalar_read_may_avoid_visible_clone=1" \
  "i64_key_text_may_avoid_heap_string=1" \
  "hashmap_string_hash_owner_remains=1" \
  "i64_sidecar_storage_selected=0" \
  "mapbox_storage_change_enabled=0" \
  "mapbox_public_get_contract_changed=0" \
  "mapbox_public_set_contract_changed=0" \
  "runtime_data_get_route_change_enabled=0" \
  "product_default_changed=0" \
  "winner_claim=0" \
  "implementation_started=0" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-STORAGE-OWNER-SELECTION-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to this owner selection" >&2
  exit 1
}

grep -F -q "pub(crate) fn map_key_string_from_i64(key_i64: i64) -> String" "$MAP_KEY_CODEC" || {
  echo "[$TAG] i64 key codec signature changed; update owner selection first" >&2
  exit 1
}

grep -F -q "key_i64.to_string()" "$MAP_KEY_CODEC" || {
  echo "[$TAG] owner selection assumes current i64 key codec allocates String" >&2
  exit 1
}

grep -F -q "map_key_string_from_i64(key_i64)" "$MAP_ALIASES" || {
  echo "[$TAG] scalar load helper no longer uses map_key_string_from_i64" >&2
  exit 1
}

grep -F -q "map_slot_load_str_with(handle, &key_str" "$MAP_ALIASES" || {
  echo "[$TAG] scalar load helper no longer uses visible slot-load seam" >&2
  exit 1
}

grep -F -q "map.get_opt_key_str(key_str).map(f).unwrap_or(0)" "$MAP_SLOT_LOAD" || {
  echo "[$TAG] slot-load seam no longer uses MapBox::get_opt_key_str" >&2
  exit 1
}

grep -F -q "HashMap<String, Box<dyn NyashBox>>" "$MAP_BOX" || {
  echo "[$TAG] MapBox storage key/value domain changed without this row owning it" >&2
  exit 1
}

grep -F -q "fn clone_for_visible_read(value: &dyn NyashBox)" "$MAP_BOX" || {
  echo "[$TAG] visible-read clone seam missing" >&2
  exit 1
}

echo "[$TAG] ok"
