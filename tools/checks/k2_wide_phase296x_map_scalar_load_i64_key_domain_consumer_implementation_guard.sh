#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-scalar-load-i64-key-domain-consumer-implementation"
CARD="docs/development/current/main/phases/phase-296x/296x-882-MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-881-MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_scalar_load_i64_key_domain_consumer_implementation_guard.sh"
MAP_BOX="src/boxes/map_box.rs"
MAP_SLOT_LOAD="crates/nyash_kernel/src/plugin/map_slot_load.rs"
MAP_ALIASES="crates/nyash_kernel/src/plugin/map_aliases.rs"
MAP_KEY_CODEC="crates/nyash_kernel/src/plugin/map_key_codec.rs"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$MAP_BOX" "$MAP_SLOT_LOAD" "$MAP_ALIASES" "$MAP_KEY_CODEC"; do
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
  "output_contract=hako-mimalloc-map-scalar-load-i64-key-domain-consumer-implementation-v0" \
  "source_evidence=296x-881" \
  "row_kind=implementation" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "implemented_raw_helper=MapBox::get_scalar_i64_key_i64" \
  "implemented_kernel_helper=map_scalar_load_i64" \
  "scalar_load_hi_uses_map_scalar_load_i64=1" \
  "scalar_load_hi_uses_map_key_text_from_i64=0" \
  "scalar_load_hi_uses_map_key_string_from_i64=0" \
  "map_key_text_from_i64_removed=1" \
  "i64_key_text_struct_removed=1" \
  "slot_load_hi_unchanged=1" \
  "slot_load_hh_unchanged=1" \
  "public_mapbox_semantics_changed=0" \
  "sidecar_storage_enabled=0" \
  "hashmap_hasher_swap_enabled=0" \
  "mirbuilder_changed=0" \
  "route_proof_changed=0" \
  "winner_claim=0" \
  "summary=ok" \
  "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-MEASUREMENT-001"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-IMPLEMENTATION-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to implementation" >&2
  exit 1
}

grep -F -q "pub fn get_scalar_i64_key_i64(&self, key: i64) -> Option<i64>" "$MAP_BOX" || {
  echo "[$TAG] MapBox raw i64-domain helper missing" >&2
  exit 1
}

grep -F -q "MapKeyDomain::from_i64(key)" "$MAP_BOX" || {
  echo "[$TAG] MapBox raw i64-domain helper must use MapKeyDomain::from_i64" >&2
  exit 1
}

grep -F -q "pub(super) fn map_scalar_load_i64(handle: i64, key_i64: i64) -> i64" "$MAP_SLOT_LOAD" || {
  echo "[$TAG] kernel scalar i64 helper missing" >&2
  exit 1
}

grep -F -q "map.get_scalar_i64_key_i64(key_i64).unwrap_or(0)" "$MAP_SLOT_LOAD" || {
  echo "[$TAG] kernel scalar helper must call MapBox i64-domain helper" >&2
  exit 1
}

grep -F -q "map_scalar_load_i64(handle, key_i64)" "$MAP_ALIASES" || {
  echo "[$TAG] scalar_load_hi must call map_scalar_load_i64" >&2
  exit 1
}

if grep -F -q "map_key_text_from_i64" "$MAP_ALIASES" "$MAP_KEY_CODEC"; then
  echo "[$TAG] obsolete map_key_text_from_i64 must be removed from scalar helper path" >&2
  exit 1
fi

if grep -F -q "I64KeyText" "$MAP_KEY_CODEC"; then
  echo "[$TAG] obsolete I64KeyText middle layer must be removed" >&2
  exit 1
fi

for proof in \
  "cargo fmt --check" \
  "cargo check --release --bin hakorune" \
  "cargo test --lib test_key_domain_i64_text_alias -- --nocapture"; do
  grep -F -q "$proof" "$CARD" || {
    echo "[$TAG] missing proof command: $proof" >&2
    exit 1
  }
done

echo "[$TAG] ok"
