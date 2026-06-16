#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-entry-table-materialization-guard-surface"
CARD="docs/development/current/main/phases/phase-296x/296x-924-LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-923-LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-DESIGN-001.md"
SUPERSEDING_CARD="docs/development/current/main/phases/phase-296x/296x-925-LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-PILOT-001.md"
INDEX="docs/tools/check-scripts-index.md"
COLLECTION_CALL="src/llvm_py/instructions/mir_call/collection_method_call.py"
TEST="src/llvm_py/tests/test_collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_entry_table_materialization_guard_surface_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$COLLECTION_CALL" "$TEST"; do
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

require_card_line() {
  local expected="$1"
  if ! grep -F -x -q "$expected" "$CARD"; then
    echo "[$TAG] missing card line: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-local-i64-map-entry-table-materialization-guard-surface-v0" \
  "source_evidence=296x-923" \
  "row_kind=guard_surface" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "post_target=backend_local_const_i64_entry_dispatch" \
  "allowed_emit_shape=icmp_chain_or_switch_over_key" \
  "allowed_entry_source=EntryValueTrackingRows" \
  "allowed_entry_value_shape=i64_const_value_only" \
  "allowed_entry_key_shape=i64_const_key_only" \
  "allowed_fallback=current_product_compatible_map_route" \
  "required_negative_guard_non_const_value=1" \
  "required_negative_guard_missing_entry_rows=1" \
  "required_negative_guard_missing_fastpath_fact=1" \
  "runtime_helper_import_required=0" \
  "new_runtime_helper_enabled=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "backend_lowering_enabled=0" \
  "helper_emission_changed=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-PILOT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-GUARD-SURFACE-001" "$PREV_CARD" || {
  echo "[$TAG] previous design card does not hand off to guard surface" >&2
  exit 1
}

if [[ ! -f "$SUPERSEDING_CARD" ]] || ! grep -q '^Status: Landed$' "$SUPERSEDING_CARD"; then
  for forbidden in \
    "local_const_i64_entry_dispatch" \
    "nyash.map.local_i64_entry" \
    "entry_table_materialize" \
    "local_i64_entry_get"; do
    if grep -F -q "$forbidden" "$COLLECTION_CALL" "$TEST"; then
      echo "[$TAG] implementation/test target must not land before pilot row: $forbidden" >&2
      exit 1
    fi
  done
fi

echo "[$TAG] ok"
