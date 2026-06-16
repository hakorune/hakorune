#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-scalar-load-i64-hashmap-key-domain-owner-selection"
CARD="docs/development/current/main/phases/phase-296x/296x-872-MIMALLOC-MAP-SCALAR-LOAD-I64-HASHMAP-KEY-DOMAIN-OWNER-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-871-MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-MEASUREMENT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_scalar_load_i64_hashmap_key_domain_owner_selection_guard.sh"
MAP_BOX="src/boxes/map_box.rs"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$MAP_BOX"; do
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
  "output_contract=hako-mimalloc-map-scalar-load-i64-hashmap-key-domain-owner-selection-v0" \
  "source_evidence=296x-871" \
  "row_kind=owner_selection" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "route_proof_status=closed" \
  "borrowed_scalar_lookup_status=closed" \
  "compiler_route_next_owner=none" \
  "current_remaining_hot_owner=HashMap<String>_key_hashing" \
  "current_storage_key_domain=String" \
  "current_public_key_semantics=stringified_key_namespace" \
  "current_i64_public_aliases_string_key=1" \
  "selected_owner=map_key_domain_alias_plan" \
  "selected_owner_scope=MapBox_storage_semantics_design" \
  "selected_next=MIMALLOC-MAP-KEY-DOMAIN-ALIAS-PLAN-DESIGN-001" \
  "i64_sidecar_storage_selected=0" \
  "typed_i64_map_storage_selected=0" \
  "hashmap_hasher_swap_selected=0" \
  "public_semantics_change_selected=0" \
  "implementation_started=0" \
  "winner_claim=0" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-HASHMAP-KEY-DOMAIN-OWNER-SELECTION-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to this owner selection" >&2
  exit 1
}

grep -F -q "HashMap<String, Box<dyn NyashBox>>" "$MAP_BOX" || {
  echo "[$TAG] MapBox storage changed before alias-plan design" >&2
  exit 1
}

for text in \
  "stringified-key" \
  "map.set(1, value) and map.set(\"1\", value)" \
  "do not add i64 sidecar storage"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing semantic stop-line text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
