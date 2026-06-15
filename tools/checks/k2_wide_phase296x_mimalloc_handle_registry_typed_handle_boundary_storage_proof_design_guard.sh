#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-774-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-DESIGN-001.md"
INVENTORY_CARD="docs/development/current/main/phases/phase-296x/296x-773-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-CANDIDATE-INVENTORY-001.md"
ROUTE_CARD="docs/development/current/main/phases/phase-296x/296x-706-MIMALLOC-DIRECT-ARRAY-LENGTH-BOUNDARY-DESIGN-001.md"
ARRAY_SPLIT="docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md"
ARRAY_REPR="docs/development/current/main/design/array-repr-ssot.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_storage_proof_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-handle-boundary-storage-proof-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$INVENTORY_CARD" ]] || { echo "[mimalloc-handle-boundary-storage-proof-design] missing inventory card: $INVENTORY_CARD" >&2; exit 1; }
[[ -f "$ROUTE_CARD" ]] || { echo "[mimalloc-handle-boundary-storage-proof-design] missing route card: $ROUTE_CARD" >&2; exit 1; }
[[ -f "$ARRAY_SPLIT" ]] || { echo "[mimalloc-handle-boundary-storage-proof-design] missing array split SSOT: $ARRAY_SPLIT" >&2; exit 1; }
[[ -f "$ARRAY_REPR" ]] || { echo "[mimalloc-handle-boundary-storage-proof-design] missing array repr SSOT: $ARRAY_REPR" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-handle-boundary-storage-proof-design] missing object SSOT: $OBJECT_SSOT" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-handle-boundary-storage-proof-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$INVENTORY_CARD" || {
  echo "[mimalloc-handle-boundary-storage-proof-design] inventory card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-handle-boundary-storage-proof-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-handle-boundary-storage-proof-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-storage-proof-design-v0" \
  "source_evidence=296x-773,296x-706,296x-373,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "selected_design=array_receiver_storage_proof_via_object_storage_plan" \
  "selected_design_confidence=medium" \
  "route_proof_available=1" \
  "storage_proof_available=0" \
  "implementation_allowed=0" \
  "backend_direct_handle_bypass_enabled=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "public_arraybox_handle_reinterpretation_allowed=0" \
  "raw_arraybox_layout_backend_truth=0" \
  "arrayrepr_or_object_storage_plan_required=1" \
  "fallback_to_public_arraybox_host_handle_required=1" \
  "fallback_to_generic_host_handle_required=1" \
  "benchmark_name_special_case=0" \
  "helper_name_special_case=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-SURFACE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "array_receiver_storage_proof_defined=1" \
  "array_receiver_storage_owner=ObjectStoragePlan|ArrayRepr" \
  "array_receiver_storage_residence=direct_array|exact_native_struct|scalarized|none" \
  "array_receiver_public_handle_reinterpreted=0" \
  "array_receiver_host_handle_publication_before_read=0" \
  "array_receiver_fallback_public_arraybox=1" \
  "array_receiver_backend_raw_layout_inference=0"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$INVENTORY_CARD" "receiver_storage_plan_known=0"
require_line_in_file "$INVENTORY_CARD" "eligible_site_count=0"
require_line_in_file "$INVENTORY_CARD" "selected_blocker=receiver_storage_plan_missing"
require_line_in_file "$INVENTORY_CARD" "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-DESIGN-001"

require_line_in_file "$ROUTE_CARD" "route_kind=array_slot_len"
require_line_in_file "$ROUTE_CARD" "box_name=ArrayBox"
require_line_in_file "$ROUTE_CARD" "route proof is good enough to select Array length semantics, but it is not a raw"

grep -F -q "Do not reinterpret a public ArrayBox host handle as a DirectArray pointer." "$ARRAY_SPLIT" || {
  echo "[mimalloc-handle-boundary-storage-proof-design] array split SSOT missing public-handle reinterpretation ban" >&2
  exit 1
}
grep -F -q "reinterpret a public \`ArrayBox\` host handle as a direct pointer" "$ARRAY_REPR" || {
  echo "[mimalloc-handle-boundary-storage-proof-design] array repr SSOT missing public-handle reinterpretation ban" >&2
  exit 1
}
require_line_in_file "$OBJECT_SSOT" "object_storage_plan_is_representation_truth=1"
require_line_in_file "$OBJECT_SSOT" "mirbuilder_object_boundary_removal_owner=0"

grep -F -q "do not reinterpret public ArrayBox HostHandle as direct storage" "$CARD" || {
  echo "[mimalloc-handle-boundary-storage-proof-design] missing public handle stop line" >&2
  exit 1
}
grep -F -q "do not expose Rust ArrayBox internal layout as backend truth" "$CARD" || {
  echo "[mimalloc-handle-boundary-storage-proof-design] missing raw layout stop line" >&2
  exit 1
}

echo "[mimalloc-handle-boundary-storage-proof-design] ok"
