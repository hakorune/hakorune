#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-i64-scalar-route-design"
CARD="docs/development/current/main/phases/phase-296x/296x-854-MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_i64_scalar_route_design_guard.sh"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-853-MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-INVENTORY-001.md"

for file in "$CARD" "$INDEX" "$PREV_CARD"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

grep -q '^Status: Landed$' "$CARD" || {
  echo "[$TAG] card must be Landed" >&2
  exit 1
}

grep -q "$SELF_SCRIPT" "$INDEX" || {
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
  "output_contract=hako-mimalloc-map-get-i64-scalar-route-design-v0" \
  "source_evidence=296x-853" \
  "row_kind=design" \
  "implementation_started=0" \
  "selected_route_kind=MapLoadScalarI64" \
  "selected_route_tag=map_load_scalar_i64" \
  "selected_helper=nyash.map.scalar_load_hi" \
  "selected_route_id=generic_method.get" \
  "selected_emit_kind=get" \
  "selected_effect=read.key" \
  "selected_core_op=MapGet" \
  "selected_lowering_tier=WarmDirectAbi" \
  "selected_return_shape=scalar_i64_or_missing_zero" \
  "selected_value_demand=scalar_i64" \
  "selected_publication_policy=no_publication" \
  "allowed_receiver_origin_box=MapBox" \
  "allowed_key_route=i64_const" \
  "allowed_proof_same_key=map_set_scalar_i64_same_key_no_escape" \
  "allowed_proof_dominates=map_set_scalar_i64_dominates_no_escape" \
  "runtime_data_mixed_get_preserved=1" \
  "direct_mapbox_handle_get_preserved=1" \
  "slot_load_hi_is_not_scalar_route=1" \
  "string_key_storage_remains=1" \
  "stored_value_constant_emission_deferred=1" \
  "benchmark_source_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_helper_changed=0" \
  "product_default_changed=0" \
  "selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-GUARD-SURFACE-001" \
  "summary=ok"; do
  require_line "$expected"
done

for stop_line in \
  "do not implement the route in this design row" \
  "do not use nyash.map.slot_load_hi as the scalar helper" \
  "do not reroute mixed RuntimeDataBox.get calls" \
  "do not reroute direct MapBox.get handle-return calls" \
  "do not remove String-key storage in this route row" \
  "do not add stored-value constant emission in the helper-backed route row" \
  "do not claim benchmark parity until the C pair is repaired"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q "map_get_i64_route_kind_present=0" "$PREV_CARD" || {
  echo "[$TAG] previous inventory missing absent-route proof" >&2
  exit 1
}

grep -F -q "map_get_i64_substrate_helper=nyash.map.slot_load_hi" "$PREV_CARD" || {
  echo "[$TAG] previous inventory missing slot_load_hi substrate note" >&2
  exit 1
}

grep -F -q "selected_helper=nyash.map.scalar_load_hi" "$CARD" || {
  echo "[$TAG] scalar helper contract missing" >&2
  exit 1
}

echo "[$TAG] ok"
