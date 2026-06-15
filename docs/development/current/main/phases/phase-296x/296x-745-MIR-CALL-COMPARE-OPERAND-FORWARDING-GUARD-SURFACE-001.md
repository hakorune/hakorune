---
Status: Active
Date: 2026-06-15
Task: MIR-CALL-COMPARE-OPERAND-FORWARDING-GUARD-SURFACE-001
Scope: Pin the post-implementation guard surface for MIR-call result compare
  operand forwarding before code changes.
Related:
  - docs/development/current/main/phases/phase-296x/296x-744-MIR-CALL-COMPARE-OPERAND-FORWARDING-CANDIDATE-PROBE-001.md
  - tools/allocator/hako_mimalloc_mir_call_compare_operand_forwarding_candidate_probe.py
---

# MIR-CALL-COMPARE-OPERAND-FORWARDING-GUARD-SURFACE-001

## Result

```text
output_contract=hako-mimalloc-mir-call-compare-operand-forwarding-guard-surface-v0
source_evidence=296x-744
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
pre_mir_call_expression_copy_count=2
pre_compare_operand_forwarding_candidate_count=2
pre_same_block_candidate_count=0
pre_dominance_required_candidate_count=2
pre_root_dominates_candidate_count=2
pre_unsafe_candidate_count=0
selected_optimization_owner=dominance_guarded_mir_call_compare_operand_forwarding
selected_owner_confidence=medium
post_compare_operand_forwarding_candidate_count=0
post_mir_call_expression_copy_count_upper_bound=2
post_root_dominates_candidate_count=0
post_unsafe_candidate_count=0
allowed_scope=dominance_guarded_mir_call_result_to_compare_operand
arg_forwarding_enabled=0
field_get_forwarding_enabled=0
param_forwarding_enabled=0
helper_name_special_case=0
benchmark_name_branch_count=0
optimization_open=1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

Implementation may open for one narrow seam:

```text
copy category:
  expression_materialization

origin:
  MIR call result, including one-hop copy chains

sink:
  compare operand in a block dominated by the MIR call root

target:
  remove the selected forwarding candidate family
```

The implementation must not key on `selectPage` or any benchmark/helper name.
The helper name only appears in evidence as the current origin detail.

## Stop Line

```text
do not forward call arguments
do not forward field_get values
do not forward param values
do not coalesce broad LocalSSA copies
do not change CFG, PHI placement, or block structure
do not move Box management into MIRBuilder
do not change product runtime, provider activation, hooks, or global allocator
```

## Next

```text
MIR-CALL-COMPARE-OPERAND-FORWARDING-IMPLEMENTATION-001:
  implement only the dominance-guarded MIR-call-result compare operand forwarding seam
  require post_compare_operand_forwarding_candidate_count=0
  keep winner_claim=0 until body timing is remeasured
```
