#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-entry-table-materialization-pilot"
CARD="docs/development/current/main/phases/phase-296x/296x-925-LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-PILOT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-924-LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
COLLECTION_CALL="src/llvm_py/instructions/mir_call/collection_method_call.py"
TEST="src/llvm_py/tests/test_collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_entry_table_materialization_pilot_guard.sh"

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
  "output_contract=hako-local-i64-map-entry-table-materialization-pilot-v0" \
  "source_evidence=296x-924" \
  "row_kind=implementation_pilot" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "implemented_shape=backend_local_const_i64_entry_dispatch" \
  "emit_shape=branch_chain_with_phi" \
  "entry_source=EntryValueTrackingRows" \
  "requires_local_fastpath_fact=1" \
  "requires_direct_storage_plan=1" \
  "requires_known_i64_const_keys=1" \
  "requires_known_i64_const_values=1" \
  "fallback_route=nyash.map.slot_load_hh" \
  "negative_guard_non_const_value=1" \
  "negative_guard_missing_entry_rows=1" \
  "runtime_helper_import_required=0" \
  "new_runtime_helper_enabled=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-VALIDATION-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for pattern in \
  "def _local_i64_map_const_entries_from_tracking_candidate" \
  "def _lower_local_i64_map_entry_table_dispatch" \
  "local_i64_map_entry_dispatch_result" \
  "local_i64_map_entry_dispatch_fallback" \
  "nyash.map.slot_load_hh"; do
  grep -F -q "$pattern" "$COLLECTION_CALL" || {
    echo "[$TAG] collection method call missing pattern: $pattern" >&2
    exit 1
  }
done

for pattern in \
  "test_local_i64_map_entry_table_dispatch_uses_const_tracking_rows" \
  "test_local_i64_map_entry_table_dispatch_rejects_non_const_value"; do
  grep -F -q "$pattern" "$TEST" || {
    echo "[$TAG] test missing pattern: $pattern" >&2
    exit 1
  }
done

if grep -F -q "nyash.map.local_i64_entry" "$COLLECTION_CALL" "$TEST"; then
  echo "[$TAG] new runtime helper must not be introduced" >&2
  exit 1
fi

grep -F -q "next_task=LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-PILOT-001" "$PREV_CARD" || {
  echo "[$TAG] previous guard-surface card does not hand off to pilot" >&2
  exit 1
}

echo "[$TAG] ok"
