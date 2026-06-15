#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-782-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INVENTORY-001.md"
SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-781-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-SURFACE-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-780-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-DESIGN-001.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_residence_producer_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-producer-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$SURFACE_CARD" ]] || { echo "[mimalloc-array-residence-producer-inventory] missing surface card: $SURFACE_CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-array-residence-producer-inventory] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-residence-producer-inventory] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-residence-producer-inventory] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-producer-inventory] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$SURFACE_CARD" || {
  echo "[mimalloc-array-residence-producer-inventory] surface card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-producer-inventory] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-producer-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-inventory-v0" \
  "source_evidence=296x-781,296x-780,296x-779,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "array_residence_producer_surface_defined=1" \
  "producer_owner=none" \
  "producer_output=none" \
  "producer_input_routeplan_available=1" \
  "producer_input_direct_array_plan_available=0" \
  "producer_input_object_storage_plan_available=0" \
  "producer_input_escape_facts_available=1" \
  "producer_order_valid=0" \
  "producer_runtime_execution=0" \
  "producer_backend_inference=0" \
  "producer_mirbuilder_owner=0" \
  "producer_helper_name_inference=0" \
  "producer_public_handle_reinterpretation=0" \
  "producer_preserves_public_arraybox_fallback=1" \
  "producer_materialization_route_required=1" \
  "fact_candidate_count=1" \
  "fact_eligible_count=0" \
  "fact_rejected_count=1" \
  "selected_fact_candidate_count=0" \
  "selected_fact_candidate_confidence=low" \
  "selected_blocker=missing_direct_array_or_object_storage_input" \
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
  "selected_decision=reject_producer_implementation_until_representation_input_exists" \
  "route_proof_available=1" \
  "producer_input_available=0" \
  "array_receiver_residence_fact_producer_available=0" \
  "direct_array_or_object_storage_input_required=1" \
  "producer_runtime_execution=0" \
  "producer_backend_inference=0" \
  "producer_mirbuilder_owner=0" \
  "producer_helper_name_inference=0" \
  "producer_public_handle_reinterpretation=0" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-DESIGN-001"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$SURFACE_CARD" "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INVENTORY-001"
require_line_in_file "$SURFACE_CARD" "producer_input_direct_array_plan_available=<0|1>"
require_line_in_file "$DESIGN_CARD" "selected_design=array_receiver_residence_fact_producer"
grep -F -q "DirectI64" "$ARRAY_REPR" || {
  echo "[mimalloc-array-residence-producer-inventory] ArrayRepr SSOT lacks DirectI64" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"

grep -F -q "do not implement the producer from this row" "$CARD" || {
  echo "[mimalloc-array-residence-producer-inventory] missing producer stop line" >&2
  exit 1
}

echo "[mimalloc-array-residence-producer-inventory] ok"
