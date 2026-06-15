#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-808-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-IMPLEMENTATION-001.md"
SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-807-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-SURFACE-001.md"
SOURCE="src/array_receiver_representation_source.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_residence_input_consumer_implementation_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-input-consumer-implementation] missing card: $CARD" >&2; exit 1; }
[[ -f "$SURFACE_CARD" ]] || { echo "[mimalloc-array-residence-input-consumer-implementation] missing surface card: $SURFACE_CARD" >&2; exit 1; }
[[ -f "$SOURCE" ]] || { echo "[mimalloc-array-residence-input-consumer-implementation] missing source: $SOURCE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-input-consumer-implementation] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$SURFACE_CARD" || {
  echo "[mimalloc-array-residence-input-consumer-implementation] surface card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-input-consumer-implementation] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-input-consumer-implementation] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-input-consumer-implementation-v0" \
  "source_evidence=296x-807,296x-806,296x-805,296x-804,array-repr-proof-chain-guide" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "implementation_module=src/array_receiver_representation_source.rs" \
  "residence_input_defined=1" \
  "residence_candidate_defined=1" \
  "consumer_input=ArrayReceiverResidenceInputSource" \
  "consumer_input_entry=ArrayReceiverResidenceProofChain" \
  "consumer_output=ArrayReceiverResidenceInput|none" \
  "consumer_constructor=ArrayReceiverResidenceInput::from_input_source" \
  "consumer_mode=fallback_only" \
  "consumer_accepts_public_arraybox_fallback=1" \
  "consumer_accepts_direct_storage_source=0" \
  "input_field_receiver_site_id=none" \
  "input_field_route_kind=array_slot_len" \
  "input_field_receiver_box_name=ArrayBox" \
  "input_field_direct_array_plan_available=0" \
  "input_field_object_storage_plan_available=0" \
  "input_field_array_repr_available=1" \
  "input_field_residence_candidate=public_arraybox_fallback" \
  "input_field_escape_facts_available=0" \
  "input_field_host_handle_publication_before_read=1" \
  "input_field_materialization_route_candidate=public_arraybox_fallback" \
  "input_field_direct_storage_proof=0" \
  "input_field_backend_bypass_authorized=0" \
  "input_public_handle_reinterpretation=0" \
  "input_backend_raw_layout_inference=0" \
  "input_helper_name_inference=0" \
  "input_mirbuilder_owner=0" \
  "input_exported_to_mir_json=0" \
  "input_consumed_by_backend=0" \
  "input_report_fields_defined=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=1" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-CLOSEOUT-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "pub enum ArrayReceiverResidenceCandidate" \
  "pub struct ArrayReceiverResidenceInput" \
  "pub fn from_input_source" \
  "pub fn array_receiver_residence_input_report_fields" \
  "(\"consumer_output\", \"ArrayReceiverResidenceInput|none\")" \
  "(\"input_field_direct_storage_proof\", \"0\")" \
  "(\"input_field_backend_bypass_authorized\", \"0\")" \
  "(\"input_consumed_by_backend\", \"0\")" \
  "(\"backend_direct_handle_bypass_enabled\", \"0\")"; do
  grep -F -q "$expected" "$SOURCE" || {
    echo "[mimalloc-array-residence-input-consumer-implementation] missing source token: $expected" >&2
    exit 1
  }
done

grep -F -q "\"input_field_residence_candidate\"" "$SOURCE" || {
  echo "[mimalloc-array-residence-input-consumer-implementation] missing source token: input_field_residence_candidate" >&2
  exit 1
}
grep -F -q "\"public_arraybox_fallback\"" "$SOURCE" || {
  echo "[mimalloc-array-residence-input-consumer-implementation] missing source token: public_arraybox_fallback" >&2
  exit 1
}

grep -F -q "do not implement backend direct handle bypass from this row" "$CARD" || {
  echo "[mimalloc-array-residence-input-consumer-implementation] missing backend stop line" >&2
  exit 1
}
grep -F -q "do not move Box/Object management into MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-residence-input-consumer-implementation] missing MIRBuilder stop line" >&2
  exit 1
}

echo "[mimalloc-array-residence-input-consumer-implementation] ok"
