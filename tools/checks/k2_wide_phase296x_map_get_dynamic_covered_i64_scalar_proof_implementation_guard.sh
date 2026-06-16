#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-get-dynamic-covered-i64-scalar-proof-implementation"
CARD="docs/development/current/main/phases/phase-296x/296x-865-MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-864-MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_get_dynamic_covered_i64_scalar_proof_implementation_guard.sh"
BENCH="benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$BENCH"; do
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
  "output_contract=hako-mimalloc-map-get-dynamic-covered-i64-scalar-proof-implementation-v0" \
  "source_evidence=296x-862,296x-863,296x-864" \
  "row_kind=implementation" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "target_source=benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako" \
  "implementation_owner=src/mir/generic_method_route_plan/map_set_scalar_proof.rs" \
  "route_policy_owner=src/mir/generic_method_route_plan/collection_read_routes.rs" \
  "backend_boundary_owner=lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc" \
  "backend_need_owner=lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_metadata_rules.inc" \
  "new_route_proof=map_set_scalar_i64_covered_dynamic_i64_key_no_escape" \
  "accepted_key_shape=nonnegative_loop_index_mod_const_positive" \
  "accepted_modulus_current=3" \
  "accepted_preseed_shape=const_i64_key_to_const_i64_value" \
  "requires_all_residues_preseeded=1" \
  "requires_no_same_receiver_unknown_mutation_or_escape_before_get=1" \
  "post_loop_map_get_route_kind=map_load_scalar_i64" \
  "post_loop_map_get_helper=nyash.map.scalar_load_hi" \
  "post_loop_runtime_data_get_hh_count=0" \
  "post_scalar_route_count_min=1" \
  "post_object_imports_scalar_load_hi=1" \
  "post_object_imports_runtime_data_get_hh=0" \
  "post_final_const_key_route_may_remain_map_load_any=1" \
  "post_slot_load_hh_allowed_for_final_const_get=1" \
  "map_storage_representation_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "helper_symbol_inference_count=0" \
  "benchmark_name_branch_count=0" \
  "c_pair_measurement_used=0" \
  "winner_claim=0" \
  "selected_next=MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-MEASUREMENT-001" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-GET-DYNAMIC-COVERED-I64-SCALAR-PROOF-IMPLEMENTATION-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to implementation" >&2
  exit 1
}

for source_line in \
  "MapSetScalarI64CoveredDynamicI64KeyNoEscape" \
  "map_set_scalar_i64_covered_dynamic_i64_key_no_escape"; do
  rg -q "$source_line" src/mir/generic_method_route_plan lang/c-abi/shims || {
    echo "[$TAG] missing implementation token: $source_line" >&2
    exit 1
  }
done

cargo build --release --bin hakorune >/dev/null
bash tools/build_hako_llvmc_ffi.sh >/dev/null

TMP_DIR="target/tmp/${TAG}"
mkdir -p "$TMP_DIR"
MIR_JSON="$TMP_DIR/map_get_dynamic_covered_i64.mir.json"
OBJ="$TMP_DIR/map_get_dynamic_covered_i64.o"

NYASH_GC_MODE=off NYASH_DISABLE_PLUGINS=1 NYASH_SKIP_TOML_ENV=1 \
  target/release/hakorune --backend mir --emit-mir-json "$MIR_JSON" "$BENCH" >/tmp/${TAG}.emit_mir.log 2>&1

scalar_route_count="$({ grep -F '"route_kind": "map_load_scalar_i64"' "$MIR_JSON" || true; } | wc -l | tr -d ' ')"
if [[ "$scalar_route_count" -lt 1 ]]; then
  echo "[$TAG] expected at least one scalar route in selected front" >&2
  exit 1
fi

grep -F -q '"symbol": "nyash.map.scalar_load_hi"' "$MIR_JSON" || {
  echo "[$TAG] MIR missing scalar helper symbol" >&2
  exit 1
}

runtime_get_count="$({ grep -F '"symbol": "nyash.runtime_data.get_hh"' "$MIR_JSON" || true; } | wc -l | tr -d ' ')"
if [[ "$runtime_get_count" -ne 0 ]]; then
  echo "[$TAG] runtime_data.get_hh should be absent after scalar proof, count=$runtime_get_count" >&2
  exit 1
fi

NYASH_LLVM_SKIP_BUILD=1 bash tools/ny_mir_builder.sh --in "$MIR_JSON" --emit obj -o "$OBJ" --quiet
nm -u "$OBJ" > "$TMP_DIR/map_get_dynamic_covered_i64.nm"

grep -F -q "nyash.map.scalar_load_hi" "$TMP_DIR/map_get_dynamic_covered_i64.nm" || {
  echo "[$TAG] object missing scalar helper import" >&2
  cat "$TMP_DIR/map_get_dynamic_covered_i64.nm" >&2 || true
  exit 1
}

if grep -F -q "nyash.runtime_data.get_hh" "$TMP_DIR/map_get_dynamic_covered_i64.nm"; then
  echo "[$TAG] object still imports runtime_data.get_hh" >&2
  cat "$TMP_DIR/map_get_dynamic_covered_i64.nm" >&2 || true
  exit 1
fi

grep -F -q "nyash.map.slot_load_hh" "$TMP_DIR/map_get_dynamic_covered_i64.nm" || {
  echo "[$TAG] final const-key fallback slot_load_hh should remain allowed" >&2
  cat "$TMP_DIR/map_get_dynamic_covered_i64.nm" >&2 || true
  exit 1
}

echo "[$TAG] ok"
