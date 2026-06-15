---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-BODY-TIMER-ALIGNMENT-PROBE-001
Scope: Reduce Hako body timer granularity risk by scaling the body repeat before
  selecting any runtime boundary implementation owner.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-703-MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-BODY-TIMER-ALIGNMENT-PROBE-001

## Purpose

296x-703 found visible runtime/object/generated-runtime seams but did not select
a single owner because measurement confidence was still low. This row scales
the in-process repeat so Hako's millisecond body timer is a smaller fraction of
the measured body window.

This row does not add `env.now_ns`, does not change runtime time APIs, and does
not patch source `.hako`. The direct-exact pair runner may generate a temporary
scaled app in `/tmp` only.

## Required Output

```text
output_contract=hako-mimalloc-body-timer-alignment-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-703
baseline_body_elapsed_ratio=<ratio>
scaled_in_process_repeat=<n>
scaled_hako_body_elapsed_ns=<n>
scaled_c_body_elapsed_ns=<n>
scaled_body_elapsed_ratio=<ratio>
hako_timer_family=workload-body-env-now-ms-v0
c_timer_family=workload-body-monotonic-v0
timer_family_matched=0
hako_timer_resolution_ns=1000000
scaled_hako_timer_resolution_pct=<pct>
body_elapsed_ratio_precision_confidence=<low|medium|high>
selected_next_owner=<runtime_boundary_direct_probe|repeat_alignment_retry>
selected_next_owner_confidence=<low|medium|high>
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
do not patch tracked source .hako
do not add env.now_ns or timing substrate in this row
do not reopen startup optimization
do not claim exact-AOT closed-world route specialization
```

## Acceptance

```text
output_contract=hako-mimalloc-body-timer-alignment-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-703
baseline_body_elapsed_ratio=1.579
scaled_in_process_repeat=65536
scaled_hako_body_elapsed_ns=53000000
scaled_c_body_elapsed_ns=25998914
scaled_body_elapsed_ratio=2.039
hako_timer_family=workload-body-env-now-ms-v0
c_timer_family=workload-body-monotonic-v0
timer_family_matched=0
hako_timer_resolution_ns=1000000
scaled_hako_timer_resolution_pct=1.887
body_elapsed_ratio_precision_confidence=medium
selected_next_owner=runtime_boundary_direct_probe
selected_next_owner_confidence=medium
implementation_started=0
compiler_lowering_changed=0
runtime_object_changed=0
product_default_changed=0
startup_lane_reopened=0
source_hako_changed=0
winner_claim=0
next_task=runtime_boundary_direct_probe
summary=ok
```

## Result

The scaled repeat reduces Hako's 1ms body timer resolution to less than 2% of
the measured Hako body window. The Hako/C gap remains around 2x, so the next
owner should be a direct runtime boundary probe rather than another compiler
lowering implementation.

```text
output_contract=hako-mimalloc-body-timer-alignment-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-703
baseline_body_elapsed_ratio=1.579
scaled_in_process_repeat=65536
scaled_hako_body_elapsed_ns=53000000
scaled_c_body_elapsed_ns=25998914
scaled_body_elapsed_ratio=2.039
hako_timer_family=workload-body-env-now-ms-v0
c_timer_family=workload-body-monotonic-v0
timer_family_matched=0
hako_timer_resolution_ns=1000000
scaled_hako_timer_resolution_pct=1.887
body_elapsed_ratio_precision_confidence=medium
selected_next_owner=runtime_boundary_direct_probe
selected_next_owner_confidence=medium
owner_reason=scaled_body_timer_resolution_small_but_gap_remains
implementation_started=0
compiler_lowering_changed=0
runtime_object_changed=0
product_default_changed=0
startup_lane_reopened=0
source_hako_changed=0
winner_claim=0
next_task=runtime_boundary_direct_probe
summary=ok
```
