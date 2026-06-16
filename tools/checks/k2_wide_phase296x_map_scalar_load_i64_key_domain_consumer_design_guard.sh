#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-scalar-load-i64-key-domain-consumer-design"
CARD="docs/development/current/main/phases/phase-296x/296x-881-MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-880-MIMALLOC-MAP-KEY-DOMAIN-STORAGE-MEASUREMENT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_scalar_load_i64_key_domain_consumer_design_guard.sh"

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
  "output_contract=hako-mimalloc-map-scalar-load-i64-key-domain-consumer-design-v0" \
  "source_evidence=296x-880" \
  "row_kind=design" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "selected_owner=scalar_helper_key_domain_consumer" \
  "selected_shape=scalar_load_hi_uses_i64_domain_helper" \
  "new_raw_helper=MapBox::get_scalar_i64_key_i64" \
  "new_kernel_helper=map_scalar_load_i64" \
  "scalar_load_hi_uses_map_key_text_from_i64=0" \
  "scalar_load_hi_uses_map_key_string_from_i64=0" \
  "slot_load_hi_unchanged=1" \
  "slot_load_hh_unchanged=1" \
  "public_mapbox_semantics_changed=0" \
  "sidecar_storage_enabled=0" \
  "hashmap_hasher_swap_enabled=0" \
  "mirbuilder_changed=0" \
  "route_proof_changed=0" \
  "winner_claim=0" \
  "summary=ok" \
  "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-IMPLEMENTATION-001"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-DOMAIN-CONSUMER-DESIGN-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to design" >&2
  exit 1
}

for text in \
  "nyash.map.scalar_load_hi(handle, key_i64)" \
  "MapKeyDomain::from_i64(key_i64)" \
  "slot_load_hi:" \
  "slot_load_hh:" \
  "scalar_load_hi:" \
  "no hasher swap"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing design text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
