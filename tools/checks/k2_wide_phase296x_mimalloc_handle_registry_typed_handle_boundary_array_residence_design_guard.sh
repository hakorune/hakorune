#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-777-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-DESIGN-001.md"
INVENTORY_CARD="docs/development/current/main/phases/phase-296x/296x-776-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-INVENTORY-001.md"
SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-775-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-SURFACE-001.md"
ARRAY_SPLIT="docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_residence_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$INVENTORY_CARD" ]] || { echo "[mimalloc-array-residence-design] missing inventory card: $INVENTORY_CARD" >&2; exit 1; }
[[ -f "$SURFACE_CARD" ]] || { echo "[mimalloc-array-residence-design] missing surface card: $SURFACE_CARD" >&2; exit 1; }
[[ -f "$ARRAY_SPLIT" ]] || { echo "[mimalloc-array-residence-design] missing ArrayBox split SSOT: $ARRAY_SPLIT" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-array-residence-design] missing ArrayRepr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-array-residence-design] missing ObjectStoragePlan SSOT: $OBJECT_SSOT" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$INVENTORY_CARD" || {
  echo "[mimalloc-array-residence-design] inventory card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-residence-design-v0" \
  "source_evidence=296x-776,296x-775,296x-373,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "selected_design=array_receiver_residence_proof_via_arrayrepr" \
  "selected_design_confidence=medium" \
  "route_proof_available=1" \
  "storage_proof_available=0" \
  "array_residence_proof_required=1" \
  "array_residence_owner=ObjectStoragePlan|ArrayRepr" \
  "array_residence_allowed_values=direct_array|exact_native_struct|scalarized|public_arraybox_fallback" \
  "array_direct_facts_source=DirectArrayAccessPlan|ArrayRepr|ObjectStoragePlan" \
  "array_materialization_route_required=1" \
  "public_arraybox_fallback_required=1" \
  "public_arraybox_handle_reinterpretation_allowed=0" \
  "backend_raw_arraybox_layout_truth=0" \
  "route_proof_as_storage_proof_allowed=0" \
  "mirbuilder_object_management_enabled=0" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-SURFACE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "array_receiver_route_kind=array_slot_len" \
  "array_receiver_residence_owner=ObjectStoragePlan|ArrayRepr" \
  "array_receiver_residence=direct_array|exact_native_struct|scalarized" \
  "array_receiver_direct_facts_proven=1" \
  "array_receiver_materialization_route_known=1" \
  "array_receiver_public_handle_reinterpreted=0" \
  "array_receiver_backend_raw_layout_inference=0" \
  "array_receiver_host_handle_publication_before_read=0" \
  "array_receiver_fallback_public_arraybox=1" \
  "selected_storage_candidate_confidence=high"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$INVENTORY_CARD" "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-DESIGN-001"
require_line_in_file "$INVENTORY_CARD" "array_receiver_storage_owner=none"
require_line_in_file "$INVENTORY_CARD" "array_receiver_storage_residence=none"
require_line_in_file "$INVENTORY_CARD" "array_receiver_host_handle_publication_before_read=1"
require_line_in_file "$SURFACE_CARD" "array_receiver_materialization_route_required=1"

require_line_in_file "$ARRAY_SPLIT" "public_arraybox_handle_reinterpret_as_direct=0"
require_line_in_file "$ARRAY_SPLIT" "direct_array_materialization_route_required=1"
grep -F -q "reinterpret a public \`ArrayBox\` host handle as a direct pointer" "$ARRAY_REPR" || {
  echo "[mimalloc-array-residence-design] ArrayRepr SSOT missing public handle reinterpretation ban" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"
require_line_in_file "$OBJECT_SSOT" "backend_consumes_object_storage_plan=1"

grep -F -q "do not treat array_slot_len route proof as receiver storage proof" "$CARD" || {
  echo "[mimalloc-array-residence-design] missing route/storage stop line" >&2
  exit 1
}
grep -F -q "do not change nyash.array.birth_h public semantics" "$CARD" || {
  echo "[mimalloc-array-residence-design] missing birth semantics stop line" >&2
  exit 1
}

echo "[mimalloc-array-residence-design] ok"
