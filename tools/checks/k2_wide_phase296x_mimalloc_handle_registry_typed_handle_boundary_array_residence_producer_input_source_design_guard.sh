#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-786-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-DESIGN-001.md"
INVENTORY_CARD="docs/development/current/main/phases/phase-296x/296x-785-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-INVENTORY-001.md"
SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-784-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SURFACE-001.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
DIRECT_PLAN="src/mir/direct_array_access_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_residence_producer_input_source_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-producer-input-source-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$INVENTORY_CARD" ]] || { echo "[mimalloc-array-residence-producer-input-source-design] missing inventory card: $INVENTORY_CARD" >&2; exit 1; }
[[ -f "$SURFACE_CARD" ]] || { echo "[mimalloc-array-residence-producer-input-source-design] missing surface card: $SURFACE_CARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-residence-producer-input-source-design] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-residence-producer-input-source-design] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }
[[ -f "$DIRECT_PLAN" ]] || { echo "[mimalloc-array-residence-producer-input-source-design] missing DirectArrayAccessPlan source: $DIRECT_PLAN" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-source-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$INVENTORY_CARD" || {
  echo "[mimalloc-array-residence-producer-input-source-design] inventory card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-producer-input-source-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-producer-input-source-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-producer-input-source-design-v0" \
  "source_evidence=296x-785,296x-784,array-repr-ssot,object-storage-plan-boundary-ssot,direct-array-access-plan" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "selected_design=array_receiver_residence_input_source" \
  "selected_design_confidence=medium" \
  "input_source_owner=RepresentationPlanner|ArrayReprSourcePlanner|ObjectStoragePlan" \
  "input_source_output=ArrayReceiverResidenceInputSource" \
  "input_source_scope=receiver_site_before_length_read" \
  "input_source_consumed_by=ArrayReceiverResidenceInput" \
  "input_source_may_reference_direct_array_access_plan=1" \
  "input_source_must_not_be_direct_array_access_plan_only=1" \
  "input_source_must_include_array_repr_or_object_storage=1" \
  "input_source_must_include_escape_publication_evidence=1" \
  "input_source_must_include_materialization_route=1" \
  "input_source_must_preserve_public_arraybox_fallback=1" \
  "input_source_must_not_reinterpret_public_arraybox_handle=1" \
  "input_source_must_not_infer_backend_raw_layout=1" \
  "input_source_must_not_use_helper_name=1" \
  "input_source_must_not_run_in_mirbuilder=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SOURCE-SURFACE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "input_source_routeplan_available=1" \
  "input_source_direct_array_access_plan_available=0" \
  "input_source_object_storage_plan_available=0" \
  "input_source_array_repr_available=0" \
  "input_source_escape_facts_available=1" \
  "selected_blocker=missing_array_receiver_representation_input_source"; do
  require_line_in_file "$INVENTORY_CARD" "$expected"
done

require_line_in_file "$SURFACE_CARD" "input_is_direct_array_access_plan_only=0"
grep -F -q "DirectArrayAccessPlan" "$DIRECT_PLAN" || {
  echo "[mimalloc-array-residence-producer-input-source-design] DirectArrayAccessPlan source missing type" >&2
  exit 1
}
grep -F -q "ArrayRepr" "$ARRAY_REPR" || {
  echo "[mimalloc-array-residence-producer-input-source-design] ArrayRepr SSOT missing ArrayRepr" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"

grep -F -q "reject: DirectArrayAccessPlan-only source" "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-source-design] missing DirectArrayAccessPlan-only rejection" >&2
  exit 1
}
grep -F -q "do not implement the input source from this row" "$CARD" || {
  echo "[mimalloc-array-residence-producer-input-source-design] missing input-source stop line" >&2
  exit 1
}

echo "[mimalloc-array-residence-producer-input-source-design] ok"
