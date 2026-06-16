#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-entry-value-tracking-surface"
CARD="docs/development/current/main/phases/phase-296x/296x-919-LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-918-LOCAL-I64-MAP-DIRECT-STORAGE-ENABLEMENT-DESIGN-001.md"
SUPERSEDING_CARD="docs/development/current/main/phases/phase-296x/296x-920-LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-BACKEND-LOADER-001.md"
INDEX="docs/tools/check-scripts-index.md"
MAP_PLAN="src/mir/map_repr_plan.rs"
METADATA="src/mir/function/metadata.rs"
JSON_EMIT="src/runner/mir_json_emit/plan_metadata.rs"
JSON_TEST="src/runner/mir_json_emit/tests/map_repr_plans.rs"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_entry_value_tracking_surface_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$MAP_PLAN" "$METADATA" "$JSON_EMIT" "$JSON_TEST"; do
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
  "output_contract=hako-local-i64-map-entry-value-tracking-surface-v0" \
  "source_evidence=296x-918" \
  "row_kind=passive_metadata_surface" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "metadata_surface=FunctionMetadata.local_i64_map_entry_value_tracking_plans" \
  "mir_json_surface=metadata.local_i64_map_entry_value_tracking_plans" \
  "entry_tracking_owner=MapStoragePlan" \
  "tracked_receiver_value=1" \
  "tracked_set_site=1" \
  "tracked_key_value=1" \
  "tracked_value_value=1" \
  "tracked_key_const_if_known=1" \
  "tracked_value_const_if_known=1" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "new_runtime_helper_enabled=0" \
  "backend_loader_enabled=0" \
  "backend_lowering_enabled=0" \
  "helper_emission_changed=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-BACKEND-LOADER-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for pattern in \
  "pub struct LocalI64MapEntryValueTrackingPlan" \
  "build_local_i64_map_entry_value_tracking_plans" \
  "set_route_key_value_operands" \
  "value_const_if_known" \
  "backend_lowering_enabled: false" \
  "runtime_helper_enabled: false"; do
  grep -F -q "$pattern" "$MAP_PLAN" || {
    echo "[$TAG] map plan missing pattern: $pattern" >&2
    exit 1
  }
done

grep -F -q "pub local_i64_map_entry_value_tracking_plans: Vec<LocalI64MapEntryValueTrackingPlan>" "$METADATA" || {
  echo "[$TAG] metadata missing entry value tracking field" >&2
  exit 1
}

for pattern in \
  '"local_i64_map_entry_value_tracking_plans"' \
  '"set_block"' \
  '"set_instruction_index"' \
  '"key_value"' \
  '"value_value"' \
  '"key_const_if_known"' \
  '"value_const_if_known"'; do
  grep -F -q "$pattern" "$JSON_EMIT" || {
    echo "[$TAG] json emitter missing pattern: $pattern" >&2
    exit 1
  }
done

grep -F -q "build_mir_json_root_emits_local_i64_map_entry_value_tracking_plans" "$JSON_TEST" || {
  echo "[$TAG] json test missing entry value tracking test" >&2
  exit 1
}

if [[ ! -f "$SUPERSEDING_CARD" ]] || ! grep -q '^Status: Landed$' "$SUPERSEDING_CARD"; then
  if grep -R -n "local_i64_map_entry_value_tracking_plans_by_receiver" src/llvm_py >/tmp/local_i64_entry_tracking_backend_loader_hits.$$; then
    cat /tmp/local_i64_entry_tracking_backend_loader_hits.$$ >&2
    rm -f /tmp/local_i64_entry_tracking_backend_loader_hits.$$
    echo "[$TAG] backend loader must remain disabled until 296x-920 lands" >&2
    exit 1
  fi
fi
rm -f /tmp/local_i64_entry_tracking_backend_loader_hits.$$

grep -F -q "next_task=LOCAL-I64-MAP-ENTRY-VALUE-TRACKING-SURFACE-001" "$PREV_CARD" || {
  echo "[$TAG] previous enablement design card does not hand off to entry tracking surface" >&2
  exit 1
}

echo "[$TAG] ok"
