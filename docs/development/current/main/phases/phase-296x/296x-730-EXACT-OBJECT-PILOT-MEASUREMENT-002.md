---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-PILOT-MEASUREMENT-002
Scope: Measure the ny-llvmc-boundary exact-object pilot after ObjectStoragePlan
  metadata reaches the measured route.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-729-EXACT-OBJECT-PILOT-001V.md
---

# EXACT-OBJECT-PILOT-MEASUREMENT-002

## Purpose

`EXACT-OBJECT-PILOT-001V` proved that the selected exact-object pilot is enabled
through the measured `ny-llvmc` boundary route:

```text
boundary_driver_flattened_nested_consumer=1
field_access_lowering_connected=1
nested_method_lowering_connected=1
generated_artifact_reachability_proven=1
pilot_exact_object_enabled=1
```

This row measures the product exact-AOT route after that reachability fix.  It
must not change implementation while measuring.

## Required Output

```text
output_contract=hako-exact-object-pilot-measurement-002-v0
source_evidence=296x-729
target_front=object_lifecycle_body
pilot_exact_object_enabled=1
product_default_changed=0
global_arc_retirement_claim=0
body_elapsed_ratio_before=<n>
body_elapsed_ratio_after=<n>
winner_claim=<0|1>
selected_next=<task|closeout>
summary=<ok|blocked>
```

## Result

```text
output_contract=hako-exact-object-pilot-measurement-002-v0
source_evidence=296x-729
target_front=object_lifecycle_body
pilot_exact_object_enabled=1
product_default_changed=0
global_arc_retirement_claim=0
body_elapsed_ratio_before=114.326
body_elapsed_ratio_after=117.038
hako_body_elapsed_ns_after=368000000
c_body_elapsed_ns_after=3144269
measurement_pair_report=/tmp/hakorune_row730_measure.wRCV4q/pair.out
winner_claim=0
selected_next=EXACT-OBJECT-PILOT-CLOSEOUT-001
summary=ok
```

The measured route now consumes the ObjectStoragePlan, but the product exact-AOT
body timing did not improve.  Do not open another ObjectStoragePlan
implementation row from this evidence alone.  Close the pilot as a no-keeper
exact-AOT boundary experiment and return to fresh owner selection.

Proof:

```text
bash tools/allocator/hako_exe_memory_runner.sh --app apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako --workload representative-object-lifecycle-small-block-v0 --runtime-config empty --operation-repeat 1 --out /tmp/hakorune_row730_measure.wRCV4q/hako.out
bash tools/allocator/c_mimalloc_explicit_runner.sh --out /tmp/hakorune_row730_measure.wRCV4q/c.out --allow-ldconfig-discovery --workload representative-object-lifecycle-small-block-v0 --in-process-repeat 8192 --operation-repeat 1
python3 tools/allocator/hako_mimalloc_object_lifecycle_body_timing_pair_adapter.py --hako-report /tmp/hakorune_row730_measure.wRCV4q/hako.out --c-report /tmp/hakorune_row730_measure.wRCV4q/c.out --out /tmp/hakorune_row730_measure.wRCV4q/pair.out
tools/allocator/hako_exact_object_pilot_measurement_002.py --pair-report /tmp/hakorune_row730_measure.wRCV4q/pair.out
```

## Task List

```text
1. Run the object-lifecycle Hako/C body timing pair for the same front used by
   earlier measurement rows.
2. Record before/after ratio with the previous 724 measurement as historical
   baseline, but do not claim product NyRT default speedup.
3. If the route improves, open pilot closeout as per-site exact-AOT boundary
   win only.
4. If no win, close or select a fresh owner from evidence. Do not add another
   ObjectStoragePlan implementation row from timing alone.
```

## Stop Line

```text
do not change implementation during measurement
do not claim global Arc retirement
do not claim product NyRT default speedup
do not modify MIRBuilder
do not add benchmark/helper-name branches
```
