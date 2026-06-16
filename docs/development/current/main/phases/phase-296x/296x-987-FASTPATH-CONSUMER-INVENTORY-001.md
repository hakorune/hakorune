# 296x-987 FASTPATH-CONSUMER-INVENTORY-001

Status: Landed
Date: 2026-06-17
Scope: tooling / guard / docs only

## Contract

```text
output_contract=hako-fastpath-consumer-inventory-v0
source_evidence=296x-986
row_kind=inventory
consumer_inventory_kind=current_lane_known_families
consumer_count=5
backend_consumer_code_is_not_reachability=1
winner_claim_requires_reachable_consumer=1
unknown_consumer_winner_claim_allowed=0
backend_lowering_changed=0
route_priority_changed=0
next_task=FASTPATH-CLOSEOUT-001
summary=ok
```

## Purpose

List the current fast-path consumer families before closeout.

This row is intentionally a small inventory, not a source scanner and not a
route selector. Active-front reachability still belongs to the reachability
ledger.

## Consumer Families

```text
exact_seed:
  status=selected_route_family

local_fastpath_fact:
  status=positive_fact_surface

local_i64_map_entry_table:
  status=landed_reachable_closed

string_dead_text_region:
  status=backend_consumer_exists_reachability_blocked

runtime_helper_fallback:
  status=fallback_not_fastpath
```

## Added

```text
tools/hako_check/fastpath_consumer_inventory.py
tools/hako_check/tests/test_fastpath_consumer_inventory.py
tools/checks/k2_wide_phase296x_fastpath_consumer_inventory_guard.sh
```

## Stop Line

This row does not:

```text
scan source for new consumers
change backend lowering
change route priority
force reachability
retire exact seeds
make a winner claim
```

## Next

```text
FASTPATH-CLOSEOUT-001
```
