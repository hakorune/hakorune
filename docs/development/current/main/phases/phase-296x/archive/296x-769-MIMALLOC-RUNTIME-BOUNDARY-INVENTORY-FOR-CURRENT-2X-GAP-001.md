---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-FOR-CURRENT-2X-GAP-001
Scope: Inventory runtime/object/generated-runtime boundary evidence for the
  current reproducible ~2x object-lifecycle body gap after closing the stale
  79.586x measurement outlier.
Related:
  - docs/development/current/main/phases/phase-296x/296x-768-MIMALLOC-MEASUREMENT-STATE-PROVENANCE-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-767-MIMALLOC-MEASUREMENT-HYGIENE-REFRESH-001.md
  - docs/development/current/main/phases/phase-296x/296x-703-MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-FOR-CURRENT-2X-GAP-001

## Purpose

296x-768 closed the old row753 79.586x body gap as stale/transient measurement
evidence. This row inventories the current reproducible ~2x body gap and
decides whether implementation can reopen.

This is an inventory row, not an implementation row.

## Result

```text
output_contract=hako-mimalloc-runtime-boundary-inventory-for-current-2x-gap-v0
source_evidence=296x-768,296x-767,296x-703
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
old_large_gap_classification=stale_or_transient_hako_runner_measurement_outlier
old_large_gap_allowed_as_optimization_owner=0
current_body_elapsed_ratio_median=2.119
current_body_elapsed_ratio_max=2.363
current_reliable_body_ratio_floor=about_2x
measurement_boundary_confidence=low
box_callable_registry_visible=1
routeplan_visible=1
box_method_boundary_visible=1
routeplan_slow_dynamic_hit_count=unknown
arc_dynbox_boundary_visible=1
object_refcount_boundary_visible=1
object_handle_boundary_visible=1
host_handle_boundary_visible=1
runtime_helper_call_boundary_visible=1
generated_runtime_boundary_visible=1
body_timer_env_now_boundary_visible=1
mixed_runtime_boundary_visible=1
single_high_confidence_owner_selected=0
selected_owner=none
selected_owner_confidence=low
owner_reason=mixed_runtime_object_generated_runtime_boundaries_visible_without_single_current_hot_owner
closed_world_routeplan_allowed=0
exact_aot_specialization_selected=0
implementation_allowed=0
implementation_started=0
compiler_lowering_changed=0
runtime_object_changed=0
product_default_changed=0
startup_lane_reopened=0
source_hako_changed=0
mirbuilder_object_management_enabled=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
next_task=current_2x_asm_boundary_attribution
summary=ok
```

## Evidence

The old large-gap measurement is not allowed to drive another implementation:

```text
old_large_gap_classification=stale_or_transient_hako_runner_measurement_outlier
old_large_gap_allowed_as_optimization_owner=0
```

The current direct-exact pair is much smaller but still not explained by a
single owner:

```text
current_body_elapsed_ratio_median=2.119
current_body_elapsed_ratio_max=2.363
measurement_boundary_confidence=low
```

The repo contains the expected boundary seams:

```text
box_callable_registry_visible=1
routeplan_visible=1
arc_dynbox_boundary_visible=1
object_handle_boundary_visible=1
host_handle_boundary_visible=1
body_timer_env_now_boundary_visible=1
```

Those seams are not enough to justify implementation. They prove that multiple
runtime/object/generated-runtime boundaries are still visible; they do not prove
which one dominates the current ~2x gap.

## Decision

Do not reopen compiler-lowering or object-representation implementation from
this inventory alone.

The next row must attribute the current ~2x body gap with assembly/perf evidence
under the current canonical direct-exact pair. Only a fresh high-confidence hot
owner may reopen implementation.

## Stop Line

```text
do not implement from the stale 79.586x gap
do not reopen LocalSSA, PHI-edge, block-entry, receiver, arg, or route-carrier
  forwarding without fresh owner evidence
do not move Box/Object management into MIRBuilder
do not change runtime object representation
do not change source .hako
do not change product defaults, provider activation, replacement, hooks, or
  global allocator
do not add benchmark-name, source-name, helper-name, or method-name special cases
do not claim exact-AOT closed-world specialization before a high-confidence
  owner appears
```

## Next

```text
MIMALLOC-CURRENT-2X-ASM-BOUNDARY-ATTRIBUTION-001:
  run a current canonical direct-exact assembly/perf attribution pass for the
  ~2x object-lifecycle body gap, classify the hot symbol / hot block / boundary
  family, and keep implementation closed unless one high-confidence owner is
  selected
```
