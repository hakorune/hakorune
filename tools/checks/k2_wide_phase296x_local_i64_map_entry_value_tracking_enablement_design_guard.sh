#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-entry-value-tracking-enablement-design"
CARD="docs/development/current/main/phases/phase-296x/296x-922-LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-ENABLEMENT-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-921-LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SHADOW-001.md"
INDEX="docs/tools/check-scripts-index.md"
COLLECTION_CALL="src/llvm_py/instructions/mir_call/collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_entry_value_tracking_enablement_design_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$COLLECTION_CALL"; do
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
  "output_contract=hako-local-i64-map-entry-value-tracking-enablement-design-v0" \
  "source_evidence=296x-921" \
  "row_kind=design_decision" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "shadow_candidate_available=1" \
  "entry_value_tracking_available=1" \
  "executable_lowering_enabled=0" \
  "selected_decision=defer_until_entry_table_materialization_design" \
  "entry_table_materialization_owner_required=1" \
  "publication_materialization_policy_required=1" \
  "runtime_helper_abi_required=1" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "new_runtime_helper_enabled=0" \
  "backend_lowering_enabled=0" \
  "helper_emission_changed=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-DESIGN-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-ENABLEMENT-DESIGN-001" "$PREV_CARD" || {
  echo "[$TAG] previous shadow card does not hand off to enablement design" >&2
  exit 1
}

for forbidden in \
  "nyash.map.local_i64_entry" \
  "entry_table_materialize" \
  "local_i64_entry_get"; do
  if grep -F -q "$forbidden" "$COLLECTION_CALL"; then
    echo "[$TAG] executable entry-table lowering must remain disabled: $forbidden" >&2
    exit 1
  fi
done

echo "[$TAG] ok"
