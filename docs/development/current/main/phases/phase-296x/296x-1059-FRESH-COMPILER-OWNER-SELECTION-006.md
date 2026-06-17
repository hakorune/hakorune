Status: Done
Date: 2026-06-17
Scope: fresh compiler owner selection after object-storage vocabulary cleanup
Related:
  - docs/development/current/main/phases/phase-296x/296x-1042-FASTPATH-GAP-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-1058-FASTPATH-DENY-OWNER-CODE-RETIRE-001.md
Artifact:
  - target/tmp/mimalloc_object_lifecycle.mir.json

# FRESH-COMPILER-OWNER-SELECTION-006

## Purpose

Return from object-storage vocabulary cleanup to the active compiler fastpath
lane and select the next owner from current MIR evidence.

This row is selection only. It does not add a backend fastpath, does not change
route priority, and does not treat direct route metadata as a
`LocalFastPathFact`.

## Evidence

Refreshed `FASTPATH-GAP-INVENTORY-001` on the current object-lifecycle MIR:

```bash
python3 tools/hako_check/fastpath_gap_inventory.py \
  --mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  --method 'HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' \
  --front object_lifecycle_body
```

Focused target:

```text
front=object_lifecycle_body
function_filter=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
function_count=1
known_receiver_direct_method_route_count=19
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=19
thin_entry_method_candidate_count=19
top_gap_function=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
top_gap_count=19
top_missing_subject=HakoAllocObjectLifecycleAllocResult.recordFailureAfterSelectedPage/1
top_missing_subject_count=4
```

Whole artifact context:

```text
function_count=255
known_receiver_direct_method_route_count=184
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=184
thin_entry_method_candidate_count=205
top_gap_function=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
top_gap_count=19
```

## Decision

Select the next owner as the positive fact producer gap:

```text
selected_owner=local_fastpath_fact_producer_gap
selected_front=object_lifecycle_body
selected_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
selected_owner_confidence=high
implementation_started=0
```

The evidence says:

```text
RoutePlan/user_box_method_routes:
  already has known direct same-module method routes

FunctionMetadata.local_fastpath_facts:
  missing for those same callsites

Backend consumer:
  not the next owner in this row
```

## Not Selected

```text
backend_lowering:
  not selected
  reason=positive fact is missing before backend can consume it

route_priority:
  not selected
  reason=route preemption is not the observed gap in this artifact

fallback_fact:
  not selected
  reason=fallback evidence must not become a backend fact

source_shape_matcher:
  not selected
  reason=direct route metadata already exists; do not infer from source names
```

## Contract

```text
output_contract=fresh-compiler-owner-selection-v6
source_evidence=296x-1042,target/tmp/mimalloc_object_lifecycle.mir.json
fresh_compiler_optimization_owner_selected=1
selected_owner=local_fastpath_fact_producer_gap
selected_owner_confidence=high

known_receiver_direct_method_route_count=19
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=19
thin_entry_method_candidate_count=19

whole_known_receiver_direct_method_route_count=184
whole_local_fastpath_fact_count=0
whole_known_receiver_direct_method_without_fact_count=184

fallback_evidence_fact_enabled=0
backend_lowering_changed=0
route_priority_changed=0
winner_claim_allowed=0
implementation_started=0

next_task=LOCAL-FASTPATH-FACT-PRODUCER-GAP-DESIGN-001
summary=ok
```

## Stop Lines

```text
do not emit backend code in this row
do not treat user_box_method_routes as LocalFastPathFact
do not create fallback facts
do not change route priority
do not add source-name or helper-name special cases
do not proceed to implementation before fact producer design
```

## Validation

```text
python3 tools/hako_check/fastpath_gap_inventory.py \
  --mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  --method 'HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' \
  --front object_lifecycle_body

python3 tools/hako_check/fastpath_gap_inventory.py \
  --mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  --front object_lifecycle_body

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
