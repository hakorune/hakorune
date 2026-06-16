#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-scalar-load-i64-borrowed-lookup-design"
CARD="docs/development/current/main/phases/phase-296x/296x-868-MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-867-MIMALLOC-MAP-SCALAR-LOAD-I64-KEY-STORAGE-OWNER-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_scalar_load_i64_borrowed_lookup_design_guard.sh"

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
  "output_contract=hako-mimalloc-map-scalar-load-i64-borrowed-lookup-design-v0" \
  "source_evidence=296x-867" \
  "row_kind=design" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "selected_shape=scalar_helper_borrowed_lookup" \
  "selected_helper=nyash.map.scalar_load_hi" \
  "selected_owner=MapBox_no_publication_scalar_read" \
  "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-IMPLEMENTATION-001" \
  "new_mapbox_helper=get_scalar_i64_key_str" \
  "new_key_text_helper=map_key_text_from_i64" \
  "helper_scope=scalar_load_hi_internal_only" \
  "scalar_helper_publication_enabled=0" \
  "visible_read_clone_on_scalar_path=0" \
  "i64_key_heap_string_on_scalar_path=0" \
  "mapbox_storage_change_enabled=0" \
  "i64_sidecar_storage_enabled=0" \
  "slot_load_hi_changed=0" \
  "slot_load_hh_changed=0" \
  "mapbox_public_get_contract_changed=0" \
  "mapbox_public_set_contract_changed=0" \
  "runtime_data_get_route_change_enabled=0" \
  "product_default_changed=0" \
  "winner_claim=0" \
  "implementation_started=0" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-DESIGN-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to this design" >&2
  exit 1
}

for forbidden in \
  "i64 sidecar" \
  "public key alias"; do
  grep -F -q "$forbidden" "$CARD" || {
    echo "[$TAG] design must discuss stop-line: $forbidden" >&2
    exit 1
  }
done

echo "[$TAG] ok"
