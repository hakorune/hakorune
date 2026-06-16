#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-i64-scalar-route-guard-surface"
CARD="docs/development/current/main/phases/phase-296x/296x-855-MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-854-MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_i64_scalar_route_guard_surface_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX"; do
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
  "output_contract=hako-mimalloc-map-get-i64-scalar-route-guard-surface-v0" \
  "source_evidence=296x-854" \
  "row_kind=guard_surface" \
  "implementation_started=0" \
  "allowed_route_kind=MapLoadScalarI64" \
  "allowed_route_tag=map_load_scalar_i64" \
  "allowed_helper=nyash.map.scalar_load_hi" \
  "allowed_receiver_origin_box=MapBox" \
  "allowed_key_route=i64_const" \
  "allowed_proofs=map_set_scalar_i64_same_key_no_escape,map_set_scalar_i64_dominates_no_escape" \
  "post_map_get_scalar_i64_route_kind_present=1" \
  "post_map_get_scalar_i64_route_tag=map_load_scalar_i64" \
  "post_map_get_scalar_i64_helper=nyash.map.scalar_load_hi" \
  "post_scalar_proof_runtime_data_get_route_kind=MapLoadScalarI64" \
  "post_scalar_proof_lowering_tier=WarmDirectAbi" \
  "post_mixed_runtime_data_get_route_kind=RuntimeDataLoadAny" \
  "post_mixed_runtime_data_get_helper=nyash.runtime_data.get_hh" \
  "post_direct_mapbox_get_route_kind=MapLoadAny" \
  "post_direct_mapbox_get_helper=nyash.map.slot_load_hh" \
  "post_slot_load_hi_scalar_route_usage=0" \
  "benchmark_source_changed=0" \
  "product_default_changed=0" \
  "stored_value_constant_emission_enabled=0" \
  "typed_i64_key_map_storage_enabled=0" \
  "selected_next=MIMALLOC-MAP-GET-I64-SCALAR-ROUTE-IMPLEMENTATION-001" \
  "summary=ok"; do
  require_line "$expected"
done

for stop_line in \
  "do not implement outside the next implementation row" \
  "do not route unproven get calls to MapLoadScalarI64" \
  "do not use nyash.map.slot_load_hi as scalar helper" \
  "do not change C benchmark source" \
  "do not add typed i64-key map storage" \
  "do not emit stored_value constants in this helper route" \
  "do not change RuntimeDataBox.get mixed return contract" \
  "do not change direct MapBox.get handle return contract"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q "selected_route_kind=MapLoadScalarI64" "$PREV_CARD" || {
  echo "[$TAG] previous design missing selected route kind" >&2
  exit 1
}

grep -F -q "selected_helper=nyash.map.scalar_load_hi" "$PREV_CARD" || {
  echo "[$TAG] previous design missing scalar helper" >&2
  exit 1
}

echo "[$TAG] ok"
