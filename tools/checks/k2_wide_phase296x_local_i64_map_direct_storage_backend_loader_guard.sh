#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-direct-storage-backend-loader"
CARD="docs/development/current/main/phases/phase-296x/296x-916-LOCAL-I64-MAP-DIRECT-STORAGE-BACKEND-LOADER-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-915-LOCAL-I64-MAP-DIRECT-STORAGE-PLAN-SURFACE-001.md"
SUPERSEDING_CARD="docs/development/current/main/phases/phase-296x/296x-917-LOCAL-I64-MAP-DIRECT-STORAGE-SHADOW-001.md"
INDEX="docs/tools/check-scripts-index.md"
METADATA_LOADER="src/llvm_py/builders/function_metadata.py"
FUNCTION_LOWER="src/llvm_py/builders/function_lower.py"
CONTEXT="src/llvm_py/context/function_lower_context.py"
RESOLVER="src/llvm_py/resolver.py"
TEST_FILE="src/llvm_py/tests/test_fastmem_metadata_loader.py"
COLLECTION_CALL="src/llvm_py/instructions/mir_call/collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_direct_storage_backend_loader_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$METADATA_LOADER" "$FUNCTION_LOWER" "$CONTEXT" "$RESOLVER" "$TEST_FILE" "$COLLECTION_CALL"; do
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
  "output_contract=hako-local-i64-map-direct-storage-backend-loader-v0" \
  "source_evidence=296x-915" \
  "row_kind=metadata_loader" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "metadata_surface=metadata.local_i64_map_direct_storage_plans" \
  "backend_loader=local_i64_map_direct_storage_plans_by_receiver" \
  "loader_key=receiver_value" \
  "selected_representation=closed_world_i64_key_value_table" \
  "known_i64_key_set_count_loaded=1" \
  "scalar_get_count_loaded=1" \
  "entry_value_tracking_enabled_loaded=1" \
  "publication_materialization_required_loaded=1" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "new_runtime_helper_enabled=0" \
  "backend_lowering_enabled=0" \
  "backend_consumer_enabled=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-DIRECT-STORAGE-SHADOW-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for pattern in \
  "def _load_local_i64_map_direct_storage_plan_metadata" \
  "local_i64_map_direct_storage_plans" \
  "local_i64_map_direct_storage_plans_by_receiver" \
  "known_i64_key_set_count" \
  "entry_value_tracking_enabled"; do
  grep -F -q "$pattern" "$METADATA_LOADER" || {
    echo "[$TAG] loader missing pattern: $pattern" >&2
    exit 1
  }
done

grep -F -q "_load_local_i64_map_direct_storage_plan_metadata(builder, func_data)" "$FUNCTION_LOWER" || {
  echo "[$TAG] function_lower does not call direct storage loader" >&2
  exit 1
}

grep -F -q "local_i64_map_direct_storage_plans_by_receiver" "$CONTEXT" || {
  echo "[$TAG] context missing direct storage plan table" >&2
  exit 1
}

grep -F -q "local_i64_map_direct_storage_plans_by_receiver" "$RESOLVER" || {
  echo "[$TAG] resolver missing direct storage plan table" >&2
  exit 1
}

grep -F -q "test_local_i64_map_direct_storage_plan_loader_indexes_receivers" "$TEST_FILE" || {
  echo "[$TAG] loader test missing" >&2
  exit 1
}

if [[ ! -f "$SUPERSEDING_CARD" ]] || ! grep -q '^Status: Landed$' "$SUPERSEDING_CARD"; then
  if grep -F -q "local_i64_map_direct_storage_plans_by_receiver" "$COLLECTION_CALL"; then
    echo "[$TAG] collection call consumer must remain disabled until 296x-917 lands" >&2
    exit 1
  fi
fi

grep -F -q "next_task=LOCAL-I64-MAP-DIRECT-STORAGE-BACKEND-LOADER-001" "$PREV_CARD" || {
  echo "[$TAG] previous plan-surface card does not hand off to backend loader" >&2
  exit 1
}

echo "[$TAG] ok"
