#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-key-domain-vocabulary"
CARD="docs/development/current/main/phases/phase-296x/296x-874-MIMALLOC-MAP-KEY-DOMAIN-VOCABULARY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-873-MIMALLOC-MAP-KEY-DOMAIN-ALIAS-PLAN-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_key_domain_vocabulary_guard.sh"
MODULE="crates/nyash_kernel/src/plugin/map_key_domain.rs"
PLUGIN_MOD="crates/nyash_kernel/src/plugin/mod.rs"
MAP_BOX="src/boxes/map_box.rs"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$MODULE" "$PLUGIN_MOD" "$MAP_BOX"; do
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
  "output_contract=hako-mimalloc-map-key-domain-vocabulary-v0" \
  "source_evidence=296x-873" \
  "row_kind=implementation" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "implemented_shape=MapKeyDomain_vocabulary" \
  "implemented_module=crates/nyash_kernel/src/plugin/map_key_domain.rs" \
  "implemented_variants=CanonicalI64,Text" \
  "implemented_from_i64=1" \
  "implemented_from_text=1" \
  "implemented_public_text=1" \
  "canonical_i64_text_alias_tested=1" \
  "noncanonical_numeric_text_reject_tested=1" \
  "public_text_roundtrip_tested=1" \
  "mapbox_storage_change_enabled=0" \
  "mapbox_storage_consumes_map_key_domain=0" \
  "scalar_load_hi_consumes_map_key_domain=0" \
  "i64_sidecar_storage_enabled=0" \
  "hashmap_hasher_swap_enabled=0" \
  "public_semantics_change_enabled=0" \
  "winner_claim=0" \
  "selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-DESIGN-001" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-KEY-DOMAIN-VOCABULARY-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to vocabulary" >&2
  exit 1
}

grep -F -q "mod map_key_domain;" "$PLUGIN_MOD" || {
  echo "[$TAG] plugin module missing map_key_domain" >&2
  exit 1
}

for expected in \
  "pub(crate) enum MapKeyDomain" \
  "CanonicalI64(i64)" \
  "Text(String)" \
  "pub(crate) fn from_i64(value: i64) -> Self" \
  "pub(crate) fn from_text(text: &str) -> Self" \
  "pub(crate) fn public_text(&self) -> String" \
  "parse_canonical_i64_text" \
  "i64_and_canonical_text_share_domain" \
  "noncanonical_numeric_text_stays_text" \
  "canonical_text_keeps_expected_alias_examples"; do
  grep -F -q "$expected" "$MODULE" || {
    echo "[$TAG] missing module text: $expected" >&2
    exit 1
  }
done

grep -F -q "HashMap<String, Box<dyn NyashBox>>" "$MAP_BOX" || {
  echo "[$TAG] MapBox storage changed in vocabulary row" >&2
  exit 1
}

if rg -n "MapKeyDomain" src/boxes crates/nyash_kernel/src/plugin \
  | grep -v "map_key_domain.rs" \
  | grep -v "mod map_key_domain" \
  | grep -v "$CARD" >/dev/null; then
  echo "[$TAG] MapKeyDomain consumed outside vocabulary module" >&2
  exit 1
fi

echo "[$TAG] ok"
