# 296x-985 EXACT-SEED-RETIRE-INVENTORY-001

Status: Landed
Date: 2026-06-17
Scope: tooling / guard / docs only

## Contract

```text
output_contract=hako-exact-seed-retire-inventory-v0
source_evidence=296x-984
row_kind=inventory
replacement_candidate_required=1
replacement_reachable_required=1
retire_allowed=0
drive_by_retire_allowed=0
exact_seed_retired=0
backend_lowering_changed=0
route_priority_changed=0
forced_reachability_allowed=0
next_task=FASTPATH-CONSUMER-REACHABILITY-GATE-001
summary=ok
```

## Purpose

Inventory exact-seed routes before any retirement decision.

296x-984 made exact-seed preemption visible. This row adds a smaller inventory
surface that answers only:

```text
Does the active MIR front have an exact seed?
Does it have a replacement candidate?
Is that replacement already reachable?
```

If the replacement is absent or not reachable, exact-seed retirement remains
closed.

## Added

```text
tools/hako_check/exact_seed_retire_inventory.py
tools/hako_check/tests/test_exact_seed_retire_inventory.py
tools/checks/k2_wide_phase296x_exact_seed_retire_inventory_guard.sh
```

## Report Contract

```text
output_contract=hako-exact-seed-retire-inventory-v0
route_priority_table_version=v0
front
function
exact_seed_present
exact_seed_tag
exact_seed_source_route
exact_seed_proof
exact_seed_selected_value
replacement_family
replacement_candidate_exists
replacement_reachable
preemption_detected
retire_allowed=0
retire_blocker
drive_by_retire_allowed=0
backend_lowering_changed=0
exact_seed_retired=0
winner_claim_allowed=0
summary=ok
```

## Stop Line

This row does not:

```text
delete exact seed routes
change route priority
change backend lowering
force reachability
add replacement consumers
make a winner claim
```

## Next

```text
FASTPATH-CONSUMER-REACHABILITY-GATE-001
```

With exact-seed retirement gated by reachable replacement evidence, the next
row fixes the general rule for adding backend consumers.
