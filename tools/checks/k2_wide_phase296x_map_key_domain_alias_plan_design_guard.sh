#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-key-domain-alias-plan-design"
CARD="docs/development/current/main/phases/phase-296x/296x-873-MIMALLOC-MAP-KEY-DOMAIN-ALIAS-PLAN-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-872-MIMALLOC-MAP-SCALAR-LOAD-I64-HASHMAP-KEY-DOMAIN-OWNER-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_key_domain_alias_plan_design_guard.sh"
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
  "output_contract=hako-mimalloc-map-key-domain-alias-plan-design-v0" \
  "source_evidence=296x-872" \
  "row_kind=design" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "selected_shape=normalized_map_key_domain" \
  "selected_owner=MapBox_key_domain" \
  "selected_next=MIMALLOC-MAP-KEY-DOMAIN-VOCABULARY-001" \
  "public_key_semantics=stringified_key_namespace" \
  "canonical_i64_text_alias_enabled=1" \
  "noncanonical_numeric_text_preserved_as_text=1" \
  "map_keys_public_text_output_required=1" \
  "map_key_domain_variants=CanonicalI64,Text" \
  "canonical_i64_accepts=0,1,-1,i64_MIN,i64_MAX" \
  "canonical_i64_rejects=leading_plus,leading_zero_except_zero,negative_zero,empty,whitespace,overflow" \
  "i64_sidecar_storage_selected=0" \
  "typed_i64_map_storage_selected=0" \
  "hashmap_hasher_swap_selected=0" \
  "public_semantics_change_selected=0" \
  "mapbox_storage_change_enabled=0" \
  "implementation_started=0" \
  "winner_claim=0" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-KEY-DOMAIN-ALIAS-PLAN-DESIGN-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to this design" >&2
  exit 1
}

grep -F -q "HashMap<String, Box<dyn NyashBox>>" "$MAP_BOX" || {
  echo "[$TAG] MapBox storage changed before vocabulary row" >&2
  exit 1
}

for text in \
  'MapKeyDomain:' \
  'CanonicalI64(i64)' \
  'Text(String)' \
  'map.set(1, value)' \
  'map.get("1")' \
  '"01"  -> Text("01")' \
  'do not change `keys()`'; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing design text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
