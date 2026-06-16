# 296x-982 FASTPATH-UNREACHABLE-CONSUMER-GUARD-001

Status: Landed
Date: 2026-06-17
Scope: guard / docs / tests only

## Contract

```text
output_contract=hako-fastpath-unreachable-consumer-guard-v0
source_evidence=296x-981
row_kind=guard
unreachable_consumer_winner_claim_allowed=0
preempted_consumer_winner_claim_allowed=0
candidate_only_winner_claim_allowed=0
forced_reachability_allowed=0
backend_consumer_code_is_not_reachability=1
active_mir_metadata_candidate_required=1
selected_route_required_for_reachability=1
new_backend_consumer_requires_reachability_or_scaffold=1
scaffold_requires_winner_claim_allowed_0=1
scaffold_requires_followup_row=1
route_priority_changed=0
backend_lowering_changed=0
exact_seed_retired=0
summary=ok
```

## Purpose

Prevent backend consumer code from being counted as a fast-path win unless it
is actually reachable in the active front.

This row turns the 296x-981 reachability ledger into a guard rule:

```text
If a row adds a new backend consumer, it must provide one of:

A. reachable_in_active_front=1

B. intentionally_unreachable_scaffold=1
   winner_claim_allowed=0
   follow-up route selection/retire row named
```

The rule closes the ambiguity between:

```text
consumer code exists
candidate metadata exists
selected route reaches that consumer
```

Only the last one can support a winner claim.

## Guard

Added:

```text
tools/checks/k2_wide_phase296x_fastpath_unreachable_consumer_guard.sh
```

The guard checks:

```text
card contract lines are fixed
check-scripts-index lists the guard
reachability ledger unit tests pass
preempted synthetic candidate reports winner_claim_allowed=0
candidate-only synthetic report has selected_route=none and winner_claim_allowed=0
```

## Test Update

Updated:

```text
tools/hako_check/tests/test_fastpath_reachability_ledger.py
```

New unit coverage:

```text
unselected candidate is not reachable
candidate-only report has winner_claim_allowed=0
```

## Stop Line

This row does not:

```text
change backend lowering
change route priority
force reachability
retire exact seed routes
add new backend consumers
make a performance or winner claim
```

## Next

```text
FASTPATH-ROUTE-PRIORITY-TABLE-001
```

The next design row should make route priority explicit enough that future
preemption is intentional instead of discovered from backend execution traces.
