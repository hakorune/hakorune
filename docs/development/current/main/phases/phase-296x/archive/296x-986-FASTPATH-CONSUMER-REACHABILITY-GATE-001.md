# 296x-986 FASTPATH-CONSUMER-REACHABILITY-GATE-001

Status: Landed
Date: 2026-06-17
Scope: process guard / docs only

## Contract

```text
output_contract=hako-fastpath-consumer-reachability-gate-v0
source_evidence=296x-985
row_kind=process_guard
new_consumer_requires_reachable_or_scaffold=1
scaffold_requires_followup_row=1
scaffold_requires_winner_claim_allowed_0=1
backend_consumer_code_is_not_reachability=1
gate_reuses_reachability_ledger=1
gate_reuses_exact_seed_retire_inventory=1
exact_seed_retire_inventory_required_before_retire=1
forced_reachability_allowed=0
backend_lowering_changed=0
next_task=FASTPATH-CONSUMER-INVENTORY-001
summary=ok
```

## Purpose

Fix the process rule for fast-path backend consumers:

```text
consumer code exists != active executable route reached
```

If a row adds a backend consumer, it must prove one of these:

```text
reachable_in_active_front=1
```

or:

```text
intentionally_unreachable_scaffold=1
winner_claim_allowed=0
followup_row_named=1
```

## Guard Surface

The guard reuses the landed reachability and exact-seed inventory surfaces:

```text
tools/hako_check/fastpath_reachability_ledger.py
tools/hako_check/exact_seed_retire_inventory.py
```

It verifies candidate-only consumers and preempted replacement candidates do
not allow winner claims or exact-seed retirement.

## Stop Line

This row does not:

```text
add a backend consumer
change route selection
change backend lowering
retire exact seed routes
force reachability
make a winner claim
```

## Next

```text
FASTPATH-CONSUMER-INVENTORY-001
```

With the gate fixed, the next row can list the current fast-path consumer
families and their reachability status.
