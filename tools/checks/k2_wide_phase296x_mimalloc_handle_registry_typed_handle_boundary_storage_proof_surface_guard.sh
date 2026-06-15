#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-775-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-SURFACE-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-774-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-DESIGN-001.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_storage_proof_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-handle-boundary-storage-proof-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-handle-boundary-storage-proof-surface] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-handle-boundary-storage-proof-surface] missing array repr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-handle-boundary-storage-proof-surface] missing object SSOT: $OBJECT_SSOT" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-handle-boundary-storage-proof-surface] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$DESIGN_CARD" || {
  echo "[mimalloc-handle-boundary-storage-proof-surface] design card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-handle-boundary-storage-proof-surface] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-handle-boundary-storage-proof-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-storage-proof-surface-v0" \
  "source_evidence=296x-774,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "array_receiver_storage_proof_surface_defined=1" \
  "array_receiver_storage_proof_defined=1" \
  "array_receiver_storage_owner=ObjectStoragePlan|ArrayRepr" \
  "array_receiver_storage_residence=direct_array|exact_native_struct|scalarized|none" \
  "array_receiver_public_handle_reinterpreted=0" \
  "array_receiver_host_handle_publication_before_read=<0|1>" \
  "array_receiver_fallback_public_arraybox=1" \
  "array_receiver_backend_raw_layout_inference=0" \
  "array_receiver_direct_facts_required=1" \
  "array_receiver_materialization_route_required=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "public_arraybox_handle_reinterpretation_allowed=0" \
  "raw_arraybox_layout_backend_truth=0" \
  "fallback_to_public_arraybox_host_handle_required=1" \
  "fallback_to_generic_host_handle_required=1" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-INVENTORY-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "array_receiver_storage_owner=ObjectStoragePlan|ArrayRepr" \
  "array_receiver_storage_residence=direct_array|exact_native_struct|scalarized" \
  "array_receiver_direct_facts_proven=1" \
  "array_receiver_materialization_route_known=1" \
  "array_receiver_public_handle_reinterpreted=0" \
  "array_receiver_backend_raw_layout_inference=0" \
  "array_receiver_host_handle_publication_before_read=0" \
  "array_receiver_fallback_public_arraybox=1" \
  "storage_eligible_count>=1" \
  "selected_storage_candidate_confidence=high"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$DESIGN_CARD" "selected_design=array_receiver_storage_proof_via_object_storage_plan"
require_line_in_file "$DESIGN_CARD" "arrayrepr_or_object_storage_plan_required=1"
require_line_in_file "$DESIGN_CARD" "public_arraybox_handle_reinterpretation_allowed=0"
require_line_in_file "$DESIGN_CARD" "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-SURFACE-001"

grep -F -q "DirectI64" "$ARRAY_REPR" || {
  echo "[mimalloc-handle-boundary-storage-proof-surface] ArrayRepr SSOT lacks DirectI64" >&2
  exit 1
}
grep -F -q "PublicArrayBoxFallback" "$ARRAY_REPR" || {
  echo "[mimalloc-handle-boundary-storage-proof-surface] ArrayRepr SSOT lacks PublicArrayBoxFallback" >&2
  exit 1
}
grep -F -q "reinterpret a public \`ArrayBox\` host handle as a direct pointer" "$ARRAY_REPR" || {
  echo "[mimalloc-handle-boundary-storage-proof-surface] ArrayRepr SSOT missing public handle reinterpretation ban" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"
require_line_in_file "$OBJECT_SSOT" "backend_consumes_object_storage_plan=1"

grep -F -q "do not treat route proof alone as storage proof" "$CARD" || {
  echo "[mimalloc-handle-boundary-storage-proof-surface] missing route/storage stop line" >&2
  exit 1
}
grep -F -q "do not reinterpret public ArrayBox HostHandle as direct storage" "$CARD" || {
  echo "[mimalloc-handle-boundary-storage-proof-surface] missing public handle stop line" >&2
  exit 1
}

echo "[mimalloc-handle-boundary-storage-proof-surface] ok"
