#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-storage-realization-closeout"
CARD="docs/development/current/main/phases/phase-296x/296x-912-LOCAL-I64-MAP-STORAGE-REALIZATION-CLOSEOUT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-911-LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-001.md"
INDEX="docs/tools/check-scripts-index.md"
BACKEND="src/llvm_py/instructions/mir_call/collection_method_call.py"
METADATA="src/llvm_py/builders/function_metadata.py"
CONTEXT="src/llvm_py/context/function_lower_context.py"
RESOLVER="src/llvm_py/resolver.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_storage_realization_closeout_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$BACKEND" "$METADATA" "$CONTEXT" "$RESOLVER"; do
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
  "output_contract=hako-local-i64-map-storage-realization-closeout-v0" \
  "source_evidence=296x-905..296x-911" \
  "row_kind=closeout" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "closed_family=local_i64_map_storage_realization" \
  "metadata_surface_landed=1" \
  "backend_loader_landed=1" \
  "fact_plus_plan_guard_landed=1" \
  "legacy_shadow_consumer_retired=1" \
  "backend_fastpath_owner=LocalFastPathFact_plus_LocalMapStorageRealizationPlan" \
  "backend_reads_fallback_evidence=0" \
  "backend_reads_helper_symbol=0" \
  "backend_reads_source_variable_name=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "new_runtime_helper_enabled=0" \
  "winner_claim=0" \
  "next_task=MAP-HASH-OWNER-REFRESH-AFTER-LOCAL-I64-STORAGE-REALIZATION-CLOSEOUT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for text in \
  "def _load_local_map_storage_realization_plan_metadata" \
  "local_map_storage_realization_plans_by_receiver"; do
  grep -F -q "$text" "$METADATA" || {
    echo "[$TAG] missing metadata loader evidence: $text" >&2
    exit 1
  }
done

for file in "$CONTEXT" "$RESOLVER"; do
  grep -F -q "local_map_storage_realization_plans_by_receiver" "$file" || {
    echo "[$TAG] missing receiver-indexed state in $file" >&2
    exit 1
  }
done

if grep -F -q "def _current_local_i64_map_shadow_get_plan" "$BACKEND"; then
  echo "[$TAG] legacy shadow consumer function still exists" >&2
  exit 1
fi

grep -F -q "local_fastpath_map_get_hi" "$BACKEND" || {
  echo "[$TAG] Fact+Plan fastpath helper emission is missing" >&2
  exit 1
}

grep -F -q "next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-CLOSEOUT-001" "$PREV_CARD" || {
  echo "[$TAG] previous retire card does not hand off to closeout" >&2
  exit 1
}

echo "[$TAG] ok"
