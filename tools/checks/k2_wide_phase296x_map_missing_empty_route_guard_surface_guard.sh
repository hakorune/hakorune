#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-840-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-839-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_missing_empty_route_guard_surface_guard.sh"
MAP_SSOT="docs/development/current/main/design/mapbox-proof-bearing-route-ssot.md"
MAP_BOX="src/boxes/map_box.rs"

[[ -f "$CARD" ]] || { echo "[map-missing-empty-route-guard-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[map-missing-empty-route-guard-surface] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$MAP_SSOT" ]] || { echo "[map-missing-empty-route-guard-surface] missing MapBox SSOT" >&2; exit 1; }
[[ -f "$MAP_BOX" ]] || { echo "[map-missing-empty-route-guard-surface] missing MapBox source" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[map-missing-empty-route-guard-surface] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[map-missing-empty-route-guard-surface] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[map-missing-empty-route-guard-surface] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[map-missing-empty-route-guard-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-map-missing-empty-route-guard-surface-v0" \
  "source_evidence=296x-839" \
  "row_kind=guard_surface" \
  "implementation_started=0" \
  "perf_first_required=1" \
  "target_front=kilo_leaf_map_get_missing" \
  "target_site=MapGet bb19.i3" \
  "allowed_implementation=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-IMPLEMENTATION-001" \
  "post_missing_empty_map_route_count=1" \
  "post_selected_route=map_get_missing_empty_const_zero" \
  "post_route_source=MapMissingEmptyRoute" \
  "post_receiver_birth_is_new_mapbox=1" \
  "post_receiver_root_is_same_local_value=1" \
  "post_receiver_not_published_before_get=1" \
  "post_receiver_not_escaped_before_get=1" \
  "post_no_map_set_before_get=1" \
  "post_no_map_delete_before_get=1" \
  "post_no_map_clear_before_get=1" \
  "post_no_unknown_receiver_mutation_before_get=1" \
  "post_get_key_route=i64_const" \
  "post_generic_method_publication_policy=runtime_data_facade" \
  "post_generic_method_return_shape=mixed_runtime_i64_or_handle" \
  "post_backend_consumes_route_decision=1" \
  "post_backend_literal_key_special_case_count=0" \
  "post_backend_helper_name_branch_count=0" \
  "post_backend_benchmark_name_branch_count=0" \
  "post_mapbox_storage_changed=0" \
  "post_mapbox_public_semantics_changed=0" \
  "post_product_default_changed=0" \
  "selected_next=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-IMPLEMENTATION-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for stop_line in \
  "do not broaden beyond get-only missing-empty-map" \
  "do not implement a generic DirectMap" \
  "do not patch MapBox storage or visible get semantics" \
  "do not fold from literal key 0 without receiver birth and no-mutation proofs" \
  "do not let backend infer from helper names" \
  "do not use benchmark-name branches" \
  "do not claim a perf win before measurement row"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[map-missing-empty-route-guard-surface] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q 'RouteDecision selected_route:' "$MAP_SSOT" || {
  echo "[map-missing-empty-route-guard-surface] MapBox SSOT missing RouteDecision selected route contract" >&2
  exit 1
}
grep -F -q 'data: Arc<RwLock<HashMap<String, Box<dyn NyashBox>>>>' "$MAP_BOX" || {
  echo "[map-missing-empty-route-guard-surface] MapBox storage changed before implementation row" >&2
  exit 1
}

echo "[map-missing-empty-route-guard-surface] ok"
