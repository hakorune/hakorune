#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-780-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-DESIGN-001.md"
INVENTORY_CARD="docs/development/current/main/phases/phase-296x/296x-779-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-INVENTORY-001.md"
SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-778-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-SURFACE-001.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_residence_producer_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-producer-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$INVENTORY_CARD" ]] || { echo "[mimalloc-array-residence-producer-design] missing inventory card: $INVENTORY_CARD" >&2; exit 1; }
[[ -f "$SURFACE_CARD" ]] || { echo "[mimalloc-array-residence-producer-design] missing surface card: $SURFACE_CARD" >&2; exit 1; }
[[ -f "$TASKBOARD" ]] || { echo "[mimalloc-array-residence-producer-design] missing DirectArray taskboard: $TASKBOARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-residence-producer-design] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-residence-producer-design] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-producer-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$INVENTORY_CARD" || {
  echo "[mimalloc-array-residence-producer-design] inventory card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-producer-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-producer-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-design-v0" \
  "source_evidence=296x-779,296x-778,directarray-next-order-taskboard,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "selected_design=array_receiver_residence_fact_producer" \
  "selected_design_confidence=medium" \
  "route_proof_available=1" \
  "residence_proof_available=0" \
  "producer_owner=RepresentationPlanner|ArrayReprFactProducer" \
  "producer_output=ArrayReceiverResidenceFact" \
  "producer_input=RoutePlan|DirectArrayAccessPlan|ObjectStoragePlan|escape_facts" \
  "producer_must_run_before_backend_lowering=1" \
  "producer_must_not_run_in_mirbuilder=1" \
  "producer_must_not_run_in_backend_by_layout_inference=1" \
  "producer_must_not_use_helper_name=1" \
  "producer_must_not_reinterpret_public_arraybox_handle=1" \
  "producer_must_preserve_public_arraybox_fallback=1" \
  "producer_materialization_route_required=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-SURFACE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "DA-SEQ-001: DirectI64 fact inventory" \
  "DA-SEQ-002: DirectI64 ArrayRepr producer contract" \
  "DA-SEQ-003: DirectI64 ArrayRepr producer implementation" \
  "DA-SEQ-004: lowerer consumes ArrayRepr::DirectI64"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$INVENTORY_CARD" "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-DESIGN-001"
require_line_in_file "$INVENTORY_CARD" "array_receiver_residence_owner=none"
require_line_in_file "$SURFACE_CARD" "array_receiver_residence_owner=ObjectStoragePlan|ArrayRepr|none"

grep -F -q "DirectI64 ArrayRepr Producer Contract" "$TASKBOARD" || {
  echo "[mimalloc-array-residence-producer-design] DirectArray taskboard missing producer contract row" >&2
  exit 1
}
grep -F -q "Representation planner:" "$ARRAY_REPR" || {
  echo "[mimalloc-array-residence-producer-design] ArrayRepr SSOT missing representation planner owner" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"

grep -F -q "do not implement the producer from this row" "$CARD" || {
  echo "[mimalloc-array-residence-producer-design] missing producer stop line" >&2
  exit 1
}
grep -F -q "do not treat array_slot_len route proof as receiver storage proof" "$CARD" || {
  echo "[mimalloc-array-residence-producer-design] missing route/storage stop line" >&2
  exit 1
}

echo "[mimalloc-array-residence-producer-design] ok"
