#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-772-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-PROOF-SURFACE-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-771-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-DESIGN-001.md"
ASM_CARD="docs/development/current/main/phases/phase-296x/296x-770-MIMALLOC-CURRENT-2X-ASM-BOUNDARY-ATTRIBUTION-001.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_proof_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-handle-boundary-proof-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-handle-boundary-proof-surface] missing design card: $DESIGN_CARD" >&2; exit 1; }
[[ -f "$ASM_CARD" ]] || { echo "[mimalloc-handle-boundary-proof-surface] missing asm card: $ASM_CARD" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-handle-boundary-proof-surface] missing object SSOT: $OBJECT_SSOT" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-handle-boundary-proof-surface] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$DESIGN_CARD" || {
  echo "[mimalloc-handle-boundary-proof-surface] design card must be Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$ASM_CARD" || {
  echo "[mimalloc-handle-boundary-proof-surface] asm card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-handle-boundary-proof-surface] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-handle-boundary-proof-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-proof-surface-v0" \
  "source_evidence=296x-771,296x-770,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "selected_plan=closed_world_handle_resolution_plan" \
  "proof_surface_defined=1" \
  "closed_world_handle_resolution_plan_defined=1" \
  "routeplan_proof_required=1" \
  "object_storage_plan_proof_required=1" \
  "backend_consumes_route_and_storage_plans=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "runtime_object_changed=0" \
  "global_host_handle_retirement_allowed=0" \
  "global_arc_retirement_allowed=0" \
  "helper_local_fastpath_allowed=0" \
  "benchmark_name_special_case=0" \
  "helper_name_special_case=0" \
  "raw_array_layout_lowering_without_proof=0" \
  "fallback_to_generic_host_handle_required=1" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-CANDIDATE-INVENTORY-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "receiver_route_known=1" \
  "receiver_route_is_closed_world=1" \
  "receiver_route_is_plugin_or_dynamic=0" \
  "receiver_route_uses_reflection_or_by_name=0" \
  "receiver_storage_plan_known=1" \
  "receiver_storage_is_exact=1" \
  "receiver_storage_requires_host_handle=0" \
  "receiver_handle_publication_required=0" \
  "dynamic_escape_count=0" \
  "plugin_or_extern_escape_count=0" \
  "reflection_or_by_name_route_count=0" \
  "host_handle_publication_count=0" \
  "eligible_site_count>=1" \
  "selected_candidate_confidence=high"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$DESIGN_CARD" "selected_design=closed_world_handle_resolution_plan"
require_line_in_file "$DESIGN_CARD" "implementation_allowed=0"
require_line_in_file "$DESIGN_CARD" "routeplan_required=1"
require_line_in_file "$DESIGN_CARD" "object_storage_plan_required=1"
require_line_in_file "$DESIGN_CARD" "backend_consumes_plan=1"
require_line_in_file "$DESIGN_CARD" "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-PROOF-SURFACE-001"

require_line_in_file "$ASM_CARD" "selected_owner=handle_registry_typed_handle_boundary"
require_line_in_file "$ASM_CARD" "selected_owner_confidence=high"
require_line_in_file "$ASM_CARD" "helper_local_fastpath_remaining=0"

require_line_in_file "$OBJECT_SSOT" "per_site_host_handle_elimination_allowed_with_closed_world_proof=1"
require_line_in_file "$OBJECT_SSOT" "object_boundary_removal_owner=exact_aot_backend"
require_line_in_file "$OBJECT_SSOT" "mirbuilder_object_boundary_removal_owner=0"
require_line_in_file "$OBJECT_SSOT" "backend_consumes_object_storage_plan=1"

grep -F -q "do not implement backend direct handle bypass from this row" "$CARD" || {
  echo "[mimalloc-handle-boundary-proof-surface] missing backend-bypass stop line" >&2
  exit 1
}
grep -F -q "do not edit nyash_array_length_h from this row" "$CARD" || {
  echo "[mimalloc-handle-boundary-proof-surface] missing helper-edit stop line" >&2
  exit 1
}

echo "[mimalloc-handle-boundary-proof-surface] ok"
