---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-MATERIALIZATION-FORWARDING-IMPLEMENTATION-001
Scope: Implement only the selected same-block-root receiver operand forwarding
  keeper in LocalSSA.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-686-CALL-OPERAND-MATERIALIZATION-FORWARDING-GUARD-SURFACE-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# CALL-OPERAND-MATERIALIZATION-FORWARDING-IMPLEMENTATION-001

## Purpose

296x-686 fixed the implementation guard surface:

```text
selected_keeper_shape=same_block_root_receiver_operand_forwarding
pre_selected_keeper_candidate_count=2
post_selected_keeper_candidate_count_target=0
post_call_operand_unique_copy_count_upper_bound=27
arg_forwarding_enabled=0
helper_name_special_case=0
requires_dominance_guard=0
```

This row may implement only that narrow keeper.

## Required Output

```text
output_contract=hako-mimalloc-call-operand-materialization-forwarding-implementation-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-686
selected_keeper_shape=same_block_root_receiver_operand_forwarding
pre_selected_keeper_candidate_count=2
post_selected_keeper_candidate_count=0
post_call_operand_unique_copy_count=<n>
post_call_operand_unique_copy_count_upper_bound=27
arg_forwarding_enabled=0
helper_name_special_case=0
requires_dominance_guard=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
implementation_started=1
optimization_open=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not patch source .hako
do not reopen startup optimization
do not broaden LocalSSA copy coalescing
do not enable Arg forwarding
do not forward unknown-root chains
do not forward cross-block/root chains
do not special-case helper names
do not change variable_map semantics
do not change PHI lifecycle
do not claim a winner from MIR shape alone
```

## Result

```text
output_contract=hako-mimalloc-call-operand-materialization-forwarding-implementation-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-686
selected_keeper_shape=same_block_root_receiver_operand_forwarding
pre_selected_keeper_candidate_count=2
post_selected_keeper_candidate_count=0
post_call_operand_unique_copy_count=27
post_call_operand_unique_copy_count_upper_bound=27
arg_forwarding_enabled=0
helper_name_special_case=0
requires_dominance_guard=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
implementation_started=1
optimization_open=0
winner_claim=0
summary=ok
```

Interpretation:

```text
The selected receiver-only same-block-root keeper landed and removed the target
call-operand carrier family. Measure body timing next before any winner or
next-owner claim.
```

## Acceptance

```text
call_operand_materialization_forwarding_implementation_landed=1
source_evidence=296x-686
implementation_started=1
post_selected_keeper_candidate_count=0
optimization_open=0
winner_claim=0
summary=ok
```
