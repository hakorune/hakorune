#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-773-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-CANDIDATE-INVENTORY-001.md"
PROOF_CARD="docs/development/current/main/phases/phase-296x/296x-772-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-PROOF-SURFACE-001.md"
ROUTE_CARD="docs/development/current/main/phases/phase-296x/296x-706-MIMALLOC-DIRECT-ARRAY-LENGTH-BOUNDARY-DESIGN-001.md"
SHADOW_CARD="docs/development/current/main/phases/phase-296x/296x-712-EXACT-OBJECT-PLAN-SHADOW-001.md"
OBJECT_PLAN="src/object_storage_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_candidate_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-handle-boundary-candidate-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$PROOF_CARD" ]] || { echo "[mimalloc-handle-boundary-candidate-inventory] missing proof card: $PROOF_CARD" >&2; exit 1; }
[[ -f "$ROUTE_CARD" ]] || { echo "[mimalloc-handle-boundary-candidate-inventory] missing route card: $ROUTE_CARD" >&2; exit 1; }
[[ -f "$SHADOW_CARD" ]] || { echo "[mimalloc-handle-boundary-candidate-inventory] missing shadow card: $SHADOW_CARD" >&2; exit 1; }
[[ -f "$OBJECT_PLAN" ]] || { echo "[mimalloc-handle-boundary-candidate-inventory] missing ObjectStoragePlan code: $OBJECT_PLAN" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-handle-boundary-candidate-inventory] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PROOF_CARD" || {
  echo "[mimalloc-handle-boundary-candidate-inventory] proof card must be Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$ROUTE_CARD" || {
  echo "[mimalloc-handle-boundary-candidate-inventory] route card must be Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$SHADOW_CARD" || {
  echo "[mimalloc-handle-boundary-candidate-inventory] shadow card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-handle-boundary-candidate-inventory] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-handle-boundary-candidate-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-candidate-inventory-v0" \
  "source_evidence=296x-772,296x-706,296x-712" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "closed_world_handle_resolution_plan_defined=1" \
  "receiver_route_known=1" \
  "receiver_route_owner=RoutePlan" \
  "receiver_route_is_closed_world=1" \
  "receiver_route_is_plugin_or_dynamic=0" \
  "receiver_route_uses_reflection_or_by_name=0" \
  "receiver_storage_plan_known=0" \
  "receiver_storage_owner=none" \
  "receiver_storage_is_exact=0" \
  "receiver_storage_requires_host_handle=1" \
  "receiver_handle_publication_required=1" \
  "dynamic_escape_count=0" \
  "plugin_or_extern_escape_count=0" \
  "reflection_or_by_name_route_count=0" \
  "host_handle_publication_count=1" \
  "unsupported_storage_reason_count=1" \
  "candidate_site_count=1" \
  "eligible_site_count=0" \
  "rejected_site_count=1" \
  "selected_candidate_count=0" \
  "selected_candidate_confidence=low" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "benchmark_name_special_case=0" \
  "helper_name_special_case=0" \
  "fallback_to_generic_host_handle_required=1" \
  "selected_blocker=receiver_storage_plan_missing" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-STORAGE-PROOF-DESIGN-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$PROOF_CARD" "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-CANDIDATE-INVENTORY-001"
require_line_in_file "$ROUTE_CARD" "route_id=generic_method.len"
require_line_in_file "$ROUTE_CARD" "route_kind=array_slot_len"
require_line_in_file "$ROUTE_CARD" "box_name=ArrayBox"
require_line_in_file "$ROUTE_CARD" "method=length"
require_line_in_file "$ROUTE_CARD" "receiver_origin_box=ArrayBox"
require_line_in_file "$ROUTE_CARD" "helper_symbol=nyash.array.slot_len_h"
require_line_in_file "$ROUTE_CARD" "publication_policy=no_publication"

require_line_in_file "$SHADOW_CARD" "selected_pilot_candidate=HakoAllocObjectLifecycleAlignmentResult"
require_line_in_file "$SHADOW_CARD" "selected_pilot_confidence=medium"
require_line_in_file "$SHADOW_CARD" "host_handle_escaped_plan_count=4"

grep -R -F -q "HostHandleEscaped" src/object_storage_plan.rs src/object_storage_plan || {
  echo "[mimalloc-handle-boundary-candidate-inventory] ObjectStoragePlan lacks HostHandleEscaped vocabulary" >&2
  exit 1
}
grep -R -F -q "HostHandlePublicationRequired" src/object_storage_plan.rs src/object_storage_plan || {
  echo "[mimalloc-handle-boundary-candidate-inventory] ObjectStoragePlan lacks host-handle publication reason" >&2
  exit 1
}
grep -F -q "do not implement backend direct handle bypass from this inventory" "$CARD" || {
  echo "[mimalloc-handle-boundary-candidate-inventory] missing implementation stop line" >&2
  exit 1
}
grep -F -q "do not treat route proof alone as storage proof" "$CARD" || {
  echo "[mimalloc-handle-boundary-candidate-inventory] missing route-vs-storage stop line" >&2
  exit 1
}

echo "[mimalloc-handle-boundary-candidate-inventory] ok"
