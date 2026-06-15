#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-797-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-IMPLEMENTATION-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-796-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-DESIGN-001.md"
SOURCE="src/array_receiver_representation_source.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_constructor_handoff_implementation_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] missing card: $CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$SOURCE" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] missing source module: $SOURCE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$DESIGN_CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] design card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-implementation-v0" \
  "source_evidence=296x-796,296x-795,296x-794,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "handoff_module=src/array_receiver_representation_source.rs" \
  "handoff_kind_defined=1" \
  "handoff_struct_defined=1" \
  "handoff_input=ArrayReceiverRepresentationSource" \
  "handoff_consumer=ArrayReceiverResidenceSourceConstructor" \
  "handoff_output_kind=fallback_residence_candidate" \
  "handoff_input_array_repr=PublicArrayBoxFallback" \
  "handoff_input_is_fallback_only=1" \
  "handoff_output_direct_storage_proof=0" \
  "handoff_output_backend_bypass_authorized=0" \
  "handoff_materialization_route=public_arraybox_fallback" \
  "handoff_preserves_public_arraybox_fallback=1" \
  "handoff_report_fields_defined=1" \
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
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-INVENTORY-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "pub enum ArrayReceiverConstructorHandoffKind" \
  "pub struct ArrayReceiverConstructorHandoff" \
  "pub fn constructor_handoff(&self) -> ArrayReceiverConstructorHandoff" \
  "pub const fn is_fallback_residence_candidate(&self) -> bool" \
  "pub const fn authorizes_backend_direct_handle_bypass(&self) -> bool" \
  "pub fn array_receiver_constructor_handoff_report_fields"; do
  grep -F -q "$expected" "$SOURCE" || {
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] missing source token: $expected" >&2
    exit 1
  }
done

for expected in \
  "FallbackResidenceCandidate" \
  "DirectResidenceCandidate" \
  "fallback_residence_candidate" \
  "(\"handoff_output_direct_storage_proof\", \"0\")" \
  "(\"handoff_output_backend_bypass_authorized\", \"0\")" \
  "(\"source_connected_to_constructor\", \"0\")" \
  "(\"backend_direct_handle_bypass_enabled\", \"0\")" \
  "(\"mirbuilder_object_management_enabled\", \"0\")"; do
  grep -F -q "$expected" "$SOURCE" || {
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] missing invariant: $expected" >&2
    exit 1
  }
done

grep -F -q "do not connect constructor handoff to ArrayReceiverResidenceSourceConstructor from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] missing constructor stop line" >&2
  exit 1
}
grep -F -q "do not implement backend direct handle bypass from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] missing backend bypass stop line" >&2
  exit 1
}
grep -F -q "do not move Box/Object management into MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] missing MIRBuilder stop line" >&2
  exit 1
}

echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-implementation] ok"
