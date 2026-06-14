---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-RESIDUAL-POLICY-SELECTION-001
Scope: Select the next residual call-operand policy family after the
  receiver-root forwarding keeper failed to improve body timing.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-689-POST-CALL-OPERAND-MATERIALIZATION-FORWARDING-OWNER-REFRESH-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# CALL-OPERAND-RESIDUAL-POLICY-SELECTION-001

## Purpose

296x-689 selected residual policy selection:

```text
copy_count=54
call_operand_route_carrier_copy_count=27
call_adjacent_copy_count=27
arg_same_block_root_call_operand_chain_count=7
dominance_required_candidate_count=14
unknown_root_call_operand_chain_count=3
receiver_cross_block_root_call_operand_chain_count=13
```

The next row must choose a policy family before any further implementation.

## Required Output

```text
output_contract=hako-mimalloc-call-operand-residual-policy-selection-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-689
arg_same_block_root_call_operand_chain_count=7
dominance_required_candidate_count=14
unknown_root_call_operand_chain_count=3
receiver_cross_block_root_call_operand_chain_count=13
selected_policy_family=<family>
selected_policy_candidate_count=<n>
rejected_policy_family=<family>
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
do not claim a winner
```

## Result

```text
output_contract=hako-mimalloc-call-operand-residual-policy-selection-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-689
arg_same_block_root_call_operand_chain_count=7
dominance_required_candidate_count=14
unknown_root_call_operand_chain_count=3
receiver_cross_block_root_call_operand_chain_count=13
selected_policy_family=dominance_required_call_operand_forwarding
selected_policy_candidate_count=14
rejected_policy_family=arg_same_block_root_forwarding
rejected_policy_candidate_count=7
selected_next_owner=call_operand_dominance_required_forwarding_design
selected_owner_confidence=medium
selected_reason=dominance_required_surface_is_larger_than_arg_safe_surface_and_previous_tiny_receiver_keeper_was_not_a_body_time_keeper
next_task=call_operand_dominance_required_forwarding_design
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

Interpretation:

```text
Do not implement immediately. The next row must design the dominance/visibility
contract for cross-root call operands before LocalSSA can forward them safely.
```

## Acceptance

```text
call_operand_residual_policy_selection_landed=1
source_evidence=296x-689
policy_selection_run=1
selected_policy_family=dominance_required_call_operand_forwarding
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```
