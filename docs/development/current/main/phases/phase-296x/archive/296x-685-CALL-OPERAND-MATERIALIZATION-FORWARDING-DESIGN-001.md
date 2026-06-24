---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-MATERIALIZATION-FORWARDING-DESIGN-001
Scope: Design a narrow forwarding policy for call-operand materialization Copy
  chains after inventory.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-684-CALL-OPERAND-MATERIALIZATION-COPY-CHAIN-INVENTORY-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# CALL-OPERAND-MATERIALIZATION-FORWARDING-DESIGN-001

## Purpose

296x-684 inventoried the current call-operand Copy surface:

```text
call_operand_chain_count=26
call_operand_unique_copy_count=29
same_block_call_operand_chain_count=26
cross_block_call_operand_chain_count=0
same_block_root_call_operand_chain_count=9
cross_block_root_call_operand_chain_count=14
unknown_root_call_operand_chain_count=3
receiver_operand_chain_count=17
arg_operand_chain_count=9
safe_forwarding_candidate_count=9
dominance_required_candidate_count=14
```

This row designs the next keeper surface. It must not implement forwarding yet.

## Required Output

```text
output_contract=hako-mimalloc-call-operand-materialization-forwarding-design-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-684
call_operand_chain_count=26
safe_forwarding_candidate_count=9
dominance_required_candidate_count=14
unknown_root_call_operand_chain_count=3
selected_keeper_shape=<shape>
selected_keeper_candidate_count=<n>
rejected_arg_forwarding_count=<n>
rejected_unknown_root_count=<n>
requires_dominance_guard=<0|1>
arg_forwarding_enabled=0
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
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
do not enable Arg forwarding
do not forward unknown-root chains
do not change variable_map semantics
do not change PHI lifecycle
do not claim a winner
```

## Result

```text
output_contract=hako-mimalloc-call-operand-materialization-forwarding-design-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-684
call_operand_chain_count=26
safe_forwarding_candidate_count=9
dominance_required_candidate_count=14
unknown_root_call_operand_chain_count=3
selected_keeper_shape=same_block_root_receiver_operand_forwarding
selected_keeper_candidate_count=2
receiver_same_block_root_candidate_count=2
arg_same_block_root_candidate_count=7
rejected_arg_forwarding_count=9
rejected_unknown_root_count=3
rejected_dominance_required_count=14
rejected_receiver_nonlocal_root_count=15
requires_dominance_guard=0
arg_forwarding_enabled=0
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
implementation_started=0
optimization_open=0
winner_claim=0
next_task=call_operand_materialization_forwarding_guard_surface
summary=ok
```

Interpretation:

```text
The first keeper is intentionally tiny: only same-block-root receiver operand
forwarding. Arg forwarding remains closed, unknown-root chains remain closed,
and cross-block/root dominance candidates need a separate design row.
```

## Acceptance

```text
call_operand_materialization_forwarding_design_landed=1
source_evidence=296x-684
design_run=1
selected_keeper_shape=same_block_root_receiver_operand_forwarding
selected_keeper_candidate_count=2
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```
