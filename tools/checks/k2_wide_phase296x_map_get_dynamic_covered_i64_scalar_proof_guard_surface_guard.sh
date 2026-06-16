#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-dynamic-covered-i64-scalar-proof-guard-surface"
CARD="docs/development/current/main/phases/phase-296x/296x-864-MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-863-MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_dynamic_covered_i64_scalar_proof_guard_surface_guard.sh"

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
  "output_contract=hako-mimalloc-map-get-dynamic-covered-i64-scalar-proof-guard-surface-v0" \
  "source_evidence=296x-863" \
  "row_kind=guard_surface" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "target_source=benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako" \
  "post_loop_map_get_route_kind=map_load_scalar_i64" \
  "post_loop_map_get_helper=nyash.map.scalar_load_hi" \
  "post_loop_runtime_data_get_hh_count=0" \
  "post_scalar_route_count_min=1" \
  "post_final_const_key_route_may_remain_map_load_any=1" \
  "post_slot_load_hh_allowed_for_final_const_get=1" \
  "proof_owner=generic_method_route_plan_map_key_coverage_proof" \
  "accepted_key_shape=nonnegative_loop_index_mod_const_positive" \
  "accepted_preseed_shape=const_i64_key_to_const_i64_value" \
  "requires_all_residues_preseeded=1" \
  "benchmark_name_branch_count=0" \
  "helper_symbol_inference_count=0" \
  "map_storage_representation_changed=0" \
  "product_mapbox_semantics_changed=0" \
  "c_pair_measurement_used=0" \
  "winner_claim=0" \
  "selected_next=MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-IMPLEMENTATION-001" \
  "implementation_started=0" \
  "summary=ok"; do
  require_line "$expected"
done

for stop_line in \
  'do not pass the guard by editing the benchmark shape' \
  'do not special-case `kilo_leaf_map_get_dynamic_covered_i64`' \
  'do not special-case `i % 3` in backend shims' \
  'do not route unproven dynamic keys to `nyash.map.scalar_load_hi`' \
  'do not remove the final const-key fallback unless separately proven' \
  'do not claim C parity or winner status'; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q "selected_next=MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-GUARD-SURFACE-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to guard surface" >&2
  exit 1
}

echo "[$TAG] ok"
