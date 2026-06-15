---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-001
Scope: Inventory the remaining mimalloc object-lifecycle body gap as
  runtime/object/generated-runtime boundary, without changing implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-702-MIMALLOC-BODY-TIMING-PRECISION-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-001

## Purpose

296x-702 confirmed that the current 1.5x-1.8x body gap is still measured across
different timer families:

```text
hako_timer_family=workload-body-env-now-ms-v0
c_timer_family=workload-body-monotonic-v0
timer_family_matched=0
measurement_boundary_confidence=low
```

This row classifies the remaining body gap by runtime boundary family before
any new optimization implementation. It is an inventory row, not a keeper row.

## Classification Surface

```text
method route:
  BoxCallable / RoutePlan target visibility
  slow or dynamic route hits
  name/string dispatch residue

object boundary:
  Arc clone/drop
  share_box / clone_box
  host handle / object identity boundary
  birth/drop route

generated runtime:
  exact-AOT helper call boundary
  runtime ABI helper entry
  body timer / env.now route
  safepoint or poll path

measurement:
  timer family mismatch
  harness body extraction
  repeat/warmup boundary
```

## Required Output

```text
output_contract=hako-mimalloc-runtime-boundary-inventory-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-702
body_elapsed_ratio_raw=1.579
measurement_boundary_confidence=low
box_method_boundary_visible=<0|1|unknown>
routeplan_slow_dynamic_hit_count=<n|unknown>
object_refcount_boundary_visible=<0|1|unknown>
host_handle_boundary_visible=<0|1|unknown>
runtime_helper_call_boundary_visible=<0|1|unknown>
generated_runtime_boundary_visible=<0|1|unknown>
selected_owner=<owner|none>
selected_owner_confidence=<low|medium|high>
implementation_started=0
compiler_lowering_changed=0
runtime_object_changed=0
product_default_changed=0
startup_lane_reopened=0
source_hako_changed=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not change compiler lowering
do not change runtime object representation
do not change product NyRT default
do not patch source .hako
do not reopen startup optimization
do not introduce benchmark/helper-name special cases
do not claim exact-AOT closed-world route specialization before a
  high-confidence owner appears
```

## Next If Owner Appears

Only if this row selects a high-confidence runtime boundary owner:

```text
EXACT-AOT-CLOSED-WORLD-ROUTEPLAN-001:
  BoxCallableRegistry -> RoutePlan -> exact-AOT closed-world lowering
  no MIRBuilder truth
  no Type ABI execution truth
  no product default change
```

Otherwise, keep the compiler-lowering lane paused.

## Acceptance

```text
output_contract=hako-mimalloc-runtime-boundary-inventory-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-702
body_elapsed_ratio_raw=1.579
measurement_boundary_confidence=low
box_method_boundary_visible=1
routeplan_slow_dynamic_hit_count=unknown
object_refcount_boundary_visible=1
host_handle_boundary_visible=1
runtime_helper_call_boundary_visible=1
generated_runtime_boundary_visible=1
selected_owner=none
selected_owner_confidence=low
closed_world_routeplan_allowed=0
exact_aot_specialization_selected=0
implementation_started=0
compiler_lowering_changed=0
runtime_object_changed=0
product_default_changed=0
startup_lane_reopened=0
source_hako_changed=0
winner_claim=0
next_task=body_timer_alignment_or_boundary_probe
summary=ok
```

## Result

The inventory finds visible runtime/object/generated-runtime seams, but not a
single high-confidence optimization owner. The body timing boundary is still low
confidence because the timer families remain mismatched, so exact-AOT
closed-world route specialization stays closed for now.

```text
output_contract=hako-mimalloc-runtime-boundary-inventory-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-702
body_elapsed_ratio_raw=1.579
measurement_boundary_confidence=low
box_method_boundary_visible=1
routeplan_slow_dynamic_hit_count=unknown
object_refcount_boundary_visible=1
host_handle_boundary_visible=1
runtime_helper_call_boundary_visible=1
generated_runtime_boundary_visible=1
selected_owner=none
selected_owner_confidence=low
owner_reason=mixed_runtime_boundary_visible_but_measurement_boundary_low_confidence
closed_world_routeplan_allowed=0
exact_aot_specialization_selected=0
implementation_started=0
compiler_lowering_changed=0
runtime_object_changed=0
product_default_changed=0
startup_lane_reopened=0
source_hako_changed=0
winner_claim=0
next_task=body_timer_alignment_or_boundary_probe
summary=ok
```
