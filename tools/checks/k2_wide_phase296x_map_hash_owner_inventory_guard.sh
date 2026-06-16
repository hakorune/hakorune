#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-hash-owner-inventory"
CARD="docs/development/current/main/phases/phase-296x/296x-887-MAP-HASH-OWNER-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-886-MAP-STORAGE-POLICY-SSOT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_hash_owner_inventory_guard.sh"

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
  "output_contract=hako-map-hash-owner-inventory-v0" \
  "source_evidence=296x-886" \
  "row_kind=inventory" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "remaining_hot_owner=map_hash_lookup_boundary" \
  "mapbox_get_scalar_i64_key_domain_pct=64.18" \
  "build_hasher_hash_one_pct=31.43" \
  "canonical_i64_hot_lookup_visible=1" \
  "text_key_hot_lookup_visible=0" \
  "mixed_key_hot_lookup_visible=0" \
  "product_hasher_policy=std_default" \
  "product_hasher_swap_allowed=0" \
  "product_mapbox_i64_only_allowed=0" \
  "sidecar_storage_allowed=0" \
  "implementation_allowed=0" \
  "selected_next=LOCAL-I64-MAP-FRONT-SELECTION-001" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "next_task=MAP-HASH-OWNER-INVENTORY-001" "$PREV_CARD" || {
  echo "[$TAG] Map storage policy SSOT does not hand off to hash inventory" >&2
  exit 1
}

for text in \
  "Do not optimize the product hasher directly from this evidence." \
  "LocalI64KeyMap candidate?" \
  "no product hasher swap" \
  "no product MapBox i64-only storage" \
  "no sidecar storage" \
  "no implementation from inventory evidence"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing inventory text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
