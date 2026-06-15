#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-788-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-INVENTORY-001.md"
SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-787-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-SURFACE-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-786-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-DESIGN-001.md"
INPUT_INVENTORY_CARD="docs/development/current/main/phases/phase-296x/296x-785-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-INVENTORY-001.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
DIRECT_PLAN="src/mir/direct_array_access_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_residence_producer_input_source_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-producer-input-source-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$SURFACE_CARD" ]] || { echo "[mimalloc-array-residence-producer-input-source-inventory] missing surface card: $SURFACE_CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-array-residence-producer-input-source-inventory] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$INPUT_INVENTORY_CARD" ]] || { echo "[mimalloc-array-residence-producer-input-source-inventory] missing input inventory card: $INPUT_INVENTORY_CARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-residence-producer-input-source-inventory] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-residence-producer-input-source-inventory] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }
[[ -f "$DIRECT_PLAN" ]] || { echo "[mimalloc-array-residence-producer-input-source-inventory] missing DirectArrayAccessPlan source: $DIRECT_PLAN" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-source-inventory] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$SURFACE_CARD" || {
  echo "[mimalloc-array-residence-producer-input-source-inventory] surface card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-producer-input-source-inventory] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-producer-input-source-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-source-inventory-v0" \
  "source_evidence=296x-787,296x-786,296x-785,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "array_receiver_residence_input_source_surface_defined=1" \
  "input_source_owner=none" \
  "input_source_output=none" \
  "input_source_scope=receiver_site_before_length_read" \
  "input_source_consumed_by=ArrayReceiverResidenceInput" \
  "input_source_route_kind=array_slot_len" \
  "input_source_receiver_box_name=ArrayBox" \
  "input_source_routeplan_available=1" \
  "input_source_direct_array_access_plan_ref=none" \
  "input_source_object_storage_plan_ref=none" \
  "input_source_array_repr=none" \
  "input_source_escape_facts_ref=available" \
  "input_source_host_handle_publication_before_read=1" \
  "input_source_materialization_route=public_arraybox_fallback" \
  "input_source_confidence=low" \
  "input_source_may_reference_direct_array_access_plan=1" \
  "input_source_is_direct_array_access_plan_only=0" \
  "input_source_includes_array_repr_or_object_storage=0" \
  "input_source_includes_escape_publication_evidence=1" \
  "input_source_includes_materialization_route=1" \
  "input_source_preserves_public_arraybox_fallback=1" \
  "input_source_public_handle_reinterpretation=0" \
  "input_source_backend_raw_layout_inference=0" \
  "input_source_helper_name_inference=0" \
  "input_source_mirbuilder_owner=0" \
  "source_candidate_count=1" \
  "source_eligible_count=0" \
  "source_rejected_count=1" \
  "selected_source_candidate_count=0" \
  "selected_source_candidate_confidence=low" \
  "selected_blocker=missing_array_repr_or_object_storage_source" \
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
  "selected_decision=reject_input_source_implementation_until_representation_source_exists" \
  "route_proof_available=1" \
  "escape_publication_evidence_available=1" \
  "materialization_route_available=1" \
  "array_receiver_residence_input_source_available=0" \
  "array_repr_source_available=0" \
  "object_storage_plan_source_available=0" \
  "direct_array_access_plan_source_available=0" \
  "input_source_construction_required=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-CONSTRUCTION-DESIGN-001"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$SURFACE_CARD" "array_receiver_residence_input_source_surface_defined=1"
require_line_in_file "$SURFACE_CARD" "source_eligible_count>=1"
require_line_in_file "$DESIGN_CARD" "selected_design=array_receiver_residence_input_source"
require_line_in_file "$INPUT_INVENTORY_CARD" "input_source_routeplan_available=1"
require_line_in_file "$INPUT_INVENTORY_CARD" "input_source_array_repr_available=0"
require_line_in_file "$INPUT_INVENTORY_CARD" "input_source_object_storage_plan_available=0"
grep -F -q "DirectArrayAccessPlan" "$DIRECT_PLAN" || {
  echo "[mimalloc-array-residence-producer-input-source-inventory] DirectArrayAccessPlan source missing type" >&2
  exit 1
}
grep -F -q "ArrayRepr" "$ARRAY_REPR" || {
  echo "[mimalloc-array-residence-producer-input-source-inventory] ArrayRepr SSOT missing ArrayRepr" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"

grep -F -q "do not implement the input source from this row" "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-source-inventory] missing input-source stop line" >&2
  exit 1
}
grep -F -q "design source construction under RepresentationPlanner / ArrayReprSourcePlanner" "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-source-inventory] missing next construction design guidance" >&2
  exit 1
}

echo "[mimalloc-array-residence-producer-input-source-inventory] ok"
