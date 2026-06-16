#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-key-domain-storage-implementation"
CARD="docs/development/current/main/phases/phase-296x/296x-878-MIMALLOC-MAP-KEY-DOMAIN-STORAGE-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-877-MIMALLOC-MAP-KEY-DOMAIN-STORAGE-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_key_domain_storage_implementation_guard.sh"
MAP_BOX="src/boxes/map_box.rs"
JSON_BOX="src/boxes/json/mod.rs"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$MAP_BOX" "$JSON_BOX"; do
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
  "output_contract=hako-mimalloc-map-key-domain-storage-implementation-v0" \
  "source_evidence=296x-877" \
  "row_kind=implementation" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "implemented_storage_shape=HashMap<MapKeyDomain, Box<dyn NyashBox>>" \
  "mapbox_set_normalizes_key_domain=1" \
  "mapbox_get_normalizes_key_domain=1" \
  "mapbox_has_normalizes_key_domain=1" \
  "mapbox_delete_normalizes_key_domain=1" \
  "mapbox_keys_uses_public_text=1" \
  "mapbox_values_order_uses_public_text_sort=1" \
  "mapbox_json_uses_public_text=1" \
  "i64_text_alias_test_present=1" \
  "noncanonical_text_preservation_test_present=1" \
  "public_keys_text_output_test_present=1" \
  "scalar_load_hi_consumes_map_key_domain=0" \
  "slot_load_hi_consumes_map_key_domain=0" \
  "slot_load_hh_consumes_map_key_domain=0" \
  "kernel_scalar_helper_route_changed=0" \
  "i64_sidecar_storage_enabled=0" \
  "hashmap_hasher_swap_enabled=0" \
  "product_default_changed=0" \
  "winner_claim=0" \
  "summary=ok" \
  "selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-VALIDATION-001"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-IMPLEMENTATION-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to implementation" >&2
  exit 1
}

grep -F -q "use crate::boxes::map_key_domain::MapKeyDomain;" "$MAP_BOX" || {
  echo "[$TAG] MapBox must import MapKeyDomain" >&2
  exit 1
}

grep -F -q "HashMap<MapKeyDomain, Box<dyn NyashBox>>" "$MAP_BOX" || {
  echo "[$TAG] MapBox storage must use MapKeyDomain" >&2
  exit 1
}

from_text_count="$(grep -F "MapKeyDomain::from_text" "$MAP_BOX" | wc -l | tr -d ' ')"
if [[ "$from_text_count" -lt 5 ]]; then
  echo "[$TAG] MapBox must normalize public/raw key entry points through MapKeyDomain::from_text" >&2
  exit 1
fi

public_text_count="$(( $(grep -F ".public_text()" "$MAP_BOX" | wc -l | tr -d ' ') + $(grep -F "MapKeyDomain::public_text" "$MAP_BOX" | wc -l | tr -d ' ') ))"
if [[ "$public_text_count" -lt 3 ]]; then
  echo "[$TAG] MapBox must use public_text for keys/values/json/debug public output" >&2
  exit 1
fi

grep -F -q "key.public_text()" "$JSON_BOX" || {
  echo "[$TAG] JSON conversion must emit public_text keys" >&2
  exit 1
}

for test_name in \
  "test_key_domain_i64_text_alias" \
  "test_key_domain_noncanonical_text_preserved" \
  "test_keys_public_text_after_key_domain_storage"; do
  grep -F -q "fn $test_name" "$MAP_BOX" || {
    echo "[$TAG] missing semantic test: $test_name" >&2
    exit 1
  }
done

for forbidden in \
  "HashMap<String, Box<dyn NyashBox>>" \
  "i64_sidecar" \
  "BuildHasherDefault"; do
  if grep -F -q "$forbidden" "$MAP_BOX"; then
    echo "[$TAG] forbidden storage/helper text in MapBox: $forbidden" >&2
    exit 1
  fi
done

echo "[$TAG] ok"
