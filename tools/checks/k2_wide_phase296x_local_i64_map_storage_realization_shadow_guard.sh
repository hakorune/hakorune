#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-storage-realization-shadow"
CARD="docs/development/current/main/phases/phase-296x/296x-909-LOCAL-I64-MAP-STORAGE-REALIZATION-SHADOW-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-908-LOCAL-I64-MAP-STORAGE-REALIZATION-BACKEND-LOADER-001.md"
INDEX="docs/tools/check-scripts-index.md"
BACKEND="src/llvm_py/instructions/mir_call/collection_method_call.py"
TEST="src/llvm_py/tests/test_collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_storage_realization_shadow_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$BACKEND" "$TEST"; do
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
  "output_contract=hako-local-i64-map-storage-realization-shadow-v0" \
  "source_evidence=296x-908" \
  "row_kind=backend_guard_refinement" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "fact_only_fastpath_enabled=0" \
  "fact_plus_storage_plan_required=1" \
  "storage_plan_lookup_key=receiver_value" \
  "required_storage_representation=local_i64_key_map" \
  "requires_publication_materialization_required=1" \
  "requires_backend_lowering_enabled=0" \
  "requires_runtime_helper_enabled=0" \
  "legacy_local_i64_shadow_consumer_retained=1" \
  "legacy_shadow_retire_required=1" \
  "new_runtime_helper_enabled=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-DESIGN-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for text in \
  "def _current_local_i64_map_storage_realization_plan" \
  "local_map_storage_realization_plans_by_receiver" \
  'plan.get("representation") != "local_i64_key_map"' \
  'plan.get("publication_materialization_required") is not True' \
  'plan.get("backend_lowering_enabled") is not False' \
  'plan.get("runtime_helper_enabled") is not False' \
  "local_storage_plan = _current_local_i64_map_storage_realization_plan"; do
  grep -F -q "$text" "$BACKEND" || {
    echo "[$TAG] missing backend guard evidence: $text" >&2
    exit 1
  }
done

for text in \
  "def _seed_local_i64_map_storage_realization_plan" \
  "test_mapbox_local_fastpath_fact_get_requires_storage_plan" \
  "test_mapbox_local_fastpath_fact_get_uses_known_receiver_direct_call_helper"; do
  grep -F -q "$text" "$TEST" || {
    echo "[$TAG] missing test evidence: $text" >&2
    exit 1
  }
done

grep -F -q "next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-SHADOW-001" "$PREV_CARD" || {
  echo "[$TAG] previous backend-loader card does not hand off to shadow" >&2
  exit 1
}

echo "[$TAG] ok"
