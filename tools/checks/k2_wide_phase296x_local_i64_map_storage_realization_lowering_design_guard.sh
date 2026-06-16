#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-storage-realization-lowering-design"
CARD="docs/development/current/main/phases/phase-296x/296x-907-LOCAL-I64-MAP-STORAGE-REALIZATION-LOWERING-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-906-LOCAL-I64-MAP-STORAGE-REALIZATION-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_storage_realization_lowering_design_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX"; do
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
  "output_contract=hako-local-i64-map-storage-realization-lowering-design-v0" \
  "source_evidence=296x-906" \
  "row_kind=lowering_design" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "selected_backend_consumer=local_map_storage_realization_plan_loader" \
  "selected_lookup_key=receiver_value" \
  "selected_first_lowering_shape=metadata_gated_local_i64_map_lookup" \
  "requires_local_fastpath_fact=1" \
  "requires_local_storage_realization_plan=1" \
  "backend_reads_fallback_evidence=0" \
  "backend_reads_helper_symbol=0" \
  "backend_reads_source_variable_name=0" \
  "backend_loader_next=1" \
  "backend_lowering_enabled=0" \
  "runtime_helper_enabled=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-BACKEND-LOADER-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for text in \
  "LocalFastPathFact at the callsite" \
  "LocalMapStorageRealizationPlan for the receiver_value" \
  "local_map_storage_realization_plans_by_receiver[receiver_value]" \
  "fallback_reason" \
  "helper symbol" \
  "source variable name" \
  "missing plan, missing fact, unknown receiver, dynamic" \
  "behavior-neutral until a later explicit"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing design text: $text" >&2
    exit 1
  }
done

grep -F -q "next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-LOWERING-DESIGN-001" "$PREV_CARD" || {
  echo "[$TAG] previous guard-surface card does not hand off to lowering design" >&2
  exit 1
}

echo "[$TAG] ok"
