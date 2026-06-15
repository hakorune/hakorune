#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-795-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-IMPLEMENTATION-001.md"
INVENTORY_CARD="docs/development/current/main/phases/phase-296x/296x-794-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-INVENTORY-001.md"
SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-793-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-SURFACE-001.md"
SOURCE="src/array_receiver_representation_source.rs"
LIB="src/lib.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_implementation_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-implementation] missing card: $CARD" >&2; exit 1; }
[[ -f "$INVENTORY_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-implementation] missing inventory card: $INVENTORY_CARD" >&2; exit 1; }
[[ -f "$SURFACE_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-implementation] missing surface card: $SURFACE_CARD" >&2; exit 1; }
[[ -f "$SOURCE" ]] || { echo "[mimalloc-array-repr-object-storage-source-implementation] missing source module: $SOURCE" >&2; exit 1; }
[[ -f "$LIB" ]] || { echo "[mimalloc-array-repr-object-storage-source-implementation] missing lib: $LIB" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-implementation] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$INVENTORY_CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-implementation] inventory card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-repr-object-storage-source-implementation] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-repr-object-storage-source-implementation] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-implementation-v0" \
  "source_evidence=296x-794,296x-793,296x-792,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "array_receiver_representation_source_module=src/array_receiver_representation_source.rs" \
  "array_receiver_representation_source_defined=1" \
  "array_receiver_representation_source_exported=1" \
  "representation_source_owner=ArrayRepr" \
  "representation_source_output=ArrayReceiverRepresentationSource" \
  "representation_source_consumed_by=ArrayReceiverResidenceSourceConstructor" \
  "representation_source_route_kind=array_slot_len" \
  "representation_source_receiver_box_name=ArrayBox" \
  "representation_source_array_repr=PublicArrayBoxFallback" \
  "representation_source_object_storage_plan_ref=none" \
  "representation_source_direct_array_access_plan_ref=none" \
  "representation_source_materialization_route=public_arraybox_fallback" \
  "representation_source_confidence=high" \
  "representation_source_is_fallback_only=1" \
  "representation_source_proves_direct_storage=0" \
  "representation_source_authorizes_backend_bypass=0" \
  "representation_source_report_fields_defined=1" \
  "source_connected_to_constructor=0" \
  "source_exported_to_mir_json=0" \
  "source_consumed_by_backend=0" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=1" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-DESIGN-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "source_implementation_may_emit_fallback_only=1" \
  "source_implementation_must_not_enable_backend_bypass=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-IMPLEMENTATION-001"; do
  require_line_in_file "$INVENTORY_CARD" "$expected"
done

for expected in \
  "pub enum ArrayReceiverRepresentationOwner" \
  "pub enum ArrayReceiverArrayRepr" \
  "pub enum ArrayReceiverMaterializationRoute" \
  "pub struct ArrayReceiverRepresentationSource" \
  "pub const fn public_arraybox_fallback() -> Self" \
  "pub const fn is_fallback_only(&self) -> bool" \
  "pub const fn proves_direct_storage(&self) -> bool" \
  "pub const fn authorizes_backend_direct_handle_bypass(&self) -> bool" \
  "pub fn array_receiver_representation_source_report_fields"; do
  grep -F -q "$expected" "$SOURCE" || {
    echo "[mimalloc-array-repr-object-storage-source-implementation] missing source token: $expected" >&2
    exit 1
  }
done

for expected in \
  "ArrayReceiverArrayRepr::PublicArrayBoxFallback" \
  "ArrayReceiverMaterializationRoute::PublicArrayBoxFallback" \
  "RepresentationConfidence::High" \
  "(\"representation_source_array_repr\", \"PublicArrayBoxFallback\")" \
  "(\"representation_source_is_fallback_only\", \"1\")" \
  "(\"representation_source_proves_direct_storage\", \"0\")" \
  "(\"backend_direct_handle_bypass_enabled\", \"0\")" \
  "(\"mirbuilder_object_management_enabled\", \"0\")"; do
  grep -F -q "$expected" "$SOURCE" || {
    echo "[mimalloc-array-repr-object-storage-source-implementation] missing source invariant: $expected" >&2
    exit 1
  }
done

grep -F -q "pub mod array_receiver_representation_source;" "$LIB" || {
  echo "[mimalloc-array-repr-object-storage-source-implementation] lib.rs must export array_receiver_representation_source" >&2
  exit 1
}

grep -F -q "do not connect ArrayReceiverRepresentationSource to backend lowering from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-implementation] missing backend lowering stop line" >&2
  exit 1
}
grep -F -q "do not treat PublicArrayBoxFallback as direct storage proof" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-implementation] missing fallback-not-direct stop line" >&2
  exit 1
}
grep -F -q "do not move Box/Object management into MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-implementation] missing MIRBuilder stop line" >&2
  exit 1
}

echo "[mimalloc-array-repr-object-storage-source-implementation] ok"
