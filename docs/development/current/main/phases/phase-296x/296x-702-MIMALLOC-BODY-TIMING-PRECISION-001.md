---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-BODY-TIMING-PRECISION-001
Scope: Classify the body-timing precision mismatch before reopening compiler
  or runtime/object boundary optimization.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-701-MIMALLOC-COMPILER-LOWERING-OPTIMIZATION-CHECKPOINT-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-BODY-TIMING-PRECISION-001

## Purpose

296x-701 paused compiler-lowering optimization:

```text
compiler_lowering_optimization_pause=1
stable_body_elapsed_ratio=1.790
fresh_body_elapsed_ratio=1.865
next_compiler_owner_selected=0
```

This row classifies whether the remaining 1.8x body gap is a real
runtime/object boundary or partly a measurement-boundary mismatch.

## Required Output

```text
output_contract=hako-mimalloc-body-timing-precision-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-701
hako_timer_family=workload-body-env-now-ms-v0
c_timer_family=workload-body-monotonic-v0
timer_family_matched=0
hako_timer_resolution_ns=1000000
c_timer_resolution_ns=<n|unknown>
body_elapsed_ratio_raw=<ratio>
body_elapsed_ratio_precision_confidence=<low|medium|high>
measurement_boundary_confidence=<low|medium|high>
selected_next_owner=<runtime_boundary_inventory|pause>
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
do not claim a new winner
```

## Acceptance

```text
output_contract=hako-mimalloc-body-timing-precision-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-701
hako_timer_family=workload-body-env-now-ms-v0
c_timer_family=workload-body-monotonic-v0
timer_family_matched=0
hako_timer_resolution_ns=1000000
c_timer_resolution_ns=unknown
body_elapsed_ratio_raw=1.579
body_elapsed_ratio_precision_confidence=low
measurement_boundary_confidence=low
selected_next_owner=runtime_boundary_inventory
implementation_started=0
compiler_lowering_changed=0
runtime_object_changed=0
product_default_changed=0
startup_lane_reopened=0
source_hako_changed=0
winner_claim=0
next_task=mimalloc_runtime_boundary_inventory
summary=ok
```

## Result

`MIMALLOC-BODY-TIMING-PRECISION-001` does not select a new compiler owner.
The current Hako and C body timing reports still use different timer families:
Hako reports through the millisecond-resolution environment timer, while the C
side uses the monotonic timer route.

```text
output_contract=hako-mimalloc-body-timing-precision-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-701
hako_timer_family=workload-body-env-now-ms-v0
c_timer_family=workload-body-monotonic-v0
timer_family_matched=0
hako_timer_resolution_ns=1000000
c_timer_resolution_ns=unknown
hako_body_elapsed_ns=7000000
c_body_elapsed_ns=4433160
body_elapsed_ratio_raw=1.579
body_elapsed_ratio_precision_confidence=low
measurement_boundary_confidence=low
selected_next_owner=runtime_boundary_inventory
implementation_started=0
compiler_lowering_changed=0
runtime_object_changed=0
product_default_changed=0
startup_lane_reopened=0
source_hako_changed=0
winner_claim=0
next_task=mimalloc_runtime_boundary_inventory
summary=ok
```
