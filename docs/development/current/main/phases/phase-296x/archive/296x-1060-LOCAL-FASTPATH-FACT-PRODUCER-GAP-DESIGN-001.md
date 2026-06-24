Status: Done
Date: 2026-06-17
Scope: local fastpath fact producer gap design
Related:
  - docs/development/current/main/phases/phase-296x/296x-1059-FRESH-COMPILER-OWNER-SELECTION-006.md
  - docs/development/current/main/phases/phase-296x/296x-1042-FASTPATH-GAP-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-902-LOCAL-FASTPATH-FACT-PRODUCER-SELECTION-001.md

# LOCAL-FASTPATH-FACT-PRODUCER-GAP-DESIGN-001

## Purpose

Close the design gap selected by `FRESH-COMPILER-OWNER-SELECTION-006`:
current MIR has known direct same-module user-box method routes, but no
positive `LocalFastPathFact` entries for those callsites.

This row is design only. It does not add facts, does not change backend
lowering, and does not change route priority.

## Current Shape

The current positive fact producer is map-specific:

```text
producer=src/mir/map_repr_plan/fastpath.rs
input=MapReprPlan
output=LocalFastPathFact
selected_route=map_repr.generic_hash_runtime
```

`refresh_function_map_repr_plans()` currently assigns:

```text
function.metadata.local_fastpath_facts =
  build_local_fastpath_facts_from_map_repr_plans(&plans)
```

This makes `map_repr_plan` the current metadata writer. Adding a user-box
producer by appending elsewhere would create multiple assignment owners or
clobber order hazards.

The MIR JSON exporter also currently emits:

```text
route_plan=map_repr.generic_hash_runtime
```

for every `LocalFastPathFact`. That is correct for the current map-only
producer, but it would be wrong for user-box method facts.

## Evidence

From the selected target in `FRESH-COMPILER-OWNER-SELECTION-006`:

```text
front=object_lifecycle_body
function=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
known_receiver_direct_method_route_count=19
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=19
```

Whole artifact context:

```text
known_receiver_direct_method_route_count=184
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=184
```

The missing facts are not backend failures yet. They are producer-surface
failures: route metadata exists, but the backend-consumable positive fact
surface has no owner for user-box direct method routes.

## Decision

Select a single aggregation owner for all `LocalFastPathFact` metadata:

```text
selected_owner=local_fastpath_fact_aggregator
selected_assignment_owner=function.metadata.local_fastpath_facts
```

The aggregator owns the final assignment to
`function.metadata.local_fastpath_facts`. Existing producer families become
inputs:

```text
map_repr_plan:
  role=producer_input
  output=positive facts for map scalar no-publication get

user_box_method_routes:
  role=producer_input
  output=positive facts for known direct same-module receiver method calls
```

This prevents two failure modes:

```text
multiple_metadata_assignment_owners
  avoided by single aggregator assignment

wrong_route_plan_label
  avoided by making route-plan label an explicit fact/export concern
```

## Required Implementation Split

The implementation must be split into narrow rows:

```text
1. LOCAL-FASTPATH-FACT-AGGREGATOR-SURFACE-001
   - create the aggregation owner
   - move final local_fastpath_facts assignment out of map_repr refresh
   - keep existing map facts behavior unchanged

2. LOCAL-FASTPATH-FACT-ROUTE-LABEL-SURFACE-001
   - remove MIR JSON hardcoded map_repr route_plan label
   - make each fact export its producer/route-plan label accurately
   - keep backend behavior unchanged

3. USER-BOX-METHOD-LOCAL-FASTPATH-FACT-PRODUCER-001
   - produce facts from direct same-module user_box_method_routes
   - no backend winner claim
   - no fallback facts
```

Rows 1 and 2 may be combined only if the diff remains behavior-preserving and
the route label is required to avoid incorrect metadata.

## Contract

```text
output_contract=local-fastpath-fact-producer-gap-design-v0
source_evidence=296x-1059,target/tmp/mimalloc_object_lifecycle.mir.json

selected_owner=local_fastpath_fact_aggregator
single_metadata_assignment_owner=local_fastpath_fact_aggregator
map_repr_plan_fact_role=producer_input
user_box_method_route_fact_role=producer_input

current_map_repr_refresh_writes_local_fastpath_facts=1
target_map_repr_refresh_writes_local_fastpath_facts=0

current_mir_json_route_plan_label_hardcoded_map_repr=1
target_mir_json_route_plan_label_hardcoded_map_repr=0
implementation_requires_route_plan_label_source=1

user_box_fact_route_plan_label=user_box.method_call
map_repr_fact_route_plan_label=map_repr.generic_hash_runtime

known_receiver_direct_method_route_count=19
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=19

fallback_fact_enabled=0
backend_lowering_changed=0
route_priority_changed=0
winner_claim_allowed=0
implementation_started=0

next_task=LOCAL-FASTPATH-FACT-AGGREGATOR-SURFACE-001
summary=ok
```

## Stop Lines

```text
do not add backend lowering in this row
do not add user-box facts before route_plan labels are accurate
do not let map_repr_plan and user_box_method_route_plan both assign
  function.metadata.local_fastpath_facts
do not emit a user-box fact with route_plan=map_repr.generic_hash_runtime
do not create fallback facts
do not infer facts from source names, helper names, or benchmark names
do not change route priority
do not claim a body-time or ASM win
```

## Validation

Design row only:

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
