---
Status: Active
Date: 2026-06-15
Task: PAGE-HOTPATH-HELPER-RESULT-MATERIALIZATION-INVENTORY-001
Scope: Inventory page-hotpath helper result materialization copy chains after
  the field_get alias keeper.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-673-POST-FIELD-GET-ALIAS-KEEPER-OWNER-REFRESH-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# PAGE-HOTPATH-HELPER-RESULT-MATERIALIZATION-INVENTORY-001

## Purpose

296x-673 selected the next owner:

```text
selected_next_owner=page_hotpath_helper_result_materialization_copy_chain
page_hotpath_helpers_call_count=5
page_hotpath_helpers_attributed_copy_count=22
result_materialization_copy_count=21
dominant_position=call_adjacent
dominant_route_carrier_role=call_operand
```

This row inventories the helper result copy chains before any implementation.

## Required Output

```text
output_contract=hako-mimalloc-page-hotpath-helper-result-materialization-inventory-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
page_hotpath_helpers_call_count=5
page_hotpath_helpers_attributed_copy_count=22
result_materialization_copy_count=21
dominant_helper=<helper>
dominant_result_chain_shape=<shape>
selected_owner=<owner>
selected_owner_confidence=<low|medium|high>
next_task=<task>
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not change helper lowering yet
do not broaden LocalSSA coalescing
do not reopen field_get alias keeper
do not touch allocator provider activation
```

## Acceptance

```text
page_hotpath_helper_result_materialization_inventory_active=1
source_evidence=296x-673
inventory_run=0
selected_owner=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=pending
```
