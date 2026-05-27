---
Status: Landed
Date: 2026-05-27
Scope: apply source/MIR observation to multiple object-lifecycle methods and select the next keeper candidate.
Blocker: HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION-296X-001
Related:
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-87-HAKO-MIR-METHOD-SHAPE-HAKO-MIGRATION-SELECTION.md
---

# 296x-88 Hako Mimalloc Multi-Method Source/MIR Observation

## Purpose

Use the Python source/MIR observation stack across multiple object-lifecycle
methods before selecting the next keeper. Keep `.hako` MIR migration parked.

Candidate methods:

```text
HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
HakoAllocObjectLifecyclePageQueue.selectPage/0
```

## Required Output

```text
output_contract=hako-mimalloc-multi-method-source-mir-observation-v0
input_contract=hako-mir-method-shape-hako-migration-selection-v0
method_count
confirmed_source_mir_risk_count
selected_method
selected_risk_kind
next_keeper
summary=ok
```

## Stop Line

Do not implement the keeper in this row. Do not migrate MIR observation to
`.hako`.

## Tool Stop Finding

Multi-method source/MIR observation found a tool gap before the next keeper
selection.

Observed reports:

```text
objectLifecycleSmallAlloc:
  source_loop_array_access_count=0
  source_loop_field_access_count=0
  source_loop_method_call_count=0
  mir_array_access_count=1
  mir_field_access_count=16
  mir_call_count=25
  source_risk_confirmed_in_mir=0

objectLifecycleReleaseBlock:
  source_loop_array_access_count=0
  source_loop_field_access_count=0
  source_loop_method_call_count=0
  mir_array_access_count=1
  mir_field_access_count=3
  mir_call_count=21
  source_risk_confirmed_in_mir=0

selectPage:
  source_loop_array_access_count=1
  source_loop_field_access_count=6
  source_loop_method_call_count=0
  mir_array_access_count=1
  mir_field_access_count=15
  mir_call_count=9
  source_risk_confirmed_in_mir=1
```

Interpretation:

`hako_source_mir_shape_join.py` currently confirms only loop-local source
risks. That is too narrow for allocator methods that are hot because an outer
workload loop calls them repeatedly. `objectLifecycleSmallAlloc` and
`objectLifecycleReleaseBlock` have meaningful MIR call/field/array cost but no
source-local loop, so the join adapter reports them as unconfirmed.

Next tool improvement before keeper selection:

```text
output_contract=hako-source-mir-shape-join-v1
method_hot_context=direct_loop|caller_repeated|unknown
source_method_call_count
source_field_get_count
source_field_set_count
source_array_access_count
mir_call_count
mir_field_access_count
mir_array_access_count
source_risk_confirmed_in_mir=0|1
confirmed_risk_kind=array_access|field_access|method_call|none
summary=ok
```

Stop decision:

Do not select the next keeper from row88 until the join adapter can distinguish
loop-local risk from repeated-method hot context.

Tool update:

```text
tool=tools/allocator/hako_source_mir_shape_join.py
guard=tools/checks/k2_wide_phase296x_hako_source_mir_shape_join_v1_guard.sh
default_output_contract=hako-source-mir-shape-join-v1
method_hot_context_auto=direct_loop|caller_repeated|unknown
v0_output_available=--contract-version v0
summary=ok
```

The next row88 observation pass may use the v1 join output before selecting the
next keeper. Keep the keeper implementation outside this row.

Verification:

```text
objectLifecycleSmallAlloc:
  method_hot_context=caller_repeated
  source_risk_confirmed_in_mir=1
  confirmed_risk_kind=array_access

objectLifecycleReleaseBlock:
  method_hot_context=caller_repeated
  source_risk_confirmed_in_mir=1
  confirmed_risk_kind=array_access

selectPage:
  method_hot_context=direct_loop
  source_risk_confirmed_in_mir=1
  confirmed_risk_kind=array_access
```

## Landed Evidence

```text
output_contract=hako-mimalloc-multi-method-source-mir-observation-v0
input_contract=hako-source-mir-shape-join-v1
method_count=3
confirmed_source_mir_risk_count=3
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
selected_source_method=objectLifecycleSmallAlloc
selected_hot_context=caller_repeated
selected_risk_kind=array_access
next_keeper=small_alloc_selected_page_return_reuse
next_keeper_kind=box_count
next_row=HAKO-MIMALLOC-SMALL-ALLOC-SELECTED-PAGE-RETURN-KEEPER-296X-001
winner_claim=0
replacement_active=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_multi_method_source_mir_observation_guard.sh
```
