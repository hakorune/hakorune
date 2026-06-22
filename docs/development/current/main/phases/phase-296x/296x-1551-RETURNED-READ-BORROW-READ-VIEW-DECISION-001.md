# 296x-1551 RETURNED-READ-BORROW-READ-VIEW-DECISION-001

Status: landed
Date: 2026-06-22

## Purpose

Record the read-view decision for `VariableContext::variable_map()` without
opening a true read-view route.

The current contract remains:

```text
NoReturnedAlias + OwnedReadSnapshotProjection
```

That keeps bulk read consumers on owned snapshots and defers true read views
to a later hard tier.

## Scope

```text
BoxCount: one consultation inventory
owner: VariableContext returned read borrow / read-view decision
input: current returned-borrow boundary inventory
output: one durable decision inventory and guard
```

## Decision

```text
keep OwnedReadSnapshotProjection for bulk read consumers
defer true read-view selection
do not re-open variable_map() as a naked borrowed alias
```

## Required Checks

```text
python3 tools/rust_lifecycle/mirbuilder_returned_read_borrow_read_view_decision_inventory.py --check-reference
bash tools/checks/rust_mirbuilder_returned_read_borrow_read_view_decision_guard.sh
```

## Acceptance

```text
the read-view decision space is fixed in one machine-readable fixture
NoReturnedAlias remains the current contract
route selection remains unopened
nightly rustc adapter remains unopened
```

## Stop Line

```text
do_not_select_route=1
do_not_open_nightly_rustc_adapter=1
do_not_claim_mirbuilder_wide_conversion=1
do_not_add_runtime_fallback=1
```
