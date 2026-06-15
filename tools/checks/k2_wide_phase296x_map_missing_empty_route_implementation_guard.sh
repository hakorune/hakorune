#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-841-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-840-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_missing_empty_route_implementation_guard.sh"
PLAN="src/mir/map_missing_empty_route_plan.rs"
ROUTE_DECISION="src/mir/route_decision/mod.rs"
BACKEND="src/llvm_py/instructions/mir_call/collection_method_call.py"
BACKEND_TEST="src/llvm_py/tests/test_collection_method_call.py"
BOUNDARY_BACKEND="lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc"
MAP_BOX="src/boxes/map_box.rs"

[[ -f "$CARD" ]] || { echo "[map-missing-empty-route-implementation] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[map-missing-empty-route-implementation] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$PLAN" ]] || { echo "[map-missing-empty-route-implementation] missing plan source: $PLAN" >&2; exit 1; }
[[ -f "$ROUTE_DECISION" ]] || { echo "[map-missing-empty-route-implementation] missing route decision source" >&2; exit 1; }
[[ -f "$BACKEND" ]] || { echo "[map-missing-empty-route-implementation] missing backend source" >&2; exit 1; }
[[ -f "$BACKEND_TEST" ]] || { echo "[map-missing-empty-route-implementation] missing backend test source" >&2; exit 1; }
[[ -f "$BOUNDARY_BACKEND" ]] || { echo "[map-missing-empty-route-implementation] missing boundary backend source" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[map-missing-empty-route-implementation] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[map-missing-empty-route-implementation] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[map-missing-empty-route-implementation] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[map-missing-empty-route-implementation] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-map-missing-empty-route-implementation-v0" \
  "source_evidence=296x-840" \
  "row_kind=implementation" \
  "implementation_started=1" \
  "target_front=kilo_leaf_map_get_missing" \
  "target_site=MapGet bb19.i3" \
  "missing_empty_map_route_plan_added=1" \
  "route_decision_source_plan_kind=MapMissingEmptyRoute" \
  "selected_route=map_get_missing_empty_const_zero" \
  "selected_i64_const=0" \
  "selected_route_count_on_target_front=1" \
  "post_missing_empty_map_route_count=1" \
  "post_selected_route=map_get_missing_empty_const_zero" \
  "post_route_source=MapMissingEmptyRoute" \
  "post_backend_consumes_route_decision=1" \
  "post_boundary_backend_consumes_route_decision=1" \
  "post_backend_literal_key_special_case_count=0" \
  "post_backend_helper_name_branch_count=0" \
  "post_backend_benchmark_name_branch_count=0" \
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
  "post_mapbox_storage_changed=0" \
  "post_mapbox_public_semantics_changed=0" \
  "post_product_default_changed=0" \
  "perf_win_claim=0" \
  "selected_next=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-MEASUREMENT-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  'pub struct MapMissingEmptyRoute' \
  'collect_function_map_missing_empty_routes' \
  '"receiver_birth_is_new_mapbox"' \
  '"receiver_root_is_same_local_value"' \
  '"receiver_not_published_before_get"' \
  '"receiver_not_escaped_before_get"' \
  '"no_map_set_before_get"' \
  '"no_map_delete_before_get"' \
  '"no_map_clear_before_get"' \
  '"no_unknown_receiver_mutation_before_get"' \
  '"i64_const_key"' \
  '"runtime_data_facade_missing_shape"'; do
  grep -F -q "$expected" "$PLAN" || {
    echo "[map-missing-empty-route-implementation] plan missing expected token: $expected" >&2
    exit 1
  }
done

for expected in \
  'MapMissingEmptyRoute' \
  'collect_function_map_missing_empty_routes' \
  '"map_missing_empty_get"' \
  '"map_get_missing_empty_const_zero"' \
  '"generic_map_runtime_data_load_any"' \
  'selected_i64_const: Some(0)' \
  'receiver_box_name: Some("MapBox".to_string())'; do
  grep -F -q "$expected" "$ROUTE_DECISION" || {
    echo "[map-missing-empty-route-implementation] route decision missing token: $expected" >&2
    exit 1
  }
done

for expected in \
  'MAP_MISSING_EMPTY_CONST_ZERO_ROUTE = "map_get_missing_empty_const_zero"' \
  'MAP_MISSING_EMPTY_SOURCE_PLAN = "MapMissingEmptyRoute"' \
  'source_plan_kind != MAP_MISSING_EMPTY_SOURCE_PLAN'; do
  grep -F -q "$expected" "$BACKEND" || {
    echo "[map-missing-empty-route-implementation] backend missing token: $expected" >&2
    exit 1
  }
done

for expected in \
  'test_mapbox_missing_empty_route_decision_folds_to_zero' \
  'test_mapbox_missing_empty_ignores_non_map_source_plan_kind'; do
  grep -F -q "$expected" "$BACKEND_TEST" || {
    echo "[map-missing-empty-route-implementation] backend tests missing token: $expected" >&2
    exit 1
  }
done

for expected in \
  'match_map_missing_empty_route_decision' \
  '"route.decision"' \
  '"MapGet"' \
  '"map_missing_empty_get"' \
  '"map_get_missing_empty_const_zero"' \
  '"generic_map_runtime_data_load_any"' \
  '"MapMissingEmptyRoute"' \
  '"selected_i64_const"' \
  'hako_llvmc_emit_route_trace(' \
  '"map_missing_empty_route"' \
  'fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);'; do
  grep -F -q "$expected" "$BOUNDARY_BACKEND" || {
    echo "[map-missing-empty-route-implementation] boundary backend missing token: $expected" >&2
    exit 1
  }
done

grep -F -q 'data: Arc<RwLock<HashMap<String, Box<dyn NyashBox>>>>' "$MAP_BOX" || {
  echo "[map-missing-empty-route-implementation] MapBox storage changed" >&2
  exit 1
}

echo "[map-missing-empty-route-implementation] ok"
