#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-792-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-DESIGN-001.md"
CONSTRUCTOR_INVENTORY_CARD="docs/development/current/main/phases/phase-296x/296x-791-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-INVENTORY-001.md"
CONSTRUCTOR_SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-790-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-SURFACE-001.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
DIRECT_PLAN="src/mir/direct_array_access_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$CONSTRUCTOR_INVENTORY_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-design] missing constructor inventory card: $CONSTRUCTOR_INVENTORY_CARD" >&2; exit 1; }
[[ -f "$CONSTRUCTOR_SURFACE_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-design] missing constructor surface card: $CONSTRUCTOR_SURFACE_CARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-repr-object-storage-source-design] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-repr-object-storage-source-design] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }
[[ -f "$DIRECT_PLAN" ]] || { echo "[mimalloc-array-repr-object-storage-source-design] missing DirectArrayAccessPlan source: $DIRECT_PLAN" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$CONSTRUCTOR_INVENTORY_CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-design] constructor inventory card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-repr-object-storage-source-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-repr-object-storage-source-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-design-v0" \
  "source_evidence=296x-791,296x-790,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "selected_design=array_receiver_representation_source" \
  "selected_design_confidence=medium" \
  "representation_source_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr" \
  "representation_source_output=ArrayReceiverRepresentationSource" \
  "representation_source_scope=receiver_site_before_length_read" \
  "representation_source_consumed_by=ArrayReceiverResidenceSourceConstructor" \
  "representation_source_may_provide_array_repr=1" \
  "representation_source_may_provide_object_storage_plan=1" \
  "representation_source_may_reference_direct_array_access_plan=1" \
  "representation_source_must_not_be_direct_array_access_plan_only=1" \
  "representation_source_must_preserve_public_arraybox_fallback=1" \
  "representation_source_must_include_materialization_route=1" \
  "representation_source_must_not_reinterpret_public_arraybox_handle=1" \
  "representation_source_must_not_infer_backend_raw_layout=1" \
  "representation_source_must_not_use_helper_name=1" \
  "representation_source_must_not_run_in_mirbuilder=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-SURFACE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "array_repr_or_object_storage_constructor_input_available=0" \
  "array_repr_input_available=0" \
  "object_storage_plan_input_available=0" \
  "selected_blocker=missing_array_repr_or_object_storage_constructor_input"; do
  require_line_in_file "$CONSTRUCTOR_INVENTORY_CARD" "$expected"
done

require_line_in_file "$CONSTRUCTOR_SURFACE_CARD" "constructor_required_input_array_repr_or_object_storage=<0|1>"
grep -F -q "ArrayRepr" "$ARRAY_REPR" || {
  echo "[mimalloc-array-repr-object-storage-source-design] ArrayRepr SSOT missing ArrayRepr" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"
grep -F -q "DirectArrayAccessPlan" "$DIRECT_PLAN" || {
  echo "[mimalloc-array-repr-object-storage-source-design] DirectArrayAccessPlan source missing type" >&2
  exit 1
}

grep -F -q "PublicArrayBoxFallback" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-design] missing fallback source decision" >&2
  exit 1
}
grep -F -q "do not implement the representation source from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-design] missing representation-source stop line" >&2
  exit 1
}

echo "[mimalloc-array-repr-object-storage-source-design] ok"
