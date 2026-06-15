#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-799-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-798-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_constructor_handoff_consumer_implementation_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design-v0" \
  "source_evidence=296x-798,296x-797,296x-790,296x-789,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "selected_design=array_receiver_residence_source_constructor_fallback_consumer" \
  "selected_design_confidence=medium" \
  "consumer_owner=RepresentationPlanner|ArrayReprSourcePlanner" \
  "consumer_input=ArrayReceiverConstructorHandoff" \
  "consumer_output=ArrayReceiverResidenceInputSource|none" \
  "consumer_scope=receiver_site_before_length_read" \
  "consumer_mode=fallback_only" \
  "consumer_accepts_fallback_residence_candidate=1" \
  "consumer_accepts_direct_residence_candidate=0" \
  "consumer_may_emit_fallback_input_source=1" \
  "consumer_may_emit_direct_input_source=0" \
  "consumer_preserves_public_arraybox_fallback=1" \
  "consumer_runtime_execution=0" \
  "required_input_handoff_kind=fallback_residence_candidate" \
  "required_input_materialization_route=public_arraybox_fallback" \
  "required_input_backend_bypass_authorized=0" \
  "required_input_direct_storage_proof=0" \
  "constructor_connection_allowed_next_row=1" \
  "source_exported_to_mir_json=0" \
  "source_consumed_by_backend=0" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-CONSUMER-IMPLEMENTATION-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "reject: consume handoff in backend" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] missing backend rejection" >&2
  exit 1
}
grep -F -q "reject: consume handoff in MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] missing MIRBuilder rejection" >&2
  exit 1
}
grep -F -q "reject: treat fallback handoff as DirectArray proof" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] missing fallback/direct rejection" >&2
  exit 1
}
grep -F -q "do not implement the consumer from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] missing consumer stop line" >&2
  exit 1
}
grep -F -q "do not implement backend direct handle bypass from this row" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] missing backend bypass stop line" >&2
  exit 1
}
grep -F -q "do not move Box/Object management into MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] missing MIRBuilder stop line" >&2
  exit 1
}

echo "[mimalloc-array-repr-object-storage-source-constructor-handoff-consumer-implementation-design] ok"
