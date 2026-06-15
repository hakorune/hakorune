#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-771-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-DESIGN-001.md"
ASM_CARD="docs/development/current/main/phases/phase-296x/296x-770-MIMALLOC-CURRENT-2X-ASM-BOUNDARY-ATTRIBUTION-001.md"
HELPER_CARD="docs/development/current/main/phases/phase-296x/296x-709-MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001.md"
PILOT_CLOSEOUT="docs/development/current/main/phases/phase-296x/296x-731-EXACT-OBJECT-PILOT-CLOSEOUT-001.md"
OBJECT_SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-handle-boundary-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$ASM_CARD" ]] || { echo "[mimalloc-handle-boundary-design] missing asm card: $ASM_CARD" >&2; exit 1; }
[[ -f "$HELPER_CARD" ]] || { echo "[mimalloc-handle-boundary-design] missing helper card: $HELPER_CARD" >&2; exit 1; }
[[ -f "$PILOT_CLOSEOUT" ]] || { echo "[mimalloc-handle-boundary-design] missing pilot closeout: $PILOT_CLOSEOUT" >&2; exit 1; }
[[ -f "$OBJECT_SSOT" ]] || { echo "[mimalloc-handle-boundary-design] missing object SSOT: $OBJECT_SSOT" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-handle-boundary-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$ASM_CARD" || {
  echo "[mimalloc-handle-boundary-design] asm card must be Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$HELPER_CARD" || {
  echo "[mimalloc-handle-boundary-design] helper card must be Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PILOT_CLOSEOUT" || {
  echo "[mimalloc-handle-boundary-design] pilot closeout must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-handle-boundary-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-handle-boundary-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-design-v0" \
  "source_evidence=296x-770,296x-709,296x-731" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "remaining_owner=handle_registry_typed_handle_boundary" \
  "remaining_owner_confidence=high" \
  "selected_design=closed_world_handle_resolution_plan" \
  "selected_design_confidence=medium" \
  "implementation_allowed=0" \
  "helper_local_fastpath_allowed=0" \
  "global_host_handle_retirement_allowed=0" \
  "global_arc_retirement_allowed=0" \
  "mirbuilder_object_management_enabled=0" \
  "routeplan_required=1" \
  "object_storage_plan_required=1" \
  "backend_consumes_plan=1" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "benchmark_name_special_case=0" \
  "helper_name_special_case=0" \
  "raw_array_layout_lowering_without_proof=0" \
  "fallback_to_generic_host_handle_required=1" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-PROOF-SURFACE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

require_line_in_file "$ASM_CARD" "selected_owner=handle_registry_typed_handle_boundary"
require_line_in_file "$ASM_CARD" "selected_owner_confidence=high"
require_line_in_file "$ASM_CARD" "implementation_allowed=0"
require_line_in_file "$ASM_CARD" "design_required=1"
require_line_in_file "$ASM_CARD" "helper_local_fastpath_remaining=0"
require_line_in_file "$HELPER_CARD" "remaining_owner=handle_registry_typed_handle_boundary"
require_line_in_file "$HELPER_CARD" "closed_world_route_required=1"
require_line_in_file "$HELPER_CARD" "object_substrate_required=1"
require_line_in_file "$PILOT_CLOSEOUT" "winner_claim=0"
require_line_in_file "$PILOT_CLOSEOUT" "global_host_handle_retirement_claim=0"
require_line_in_file "$PILOT_CLOSEOUT" "mirbuilder_object_management_enabled=0"

require_line_in_file "$OBJECT_SSOT" "per_site_host_handle_elimination_allowed_with_closed_world_proof=1"
require_line_in_file "$OBJECT_SSOT" "object_boundary_removal_owner=exact_aot_backend"
require_line_in_file "$OBJECT_SSOT" "mirbuilder_object_boundary_removal_owner=0"
require_line_in_file "$OBJECT_SSOT" "backend_consumes_object_storage_plan=1"

rg -n "StableBox\\(Arc<dyn NyashBox>\\)" src/runtime/host_handles.rs >/dev/null || {
  echo "[mimalloc-handle-boundary-design] host handle registry no longer documents StableBox Arc carrier" >&2
  exit 1
}
rg -n "pub fn with_handle_ready" src/runtime/host_handles.rs >/dev/null || {
  echo "[mimalloc-handle-boundary-design] host handle ready lookup seam missing" >&2
  exit 1
}
rg -n "handles::with_handle_ready\\(handle as u64" crates/nyash_kernel/src/plugin/array_handle_cache.rs >/dev/null || {
  echo "[mimalloc-handle-boundary-design] array handle cache no longer uses host handle ready seam" >&2
  exit 1
}
python3 - <<'PY'
from pathlib import Path
text = Path("crates/nyash_kernel/src/plugin/array_compat.rs").read_text(encoding="utf-8")
needle = "pub extern \"C\" fn nyash_array_length_h(handle: i64) -> i64 {\n    with_array_box_ready(handle, |arr| arr.len() as i64).unwrap_or(0)\n}"
if needle not in text:
    raise SystemExit("[mimalloc-handle-boundary-design] nyash_array_length_h is not on borrowed-ready path")
PY

echo "[mimalloc-handle-boundary-design] ok"
