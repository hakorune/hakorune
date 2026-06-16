#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-key-domain-storage-guard-surface"
CARD="docs/development/current/main/phases/phase-296x/296x-877-MIMALLOC-MAP-KEY-DOMAIN-STORAGE-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-876-MIMALLOC-MAP-KEY-DOMAIN-CORE-PROMOTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_key_domain_storage_guard_surface_guard.sh"
CORE_MODULE="src/boxes/map_key_domain.rs"
MAP_BOX="src/boxes/map_box.rs"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$CORE_MODULE" "$MAP_BOX"; do
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
  "output_contract=hako-mimalloc-map-key-domain-storage-guard-surface-v0" \
  "source_evidence=296x-876" \
  "row_kind=guard_surface" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-IMPLEMENTATION-001" \
  "post_storage_shape=HashMap<MapKeyDomain, Box<dyn NyashBox>>" \
  "post_mapbox_set_normalizes_key_domain=1" \
  "post_mapbox_get_normalizes_key_domain=1" \
  "post_mapbox_has_normalizes_key_domain=1" \
  "post_mapbox_delete_normalizes_key_domain=1" \
  "post_mapbox_keys_uses_public_text=1" \
  "post_mapbox_values_order_uses_public_text_sort=1" \
  "post_i64_text_alias_test_required=1" \
  "post_noncanonical_text_preservation_test_required=1" \
  "post_public_keys_text_output_test_required=1" \
  "scalar_load_hi_consumes_map_key_domain=0" \
  "slot_load_hi_consumes_map_key_domain=0" \
  "slot_load_hh_consumes_map_key_domain=0" \
  "i64_sidecar_storage_enabled=0" \
  "hashmap_hasher_swap_enabled=0" \
  "product_default_changed=0" \
  "winner_claim=0" \
  "implementation_started=0" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-GUARD-SURFACE-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to storage guard surface" >&2
  exit 1
}

grep -F -q "pub enum MapKeyDomain" "$CORE_MODULE" || {
  echo "[$TAG] core MapKeyDomain missing" >&2
  exit 1
}

grep -F -q "HashMap<String, Box<dyn NyashBox>>" "$MAP_BOX" || {
  echo "[$TAG] guard-surface row must not already change MapBox storage" >&2
  exit 1
}

for text in \
  "map.set(1, value); map.get(\"1\") == value" \
  "map.set(\"01\", value); map.get(1) != value" \
  "map.keys() returns public text keys" \
  "do not connect scalar helper before MapBox public semantics tests exist"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing guard text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
