---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-PILOT-CLOSEOUT-001
Scope: Close the first exact-object ObjectStoragePlan pilot as a no-keeper
  boundary experiment and return to fresh owner selection.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-730-EXACT-OBJECT-PILOT-MEASUREMENT-002.md
---

# EXACT-OBJECT-PILOT-CLOSEOUT-001

## Purpose

`EXACT-OBJECT-PILOT-001V` proved that the measured `ny-llvmc` boundary route can
consume the flattened nested ObjectStoragePlan:

```text
boundary_driver_flattened_nested_consumer=1
field_access_lowering_connected=1
nested_method_lowering_connected=1
generated_artifact_reachability_proven=1
pilot_exact_object_enabled=1
```

`EXACT-OBJECT-PILOT-MEASUREMENT-002` then measured the product exact-AOT route
and found no keeper:

```text
body_elapsed_ratio_before=114.326
body_elapsed_ratio_after=117.038
winner_claim=0
```

This closeout prevents the no-win evidence from turning into broader Box
management work.  The pilot is complete as a boundary experiment, but it is not
a performance keeper and does not authorize global object-model replacement.

## Decision

```text
output_contract=hako-exact-object-pilot-closeout-v0
source_evidence=296x-730
target_front=object_lifecycle_body
object_storage_plan_route_reached=1
pilot_exact_object_enabled=1
body_elapsed_ratio_before=114.326
body_elapsed_ratio_after=117.038
winner_claim=0
keeper_claim=0
global_arc_retirement_claim=0
global_host_handle_retirement_claim=0
product_default_changed=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
type_abi_execution_truth=0
selected_next=MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-002
summary=ok
```

## What This Pilot Proved

```text
ObjectStoragePlan can be exported to MIR JSON.
The ny-llvmc boundary shim can consume that plan without using benchmark,
helper, or source-file names as proof.
Flattened nested field state can be shared by owner field access and selected
nested method calls.
The generic product runtime route remains available.
```

## What This Pilot Did Not Prove

```text
It did not improve the measured body timing.
It did not justify another ObjectStoragePlan implementation row.
It did not justify moving Box management into MIRBuilder.
It did not justify global Arc retirement.
It did not justify global HostHandle retirement.
It did not justify changing product NyRT default behavior.
It did not prove that Type ABI / hako_check are execution truth.
```

## Task List

```text
1. Keep the ObjectStoragePlan boundary SSOT as the long-term C-like lowering
   path:
     MIRBuilder records meaning.
     BoxCallableRegistry owns callable truth.
     RoutePlan owns execution route.
     ObjectStoragePlan owns representation.
     exact-AOT backend consumes the plans.

2. Close the first exact-object pilot as no-keeper:
     object_storage_plan_route_reached=1
     winner_claim=0
     keeper_claim=0

3. Do not start a second exact-object implementation from this evidence.
   A new implementation row requires fresh high-confidence owner evidence.

4. Return to body-timing owner selection:
     selected_next=MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-002
```

## Stop Line

```text
do not move Box object management into MIRBuilder
do not start global Arc retirement from this row
do not start global HostHandle retirement from this row
do not add benchmark/helper/source-name branches
do not change product NyRT default behavior
do not open another ObjectStoragePlan implementation row without fresh owner
evidence
```
