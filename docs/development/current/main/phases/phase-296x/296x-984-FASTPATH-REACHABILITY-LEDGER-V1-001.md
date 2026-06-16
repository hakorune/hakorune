# 296x-984 FASTPATH-REACHABILITY-LEDGER-V1-001

Status: Landed
Date: 2026-06-17
Scope: tooling / guard / docs only

## Contract

```text
output_contract=hako-fastpath-reachability-ledger-v1
source_evidence=296x-983
row_kind=tooling
route_priority_table_version=v0
selected_route_priority_source=route_priority_table_v0
preempted_reason=lower_priority_selected_route
candidate_only_selected_route=none
candidate_only_winner_claim_allowed=0
route_priority_changes_backend_lowering=0
route_priority_retires_exact_seed=0
forced_reachability_allowed=0
next_task=EXACT-SEED-RETIRE-INVENTORY-001
summary=ok
```

## Purpose

Connect the reachability ledger to the route priority table from 296x-983.

The ledger still does not select routes. It reads existing MIR JSON metadata,
recognizes explicitly selected routes, and now explains selected/preempted rows
with table-derived priority values.

## Changes

Updated:

```text
tools/hako_check/fastpath_reachability_ledger.py
tools/hako_check/tests/test_fastpath_reachability_ledger.py
tools/hako_check/README.md
```

Added:

```text
tools/checks/k2_wide_phase296x_fastpath_reachability_ledger_v1_guard.sh
```

## V1 Additions

```text
route_priority_table_version=v0
selected_route_priority_source=route_priority_table_v0
candidate_N_preempted_reason=lower_priority_selected_route
```

Candidate-only rows remain unreachable:

```text
selected_route=none
selected_route_priority_source=none
winner_claim_allowed=0
```

## Stop Line

This row does not:

```text
change backend lowering
change route selection
force reachability
retire exact seed routes
add backend consumers
make a performance or winner claim
```

## Next

```text
EXACT-SEED-RETIRE-INVENTORY-001
```

With selected/preempted route priority visible, the next row can inventory old
exact seeds deliberately instead of deleting or keeping them by guesswork.
