#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-800-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-799-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-DESIGN-001.md"
SOURCE="src/array_receiver_representation_source.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_constructor_handoff_consumer_implementation_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation] missing card: $CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$SOURCE" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation] missing source module: $SOURCE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$DESIGN_CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation] design card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-consumer-implementation-v0" \
  "source_evidence=296x-799,296x-798,296x-797,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "implementation_module=src/array_receiver_representation_source.rs" \
  "residence_input_source_kind_defined=1" \
  "residence_input_source_struct_defined=1" \
  "residence_source_constructor_defined=1" \
  "constructor_input=ArrayReceiverConstructorHandoff" \
  "constructor_output=ArrayReceiverResidenceInputSource|none" \
  "constructor_mode=fallback_only" \
  "constructor_accepts_fallback_residence_candidate=1" \
  "constructor_accepts_direct_residence_candidate=0" \
  "constructor_preserves_public_arraybox_fallback=1" \
  "constructor_output_direct_storage_proof=0" \
  "constructor_output_backend_bypass_authorized=0" \
  "constructor_report_fields_defined=1" \
  "source_connected_to_constructor=1" \
  "source_exported_to_mir_json=0" \
  "source_consumed_by_backend=0" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=1" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-INPUT-SOURCE-CONSUMER-INVENTORY-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "pub enum ArrayReceiverResidenceInputSourceKind" \
  "pub struct ArrayReceiverResidenceInputSource" \
  "pub struct ArrayReceiverResidenceSourceConstructor" \
  "pub fn construct(" \
  "pub fn array_receiver_residence_source_constructor_report_fields" \
  "ArrayReceiverResidenceInputSourceKind::PublicArrayBoxFallback" \
  "(\"constructor_mode\", \"fallback_only\")" \
  "(\"constructor_accepts_direct_residence_candidate\", \"0\")" \
  "(\"constructor_output_direct_storage_proof\", \"0\")" \
  "(\"constructor_output_backend_bypass_authorized\", \"0\")" \
  "(\"source_connected_to_constructor\", \"1\")" \
  "(\"source_consumed_by_backend\", \"0\")" \
  "(\"backend_direct_handle_bypass_enabled\", \"0\")"; do
  grep -F -q "$expected" "$SOURCE" || {
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation] missing source token: $expected" >&2
    exit 1
  }
done

grep -F -q "do not implement backend direct handle bypass from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation] missing backend bypass stop line" >&2
  exit 1
}
grep -F -q "do not export ArrayReceiverResidenceInputSource to MIR JSON from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation] missing MIR JSON stop line" >&2
  exit 1
}
grep -F -q "do not move Box/Object management into MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation] missing MIRBuilder stop line" >&2
  exit 1
}

echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation] ok"
