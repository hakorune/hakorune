#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-781-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-SURFACE-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-780-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-DESIGN-001.md"
INVENTORY_CARD="docs/development/current/main/phases/phase-296x/296x-779-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-INVENTORY-001.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_residence_producer_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-producer-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-array-residence-producer-surface] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$INVENTORY_CARD" ]] || { echo "[mimalloc-array-residence-producer-surface] missing inventory card: $INVENTORY_CARD" >&2; exit 1; }
[[ -f "$TASKBOARD" ]] || { echo "[mimalloc-array-residence-producer-surface] missing DirectArray taskboard: $TASKBOARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-residence-producer-surface] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-residence-producer-surface] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-producer-surface] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$DESIGN_CARD" || {
  echo "[mimalloc-array-residence-producer-surface] design card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-producer-surface] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-producer-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-surface-v0" \
  "source_evidence=296x-780,296x-779,directarray-next-order-taskboard,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "array_residence_producer_surface_defined=1" \
  "producer_owner=RepresentationPlanner|ArrayReprFactProducer" \
  "producer_output=ArrayReceiverResidenceFact" \
  "producer_input=RoutePlan|DirectArrayAccessPlan|ObjectStoragePlan|escape_facts" \
  "producer_order=after_routeplan_and_object_storage_facts_before_backend_lowering" \
  "producer_runtime_execution=0" \
  "producer_backend_inference=0" \
  "producer_mirbuilder_owner=0" \
  "producer_helper_name_inference=0" \
  "producer_public_handle_reinterpretation=0" \
  "producer_preserves_public_arraybox_fallback=1" \
  "producer_materialization_route_required=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INVENTORY-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "producer_owner=RepresentationPlanner|ArrayReprFactProducer" \
  "producer_output=ArrayReceiverResidenceFact" \
  "producer_input_routeplan_available=1" \
  "producer_input_escape_facts_available=1" \
  "producer_order_valid=1" \
  "producer_runtime_execution=0" \
  "producer_backend_inference=0" \
  "producer_mirbuilder_owner=0" \
  "producer_helper_name_inference=0" \
  "producer_public_handle_reinterpretation=0" \
  "producer_preserves_public_arraybox_fallback=1" \
  "producer_materialization_route_required=1" \
  "fact_eligible_count>=1" \
  "selected_fact_candidate_confidence=high"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$DESIGN_CARD" "selected_design=array_receiver_residence_fact_producer"
require_line_in_file "$DESIGN_CARD" "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-SURFACE-001"
require_line_in_file "$INVENTORY_CARD" "selected_blocker=array_receiver_residence_missing"

grep -F -q "DirectI64 ArrayRepr Producer Contract" "$TASKBOARD" || {
  echo "[mimalloc-array-residence-producer-surface] DirectArray taskboard missing producer contract row" >&2
  exit 1
}
grep -F -q "Representation planner:" "$ARRAY_REPR" || {
  echo "[mimalloc-array-residence-producer-surface] ArrayRepr SSOT missing representation planner owner" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"

grep -F -q "do not implement the producer from this row" "$CARD" || {
  echo "[mimalloc-array-residence-producer-surface] missing producer stop line" >&2
  exit 1
}

echo "[mimalloc-array-residence-producer-surface] ok"
