#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-769-MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-FOR-CURRENT-2X-GAP-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-768-MIMALLOC-MEASUREMENT-STATE-PROVENANCE-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_runtime_boundary_inventory_for_current_2x_gap_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-current-2x-runtime-boundary] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-current-2x-runtime-boundary] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-current-2x-runtime-boundary] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[mimalloc-current-2x-runtime-boundary] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-current-2x-runtime-boundary] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-current-2x-runtime-boundary] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-runtime-boundary-inventory-for-current-2x-gap-v0" \
  "source_evidence=296x-768,296x-767,296x-703" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "old_large_gap_classification=stale_or_transient_hako_runner_measurement_outlier" \
  "old_large_gap_allowed_as_optimization_owner=0" \
  "current_body_elapsed_ratio_median=2.119" \
  "current_body_elapsed_ratio_max=2.363" \
  "current_reliable_body_ratio_floor=about_2x" \
  "measurement_boundary_confidence=low" \
  "box_callable_registry_visible=1" \
  "routeplan_visible=1" \
  "box_method_boundary_visible=1" \
  "routeplan_slow_dynamic_hit_count=unknown" \
  "arc_dynbox_boundary_visible=1" \
  "object_refcount_boundary_visible=1" \
  "object_handle_boundary_visible=1" \
  "host_handle_boundary_visible=1" \
  "runtime_helper_call_boundary_visible=1" \
  "generated_runtime_boundary_visible=1" \
  "body_timer_env_now_boundary_visible=1" \
  "mixed_runtime_boundary_visible=1" \
  "single_high_confidence_owner_selected=0" \
  "selected_owner=none" \
  "selected_owner_confidence=low" \
  "owner_reason=mixed_runtime_object_generated_runtime_boundaries_visible_without_single_current_hot_owner" \
  "closed_world_routeplan_allowed=0" \
  "exact_aot_specialization_selected=0" \
  "implementation_allowed=0" \
  "implementation_started=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "product_default_changed=0" \
  "startup_lane_reopened=0" \
  "source_hako_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "provider_active=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "winner_claim=0" \
  "next_task=current_2x_asm_boundary_attribution" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "MIMALLOC-CURRENT-2X-ASM-BOUNDARY-ATTRIBUTION-001:" "$CARD" || {
  echo "[mimalloc-current-2x-runtime-boundary] next asm boundary attribution row is not documented" >&2
  exit 1
}

for rel_needle in \
  "pub struct BoxCallableRegistry" \
  "pub enum MethodCallRoutePlan" \
  "ArcDynNyashBox" \
  "pub struct ObjectHandle" \
  "StableBox(Arc<dyn NyashBox>)"; do
  if ! rg -F -q "$rel_needle" src; then
    echo "[mimalloc-current-2x-runtime-boundary] repo evidence missing: $rel_needle" >&2
    exit 1
  fi
done

rg -F -q "env.now_ms()" apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako || {
  echo "[mimalloc-current-2x-runtime-boundary] body timer env.now_ms boundary evidence missing" >&2
  exit 1
}

echo "[mimalloc-current-2x-runtime-boundary] ok"
