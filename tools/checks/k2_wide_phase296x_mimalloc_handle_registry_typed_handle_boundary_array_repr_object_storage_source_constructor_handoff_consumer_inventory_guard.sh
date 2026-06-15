#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-798-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-797-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-IMPLEMENTATION-001.md"
SOURCE="src/array_receiver_representation_source.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_constructor_handoff_consumer_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$SOURCE" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] missing source module: $SOURCE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-consumer-inventory-v0" \
  "source_evidence=296x-797,296x-796,296x-791,296x-789,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "handoff_input=ArrayReceiverConstructorHandoff" \
  "handoff_input_kind=fallback_residence_candidate" \
  "handoff_input_array_repr=PublicArrayBoxFallback" \
  "handoff_input_direct_storage_proof=0" \
  "handoff_input_backend_bypass_authorized=0" \
  "handoff_consumer_candidate_count=1" \
  "handoff_consumer_candidate_0=ArrayReceiverResidenceSourceConstructor" \
  "handoff_consumer_candidate_0_owner=RepresentationPlanner|ArrayReprSourcePlanner" \
  "handoff_consumer_candidate_0_status=design_only" \
  "handoff_consumer_candidate_0_code_exists=0" \
  "handoff_consumer_candidate_0_accepts_fallback_candidate=1" \
  "handoff_consumer_candidate_0_may_emit=ArrayReceiverResidenceInputSource" \
  "handoff_consumer_candidate_0_must_preserve_public_arraybox_fallback=1" \
  "handoff_safe_to_connect_as_fallback_candidate=1" \
  "handoff_safe_to_connect_as_direct_residence_proof=0" \
  "handoff_safe_to_connect_to_backend=0" \
  "handoff_safe_to_export_to_mir_json=0" \
  "handoff_safe_to_enable_direct_handle_bypass=0" \
  "selected_consumer=ArrayReceiverResidenceSourceConstructor" \
  "selected_consumer_confidence=medium" \
  "selected_consumer_mode=fallback_only" \
  "selected_next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-DESIGN-001" \
  "constructor_connection_allowed=0" \
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
  "ArrayReceiverConstructorHandoff" \
  "FallbackResidenceCandidate" \
  "(\"handoff_output_direct_storage_proof\", \"0\")" \
  "(\"handoff_output_backend_bypass_authorized\", \"0\")" \
  "(\"source_connected_to_constructor\", \"0\")" \
  "(\"backend_direct_handle_bypass_enabled\", \"0\")"; do
  grep -F -q "$expected" "$SOURCE" || {
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] missing source invariant: $expected" >&2
    exit 1
  }
done

grep -F -q "do not connect constructor handoff to ArrayReceiverResidenceSourceConstructor from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] missing constructor stop line" >&2
  exit 1
}
grep -F -q "do not implement backend direct handle bypass from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] missing backend bypass stop line" >&2
  exit 1
}
grep -F -q "do not treat PublicArrayBoxFallback as direct storage proof" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] missing fallback-not-direct stop line" >&2
  exit 1
}
grep -F -q "do not move Box/Object management into MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] missing MIRBuilder stop line" >&2
  exit 1
}

echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-inventory] ok"
