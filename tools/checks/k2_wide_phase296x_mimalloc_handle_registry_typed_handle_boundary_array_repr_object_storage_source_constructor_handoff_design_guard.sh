#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-796-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-DESIGN-001.md"
SOURCE_CARD="docs/development/current/main/phases/phase-296x/296x-795-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-IMPLEMENTATION-001.md"
SOURCE="src/array_receiver_representation_source.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_constructor_handoff_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$SOURCE_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] missing source card: $SOURCE_CARD" >&2; exit 1; }
[[ -f "$SOURCE" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] missing source module: $SOURCE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$SOURCE_CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] source implementation card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-design-v0" \
  "source_evidence=296x-795,296x-794,296x-793,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "selected_design=fallback_source_constructor_handoff" \
  "selected_design_confidence=high" \
  "handoff_input=ArrayReceiverRepresentationSource" \
  "handoff_input_array_repr=PublicArrayBoxFallback" \
  "handoff_input_is_fallback_only=1" \
  "handoff_consumer=ArrayReceiverResidenceSourceConstructor" \
  "handoff_output_kind=fallback_residence_candidate" \
  "handoff_output_direct_storage_proof=0" \
  "handoff_output_backend_bypass_authorized=0" \
  "handoff_materialization_route=public_arraybox_fallback" \
  "handoff_preserves_public_arraybox_fallback=1" \
  "handoff_requires_direct_source_for_bypass=1" \
  "handoff_accepts_fallback_source=1" \
  "handoff_rejects_fallback_as_direct_source=1" \
  "source_connected_to_constructor=0" \
  "source_exported_to_mir_json=0" \
  "source_consumed_by_backend=0" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-SURFACE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "ArrayReceiverRepresentationSource::public_arraybox_fallback():" \
  "representation_source_array_repr=PublicArrayBoxFallback" \
  "representation_source_is_fallback_only=1" \
  "representation_source_proves_direct_storage=0" \
  "backend_direct_handle_bypass_enabled=0"; do
  grep -F -q "$expected" "$SOURCE_CARD" || {
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] missing source-card evidence: $expected" >&2
    exit 1
  }
done

for expected in \
  "pub struct ArrayReceiverRepresentationSource" \
  "pub const fn public_arraybox_fallback() -> Self" \
  "pub const fn is_fallback_only(&self) -> bool" \
  "pub const fn proves_direct_storage(&self) -> bool" \
  "pub const fn authorizes_backend_direct_handle_bypass(&self) -> bool"; do
  grep -F -q "$expected" "$SOURCE" || {
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] missing source token: $expected" >&2
    exit 1
  }
done

grep -F -q "fallback residence candidate:" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] missing fallback candidate contract" >&2
  exit 1
}
grep -F -q "direct residence candidate:" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] missing direct candidate contract" >&2
  exit 1
}
grep -F -q "do not treat PublicArrayBoxFallback as direct storage proof" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] missing fallback-not-direct stop line" >&2
  exit 1
}
grep -F -q "do not implement backend direct handle bypass from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] missing backend bypass stop line" >&2
  exit 1
}
grep -F -q "do not move Box/Object management into MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] missing MIRBuilder stop line" >&2
  exit 1
}

echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-design] ok"
