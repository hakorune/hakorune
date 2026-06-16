#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-storage-realization-backend-loader"
CARD="docs/development/current/main/phases/phase-296x/296x-908-LOCAL-I64-MAP-STORAGE-REALIZATION-BACKEND-LOADER-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-907-LOCAL-I64-MAP-STORAGE-REALIZATION-LOWERING-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
METADATA="src/llvm_py/builders/function_metadata.py"
LOWER="src/llvm_py/builders/function_lower.py"
CONTEXT="src/llvm_py/context/function_lower_context.py"
RESOLVER="src/llvm_py/resolver.py"
TEST="src/llvm_py/tests/test_fastmem_metadata_loader.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_storage_realization_backend_loader_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$METADATA" "$LOWER" "$CONTEXT" "$RESOLVER" "$TEST"; do
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
  "output_contract=hako-local-i64-map-storage-realization-backend-loader-v0" \
  "source_evidence=296x-907" \
  "row_kind=backend_metadata_loader" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "metadata_field=local_map_storage_realization_plans" \
  "loader=src/llvm_py/builders/function_metadata.py::_load_local_map_storage_realization_plan_metadata" \
  "context_field=local_map_storage_realization_plans_by_receiver" \
  "resolver_field=local_map_storage_realization_plans_by_receiver" \
  "lookup_key=receiver_value" \
  "normalizes_receiver_value=1" \
  "normalizes_candidate_counts=1" \
  "normalizes_enablement_booleans=1" \
  "backend_lowering_enabled=0" \
  "runtime_helper_enabled=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-SHADOW-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for text in \
  "def _load_local_map_storage_realization_plan_metadata" \
  'metadata.get("local_map_storage_realization_plans", [])' \
  "by_receiver: Dict[int, List[Dict[str, Any]]]" \
  "receiver_value = _as_int_or_none" \
  "builder.resolver.local_map_storage_realization_plans_by_receiver = by_receiver"; do
  grep -F -q "$text" "$METADATA" || {
    echo "[$TAG] missing loader evidence: $text" >&2
    exit 1
  }
done

grep -F -q "_load_local_map_storage_realization_plan_metadata" "$LOWER" || {
  echo "[$TAG] function lowering does not call loader" >&2
  exit 1
}

for file in "$CONTEXT" "$RESOLVER" "$TEST"; do
  grep -F -q "local_map_storage_realization_plans_by_receiver" "$file" || {
    echo "[$TAG] missing receiver-indexed field in $file" >&2
    exit 1
  }
done

grep -F -q "test_local_map_storage_realization_plan_loader_indexes_receivers" "$TEST" || {
  echo "[$TAG] missing loader unit test" >&2
  exit 1
}

grep -F -q "next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-BACKEND-LOADER-001" "$PREV_CARD" || {
  echo "[$TAG] previous lowering-design card does not hand off to backend loader" >&2
  exit 1
}

echo "[$TAG] ok"
