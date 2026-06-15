---
Status: Landed
Date: 2026-06-15
Task: EXACT-OBJECT-PILOT-MEASUREMENT-001
Scope: Measure the first guarded exact-object pilot without changing product
  NyRT defaults or making global Arc retirement claims.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-723-EXACT-OBJECT-PILOT-001U.md
---

# EXACT-OBJECT-PILOT-MEASUREMENT-001

## Purpose

`EXACT-OBJECT-PILOT-001U` enabled the first narrow exact-object pilot:

```text
target_front=object_lifecycle_body
nested_owner=HakoAllocObjectLifecycleFacade.alignment_result
nested_object=HakoAllocObjectLifecycleAlignmentResult
representation_choice=flatten_nested_fields
```

This row measures the result and selects the next owner from evidence.

## Result

```text
output_contract=hako-exact-object-pilot-measurement-v0
source_evidence=296x-723
target_front=object_lifecycle_body
pilot_exact_object_enabled=1
product_default_changed=0
global_arc_retirement_claim=0
body_elapsed_ratio_before=112.969
body_elapsed_ratio_after=114.326
hako_body_elapsed_ns_after=374000000
c_body_elapsed_ns_after=3271344
measurement_pair_report=/tmp/row724-exact-object-pilot-measurement/pair.out
winner_claim=0
selected_next=EXACT-OBJECT-PILOT-EFFECT-ATTRIBUTION-001
summary=ok
```

The pilot executes successfully, but the selected body surface did not improve.
Do not claim `EXACT-OBJECT-PILOT-001` success from this row.

The next row must attribute whether the enabled flattened-nested-field route is
actually present in the generated exact-AOT hot path.  If the route is absent,
the owner is backend route reachability.  If the route is present but not a
meaningful hot owner, close this exact-object pilot and return to
perf-owner-first selection.

## Required Output

```text
output_contract=hako-exact-object-pilot-measurement-v0
source_evidence=296x-723
target_front=object_lifecycle_body
pilot_exact_object_enabled=1
product_default_changed=0
global_arc_retirement_claim=0
body_elapsed_ratio_before=<n>
body_elapsed_ratio_after=<n>
selected_next=<task|closeout>
summary=<ok|blocked>
```

## Task List

Run these in order. Do not start another object-representation implementation
before this measurement row selects the next owner.

```text
1. Measure the enabled pilot
   - run the object-lifecycle Hako/C body timing pair
   - keep product_default_changed=0
   - keep global_arc_retirement_claim=0

2. Classify the result
   - if the pilot fails to run, select a narrow backend correctness/fallback row
   - if the pilot runs but body ratio is noisy, repeat measurement before any
     implementation claim
   - if the pilot runs and improves the selected body surface, close the pilot
     with exact-AOT product-route evidence only

3. Select the next owner
   - if remaining owner is still object-boundary visible, create a new
     ObjectStoragePlan proof row
   - if remaining owner moves to generated runtime/helper boundary, return to
     perf-owner-first selection
   - if no high-confidence owner remains, close the exact-object pilot lane and
     return to the current mimalloc/body-timing taskboard
```

Acceptance for this row is measurement and owner selection only:

```text
implementation_started_after_measurement=0
mirbuilder_object_management_enabled=0
benchmark_name_branch_count=0
helper_name_branch_count=0
product_default_changed=0
global_arc_retirement_claim=0
```

## Stop Line

```text
do not claim product NyRT default speedup
do not generalize to global Arc retirement
do not change MIRBuilder behavior
do not add benchmark/helper-name branches
do not start another ObjectStoragePlan implementation before effect attribution
```
