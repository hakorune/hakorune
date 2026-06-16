#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-entry-value-tracking-backend-loader"
CARD="docs/development/current/main/phases/phase-296x/296x-920-LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-BACKEND-LOADER-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-919-LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SURFACE-001.md"
SUPERSEDING_CARD="docs/development/current/main/phases/phase-296x/296x-921-LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SHADOW-001.md"
INDEX="docs/tools/check-scripts-index.md"
FUNCTION_METADATA="src/llvm_py/builders/function_metadata.py"
FUNCTION_LOWER="src/llvm_py/builders/function_lower.py"
CONTEXT="src/llvm_py/context/function_lower_context.py"
RESOLVER="src/llvm_py/resolver.py"
TEST="src/llvm_py/tests/test_fastmem_metadata_loader.py"
COLLECTION_CALL="src/llvm_py/instructions/mir_call/collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_entry_value_tracking_backend_loader_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$FUNCTION_METADATA" "$FUNCTION_LOWER" "$CONTEXT" "$RESOLVER" "$TEST" "$COLLECTION_CALL"; do
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
  "output_contract=hako-local-i64-map-entry-value-tracking-backend-loader-v0" \
  "source_evidence=296x-919" \
  "row_kind=metadata_loader" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "metadata_surface=metadata.local_i64_map_entry_value_tracking_plans" \
  "backend_loader=local_i64_map_entry_value_tracking_plans_by_receiver" \
  "loader_key=receiver_value" \
  "set_site_loaded=1" \
  "key_value_loaded=1" \
  "value_value_loaded=1" \
  "key_const_if_known_loaded=1" \
  "value_const_if_known_loaded=1" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "new_runtime_helper_enabled=0" \
  "backend_lowering_enabled=0" \
  "backend_consumer_enabled=0" \
  "helper_emission_changed=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SHADOW-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for pattern in \
  "def _load_local_i64_map_entry_value_tracking_plan_metadata" \
  '"local_i64_map_entry_value_tracking_plans"' \
  "local_i64_map_entry_value_tracking_plans_by_receiver" \
  '"set_block"' \
  '"set_instruction_index"' \
  '"key_value"' \
  '"value_value"' \
  '"key_const_if_known"' \
  '"value_const_if_known"' \
  '"backend_lowering_enabled"' \
  '"runtime_helper_enabled"'; do
  grep -F -q "$pattern" "$FUNCTION_METADATA" || {
    echo "[$TAG] function metadata loader missing pattern: $pattern" >&2
    exit 1
  }
done

grep -F -q "_load_local_i64_map_entry_value_tracking_plan_metadata" "$FUNCTION_LOWER" || {
  echo "[$TAG] function lower must import/call entry value tracking loader" >&2
  exit 1
}

for file in "$CONTEXT" "$RESOLVER"; do
  grep -F -q "local_i64_map_entry_value_tracking_plans_by_receiver" "$file" || {
    echo "[$TAG] missing resolver/context metadata table in $file" >&2
    exit 1
  }
done

grep -F -q "test_local_i64_map_entry_value_tracking_plan_loader_indexes_receivers" "$TEST" || {
  echo "[$TAG] loader test missing entry value tracking coverage" >&2
  exit 1
}

if [[ ! -f "$SUPERSEDING_CARD" ]] || ! grep -q '^Status: Landed$' "$SUPERSEDING_CARD"; then
  if grep -F -q "local_i64_map_entry_value_tracking_plans_by_receiver" "$COLLECTION_CALL"; then
    echo "[$TAG] collection call consumer must remain disabled until 296x-921 lands" >&2
    exit 1
  fi
fi

grep -F -q "next_task=LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-BACKEND-LOADER-001" "$PREV_CARD" || {
  echo "[$TAG] previous surface card does not hand off to backend loader" >&2
  exit 1
}

echo "[$TAG] ok"
