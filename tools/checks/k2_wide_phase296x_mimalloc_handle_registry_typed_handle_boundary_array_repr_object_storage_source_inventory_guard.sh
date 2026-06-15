#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-794-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-INVENTORY-001.md"
SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-793-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-SURFACE-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-792-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-DESIGN-001.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
DIRECT_PLAN="src/mir/direct_array_access_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$SURFACE_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-inventory] missing surface card: $SURFACE_CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-inventory] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-repr-object-storage-source-inventory] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-repr-object-storage-source-inventory] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }
[[ -f "$DIRECT_PLAN" ]] || { echo "[mimalloc-array-repr-object-storage-source-inventory] missing DirectArrayAccessPlan source: $DIRECT_PLAN" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-inventory] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$SURFACE_CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-inventory] surface card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-repr-object-storage-source-inventory] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-repr-object-storage-source-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-inventory-v0" \
  "source_evidence=296x-793,296x-792,296x-791,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "array_receiver_representation_source_surface_defined=1" \
  "representation_source_owner=ArrayRepr" \
  "representation_source_output=ArrayReceiverRepresentationSource" \
  "representation_source_scope=receiver_site_before_length_read" \
  "representation_source_consumed_by=ArrayReceiverResidenceSourceConstructor" \
  "representation_source_route_kind=array_slot_len" \
  "representation_source_receiver_box_name=ArrayBox" \
  "representation_source_array_repr=PublicArrayBoxFallback" \
  "representation_source_object_storage_plan_ref=none" \
  "representation_source_direct_array_access_plan_ref=none" \
  "representation_source_materialization_route=public_arraybox_fallback" \
  "representation_source_confidence=high" \
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
  "representation_candidate_count=2" \
  "representation_eligible_count=1" \
  "representation_rejected_count=1" \
  "selected_representation_candidate_count=1" \
  "selected_representation_candidate_confidence=high" \
  "selected_representation_candidate=public_arraybox_fallback_source" \
  "selected_blocker=none" \
  "direct_representation_candidate_count=1" \
  "direct_representation_eligible_count=0" \
  "direct_array_repr_available=0" \
  "direct_object_storage_plan_available=0" \
  "direct_array_access_plan_sufficient=0" \
  "direct_representation_selected=0" \
  "source_implementation_may_emit_fallback_only=1" \
  "source_implementation_must_not_enable_backend_bypass=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-IMPLEMENTATION-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "representation_source_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr" \
  "representation_source_output=ArrayReceiverRepresentationSource" \
  "representation_source_includes_materialization_route=1" \
  "representation_eligible_count>=1" \
  "selected_representation_candidate_confidence=high"; do
  require_line_in_file "$SURFACE_CARD" "$expected"
done

for expected in \
  "representation_source_output=ArrayReceiverRepresentationSource" \
  "representation_source_must_preserve_public_arraybox_fallback=1" \
  "implementation_allowed=0"; do
  require_line_in_file "$DESIGN_CARD" "$expected"
done

grep -F -q "PublicArrayBoxFallback" "$ARRAY_REPR" || {
  echo "[mimalloc-array-repr-object-storage-source-inventory] ArrayRepr SSOT missing PublicArrayBoxFallback" >&2
  exit 1
}
grep -F -q "DirectI64" "$ARRAY_REPR" || {
  echo "[mimalloc-array-repr-object-storage-source-inventory] ArrayRepr SSOT missing DirectI64" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"
grep -F -q "DirectArrayAccessPlan" "$DIRECT_PLAN" || {
  echo "[mimalloc-array-repr-object-storage-source-inventory] DirectArrayAccessPlan source missing type" >&2
  exit 1
}

grep -F -q "do not treat PublicArrayBoxFallback as direct storage proof" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-inventory] missing fallback-not-direct stop line" >&2
  exit 1
}
grep -F -q "do not implement backend direct handle bypass from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-inventory] missing backend bypass stop line" >&2
  exit 1
}
grep -F -q "do not move Box/Object management into MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-inventory] missing MIRBuilder stop line" >&2
  exit 1
}

echo "[mimalloc-array-repr-object-storage-source-inventory] ok"
