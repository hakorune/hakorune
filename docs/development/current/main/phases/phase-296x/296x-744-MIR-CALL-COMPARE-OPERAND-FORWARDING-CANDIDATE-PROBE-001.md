---
Status: Active
Date: 2026-06-15
Task: MIR-CALL-COMPARE-OPERAND-FORWARDING-CANDIDATE-PROBE-001
Scope: Count MIR-call-result expression copies that feed compare operands
  before opening any forwarding implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-743-MIR-CALL-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001.md
  - tools/allocator/hako_mimalloc_mir_call_compare_operand_forwarding_candidate_probe.py
---

# MIR-CALL-COMPARE-OPERAND-FORWARDING-CANDIDATE-PROBE-001

## Result

```text
output_contract=hako-mimalloc-mir-call-compare-operand-forwarding-candidate-probe-v0
input_contract=hako-mimalloc-mir-call-expression-copy-chain-policy-selection-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
mir_call_expression_copy_count=2
compare_operand_forwarding_candidate_count=2
same_block_candidate_count=0
dominance_required_candidate_count=2
root_dominates_candidate_count=2
unsafe_candidate_count=0
dominant_candidate_sink=compare_eq
dominant_origin_detail=selectPage
selected_optimization_owner=dominance_guarded_mir_call_compare_operand_forwarding
selected_owner_confidence=medium
next_task=mir_call_compare_operand_forwarding_guard_surface
optimization_open=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
copy_chain_len_0_count=1
copy_chain_len_1_count=1
sink_compare_eq_copy_count=2
origin_detail_selectPage_copy_count=2
summary=ok
```

## Decision

The candidate set is small, but it is not same-block. The MIR call root is in
`block_592`, while the copies and compare are in `block_600`. Therefore this
family requires a dominance guard:

```text
candidate_count=2
same_block_candidate_count=0
dominance_required_candidate_count=2
root_dominates_candidate_count=2
unsafe_candidate_count=0
sink=compare_eq
```

This is enough to define a narrow guard surface, but not enough to implement
from this row. The next row must pin the post-implementation target before any
code changes.

## Stop Line

```text
do not implement from the candidate probe row
do not special-case selectPage by name
do not broaden to all MIR-call results
do not broaden to all compare operands
do not reopen LocalSSA broad copy coalescing
do not change MIRBuilder object management
do not change product runtime, provider activation, hooks, or global allocator
```

## Next

```text
MIR-CALL-COMPARE-OPERAND-FORWARDING-GUARD-SURFACE-001:
  post_compare_operand_forwarding_candidate_count=0
  post_mir_call_expression_copy_count<=2
  post_root_dominates_candidate_count=0
  post_unsafe_candidate_count=0
  optimization implementation still closed until guard lands
```
