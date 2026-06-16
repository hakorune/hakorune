#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-storage-policy-ssot"
CARD="docs/development/current/main/phases/phase-296x/296x-886-MAP-STORAGE-POLICY-SSOT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-885-MIMALLOC-MAP-GET-SCALAR-KEYDOMAIN-CLOSEOUT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_storage_policy_ssot_guard.sh"

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
  "output_contract=hako-map-storage-policy-ssot-v0" \
  "source_evidence=296x-885" \
  "row_kind=design_ssot" \
  "product_map_storage_owner=MapBox" \
  "product_map_storage_shape=HashMap<MapKeyDomain, Box<dyn NyashBox>>" \
  "product_map_hasher_policy=std_default" \
  "product_map_public_semantics_owner=MapBox" \
  "local_map_storage_plan_reserved=1" \
  "local_i64_key_map_reserved=1" \
  "local_text_key_map_reserved=1" \
  "local_canonical_map_reserved=1" \
  "published_mapbox_fallback_reserved=1" \
  "hasher_swap_enabled=0" \
  "typed_i64_product_map_enabled=0" \
  "i64_sidecar_storage_enabled=0" \
  "map_storage_substrate_implementation_enabled=0" \
  "mirbuilder_map_storage_owner_enabled=0" \
  "route_proof_changed=0" \
  "winner_claim=0" \
  "next_task=MAP-HASH-OWNER-INVENTORY-001" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "next_task=MAP-STORAGE-POLICY-SSOT-001" "$PREV_CARD" || {
  echo "[$TAG] closeout does not hand off to MapStoragePlan SSOT" >&2
  exit 1
}

for text in \
  'ProductMapStorage:' \
  'MapStoragePlan:' \
  'GenericCanonicalMap' \
  'LocalI64KeyMap' \
  'LocalTextKeyMap' \
  'LocalScalarValueMap' \
  'PublishedMapBoxFallback' \
  'Do not make product `MapBox` i64-only.' \
  'no product `MapBox` hasher swap' \
  'no map storage decision in MIRBuilder'; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing policy text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
