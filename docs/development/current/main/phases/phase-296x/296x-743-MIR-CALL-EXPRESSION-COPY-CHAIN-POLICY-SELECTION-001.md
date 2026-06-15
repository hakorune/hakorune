---
Status: Active
Date: 2026-06-15
Task: MIR-CALL-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001
Scope: Select the policy owner for MIR-call-origin expression copy chains
  before any LocalSSA or lowering implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-742-MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-002.md
  - tools/allocator/hako_mimalloc_expression_materialization_copy_origin_probe.py
  - tools/allocator/hako_mimalloc_mir_call_expression_copy_chain_policy_selection.py
---

# MIR-CALL-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001

## Result

```text
output_contract=hako-mimalloc-mir-call-expression-copy-chain-policy-selection-v0
input_contract=hako-mimalloc-expression-materialization-copy-origin-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
mir_call_origin_copy_count=2
expression_materialization_copy_count=3
mir_call_origin_ratio_bp=6666
mir_call_compare_sink_copy_count=2
mir_call_select_page_origin_copy_count=2
const_unused_copy_count=1
origin_copy_chain_len_0_count=2
origin_copy_chain_len_1_count=1
selected_chain_policy=mir_call_compare_operand_value_forwarding_candidate_probe
selected_chain_policy_confidence=medium
selected_chain_policy_reason=mir_call_origin_reaches_compare_operand_but_expression_count_is_small
rejected_chain_policy=field_get_expression_value_copy_chain
rejected_reason=current_expression_origin_is_mir_call_not_field_get
rejected_chain_policy_2=param_direct_consumer_value_forwarding
rejected_reason_2=current_expression_origin_is_mir_call_not_param
rejected_chain_policy_3=local_ssa_broad_copy_coalescing
rejected_reason_3=recent_local_ssa_same_block_reuse_nonkeeper
next_diagnostic=mir_call_compare_operand_forwarding_candidate_probe
optimization_open=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The fresh owner from 296x-742 is a small MIR-call expression copy chain, not a
field-get or parameter chain. The evidence is strong enough to inspect a narrow
candidate family, but not strong enough to open implementation directly:

```text
selectPage result -> expression copy -> compare_eq operand
```

The next row should count candidate sites where a MIR-call result can be used
directly as a compare operand without intermediate expression materialization.

## Stop Line

```text
do not implement from this policy-selection row
do not special-case selectPage or any helper by name
do not reopen field_get or param forwarding rows from this evidence
do not broaden to LocalSSA copy coalescing
do not change MIRBuilder object management
do not change product runtime, provider activation, hooks, or global allocator
```

## Next

```text
MIR-CALL-COMPARE-OPERAND-FORWARDING-CANDIDATE-PROBE-001:
  count direct compare-operand forwarding candidates for MIR-call results
  report whether the candidate set is small, safe, and dominance-local
  keep optimization_open=0 until a guard surface is pinned
```
