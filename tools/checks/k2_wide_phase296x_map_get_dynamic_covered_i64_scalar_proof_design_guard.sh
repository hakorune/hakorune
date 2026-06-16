#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-dynamic-covered-i64-scalar-proof-design"
CARD="docs/development/current/main/phases/phase-296x/296x-863-MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-862-MIMALLOC-MAP-GET-NONFOLDED-SCALAR-FRONT-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_dynamic_covered_i64_scalar_proof_design_guard.sh"

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
  "output_contract=hako-mimalloc-map-get-dynamic-covered-i64-scalar-proof-design-v0" \
  "source_evidence=296x-862" \
  "row_kind=design" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "target_current_loop_helper=nyash.runtime_data.get_hh" \
  "target_desired_loop_helper=nyash.map.scalar_load_hi" \
  "selected_owner=generic_method_route_plan_map_key_coverage_proof" \
  "selected_owner_scope=route_proof_only" \
  "map_storage_representation_changed=0" \
  "runtime_helper_semantics_changed=0" \
  "product_mapbox_semantics_changed=0" \
  "proof_shape=covered_dynamic_i64_key_scalar_values" \
  "accepted_key_shape=nonnegative_loop_index_mod_const_positive" \
  "accepted_key_modulus=3" \
  "accepted_preseed_shape=const_i64_key_to_const_i64_value" \
  "requires_all_residues_preseeded=1" \
  "requires_no_same_receiver_mutation_between_preseed_and_get=1" \
  "requires_no_publication_or_unknown_escape_before_get=1" \
  "route_kind_on_success=map_load_scalar_i64" \
  "helper_on_success=nyash.map.scalar_load_hi" \
  "value_demand_on_success=scalar_i64" \
  "publication_policy_on_success=no_publication" \
  "fallback_on_missing_proof=existing_runtime_data_get_or_map_load_any" \
  "fallback_silent=0" \
  "benchmark_name_branch_count=0" \
  "helper_symbol_inference_count=0" \
  "implementation_started=0" \
  "selected_next=MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-GUARD-SURFACE-001" \
  "summary=ok"; do
  require_line "$expected"
done

for stop_line in \
  'do not infer coverage from benchmark name' \
  'do not infer scalar legality from helper symbol names' \
  'do not accept arbitrary `% n` keys without nonnegative index proof' \
  'do not accept partial residue coverage' \
  'do not accept non-const stored scalar values in this first row' \
  'do not change MapBox storage representation' \
  'do not make LLVM backend reinterpret generic map layout' \
  'do not claim C parity or winner status from this proof row'; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[$TAG] missing stop line: $stop_line" >&2
    exit 1
  }
done

grep -F -q "selected_next=MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-DESIGN-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to design" >&2
  exit 1
}

echo "[$TAG] ok"
