#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="map-scalar-load-i64-borrowed-lookup-validation"
CARD="docs/development/current/main/phases/phase-296x/296x-870-MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-VALIDATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-869-MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-IMPLEMENTATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_map_scalar_load_i64_borrowed_lookup_validation_guard.sh"
DESIGN_GUARD="tools/checks/k2_wide_phase296x_map_scalar_load_i64_borrowed_lookup_design_guard.sh"
IMPLEMENTATION_GUARD="tools/checks/k2_wide_phase296x_map_scalar_load_i64_borrowed_lookup_implementation_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$DESIGN_GUARD" "$IMPLEMENTATION_GUARD"; do
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
  "output_contract=hako-mimalloc-map-scalar-load-i64-borrowed-lookup-validation-v0" \
  "source_evidence=296x-869" \
  "row_kind=validation" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "implementation_guard_green=1" \
  "design_guard_green=1" \
  "cargo_fmt_check_green=1" \
  "cargo_check_release_bin_hakorune_green=1" \
  "current_state_pointer_guard_green=1" \
  "git_diff_check_green=1" \
  "validated_shape=scalar_helper_borrowed_lookup" \
  "validated_helper=nyash.map.scalar_load_hi" \
  "mapbox_storage_change_enabled=0" \
  "i64_sidecar_storage_enabled=0" \
  "mapbox_public_get_contract_changed=0" \
  "mapbox_public_set_contract_changed=0" \
  "slot_load_hi_changed=0" \
  "slot_load_hh_changed=0" \
  "runtime_data_get_route_change_enabled=0" \
  "product_default_changed=0" \
  "winner_claim=0" \
  "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-MEASUREMENT-001" \
  "summary=ok"; do
  require_line "$expected"
done

grep -F -q "selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-VALIDATION-001" "$PREV_CARD" || {
  echo "[$TAG] previous row does not hand off to this validation" >&2
  exit 1
}

bash "$DESIGN_GUARD"
bash "$IMPLEMENTATION_GUARD"

echo "[$TAG] ok"
