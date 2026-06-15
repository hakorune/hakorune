#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-783-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-DESIGN-001.md"
INVENTORY_CARD="docs/development/current/main/phases/phase-296x/296x-782-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INVENTORY-001.md"
SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-781-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-SURFACE-001.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
DIRECT_PLAN="src/mir/direct_array_access_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_residence_producer_input_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-producer-input-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$INVENTORY_CARD" ]] || { echo "[mimalloc-array-residence-producer-input-design] missing inventory card: $INVENTORY_CARD" >&2; exit 1; }
[[ -f "$SURFACE_CARD" ]] || { echo "[mimalloc-array-residence-producer-input-design] missing surface card: $SURFACE_CARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-residence-producer-input-design] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-residence-producer-input-design] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }
[[ -f "$DIRECT_PLAN" ]] || { echo "[mimalloc-array-residence-producer-input-design] missing DirectArrayAccessPlan source: $DIRECT_PLAN" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$INVENTORY_CARD" || {
  echo "[mimalloc-array-residence-producer-input-design] inventory card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-producer-input-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-producer-input-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-design-v0" \
  "source_evidence=296x-782,296x-781,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "selected_design=array_receiver_residence_input_surface" \
  "selected_design_confidence=medium" \
  "route_proof_available=1" \
  "producer_input_available=0" \
  "input_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr" \
  "input_output=ArrayReceiverResidenceInput" \
  "input_source=RoutePlan|DirectArrayAccessPlan|ObjectStoragePlan|escape_facts" \
  "input_can_reference_direct_array_access_plan=1" \
  "input_must_not_be_direct_array_access_plan_only=1" \
  "input_must_support_length_receiver_residence=1" \
  "input_must_preserve_public_arraybox_fallback=1" \
  "input_must_not_reinterpret_public_arraybox_handle=1" \
  "input_must_not_infer_backend_raw_layout=1" \
  "input_must_not_use_helper_name=1" \
  "input_must_not_run_in_mirbuilder=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SURFACE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$INVENTORY_CARD" "selected_blocker=missing_direct_array_or_object_storage_input"
require_line_in_file "$SURFACE_CARD" "producer_input_direct_array_plan_available=<0|1>"
grep -F -q "DirectArrayAccessPlan" "$DIRECT_PLAN" || {
  echo "[mimalloc-array-residence-producer-input-design] DirectArrayAccessPlan source missing type" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"
grep -F -q "ArrayRepr" "$ARRAY_REPR" || {
  echo "[mimalloc-array-residence-producer-input-design] ArrayRepr SSOT missing ArrayRepr" >&2
  exit 1
}

grep -F -q "do not implement the producer input from this row" "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-design] missing input stop line" >&2
  exit 1
}
grep -F -q "reject: use DirectArrayAccessPlan alone as the producer input" "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-design] missing DirectArrayAccessPlan-alone rejection" >&2
  exit 1
}

echo "[mimalloc-array-residence-producer-input-design] ok"
