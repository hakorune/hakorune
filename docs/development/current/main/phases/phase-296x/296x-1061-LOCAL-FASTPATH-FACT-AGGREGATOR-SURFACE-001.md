Status: Done
Date: 2026-06-17
Scope: LocalFastPathFact aggregation owner surface
Related:
  - docs/development/current/main/phases/phase-296x/296x-1060-LOCAL-FASTPATH-FACT-PRODUCER-GAP-DESIGN-001.md

# LOCAL-FASTPATH-FACT-AGGREGATOR-SURFACE-001

## Purpose

Implement the first behavior-preserving slice of the local fastpath fact
producer design: move the final `function.metadata.local_fastpath_facts`
assignment to a single aggregation owner.

This row does not add user-box facts and does not change backend lowering.

## Implementation

Added:

```text
src/mir/local_fastpath_fact.rs
```

This module owns:

```text
refresh_function_local_fastpath_facts(function)
```

and is now the only source assignment owner for:

```text
function.metadata.local_fastpath_facts
```

Changed `map_repr_plan` from assignment owner to producer input:

```text
before:
  map_repr_plan/refresh.rs assigned local_fastpath_facts directly

after:
  map_repr_plan only publishes MapReprPlan evidence
  local_fastpath_fact aggregator consumes map_repr_plans and assigns facts
```

The existing map scalar no-publication positive fact behavior remains intact.

## Contract

```text
output_contract=local-fastpath-fact-aggregator-surface-v0
source_evidence=296x-1060

local_fastpath_fact_aggregator_defined=1
single_metadata_assignment_owner=local_fastpath_fact_aggregator
local_fastpath_fact_assignment_owner_count=1

map_repr_plan_fact_role=producer_input
map_repr_refresh_writes_local_fastpath_facts=0
map_scalar_no_publication_fact_behavior_preserved=1

user_box_method_fact_producer_enabled=0
mir_json_route_plan_label_hardcoded_map_repr_still_current=1
backend_lowering_changed=0
route_priority_changed=0
fallback_fact_enabled=0
winner_claim_allowed=0

next_task=LOCAL-FASTPATH-FACT-ROUTE-LABEL-SURFACE-001
summary=ok
```

## Stop Lines

```text
do not add user-box facts in this row
do not change MIR JSON route_plan label in this row
do not change backend lowering
do not change route priority
do not create fallback facts
```

## Validation

```text
cargo test -q map_repr_plan --lib
cargo test -q build_mir_json_root_emits_local_fastpath_facts --lib
rg -n 'local_fastpath_facts\s*=' src/mir -S
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
