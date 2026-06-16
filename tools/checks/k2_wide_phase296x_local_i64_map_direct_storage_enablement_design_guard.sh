#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-direct-storage-enablement-design"
CARD="docs/development/current/main/phases/phase-296x/296x-918-LOCAL-I64-MAP-DIRECT-STORAGE-ENABLEMENT-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-917-LOCAL-I64-MAP-DIRECT-STORAGE-SHADOW-001.md"
INDEX="docs/tools/check-scripts-index.md"
COLLECTION_CALL="src/llvm_py/instructions/mir_call/collection_method_call.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_direct_storage_enablement_design_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$COLLECTION_CALL"; do
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
  "output_contract=hako-local-i64-map-direct-storage-enablement-design-v0" \
  "source_evidence=296x-917" \
  "row_kind=design" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "selected_decision=entry_value_tracking_required_before_executable_lowering" \
  "direct_storage_helper_emission_allowed=0" \
  "direct_storage_backend_lowering_allowed=0" \
  "entry_value_tracking_required=1" \
  "entry_value_tracking_owner=MapStoragePlan" \
  "entry_value_tracking_surface_next=1" \
  "entry_value_tracking_backend_lowering_enabled=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "new_runtime_helper_enabled=0" \
  "backend_lowering_enabled=0" \
  "helper_emission_changed=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SURFACE-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for text in \
  "They do not yet prove which value should be returned" \
  "forbidden" \
  "receiver_value" \
  "value_value" \
  "key_const_if_known" \
  "value_const_if_known" \
  "must not lower differently"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing design text: $text" >&2
    exit 1
  }
done

if grep -F -q "local_i64_map_direct_storage_get" "$COLLECTION_CALL"; then
  echo "[$TAG] direct storage helper emission must remain disabled" >&2
  exit 1
fi

grep -F -q "next_task=LOCAL-I64-MAP-DIRECT-STORAGE-ENABLEMENT-DESIGN-001" "$PREV_CARD" || {
  echo "[$TAG] previous shadow card does not hand off to enablement design" >&2
  exit 1
}

echo "[$TAG] ok"
