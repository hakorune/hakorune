#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-entry-table-materialization-design"
CARD="docs/development/current/main/phases/phase-296x/296x-923-LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-922-LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-ENABLEMENT-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
COLLECTION_CALL="src/llvm_py/instructions/mir_call/collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_entry_table_materialization_design_guard.sh"

required_files=(
  "$CARD"
  "$PREV_CARD"
  "$INDEX"
  "$COLLECTION_CALL"
)

required_card_lines=(
  "output_contract=hako-local-i64-map-entry-table-materialization-design-v0"
  "source_evidence=296x-922"
  "row_kind=design_decision"
  "target_front=kilo_leaf_map_get_dynamic_covered_i64"

  "selected_materialization=backend_local_const_i64_entry_table"
  "runtime_helper_required=0"
  "runtime_helper_abi_enabled=0"
  "entry_table_owner=PythonBackendExactAOT"
  "entry_table_source=LocalI64MapEntryValueTrackingRows"
  "allowed_value_shape=i64_const_value_only"
  "allowed_key_shape=i64_const_key_only"
  "fallback_if_non_const_entry=generic_product_map_route"
  "fallback_if_incomplete_coverage=generic_product_map_route"
  "publication_materialization_policy=defer_to_product_mapbox_fallback"

  "product_mapbox_storage_changed=0"
  "product_hasher_swap=0"
  "sidecar_storage=0"
  "mirbuilder_map_storage_ownership=0"
  "new_runtime_helper_enabled=0"
  "backend_lowering_enabled=0"
  "helper_emission_changed=0"
  "winner_claim=0"

  "next_task=LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-GUARD-SURFACE-001"
  "summary=ok"
)

forbidden_runtime_patterns=(
  "nyash.map.local_i64_entry"
  "entry_table_materialize"
  "local_i64_entry_get"
)

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

require_file() {
  local file="$1"
  [[ -f "$file" ]] || fail "missing file: $file"
}

require_card_line() {
  local expected="$1"
  grep -F -x -q "$expected" "$CARD" || fail "missing card line: $expected"
}

require_index_entry() {
  grep -F -q "$SELF_SCRIPT" "$INDEX" || fail "check index missing guard entry"
}

require_previous_handoff() {
  grep -F -q "next_task=LOCAL-I64-MAP-ENTRY-TABLE-MATERIALIZATION-DESIGN-001" "$PREV_CARD" \
    || fail "previous enablement design card does not hand off to entry table design"
}

reject_runtime_lowering() {
  local pattern
  for pattern in "${forbidden_runtime_patterns[@]}"; do
    if grep -F -q "$pattern" "$COLLECTION_CALL"; then
      fail "executable entry-table lowering must remain disabled: $pattern"
    fi
  done
}

for file in "${required_files[@]}"; do
  require_file "$file"
done

grep -q '^Status: Landed$' "$CARD" || fail "card must be Landed"
require_index_entry

for expected in "${required_card_lines[@]}"; do
  require_card_line "$expected"
done

require_previous_handoff
reject_runtime_lowering

echo "[$TAG] ok"
