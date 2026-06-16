#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-legacy-shadow-consumer-retire-design"
CARD="docs/development/current/main/phases/phase-296x/296x-910-LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-909-LOCAL-I64-MAP-STORAGE-REALIZATION-SHADOW-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_legacy_shadow_consumer_retire_design_guard.sh"

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
  "output_contract=hako-local-i64-map-legacy-shadow-consumer-retire-design-v0" \
  "source_evidence=296x-909" \
  "row_kind=retire_design" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "legacy_consumer=map_repr.local_i64_key_map_shadow" \
  "legacy_consumer_backend_proof=0" \
  "selected_action=retire_backend_consumer" \
  "remaining_fastpath_owner=LocalFastPathFact_plus_LocalMapStorageRealizationPlan" \
  "required_fact=LocalFastPathFact" \
  "required_plan=LocalMapStorageRealizationPlan" \
  "required_plan_lookup_key=receiver_value" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "new_runtime_helper_enabled=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for text in \
  "is no longer the backend proof owner" \
  "LocalFastPathFact at the callsite" \
  "LocalMapStorageRealizationPlan for receiver_value" \
  "remove the old direct consumer" \
  "producer as observation / historical evidence"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing design text: $text" >&2
    exit 1
  }
done

grep -F -q "next_task=LOCAL-I64-MAP-LEGACY-SHADOW-CONSUMER-RETIRE-DESIGN-001" "$PREV_CARD" || {
  echo "[$TAG] previous shadow card does not hand off to retire design" >&2
  exit 1
}

echo "[$TAG] ok"
