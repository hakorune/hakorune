#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-key-domain-core-promotion"
CARD="docs/development/current/main/phases/phase-296x/296x-876-MIMALLOC-MAP-KEY-DOMAIN-CORE-PROMOTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-875-MIMALLOC-MAP-KEY-DOMAIN-STORAGE-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_key_domain_core_promotion_guard.sh"
CORE_MODULE="src/boxes/map_key_domain.rs"
BOXES_MOD="src/boxes/mod.rs"
KERNEL_MODULE="crates/nyash_kernel/src/plugin/map_key_domain.rs"
MAP_BOX="src/boxes/map_box.rs"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$CORE_MODULE" "$BOXES_MOD" "$KERNEL_MODULE" "$MAP_BOX"; do
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
  "output_contract=hako-mimalloc-map-key-domain-core-promotion-v0" \
  "source_evidence=296x-875" \
  "row_kind=implementation" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "implemented_shape=core_map_key_domain_vocabulary" \
  "implemented_module=src/boxes/map_key_domain.rs" \
  "implemented_export=src/boxes/mod.rs" \
  "implemented_variants=CanonicalI64,Text" \
  "implemented_from_i64=1" \
  "implemented_from_text=1" \
  "implemented_public_text=1" \
  "core_alias_tests_green=1" \
  "kernel_duplicate_kept_temporarily=1" \
  "kernel_duplicate_consumed_by_mapbox=0" \
  "mapbox_storage_change_enabled=0" \
  "mapbox_storage_consumes_map_key_domain=0" \
  "scalar_load_hi_consumes_map_key_domain=0" \
  "i64_sidecar_storage_enabled=0" \
  "hashmap_hasher_swap_enabled=0" \
  "public_semantics_change_enabled=0" \
  "winner_claim=0" \
  "selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-GUARD-SURFACE-001" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-KEY-DOMAIN-CORE-PROMOTION-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to core promotion" >&2
  exit 1
}

grep -F -q "pub mod map_key_domain;" "$BOXES_MOD" || {
  echo "[$TAG] boxes module does not export map_key_domain" >&2
  exit 1
}

for expected in \
  "pub enum MapKeyDomain" \
  "CanonicalI64(i64)" \
  "Text(String)" \
  "pub fn from_i64(value: i64) -> Self" \
  "pub fn from_text(text: &str) -> Self" \
  "pub fn public_text(&self) -> String" \
  "i64_and_canonical_text_share_domain" \
  "noncanonical_numeric_text_stays_text"; do
  grep -F -q "$expected" "$CORE_MODULE" || {
    echo "[$TAG] missing core module text: $expected" >&2
    exit 1
  }
done

grep -F -q "HashMap<String, Box<dyn NyashBox>>" "$MAP_BOX" || {
  echo "[$TAG] MapBox storage changed during core promotion" >&2
  exit 1
}

if rg -n "MapKeyDomain" src/boxes/map_box.rs crates/nyash_kernel/src/plugin/map_aliases.rs crates/nyash_kernel/src/plugin/map_slot_load.rs >/dev/null; then
  echo "[$TAG] MapKeyDomain consumed before storage guard surface" >&2
  exit 1
fi

echo "[$TAG] ok"
