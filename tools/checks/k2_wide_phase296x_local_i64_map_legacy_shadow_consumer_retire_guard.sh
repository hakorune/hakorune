#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-legacy-shadow-consumer-retire"
CARD="docs/development/current/main/phases/phase-296x/296x-911-LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-910-LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
BACKEND="src/llvm_py/instructions/mir_call/collection_method_call.py"
TEST="src/llvm_py/tests/test_collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_legacy_shadow_consumer_retire_guard.sh"

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
  "output_contract=hako-local-i64-map-legacy-shadow-consumer-retire-v0" \
  "source_evidence=296x-910" \
  "row_kind=implementation" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "retired_backend_consumer=map_repr.local_i64_key_map_shadow" \
  "retired_backend_function=_current_local_i64_map_shadow_get_plan" \
  "legacy_shadow_helper_emission_enabled=0" \
  "remaining_fastpath_owner=LocalFastPathFact_plus_LocalMapStorageRealizationPlan" \
  "fact_only_fastpath_enabled=0" \
  "fact_plus_storage_plan_required=1" \
  "legacy_metadata_producer_retained=1" \
  "legacy_metadata_backend_consumable=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "new_runtime_helper_enabled=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-CLOSEOUT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

if grep -F -q "def _current_local_i64_map_shadow_get_plan" "$BACKEND"; then
  echo "[$TAG] legacy backend function still exists" >&2
  exit 1
fi

if grep -F -q "local_i64_map_get_hi" "$BACKEND"; then
  echo "[$TAG] legacy helper call name still exists" >&2
  exit 1
fi

grep -F -q "local_fastpath_map_get_hi" "$BACKEND" || {
  echo "[$TAG] Fact+Plan fastpath call is missing" >&2
  exit 1
}

for text in \
  "test_mapbox_local_i64_shadow_get_falls_back_after_consumer_retire" \
  "test_mapbox_local_fastpath_fact_get_uses_known_receiver_direct_call_helper"; do
  grep -F -q "$text" "$TEST" || {
    echo "[$TAG] missing test evidence: $text" >&2
    exit 1
  }
done

grep -F -q "next_task=LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-001" "$PREV_CARD" || {
  echo "[$TAG] previous retire design card does not hand off to implementation" >&2
  exit 1
}

echo "[$TAG] ok"
