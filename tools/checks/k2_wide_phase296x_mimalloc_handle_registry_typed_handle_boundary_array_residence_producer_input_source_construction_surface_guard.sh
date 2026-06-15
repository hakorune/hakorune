#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-790-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-SURFACE-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-789-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-DESIGN-001.md"
SOURCE_INVENTORY_CARD="docs/development/current/main/phases/phase-296x/296x-788-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-INVENTORY-001.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
DIRECT_PLAN="src/mir/direct_array_access_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_residence_producer_input_source_construction_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-producer-input-source-construction-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-array-residence-producer-input-source-construction-surface] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$SOURCE_INVENTORY_CARD" ]] || { echo "[mimalloc-array-residence-producer-input-source-construction-surface] missing source inventory card: $SOURCE_INVENTORY_CARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-residence-producer-input-source-construction-surface] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-residence-producer-input-source-construction-surface] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }
[[ -f "$DIRECT_PLAN" ]] || { echo "[mimalloc-array-residence-producer-input-source-construction-surface] missing DirectArrayAccessPlan source: $DIRECT_PLAN" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-source-construction-surface] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$DESIGN_CARD" || {
  echo "[mimalloc-array-residence-producer-input-source-construction-surface] design card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-producer-input-source-construction-surface] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-producer-input-source-construction-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-source-construction-surface-v0" \
  "source_evidence=296x-789,296x-788,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "array_receiver_residence_source_constructor_surface_defined=1" \
  "constructor_owner=RepresentationPlanner|ArrayReprSourcePlanner|none" \
  "constructor_output=ArrayReceiverResidenceInputSource|none" \
  "constructor_scope=receiver_site_before_length_read" \
  "constructor_inputs=RoutePlan|escape_publication_facts|materialization_route|ArrayRepr|ObjectStoragePlan|DirectArrayAccessPlan" \
  "constructor_required_input_routeplan=<0|1>" \
  "constructor_required_input_escape_publication=<0|1>" \
  "constructor_required_input_materialization_route=<0|1>" \
  "constructor_required_input_array_repr_or_object_storage=<0|1>" \
  "constructor_optional_input_direct_array_access_plan=1" \
  "constructor_uses_direct_array_access_plan_only=0" \
  "constructor_reinterprets_public_arraybox_handle=0" \
  "constructor_backend_raw_layout_inference=0" \
  "constructor_helper_name_inference=0" \
  "constructor_mirbuilder_owner=0" \
  "constructor_preserves_public_arraybox_fallback=1" \
  "constructor_runtime_execution=0" \
  "constructor_candidate_count=<n>" \
  "constructor_eligible_count=<n>" \
  "constructor_rejected_count=<n>" \
  "selected_constructor_candidate_count=<n>" \
  "selected_constructor_candidate_confidence=low|medium|high" \
  "selected_blocker=<blocker|none>" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "constructor_owner=RepresentationPlanner|ArrayReprSourcePlanner" \
  "constructor_output=ArrayReceiverResidenceInputSource" \
  "constructor_scope=receiver_site_before_length_read" \
  "constructor_required_input_routeplan=1" \
  "constructor_required_input_escape_publication=1" \
  "constructor_required_input_materialization_route=1" \
  "constructor_required_input_array_repr_or_object_storage=1" \
  "constructor_optional_input_direct_array_access_plan=1" \
  "constructor_uses_direct_array_access_plan_only=0" \
  "constructor_reinterprets_public_arraybox_handle=0" \
  "constructor_backend_raw_layout_inference=0" \
  "constructor_helper_name_inference=0" \
  "constructor_mirbuilder_owner=0" \
  "constructor_preserves_public_arraybox_fallback=1" \
  "constructor_runtime_execution=0" \
  "constructor_eligible_count>=1" \
  "selected_constructor_candidate_confidence=high"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$DESIGN_CARD" "selected_design=array_receiver_residence_source_constructor"
require_line_in_file "$DESIGN_CARD" "constructor_must_not_run_in_mirbuilder=1"
require_line_in_file "$SOURCE_INVENTORY_CARD" "selected_blocker=missing_array_repr_or_object_storage_source"
grep -F -q "DirectArrayAccessPlan" "$DIRECT_PLAN" || {
  echo "[mimalloc-array-residence-producer-input-source-construction-surface] DirectArrayAccessPlan source missing type" >&2
  exit 1
}
grep -F -q "ArrayRepr" "$ARRAY_REPR" || {
  echo "[mimalloc-array-residence-producer-input-source-construction-surface] ArrayRepr SSOT missing ArrayRepr" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"

grep -F -q "do not implement the source constructor from this row" "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-source-construction-surface] missing constructor stop line" >&2
  exit 1
}
grep -F -q "input-source execution" "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-source-construction-surface] missing restricted gate wording" >&2
  exit 1
}

echo "[mimalloc-array-residence-producer-input-source-construction-surface] ok"
