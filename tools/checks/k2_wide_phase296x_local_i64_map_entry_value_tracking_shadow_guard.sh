#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-entry-value-tracking-shadow"
CARD="docs/development/current/main/phases/phase-296x/296x-921-LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SHADOW-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-920-LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-BACKEND-LOADER-001.md"
INDEX="docs/tools/check-scripts-index.md"
COLLECTION_CALL="src/llvm_py/instructions/mir_call/collection_method_call.py"
TEST="src/llvm_py/tests/test_collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_entry_value_tracking_shadow_guard.sh"

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
  "output_contract=hako-local-i64-map-entry-value-tracking-shadow-v0" \
  "source_evidence=296x-920" \
  "row_kind=shadow_only" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "shadow_candidate=LocalFastPathFact+LocalI64MapDirectStoragePlan+EntryValueTrackingRows" \
  "requires_local_fastpath_fact=1" \
  "requires_direct_storage_plan=1" \
  "requires_entry_value_tracking_rows=1" \
  "requires_receiver_match=1" \
  "requires_key_value=1" \
  "requires_value_value=1" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "new_runtime_helper_enabled=0" \
  "backend_lowering_enabled=0" \
  "helper_emission_changed=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-ENABLEMENT-DESIGN-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for pattern in \
  "def _current_local_i64_map_entry_value_tracking_shadow_candidate" \
  "_current_local_i64_map_direct_storage_shadow_candidate" \
  "local_i64_map_entry_value_tracking_plans_by_receiver" \
  '"entry_value_tracking"' \
  '"value_value"' \
  '"key_value"' \
  '"backend_lowering_enabled"' \
  '"runtime_helper_enabled"'; do
  grep -F -q "$pattern" "$COLLECTION_CALL" || {
    echo "[$TAG] collection method call missing pattern: $pattern" >&2
    exit 1
  }
done

for pattern in \
  "_seed_local_i64_map_entry_value_tracking_plan" \
  "test_local_i64_map_entry_value_tracking_shadow_requires_entry_rows" \
  "test_local_i64_map_entry_value_tracking_shadow_rejects_missing_rows"; do
  grep -F -q "$pattern" "$TEST" || {
    echo "[$TAG] test missing pattern: $pattern" >&2
    exit 1
  }
done

if grep -F -q "nyash.map.local_i64_entry" "$COLLECTION_CALL"; then
  echo "[$TAG] entry table helper emission must remain disabled" >&2
  exit 1
fi

grep -F -q "next_task=LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SHADOW-001" "$PREV_CARD" || {
  echo "[$TAG] previous backend-loader card does not hand off to shadow row" >&2
  exit 1
}

echo "[$TAG] ok"
