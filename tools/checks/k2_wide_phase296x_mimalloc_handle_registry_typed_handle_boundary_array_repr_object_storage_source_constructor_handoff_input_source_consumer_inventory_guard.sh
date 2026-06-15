#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-801-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-INPUT-SOURCE-CONSUMER-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-800-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-001.md"
SOURCE="src/array_receiver_representation_source.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_constructor_handoff_input_source_consumer_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$SOURCE" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory] missing source module: $SOURCE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory-v0" \
  "source_evidence=296x-800,296x-786,296x-783,296x-780,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "input_source_input=ArrayReceiverResidenceInputSource" \
  "input_source_kind=public_arraybox_fallback" \
  "input_source_direct_storage_proof=0" \
  "input_source_backend_bypass_authorized=0" \
  "input_source_consumer_candidate_count=1" \
  "input_source_consumer_candidate_0=ArrayReceiverResidenceInput" \
  "input_source_consumer_candidate_0_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr" \
  "input_source_consumer_candidate_0_status=design_only" \
  "input_source_consumer_candidate_0_code_exists=0" \
  "input_source_consumer_candidate_0_accepts_fallback_source=1" \
  "input_source_consumer_candidate_0_may_emit=ArrayReceiverResidenceInput" \
  "input_source_consumer_candidate_0_must_preserve_public_arraybox_fallback=1" \
  "input_source_safe_to_connect_as_fallback_source=1" \
  "input_source_safe_to_connect_as_direct_residence_proof=0" \
  "input_source_safe_to_connect_to_backend=0" \
  "input_source_safe_to_export_to_mir_json=0" \
  "input_source_safe_to_enable_direct_handle_bypass=0" \
  "residence_chain_current_shape=RepresentationSource->ConstructorHandoff->InputSource->Input->Fact" \
  "residence_chain_stage_count=5" \
  "residence_chain_status=controlled_but_long" \
  "residence_chain_thinning_needed_before_next_implementation=1" \
  "residence_chain_thinning_goal=collapse_naming_and_owner_facade_without_collapsing_proof_gates" \
  "selected_consumer=ArrayReceiverResidenceInput" \
  "selected_consumer_confidence=medium" \
  "selected_consumer_mode=fallback_only" \
  "selected_next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-DESIGN-001" \
  "input_connection_allowed=0" \
  "producer_fact_connection_allowed=0" \
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
  "ArrayReceiverResidenceInputSource" \
  "ArrayReceiverResidenceSourceConstructor" \
  "\"constructor_output\"" \
  "\"ArrayReceiverResidenceInputSource|none\"" \
  "(\"source_connected_to_constructor\", \"1\")" \
  "(\"source_consumed_by_backend\", \"0\")" \
  "(\"backend_direct_handle_bypass_enabled\", \"0\")"; do
  grep -F -q "$expected" "$SOURCE" || {
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory] missing source invariant: $expected" >&2
    exit 1
  }
done

grep -F -q "do not implement ArrayReceiverResidenceInput from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory] missing input stop line" >&2
  exit 1
}
grep -F -q "do not collapse proof gates without preserving report fields" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory] missing proof-gate stop line" >&2
  exit 1
}
grep -F -q "do not move Box/Object management into MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory] missing MIRBuilder stop line" >&2
  exit 1
}

echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-input-source-consumer-inventory] ok"
