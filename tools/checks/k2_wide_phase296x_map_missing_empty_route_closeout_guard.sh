#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-missing-empty-route-closeout"
CARD="docs/development/current/main/phases/phase-296x/296x-843-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-CLOSEOUT-001.md"
IMPL_CARD="docs/development/current/main/phases/phase-296x/296x-841-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-IMPLEMENTATION-001.md"
MEASURE_CARD="docs/development/current/main/phases/phase-296x/296x-842-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-MEASUREMENT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_missing_empty_route_closeout_guard.sh"
PLAN="src/mir/map_missing_empty_route_plan.rs"
ROUTE_DECISION="src/mir/route_decision/mod.rs"
BOUNDARY_BACKEND="lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc"
PY_BACKEND="src/llvm_py/instructions/mir_call/collection_method_call.py"

for file in "$CARD" "$IMPL_CARD" "$MEASURE_CARD" "$PLAN" "$ROUTE_DECISION" "$BOUNDARY_BACKEND" "$PY_BACKEND"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

for card in "$CARD" "$IMPL_CARD" "$MEASURE_CARD"; do
  grep -q '^Status: Landed$' "$card" || {
    echo "[$TAG] card must be Landed: $card" >&2
    exit 1
  }
done

grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[$TAG] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[$TAG] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-map-missing-empty-route-closeout-v0" \
  "source_evidence=296x-841,296x-842" \
  "row_kind=closeout" \
  "target_front=kilo_leaf_map_get_missing" \
  "route_plan_owner=MapMissingEmptyRoute" \
  "route_decision_owner=RouteDecision" \
  "backend_consumer_owner=generic_method_get_policy" \
  "python_backend_unit_consumer=1" \
  "boundary_backend_consumer=1" \
  "selected_route=map_get_missing_empty_const_zero" \
  "source_plan_kind=MapMissingEmptyRoute" \
  "selected_i64_const=0" \
  "ny_main_runtime_data_get_hh_call_count=0" \
  "route_winner_claim=1" \
  "front_closed=1" \
  "kernel_path_closed_for_this_front=1" \
  "generic_direct_map_enabled=0" \
  "mapbox_storage_changed=0" \
  "mapbox_public_semantics_changed=0" \
  "product_default_changed=0" \
  "helper_name_inference_enabled=0" \
  "literal_key_only_fold_enabled=0" \
  "selected_next=MIMALLOC-FRESH-FRONT-SELECTION-AFTER-MAP-MISSING-EMPTY-CLOSEOUT-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  'MapMissingEmptyRoute' \
  '"map_get_missing_empty_const_zero"' \
  '"MapMissingEmptyRoute"'; do
  grep -F -q "$expected" "$PLAN" "$ROUTE_DECISION" "$BOUNDARY_BACKEND" "$PY_BACKEND" || {
    echo "[$TAG] implementation missing expected token: $expected" >&2
    exit 1
  }
done

echo "[$TAG] ok"
