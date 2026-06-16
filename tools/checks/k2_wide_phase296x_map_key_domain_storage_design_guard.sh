#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-key-domain-storage-design"
CARD="docs/development/current/main/phases/phase-296x/296x-875-MIMALLOC-MAP-KEY-DOMAIN-STORAGE-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-874-MIMALLOC-MAP-KEY-DOMAIN-VOCABULARY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_key_domain_storage_design_guard.sh"
ROOT_CARGO="Cargo.toml"
KERNEL_CARGO="crates/nyash_kernel/Cargo.toml"
MAP_BOX="src/boxes/map_box.rs"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$ROOT_CARGO" "$KERNEL_CARGO" "$MAP_BOX"; do
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
  "output_contract=hako-mimalloc-map-key-domain-storage-design-v0" \
  "source_evidence=296x-874" \
  "row_kind=design" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "storage_truth_owner=nyash_rust::boxes::MapBox" \
  "map_key_domain_final_owner=nyash_rust::boxes::map_key_domain" \
  "kernel_map_key_domain_role=prototype_only" \
  "selected_next=MIMALLOC-MAP-KEY-DOMAIN-CORE-PROMOTION-001" \
  "selected_storage_shape=HashMap<MapKeyDomain, Box<dyn NyashBox>>" \
  "selected_public_key_output=MapKeyDomain::public_text" \
  "selected_public_alias_rule=CanonicalI64_and_canonical_decimal_Text_alias" \
  "kernel_vocabulary_duplicate_allowed_temporarily=1" \
  "kernel_vocabulary_duplicate_retire_required=1" \
  "mapbox_storage_change_enabled=0" \
  "scalar_load_hi_consumes_map_key_domain=0" \
  "i64_sidecar_storage_enabled=0" \
  "hashmap_hasher_swap_enabled=0" \
  "public_semantics_change_enabled=0" \
  "implementation_started=0" \
  "winner_claim=0" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-DESIGN-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to storage design" >&2
  exit 1
}

grep -F -q 'nyash-rust = { path = "../../" }' "$KERNEL_CARGO" || {
  echo "[$TAG] expected kernel -> nyash-rust dependency not found" >&2
  exit 1
}

if grep -F -q 'nyash_kernel' "$ROOT_CARGO"; then
  echo "[$TAG] root crate must not depend on nyash_kernel for MapBox storage" >&2
  exit 1
fi

grep -F -q "HashMap<String, Box<dyn NyashBox>>" "$MAP_BOX" || {
  echo "[$TAG] MapBox storage changed before core promotion / guard surface" >&2
  exit 1
}

for text in \
  "nyash-rust cannot depend on nyash_kernel" \
  "HashMap<MapKeyDomain, Box<dyn NyashBox>>" \
  "add src/boxes/map_key_domain.rs" \
  "keep nyash_kernel prototype duplicate temporarily"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing design text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
