# 296x-983 FASTPATH-ROUTE-PRIORITY-TABLE-001

Status: Landed
Date: 2026-06-17
Scope: design / tooling / guard only

## Contract

```text
output_contract=hako-fastpath-route-priority-table-v0
source_evidence=296x-982
row_kind=design_guard
route_priority_table_version=v0
lowest_priority_wins=1
exact_seed_priority=10
local_fastpath_fact_priority=20
generic_metadata_consumer_priority=30
runtime_helper_fallback_priority=90
route_priority_changes_backend_lowering=0
route_priority_retires_exact_seed=0
forced_reachability_allowed=0
winner_claim_from_priority_table_allowed=0
next_task=FASTPATH-REACHABILITY-LEDGER-V1-001
summary=ok
```

## Purpose

Make fast-path route priority explicit before connecting it to reachability
reports.

296x-982 fixed the rule that unreachable consumers cannot support winner
claims. This row fixes the priority vocabulary so future preemption is explained
by a table rather than discovered from backend traces.

## Priority Table

```text
priority=10
family=exact_seed
route_owner=function_level_exact_seed
route_name=exact_seed_backend_route
selected_before=local_fastpath_fact,generic_metadata_consumer,runtime_helper_fallback

priority=20
family=local_fastpath_fact
route_owner=LocalFastPathFact
route_name=local_fastpath_fact
selected_before=generic_metadata_consumer,runtime_helper_fallback

priority=30
family=string_dead_text_region
route_owner=generic_metadata_consumer
route_name=StringDeadTextRegionPlan
selected_before=runtime_helper_fallback

priority=90
family=runtime_helper_fallback
route_owner=runtime_helper_fallback
route_name=runtime_helper_fallback
selected_before=none
```

Lower priority number wins.

## Implementation

Added:

```text
tools/hako_check/fastpath_route_priority.py
tools/hako_check/fastpath_route_priority_table.py
tools/hako_check/tests/test_fastpath_route_priority_table.py
tools/checks/k2_wide_phase296x_fastpath_route_priority_table_guard.sh
```

The table is data-only. It does not inspect MIR, select routes, change backend
lowering, retire exact seeds, force reachability, or make winner claims.

## Guard

```text
bash tools/checks/k2_wide_phase296x_fastpath_route_priority_table_guard.sh
```

The guard checks:

```text
card contract lines
check-scripts-index entry
route priority table unit test
tool output for all v0 entries
```

## Stop Line

This row does not:

```text
change route selection
change backend lowering
connect ledger to priority table
retire exact seed routes
force any candidate to become reachable
make a performance or winner claim
```

## Next

```text
FASTPATH-REACHABILITY-LEDGER-V1-001
```

The next row should make the ledger read this table so selected/preempted rows
carry table-derived priority and stable preemption wording.
