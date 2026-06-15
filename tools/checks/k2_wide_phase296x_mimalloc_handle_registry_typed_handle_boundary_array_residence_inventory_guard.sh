#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-779-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-INVENTORY-001.md"
SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-778-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-SURFACE-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-777-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-DESIGN-001.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_residence_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$SURFACE_CARD" ]] || { echo "[mimalloc-array-residence-inventory] missing surface card: $SURFACE_CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-array-residence-inventory] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$TASKBOARD" ]] || { echo "[mimalloc-array-residence-inventory] missing DirectArray taskboard: $TASKBOARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-residence-inventory] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-residence-inventory] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-inventory] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$SURFACE_CARD" || {
  echo "[mimalloc-array-residence-inventory] surface card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-inventory] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-inventory-v0" \
  "source_evidence=296x-778,296x-777,array-repr-ssot,object-storage-plan-boundary-ssot,directarray-next-order-taskboard" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "array_receiver_residence_surface_defined=1" \
  "array_receiver_route_kind=array_slot_len" \
  "array_receiver_box_name=ArrayBox" \
  "array_receiver_residence_owner=none" \
  "array_receiver_residence=none" \
  "array_receiver_direct_facts_source=none" \
  "array_receiver_direct_facts_proven=0" \
  "array_receiver_materialization_route_known=0" \
  "array_receiver_materialization_route=none" \
  "array_receiver_public_handle_reinterpreted=0" \
  "array_receiver_backend_raw_layout_inference=0" \
  "array_receiver_route_proof_as_storage_proof=0" \
  "array_receiver_host_handle_publication_before_read=1" \
  "array_receiver_fallback_public_arraybox=1" \
  "residence_candidate_count=1" \
  "residence_eligible_count=0" \
  "residence_rejected_count=1" \
  "selected_residence_candidate_count=0" \
  "selected_residence_candidate_confidence=low" \
  "selected_blocker=array_receiver_residence_missing" \
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
  "selected_decision=reject_backend_direct_handle_bypass_until_array_residence_producer_exists" \
  "route_proof_available=1" \
  "residence_proof_available=0" \
  "array_residence_producer_required=1" \
  "arrayrepr_or_object_storage_plan_required=1" \
  "public_arraybox_handle_reinterpretation_allowed=0" \
  "backend_raw_arraybox_layout_truth=0" \
  "route_proof_as_storage_proof_allowed=0" \
  "fallback_to_public_arraybox_host_handle_required=1" \
  "fallback_to_generic_host_handle_required=1" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-DESIGN-001"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$SURFACE_CARD" "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-INVENTORY-001"
require_line_in_file "$SURFACE_CARD" "array_receiver_residence_owner=ObjectStoragePlan|ArrayRepr|none"
require_line_in_file "$SURFACE_CARD" "array_receiver_residence=direct_array|exact_native_struct|scalarized|public_arraybox_fallback|none"
require_line_in_file "$DESIGN_CARD" "selected_design=array_receiver_residence_proof_via_arrayrepr"

grep -F -q "DirectI64 ArrayRepr Producer Contract" "$TASKBOARD" || {
  echo "[mimalloc-array-residence-inventory] DirectArray taskboard missing producer contract row" >&2
  exit 1
}
grep -F -q "DirectI64" "$ARRAY_REPR" || {
  echo "[mimalloc-array-residence-inventory] ArrayRepr SSOT lacks DirectI64" >&2
  exit 1
}
grep -F -q "PublicArrayBoxFallback" "$ARRAY_REPR" || {
  echo "[mimalloc-array-residence-inventory] ArrayRepr SSOT lacks PublicArrayBoxFallback" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"
require_line_in_file "$OBJECT_SSOT" "backend_consumes_object_storage_plan=1"

grep -F -q "do not treat array_slot_len route proof as receiver storage proof" "$CARD" || {
  echo "[mimalloc-array-residence-inventory] missing route/storage stop line" >&2
  exit 1
}

echo "[mimalloc-array-residence-inventory] ok"
