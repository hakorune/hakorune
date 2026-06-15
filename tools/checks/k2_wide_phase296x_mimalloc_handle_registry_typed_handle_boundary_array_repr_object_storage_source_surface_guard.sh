#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-793-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-SURFACE-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-792-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-DESIGN-001.md"
CONSTRUCTOR_INVENTORY_CARD="docs/development/current/main/phases/phase-296x/296x-791-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-INVENTORY-001.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
DIRECT_PLAN="src/mir/direct_array_access_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-surface] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$CONSTRUCTOR_INVENTORY_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-surface] missing constructor inventory card: $CONSTRUCTOR_INVENTORY_CARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-repr-object-storage-source-surface] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-repr-object-storage-source-surface] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }
[[ -f "$DIRECT_PLAN" ]] || { echo "[mimalloc-array-repr-object-storage-source-surface] missing DirectArrayAccessPlan source: $DIRECT_PLAN" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-surface] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$DESIGN_CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-surface] design card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-repr-object-storage-source-surface] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-repr-object-storage-source-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-surface-v0" \
  "source_evidence=296x-792,296x-791,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "array_receiver_representation_source_surface_defined=1" \
  "representation_source_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr|none" \
  "representation_source_output=ArrayReceiverRepresentationSource|none" \
  "representation_source_scope=receiver_site_before_length_read" \
  "representation_source_consumed_by=ArrayReceiverResidenceSourceConstructor" \
  "representation_source_route_kind=array_slot_len" \
  "representation_source_receiver_box_name=ArrayBox" \
  "representation_source_array_repr=DirectI64|PublicArrayBoxFallback|none" \
  "representation_source_object_storage_plan_ref=<id|none>" \
  "representation_source_direct_array_access_plan_ref=<id|none>" \
  "representation_source_materialization_route=public_arraybox_fallback|snapshot|none" \
  "representation_source_confidence=low|medium|high" \
  "representation_source_may_provide_array_repr=1" \
  "representation_source_may_provide_object_storage_plan=1" \
  "representation_source_may_reference_direct_array_access_plan=1" \
  "representation_source_is_direct_array_access_plan_only=0" \
  "representation_source_preserves_public_arraybox_fallback=1" \
  "representation_source_includes_materialization_route=<0|1>" \
  "representation_source_public_handle_reinterpretation=0" \
  "representation_source_backend_raw_layout_inference=0" \
  "representation_source_helper_name_inference=0" \
  "representation_source_mirbuilder_owner=0" \
  "representation_candidate_count=<n>" \
  "representation_eligible_count=<n>" \
  "representation_rejected_count=<n>" \
  "selected_representation_candidate_count=<n>" \
  "selected_representation_candidate_confidence=low|medium|high" \
  "selected_blocker=<blocker|none>" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-INVENTORY-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "representation_source_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr" \
  "representation_source_output=ArrayReceiverRepresentationSource" \
  "representation_source_scope=receiver_site_before_length_read" \
  "representation_source_consumed_by=ArrayReceiverResidenceSourceConstructor" \
  "representation_source_route_kind=array_slot_len" \
  "representation_source_receiver_box_name=ArrayBox" \
  "representation_source_may_provide_array_repr=1" \
  "representation_source_may_provide_object_storage_plan=1" \
  "representation_source_may_reference_direct_array_access_plan=1" \
  "representation_source_is_direct_array_access_plan_only=0" \
  "representation_source_preserves_public_arraybox_fallback=1" \
  "representation_source_includes_materialization_route=1" \
  "representation_source_public_handle_reinterpretation=0" \
  "representation_source_backend_raw_layout_inference=0" \
  "representation_source_helper_name_inference=0" \
  "representation_source_mirbuilder_owner=0" \
  "representation_eligible_count>=1" \
  "selected_representation_candidate_confidence=high"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "representation_source_output=ArrayReceiverRepresentationSource" \
  "representation_source_must_preserve_public_arraybox_fallback=1" \
  "representation_source_must_include_materialization_route=1" \
  "implementation_allowed=0"; do
  require_line_in_file "$DESIGN_CARD" "$expected"
done

require_line_in_file "$CONSTRUCTOR_INVENTORY_CARD" "selected_blocker=missing_array_repr_or_object_storage_constructor_input"
grep -F -q "ArrayRepr" "$ARRAY_REPR" || {
  echo "[mimalloc-array-repr-object-storage-source-surface] ArrayRepr SSOT missing ArrayRepr" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"
grep -F -q "DirectArrayAccessPlan" "$DIRECT_PLAN" || {
  echo "[mimalloc-array-repr-object-storage-source-surface] DirectArrayAccessPlan source missing type" >&2
  exit 1
}

grep -F -q "do not implement ArrayReceiverRepresentationSource from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-surface] missing source implementation stop line" >&2
  exit 1
}
grep -F -q "do not implement backend direct handle bypass from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-surface] missing backend bypass stop line" >&2
  exit 1
}
grep -F -q "do not move Box/Object management into MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-surface] missing MIRBuilder stop line" >&2
  exit 1
}

echo "[mimalloc-array-repr-object-storage-source-surface] ok"
