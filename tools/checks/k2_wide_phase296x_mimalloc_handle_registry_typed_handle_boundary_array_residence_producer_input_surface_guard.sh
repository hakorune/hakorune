#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-784-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SURFACE-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-783-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-DESIGN-001.md"
PRODUCER_INVENTORY_CARD="docs/development/current/main/phases/phase-296x/296x-782-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INVENTORY-001.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
DIRECT_PLAN="src/mir/direct_array_access_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_residence_producer_input_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-producer-input-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-array-residence-producer-input-surface] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$PRODUCER_INVENTORY_CARD" ]] || { echo "[mimalloc-array-residence-producer-input-surface] missing producer inventory card: $PRODUCER_INVENTORY_CARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-residence-producer-input-surface] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-residence-producer-input-surface] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }
[[ -f "$DIRECT_PLAN" ]] || { echo "[mimalloc-array-residence-producer-input-surface] missing DirectArrayAccessPlan source: $DIRECT_PLAN" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-surface] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$DESIGN_CARD" || {
  echo "[mimalloc-array-residence-producer-input-surface] design card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-producer-input-surface] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-producer-input-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-surface-v0" \
  "source_evidence=296x-783,296x-782,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "array_receiver_residence_input_surface_defined=1" \
  "input_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr|none" \
  "input_output=ArrayReceiverResidenceInput|none" \
  "input_source_routeplan_available=<0|1>" \
  "input_source_direct_array_access_plan_available=<0|1>" \
  "input_source_object_storage_plan_available=<0|1>" \
  "input_source_array_repr_available=<0|1>" \
  "input_source_escape_facts_available=<0|1>" \
  "input_can_reference_direct_array_access_plan=1" \
  "input_is_direct_array_access_plan_only=0" \
  "input_supports_length_receiver_residence=<0|1>" \
  "input_preserves_public_arraybox_fallback=1" \
  "input_public_handle_reinterpretation=0" \
  "input_backend_raw_layout_inference=0" \
  "input_helper_name_inference=0" \
  "input_mirbuilder_owner=0" \
  "input_materialization_route_required=1" \
  "input_candidate_count=<n>" \
  "input_eligible_count=<n>" \
  "input_rejected_count=<n>" \
  "selected_input_candidate_count=<n>" \
  "selected_input_candidate_confidence=low|medium|high" \
  "selected_blocker=<blocker|none>" \
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
  "input_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr" \
  "input_output=ArrayReceiverResidenceInput" \
  "input_source_routeplan_available=1" \
  "input_source_escape_facts_available=1" \
  "input_can_reference_direct_array_access_plan=1" \
  "input_is_direct_array_access_plan_only=0" \
  "input_supports_length_receiver_residence=1" \
  "input_preserves_public_arraybox_fallback=1" \
  "input_public_handle_reinterpretation=0" \
  "input_backend_raw_layout_inference=0" \
  "input_helper_name_inference=0" \
  "input_mirbuilder_owner=0" \
  "input_materialization_route_required=1" \
  "input_eligible_count>=1" \
  "selected_input_candidate_confidence=high"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$DESIGN_CARD" "selected_design=array_receiver_residence_input_surface"
require_line_in_file "$DESIGN_CARD" "input_must_not_be_direct_array_access_plan_only=1"
require_line_in_file "$PRODUCER_INVENTORY_CARD" "selected_blocker=missing_direct_array_or_object_storage_input"
grep -F -q "DirectArrayAccessPlan" "$DIRECT_PLAN" || {
  echo "[mimalloc-array-residence-producer-input-surface] DirectArrayAccessPlan source missing type" >&2
  exit 1
}
grep -F -q "ArrayRepr" "$ARRAY_REPR" || {
  echo "[mimalloc-array-residence-producer-input-surface] ArrayRepr SSOT missing ArrayRepr" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"

grep -F -q "do not implement the producer input from this row" "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-surface] missing input stop line" >&2
  exit 1
}
grep -F -q "do not treat DirectArrayAccessPlan alone as receiver residence proof" "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-surface] missing DirectArrayAccessPlan stop line" >&2
  exit 1
}
grep -F -q "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SURFACE-001" "$DESIGN_CARD" || {
  echo "[mimalloc-array-residence-producer-input-surface] design card does not point to this row" >&2
  exit 1
}

echo "[mimalloc-array-residence-producer-input-surface] ok"
