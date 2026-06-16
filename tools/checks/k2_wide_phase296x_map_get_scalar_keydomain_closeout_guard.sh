#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-scalar-keydomain-closeout"
CARD="docs/development/current/main/phases/phase-296x/296x-885-MIMALLOC-MAP-GET-SCALAR-KEYDOMAIN-CLOSEOUT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-884-MIMALLOC-MAP-SCALAR-LOAD-I64-POST-DOMAIN-CONSUMER-OWNER-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_scalar_keydomain_closeout_guard.sh"

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
  "output_contract=hako-map-get-scalar-keydomain-closeout-v0" \
  "source_evidence=296x-884" \
  "row_kind=closeout" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "map_key_domain_storage_enabled=1" \
  "public_semantics_preserved=1" \
  "int_string_alias_preserved=1" \
  "text_noncanonical_key_separate=1" \
  "keys_values_json_public_text=1" \
  "spec_to_string_hot_path_removed=1" \
  "scalar_load_hi_pct_before=31.14" \
  "scalar_load_hi_pct_after=3.30" \
  "spec_to_string_pct_before=23.81" \
  "spec_to_string_pct_after=0" \
  "cycles_before=1180142598" \
  "cycles_after=484694805" \
  "keeper_claim=1" \
  "remaining_hot_owner=map_hash_lookup_boundary" \
  "remaining_owner_requires_storage_policy=1" \
  "implementation_allowed=0" \
  "next_task=MAP-STORAGE-POLICY-SSOT-001" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=DESIGN-CONSULT-MAP-KEY-DOMAIN-HASH-LOOKUP-POLICY-001" "$PREV_CARD" || {
  echo "[$TAG] previous row is not the design-consult owner selection" >&2
  exit 1
}

for text in \
  "hasher_swap_enabled=0" \
  "typed_i64_product_map_enabled=0" \
  "i64_sidecar_storage_enabled=0" \
  "map_storage_substrate_implementation_enabled=0" \
  "mirbuilder_map_storage_owner_enabled=0" \
  "MAP-STORAGE-POLICY-SSOT-001"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing closeout text: $text" >&2
    exit 1
  }
done

echo "[$TAG] ok"
