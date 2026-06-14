---
Status: Active
Date: 2026-06-15
Task: CALL-OPERAND-MATERIALIZATION-COPY-CHAIN-INVENTORY-001
Scope: Inventory remaining call-operand materialization Copy chains after the
  LocalSSA call-result fallback Copy keeper.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-683-POST-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-OWNER-REFRESH-REPEAT-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# CALL-OPERAND-MATERIALIZATION-COPY-CHAIN-INVENTORY-001

## Purpose

296x-683 repeated current-MIR owner selection:

```text
copy_count=55
local_ssa_copy_materialization_copy_count=20
call_adjacent_copy_count=29
call_operand_route_carrier_copy_count=29
dominant_copy_owner=local_ssa_copy_materialization
dominant_dynamic_owner=local_ssa_copy_materialization
dominant_position=call_adjacent
dominant_route_carrier_role=call_operand
selected_next_owner=call_operand_materialization_copy_chain_inventory
selected_owner_confidence=medium
```

This row inventories the remaining call-operand materialization Copy chains
before any LocalSSA or call lowering implementation attempt.

## Required Output

```text
output_contract=hako-mimalloc-call-operand-materialization-copy-chain-inventory-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-683
copy_count=55
call_operand_route_carrier_copy_count=29
call_adjacent_copy_count=29
call_operand_chain_count=<n>
same_block_call_operand_chain_count=<n>
cross_block_call_operand_chain_count=<n>
receiver_operand_chain_count=<n>
arg_operand_chain_count=<n>
safe_forwarding_candidate_count=<n>
selected_next_owner=<owner>
selected_owner_confidence=<low|medium|high>
next_task=<task>
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not implement in this row
do not patch source .hako
do not reopen startup optimization
do not broaden LocalSSA copy coalescing
do not forward Arg values without a dedicated design row
do not change variable_map semantics
do not change PHI lifecycle
do not claim a winner
```

## Acceptance

```text
call_operand_materialization_copy_chain_inventory_active=1
source_evidence=296x-683
inventory_run=0
selected_next_owner=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=pending
```
