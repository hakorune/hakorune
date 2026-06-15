---
Status: Landed
Date: 2026-06-15
Task: MIR-CALL-COMPARE-OPERAND-FORWARDING-IMPLEMENTATION-001
Scope: Implement only the dominance-guarded MIR-call-result compare operand
  forwarding seam selected by 296x-745.
Related:
  - docs/development/current/main/phases/phase-296x/296x-745-MIR-CALL-COMPARE-OPERAND-FORWARDING-GUARD-SURFACE-001.md
  - src/mir/builder/ssa/local.rs
  - tools/allocator/hako_mimalloc_mir_call_compare_operand_forwarding_post_probe.py
---

# MIR-CALL-COMPARE-OPERAND-FORWARDING-IMPLEMENTATION-001

## Result

```text
output_contract=hako-mimalloc-mir-call-compare-operand-forwarding-implementation-v0
source_evidence=296x-745
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
implementation_file=src/mir/builder/ssa/local.rs
implemented_scope=dominance_guarded_mir_call_result_to_compare_operand
mirbuilder_object_management_enabled=0
arg_forwarding_enabled=0
field_get_forwarding_enabled=0
param_forwarding_enabled=0
helper_name_special_case=0
benchmark_name_branch_count=0
pre_compare_operand_forwarding_candidate_count=2
post_compare_operand_forwarding_candidate_count=0
post_mir_call_expression_copy_count=0
post_expression_materialization_copy_count=1
post_root_dominates_candidate_count=0
post_unsafe_candidate_count=0
target_met=1
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The implementation extends the existing CompareOperand-only call-result
forwarding seam from same-block roots to dominance-guarded roots:

```text
copy chain:
  Call result -> Copy* -> Compare operand

allowed:
  root Call dominates the compare-use block

not allowed:
  Arg forwarding
  receiver forwarding
  field_get forwarding
  arbitrary LocalSSA copy coalescing
```

This does not change MIRBuilder object management, Box route truth, runtime
objects, product defaults, allocator providers, hooks, or global allocator
behavior.

## Verification

```text
cargo_check=ok
cargo_build_release_hakorune=ok
post_probe=ok
current_state_pointer_guard=ok
diff_check=ok
```

## Stop Line

```text
do not claim a body-time winner from MIR shape only
do not broaden to all MIR-call results
do not key on selectPage/helper names
do not change CFG, PHI placement, or block structure
do not change product runtime/provider behavior
```

## Next

```text
MIR-CALL-COMPARE-OPERAND-FORWARDING-MEASUREMENT-001:
  remeasure object-lifecycle body timing
  compare against pre row 742 / 745 evidence
  decide winner_claim or next owner
```
