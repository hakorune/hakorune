#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-missing-empty-route-measurement"
CARD="docs/development/current/main/phases/phase-296x/296x-842-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-MEASUREMENT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-841-MIMALLOC-MAP-MISSING-EMPTY-ROUTE-IMPLEMENTATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_missing_empty_route_measurement_guard.sh"
BOUNDARY_BACKEND="lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc"

[[ -f "$CARD" ]] || { echo "[$TAG] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[$TAG] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$BOUNDARY_BACKEND" ]] || { echo "[$TAG] missing boundary backend" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[$TAG] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[$TAG] previous card must be Landed" >&2
  exit 1
}
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
  "output_contract=hako-mimalloc-map-missing-empty-route-measurement-v0" \
  "source_evidence=296x-841" \
  "row_kind=measurement" \
  "target_front=kilo_leaf_map_get_missing" \
  "selected_route=map_get_missing_empty_const_zero" \
  "source_plan_kind=MapMissingEmptyRoute" \
  "before_ny_aot_instr=896474449" \
  "before_ny_aot_cycles=220979722" \
  "before_ny_aot_ms=45" \
  "after_c_instr=10125074" \
  "after_c_cycles=2191922" \
  "after_c_ms=4" \
  "after_ny_aot_instr=472935" \
  "after_ny_aot_cycles=749368" \
  "after_ny_aot_ms=3" \
  "after_ratio_instr_c_over_hako=21.41" \
  "after_ratio_cycles_c_over_hako=2.93" \
  "after_ratio_ms_c_over_hako=1.33" \
  "ny_main_runtime_data_get_hh_call_count=0" \
  "ny_main_map_birth_h_call_count=1" \
  "ny_main_return_const=2000000" \
  "route_winner_claim=1" \
  "kernel_path_closed_for_this_front=1" \
  "product_default_changed=0" \
  "mapbox_storage_changed=0" \
  "mapbox_public_semantics_changed=0" \
  "selected_next=MIMALLOC-MAP-MISSING-EMPTY-ROUTE-CLOSEOUT-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  'match_map_missing_empty_route_decision' \
  '"map_get_missing_empty_const_zero"' \
  '"MapMissingEmptyRoute"' \
  'fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);'; do
  grep -F -q "$expected" "$BOUNDARY_BACKEND" || {
    echo "[$TAG] boundary backend missing token: $expected" >&2
    exit 1
  }
done

echo "[$TAG] ok"
