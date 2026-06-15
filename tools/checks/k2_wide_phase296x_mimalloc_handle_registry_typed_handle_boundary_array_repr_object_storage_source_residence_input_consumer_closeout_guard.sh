#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-809-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-CLOSEOUT-001.md"
IMPL_CARD="docs/development/current/main/phases/phase-296x/296x-808-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-IMPLEMENTATION-001.md"
SOURCE="src/array_receiver_representation_source.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_residence_input_consumer_closeout_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-input-consumer-closeout] missing card: $CARD" >&2; exit 1; }
[[ -f "$IMPL_CARD" ]] || { echo "[mimalloc-array-residence-input-consumer-closeout] missing implementation card: $IMPL_CARD" >&2; exit 1; }
[[ -f "$SOURCE" ]] || { echo "[mimalloc-array-residence-input-consumer-closeout] missing source: $SOURCE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-input-consumer-closeout] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$IMPL_CARD" || {
  echo "[mimalloc-array-residence-input-consumer-closeout] implementation card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-input-consumer-closeout] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-input-consumer-closeout] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-input-consumer-closeout-v0" \
  "source_evidence=296x-808,296x-807,296x-806,296x-805,array-repr-proof-chain-guide" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "residence_input_consumer_closed=1" \
  "developer_facing_entry=ArrayReceiverResidenceProofChain" \
  "closed_flow=ArrayReceiverRepresentationSource->ArrayReceiverResidenceProofChain->ArrayReceiverResidenceInputSource->ArrayReceiverResidenceInput" \
  "closed_stage=ArrayReceiverResidenceInput" \
  "closed_stage_mode=fallback_only" \
  "closed_stage_backend_consumable=0" \
  "closed_stage_direct_storage_proof=0" \
  "closed_stage_backend_bypass_authorized=0" \
  "residence_fact_producer_implemented=0" \
  "residence_fact_backend_consumable=0" \
  "backend_direct_handle_bypass_enabled=0" \
  "mir_json_export_enabled=0" \
  "backend_consumption_enabled=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_stage=ArrayReceiverResidenceFact" \
  "next_stage_requires_design_consultation=1" \
  "next_stage_reason=backend_consumable_proof_boundary" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-FACT-PRODUCER-DESIGN-CONSULTATION-001" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "pub struct ArrayReceiverResidenceInput" \
  "pub fn from_input_source" \
  "pub fn array_receiver_residence_input_report_fields" \
  "(\"input_consumed_by_backend\", \"0\")" \
  "(\"backend_direct_handle_bypass_enabled\", \"0\")"; do
  grep -F -q "$expected" "$SOURCE" || {
    echo "[mimalloc-array-residence-input-consumer-closeout] missing source token: $expected" >&2
    exit 1
  }
done

grep -F -q "Do not continue into backend consumption without this decision." "$CARD" || {
  echo "[mimalloc-array-residence-input-consumer-closeout] missing design consultation stop" >&2
  exit 1
}

echo "[mimalloc-array-residence-input-consumer-closeout] ok"
