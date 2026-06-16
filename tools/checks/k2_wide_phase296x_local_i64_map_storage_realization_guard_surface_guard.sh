#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="local-i64-map-storage-realization-guard-surface"
CARD="docs/development/current/main/phases/phase-296x/296x-906-LOCAL-I64-MAP-STORAGE-REALIZATION-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-905-LOCAL-I64-MAP-STORAGE-REALIZATION-DESIGN-001.md"
PLAN="src/mir/map_repr_plan.rs"
METADATA="src/mir/function/metadata.rs"
JSON_EMIT="src/runner/mir_json_emit/plan_metadata.rs"
JSON_TEST="src/runner/mir_json_emit/tests/map_repr_plans.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_local_i64_map_storage_realization_guard_surface_guard.sh"

for file in "$CARD" "$PREV_CARD" "$PLAN" "$METADATA" "$JSON_EMIT" "$JSON_TEST" "$INDEX"; do
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
  "output_contract=hako-local-i64-map-storage-realization-guard-surface-v0" \
  "source_evidence=296x-905" \
  "row_kind=passive_plan_surface" \
  "target_front=kilo_leaf_map_get_dynamic_covered_i64" \
  "plan_surface=FunctionMetadata.local_map_storage_realization_plans" \
  "plan_struct=LocalMapStorageRealizationPlan" \
  "plan_owner=src/mir/map_repr_plan.rs" \
  "json_export_enabled=1" \
  "json_field=local_map_storage_realization_plans" \
  "representation=local_i64_key_map" \
  "publication_materialization_required=1" \
  "backend_lowering_enabled=0" \
  "runtime_helper_enabled=0" \
  "product_mapbox_storage_changed=0" \
  "product_hasher_swap=0" \
  "sidecar_storage=0" \
  "mirbuilder_map_storage_ownership=0" \
  "winner_claim=0" \
  "next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-LOWERING-DESIGN-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

for text in \
  "pub struct LocalMapStorageRealizationPlan" \
  "publication_materialization_required: true" \
  "backend_lowering_enabled: false" \
  "runtime_helper_enabled: false" \
  "build_local_map_storage_realization_plans"; do
  grep -F -q "$text" "$PLAN" || {
    echo "[$TAG] missing plan evidence: $text" >&2
    exit 1
  }
done

grep -F -q "pub local_map_storage_realization_plans: Vec<LocalMapStorageRealizationPlan>" "$METADATA" || {
  echo "[$TAG] FunctionMetadata missing local_map_storage_realization_plans" >&2
  exit 1
}

for text in \
  '"local_map_storage_realization_plans"' \
  '"representation": plan.representation()' \
  '"publication_materialization_required": plan.publication_materialization_required()' \
  '"backend_lowering_enabled": plan.backend_lowering_enabled()' \
  '"runtime_helper_enabled": plan.runtime_helper_enabled()'; do
  grep -F -q "$text" "$JSON_EMIT" || {
    echo "[$TAG] missing JSON export evidence: $text" >&2
    exit 1
  }
done

grep -F -q "build_mir_json_root_emits_local_map_storage_realization_plans" "$JSON_TEST" || {
  echo "[$TAG] missing JSON test" >&2
  exit 1
}

grep -F -q "next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-GUARD-SURFACE-001" "$PREV_CARD" || {
  echo "[$TAG] previous design card does not hand off to guard surface" >&2
  exit 1
}

echo "[$TAG] ok"
